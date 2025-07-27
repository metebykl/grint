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
    let max_name_width = grintfile.tasks.keys().map(|v| v.len()).max().unwrap_or(0);

    println!("Available tasks:");
    for (name, task) in &grintfile.tasks {
      print!("  {:<max_name_width$}", name);

      if let Some(desc) = &task.desc {
        println!("  # {}", desc);
      } else {
        println!();
      }
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
