//! The `meta` module provides meta-information about the alloy engine that was
//! queried at build-time.

/// A utility type that represents a semantic version.
///
/// This type is used for representing Alloy meta-information such as the
/// engine [`VERSION`].
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Version {
  /// The major version component
  pub major: u32,
  /// The minor version component
  pub minor: u32,
  /// The patch version component
  pub patch: u32,
}

impl Default for Version {
  fn default() -> Self {
    Self {
      major: 0,
      minor: 1,
      patch: 0,
    }
  }
}

impl std::fmt::Display for Version {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
  }
}

/// The version that this engine was built at, as specified in the Cargo.toml
/// manifest.
pub const VERSION: Version = include!(concat!(env!("OUT_DIR"), "/version.rs"));
