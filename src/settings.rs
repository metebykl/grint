use std::process::Command;

use crate::Config;

pub(crate) const UNIX_SHELL: &str = "sh";
pub(crate) const UNIX_SHELL_ARGS: &[&str] = &["-cu"];
pub(crate) const WINDOWS_POWERSHELL_SHELL: &str = "powershell.exe";
pub(crate) const WINDOWS_POWERSHELL_ARGS: &[&str] = &["-NoLogo", "-Command"];

#[derive(Debug)]
pub(crate) struct Settings {}

impl Settings {
  pub(crate) fn new() -> Self {
    Self {}
  }

  pub(crate) fn shell_command(&self, config: &Config) -> Command {
    let (command, args) = self.shell(config);

    let mut cmd = Command::new(command);
    cmd.args(args);

    cmd
  }

  pub(crate) fn shell<'a>(&'a self, config: &'a Config) -> (&'a str, Vec<&'a str>) {
    match (&config.shell, &config.shell_args) {
      (Some(shell), Some(shell_args)) => (shell, shell_args.iter().map(String::as_ref).collect()),
      (Some(shell), None) => (shell, UNIX_SHELL_ARGS.to_vec()),
      (None, Some(shell_args)) => (UNIX_SHELL, shell_args.iter().map(String::as_ref).collect()),
      (None, None) => {
        if cfg!(windows) {
          (WINDOWS_POWERSHELL_SHELL, WINDOWS_POWERSHELL_ARGS.to_vec())
        } else {
          (UNIX_SHELL, UNIX_SHELL_ARGS.to_vec())
        }
      }
    }
  }
}
