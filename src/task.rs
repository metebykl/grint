use std::path::PathBuf;

use crate::{Config, Grintfile};

pub(crate) struct Task {
  pub(crate) name: String,
  pub(crate) desc: Option<String>,
  pub(crate) cwd: Option<PathBuf>,
  pub(crate) body: String,
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

    if let Some(cwd) = &self.cwd {
      command.current_dir(cwd);
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
