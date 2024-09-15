use std::borrow::{Borrow, BorrowMut};
use std::ops::{Add, AddAssign, Deref, DerefMut, Div, Mul, Sub, SubAssign};

use crate::math::vec::Vec3;
use crate::math::{Angle, Radian};

/// Euler angles are a representation of orientation in 3D space.
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct EulerAngles([Radian; 3]);

// Constructors

impl EulerAngles {
  /// Create a new Euler angle with the given yaw, pitch, and roll.
  ///
  /// # Parameters
  ///
  /// * `yaw` - The yaw angle.
  /// * `pitch` - The pitch angle.
  /// * `roll` - The roll angle.
  #[must_use]
  #[inline(always)]
  pub fn new<A: Angle>(yaw: A, pitch: A, roll: A) -> Self {
    Self([yaw.to_angle(), pitch.to_angle(), roll.to_angle()])
  }

  /// Create a new Euler angle with the given yaw, pitch, and roll in radians.
  ///
  /// # Parameters
  ///
  /// * `yaw` - The yaw angle in radians.
  /// * `pitch` - The pitch angle in radians.
  /// * `roll` - The roll angle in radians.
  #[must_use]
  #[inline(always)]
  pub const fn from_radians(yaw: Radian, pitch: Radian, roll: Radian) -> Self {
    Self([yaw, pitch, roll])
  }

  /// Create a new Euler angle with the given yaw.
  ///
  /// # Parameters
  ///
  /// * `yaw` - The yaw angle.
  #[must_use]
  #[inline(always)]
  pub fn from_yaw<A: Angle>(yaw: A) -> Self {
    Self([yaw.to_angle(), Radian::new(0.0), Radian::new(0.0)])
  }

  /// Create a new Euler angle with the given pitch.
  ///
  /// # Parameters
  ///
  /// * `pitch` - The pitch angle.
  #[must_use]
  #[inline(always)]
  pub fn from_pitch<A: Angle>(pitch: A) -> Self {
    Self([Radian::new(0.0), pitch.to_angle(), Radian::new(0.0)])
  }

  /// Create a new Euler angle with the given roll.
  ///
  /// # Parameters
  ///
  /// * `roll` - The roll angle.
  #[must_use]
  #[inline(always)]
  pub fn from_roll<A: Angle>(roll: A) -> Self {
    Self([Radian::new(0.0), Radian::new(0.0), roll.to_angle()])
  }
}

// Conversions

impl EulerAngles {
  /// Convert the Euler angles to a 3D vector.
  #[must_use]
  #[inline(always)]
  pub const fn as_vec3(&self) -> &Vec3 {
    // Safety: [Angle] and [f32] are layout compatible because Angle is repr(transparent) over f32.
    let slice: &[f32] = unsafe { std::mem::transmute(self.0.as_slice()) };

    // Safety: The slice is of length 3.
    unsafe { Vec3::from_slice_unchecked(slice) }
  }

  /// Convert the Euler angles to a mutable 3D vector.
  #[must_use]
  #[inline(always)]
  pub fn as_vec3_mut(&mut self) -> &mut Vec3 {
    // Safety: [Angle] and [f32] are layout compatible because Angle is repr(transparent) over f32.
    let slice: &mut [f32] = unsafe { std::mem::transmute(self.0.as_mut_slice()) };

    // Safety: The slice is of length 3.
    unsafe { Vec3::from_mut_slice_unchecked(slice) }
  }

  /// Convert the Euler angles to a tuple of yaw, pitch, and roll.
  #[must_use]
  #[inline(always)]
  pub fn to_tuple<A: Angle>(&self) -> (A, A, A) {
    (
      self.0[0].to_angle::<A>(),
      self.0[1].to_angle::<A>(),
      self.0[2].to_angle::<A>(),
    )
  }
}

impl Borrow<Vec3> for EulerAngles {
  #[must_use]
  #[inline(always)]
  fn borrow(&self) -> &Vec3 {
    self.as_vec3()
  }
}

impl BorrowMut<Vec3> for EulerAngles {
  #[must_use]
  #[inline(always)]
  fn borrow_mut(&mut self) -> &mut Vec3 {
    self.as_vec3_mut()
  }
}

impl AsRef<Vec3> for EulerAngles {
  #[must_use]
  #[inline(always)]
  fn as_ref(&self) -> &Vec3 {
    self.as_vec3()
  }
}

impl AsMut<Vec3> for EulerAngles {
  #[must_use]
  #[inline(always)]
  fn as_mut(&mut self) -> &mut Vec3 {
    self.as_vec3_mut()
  }
}

impl Deref for EulerAngles {
  type Target = Vec3;

  #[must_use]
  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    self.as_vec3()
  }
}

impl DerefMut for EulerAngles {
  #[must_use]
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.as_vec3_mut()
  }
}

// Properties

impl EulerAngles {
  /// Return the yaw angle of the Euler angles.
  #[must_use]
  #[inline(always)]
  pub const fn yaw(&self) -> Radian {
    self.0[0]
  }

  /// Return the pitch angle of the Euler angles.
  #[must_use]
  #[inline(always)]
  pub const fn pitch(&self) -> Radian {
    self.0[1]
  }

  /// Return the roll angle of the Euler angles.
  #[must_use]
  #[inline(always)]
  pub const fn roll(&self) -> Radian {
    self.0[2]
  }

  /// Return a mutable reference to the yaw angle of the Euler angles.
  #[must_use]
  #[inline(always)]
  pub fn yaw_mut(&mut self) -> &mut Radian {
    &mut self.0[0]
  }

  /// Return a mutable reference to the pitch angle of the Euler angles.
  #[must_use]
  #[inline(always)]
  pub fn pitch_mut(&mut self) -> &mut Radian {
    &mut self.0[1]
  }

  /// Return a mutable reference to the roll angle of the Euler angles.
  #[must_use]
  #[inline(always)]
  pub fn roll_mut(&mut self) -> &mut Radian {
    &mut self.0[2]
  }
}

// Modifiers

impl EulerAngles {
  /// Set the yaw angle of the Euler angles.
  ///
  /// # Parameters
  ///
  /// * `yaw` - The new yaw angle.
  pub fn set_yaw<A: Angle>(&mut self, yaw: A) {
    self.0[0] = yaw.to_angle();
  }

  /// Set the pitch angle of the Euler angles.
  ///
  /// # Parameters
  ///
  /// * `pitch` - The new pitch angle.
  pub fn set_pitch<A: Angle>(&mut self, pitch: A) {
    self.0[1] = pitch.to_angle();
  }

  /// Set the roll angle of the Euler angles.
  ///
  /// # Parameters
  ///
  /// * `roll` - The new roll angle.
  pub fn set_roll<A: Angle>(&mut self, roll: A) {
    self.0[2] = roll.to_angle();
  }
}

// Arithmetic

impl Add for EulerAngles {
  type Output = Self;

  #[inline(always)]
  fn add(self, rhs: Self) -> Self::Output {
    Self([
      self.0[0] + rhs.0[0],
      self.0[1] + rhs.0[1],
      self.0[2] + rhs.0[2],
    ])
  }
}

impl Add for &EulerAngles {
  type Output = EulerAngles;

  #[inline(always)]
  fn add(self, rhs: Self) -> Self::Output {
    EulerAngles([
      self.0[0] + rhs.0[0],
      self.0[1] + rhs.0[1],
      self.0[2] + rhs.0[2],
    ])
  }
}

impl AddAssign for EulerAngles {
  fn add_assign(&mut self, rhs: Self) {
    self.0[0] += rhs.0[0];
    self.0[1] += rhs.0[1];
    self.0[2] += rhs.0[2];
  }
}

impl AddAssign<&EulerAngles> for EulerAngles {
  fn add_assign(&mut self, rhs: &EulerAngles) {
    self.0[0] += rhs.0[0];
    self.0[1] += rhs.0[1];
    self.0[2] += rhs.0[2];
  }
}

impl Sub for EulerAngles {
  type Output = Self;

  #[inline(always)]
  fn sub(self, rhs: Self) -> Self::Output {
    EulerAngles([
      self.0[0] - rhs.0[0],
      self.0[1] - rhs.0[1],
      self.0[2] - rhs.0[2],
    ])
  }
}

impl Sub for &EulerAngles {
  type Output = EulerAngles;

  #[inline(always)]
  fn sub(self, rhs: Self) -> Self::Output {
    EulerAngles([
      self.0[0] - rhs.0[0],
      self.0[1] - rhs.0[1],
      self.0[2] - rhs.0[2],
    ])
  }
}

impl SubAssign for EulerAngles {
  fn sub_assign(&mut self, rhs: Self) {
    self.0[0] -= rhs.0[0];
    self.0[1] -= rhs.0[1];
    self.0[2] -= rhs.0[2];
  }
}

impl SubAssign<&EulerAngles> for EulerAngles {
  fn sub_assign(&mut self, rhs: &EulerAngles) {
    self.0[0] -= rhs.0[0];
    self.0[1] -= rhs.0[1];
    self.0[2] -= rhs.0[2];
  }
}

impl Mul<f32> for EulerAngles {
  type Output = Self;

  #[inline(always)]
  fn mul(self, rhs: f32) -> Self::Output {
    EulerAngles([self.0[0] * rhs, self.0[1] * rhs, self.0[2] * rhs])
  }
}

impl Mul<f32> for &EulerAngles {
  type Output = EulerAngles;

  #[inline(always)]
  fn mul(self, rhs: f32) -> Self::Output {
    EulerAngles([self.0[0] * rhs, self.0[1] * rhs, self.0[2] * rhs])
  }
}

impl Mul<EulerAngles> for f32 {
  type Output = EulerAngles;

  #[inline(always)]
  fn mul(self, rhs: EulerAngles) -> Self::Output {
    EulerAngles([rhs.0[0] * self, rhs.0[1] * self, rhs.0[2] * self])
  }
}

impl Mul<&EulerAngles> for f32 {
  type Output = EulerAngles;

  #[inline(always)]
  fn mul(self, rhs: &EulerAngles) -> Self::Output {
    EulerAngles([rhs.0[0] * self, rhs.0[1] * self, rhs.0[2] * self])
  }
}

impl Div<f32> for EulerAngles {
  type Output = Self;

  #[inline(always)]
  fn div(self, rhs: f32) -> Self::Output {
    self * (1.0 / rhs)
  }
}

impl Div<f32> for &EulerAngles {
  type Output = EulerAngles;

  #[inline(always)]
  fn div(self, rhs: f32) -> Self::Output {
    self * (1.0 / rhs)
  }
}
