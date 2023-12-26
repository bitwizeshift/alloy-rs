/// The location of source code at a point in the codebase.
///
/// This should generally be constructed by using the [`current_location`] macro.
///
/// # Example
///
/// Basic Use:
///
/// ```rust
/// #[doc(hidden)]
/// pub fn _inner_log(message: &str, location: astd::SourceLocation) {
///   println!("{}: {}", location, message)
/// }
///
/// macro_rules! log {
///   ($($arg:tt)*) => {
/// #   use $crate::*;
///     _inner_log($($arg)*, astd::current_location!())
///   };
/// }
///
/// // ...
///
/// # fn test_log() {
/// // Implicitly captures source-location at the same time
/// log!("logging something with source location...");
/// # }
/// ```
#[derive(Eq, PartialEq, PartialOrd, Ord, Default)]
pub struct SourceLocation {
  /// The path to the file
  pub file: &'static str,
  /// The line within the file
  pub line: u32,
  /// The column
  pub column: u32,
}

impl std::fmt::Display for SourceLocation {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}:{}:{}", self.file, self.line, self.column)
  }
}

impl std::fmt::Debug for SourceLocation {
  #[inline(always)]
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    <Self as std::fmt::Display>::fmt(self, f)
  }
}

/// Creates a [`SourceLocation`] object that represents the exact point in the
/// source file where this macro is invoked.
///
/// This can be used to mechanically capture the location of events in source
/// code for the purposes of debugging.
#[macro_export]
macro_rules! current_location {
  () => {
    $crate::SourceLocation {
      file: file!(),
      line: line!(),
      column: column!(),
    }
  };
}
