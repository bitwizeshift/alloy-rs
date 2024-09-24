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

/// A marker intrinsic that tells the compiler that the slice will have a
/// fixed size of `N`.
///
/// # Parameters
///
/// * `slice` - The slice to check.
/// * `length` - The expected length of the slice.
///
/// # Safety
///
/// The size of the slice must be equal to `N`.
pub const unsafe fn fixed_size<T>(slice: &[T], length: usize) {
  if slice.len() != length {
    unsafe { std::hint::unreachable_unchecked() }
  }
}

/// A marker intrinsic that tells the compiler that the given condition is
/// assumed to be true.
///
/// # Parameters
///
/// * `cond` - The condition to assume.
///
/// # Safety
///
/// The condition being assumed must always hold true, otherwise this will lead
/// to undefined behavior.
pub const unsafe fn assume(cond: bool) {
  if !cond {
    unsafe { std::hint::unreachable_unchecked() }
  }
}
