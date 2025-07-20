use std::env;

use crate::{Config, Grintfile};

#[derive(Debug)]
pub(crate) enum Subcommand {
  List,
  Run { arguments: Vec<String> },
}

impl Subcommand {
  pub(crate) fn execute(&self, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    match self {
      Self::List => Self::list(config),
      Self::Run { arguments } => Self::run(config, arguments),
    }
  }

  fn list(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    println!("List");
    Ok(())
  }

  fn run(config: &Config, arguments: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let grintfile_path = match &config.grintfile {
      Some(g) => g.to_owned(),
      None => {
        let cwd = env::current_dir()?;
        cwd.join("Grint.toml")
      }
    };

    let grintfile = Grintfile::parse(grintfile_path)?;
    grintfile.run(arguments)
  }
}
