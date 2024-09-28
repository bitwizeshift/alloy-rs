//! An internal crate containing shared errors for the Vector types.

use std::error::Error;
use std::fmt;

/// An error that may be returned from a Vector operation when constructing
/// a vector from a slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VecError {
  expected: usize,
  actual: usize,
}

impl VecError {
  /// Create a new error with the expected length
  ///
  /// # Parameters
  ///
  /// * `expected` - The expected length of the slice.
  /// * `length` - The actual length of the slice.
  pub(super) const fn new(expected: usize, actual: usize) -> Self {
    Self { expected, actual }
  }

  /// Get the length of the slice that was attempted to be converted.
  pub const fn length(&self) -> usize {
    self.actual
  }
}

impl fmt::Display for VecError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "expected a slice of length {}, but got {}",
      self.expected, self.actual
    )
  }
}

impl Error for VecError {}
