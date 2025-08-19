use std::ffi::OsString;
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
  EditorInvoke {
    editor: OsString,
    io_error: io::Error,
  },
  EditorStatus {
    editor: OsString,
    status: ExitStatus,
  },
  DependencyCycle {
    task: String,
    cycle: Vec<String>,
  },
  Load {
    path: PathBuf,
    io_error: io::Error,
  },
  MissingCommand {
    task: String,
  },
  Parse {
    path: PathBuf,
    toml_error: TomlError,
  },
  UnknownTask {
    task: String,
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
      DependencyCycle { task, cycle } => {
        write!(
          f,
          "Dependency cycle detected: {} -> {task}",
          cycle.join(" -> ")
        )?;
      }
      EditorInvoke { editor, io_error } => {
        let editor = editor.to_string_lossy();
        write!(f, "Editor `{editor}` invocation failed: {io_error}")?;
      }
      EditorStatus { editor, status } => {
        let editor = editor.to_string_lossy();
        write!(f, "Editor `{editor}` failed: {status}")?;
      }
      Load { path, io_error } => {
        write!(
          f,
          "Failed to read Grint.toml at `{}`: {io_error}",
          path.display()
        )?;
      }
      MissingCommand { task } => {
        write!(f, "Task `{task}` is missing required `cmd` attribute")?;
      }
      Parse { path, toml_error } => {
        write!(
          f,
          "Failed to parse Grint.toml at `{}`: {toml_error}",
          path.display()
        )?;
      }
      UnknownTask { task } => {
        write!(f, "Task `{task}` not found")?;
      }
    }

    Ok(())
  }
}
