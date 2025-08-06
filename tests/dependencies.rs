use super::*;

#[test]
fn dependency_cycle_error() {
  Test::new()
    .grintfile(
      r#"
    [task.foo]
    deps = ["bar"]
    cmd = "echo FOO"

    [task.bar]
    deps = ["foo"]
    cmd = "echo BAR"
    "#,
    )
    .arg("foo")
    .status(1)
    .stderr("error: Dependency cycle detected: foo -> bar -> foo\n")
    .run();
}

#[test]
fn self_dependency_cycle_error() {
  Test::new()
    .grintfile(
      r#"
    [task.foo]
    deps = ["foo"]
    cmd = "echo FOO"
    "#,
    )
    .arg("foo")
    .status(1)
    .stderr("error: Dependency cycle detected: foo -> foo\n")
    .run();
}

#[test]
fn dependency_not_found_error() {
  Test::new()
    .grintfile(
      r#"
    [task.foo]
    deps = ["bar"]
    cmd = "echo FOO"
    "#,
    )
    .arg("foo")
    .status(1)
    .stderr("error: Task `bar` not found\n")
    .run();
}
