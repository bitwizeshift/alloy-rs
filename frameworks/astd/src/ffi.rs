//! This module provides access to FFI utilities not present in the standard
//! library, but very helfpul for working with interoperating with C APIs.
use std::ffi::{c_char, CStr, CString};

/// Creates a compile-time [`CStr`] object from a string literal.
///
/// This will statically verify the invariant that the [`CStr`] does not contain
/// any embedded nul character, and will also append the trailing nul terminator.
///
/// # Examples
///
/// ```rust
/// # use astd::ffi::cstr;
///
/// let cstr = cstr!("Hello world!");
/// ```
///
/// [`CStr`]: core::ffi::CStr
#[macro_export]
#[doc(hidden)]
macro_rules! cstr {
  ($e:expr) => {{
    use core::ffi::CStr;

    const S: &'static str = concat!($e, "\0");
    const _IS_VALID_C_STR: [(); $e.len()] = [(); $crate::_null_terminated_length(S)];

    // SAFETY: `from_bytes_with_nul_unchecked` asserts at compile-time that there
    // cannot be any embedded nuls. This guarantees that safety is upheld, despite
    // being an unsafe API.
    const RESULT: &'static CStr = unsafe { &CStr::from_bytes_with_nul_unchecked(S.as_bytes()) };

    RESULT
  }};
}

#[doc(inline)]
pub use super::cstr;

/// A module containing [`CStr`] utilities and functionality.
///
/// [`CStr`]: std::ffi::CStr
pub mod cstr {
  use super::*;

  /// Constructs a [`CStr`] reference from a [`c_char`] slice which contains a
  /// nul-terminator.
  ///
  /// # Arguments
  ///
  /// * `slice` - a slice of [`c_char`] that ends with a nul-terminator.
  ///
  /// # Safety
  ///
  /// This function requires that `slice` be nul-terminated.
  #[inline]
  pub const unsafe fn from_char_slice_unchecked(slice: &[c_char]) -> &CStr {
    // SAFETY: c_char (i8) and u8 are interconvertible
    let s = unsafe { &*(slice as *const _ as *const [u8]) };

    CStr::from_bytes_with_nul_unchecked(s)
  }
}

/// A module containing [`CString`] utilities and functionality.
///
/// [`CString`]: std::ffi::CString
pub mod cstring {
  use super::*;

  /// Constructs a [`CString`] from a non null-terminated [`str`] reference.
  ///
  /// # Arguments
  ///
  /// * `string` - a standar rust string.
  ///
  /// # Safety
  ///
  /// `string` must not contain any interior nul bytes.
  pub unsafe fn from_str(string: &str) -> CString {
    let bytes = string.as_bytes().to_owned();
    unsafe { CString::from_vec_unchecked(bytes) }
  }
}
