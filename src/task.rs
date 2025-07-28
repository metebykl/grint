use std::process::Command;

pub(crate) struct Task {
  pub(crate) name: String,
  pub(crate) desc: Option<String>,
  pub(crate) body: String,
}

impl Task {
  pub(crate) fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("> {}", self.body);

    let status = if cfg!(target_os = "windows") {
      Command::new("cmd").arg("/C").arg(&self.body).status()?
    } else {
      Command::new("sh").arg("-c").arg(&self.body).status()?
    };

    if !status.success() {
      return Err(
        format!(
          "task '{}' failed with exit code {}",
          self.name,
          status.code().unwrap_or(1)
        )
        .into(),
      );
    }

    Ok(())
  }
}
