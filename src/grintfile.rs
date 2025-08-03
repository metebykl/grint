use std::collections::{HashSet, VecDeque};
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
    let mut queue: VecDeque<String> = arguments.iter().cloned().collect();

    while let Some(name) = queue.pop_front() {
      if ran.contains(&name) {
        continue;
      }

      let mut stack = vec![name.clone()];
      let mut visited = HashSet::new();
      visited.insert(name.clone());

      while let Some(current_name) = stack.last().cloned() {
        let task = self
          .tasks
          .get(&current_name)
          .ok_or_else(|| format!("task '{}' not found", current_name))?;

        let mut all_dependencies_ran = true;
        for dependency in &task.dependencies {
          if ran.contains(dependency) {
            continue;
          }

          if visited.contains(dependency) {
            return Err(format!("detected dependency cycle involving '{}'", dependency).into());
          }

          visited.insert(dependency.clone());
          stack.push(dependency.clone());
          all_dependencies_ran = false;
        }

        if all_dependencies_ran {
          stack.pop();
          if !ran.contains(&current_name) {
            task.run(config, self)?;
            ran.insert(current_name);
          }
        }
      }
    }

    Ok(())
  }
}
