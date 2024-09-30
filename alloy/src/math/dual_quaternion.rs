use std::ops;

use crate::math::vec::Vec3;
use crate::math::{Angle, Quaternion};

use super::mat::Matrix4;
use super::vec::Vector3;

/// A dual-quaternion represents both translations and rotations in a combined
/// format.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DualQuaternion {
  real: Quaternion,
  dual: Quaternion,
}

// Construction

impl DualQuaternion {
  /// Creates a new dual quaternion from a real and dual quaternion.
  ///
  /// # Parameters
  ///
  /// * `real` - The real part of the dual quaternion.
  /// * `dual` - The dual part of the dual quaternion.
  #[inline(always)]
  #[must_use]
  pub const fn new(real: Quaternion, dual: Quaternion) -> Self {
    Self { real, dual }
  }

  /// Constructs a new dual quaternion with the identity rotation and no translation.
  #[inline(always)]
  #[must_use]
  pub const fn identity() -> Self {
    Self::new(Quaternion::identity(), Quaternion::ZERO)
  }

  /// Constructs this dual quaternion from a rotation quaternion.
  ///
  /// # Parameters
  ///
  /// * `rotation` - The rotation quaternion to construct the dual quaternion from.
  #[must_use]
  pub const fn from_rotation(rotation: Quaternion) -> Self {
    Self::new(rotation, Quaternion::ZERO)
  }

  /// Constructs this dual quaternion from the yaw angle
  ///
  /// # Parameters
  ///
  /// * `angle` - The yaw angle.
  #[inline(always)]
  #[must_use]
  pub fn from_yaw<A: Angle>(angle: A) -> Self {
    Self::from_rotation(Quaternion::from_yaw(angle))
  }

  /// Constructs this dual quaternion from the pitch angle
  ///
  /// # Parameters
  ///
  /// * `angle` - The pitch angle.
  #[inline(always)]
  #[must_use]
  pub fn from_pitch<A: Angle>(angle: A) -> Self {
    Self::from_rotation(Quaternion::from_pitch(angle))
  }

  /// Constructs this dual quaternion from the roll angle
  ///
  /// # Parameters
  ///
  /// * `angle` - The roll angle.
  #[inline(always)]
  #[must_use]
  pub fn from_roll<A: Angle>(angle: A) -> Self {
    Self::from_rotation(Quaternion::from_roll(angle))
  }

  /// Constructs this dual quaternion from a translation vector.
  ///
  /// # Parameters
  ///
  /// * `translation` - The translation vector to construct the dual quaternion from.
  pub fn from_translation(translation: &Vec3) -> Self {
    Self::new(
      Quaternion::identity(),
      Quaternion::new(
        0.0,
        0.5 * translation.x(),
        0.5 * translation.y(),
        0.5 * translation.z(),
      ),
    )
  }
}

// Conversions
impl DualQuaternion {
  /// Returns the transform matrix representation of the quaternion.
  #[must_use]
  pub fn to_matrix4(&self) -> Matrix4 {
    let translation = self.translation();
    let mut matrix4 = self.real.to_matrix4();
    matrix4[0][3] = translation.x();
    matrix4[1][3] = translation.y();
    matrix4[2][3] = translation.z();
    matrix4
  }
}

// Properties
impl DualQuaternion {
  /// Returns the real part of the dual quaternion.
  #[must_use]
  #[inline(always)]
  pub const fn real(&self) -> &Quaternion {
    &self.real
  }

  /// Returns the dual part of the dual quaternion.
  #[must_use]
  #[inline(always)]
  pub const fn dual(&self) -> &Quaternion {
    &self.dual
  }

  /// Returns the translation component in local-space.
  #[must_use]
  pub fn translation(&self) -> Vector3 {
    let dual = Quaternion::new(
      self.dual.w() * 2.0,
      self.dual.i() * 2.0,
      self.dual.j() * 2.0,
      self.dual.k() * 2.0,
    );

    let translation_quat = dual * self.real.conjugate();

    Vector3::new(
      translation_quat.i(),
      translation_quat.j(),
      translation_quat.k(),
    )
  }

  /// Returns the translation of the dual quaternion in world-space.
  #[must_use]
  pub fn world_translation(&self) -> Vector3 {
    Vector3::new(
      self.dual.i() * 2.0,
      self.dual.j() * 2.0,
      self.dual.k() * 2.0,
    )
  }

  /// Returns the rotation component of the dual quaternion.
  #[must_use]
  #[inline(always)]
  pub const fn rotation(&self) -> &Quaternion {
    &self.real
  }

  /// Returns a normalized DualQuaternion.
  #[must_use]
  #[inline(always)]
  pub fn normalized(&self) -> DualQuaternion {
    let norm = self.real.norm();
    Self {
      real: &self.real / norm,
      dual: &self.dual / norm,
    }
  }

  /// Returns the conjugate of the dual quaternion.
  #[must_use]
  pub fn conjugate(&self) -> DualQuaternion {
    Self {
      real: self.real.conjugate(),
      dual: self.dual.conjugate(),
    }
  }
}

// Modifiers
impl DualQuaternion {
  /// Normalizes the dual quaternion.
  pub fn normalize(&mut self) {
    let norm = self.real.norm();
    self.real /= norm;
    self.dual /= norm;
  }

  /// Translates the dual quaternion by the given vector in world space.
  ///
  /// # Parameters
  ///
  /// * `translation` - The translation vector to apply to the dual quaternion.
  pub fn translate_world(&mut self, translation: &Vec3) {
    self.dual += &Quaternion::new(
      0.0,
      0.5 * translation.x(),
      0.5 * translation.y(),
      0.5 * translation.z(),
    );
  }

  /// Translates the dual quaternion by the given vector in local space.
  ///
  /// This will translate relative to the orientation of the dual quaternion.
  ///
  /// # Parameters
  ///
  /// * `translation` - The translation vector to apply to the dual quaternion.
  pub fn translate(&mut self, translation: &Vec3) {
    self.dual += &(Quaternion::new(
      0.0,
      0.5 * translation.x(),
      0.5 * translation.y(),
      0.5 * translation.z(),
    ) * self.real);
  }

  /// Rotates the dual quaternion by the given angle.
  pub fn rotate(&mut self, rotation: &Quaternion) {
    self.real = rotation * self.real;
    self.dual = rotation * self.dual;
  }

  /// Rotates the dual quaternion by the given angle around the x-axis.
  #[inline(always)]
  pub fn rotate_yaw<A: Angle>(&mut self, angle: A) {
    self.rotate(&Quaternion::from_yaw(angle));
  }

  /// Rotates the dual quaternion by the given angle around the y-axis.
  #[inline(always)]
  pub fn rotate_pitch<A: Angle>(&mut self, angle: A) {
    self.rotate(&Quaternion::from_pitch(angle));
  }

  /// Rotates the dual quaternion by the given angle around the z-axis.
  #[inline(always)]
  pub fn rotate_roll<A: Angle>(&mut self, angle: A) {
    self.rotate(&Quaternion::from_roll(angle));
  }
}

impl ops::Mul for DualQuaternion {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self {
    Self {
      real: self.real * rhs.real,
      dual: (self.real * rhs.dual) + (self.dual * rhs.real),
    }
  }
}

impl ops::Add for DualQuaternion {
  type Output = Self;

  fn add(self, rhs: Self) -> Self {
    Self {
      real: self.real + rhs.real,
      dual: self.dual + rhs.dual,
    }
  }
}

#[cfg(test)]
#[path = "dual_quaternion.test.rs"]
mod test;
