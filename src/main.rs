fn main() {
  if let Err(code) = grint::run() {
    std::process::exit(code);
  }
}
