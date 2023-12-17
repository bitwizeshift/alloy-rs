//! A small crate used for supplying helpers to build.rs scripts.
//!
//! This just simplifies writing `build.rs` scripts and helps to make them more
//! generally readable.
//!
//! This crate should not ever be included as a `[dependency]`.
#![deny(missing_docs)]
#![deny(unused_results)]
#![deny(rust_2018_idioms)]

/// The [`warning`]` instruction tells Cargo to display a warning after the
/// build script has finished running. Warnings are only shown for path
/// dependencies (that is, those you’re working on locally), so for example
/// warnings printed out in crates.io crates are not emitted by default. The
/// `-vv` “very verbose” flag may be used to have Cargo display warnings for all
/// crates.
///
/// For more information, see this [cargo reference].
///
/// [cargo reference]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargo-warning
#[macro_export]
macro_rules! warning {
  ($($tokens:tt)*) => {
    println!("cargo:warning={}", ::std::format!($($tokens)*))
  }
}

/// The [`rustc_link_search`] instruction tells Cargo to pass the -L flag to the
/// compiler to add a directory to the library search path.
///
/// The optional KIND may be one of dependency, crate, native, framework, or
/// all. See the rustc book for more detail.
///
/// These paths are also added to the dynamic library search path environment
/// variable if they are within the OUT_DIR. Depending on this behavior is
/// discouraged since this makes it difficult to use the resulting binary. In
/// general, it is best to avoid creating dynamic libraries in a build script
/// (using existing system libraries is fine).
///
/// For more information, see this [cargo reference].
///
/// [cargo reference]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-search
#[macro_export]
macro_rules! rustc_link_search {
  ($($tokens:tt)*) => {
    println!("cargo:rustc-link-search={}", ::std::format!($($tokens)*))
  };
}

/// The [`rustc_link_lib`] instruction tells Cargo to link the given library using
/// the compiler’s `-l` flag. This is typically used to link a native library
/// using FFI.
///
/// The LIB string is passed directly to rustc, so it supports any syntax that
/// `-l` does.
/// Currently the full supported syntax for `LIB` is
/// `[KIND[:MODIFIERS]=]NAME[:RENAME]`.
///
/// The -l flag is only passed to the library target of the package, unless
/// there is no library target, in which case it is passed to all targets.
/// This is done because all other targets have an implicit dependency on the
/// library target, and the given library to link should only be included once.
/// This means that if a package has both a library and a binary target, the
/// library has access to the symbols from the given lib, and the binary should
/// access them through the library target’s public API.
///
/// The optional KIND may be one of dylib, static, or framework. See the rustc
/// book for more detail.
///
/// For more information, see this [cargo reference].
///
/// [cargo reference]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-lib
#[macro_export]
macro_rules! rustc_link_lib {
  ($($tokens:tt)*) => {
    println!("cargo:rustc-link-lib={}", ::std::format!($($tokens)*))
  };
}

/// The [`rerun_if_changed`] instruction tells Cargo to re-run the build script if
/// the file at the given path has changed. Currently, Cargo only uses the
/// filesystem last-modified “mtime” timestamp to determine if the file has
/// changed. It compares against an internal cached timestamp of when the build
/// script last ran.
///
/// If the path points to a directory, it will scan the entire directory for any
/// modifications.
///
/// If the build script inherently does not need to re-run under any circumstance,
/// then emitting cargo:rerun-if-changed=build.rs is a simple way to prevent it
/// from being re-run (otherwise, the default if no rerun-if instructions are
/// emitted is to scan the entire package directory for changes). Cargo
/// automatically handles whether or not the script itself needs to be
/// recompiled, and of course the script will be re-run after it has been
/// recompiled. Otherwise, specifying build.rs is redundant and unnecessary.
///
/// For more information, see this [cargo reference].
///
/// [cargo reference]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed
#[macro_export]
macro_rules! rerun_if_changed {
  ($($tokens:tt)*) => {
    println!("cargo:rerun-if-changed={}", ::std::format!($($tokens)*))
  };
}

/// The [`rerun_if_env_changed`] instruction tells Cargo to re-run the build
/// script if the value of an environment variable of the given name has changed.
///
/// Note that the environment variables here are intended for global environment
/// variables like CC and such, it is not possible to use this for environment
/// variables like TARGET that Cargo sets for build scripts. The environment
/// variables in use are those received by cargo invocations, not those received
/// by the executable of the build script.
///
/// For more information, see this [cargo reference].
///
/// [cargo reference]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-env-changed
#[macro_export]
macro_rules! rerun_if_env_changed {
  ($($tokens:tt)*) => {
    println!("cargo:rerun-if-env-changed={}", ::std::format!($($tokens)*))
  };
}

/// The [`rustc_link_arg_bin`] instruction tells Cargo to pass the
/// `-C link-arg=FLAG` option to the compiler, but only when building the binary
/// target with name BIN. Its usage is highly platform specific. It is useful
/// to set a linker script or other linker options.
///
/// For more information, see this [cargo reference].
///
/// [cargo reference]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg-bin
#[macro_export]
macro_rules! rustc_link_arg_bin {
  ($($tokens:tt)*) => {
    println!("cargo:rustc-link-arg-bin={}", ::std::format!($($tokens)*))
  };
}

/// The [`rustc_link_arg_bins`] instruction tells Cargo to pass the
/// `-C link-arg=FLAG` option to the compiler, but only when building a binary
/// target. Its usage is highly platform specific. It is useful to set a linker
/// script or other linker options.
///
/// For more information, see this [cargo reference].
///
/// [cargo reference]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg-bins
#[macro_export]
macro_rules! rustc_link_arg_bins {
  ($($tokens:tt)*) => {
    println!("cargo:rustc-link-arg-bins={}", ::std::format!($($tokens)*))
  };
}

/// The [`rustc_link_arg_tests`] instruction tells Cargo to pass the
/// `-C link-arg=FLAG` option to the compiler, but only when building a tests
/// target.
///
/// For more information, see this [cargo reference].
///
/// [cargo reference]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg-tests
#[macro_export]
macro_rules! rustc_link_arg_tests {
  ($($tokens:tt)*) => {
    println!("cargo:rustc-link-arg-tests={}", ::std::format!($($tokens)*))
  };
}

/// The [`rustc_link_arg_examples`] instruction tells Cargo to pass the
/// `-C link-arg=FLAG` option to the compiler, but only when building an
/// examples target.
///
/// For more information, see this [cargo reference].
///
/// [cargo reference]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg-examples
#[macro_export]
macro_rules! rustc_link_arg_examples {
  ($($tokens:tt)*) => {
    println!("cargo:rustc-link-arg-examples={}", ::std::format!($($tokens)*))
  };
}

/// The [`rustc_link_arg_benches`] instruction tells Cargo to pass the
/// `-C link-arg=FLAG` option to the compiler, but only when building a
/// benchmark target.
///
/// For more information, see this [cargo reference].
///
/// [cargo reference]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg-benches
#[macro_export]
macro_rules! rustc_link_arg_benches {
  ($($tokens:tt)*) => {
    println!("cargo:rustc-link-arg-benches={}", ::std::format!($($tokens)*))
  };
}

/// The [`rustc_flags`] instruction tells Cargo to pass the given space-separated
/// flags to the compiler. This only allows the `-l` and `-L` flags, and is
/// equivalent to using `rustc-link-lib` and `rustc-link-search`.
///
/// For more information, see this [cargo reference].
///
/// [cargo reference]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-flags
#[macro_export]
macro_rules! rustc_flags {
  ($($tokens:tt)*) => {
    println!("cargo:rustc-flags={}", ::std::format!($($tokens)*))
  };
}

/// The [`rustc_cfg`] instruction tells Cargo to pass the given value to the
/// `--cfg flag` to the compiler. This may be used for compile-time detection
/// of features to enable conditional compilation.
///
/// Note that this does not affect Cargo’s dependency resolution. This cannot
/// be used to enable an optional dependency, or enable other Cargo features.
///
/// Be aware that Cargo features use the form feature="foo". cfg values passed
/// with this flag are not restricted to that form, and may provide just a
/// single identifier, or any arbitrary key/value pair. For example, emitting
/// `cargo:rustc-cfg=abc` will then allow code to use `#[cfg(abc)]`
/// (note the lack of feature=). Or an arbitrary key/value pair may be used
/// with an = symbol like `cargo:rustc-cfg=my_component="foo"`. The key should
/// be a Rust identifier, the value should be a string.
///
/// For more information, see this [cargo reference].
///
/// [cargo reference]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-cfg
///
/// # Examples
///
/// basic use:
///
/// ```rust
/// # use build::rustc_cfg;
/// rustc_cfg!("abc");                 // allows #[cfg(abc)]
/// rustc_cfg!("my_component=\"foo\"") // allows #[cfg(my_component="foo")]
/// ```
#[macro_export]
macro_rules! rustc_cfg   {
  ($($tokens:tt)*) => {
    println!("cargo:rustc-cfg={}", ::std::format!($($tokens)*))
  }
}

/// The [`rustc_env`] instruction tells Cargo to set the given environment
/// variable when compiling the package. The value can be then retrieved by the
/// env! macro in the compiled crate. This is useful for embedding additional
/// metadata in crate’s code, such as the hash of git HEAD or the unique
/// identifier of a continuous integration server.
///
/// See also the [environment variables automatically included by Cargo][link].
///
/// For more information, see this [cargo reference].
///
/// [link]: https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates
/// [cargo reference]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-env
#[macro_export]
macro_rules! rustc_env {
  ($($tokens:tt)*) => {
    println!("cargo:rustc-env={}", ::std::format!($($tokens)*))
  };
}

/// The [`rustc_cdylib_link_arg`] instruction tells Cargo to pass the
/// `-C link-arg=FLAG` option to the compiler, but only when building a cdylib
/// library target. Its usage is highly platform specific. It is useful to set
/// the shared library version or the runtime-path.
///
/// For more information, see this [cargo reference].
///
/// [cargo reference]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-cdylib-link-arg
#[macro_export]
macro_rules! rustc_cdylib_link_arg {
  ($($tokens:tt)*) => {
    println!("cargo:rustc-cdylib-link-arg={}", ::std::format!($($tokens)*))
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
