use super::*;

#[test]
fn show() {
  Test::new()
    .grintfile(
      r#"
      [task.foo]
      cmd = "echo 'foo'"
      "#,
    )
    .arg("--show")
    .arg("foo")
    .stdout("foo:\n  echo 'foo'\n")
    .run();
}

#[test]
fn show_description() {
  Test::new()
    .grintfile(
      r#"
      [task.foo]
      desc = "Example task"
      cmd = "echo 'foo'"
      "#,
    )
    .arg("--show")
    .arg("foo")
    .stdout("# Example task\nfoo:\n  echo 'foo'\n")
    .run();
}

#[test]
fn show_dependencies() {
  Test::new()
    .grintfile(
      r#"
      [task.foo]
      deps = ["bar", "baz"]
      cmd = "echo 'foo'"
      "#,
    )
    .arg("--show")
    .arg("foo")
    .stdout("foo: @bar @baz\n  echo 'foo'\n")
    .run();
}

#[test]
fn show_all_fields() {
  Test::new()
    .grintfile(
      r#"
      [task.foo]
      desc = "Example task"
      deps = ["bar", "baz"]
      cmd = "echo 'foo'"
      "#,
    )
    .arg("--show")
    .arg("foo")
    .stdout("# Example task\nfoo: @bar @baz\n  echo 'foo'\n")
    .run();
}

#[test]
fn show_unknown_command() {
  Test::new()
    .arg("--show")
    .arg("foo")
    .status(1)
    .stderr("error: Task `foo` not found\n")
    .run();
}
