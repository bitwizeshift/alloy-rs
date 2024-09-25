//! A module containing comparator definitions used throughout Alloy.
//!
//! This is analogous to the [`std::cmp`] crate, with extensions that are
//! only sensible for graphics management.

/// Trait for if something is "near" within a specified tolerance.
///
/// # Examples
///
/// Basic use:
///
/// ```rust
/// # use alloy::cmp::Near;
/// pub struct MyFloat(f32);
///
/// impl Near for MyFloat {
///   fn near(&self, other: &MyFloat, tolerance: &f32) -> bool {
///     self.0.near(&other.0, tolerance)
///   }
/// }
///
/// # fn test() {
/// assert!(&MyFloat(0.0001).near(&MyFloat(0.00011), &0.0001));
/// assert!(!&MyFloat(0.0001).near(&MyFloat(0.0011), &0.000001));
/// # }
/// ```
pub trait Near<Rhs: ?Sized = Self, Tolerance = f32> {
  /// Tests whether `self` is "near" `other` relative top the specified
  /// `tolerance`
  ///
  /// # Parameters
  ///
  /// * `other` - the other value to test against
  /// * `tolerance` - the tolerance being tested
  fn near(&self, other: &Rhs, tolerance: &Tolerance) -> bool;
}

impl Near for f32 {
  fn near(&self, other: &Self, distance: &f32) -> bool {
    (self - other).abs() <= *distance
  }
}

impl<T, U> Near<[T], U> for [T]
where
  T: Near<T, U>,
{
  fn near(&self, other: &Self, tolerance: &U) -> bool {
    if self.len() != other.len() {
      return false;
    }
    for (lhs, rhs) in self.iter().zip(other.iter()) {
      if !lhs.near(rhs, tolerance) {
        return false;
      }
    }
    true
  }
}

impl<const N: usize, T, U> Near<[T; N], U> for [T; N]
where
  T: Near<T, U>,
{
  fn near(&self, other: &Self, tolerance: &U) -> bool {
    for i in 0..N {
      // SAFETY: The size is bound between [0, N)
      if !unsafe {
        self
          .get_unchecked(i)
          .near(other.get_unchecked(i), tolerance)
      } {
        return false;
      }
    }
    true
  }
}

impl Near<f32, f64> for f32 {
  fn near(&self, other: &Self, distance: &f64) -> bool {
    (self - other).abs() as f64 <= *distance
  }
}

impl Near<f64, f64> for f64 {
  fn near(&self, other: &Self, distance: &f64) -> bool {
    (self - other).abs() <= *distance
  }
}
impl Near<f64, f32> for f64 {
  fn near(&self, other: &Self, distance: &f32) -> bool {
    (self - other).abs() <= *distance as f64
  }
}

/// Trait for comparing objects for near-equality relative to a default tolerance.
///
/// This is effectively a short-hand for `Near` with a default `f32` tolerance.
pub trait AlmostEq<Rhs: ?Sized = Self>: Near<Rhs> {
  /// Tests whether `self` is almost equal to `other`
  ///
  /// # Parameters
  ///
  /// * `other` - the other value to compare against
  fn almost_eq(&self, other: &Rhs) -> bool;
}

impl AlmostEq for f32 {
  fn almost_eq(&self, other: &Self) -> bool {
    self.near(other, &f32::EPSILON)
  }
}

impl AlmostEq for f64 {
  fn almost_eq(&self, other: &Self) -> bool {
    self.near(other, &f64::EPSILON)
  }
}

impl<T> AlmostEq<[T]> for [T]
where
  T: AlmostEq<T>,
{
  #[inline]
  fn almost_eq(&self, other: &[T]) -> bool {
    if self.len() != other.len() {
      return false;
    }
    for i in 0..self.len() {
      if !self[i].almost_eq(&other[i]) {
        return false;
      }
    }
    true
  }
}

impl<const N: usize, T> AlmostEq<[T; N]> for [T; N]
where
  T: AlmostEq<T>,
{
  #[inline]
  fn almost_eq(&self, other: &[T; N]) -> bool {
    for i in 0..N {
      // SAFETY: the index is bound between [0, N)
      if !unsafe { self.get_unchecked(i).almost_eq(other.get_unchecked(i)) } {
        return false;
      }
    }
    true
  }
}
