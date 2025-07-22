use std::env;
use std::fmt::Debug;
use std::path::PathBuf;

use pretty_assertions::{Comparison, StrComparison};

pub(crate) fn compare<T: PartialEq + Debug>(name: &str, have: T, want: T) -> bool {
  let equal = have == want;
  if !equal {
    eprintln!("Bad {name}: {}", Comparison::new(&have, &want));
  }
  equal
}

pub(crate) fn compare_string(name: &str, have: &str, want: &str) -> bool {
  let equal = have == want;
  if !equal {
    eprintln!("Bad {name}: {}", StrComparison::new(&have, &want));
  }
  equal
}

pub(crate) fn executable_path(name: &str) -> PathBuf {
  let mut path = env::current_exe().unwrap();
  path.pop();

  if path.ends_with("deps") {
    path.pop();
  }

  let exe = String::from(name) + env::consts::EXE_SUFFIX;
  path.push(exe);

  path
}
