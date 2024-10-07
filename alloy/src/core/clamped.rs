use std::borrow::Borrow;
use std::cmp;
use std::fmt;
use std::ops;

/// An [`f32`] point value clamped between 0.0 and 1.0.
///
/// This upholds the invariant that the value is always clamped at construction
/// time so that the caller is not required to make this check.
///
/// This is a drop-in replacement for cases where GLclampf is used.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct ClampedF32(f32);

/// An [`f64`] point value clamped between 0.0 and 1.0.
///
/// This upholds the invariant that the value is always clamped at construction
/// time so that the caller is not required to make this check.
///
/// This is a drop-in replacement for cases where GLclampd is used.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct ClampedF64(f64);

/// An error returned when a value is not clamped between 0.0 and 1.0.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct ClampedError(f64);

impl fmt::Display for ClampedError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Input '{}' is not clamped between 0.0 and 1.0", self.0)
  }
}

impl std::error::Error for ClampedError {}

macro_rules! impl_clamped {
  ($($Type:ident($Underlying:ident)$(,)?)+) => {
    $(
      // Constructors
      impl $Type {
        /// The minimum value of a Clamped floating point value.
        pub const MIN: Self = Self(0.0);

        /// The maximum value of a Clamped floating point value.
        pub const MAX: Self = Self(1.0);

        /// Create a new Clamped floating point value.
        ///
        /// The value will be clamped between 0.0 and 1.0.
        ///
        /// # Parameters
        ///
        /// * `value` - The value to clamp.
        #[must_use]
        #[inline(always)]
        pub fn clamp(value: $Underlying) -> Self {
          Self(value.clamp(Self::MIN.0, Self::MAX.0))
        }

        /// Create a new Clamped floating point value if the value is clamped between
        /// 0.0 and 1.0.
        ///
        /// If the value is not clamped between 0.0 and 1.0, a [`ClampedError`] is
        /// returned.
        ///
        /// # Parameters
        ///
        /// * `value` - The value to clamp.
        pub fn new(value: $Underlying) -> Result<Self, ClampedError> {
          if (Self::MIN.0..=Self::MAX.0).contains(&value) {
            Ok(Self(value))
          } else {
            Err(ClampedError(value as f64))
          }
        }

        /// Create a new Clamped instance without clamping the value.
        ///
        /// Consider using the safe equivalent [`Self::new`], or the
        /// automatic clamping [`Self::clamp`] instead.
        ///
        /// # Safety
        ///
        /// The user is required to ensure that the value is clamped between 0.0 and
        /// 1.0, otherwise the behavior is undefined.
        ///
        /// # Parameters
        ///
        /// * `value` - The value to clamp.
        #[must_use]
        #[inline(always)]
        pub const unsafe fn new_unchecked(value: $Underlying) -> Self {
          Self(value)
        }
      }

      // Conversion

      impl From<$Underlying> for $Type {
        fn from(value: $Underlying) -> Self {
          Self::clamp(value)
        }
      }

      impl ops::Deref for $Type {
        type Target = $Underlying;

        fn deref(&self) -> &Self::Target {
          &self.0
        }
      }

      impl AsRef<$Underlying> for $Type {
        fn as_ref(&self) -> &$Underlying {
          &self.0
        }
      }

      impl Borrow<$Underlying> for $Type {
        fn borrow(&self) -> &$Underlying {
          &self.0
        }
      }

      // Properties
      impl $Type {
        /// Get the value as a floating point number.
        #[must_use]
        #[inline(always)]
        pub fn get(self) -> $Underlying {
          self.0
        }
      }

      // Operators
      impl $Type {
        /// Add two Clamped floating point values, clamping the result between 0.0
        /// and 1.0.
        ///
        /// # Parameters
        ///
        /// * `rhs` - The right-hand side value.
        #[must_use]
        #[inline(always)]
        pub fn saturating_add(self, rhs: Self) -> Self {
          Self::clamp(self.0 + rhs.0)
        }

        /// Subtract two Clamped floating point values, clamping the result between
        /// 0.0 and 1.0.
        ///
        /// # Parameters
        ///
        /// * `rhs` - The right-hand side value.
        #[must_use]
        #[inline(always)]
        pub fn saturating_sub(self, rhs: Self) -> Self {
          Self::clamp(self.0 - rhs.0)
        }

        /// Multiply two Clamped floating point values, clamping the result between
        /// 0.0 and 1.0.
        ///
        /// # Parameters
        ///
        /// * `rhs` - The right-hand side value.
        #[must_use]
        #[inline(always)]
        pub fn saturating_mul(self, rhs: Self) -> Self {
          Self(self.0 * rhs.0)
        }

        /// Divide two Clamped floating point values, clamping the result between
        /// 0.0 and 1.0.
        ///
        /// # Parameters
        ///
        /// * `rhs` - The right-hand side value.
        #[must_use]
        #[inline(always)]
        pub fn saturating_div(self, rhs: Self) -> Self {
          Self::clamp(self.0 / rhs.0)
        }

        /// Modulo two Clamped floating point values, clamping the result between
        /// 0.0 and 1.0.
        ///
        /// # Parameters
        ///
        /// * `rhs` - The right-hand side value.
        #[must_use]
        #[inline(always)]
        pub fn saturating_rem(self, rhs: Self) -> Self {
          Self::clamp(self.0 % rhs.0)
        }

        /// Add a Clamped floating point value to the current value, clamping the
        /// result between 0.0 and 1.0.
        ///
        /// # Parameters
        ///
        /// * `rhs` - The right-hand side value.
        #[inline(always)]
        pub fn saturating_add_assign(&mut self, rhs: Self) {
          *self = self.saturating_add(rhs);
        }

        /// Subtract a Clamped floating point value from the current value, clamping
        /// the result between 0.0 and 1.0.
        ///
        /// # Parameters
        ///
        /// * `rhs` - The right-hand side value.
        #[inline(always)]
        pub fn saturating_sub_assign(&mut self, rhs: Self) {
          *self = self.saturating_sub(rhs);
        }

        /// Multiply a Clamped floating point value with the current value, clamping
        /// the result between 0.0 and 1.0.
        ///
        /// # Parameters
        ///
        /// * `rhs` - The right-hand side value.
        #[inline(always)]
        pub fn saturating_mul_assign(&mut self, rhs: Self) {
          *self = self.saturating_mul(rhs);
        }

        /// Divide a Clamped floating point value with the current value, clamping
        /// the result between 0.0 and 1.0.
        ///
        /// # Parameters
        ///
        /// * `rhs` - The right-hand side value.
        #[inline(always)]
        pub fn saturating_div_assign(&mut self, rhs: Self) {
          *self = self.saturating_div(rhs);
        }

        /// Modulo a Clamped floating point value with the current value, clamping
        /// the result between 0.0 and 1.0.
        ///
        /// # Parameters
        ///
        /// * `rhs` - The right-hand side value.
        #[inline(always)]
        pub fn saturating_rem_assign(&mut self, rhs: Self) {
          *self = self.saturating_rem(rhs);
        }
      }

      impl ops::Add for $Type {
        type Output = $Underlying;

        #[inline(always)]
        fn add(self, rhs: Self) -> Self::Output {
          self.0 + rhs.0
        }
      }

      impl ops::Add<$Underlying> for $Type {
        type Output = $Underlying;

        #[inline(always)]
        fn add(self, rhs: $Underlying) -> Self::Output {
          self.0 + rhs
        }
      }

      impl ops::Add<$Type> for $Underlying {
        type Output = $Underlying;

        #[inline(always)]
        fn add(self, rhs: $Type) -> Self::Output {
          self + rhs.0
        }
      }

      impl ops::Sub for $Type {
        type Output = $Underlying;

        #[inline(always)]
        fn sub(self, rhs: Self) -> Self::Output {
          self.0 - rhs.0
        }
      }

      impl ops::Sub<$Underlying> for $Type {
        type Output = $Underlying;

        #[inline(always)]
        fn sub(self, rhs: $Underlying) -> Self::Output {
          self.0 - rhs
        }
      }

      impl ops::Sub<$Type> for $Underlying {
        type Output = $Underlying;

        #[inline(always)]
        fn sub(self, rhs: $Type) -> Self::Output {
          self - rhs.0
        }
      }

      impl ops::Mul for $Type {
        type Output = Self;

        #[inline(always)]
        fn mul(self, rhs: Self) -> Self::Output {
          // SAFETY: The product of two values clamped between 0.0 and 1.0 is also
          // clamped between 0.0 and 1.0.
          unsafe { Self::new_unchecked(self.0 * rhs.0) }
        }
      }

      impl ops::Mul<$Underlying> for $Type {
        type Output = $Underlying;

        #[inline(always)]
        fn mul(self, rhs: $Underlying) -> Self::Output {
          self.0 * rhs
        }
      }

      impl ops::Mul<$Type> for $Underlying {
        type Output = $Underlying;

        #[inline(always)]
        fn mul(self, rhs: $Type) -> Self::Output {
          self * rhs.0
        }
      }

      impl ops::Div for $Type {
        type Output = $Underlying;

        #[inline(always)]
        fn div(self, rhs: Self) -> Self::Output {
          self.0 / rhs.0
        }
      }

      impl ops::Div<$Underlying> for $Type {
        type Output = $Underlying;

        #[inline(always)]
        fn div(self, rhs: $Underlying) -> Self::Output {
          self.0 / rhs
        }
      }

      impl ops::Div<$Type> for $Underlying {
        type Output = $Underlying;

        #[inline(always)]
        fn div(self, rhs: $Type) -> Self::Output {
          self / rhs.0
        }
      }

      impl ops::Rem for $Type {
        type Output = Self;

        #[inline(always)]
        fn rem(self, rhs: Self) -> Self::Output {
          // SAFETY: The modulo of two values clamped between 0.0 and 1.0 is also
          // clamped between 0.0 and 1.0.
          unsafe { Self::new_unchecked(self.0 % rhs.0) }
        }
      }

      impl ops::Rem<$Underlying> for $Type {
        type Output = $Underlying;

        #[inline(always)]
        fn rem(self, rhs: $Underlying) -> Self::Output {
          self.0 % rhs
        }
      }

      impl ops::Rem<$Type> for $Underlying {
        type Output = $Underlying;

        #[inline(always)]
        fn rem(self, rhs: $Type) -> Self::Output {
          self % rhs.0
        }
      }

      impl ops::MulAssign for $Type {
        #[inline(always)]
        fn mul_assign(&mut self, rhs: Self) {
          self.0 *= rhs.0;
        }
      }

      // Comparison

      impl cmp::PartialEq<$Underlying> for $Type {
        #[inline(always)]
        fn eq(&self, other: &$Underlying) -> bool {
          self.0.eq(other)
        }
      }

      impl cmp::PartialEq<$Type> for $Underlying {
        #[inline(always)]
        fn eq(&self, other: &$Type) -> bool {
          self.eq(&other.0)
        }
      }

      impl cmp::PartialOrd<$Underlying> for $Type {
        #[inline(always)]
        fn partial_cmp(&self, other: &$Underlying) -> Option<cmp::Ordering> {
          self.0.partial_cmp(other)
        }
      }

      impl cmp::PartialOrd<$Type> for $Underlying {
        #[inline(always)]
        fn partial_cmp(&self, other: &$Type) -> Option<cmp::Ordering> {
          self.partial_cmp(&other.0)
        }
      }

      // Formatting

      impl fmt::Display for $Type {
        #[inline(always)]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
          self.0.fmt(f)
        }
      }
    )+
  };
}

impl_clamped!(ClampedF32(f32), ClampedF64(f64));

#[cfg(test)]
#[path = "clamped.test.rs"]
mod test;
