use super::*;

#[test]
fn list_tasks_single() {
  Test::new()
    .grintfile(
      "
    [task.build]
    cmd = \"\"
    ",
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
      "
    [task.foo]
    cmd = \"\"

    [task.bar]
    cmd = \"\"

    [task.baz]
    cmd = \"\"
    ",
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
