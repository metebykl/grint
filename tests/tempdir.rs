use super::*;

pub(crate) fn tempdir() -> TempDir {
  tempfile::tempdir().expect("failed to create temporary directory")
}
