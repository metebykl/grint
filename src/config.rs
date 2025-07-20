use std::path::PathBuf;

use clap::{Arg, ArgAction, ArgGroup, ArgMatches, Command, value_parser};

use crate::Subcommand;

#[derive(Debug)]
pub(crate) struct Config {
  pub(crate) grintfile: Option<PathBuf>,
  pub(crate) subcommand: Subcommand,
}

mod cmd {
  pub(crate) const LIST: &str = "LIST";

  pub(crate) const ALL: &[&str] = &[LIST];
}

mod arg {
  pub(crate) const ARGUMENTS: &str = "ARGUMENTS";
  pub(crate) const GRINTFILE: &str = "GRINTFILE";
}

impl Config {
  pub(crate) fn app() -> Command {
    Command::new("grint")
      .about("Modern, declarative build tool")
      .arg(
        Arg::new(arg::GRINTFILE)
          .short('f')
          .long("grintfile")
          .action(ArgAction::Set)
          .value_parser(value_parser!(PathBuf))
          .help("Use <GRINTFILE> as grintfile"),
      )
      .arg(
        Arg::new(cmd::LIST)
          .short('l')
          .long("list")
          .action(ArgAction::SetTrue)
          .conflicts_with(arg::ARGUMENTS)
          .help("List available tasks"),
      )
      .group(ArgGroup::new("SUBCOMMAND").args(cmd::ALL))
      .arg(
        Arg::new(arg::ARGUMENTS)
          .num_args(1..)
          .action(ArgAction::Append)
          .help("Task(s) to run"),
      )
  }

  pub(crate) fn from_matches(matches: &ArgMatches) -> Result<Self, Box<dyn std::error::Error>> {
    let subcommand = if matches.get_flag(cmd::LIST) {
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
      subcommand,
    })
  }
}
