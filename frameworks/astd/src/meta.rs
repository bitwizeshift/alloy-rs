//! The [`meta`] module provides details about the binary being compiled.
//!
//! [`meta`]: crate::meta

/// The full version of your package.
#[macro_export]
#[doc(hidden)]
macro_rules! cargo_pkg_version {
  () => {
    env!("CARGO_PKG_VERSION")
  };
}

/// The major version of your package.
#[macro_export]
#[doc(hidden)]
macro_rules! cargo_pkg_version_major {
  () => {
    env!("CARGO_PKG_VERSION_MAJOR")
  };
}

/// The minor version of your package.
#[macro_export]
#[doc(hidden)]
macro_rules! cargo_pkg_version_minor {
  () => {
    env!("CARGO_PKG_VERSION_MINOR")
  };
}

/// The patch version of your package.
#[macro_export]
#[doc(hidden)]
macro_rules! cargo_pkg_version_patch {
  () => {
    env!("CARGO_PKG_VERSION_PATCH")
  };
}

/// The pre-release version of your package.
#[macro_export]
#[doc(hidden)]
macro_rules! cargo_pkg_version_pre {
  () => {
    env!("CARGO_PKG_VERSION_PRE")
  };
}

/// Colon separated list of authors from the manifest of your package.
#[macro_export]
#[doc(hidden)]
macro_rules! cargo_pkg_authors {
  () => {
    env!("CARGO_PKG_AUTHORS")
  };
}

/// The name of your package.
#[macro_export]
#[doc(hidden)]
macro_rules! cargo_pkg_name {
  () => {
    env!("CARGO_PKG_NAME")
  };
}

/// The description from the manifest of your package.
#[macro_export]
#[doc(hidden)]
macro_rules! cargo_pkg_description {
  () => {
    env!("CARGO_PKG_DESCRIPTION")
  };
}

/// The home page from the manifest of your package.
#[macro_export]
#[doc(hidden)]
macro_rules! cargo_pkg_homepage {
  () => {
    env!("CARGO_PKG_HOMEPAGE")
  };
}

/// The repository from the manifest of your package.
#[macro_export]
#[doc(hidden)]
macro_rules! cargo_pkg_repository {
  () => {
    env!("CARGO_PKG_REPOSITORY")
  };
}

/// The license from the manifest of your package.
#[macro_export]
#[doc(hidden)]
macro_rules! cargo_pkg_license {
  () => {
    env!("CARGO_PKG_LICENSE")
  };
}

/// The license file from the manifest of your package.
#[macro_export]
#[doc(hidden)]
macro_rules! cargo_pkg_license_file {
  () => {
    env!("CARGO_PKG_LICENSE_FILE")
  };
}

#[doc(inline)]
pub use cargo_pkg_version;

#[doc(inline)]
pub use cargo_pkg_version_major;

#[doc(inline)]
pub use cargo_pkg_version_minor;

#[doc(inline)]
pub use cargo_pkg_version_patch;

#[doc(inline)]
pub use cargo_pkg_version_pre;

#[doc(inline)]
pub use cargo_pkg_authors;

#[doc(inline)]
pub use cargo_pkg_name;

#[doc(inline)]
pub use cargo_pkg_description;

#[doc(inline)]
pub use cargo_pkg_homepage;

#[doc(inline)]
pub use cargo_pkg_repository;

#[doc(inline)]
pub use cargo_pkg_license;

#[doc(inline)]
pub use cargo_pkg_license_file;

/// Representation of a semantic-version in `(major, minor, patch)` notation.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Version {
  /// The major version component
  pub major: u16,
  /// The minor version component
  pub minor: u16,
  /// The patch version component
  pub patch: u16,
}

impl std::fmt::Display for Version {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
  }
}

impl std::fmt::Debug for Version {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    <Self as std::fmt::Display>::fmt(self, f)
  }
}

/// Gets the current crate's version information from the meta cargo manifest.
#[macro_export]
#[doc(hidden)]
macro_rules! crate_version {
  () => {
    $crate::meta::Version {
      major: $crate::u16::from_str_radix10($crate::meta::cargo_pkg_version_major!()),
      minor: $crate::u16::from_str_radix10($crate::meta::cargo_pkg_version_minor!()),
      patch: $crate::u16::from_str_radix10($crate::meta::cargo_pkg_version_patch!()),
    }
  };
}

#[doc(inline)]
pub use crate_version;
