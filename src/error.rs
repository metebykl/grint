use std::fmt::{Display, Formatter, Result};
use std::io;
use std::path::PathBuf;
use std::process::ExitStatus;

use toml_edit::TomlError;

pub(crate) enum Error {
  CommandInvoke {
    command: String,
    io_error: io::Error,
  },
  CommandStatus {
    command: String,
    status: ExitStatus,
  },
  DependencyCycle {
    task_name: String,
    cycle: Vec<String>,
  },
  Load {
    path: PathBuf,
    io_error: io::Error,
  },
  MissingCommand {
    task_name: String,
  },
  MissingTask {
    task_name: String,
  },
  Parse {
    path: PathBuf,
    toml_error: TomlError,
  },
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    use Error::*;

    write!(f, "error: ")?;

    match self {
      CommandInvoke { command, io_error } => {
        write!(f, "Failed to invoke {command}: {io_error}")?;
      }
      CommandStatus { command, status } => {
        write!(f, "Command {command} failed ({status})")?;
      }
      DependencyCycle { task_name, cycle } => {
        write!(
          f,
          "Dependency cycle detected: {} -> {}",
          cycle.join(" -> "),
          task_name
        )?;
      }
      Load { path, io_error } => {
        write!(
          f,
          "Failed to read Grint.toml at `{}`: {io_error}",
          path.display()
        )?;
      }
      MissingCommand { task_name } => {
        write!(f, "Task `{task_name}` is missing required `cmd` attribute")?;
      }
      MissingTask { task_name } => {
        write!(f, "Task `{task_name}` not found")?;
      }
      Parse { path, toml_error } => {
        write!(
          f,
          "Failed to parse Grint.toml at `{}`: {toml_error}",
          path.display()
        )?;
      }
    }

    Ok(())
  }
}
