use std::fs;
use std::path::Path;

use indexmap::IndexMap;
use toml_edit::DocumentMut;

use crate::Task;

pub(crate) struct Grintfile {
  pub(crate) tasks: IndexMap<String, Task>,
}

impl Grintfile {
  pub(crate) fn parse<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(path)?;
    let doc = data.parse::<DocumentMut>()?;

    let mut tasks: IndexMap<String, Task> = IndexMap::new();
    if let Some(table) = doc.get("task").and_then(|v| v.as_table()) {
      for (name, entry) in table.iter() {
        let body = entry
          .get("cmd")
          .and_then(|v| v.as_str())
          .ok_or(format!("Missing cmd for task {}", name))?
          .to_string();

        let task = Task {
          name: name.to_string(),
          body,
        };

        tasks.insert(name.to_owned(), task);
      }
    }

    Ok(Self { tasks })
  }

  pub(crate) fn run(&self, arguments: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    for name in arguments {
      self
        .tasks
        .get(name)
        .ok_or(format!("Task {} not found", name))?
        .run()?;
    }

    Ok(())
  }
}
