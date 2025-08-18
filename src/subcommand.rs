use std::env;
use std::path::Path;
use std::process::Command;

use crate::{Config, Error, Grintfile};

#[derive(Debug)]
pub(crate) enum Subcommand {
  Edit,
  List,
  Run { arguments: Vec<String> },
  Show { task: String },
}

impl Subcommand {
  pub(crate) fn execute(&self, config: &Config) -> Result<(), Error> {
    let grintfile_path = match &config.grintfile {
      Some(g) => g.as_path(),
      None => Path::new("Grint.toml"),
    };

    let grintfile = Grintfile::parse(grintfile_path)?;

    match self {
      Self::Edit => Self::edit(grintfile_path),
      Self::List => Self::list(&grintfile),
      Self::Run { arguments } => Self::run(config, &grintfile, arguments),
      Self::Show { task } => Self::show(&grintfile, task),
    }
  }

  fn edit(grintfile_path: &Path) -> Result<(), Error> {
    let editor = env::var_os("VISUAL")
      .or_else(|| env::var_os("EDITOR"))
      .unwrap_or_else(|| "vim".into());

    let status = match Command::new(&editor).arg(grintfile_path).status() {
      Ok(status) => status,
      Err(io_error) => return Err(Error::EditorInvoke { editor, io_error }),
    };

    if !status.success() {
      return Err(Error::EditorStatus { editor, status });
    }

    Ok(())
  }

  fn list(grintfile: &Grintfile) -> Result<(), Error> {
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

  fn run(config: &Config, grintfile: &Grintfile, arguments: &[String]) -> Result<(), Error> {
    grintfile.run(config, arguments)
  }

  fn show(grintfile: &Grintfile, task: &str) -> Result<(), Error> {
    let task = grintfile
      .tasks
      .get(task)
      .ok_or_else(|| Error::UnknownTask {
        task: task.to_owned(),
      })?;

    println!("{}", task);
    Ok(())
  }
}
