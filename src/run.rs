use crate::Config;

pub fn run() -> Result<(), i32> {
  let app = Config::app();

  let matches = app.try_get_matches().map_err(|err| {
    err.print().ok();
    err.exit_code()
  })?;

  let config = Config::from_matches(&matches).map_err(|err| {
    eprintln!("{}", err);
    1
  })?;

  config.subcommand.execute(&config).map_err(|err| {
    eprintln!("{}", err);
    1
  })?;

  Ok(())
}
