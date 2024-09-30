//! A module for providing angular definitions needed for geometric operations.
use std::ops;

use crate::cmp::{AlmostEq, Near};

mod sealed {
  pub trait Sealed {}
}

/// A closed trait used to enable conversions of [`Angle`] types into [`Unit`]
/// representation.
///
/// This is a requirement for being able to generically define angular conversions.
///
/// **Note:** This may become unsealed in a future update
pub trait ToUnit: sealed::Sealed {
  /// Converts this angular unit into a [`Unit`] angle.
  fn to_unit(self) -> Unit;
}

/// A closed trait used to enable conversions of [`Unit`] angle representations
/// into [`Angle`] types.
///
/// This is a requirement for being able to generically define angular conversions.
///
/// **Note:** This may become unsealed in a future update
pub trait FromUnit: sealed::Sealed {
  /// Converts the [`Unit`] angle into the new [`Angle`] type
  ///
  /// # Parameters
  ///
  /// * `angle` - the [`Unit`] angle to convert
  fn from_unit(angle: Unit) -> Self;
}

/// A closed trait used to provide trigonmetric functionalities to angular
/// types.
///
/// **Note:** This may become unsealed in a future update
pub trait Trig: sealed::Sealed {
  /// Computes the sine of this angle.
  fn sin(self) -> f32;

  /// Computes the cosine of this angle.
  fn cos(self) -> f32;

  /// Computes the tangent of this angle.
  fn tan(self) -> f32;

  /// Computes the sine and tangent of this angle.
  fn sin_cos(self) -> (f32, f32);

  /// Computes the arc-sine of the input value, returning the [`Angle`]
  ///
  /// # Parameters
  ///
  /// * `value` - the value to compute the arc-sine of
  fn asin(value: f32) -> Self;

  /// Computes the arc-cosine of the input value, returning the [`Angle`]
  ///
  /// # Parameters
  ///
  /// * `value` - the value to compute the arc-cosine of
  fn acos(value: f32) -> Self;

  /// Computes the arc-tangent of the input value, returning the [`Angle`]
  ///
  /// # Parameters
  ///
  /// * `value` - the value to compute the arc-tangent of
  fn atan(value: f32) -> Self;
}

/// A closed trait used to represent various angular units.
///
/// Types that implement this inherit all basic trigonometric functions and
/// enable easy conversion between angular units.
pub trait Angle:
  Copy + Trig + FromUnit + ToUnit + ops::Mul<f32, Output = Self> + ops::Div<f32, Output = Self>
{
  /// Converts this [`Angle`] to any other generic [`Angle`] by converting to
  /// [`Unit`] as an intermediate value.
  ///
  /// **Note:** This function will often be less-optimized than calling the
  /// specific `to_radians`, `to_degrees`, etc functions.
  fn to_angle<A: Angle>(self) -> A {
    A::from_unit(self.to_unit())
  }

  /// Constructs this angle from any other [`Angle`] unit value
  ///
  /// **Note:** This function will often be less-optimized than calling the
  /// specific `to_radians`, `to_degrees`, etc functions.
  ///
  /// # Parameters
  ///
  /// * `angle` - the angle to construct from
  fn from_angle<A: Angle>(angle: A) -> Self {
    Self::from_unit(angle.to_unit())
  }
}

const ANGLE_EPSILON: f32 = 1e-5;
const UNIT_EPSILON: f32 = f32::EPSILON;

/// Representation of a [Radian] angle.
///
/// The radian, denoted by the symbol rad, is the unit of angle in the
/// International System of Units (SI) and is the standard unit of angular
/// measure used in many areas of mathematics. It is defined such that one
/// radian is the angle subtended at the centre of a circle by an arc that is
/// equal in length to the radius
///
/// [Radian]: https://en.wikipedia.org/wiki/Radian
#[derive(Copy, Clone, Debug, Default)]
#[repr(transparent)]
pub struct Radian(pub f32);

impl Radian {
  /// Identity conversion.
  ///
  /// This exists for symmetry with the other angle definitions.
  ///
  /// # Example
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::Radian;
  ///
  /// assert_eq!(Radian(42.0).to_radians(), Radian(42.0));
  /// ```
  #[inline(always)]
  pub const fn to_radians(self) -> Radian {
    self
  }

  /// Converts this [`Radian`] into [`Degree`].
  ///
  /// # Example
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::{Radian, Degree};
  ///
  /// assert_eq!(Radian::ONE_REVOLUTION.to_degrees(), Degree::ONE_REVOLUTION);
  /// ```
  pub fn to_degrees(self) -> Degree {
    const FACTOR: f32 = Degree::ONE_REVOLUTION.0 / Radian::ONE_REVOLUTION.0;
    Degree(self.0 * FACTOR)
  }

  /// Converts this [`Radian`] into [`Gradian`].
  ///
  /// # Example
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::{Radian, Gradian};
  ///
  /// assert_eq!(Radian::ONE_REVOLUTION.to_degrees(), Gradian::ONE_REVOLUTION);
  /// ```
  pub fn to_gradians(self) -> Gradian {
    const FACTOR: f32 = Gradian::ONE_REVOLUTION.0 / Radian::ONE_REVOLUTION.0;
    Gradian(self.0 * FACTOR)
  }

  /// Constructs a new [`Radian`] object from an existing `angle`.
  ///
  /// This exists for symmetry with the other angle definitions. This is
  /// an identity function.
  ///
  /// # Parameters
  ///
  /// * `angle` - the angle to construct from
  ///
  /// # Example
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::Radian;
  ///
  /// assert_eq!(Radian::from_radians(Radian(42.0)), Radian(42.0));
  /// ```
  #[inline(always)]
  pub const fn from_radians(angle: Radian) -> Self {
    angle
  }

  /// Constructs a new [`Radian`] object from an existing `angle` in [`Degree`]s.
  ///
  /// # Parameters
  ///
  /// * `angle` - the angle to construct from
  ///
  /// # Example
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::{Radian, Degree};
  ///
  /// assert_eq!(Radian::from_degrees(Degree::ONE_REVOLUTION), Radian::ONE_REVOLUTION);
  /// ```
  pub fn from_degrees(angle: Degree) -> Self {
    const FACTOR: f32 = Radian::ONE_REVOLUTION.0 / Degree::ONE_REVOLUTION.0;
    Self(angle.0 * FACTOR)
  }

  /// Constructs a new [`Radian`] object from an existing `angle` in [`Gradian`]s.
  ///
  /// # Parameters
  ///
  /// * `angle` - the angle to construct from
  ///
  /// # Example
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::{Radian, Gradian};
  ///
  /// assert_eq!(Radian::from_gradians(Gradian::ONE_REVOLUTION), Gradian::ONE_REVOLUTION);
  /// ```
  pub fn from_gradians(angle: Gradian) -> Self {
    const FACTOR: f32 = Radian::ONE_REVOLUTION.0 / Gradian::ONE_REVOLUTION.0;
    Self(angle.0 * FACTOR)
  }
}

impl std::fmt::Display for Radian {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} rad", self.0)
  }
}

impl From<Degree> for Radian {
  #[inline(always)]
  fn from(other: Degree) -> Self {
    Self::from_degrees(other)
  }
}

impl From<Gradian> for Radian {
  #[inline(always)]
  fn from(other: Gradian) -> Self {
    Self::from_gradians(other)
  }
}

impl From<Unit> for Radian {
  #[inline(always)]
  fn from(other: Unit) -> Self {
    Self::from_unit(other)
  }
}

impl AlmostEq<Radian> for Radian {
  #[inline]
  fn almost_eq(&self, other: &Radian) -> bool {
    self.near(other, &ANGLE_EPSILON)
  }
}

impl AlmostEq<Degree> for Radian {
  #[inline]
  fn almost_eq(&self, other: &Degree) -> bool {
    self.near(&other.to_radians(), &ANGLE_EPSILON)
  }
}

impl AlmostEq<Gradian> for Radian {
  #[inline]
  fn almost_eq(&self, other: &Gradian) -> bool {
    self.near(&other.to_radians(), &ANGLE_EPSILON)
  }
}

impl AlmostEq<Unit> for Radian {
  #[inline]
  fn almost_eq(&self, other: &Unit) -> bool {
    self.almost_eq(&other.to_radians())
  }
}

/// Representation of a [Degree] unit of angles.
///
/// It is not an SI unit—the SI unit of angular measure is the radian—but it is
/// mentioned in the SI brochure as an accepted unit. Because a full rotation
/// equals 2π radians, one degree is equivalent to π/180 radians.
///
/// [Degree]: https://en.wikipedia.org/wiki/Degree_(angle)
#[derive(Copy, Clone, Debug, Default)]
#[repr(transparent)]
pub struct Degree(pub f32);

impl Degree {
  /// Converts this [`Degree`] into [`Radian`].
  ///
  /// # Example
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::{Radian, Degree};
  ///
  /// assert_eq!(Degree::ONE_REVOLUTION.to_radians(), Radian::ONE_REVOLUTION);
  /// ```
  pub fn to_radians(self) -> Radian {
    const FACTOR: f32 = Radian::ONE_REVOLUTION.0 / Degree::ONE_REVOLUTION.0;
    Radian(self.0 * FACTOR)
  }

  /// Identity conversion.
  ///
  /// This exists for symmetry with the other angle definitions.
  ///
  /// # Example
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::Degree;
  ///
  /// assert_eq!(Degree::ONE_REVOLUTION.to_degrees(), Degree::ONE_REVOLUTION);
  /// ```
  #[inline(always)]
  pub const fn to_degrees(self) -> Degree {
    self
  }

  /// Converts this [`Degree`] into [`Gradian`].
  ///
  /// # Example
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::{Gradian, Degree};
  /// # use alloy::cmp::AlmostEq;
  ///
  /// let lhs = Gradian::ONE_REVOLUTION.to_gradians();
  /// let rhs = Gradian::ONE_REVOLUTION;
  /// // Degree => Gradian conversions are less exact (to ~5 decimal places with
  /// // 32-bit IEEE floats)
  /// assert!((lhs - rhs).abs() <= 0.0001);
  /// ```
  pub fn to_gradians(self) -> Gradian {
    const FACTOR: f32 = Gradian::ONE_REVOLUTION.0 / Degree::ONE_REVOLUTION.0;
    Gradian(self.0 * FACTOR)
  }

  /// Constructs a new [`Degree`] object from an existing `angle`.
  ///
  /// # Parameters
  ///
  /// * `angle` - the angle to construct from
  ///
  /// # Example
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::{Radian, Degree};
  ///
  /// assert_eq!(Degree::from_radians(Radian::ONE_REVOLUTION), Degree::ONE_REVOLUTION);
  /// ```
  pub fn from_radians(angle: Radian) -> Self {
    const FACTOR: f32 = Degree::ONE_REVOLUTION.0 / Radian::ONE_REVOLUTION.0;
    Self(angle.0 * FACTOR)
  }

  /// Constructs a new [`Degree`] object from an existing `angle`.
  ///
  /// This exists for symmetry with the other angle definitions. This is
  /// an identity function.
  ///
  /// # Parameters
  ///
  /// * `angle` - the angle to construct from
  ///
  /// # Example
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::Degree;
  ///
  /// assert_eq!(Degree::from_degrees(Degree::ONE_REVOLUTION), Degree::ONE_REVOLUTION);
  /// ```
  pub fn from_degrees(angle: Degree) -> Self {
    angle
  }

  /// Constructs a new [`Degree`] object from an existing `angle`.
  ///
  /// # Parameters
  ///
  /// * `angle` - the angle to construct from
  ///
  /// # Example
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::{Gradian, Degree};
  ///
  /// assert_eq!(Degree::from_gradians(Gradian::ONE_REVOLUTION), Degree::ONE_REVOLUTION);
  /// ```
  pub fn from_gradians(angle: Gradian) -> Self {
    const FACTOR: f32 = Degree::ONE_REVOLUTION.0 / Gradian::ONE_REVOLUTION.0;
    Self(angle.0 * FACTOR)
  }
}

impl std::fmt::Display for Degree {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}°", self.0)
  }
}

impl From<Radian> for Degree {
  #[inline(always)]
  fn from(other: Radian) -> Self {
    Self::from_radians(other)
  }
}

impl From<Gradian> for Degree {
  #[inline(always)]
  fn from(other: Gradian) -> Self {
    Self::from_gradians(other)
  }
}

impl From<Unit> for Degree {
  #[inline(always)]
  fn from(other: Unit) -> Self {
    Self::from_unit(other)
  }
}

impl AlmostEq<Radian> for Degree {
  fn almost_eq(&self, other: &Radian) -> bool {
    self.near(&other.to_degrees(), &ANGLE_EPSILON)
  }
}

impl AlmostEq<Degree> for Degree {
  fn almost_eq(&self, other: &Degree) -> bool {
    self.near(other, &ANGLE_EPSILON)
  }
}

impl AlmostEq<Gradian> for Degree {
  fn almost_eq(&self, other: &Gradian) -> bool {
    self.near(&other.to_degrees(), &ANGLE_EPSILON)
  }
}

impl AlmostEq<Unit> for Degree {
  fn almost_eq(&self, other: &Unit) -> bool {
    self.0.near(&other.to_degrees().0, &ANGLE_EPSILON)
  }
}

/// Representation of an angle in [Gradian]s.
///
/// In trigonometry, the gradian is a unit of measurement of an angle, defined
/// as one-hundredth of the right angle; in other words, 100 gradians is equal
/// to 90 degrees.
///
/// This type is provided so that developers can provide nicer/round numbers in
/// angular arithmetic, where desired.
///
/// [Gradian]: https://en.wikipedia.org/wiki/Gradian
#[derive(Copy, Clone, Debug, Default)]
#[repr(transparent)]
pub struct Gradian(pub f32);

impl std::fmt::Display for Gradian {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} grad", self.0)
  }
}

impl Gradian {
  /// Converts this [`Radian`] into [`Gradian`].
  ///
  /// # Example
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::{Radian, Gradian};
  ///
  /// assert_eq!(Gradian::ONE_REVOLUTION.to_radians(), Radian::ONE_REVOLUTION);
  /// ```
  pub fn to_radians(self) -> Radian {
    const FACTOR: f32 = Radian::ONE_REVOLUTION.0 / Gradian::ONE_REVOLUTION.0;
    Radian(self.0 * FACTOR)
  }

  /// Converts this [`Degree`] into [`Gradian`].
  ///
  /// # Example
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::{Gradian, Degree};
  ///
  /// assert_eq!(Gradian::ONE_REVOLUTION.to_degrees(), Degree::ONE_REVOLUTION);
  /// ```
  pub fn to_degrees(self) -> Degree {
    const FACTOR: f32 = Degree::ONE_REVOLUTION.0 / Gradian::ONE_REVOLUTION.0;
    Degree(self.0 * FACTOR)
  }

  /// Identity conversion.
  ///
  /// This exists for symmetry with the other angle definitions.
  ///
  /// # Example
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::Gradian;
  ///
  /// assert_eq!(Gradian::ONE_REVOLUTION.to_degrees(), Gradian::ONE_REVOLUTION);
  /// ```
  #[inline(always)]
  pub const fn to_gradians(self) -> Gradian {
    self
  }

  /// Constructs a new [`Gradian`] object from an existing `angle`.
  ///
  /// # Parameters
  ///
  /// * `angle` - the angle to construct from
  ///
  /// # Example
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::{Radian, Gradian};
  ///
  /// assert_eq!(Gradian::from_radians(Radian::ONE_REVOLUTION), Gradian::ONE_REVOLUTION);
  /// ```
  pub fn from_radians(other: Radian) -> Self {
    const FACTOR: f32 = Gradian::ONE_REVOLUTION.0 / Radian::ONE_REVOLUTION.0;
    Self(other.0 * FACTOR)
  }

  /// Constructs a new [`Gradian`] object from an existing `angle`.
  ///
  /// # Parameters
  ///
  /// * `angle` - the angle to construct from
  ///
  /// # Example
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::{Degree, Gradian};
  /// # use alloy::cmp::AlmostEq;
  ///
  /// let lhs = Gradian::from_degrees(Degree::ONE_REVOLUTION);
  /// let rhs = Gradian::ONE_REVOLUTION;
  /// // Degree => Gradian conversions are less exact (to ~5 decimal places with
  /// // 32-bit IEEE floats)
  /// assert!((lhs - rhs).abs() <= 0.0001);
  /// ```
  pub fn from_degrees(other: Degree) -> Self {
    const FACTOR: f32 = Gradian::ONE_REVOLUTION.0 / Degree::ONE_REVOLUTION.0;
    Self(other.0 * FACTOR)
  }

  /// Constructs a new [`Gradian`] object from an existing `angle`.
  ///
  /// This exists for symmetry with the other angle definitions. This is
  /// an identity function.
  ///
  /// # Parameters
  ///
  /// * `angle` - the angle to construct from
  ///
  /// # Example
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::Gradian;
  ///
  /// assert_eq!(Gradian::from_gradians(Gradian::ONE_REVOLUTION), Gradian::ONE_REVOLUTION);
  /// ```
  #[inline(always)]
  pub const fn from_gradians(other: Gradian) -> Self {
    other
  }
}

impl From<Radian> for Gradian {
  #[inline(always)]
  fn from(other: Radian) -> Self {
    Self::from_radians(other)
  }
}

impl From<Degree> for Gradian {
  #[inline(always)]
  fn from(other: Degree) -> Self {
    Self::from_degrees(other)
  }
}

impl From<Unit> for Gradian {
  #[inline(always)]
  fn from(other: Unit) -> Self {
    Self::from_unit(other)
  }
}

impl AlmostEq<Radian> for Gradian {
  fn almost_eq(&self, other: &Radian) -> bool {
    self.near(&other.to_gradians(), &ANGLE_EPSILON)
  }
}

impl AlmostEq<Degree> for Gradian {
  fn almost_eq(&self, other: &Degree) -> bool {
    self.near(&other.to_gradians(), &ANGLE_EPSILON)
  }
}

impl AlmostEq<Gradian> for Gradian {
  fn almost_eq(&self, other: &Gradian) -> bool {
    self.near(other, &ANGLE_EPSILON)
  }
}

impl AlmostEq<Unit> for Gradian {
  fn almost_eq(&self, other: &Unit) -> bool {
    self.near(&other.to_gradians(), &ANGLE_EPSILON)
  }
}

/// Representation of a unit-based [`Angle`].
///
/// A unit is an angle where each revolution is exactly `1`. This makes conversion
/// to and from other angular units simple; e.g. 1 [`Unit`] is 360 [`Degree`]
/// units which means conversion to [`Degree`] is just a simple multiplication.
/// Likewise, the inverse conversion is a simple division operation.
#[derive(Copy, Clone, Debug, Default)]
#[repr(transparent)]
pub struct Unit(pub f32);

impl Unit {
  /// Converts this [`Unit`] into [`Radian`].
  ///
  /// # Example
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::{Radian, Unit};
  ///
  /// assert_eq!(Unit::ONE_REVOLUTION.to_radians(), Radian::ONE_REVOLUTION);
  /// ```
  #[inline]
  pub fn to_radians(self) -> Radian {
    Radian(self.0 * Radian::ONE_REVOLUTION.0)
  }

  /// Converts this [`Unit`] into [`Degree`].
  ///
  /// # Example
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::{Degree, Unit};
  ///
  /// assert_eq!(Unit::ONE_REVOLUTION.to_degrees(), Degree::ONE_REVOLUTION);
  /// ```
  #[inline]
  pub fn to_degrees(self) -> Degree {
    Degree(self.0 * Degree::ONE_REVOLUTION.0)
  }

  /// Converts this [`Unit`] into [`Gradian`].
  ///
  /// # Example
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::{Gradian, Unit};
  ///
  /// assert_eq!(Unit::ONE_REVOLUTION.to_gradians(), Gradian::ONE_REVOLUTION);
  /// ```
  #[inline]
  pub fn to_gradians(self) -> Gradian {
    Gradian(self.0 * Gradian::ONE_REVOLUTION.0)
  }

  /// Constructs a new [`Unit`] object from an existing `angle`.
  ///
  /// # Parameters
  ///
  /// * `angle` - the angle to construct from
  ///
  /// # Example
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::{Radian, Unit};
  ///
  /// assert_eq!(Unit::from_radians(Radian::ONE_REVOLUTION), Unit::ONE_REVOLUTION);
  /// ```
  #[inline]
  pub fn from_radians(angle: Radian) -> Self {
    const FACTOR: f32 = 1.0 / Radian::ONE_REVOLUTION.0;
    Self(angle.0 * FACTOR)
  }

  /// Constructs a new [`Unit`] object from an existing `angle`.
  ///
  /// # Parameters
  ///
  /// * `angle` - the angle to construct from
  ///
  /// # Example
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::{Degree, Unit};
  ///
  /// assert_eq!(Unit::from_degrees(Degree::ONE_REVOLUTION), Unit::ONE_REVOLUTION);
  /// ```
  #[inline]
  pub fn from_degrees(angle: Degree) -> Self {
    const FACTOR: f32 = 1.0 / Degree::ONE_REVOLUTION.0;
    Self(angle.0 * FACTOR)
  }

  /// Constructs a new [`Unit`] object from an existing `angle`.
  ///
  /// # Parameters
  ///
  /// * `angle` - the angle to construct from
  ///
  /// # Example
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::{Gradian, Unit};
  ///
  /// assert_eq!(Unit::from_gradians(Gradian::ONE_REVOLUTION), Unit::ONE_REVOLUTION);
  /// ```
  #[inline]
  pub fn from_gradians(angle: Gradian) -> Self {
    const FACTOR: f32 = 1.0 / Gradian::ONE_REVOLUTION.0;
    Self(angle.0 * FACTOR)
  }
}

impl std::fmt::Display for Unit {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} unit", self.0)
  }
}

impl From<Radian> for Unit {
  #[inline(always)]
  fn from(other: Radian) -> Self {
    Self::from_radians(other)
  }
}

impl From<Degree> for Unit {
  #[inline(always)]
  fn from(other: Degree) -> Self {
    Self::from_degrees(other)
  }
}

impl From<Gradian> for Unit {
  #[inline(always)]
  fn from(other: Gradian) -> Self {
    Self::from_gradians(other)
  }
}

impl AlmostEq<Radian> for Unit {
  fn almost_eq(&self, other: &Radian) -> bool {
    self.near(&other.to_unit(), &ANGLE_EPSILON)
  }
}

impl AlmostEq<Degree> for Unit {
  fn almost_eq(&self, other: &Degree) -> bool {
    self.near(&other.to_unit(), &ANGLE_EPSILON)
  }
}

impl AlmostEq<Gradian> for Unit {
  fn almost_eq(&self, other: &Gradian) -> bool {
    self.near(&other.to_unit(), &ANGLE_EPSILON)
  }
}

impl AlmostEq<Unit> for Unit {
  fn almost_eq(&self, other: &Unit) -> bool {
    self.near(other, &UNIT_EPSILON)
  }
}

macro_rules! define_angle_ops {
  ($($Trait:ident::$Fn:ident($op:tt) for $Type:ident::$to:tt: [$($For:ident$(,)?)+]$(,)?)+) => {
    $(
      $(
        impl std::ops::$Trait<$For> for $Type {
          type Output = $Type;

          #[inline(always)]
          fn $Fn(self, rhs: $For) -> Self::Output {
            Self(self.0 $op rhs.$to().0)
          }
        }
      )+
    )+
  }
}

macro_rules! define_angle_compound_ops {
  ($($Trait:ident::$Fn:ident($op:tt) for $Type:ident::$to:tt: [$($For:ident$(,)?)+]$(,)?)+) => {
    $(
      $(
        impl std::ops::$Trait<$For> for $Type {
          #[inline(always)]
          fn $Fn(&mut self, rhs: $For) {
            self.0 $op rhs.$to().0
          }
        }
      )+
    )+
  }
}

macro_rules! define_angle {
  ($($Type:ident($Revolution:expr): $from:tt => $to:tt$(,)?)+) => {
    $(
      impl $Type {
        /// Constant for the number of units of [`Self`] required to perform one
        /// full revolution around a unit circle.
        pub const ONE_REVOLUTION: Self = Self($Revolution);

        /// Constructs a new instance of this angle type
        ///
        /// # Parameters
        ///
        /// * `value` - the number of units
        pub const fn new(value: f32) -> Self {
          Self(value)
        }

        /// Constructs a new instance of this angle by constraining the result
        /// to be within one revolution.
        ///
        /// **Note:** This will
        pub fn constrained(self) -> Self {
          let mut units = self.0 % Self::ONE_REVOLUTION.0;
          if units < 0.0 {
            units += Self::ONE_REVOLUTION.0
          }
          Self(units)
        }

        /// Counts the number of times this angle has revolved round the unit
        /// circle.
        pub fn revolutions(self) -> f32 {
          self.0 / Self::ONE_REVOLUTION.0
        }

        /// Computes the absolute value of this angle and returns it.
        pub fn abs(self) -> Self {
          Self(self.0.abs())
        }
      }

      impl sealed::Sealed for $Type {}

      impl FromUnit for $Type {
        fn from_unit(angle: Unit) -> Self {
          const FACTOR: f32 = $Type::ONE_REVOLUTION.0;
          Self(angle.0 * FACTOR)
        }
      }

      impl ToUnit for $Type {
        /// Converts this [`Degree`] into [`Unit`].
        fn to_unit(self) -> Unit {
          const FACTOR: f32 = 1.0 / $Type::ONE_REVOLUTION.0;
          Unit(self.0 * FACTOR)
        }
      }

      impl Trig for $Type {
        fn sin(self) -> f32 {
          self.to_radians().0.sin()
        }

        fn cos(self) -> f32 {
          self.to_radians().0.cos()
        }

        fn tan(self) -> f32 {
          self.to_radians().0.tan()
        }

        fn sin_cos(self) -> (f32, f32) {
          self.to_radians().0.sin_cos()
        }

        fn asin(value: f32) -> Self {
          Radian(value.asin()).$to()
        }

        fn acos(value: f32) -> Self {
          Radian(value.acos()).$to()
        }

        fn atan(value: f32) -> Self {
          Radian(value.atan()).$to()
        }
      }

      impl Angle for $Type {}

      impl PartialEq<Radian> for $Type
      {
        #[inline]
        fn eq(&self, other: &Radian) -> bool {
          self.0 == other.$to().0
        }
      }
      impl PartialEq<Degree> for $Type
      {
        #[inline]
        fn eq(&self, other: &Degree) -> bool {
          self.0 == other.$to().0
        }
      }
      impl PartialEq<Gradian> for $Type
      {
        #[inline]
        fn eq(&self, other: &Gradian) -> bool {
          self.0 == other.$to().0
        }
      }
      impl PartialEq<Unit> for $Type
      {
        #[inline]
        fn eq(&self, other: &Unit) -> bool {
          self.0 == other.$to().0
        }
      }
      impl PartialEq<f32> for $Type
      {
        #[inline(always)]
        fn eq(&self, other: &f32) -> bool {
          self.0.eq(other)
        }
      }
      impl PartialEq<$Type> for f32
      {
        #[inline(always)]
        fn eq(&self, other: &$Type) -> bool {
          self.eq(&other.0)
        }
      }

      impl PartialOrd<Radian> for $Type
      {
        #[inline]
        fn partial_cmp(&self, other: &Radian) -> Option<std::cmp::Ordering> {
          self.0.partial_cmp(&other.$to().0)
        }
      }
      impl PartialOrd<Gradian> for $Type
      {
        #[inline]
        fn partial_cmp(&self, other: &Gradian) -> Option<std::cmp::Ordering> {
          self.0.partial_cmp(&other.$to().0)
        }
      }
      impl PartialOrd<Degree> for $Type
      {
        #[inline]
        fn partial_cmp(&self, other: &Degree) -> Option<std::cmp::Ordering> {
          self.0.partial_cmp(&other.$to().0)
        }
      }
      impl PartialOrd<Unit> for $Type
      {
        #[inline]
        fn partial_cmp(&self, other: &Unit) -> Option<std::cmp::Ordering> {
          self.0.partial_cmp(&other.$to().0)
        }
      }
      impl PartialOrd<f32> for $Type
      {
        #[inline(always)]
        fn partial_cmp(&self, other: &f32) -> Option<std::cmp::Ordering> {
          self.0.partial_cmp(other)
        }
      }
      impl PartialOrd<$Type> for f32
      {
        #[inline(always)]
        fn partial_cmp(&self, other: &$Type) -> Option<std::cmp::Ordering> {
          self.partial_cmp(&other.0)
        }
      }

      impl Near<Radian> for $Type
      {
        #[inline]
        fn near(&self, other: &Radian, tolerance: &f32) -> bool {
          self.0.near(&other.$to().0, tolerance)
        }
      }
      impl Near<Gradian> for $Type
      {
        #[inline]
        fn near(&self, other: &Gradian, tolerance: &f32) -> bool {
          self.0.near(&other.$to().0, tolerance)
        }
      }
      impl Near<Degree> for $Type
      {
        #[inline]
        fn near(&self, other: &Degree, tolerance: &f32) -> bool {
          self.0.near(&other.$to().0, tolerance)
        }
      }
      impl Near<Unit> for $Type
      {
        #[inline]
        fn near(&self, other: &Unit, tolerance: &f32) -> bool {
          self.0.near(&other.$to().0, tolerance)
        }
      }
      impl Near<f32> for $Type
      {
        #[inline(always)]
        fn near(&self, other: &f32, tolerance: &f32) -> bool {
          self.0.near(other, tolerance)
        }
      }
      impl Near<$Type> for f32
      {
        #[inline(always)]
        fn near(&self, other: &$Type, tolerance: &f32) -> bool {
          self.near(&other.0, tolerance)
        }
      }

      define_angle_ops!{
        Add::add(+) for $Type::$to: [Radian, Degree, Gradian, Unit],
        Sub::sub(-) for $Type::$to: [Radian, Degree, Gradian, Unit],
      }
      define_angle_compound_ops!{
        AddAssign::add_assign(+=) for $Type::$to: [Radian, Degree, Gradian, Unit],
        SubAssign::sub_assign(-=) for $Type::$to: [Radian, Degree, Gradian, Unit],
      }

      impl std::ops::Mul<f32> for $Type {
        type Output = $Type;

        #[inline(always)]
        fn mul(self, rhs: f32) -> Self::Output {
          Self(self.0 * rhs)
        }
      }

      impl std::ops::Mul<$Type> for f32 {
        type Output = $Type;

        #[inline(always)]
        fn mul(self, rhs: $Type) -> Self::Output {
          $Type(self * rhs.0)
        }
      }

      impl std::ops::MulAssign for $Type {
        fn mul_assign(&mut self, rhs: Self) {
          self.0 *= rhs.0
        }
      }

      impl std::ops::Div<f32> for $Type {
        type Output = $Type;

        #[inline(always)]
        fn div(self, rhs: f32) -> Self::Output {
          Self(self.0 / rhs)
        }
      }

      impl std::ops::DivAssign for $Type {
        fn div_assign(&mut self, rhs: Self) {
          self.0 /= rhs.0
        }
      }

      impl std::ops::Rem<f32> for $Type {
        type Output = $Type;

        #[inline(always)]
        fn rem(self, rhs: f32) -> Self::Output {
          Self(self.0 % rhs)
        }
      }

      impl std::ops::RemAssign for $Type {
        fn rem_assign(&mut self, rhs: Self) {
          self.0 %= rhs.0
        }
      }

      unsafe impl astd::convert::Transparent for $Type {
        type Wrapped = f32;
      }

      impl AsRef<f32> for $Type {
        fn as_ref(&self) -> &f32 {
          &self.0
        }
      }

      impl AsMut<f32> for $Type {
        fn as_mut(&mut self) -> &mut f32 {
          &mut self.0
        }
      }

      impl std::ops::Deref for $Type {
        type Target = f32;

        fn deref(&self) -> &f32 {
          &self.0
        }
      }

      impl std::ops::DerefMut for $Type {
        fn deref_mut(&mut self) -> &mut f32 {
          &mut self.0
        }
      }
    )+
  };
}

define_angle![
  Radian(std::f32::consts::TAU): from_radians => to_radians,
  Degree(360.0): from_degrees => to_degrees,
  Gradian(400.0): from_gradians => to_gradians,
  Unit(1.0): from_unit => to_unit,
];
