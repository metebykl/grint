use std::collections::HashMap;
use std::path::PathBuf;

use crate::{Config, Error, Grintfile};

pub(crate) struct Task {
  pub(crate) body: String,
  pub(crate) dependencies: Vec<String>,
  pub(crate) desc: Option<String>,
  pub(crate) env: HashMap<String, String>,
  pub(crate) name: String,
  pub(crate) working_directory: Option<PathBuf>,
}

impl Task {
  pub(crate) fn run(&self, config: &Config, grintfile: &Grintfile) -> Result<(), Error> {
    println!("> {}", self.body);

    let mut command = grintfile.settings.shell_command(config);
    command.arg(&self.body);

    for (name, value) in &self.env {
      command.env(name, value);
    }

    if let Some(working_directory) = &self.working_directory {
      command.current_dir(working_directory);
    }

    let status = command.status().map_err(|io_error| Error::CommandInvoke {
      command: self.body.clone(),
      io_error,
    })?;

    if !status.success() {
      return Err(Error::CommandStatus {
        command: self.body.clone(),
        status,
      });
    }

    Ok(())
  }
}
