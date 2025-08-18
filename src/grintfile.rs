use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use indexmap::IndexMap;
use toml_edit::DocumentMut;

use crate::{Config, Error, Settings, Task};

pub(crate) struct Grintfile {
  pub(crate) settings: Settings,
  pub(crate) tasks: IndexMap<String, Task>,
}

impl Grintfile {
  pub(crate) fn parse(path: &Path) -> Result<Self, Error> {
    let data = fs::read_to_string(path).map_err(|io_error| Error::Load {
      path: path.to_owned(),
      io_error,
    })?;
    let doc = data
      .parse::<DocumentMut>()
      .map_err(|toml_error| Error::Parse {
        path: path.to_owned(),
        toml_error,
      })?;

    let mut tasks: IndexMap<String, Task> = IndexMap::new();
    if let Some(table) = doc.get("task").and_then(|v| v.as_table()) {
      for (name, entry) in table.iter() {
        let body = entry
          .get("cmd")
          .and_then(|v| v.as_str())
          .ok_or_else(|| Error::MissingCommand {
            task: name.to_owned(),
          })?
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

        let env: HashMap<String, String> = entry
          .get("env")
          .and_then(|v| v.as_inline_table())
          .map(|it| {
            it.iter()
              .filter_map(|(k, v)| v.as_str().map(|s| (k.to_string(), s.to_string())))
              .collect()
          })
          .unwrap_or_else(HashMap::new);

        let working_directory = entry.get("cwd").and_then(|v| v.as_str()).map(PathBuf::from);

        let task = Task {
          body,
          dependencies,
          desc,
          env,
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

  pub(crate) fn run(&self, config: &Config, arguments: &[String]) -> Result<(), Error> {
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
  ) -> Result<(), Error> {
    let task = self.tasks.get(name).ok_or_else(|| Error::UnknownTask {
      task: name.to_owned(),
    })?;

    if let Some(cycle_start) = path.iter().position(|n| n == name) {
      let cycle: Vec<String> = path[cycle_start..].to_vec();
      return Err(Error::DependencyCycle {
        task: name.to_owned(),
        cycle,
      });
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
