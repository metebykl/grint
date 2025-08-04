use std::path::PathBuf;

use crate::{Config, Grintfile};

pub(crate) struct Task {
  pub(crate) body: String,
  pub(crate) dependencies: Vec<String>,
  pub(crate) desc: Option<String>,
  pub(crate) name: String,
  pub(crate) working_directory: Option<PathBuf>,
}

impl Task {
  pub(crate) fn run(
    &self,
    config: &Config,
    grintfile: &Grintfile,
  ) -> Result<(), Box<dyn std::error::Error>> {
    println!("> {}", self.body);

    let mut command = grintfile.settings.shell_command(config);
    command.arg(&self.body);

    if let Some(working_directory) = &self.working_directory {
      command.current_dir(working_directory);
    }

    let status = command.status()?;

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
