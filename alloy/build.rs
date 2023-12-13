use std::io::Write;
use std::path::{Path, PathBuf};

pub fn main() {
  let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());

  generate_version(&out_dir)
}

fn generate_version(out_dir: &Path) {
  let major = std::env::var("CARGO_PKG_VERSION_MAJOR").unwrap();
  let minor = std::env::var("CARGO_PKG_VERSION_MINOR").unwrap();
  let patch = std::env::var("CARGO_PKG_VERSION_PATCH").unwrap();

  let mut file =
    std::fs::File::create(out_dir.join("version.rs")).expect("error creating version.rs file");

  write!(
    file,
    "Version{{major: {}, minor: {}, patch: {}}}",
    major, minor, patch
  )
  .expect("unable to write version.rs");
}
