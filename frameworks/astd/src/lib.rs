//! This crate provides core definitions that need to be shared between the
//! various wrapper utilities.
#![deny(missing_docs)]
#![deny(unused_results)]
#![deny(rust_2018_idioms)]

mod macros;
mod source_location;

pub use source_location::*;

pub mod convert;
pub mod ffi;
pub mod meta;
pub mod ptr;

/// A module for providing extension utility functions to [`u16`] types.
pub mod u16 {
  /// A small function for forming a `u16` from a string at compile-time.
  ///
  /// # Panics
  ///
  /// This function will panic if the input string is not a valid decimal
  /// (radix10) string.
  pub const fn from_str_radix10(s: &str) -> u16 {
    let mut bytes = s.as_bytes();
    let mut val: u16 = 0;
    while let [byte, rest @ ..] = bytes {
      assert!(b'0' <= *byte && *byte <= b'9', "invalid digit");
      val = val * 10 + (*byte - b'0') as u16;
      bytes = rest;
    }
    val
  }
}

/// Computes the null-terminated string length of a C-string.
///
/// This function exists since the current FFI helpers are not `const` functions,
/// and this enables checking for the value at compile-time.
///
/// This is an implementation-detail of [`c_str!`], which simply delegates the
/// call to [`_null_terminated_length_b`].
///
/// # Arguments
///
/// * `s` - the string slice
#[doc(hidden)]
pub const fn _null_terminated_length(s: &str) -> usize {
  _null_terminated_length_b(s.as_bytes(), 0, 0)
}

/// Computes the null-terminated string length of a C-string.
///
/// This counts each independent non-null byte, and will stop executing when it
/// exceeds
#[doc(hidden)]
pub const fn _null_terminated_length_b(bytes: &[u8], index: usize, count: usize) -> usize {
  if bytes[index] == 0 {
    count
  } else {
    _null_terminated_length_b(bytes, index + 1, count + 1)
  }
}

#[doc(inline)]
pub use uuid::Uuid;

unsafe impl convert::Transparent for Uuid {
  type Wrapped = uuid::Bytes;
}
