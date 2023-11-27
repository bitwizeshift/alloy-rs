//! A small crate used for supplying helpers to build.rs scripts.
//!
//! This just simplifies writing `build.rs` scripts and helps to make them more
//! generally readable.
//!
//! This crate should not ever be included as a `[dependency]`.
#![deny(missing_docs)]
#![deny(unused_results)]
#![deny(rust_2018_idioms)]

/// This macro adds the string argument as a `rustc-link-search` argument.
#[macro_export]
macro_rules! rustc_link_search {
  ($($tokens:tt)*) => {
    println!("cargo:rustc-link-search={}", ::std::format!($($tokens)*))
  };
}

/// This macro adds the string argument as a `rustc-link-lib` argument.
#[macro_export]
macro_rules! rustc_link_lib {
  ($($tokens:tt)*) => {
    println!("cargo:rustc-link-lib={}", ::std::format!($($tokens)*))
  };
}

/// This macro adds the string argument as a `rerun-if-changed` argument, which
/// tells Cargo to invalidate its cache on file changes.
#[macro_export]
macro_rules! rerun_if_changed {
  ($($tokens:tt)*) => {
    println!("cargo:rerun-if-changed={}", ::std::format!($($tokens)*))
  };
}

/// A module that is only available when building for Apple devices.
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub mod apple {
  use std::process::Stdio;

  /// The explicit Xcode SDK to use.
  pub enum SDK {
    /// SDK for the MacOS
    MacOSX,

    /// SDK for the iPhone
    Phone,

    /// SDK for the iPhone simulator
    Simulator,
  }

  impl SDK {
    fn suffix(&self) -> &str {
      match self {
        SDK::MacOSX => "MacOSX.platform/Developer/SDKs/MacOSX.sdk",
        SDK::Phone => "iPhoneOS.platform/Developer/SDKs/iPhoneOS.sdk",
        SDK::Simulator => "iPhoneSimulator.platform/Developer/SDKs/iPhoneSimulator.sdk",
      }
    }
  }

  /// Gets the underlying sysroot from the specified SDK.
  ///
  /// This function will panic if `xcode-select` is not found, or if
  /// `xcode-select` fails to find a path.
  ///
  /// # Arguments
  ///
  /// * `sdk` - the SDK to use
  pub fn xcode_sysroot(sdk: SDK) -> String {
    use std::process::Command;

    let child = Command::new("xcode-select")
      .arg("--print-path")
      .stdout(Stdio::piped())
      .spawn()
      .expect("xcode-select command failed to start");
    let output = child
      .wait_with_output()
      .expect("failed to output xcode path");
    let path = String::from_utf8(output.stdout).expect("failed to parse output");

    format!("{}/Platforms/{}", path.trim(), sdk.suffix())
  }

  /// Gets the Xcode sysroot the target_os.
  ///
  /// This will always choose the most up-to-date SDK installed on the system.
  pub fn target_xcode_sysroot() -> String {
    impl_target_xcode_sysroot()
  }

  #[cfg(target_os = "macos")]
  fn impl_target_xcode_sysroot() -> String {
    xcode_sysroot(SDK::MacOSX)
  }

  #[cfg(target_os = "ios")]
  fn impl_target_xcode_sysroot() -> String {
    xcode_sysroot(SDK::Phone)
  }
}
