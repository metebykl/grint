use super::*;

#[test]
fn list_tasks_single() {
  Test::new()
    .grintfile(
      r#"
      [task.build]
      cmd = ""
      "#,
    )
    .arg("--list")
    .stdout(
      "Available tasks:
  build
",
    )
    .run();
}

#[test]
fn list_tasks_multiple() {
  Test::new()
    .grintfile(
      r#"
      [task.foo]
      cmd = ""
      
      [task.bar]
      cmd = ""

      [task.baz]
      cmd = ""
      "#,
    )
    .arg("--list")
    .stdout(
      "Available tasks:
  foo
  bar
  baz
",
    )
    .run();
}

#[test]
fn list_tasks_description() {
  Test::new()
    .grintfile(
      r#"
      [task.test]
      desc = "Run tests"
      cmd = ""

      [task.build]
      desc = "Build app"
      cmd = ""
      "#,
    )
    .arg("--list")
    .stdout(
      "Available tasks:
  test   # Run tests
  build  # Build app
",
    )
    .run();
}
