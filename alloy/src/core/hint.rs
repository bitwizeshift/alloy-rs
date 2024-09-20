//! Hints that can be used to guide the compiler's optimization decisions.

/// A marker intrinsic that tells the compiler to optimize the case where the
/// given boolean is likely to be true.
///
/// # Parameters
///
/// * `b` - The boolean to check
#[inline(always)]
pub const fn likely(b: bool) -> bool {
  if !b {
    unlikely_branch()
  }
  b
}

/// A marker intrinsic that tells the compiler to optimize the case where the
/// given boolean is unlikely to be true.
///
/// # Parameters
///
/// * `b` - The boolean to check
#[inline(always)]
pub const fn unlikely(b: bool) -> bool {
  if b {
    unlikely_branch()
  }
  b
}

/// A marker intrinsic that tells the compiler that the branch this intrinsic
/// is placed on is unlikely to be taken.
#[inline(always)]
#[cold]
pub const fn unlikely_branch() {}
