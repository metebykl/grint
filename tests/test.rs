use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::str;

use super::*;

pub(crate) struct Output {
  pub(crate) pid: u32,
  pub(crate) stdout: String,
  pub(crate) tempdir: TempDir,
}

pub(crate) struct Test {
  pub(crate) args: Vec<String>,
  pub(crate) current_dir: PathBuf,
  pub(crate) grintfile: Option<String>,
  pub(crate) status: i32,
  pub(crate) stdin: String,
  pub(crate) stdout: String,
  pub(crate) stderr: String,
  pub(crate) tempdir: TempDir,
}

impl Test {
  pub(crate) fn new() -> Self {
    Self {
      args: Vec::new(),
      current_dir: PathBuf::new(),
      grintfile: Some(String::new()),
      status: 0,
      stdin: String::new(),
      stdout: String::new(),
      stderr: String::new(),
      tempdir: tempdir(),
    }
  }

  pub(crate) fn arg(mut self, val: &str) -> Self {
    self.args.push(val.to_owned());
    self
  }

  pub(crate) fn args(mut self, args: &[&str]) -> Self {
    for arg in args {
      self = self.arg(arg);
    }
    self
  }

  pub(crate) fn grintfile(mut self, grintfile: &str) -> Self {
    self.grintfile = Some(grintfile.to_owned());
    self
  }

  pub(crate) fn grintfile_path(&self) -> PathBuf {
    self.tempdir.path().join("Grint.toml")
  }

  pub(crate) fn no_grintfile(mut self) -> Self {
    self.grintfile = None;
    self
  }

  pub(crate) fn status(mut self, exit_status: i32) -> Self {
    self.status = exit_status;
    self
  }

  pub(crate) fn stdin(mut self, stdin: &str) -> Self {
    self.stdin = stdin.to_owned();
    self
  }

  pub(crate) fn stdout(mut self, stdout: &str) -> Self {
    self.stdout = stdout.to_owned();
    self
  }

  pub(crate) fn stderr(mut self, stderr: &str) -> Self {
    self.stderr = stderr.to_owned();
    self
  }
}

impl Test {
  pub(crate) fn run(self) -> Output {
    if let Some(grintfile) = &self.grintfile {
      fs::write(self.grintfile_path(), grintfile).unwrap();
    }

    let mut command = Command::new(executable_path("grint"));
    let mut child = command
      .args(&self.args)
      .current_dir(self.tempdir.path().join(&self.current_dir))
      .stdin(Stdio::piped())
      .stdout(Stdio::piped())
      .stderr(Stdio::piped())
      .spawn()
      .expect("grint invocation failed");

    let pid = child.id();

    let mut stdin_handle = child.stdin.take().expect("failed to take stdin handle");
    stdin_handle
      .write_all(self.stdin.as_bytes())
      .expect("failed to write stdin to grint process");

    let output = child
      .wait_with_output()
      .expect("failed to wait for grint process");

    let output_stdout = str::from_utf8(&output.stdout).unwrap();
    let output_stderr = str::from_utf8(&output.stderr).unwrap();

    if !compare("status", output.status.code(), Some(self.status))
      | !compare_string("stdout", output_stdout, &self.stdout)
      | !compare_string("stderr", output_stderr, &self.stderr)
    {
      panic!("Output mismatch.");
    }

    Output {
      pid,
      stdout: output_stdout.into(),
      tempdir: self.tempdir,
    }
  }
}
