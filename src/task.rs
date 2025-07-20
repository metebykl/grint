pub(crate) struct Task {
  pub(crate) name: String,
  pub(crate) body: String,
}

impl Task {
  pub(crate) fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("Executing '{}'", self.body);
    Ok(())
  }
}
