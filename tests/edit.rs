use std::env;
use std::env::consts::EXE_SUFFIX;
use std::fs;
use std::iter;
use std::process::Command;

use super::*;

const GRINTFILE: &str = r#"
[task.foo]
cmd = "echo foo"
"#;

#[test]
fn editor_invoke_error() {
  let tmp = tempdir();
  let grintfile_path = tmp.path().join("Grint.toml");
  fs::write(grintfile_path, GRINTFILE).unwrap();

  let output = Command::new(executable_path("grint"))
    .current_dir(tmp.path())
    .arg("--edit")
    .env("VISUAL", "/")
    .output()
    .unwrap();

  assert!(!output.status.success());

  assert_eq!(
    String::from_utf8_lossy(&output.stderr),
    if cfg!(windows) {
      "error: Editor `/` invocation failed: program path has no file name\n"
    } else {
      "error: Editor `/` invocation failed: Permission denied (os error 13)\n"
    }
  );
}

#[cfg(unix)]
#[test]
fn editor_precedence() {
  let tmp = tempdir();
  let grintfile_path = tmp.path().join("Grint.toml");
  fs::write(grintfile_path, GRINTFILE).unwrap();

  let output = Command::new(executable_path("grint"))
    .current_dir(tmp.path())
    .arg("--edit")
    .env("VISUAL", "cat")
    .env("EDITOR", "no-command")
    .output()
    .unwrap();

  assert_stdout(&output, GRINTFILE);

  let output = Command::new(executable_path("grint"))
    .current_dir(tmp.path())
    .arg("--edit")
    .env_remove("VISUAL")
    .env("EDITOR", "cat")
    .output()
    .unwrap();

  assert_stdout(&output, GRINTFILE);

  let cat = which("cat").unwrap();
  let vim = tmp.path().join(format!("vim{EXE_SUFFIX}"));

  std::os::unix::fs::symlink(cat, vim).unwrap();

  let env_path = env::join_paths(
    iter::once(tmp.path().to_owned()).chain(env::split_paths(&env::var_os("PATH").unwrap())),
  )
  .unwrap();

  let output = Command::new(executable_path("grint"))
    .current_dir(tmp.path())
    .arg("--edit")
    .env("PATH", env_path)
    .env_remove("VISUAL")
    .env_remove("EDITOR")
    .output()
    .unwrap();

  assert_stdout(&output, GRINTFILE);
}
