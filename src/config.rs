use std::path::PathBuf;

use clap::{Arg, ArgAction, ArgGroup, ArgMatches, Command, value_parser};

use crate::{Error, Subcommand};

#[derive(Debug)]
pub(crate) struct Config {
  pub(crate) grintfile: Option<PathBuf>,
  pub(crate) shell: Option<String>,
  pub(crate) shell_args: Option<Vec<String>>,
  pub(crate) subcommand: Subcommand,
}

mod cmd {
  pub(crate) const EDIT: &str = "EDIT";
  pub(crate) const LIST: &str = "LIST";

  pub(crate) const ALL: &[&str] = &[EDIT, LIST];
  pub(crate) const HEADING: &str = "Commands";
}

mod arg {
  pub(crate) const ARGUMENTS: &str = "ARGUMENTS";
  pub(crate) const GRINTFILE: &str = "GRINTFILE";
  pub(crate) const SHELL: &str = "SHELL";
  pub(crate) const SHELL_ARG: &str = "SHELL_ARG";
}

impl Config {
  pub(crate) fn app() -> Command {
    Command::new(env!("CARGO_PKG_NAME"))
      .version(env!("CARGO_PKG_VERSION"))
      .about(env!("CARGO_PKG_DESCRIPTION"))
      .arg(
        Arg::new(arg::GRINTFILE)
          .short('f')
          .long("grintfile")
          .action(ArgAction::Set)
          .value_parser(value_parser!(PathBuf))
          .help("Use <GRINTFILE> as grintfile"),
      )
      .arg(
        Arg::new(arg::SHELL)
          .long("shell")
          .action(ArgAction::Set)
          .help("Invoke <SHELL> to run tasks"),
      )
      .arg(
        Arg::new(arg::SHELL_ARG)
          .long("shell-arg")
          .action(ArgAction::Append)
          .allow_hyphen_values(true)
          .help("Invoke shell with <SHELL-ARG> as an argument"),
      )
      .arg(
        Arg::new(cmd::EDIT)
          .short('e')
          .long("edit")
          .action(ArgAction::SetTrue)
          .help("Edit Grint.toml with editor given by $VISUAL or $EDIT, falls back to `vim`")
          .help_heading(cmd::HEADING),
      )
      .arg(
        Arg::new(cmd::LIST)
          .short('l')
          .long("list")
          .action(ArgAction::SetTrue)
          .conflicts_with(arg::ARGUMENTS)
          .help("List available tasks")
          .help_heading(cmd::HEADING),
      )
      .group(ArgGroup::new("SUBCOMMAND").args(cmd::ALL))
      .arg(
        Arg::new(arg::ARGUMENTS)
          .num_args(1..)
          .action(ArgAction::Append)
          .help("Task(s) to run"),
      )
  }

  pub(crate) fn from_matches(matches: &ArgMatches) -> Result<Self, Error> {
    let subcommand = if matches.get_flag(cmd::EDIT) {
      Subcommand::Edit
    } else if matches.get_flag(cmd::LIST) {
      Subcommand::List
    } else {
      let mut arguments = Vec::new();
      let values = matches.get_many::<String>(arg::ARGUMENTS);

      if let Some(values) = values {
        for value in values {
          arguments.push(value.to_owned());
        }
      }

      Subcommand::Run { arguments }
    };

    Ok(Self {
      grintfile: matches.get_one::<PathBuf>(arg::GRINTFILE).map(Into::into),
      shell: matches.get_one::<String>(arg::SHELL).map(Into::into),
      shell_args: matches
        .get_many::<String>(arg::SHELL_ARG)
        .map(|s| s.map(Into::into).collect()),
      subcommand,
    })
  }
}
