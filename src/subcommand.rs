use crate::Config;

#[derive(Debug)]
pub(crate) enum Subcommand {
  List,
  Run { arguments: Vec<String> },
}

impl Subcommand {
  pub(crate) fn execute(&self, config: &Config) {
    match self {
      Self::List => {
        Self::list(config);
      }
      Self::Run { arguments } => {
        Self::run(config, arguments);
      }
    }
  }

  fn list(config: &Config) {
    println!("List");
  }

  fn run(config: &Config, arguments: &[String]) {
    println!("Run: {}", arguments.join(","));
  }
}
