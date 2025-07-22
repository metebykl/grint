use super::*;

#[test]
fn list_tasks() {
  Test::new()
    .grintfile(
      "
    [task.greet]
    cmd = \"echo 'Hello World!'\"
    ",
    )
    .arg("--list")
    .stdout(
      "Available tasks:
  greet
",
    )
    .run();
}
