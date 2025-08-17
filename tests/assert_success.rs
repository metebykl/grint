#[track_caller]
pub(crate) fn assert_success(output: &std::process::Output) {
  if !output.status.success() {
    eprintln!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    panic!("{}", output.status)
  }
}
