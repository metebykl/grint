use std::path::PathBuf;
use std::process::Command;

pub(crate) struct Task {
  pub(crate) name: String,
  pub(crate) desc: Option<String>,
  pub(crate) cwd: Option<PathBuf>,
  pub(crate) body: String,
}

impl Task {
  pub(crate) fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("> {}", self.body);

    let mut command = if cfg!(target_os = "windows") {
      let mut cmd = Command::new("cmd");
      cmd.arg("/C").arg(&self.body);
      cmd
    } else {
      let mut cmd = Command::new("sh");
      cmd.arg("-c").arg(&self.body);
      cmd
    };

    if let Some(cwd) = &self.cwd {
      command.current_dir(cwd);
    }

    let status = command.status()?;

    if !status.success() {
      return Err(
        format!(
          "task '{}' failed with exit code {}",
          self.name,
          status.code().unwrap_or(1)
        )
        .into(),
      );
    }

    Ok(())
  }
}
