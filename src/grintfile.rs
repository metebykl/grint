use std::collections::HashSet;
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
        let body = entry
          .get("cmd")
          .and_then(|v| v.as_str())
          .ok_or(format!("Missing cmd for task {}", name))?
          .to_string();

        let dependencies: Vec<String> = entry
          .get("deps")
          .and_then(|v| v.as_array())
          .map(|a| {
            a.iter()
              .filter_map(|v| v.as_str())
              .map(|s| s.to_string())
              .collect()
          })
          .unwrap_or_else(Vec::new);

        let desc = entry
          .get("desc")
          .and_then(|v| v.as_str())
          .map(|s| s.to_string());

        let working_directory = entry
          .get("cwd")
          .and_then(|v| v.as_str())
          .map(|s| PathBuf::from(s));

        let task = Task {
          body,
          dependencies,
          desc,
          name: name.to_string(),
          working_directory,
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
    let mut ran = HashSet::new();

    for name in arguments {
      if !ran.contains(name) {
        self.run_task_with_dependencies(config, name, &mut ran, &mut Vec::new())?;
      }
    }

    Ok(())
  }

  fn run_task_with_dependencies(
    &self,
    config: &Config,
    name: &str,
    ran: &mut HashSet<String>,
    path: &mut Vec<String>,
  ) -> Result<(), Box<dyn std::error::Error>> {
    let task = self
      .tasks
      .get(name)
      .ok_or_else(|| format!("task '{}' not found", name))?;

    if let Some(cycle_start) = path.iter().position(|n| n == name) {
      let cycle: Vec<&str> = path[cycle_start..].iter().map(|s| s.as_str()).collect();
      return Err(
        format!(
          "dependency cycle detected: {} -> {}",
          cycle.join(" -> "),
          name
        )
        .into(),
      );
    }

    if ran.contains(name) {
      return Ok(());
    }

    path.push(name.to_string());

    for dependency in &task.dependencies {
      self.run_task_with_dependencies(config, dependency, ran, path)?;
    }

    path.pop();

    if !ran.contains(name) {
      task.run(config, self)?;
      ran.insert(name.to_string());
    }

    Ok(())
  }
}
