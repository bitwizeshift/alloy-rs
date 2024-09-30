//! Module containing definitions of [Quaternion] and [DualQuaternion]
//! objects.
//!
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::cmp::{AlmostEq, Near};
use crate::ops::Lerp;
use crate::{math::vec::Vector4, ops::Dot};

use super::{
  mat::{Mat4, Matrix4},
  vec::{Vec2, Vec3, Vec4, Vector3},
  Angle, Radian,
};

// use super::vec::Vec4;

/// Represents a quaternion in 4D space.
#[derive(Clone, Copy, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Quaternion(Vector4);

// Constructors

impl Quaternion {
  /// The zero quaternion.
  pub const ZERO: Quaternion = Quaternion(Vector4::ZERO);

  /// The identity quaternion.
  pub const UNIT: Quaternion = Quaternion::new(1.0, 0.0, 0.0, 0.0);

  /// Creates a new quaternion with the given components.
  ///
  /// # Parameters
  ///
  /// * `w` - The real component of the quaternion.
  /// * `i` - The i imaginary component of the quaternion.
  /// * `j` - The j imaginary component of the quaternion.
  /// * `k` - The k imaginary component of the quaternion.
  #[must_use]
  #[inline(always)]
  pub const fn new(w: f32, i: f32, j: f32, k: f32) -> Self {
    Self(Vector4::new(w, i, j, k))
  }

  /// Returns the unit/identity quaternion.
  #[must_use]
  #[inline(always)]
  pub const fn identity() -> Self {
    Self::UNIT
  }

  /// Creates a new quaternion from a 4D vector.
  ///
  /// # Parameters
  ///
  /// * `vec` - The vector to create the quaternion from.
  #[must_use]
  #[inline(always)]
  pub const fn from_vector4(vec: Vector4) -> Self {
    Self(vec)
  }

  /// Creates a new quaternion from a 4D vector.
  ///
  /// # Parameters
  ///
  /// * `vec` - The vector to create the quaternion from.
  #[must_use]
  #[inline(always)]
  pub const fn from_vec4(vec: &Vec4) -> Self {
    Self(Vector4::from_vec4(vec))
  }

  /// Creates a quaternion from an angle (in radians) and an axis vector (x, y, z)
  ///
  /// # Parameters
  ///
  /// * `angle` - The angle of rotation.
  /// * `axis` - The axis of rotation.
  pub fn from_angle_axis<A: Angle>(angle: A, axis: &Vec3) -> Self {
    let norm = axis.magnitude();

    let half_angle = angle / 2.0;
    let (sin_half_angle, cos_half_angle) = half_angle.sin_cos();

    if norm != 0.0 {
      let naxis = axis / norm;
      Quaternion::new(
        cos_half_angle,
        naxis.x() * sin_half_angle,
        naxis.y() * sin_half_angle,
        naxis.z() * sin_half_angle,
      )
    } else {
      Quaternion::new(1.0, 0.0, 0.0, 0.0)
    }
  }

  /// Creates a quaternion from a yaw angle.
  ///
  /// # Parameters
  ///
  /// * `angle` - The yaw angle.
  pub fn from_yaw<A: Angle>(angle: A) -> Self {
    let half_angle = angle / 2.0;
    let (sin_half_angle, cos_half_angle) = half_angle.sin_cos();

    Quaternion::new(cos_half_angle, 0.0, 0.0, sin_half_angle)
  }

  /// Creates a quaternion from a pitch angle.
  ///
  /// # Parameters
  ///
  /// * `angle` - The pitch angle.
  pub fn from_pitch<A: Angle>(angle: A) -> Self {
    let half_angle = angle / 2.0;
    let (sin_half_angle, cos_half_angle) = half_angle.sin_cos();

    Quaternion::new(cos_half_angle, 0.0, sin_half_angle, 0.0)
  }

  /// Creates a quaternion from a roll angle.
  ///
  /// # Parameters
  ///
  /// * `angle` - The roll angle.
  pub fn from_roll<A: Angle>(angle: A) -> Self {
    let half_angle = angle / 2.0;
    let (sin_half_angle, cos_half_angle) = half_angle.sin_cos();

    Quaternion::new(cos_half_angle, sin_half_angle, 0.0, 0.0)
  }

  /// Creates a new quaternion from a set of Euler angles.
  ///
  /// # Parameters
  ///
  /// * `yaw`   - The yaw angle.
  /// * `pitch` - The pitch angle.
  /// * `roll`  - The roll angle.
  pub fn from_euler_angles<A: Angle>(yaw: A, pitch: A, roll: A) -> Self {
    let half_yaw = yaw / 2.0;
    let half_pitch = pitch / 2.0;
    let half_roll = roll / 2.0;

    let (sy, cy) = half_yaw.sin_cos();
    let (sp, cp) = half_pitch.sin_cos();
    let (sr, cr) = half_roll.sin_cos();

    let w = cr * cp * cy + sr * sp * sy;
    let i = sr * cp * cy - cr * sp * sy;
    let j = cr * sp * cy + sr * cp * sy;
    let k = cr * cp * sy - sr * sp * cy;

    Self::new(w, i, j, k)
  }

  /// Creates a new quaternion from a 4x4 matrix encoding a rotation.
  ///
  /// # Parameters
  ///
  /// * `matrix` - The rotation matrix to convert.
  pub fn from_mat4(matrix: &Mat4) -> Self {
    let m00 = matrix[0][0];
    let m11 = matrix[1][1];
    let m22 = matrix[2][2];

    let trace = m00 + m11 + m22;

    if trace > 0.0 {
      let s = 0.5 / (trace + 1.0).sqrt();

      let w = 0.25 / s;
      let i = (matrix[2][1] - matrix[1][2]) * s;
      let j = (matrix[0][2] - matrix[2][0]) * s;
      let k = (matrix[1][0] - matrix[0][1]) * s;

      Self::new(w, i, j, k)
    } else if m00 > m11 && m00 > m22 {
      let s = 2.0 * (1.0 + m00 - m11 - m22).sqrt();

      let w = (matrix[2][1] - matrix[1][2]) / s;
      let i = 0.25 * s;
      let j = (matrix[0][1] + matrix[1][0]) / s;
      let k = (matrix[0][2] + matrix[2][0]) / s;

      Self::new(w, i, j, k)
    } else if m11 > m22 {
      let s = 2.0 * (1.0 + m11 - m00 - m22).sqrt();

      let w = (matrix[0][2] - matrix[2][0]) / s;
      let i = (matrix[0][1] + matrix[1][0]) / s;
      let j = 0.25 * s;
      let k = (matrix[1][2] + matrix[2][1]) / s;

      Self::new(w, i, j, k)
    } else {
      let s = 2.0 * (1.0 + m22 - m00 - m11).sqrt();

      let w = (matrix[1][0] - matrix[0][1]) / s;
      let i = (matrix[0][2] + matrix[2][0]) / s;
      let j = (matrix[1][2] + matrix[2][1]) / s;
      let k = 0.25 * s;

      Self::new(w, i, j, k)
    }
  }
}

impl Default for Quaternion {
  #[inline(always)]
  fn default() -> Self {
    Quaternion::ZERO
  }
}

impl From<Vector4> for Quaternion {
  #[inline(always)]
  fn from(vec: Vector4) -> Self {
    Quaternion::from_vector4(vec)
  }
}

impl From<&Vec4> for Quaternion {
  #[inline(always)]
  fn from(vec: &Vec4) -> Self {
    Quaternion::from_vec4(vec)
  }
}

// Conversion

impl Quaternion {
  /// Converts the quaternion to a 4D vector.
  #[must_use]
  #[inline(always)]
  pub const fn as_vec4(&self) -> &Vec4 {
    self.0.as_vec4()
  }

  /// Converts the quaternion to a mutable 4D vector.
  #[must_use]
  #[inline(always)]
  pub fn as_mut_vec4(&mut self) -> &mut Vec4 {
    self.0.as_mut_vec4()
  }

  /// Returns this quaternion as a slice of [`f32`].
  #[must_use]
  #[inline(always)]
  pub const fn as_slice(&self) -> &[f32] {
    self.as_vec4().as_slice()
  }

  /// Returns this quaternion as a mutable slice of [`f32`].
  #[must_use]
  #[inline(always)]
  pub fn as_mut_slice(&mut self) -> &mut [f32] {
    self.as_mut_vec4().as_mut_slice()
  }

  /// Returns this vector as a ptr of [`f32`].
  #[must_use]
  #[inline(always)]
  pub const fn as_ptr(&self) -> *const f32 {
    self.as_slice().as_ptr()
  }

  /// Returns this vector as a mutable ptr of [`f32`].
  #[must_use]
  #[inline(always)]
  pub fn as_mut_ptr(&mut self) -> *mut f32 {
    self.as_mut_slice().as_mut_ptr()
  }

  /// Returns the rotation matrix representation of the quaternion.
  #[must_use]
  pub fn to_matrix4(&self) -> Matrix4 {
    let w = self.w();
    let i = self.i();
    let j = self.j();
    let k = self.k();

    let ww = w * w;
    let ii = i * i;
    let jj = j * j;
    let kk = k * k;

    let ij = i * j;
    let wk = w * k;
    let iw = i * w;
    let jk = j * k;
    let wj = w * j;
    let ki = k * i;

    let two = 2.0;

    let m00 = ww + ii - jj - kk;
    let m01 = two * (ij - wk);
    let m02 = two * (iw + jk);

    let m10 = two * (ij + wk);
    let m11 = ww - ii + jj - kk;
    let m12 = two * (wj - ki);

    let m20 = two * (iw - jk);
    let m21 = two * (wj + ki);
    let m22 = ww - ii - jj + kk;

    Matrix4::from_arrays([
      [m00, m01, m02, 0.0],
      [m10, m11, m12, 0.0],
      [m20, m21, m22, 0.0],
      [0.0, 0.0, 0.0, 1.0],
    ])
  }
}

/// Properties

impl Quaternion {
  /// Returns the conjugate of the quaternion.
  #[must_use]
  #[inline(always)]
  pub fn conjugate(&self) -> Self {
    Self::new(self.w(), -self.i(), -self.j(), -self.k())
  }

  /// Returns the inverse of the quaternion.
  #[must_use]
  #[inline(always)]
  pub fn inverse(&self) -> Self {
    let norm = self.norm();

    &self.conjugate() / norm
  }

  /// Returns the norm of the quaternion.
  #[must_use]
  #[inline(always)]
  pub fn norm(&self) -> f32 {
    self.dot(self).sqrt()
  }

  /// Returns the normalized quaternion.
  #[must_use]
  pub fn normalized(&self) -> Self {
    let norm = self.norm();
    self / norm
  }

  /// Returns the yaw angle of the quaternion.
  #[must_use]
  pub fn yaw<A: Angle>(&self) -> A {
    // Calculate yaw (z-axis rotation)
    let w = self.w();
    let i = self.i();
    let j = self.j();
    let k = self.k();

    let siny_cosp = 2.0 * (w * k + i * j);
    let cosy_cosp = 1.0 - 2.0 * (j * j + k * k);
    let yaw = Radian::new(siny_cosp.atan2(cosy_cosp));

    yaw.to_angle::<A>()
  }

  /// Returns the pitch angle of the quaternion.
  #[must_use]
  pub fn pitch<A: Angle>(&self) -> A {
    // Calculate roll (y-axis rotation)
    let w = self.w();
    let i = self.i();
    let j = self.j();
    let k = self.k();

    let sinp = 2.0 * (w * j - k * i);
    let pitch = Radian::new(if sinp.abs() >= 1.0 {
      // Use PI/2 if out of range (gimbal lock)
      std::f32::consts::PI / 2.0 * sinp.signum()
    } else {
      sinp.asin()
    });

    pitch.to_angle::<A>()
  }

  /// Returns the roll angle of the quaternion.
  #[must_use]
  pub fn roll<A: Angle>(&self) -> A {
    // Calculate roll (x-axis rotation)
    let w = self.w();
    let i = self.i();
    let j = self.j();
    let k = self.k();

    let sinr_cosp = 2.0 * (w * i + j * k);
    let cosr_cosp = 1.0 - 2.0 * (i * i + j * j);
    let roll = Radian::new(sinr_cosp.atan2(cosr_cosp));

    roll.to_angle::<A>()
  }

  /// Returns the euler angles of the quaternion in `(yaw, pitch, roll)` order.
  #[must_use]
  pub fn euler_angles<A: Angle>(&self) -> (A, A, A) {
    (self.yaw::<A>(), self.pitch::<A>(), self.roll::<A>())
  }

  /// Returns the angle (in radians) and axis (x, y, z) of the quaternion
  pub fn angle_axis<A: Angle>(&self) -> (A, Vector3) {
    let angle = Radian(2.0 * self.w().acos()).to_angle::<A>();
    let sin_half_angle = (1.0 - self.w() * self.w()).sqrt();

    if sin_half_angle > 0.0 {
      let axis = Vector3::new(
        self.i() / sin_half_angle,
        self.j() / sin_half_angle,
        self.k() / sin_half_angle,
      );
      (angle, axis)
    } else {
      (angle, Vector3::new(1.0, 0.0, 0.0)) // Default axis if quaternion represents no rotation
    }
  }

  /// Returns the real component of the quaternion.
  #[must_use]
  #[inline(always)]
  pub const fn w(&self) -> f32 {
    self.0.as_vec4().x()
  }

  /// Retrieves a mutable reference to the real component of the quaternion.
  #[must_use]
  #[inline(always)]
  pub fn w_mut(&mut self) -> &mut f32 {
    self.0.x_mut()
  }

  /// Returns the i imaginary component of the quaternion.
  #[must_use]
  #[inline(always)]
  pub const fn i(&self) -> f32 {
    self.0.as_vec4().y()
  }

  /// Retrieves a mutable reference to the i imaginary component of the quaternion.
  #[must_use]
  #[inline(always)]
  pub fn i_mut(&mut self) -> &mut f32 {
    self.0.y_mut()
  }

  /// Returns the j imaginary component of the quaternion.
  #[must_use]
  #[inline(always)]
  pub const fn j(&self) -> f32 {
    self.0.as_vec4().z()
  }

  /// Retrieves a mutable reference to the j imaginary component of the quaternion.
  #[must_use]
  #[inline(always)]
  pub fn j_mut(&mut self) -> &mut f32 {
    self.0.z_mut()
  }

  /// Returns the k imaginary component of the quaternion.
  #[must_use]
  #[inline(always)]
  pub const fn k(&self) -> f32 {
    self.0.as_vec4().w()
  }

  /// Retrieves a mutable reference to the k imaginary component of the quaternion.
  #[must_use]
  #[inline(always)]
  pub fn k_mut(&mut self) -> &mut f32 {
    self.0.w_mut()
  }

  /// Returns the first two components of the quaternion.
  #[must_use]
  #[inline(always)]
  pub const fn wi(&self) -> &Vec2 {
    self.0.as_vec4().xy()
  }

  /// Returns a mutable reference to the first two components of the quaternion.
  #[must_use]
  #[inline(always)]
  pub fn wi_mut(&mut self) -> &mut Vec2 {
    self.0.xy_mut()
  }

  /// Returns the middle two components of the quaternion.
  #[must_use]
  #[inline(always)]
  pub const fn ij(&self) -> &Vec2 {
    self.0.as_vec4().yz()
  }

  /// Returns a mutable reference to the middle two components of the quaternion.
  #[must_use]
  #[inline(always)]
  pub fn ij_mut(&mut self) -> &mut Vec2 {
    self.0.yz_mut()
  }

  /// Returns the last two components of the quaternion.
  #[must_use]
  #[inline(always)]
  pub const fn jk(&self) -> &Vec2 {
    self.0.as_vec4().yz()
  }

  /// Returns a mutable reference to the last two components of the quaternion.
  #[must_use]
  #[inline(always)]
  pub fn jk_mut(&mut self) -> &mut Vec2 {
    self.0.yz_mut()
  }

  /// Returns the first three components of the quaternion.
  #[must_use]
  #[inline(always)]
  pub const fn wij(&self) -> &Vec3 {
    self.0.as_vec4().xyz()
  }

  /// Returns a mutable reference to the first three components of the quaternion.
  #[must_use]
  #[inline(always)]
  pub fn wij_mut(&mut self) -> &mut Vec3 {
    self.0.xyz_mut()
  }

  /// Returns the last three components of the quaternion.
  #[must_use]
  #[inline(always)]
  pub const fn ijk(&self) -> &Vec3 {
    self.0.as_vec4().yzw()
  }

  /// Returns a mutable reference to the last three components of the quaternion.
  #[must_use]
  #[inline(always)]
  pub fn ijk_mut(&mut self) -> &mut Vec3 {
    self.0.yzw_mut()
  }
}

impl Near for Quaternion {
  #[inline(always)]
  fn near(&self, other: &Self, tolerance: &f32) -> bool {
    self.0.near(&other.0, tolerance)
  }
}

impl AlmostEq for Quaternion {
  #[inline(always)]
  fn almost_eq(&self, other: &Self) -> bool {
    self.0.almost_eq(&other.0)
  }
}

// Modifiers

impl Quaternion {
  /// Normalizes the quaternion.
  #[inline(always)]
  pub fn normalize(&mut self) {
    let norm = self.norm();

    *self /= norm;
  }

  /// Sets the real component of the quaternion.
  #[inline(always)]
  pub fn set_w(&mut self, w: f32) {
    *self.w_mut() = w;
  }

  /// Sets the i imaginary component of the quaternion.
  #[inline(always)]
  pub fn set_i(&mut self, i: f32) {
    *self.i_mut() = i;
  }

  /// Sets the j imaginary component of the quaternion.
  #[inline(always)]
  pub fn set_j(&mut self, j: f32) {
    *self.j_mut() = j;
  }

  /// Sets the k imaginary component of the quaternion.
  #[inline(always)]
  pub fn set_k(&mut self, k: f32) {
    *self.k_mut() = k;
  }
}

// Operations

impl Quaternion {
  /// Rotates the quaternion by another quaternion.
  ///
  /// # Parameters
  ///
  /// * `other` - The quaternion to rotate by.
  #[must_use]
  #[inline(always)]
  pub fn rotate(&self, other: &Self) -> Self {
    other * self * other.inverse()
  }

  /// Rotates the quaternion by an angle (in radians) and an axis vector (x, y, z).
  ///
  /// # Parameters
  ///
  /// * `angle` - The angle of rotation.
  /// * `axis` - The axis of rotation.
  #[must_use]
  #[inline(always)]
  pub fn rotate_by_angle_axis<A: Angle>(&self, angle: A, axis: &Vec3) -> Self {
    let rotation = Self::from_angle_axis(angle, axis);
    self.rotate(&rotation)
  }

  /// Rotates the quaternion by a yaw angle.
  ///
  /// # Parameters
  ///
  /// * `angle` - The yaw angle.
  #[must_use]
  pub fn rotate_by_yaw<A: Angle>(&self, angle: A) -> Self {
    let rotation = Self::from_yaw(angle);
    self.rotate(&rotation)
  }

  /// Rotates the quaternion by a pitch angle.
  ///
  /// # Parameters
  ///
  /// * `angle` - The pitch angle.
  #[must_use]
  pub fn rotate_by_pitch<A: Angle>(&self, angle: A) -> Self {
    let rotation = Self::from_pitch(angle);
    self.rotate(&rotation)
  }

  /// Rotates the quaternion by a roll angle.
  ///
  /// # Parameters
  ///
  /// * `angle` - The roll angle.
  #[must_use]
  pub fn rotate_by_roll<A: Angle>(&self, angle: A) -> Self {
    let rotation = Self::from_roll(angle);
    self.rotate(&rotation)
  }

  /// Rotates the quaternion by a set of Euler angles.
  ///
  /// # Parameters
  ///
  /// * `yaw`   - The yaw angle.
  /// * `pitch` - The pitch angle.
  /// * `roll`  - The roll angle.
  #[must_use]
  #[inline(always)]
  pub fn rotate_by_euler_angles<A: Angle>(&self, yaw: A, pitch: A, roll: A) -> Self {
    let rotation = Self::from_euler_angles(yaw, pitch, roll);
    self.rotate(&rotation)
  }

  /// Rotates the quaternion by a 4x4 matrix encoding a rotation.
  ///
  /// # Parameters
  ///
  /// * `matrix` - The rotation matrix to rotate by.
  #[must_use]
  #[inline(always)]
  pub fn rotate_by_mat4(&self, matrix: &Mat4) -> Self {
    let rotation = Self::from_mat4(matrix);
    self.rotate(&rotation)
  }

  /// Rotates a 3D vector by the angle represented by the quaternion.
  pub fn rotate_vec3(&self, vec: &Vec3) -> Vector3 {
    let qvec = Quaternion::new(0.0, vec.x(), vec.y(), vec.z());
    let result = self * qvec * self.conjugate();
    Vector3::new(result.i(), result.j(), result.k())
  }

  /// Rotates a 3D vector by the angle represented by the quaternion.
  ///
  /// # Parameters
  ///
  /// * `vec` - The vector to rotate.
  pub fn rotate_vec4(&self, vec: &Vec4) -> Vector4 {
    let qvec = Quaternion::new(0.0, vec.x(), vec.y(), vec.z());
    let result = self * qvec * self.conjugate();
    Vector4::new(result.i(), result.j(), result.k(), vec.w())
  }
}

// Arithmetic Operations

impl Add for &Quaternion {
  type Output = Quaternion;

  #[must_use]
  #[inline(always)]
  fn add(self, rhs: &Quaternion) -> Quaternion {
    Quaternion(self.as_vec4() + rhs.as_vec4())
  }
}

impl Add for Quaternion {
  type Output = Quaternion;

  #[must_use]
  #[inline(always)]
  fn add(self, rhs: Quaternion) -> Quaternion {
    (&self).add(&rhs)
  }
}

impl Add<f32> for &Quaternion {
  type Output = Quaternion;

  #[must_use]
  #[inline(always)]
  fn add(self, rhs: f32) -> Quaternion {
    let mut result = *self;
    *result.w_mut() += rhs;
    result
  }
}

impl Sub for &Quaternion {
  type Output = Quaternion;

  #[must_use]
  #[inline(always)]
  fn sub(self, rhs: &Quaternion) -> Quaternion {
    Quaternion(self.as_vec4() - rhs.as_vec4())
  }
}

impl Sub<f32> for &Quaternion {
  type Output = Quaternion;

  #[must_use]
  #[inline(always)]
  fn sub(self, rhs: f32) -> Quaternion {
    let mut result = *self;
    *result.w_mut() -= rhs;
    result
  }
}

impl Mul for &Quaternion {
  type Output = Quaternion;

  #[must_use]
  fn mul(self, rhs: &Quaternion) -> Quaternion {
    let w0 = self.w();
    let i0 = self.i();
    let j0 = self.j();
    let k0 = self.k();

    let w1 = rhs.w();
    let i1 = rhs.i();
    let j1 = rhs.j();
    let k1 = rhs.k();

    Quaternion::new(
      w0 * w1 - i0 * i1 - j0 * j1 - k0 * k1,
      w0 * i1 + i0 * w1 + j0 * k1 - k0 * j1,
      w0 * j1 - i0 * k1 + j0 * w1 + k0 * i1,
      w0 * k1 + i0 * j1 - j0 * i1 + k0 * w1,
    )
  }
}

impl Mul for Quaternion {
  type Output = Quaternion;

  #[must_use]
  #[inline(always)]
  fn mul(self, rhs: Quaternion) -> Quaternion {
    (&self).mul(&rhs)
  }
}

impl Mul<&Quaternion> for Quaternion {
  type Output = Quaternion;

  #[must_use]
  #[inline(always)]
  fn mul(self, rhs: &Quaternion) -> Quaternion {
    (&self).mul(rhs)
  }
}

impl Mul<Quaternion> for &Quaternion {
  type Output = Quaternion;

  #[must_use]
  #[inline(always)]
  fn mul(self, rhs: Quaternion) -> Quaternion {
    self.mul(&rhs)
  }
}

impl Mul<f32> for &Quaternion {
  type Output = Quaternion;

  #[must_use]
  #[inline(always)]
  fn mul(self, rhs: f32) -> Quaternion {
    Quaternion(self.as_vec4() * rhs)
  }
}

impl Mul<&Quaternion> for f32 {
  type Output = Quaternion;

  #[must_use]
  #[inline(always)]
  fn mul(self, rhs: &Quaternion) -> Quaternion {
    rhs * self
  }
}

impl Div<f32> for &Quaternion {
  type Output = Quaternion;

  #[must_use]
  #[inline(always)]
  fn div(self, rhs: f32) -> Quaternion {
    let inv = 1.0 / rhs;
    Quaternion(self.as_vec4() * inv)
  }
}

impl Lerp for Quaternion {
  type Output = Quaternion;

  #[must_use]
  #[inline(always)]
  fn lerp(&self, other: &Self, alpha: f32) -> Self::Output {
    use crate::cmp::AlmostEq;
    let lhs = self.normalized();
    let rhs = other.normalized();
    let dot = lhs.dot(&rhs);

    // if the angles are very close, we can use linear interpolation for a good
    // approximation
    if dot.almost_eq(&1.0) {
      return Quaternion::from_vector4(lhs.as_vec4().lerp(rhs.as_vec4(), alpha));
    }
    let angle = dot.acos();
    let inv_sin_angle = 1.0 / angle.sin();
    let lhs_weight = ((1.0 - alpha) * angle).sin();
    let rhs_weight = (alpha * angle).sin();

    Quaternion::from_vector4(
      ((lhs.as_vec4() * lhs_weight) + (rhs.as_vec4() * rhs_weight)) * inv_sin_angle,
    )
  }
}

impl AddAssign<&Quaternion> for Quaternion {
  #[inline(always)]
  fn add_assign(&mut self, rhs: &Quaternion) {
    self.0 += rhs.0;
  }
}

impl AddAssign<f32> for Quaternion {
  #[inline(always)]
  fn add_assign(&mut self, rhs: f32) {
    *self.w_mut() += rhs;
  }
}

impl SubAssign<f32> for Quaternion {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: f32) {
    *self.w_mut() -= rhs;
  }
}

impl MulAssign<f32> for Quaternion {
  #[inline(always)]
  fn mul_assign(&mut self, rhs: f32) {
    self.0 *= rhs;
  }
}

impl MulAssign<&Quaternion> for Quaternion {
  #[inline(always)]
  fn mul_assign(&mut self, rhs: &Quaternion) {
    *self = *self * rhs;
  }
}

impl MulAssign<Quaternion> for Quaternion {
  #[inline(always)]
  fn mul_assign(&mut self, rhs: Quaternion) {
    *self = *self * rhs;
  }
}

impl DivAssign<f32> for Quaternion {
  #[inline(always)]
  fn div_assign(&mut self, rhs: f32) {
    self.0 /= rhs;
  }
}

impl Dot for Quaternion {
  type Output = f32;
  #[inline(always)]
  fn dot(&self, other: &Self) -> Self::Output {
    self.as_vec4().dot(other.as_vec4())
  }
}

impl Dot<Vec4> for Quaternion {
  type Output = f32;
  #[inline(always)]
  fn dot(&self, other: &Vec4) -> Self::Output {
    self.0.dot(other)
  }
}

impl Dot<Quaternion> for Vec4 {
  type Output = f32;
  #[inline(always)]
  fn dot(&self, other: &Quaternion) -> Self::Output {
    self.dot(other.as_vec4())
  }
}

// Formatting

impl fmt::Display for Quaternion {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{} {} {}i {} {}j {} {}k",
      self.w(),
      if self.i() > 0.0 { '+' } else { '-' },
      self.i().abs(),
      if self.j() > 0.0 { '+' } else { '-' },
      self.j().abs(),
      if self.k() > 0.0 { '+' } else { '-' },
      self.k().abs()
    )
  }
}

impl fmt::Debug for Quaternion {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Quaternion")
      .field("w", &self.w())
      .field("i", &self.i())
      .field("j", &self.j())
      .field("k", &self.k())
      .finish()
  }
}

#[cfg(test)]
#[path = "quaternion.test.rs"]
mod test;
