//! Overloadable operators for Alloy.

use std::convert::Infallible;

/// Trait for an algabraic scalar product between two equal-length sequences of
/// numbers and returns a single value.
///
/// See [Dot Product] for more information.
///
/// [Dot Product]: https://en.wikipedia.org/wiki/Dot_product
pub trait Dot<Rhs: ?Sized = Self> {
  /// The output of this operation.
  ///
  /// In most cases, this will be [`f32`], since Alloy is largely built around
  /// 32-bit floating point numbers; though this can be configured.
  type Output;

  /// Performs the dot-product between two values, and returns the output value.
  ///
  /// # Argument
  ///
  /// * `rhs` - the right side operand of the operation.
  ///
  /// [dot-product]: https://en.wikipedia.org/wiki/Dot_product
  fn dot(&self, rhs: &Rhs) -> Self::Output;
}

impl<const N: usize> Dot for [f32; N] {
  type Output = f32;

  #[inline]
  fn dot(&self, other: &[f32; N]) -> Self::Output {
    let mut result = 0.0;
    for i in 0..N {
      result += self[i] * other[i]
    }
    result
  }
}

impl<const N: usize> Dot for [f64; N] {
  type Output = f64;

  #[inline]
  fn dot(&self, other: &[f64; N]) -> Self::Output {
    let mut result = 0.0;
    for i in 0..N {
      result += self[i] * other[i]
    }
    result
  }
}

/// Fallible trait for an algabraic scalar product between two possibly
/// inequal-length sequences of numbers and returns a single value.
///
/// This largely exists to enable a safe-ish way to perform a dot product
/// between unsized slices, as this does come up from time-to-time.
/// See [`Dot`] for the infallible equivalent. This trait is to [`TryFrom`] as
/// [`Dot`] is to [`From`].
///
/// See [Dot Product] for more information.
///
/// [Dot Product]: https://en.wikipedia.org/wiki/Dot_product
pub trait TryDot<Rhs: ?Sized = Self> {
  /// The error to return on failure.
  type Error;

  /// The type to output on success.
  type Output;

  /// Attempts to perform a [dot-product] between two possibly different-sized
  /// sequences of values.
  ///
  /// # Arguments
  ///
  /// * `rhs` - the other sequence of values to perform the product with
  fn try_dot(&self, rhs: &Rhs) -> Result<Self::Output, Self::Error>;

  /// Performs an unguarded [dot-product] between two possibly different-sized
  /// sequences of values.
  ///
  /// # Arguments
  ///
  /// * `rhs` - the other sequence of values to perform the product with
  ///
  /// # Safety
  ///
  /// This function call will be unsafe is `try_dot` does not yield a value.
  unsafe fn dot_unchecked(&self, rhs: &Rhs) -> Self::Output {
    self.try_dot(rhs).unwrap_unchecked()
  }
}

impl<T, U> TryDot<U> for T
where
  T: Dot<U>,
{
  type Error = Infallible;

  type Output = T::Output;

  fn try_dot(&self, rhs: &U) -> Result<Self::Output, Self::Error> {
    Ok(self.dot(rhs))
  }

  unsafe fn dot_unchecked(&self, rhs: &U) -> Self::Output {
    self.dot(rhs)
  }
}

///
pub struct DotError;

impl TryDot for [f32] {
  type Error = DotError;

  type Output = f32;

  fn try_dot(&self, rhs: &Self) -> Result<Self::Output, Self::Error> {
    if self.len() != rhs.len() {
      return Err(DotError);
    }
    // SAFETY: The bounds are checked above
    Ok(unsafe { self.dot_unchecked(rhs) })
  }

  unsafe fn dot_unchecked(&self, rhs: &Self) -> Self::Output {
    let mut result = 0.0;
    for i in 0..self.len() {
      result += unsafe { self.get_unchecked(i) * rhs.get_unchecked(i) }
    }
    result
  }
}

impl TryDot for [f64] {
  type Error = DotError;

  type Output = f64;

  fn try_dot(&self, rhs: &Self) -> Result<Self::Output, Self::Error> {
    if self.len() != rhs.len() {
      return Err(DotError);
    }
    // SAFETY: The bounds are checked above
    Ok(unsafe { self.dot_unchecked(rhs) })
  }

  unsafe fn dot_unchecked(&self, rhs: &Self) -> Self::Output {
    let mut result = 0.0;
    for i in 0..self.len() {
      result += unsafe { self.get_unchecked(i) * rhs.get_unchecked(i) }
    }
    result
  }
}

/// Performs the [cross-product]
///
/// [cross-product]: https://en.wikipedia.org/wiki/Cross_product
pub trait Cross<Rhs: ?Sized = Self> {
  ///
  type Output;

  ///
  fn cross(&self, other: &Rhs) -> Self::Output;
}
