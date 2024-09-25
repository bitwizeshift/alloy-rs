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
  /// # Parameters
  ///
  /// * `rhs` - the other sequence of values to perform the product with
  fn try_dot(&self, rhs: &Rhs) -> Result<Self::Output, Self::Error>;

  /// Performs an unguarded [dot-product] between two possibly different-sized
  /// sequences of values.
  ///
  /// # Parameters
  ///
  /// * `rhs` - the other sequence of values to perform the product with
  ///
  /// # Safety
  ///
  /// This function call will be undefined-behavior if `try_dot` does not yield
  /// a value.
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

/// An error that may be returned from fallable [`TryDot`] operations.
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
  /// The output type of the Cross operation
  type Output;

  /// Computes the cross-product of two values, and returns the result.
  ///
  /// # Parameters
  ///
  /// * `other` - the other value to compute the cross-product with
  fn cross(&self, other: &Rhs) -> Self::Output;
}

/// Computes a midpoint of two types.
pub trait Midpoint {
  /// The output type
  type Output;

  /// Computes the midpoint of this type, returning the result
  ///
  /// # Parameters
  ///
  /// * `other` - the other value
  fn midpoint(&self, other: &Self) -> Self::Output;
}

/// Trait for interpolating between two different states.
///
/// See [Linear Interpolation] for more information.
///
/// [Linear Interpolation]: https://en.wikipedia.org/wiki/Linear_interpolation
pub trait Lerp<Rhs: ?Sized = Self> {
  /// The output of this operation.
  type Output;

  /// Interpolates between two different states
  ///
  /// # Parameters
  ///
  /// * `a` - The first value.
  /// * `b` - The second value.
  /// * `alpha` - The interpolation factor.
  fn lerp(&self, b: &Rhs, alpha: f32) -> Self::Output;
}

macro_rules! impl_lerp {
  ($($t:ty),*) => {
    $(
      impl Lerp for $t {
        type Output = $t;

        fn lerp(&self, b: &Self, alpha: f32) -> Self {
          ((*self as f32) + ((*b as f32) - (*self as f32)) * alpha) as $t
        }
      }
    )*
  };
}

impl_lerp!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl Lerp for f32 {
  type Output = f32;

  fn lerp(&self, b: &Self, alpha: f32) -> Self {
    self + (b - self) * alpha
  }
}

impl Lerp for f64 {
  type Output = f64;

  fn lerp(&self, b: &Self, alpha: f32) -> Self {
    self + (b - self) * f64::from(alpha)
  }
}
