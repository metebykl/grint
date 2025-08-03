use std::fs;
use std::path::{Path, PathBuf};

use indexmap::IndexMap;
use toml_edit::DocumentMut;

use crate::{Config, Settings, Task};

pub(crate) struct Grintfile {
  pub(crate) settings: Settings,
  pub(crate) tasks: IndexMap<String, Task>,
}

impl Grintfile {
  pub(crate) fn parse<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(path)?;
    let doc = data.parse::<DocumentMut>()?;

    let mut tasks: IndexMap<String, Task> = IndexMap::new();
    if let Some(table) = doc.get("task").and_then(|v| v.as_table()) {
      for (name, entry) in table.iter() {
        let desc = entry
          .get("desc")
          .and_then(|v| v.as_str())
          .map(|s| s.to_string());

        let cwd = entry
          .get("cwd")
          .and_then(|v| v.as_str())
          .map(|s| PathBuf::from(s));

        let body = entry
          .get("cmd")
          .and_then(|v| v.as_str())
          .ok_or(format!("Missing cmd for task {}", name))?
          .to_string();

        let task = Task {
          name: name.to_string(),
          cwd,
          desc,
          body,
        };

        tasks.insert(name.to_owned(), task);
      }
    }

    Ok(Self {
      settings: Settings::new(),
      tasks,
    })
  }

  pub(crate) fn run(
    &self,
    config: &Config,
    arguments: &[String],
  ) -> Result<(), Box<dyn std::error::Error>> {
    for name in arguments {
      self
        .tasks
        .get(name)
        .ok_or(format!("Task {} not found", name))?
        .run(config, self)?;
    }

    Ok(())
  }
}
