//! This crate provides core definitions that need to be shared between the
//! various wrapper utilities.
#![deny(missing_docs)]
#![deny(unused_results)]
#![deny(rust_2018_idioms)]

/// Creates a compile-time [`CStr`] object from a string literal.
///
/// This will statically verify the invariant that the [`CStr`] does not contain
/// any embedded nul character, and will also append the trailing nul terminator.
///
/// # Examples
///
/// ```rust
/// # use foundation::cstr;
///
/// let cstr = cstr!("Hello world!");
/// ```
///
/// [`CStr`]: core::ffi::CStr
#[macro_export]
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
