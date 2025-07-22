use std::path::Path;

use crate::{Config, Grintfile};

#[derive(Debug)]
pub(crate) enum Subcommand {
  List,
  Run { arguments: Vec<String> },
}

impl Subcommand {
  pub(crate) fn execute(&self, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let grintfile_path = match &config.grintfile {
      Some(g) => g.as_path(),
      None => Path::new("Grint.toml"),
    };

    let grintfile = Grintfile::parse(grintfile_path)?;

    match self {
      Self::List => Self::list(config, &grintfile),
      Self::Run { arguments } => Self::run(config, &grintfile, arguments),
    }
  }

  fn list(config: &Config, grintfile: &Grintfile) -> Result<(), Box<dyn std::error::Error>> {
    println!("Available tasks:");
    for (name, _) in &grintfile.tasks {
      println!("  {}", name);
    }
    Ok(())
  }

  fn run(
    config: &Config,
    grintfile: &Grintfile,
    arguments: &[String],
  ) -> Result<(), Box<dyn std::error::Error>> {
    grintfile.run(arguments)
  }
}
