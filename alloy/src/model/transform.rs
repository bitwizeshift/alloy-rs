//! The transform module defines model transformation types

use crate::{
  core::hint,
  math::{
    mat::{Mat4, Matrix4},
    vec::{Vec3, Vec4, Vector3},
    Angle, Quaternion,
  },
};

/// A transform represents the transformation matrix of a model
///
/// This type only implements [`Clone`] and not [`Copy`] because it is a large
/// type that is more efficient to pass by reference. If you need to create a
/// new transform that is a copy of an existing transform, you can use the
/// [`Clone`] trait to do so.
#[derive(Debug, Clone)]
pub struct Transform {
  rotation: Quaternion,
  translation: Vector3,
  scale: Vector3,
  dirty: bool,
  transform: Matrix4,
}

// Transform currently takes ~2 cache entries (128 bytes), where the raw uncomputed
// content is 64 bytes, and the computed content is 64 bytes.
const _: () = assert!(std::mem::size_of::<Transform>() == 128);

// Constructors

impl Transform {
  const DEFAULT_TRANSLATION: Vector3 = Vector3::ZERO;
  const DEFAULT_SCALE: Vector3 = Vector3::new(1.0, 1.0, 1.0);
  const DEFAULT_ROTATION: Quaternion = Quaternion::UNIT;

  /// Represents the Identity transformation.
  pub const IDENTITY: Self = Self::new(
    Self::DEFAULT_TRANSLATION,
    Self::DEFAULT_ROTATION,
    Self::DEFAULT_SCALE,
  );

  /// Creates a new transform with the given position, rotation, and scale
  ///
  /// # Parameters
  ///
  /// * `translation` - The translation of the transform
  /// * `rotation` - The rotation of the transform
  /// * `scale` - The scale of the transform
  #[must_use]
  pub const fn new(translation: Vector3, rotation: Quaternion, scale: Vector3) -> Self {
    Self {
      translation,
      rotation,
      scale,
      transform: Matrix4::IDENTITY,
      dirty: true,
    }
  }

  /// Creates a new transform representing just the translation
  ///
  /// # Parameters
  ///
  /// * `translation` - The translation of the transform
  #[inline(always)]
  #[must_use]
  pub const fn from_translation(translation: Vector3) -> Self {
    Self::new(translation, Quaternion::UNIT, Self::DEFAULT_SCALE)
  }

  /// Creates a new transform representing a translation only in the x direction.
  ///
  /// # Parameters
  ///
  /// * `x` - The translation along the x-axis
  #[inline(always)]
  #[must_use]
  pub const fn from_x_translation(x: f32) -> Self {
    Self::from_translation(Vector3::new(x, 0.0, 0.0))
  }

  /// Creates a new transform representing a translation only in the y direction.
  ///
  /// # Parameters
  ///
  /// * `y` - The translation along the y-axis
  #[inline(always)]
  #[must_use]
  pub const fn from_y_translation(y: f32) -> Self {
    Self::from_translation(Vector3::new(0.0, y, 0.0))
  }

  /// Creates a new transform representing a translation only in the z direction.
  ///
  /// # Parameters
  ///
  /// * `z` - The translation along the z-axis
  #[inline(always)]
  #[must_use]
  pub const fn from_z_translation(z: f32) -> Self {
    Self::from_translation(Vector3::new(0.0, 0.0, z))
  }

  /// Creates a new transform representing just the translation
  ///
  /// # Parameters
  ///
  /// * `translation` - The translation of the transform
  #[inline(always)]
  #[must_use]
  pub const fn from_translation_vec3(translation: &Vec3) -> Self {
    let vec3 = Vector3::new(translation.x(), translation.y(), translation.z());
    Self::from_translation(vec3)
  }

  /// Creates a new transform representing just the rotation
  ///
  /// # Parameters
  ///
  /// * `rotation` - The rotation of the transform
  #[inline(always)]
  #[must_use]
  pub const fn from_rotation(rotation: Quaternion) -> Self {
    Self::new(Self::DEFAULT_TRANSLATION, rotation, Self::DEFAULT_SCALE)
  }

  /// Creates a new transform representing just the rotation identified in a
  /// [`Vec4`] encoded as a [`Quaternion`].
  ///
  /// # Parameters
  ///
  /// * `rotation` - The rotation of the transform
  pub const fn from_rotation_vec4(rotation: &Vec4) -> Self {
    let vec4 = Quaternion::new(rotation.x(), rotation.y(), rotation.z(), rotation.w());
    Self::from_rotation(vec4)
  }

  /// Creates a new transform representing just a yaw rotation
  ///
  /// # Parameters
  ///
  /// * `angle` - The yaw angle of the transform
  #[inline(always)]
  #[must_use]
  pub fn from_yaw_rotation<A: Angle>(angle: A) -> Self {
    Self::from_rotation(Quaternion::from_yaw(angle))
  }

  /// Creates a new transform representing just a pitch rotation
  ///
  /// # Parameters
  ///
  /// * `angle` - The pitch angle of the transform
  #[inline(always)]
  #[must_use]
  pub fn from_pitch_rotation<A: Angle>(angle: A) -> Self {
    Self::from_rotation(Quaternion::from_pitch(angle))
  }

  /// Creates a new transform representing just a roll rotation
  ///
  /// # Parameters
  ///
  /// * `angle` - The roll angle of the transform
  #[inline(always)]
  #[must_use]
  pub fn from_roll_rotation<A: Angle>(angle: A) -> Self {
    Self::from_rotation(Quaternion::from_roll(angle))
  }

  /// Creates a new transform representing just the scale
  ///
  /// # Parameters
  ///
  /// * `scale` - The scale of the transform
  #[inline(always)]
  pub const fn from_scale(scale: Vector3) -> Self {
    Self::new(Self::DEFAULT_TRANSLATION, Self::DEFAULT_ROTATION, scale)
  }

  /// Creates a new transform representing just the scale
  ///
  /// # Parameters
  ///
  /// * `scale` - The scale of the transform
  #[inline(always)]
  #[must_use]
  pub const fn from_scale_vec3(scale: &Vec3) -> Self {
    // Note: the ToOwned is not const in stable, so this is done to preserve
    //       constness.
    let vec3 = Vector3::new(scale.x(), scale.y(), scale.z());
    Self::from_scale(vec3)
  }

  /// Creates a new transform representing a uniform scale
  ///
  /// # Parameters
  ///
  /// * `scale` - The scale factor
  #[inline(always)]
  #[must_use]
  pub const fn from_uniform_scale(scale: f32) -> Self {
    Self::from_scale(Vector3::new(scale, scale, scale))
  }

  /// Creates a new transform representing a non-uniform scale
  ///
  /// # Parameters
  ///
  /// * `x` - The scale factor along the x-axis
  /// * `y` - The scale factor along the y-axis
  /// * `z` - The scale factor along the z-axis
  #[inline(always)]
  #[must_use]
  pub const fn from_nonuniform_scale(x: f32, y: f32, z: f32) -> Self {
    Self::from_scale(Vector3::new(x, y, z))
  }

  /// Creates a new transform representing an x-scaled transform
  ///
  /// # Parameters
  ///
  /// * `x` - The scale factor along the x-axis
  #[inline(always)]
  #[must_use]
  pub const fn from_x_scale(x: f32) -> Self {
    Self::from_nonuniform_scale(x, 1.0, 1.0)
  }

  /// Creates a new transform representing a y-scaled transform
  ///
  /// # Parameters
  ///
  /// * `y` - The scale factor along the y-axis
  #[inline(always)]
  #[must_use]
  pub const fn from_y_scale(y: f32) -> Self {
    Self::from_nonuniform_scale(1.0, y, 1.0)
  }

  /// Creates a new transform representing a z-scaled transform
  ///
  /// # Parameters
  ///
  /// * `z` - The scale factor along the z-axis
  #[inline(always)]
  #[must_use]
  pub const fn from_z_scale(z: f32) -> Self {
    Self::from_nonuniform_scale(1.0, 1.0, z)
  }
}

impl Default for Transform {
  fn default() -> Self {
    Self::IDENTITY
  }
}

// Properties

impl Transform {
  /// Returns the position of the transform
  pub fn translation(&self) -> &Vec3 {
    &self.translation
  }

  /// Returns the rotation of the transform
  pub fn rotation(&self) -> &Quaternion {
    &self.rotation
  }

  /// Returns the scale direction of the transform
  pub fn scale_direction(&self) -> &Vec3 {
    &self.scale
  }
}

// Translation

impl Transform {
  /// Sets the translation of the transform
  ///
  /// # Parameters
  ///
  /// * `translation` - The translation of the transform
  pub fn set_translation(&mut self, translation: Vector3) {
    self.translation = translation;
    self.dirty = true;
  }

  /// Sets the translation of the transform
  ///
  /// # Parameters
  ///
  /// * `translation` - The translation of the transform
  pub fn set_translation_vec3(&mut self, translation: &Vec3) {
    let vec3 = Vector3::new(translation.x(), translation.y(), translation.z());
    self.set_translation(vec3);
  }

  /// Sets the translation of the transform in the x direction
  ///
  /// # Parameters
  ///
  /// * `x` - The translation along the x-axis
  pub fn translate_x(&mut self, x: f32) {
    self.translation.x += x;
    self.dirty = true;
  }

  /// Sets the translation of the transform in the y direction
  ///
  /// # Parameters
  ///
  /// * `y` - The translation along the y-axis
  pub fn translate_y(&mut self, y: f32) {
    self.translation.y += y;
    self.dirty = true;
  }

  /// Sets the translation of the transform in the z direction
  ///
  /// # Parameters
  ///
  /// * `z` - The translation along the z-axis
  pub fn translate_z(&mut self, z: f32) {
    self.translation.z += z;
    self.dirty = true;
  }

  /// Translates the transform by the given vector
  ///
  /// # Parameters
  ///
  /// * `translation` - The translation to apply
  pub fn translate(&mut self, translation: &Vec3) {
    self.translation += translation;
    self.dirty = true;
  }
}

impl Transform {
  /// Sets the rotation of the transform
  pub fn set_rotation(&mut self, rotation: Quaternion) {
    self.rotation = rotation;
    self.dirty = true;
  }

  /// Rotates the transform in the yaw direction by the given angle.
  ///
  /// # Parameters
  ///
  /// * `angle` - The angle to rotate the transform by.
  pub fn rotate_yaw<A: Angle>(&mut self, angle: A) {
    self.rotate(&Quaternion::from_yaw(angle))
  }

  /// Rotates the transform in the pitch direction by the given angle.
  ///
  /// # Parameters
  ///
  /// * `angle` - The angle to rotate the transform by.
  #[inline(always)]
  pub fn rotate_pitch<A: Angle>(&mut self, angle: A) {
    self.rotate(&Quaternion::from_pitch(angle))
  }

  /// Rotates the transform in the roll direction by the given angle.
  ///
  /// # Parameters
  ///
  /// * `angle` - The angle to rotate the transform by.
  #[inline(always)]
  pub fn rotate_roll<A: Angle>(&mut self, angle: A) {
    self.rotate(&Quaternion::from_roll(angle))
  }

  /// Rotates the transform by the given euler angles.
  ///
  /// # Parameters
  ///
  /// * `yaw` - The yaw angle to rotate the transform by.
  /// * `pitch` - The pitch angle to rotate the transform by.
  /// * `roll` - The roll angle to rotate the transform by.
  pub fn rotate_euler_angles<A: Angle>(&mut self, yaw: A, pitch: A, roll: A) {
    self.rotate(&Quaternion::from_euler_angles(yaw, pitch, roll))
  }

  /// Rotates the transform by the given quaternion.
  ///
  /// # Parameters
  ///
  /// * `rotation` - The rotation to apply.
  pub fn rotate(&mut self, rotation: &Quaternion) {
    self.rotation *= rotation;
    self.dirty = true;
  }
}

impl Transform {
  /// Sets the scale of the transform
  ///
  /// # Parameters
  ///
  /// * `scale` - The scale to apply to the transform
  pub fn set_scale(&mut self, scale: Vector3) {
    self.scale = scale;
    self.dirty = true;
  }

  /// Scales the transformation by the vector `scale`
  ///
  /// # Parameters
  ///
  /// * `scale` - The scale to apply to the transform
  pub fn scale(&mut self, scale: &Vec3) {
    self.scale.x *= scale.x();
    self.scale.y *= scale.y();
    self.scale.z *= scale.z();
    self.dirty = true;
  }

  /// Scales the width of the transform
  ///
  /// # Parameters
  ///
  /// * `width` - The width to scale the transform by
  pub fn scale_width(&mut self, width: f32) {
    self.scale.x *= width;
    self.dirty = true;
  }

  /// Scales the height of the transform
  ///
  /// # Parameters
  ///
  /// * `height` - The height to scale the transform by
  pub fn scale_height(&mut self, height: f32) {
    self.scale.y *= height;
    self.dirty = true;
  }

  /// Scales the depth of the transform
  ///
  /// # Parameters
  ///
  /// * `depth` - The depth to scale the transform by
  pub fn scale_depth(&mut self, depth: f32) {
    self.scale.z *= depth;
    self.dirty = true;
  }

  /// Scales the transform uniformly by the given scale factor
  ///
  /// # Parameters
  ///
  /// * `scale` - The scale factor to apply to the transform
  pub fn scale_uniform(&mut self, scale: f32) {
    self.scale *= scale;
    self.dirty = true;
  }

  /// Scales the transform non-uniformly by the given scale factors
  ///
  /// # Parameters
  ///
  /// * `x` - The scale factor along the x-axis
  /// * `y` - The scale factor along the y-axis
  /// * `z` - The scale factor along the z-axis
  pub fn scale_nonuniform(&mut self, x: f32, y: f32, z: f32) {
    self.scale.x *= x;
    self.scale.y *= y;
    self.scale.z *= z;
    self.dirty = true;
  }
}

// Operations

impl Transform {
  /// Computes and reutrns the transformation matrix of the transform
  ///
  /// This function will only recompute the transformation matrix if the
  /// transform has been modified since the last time the transformation matrix
  /// was computed.
  pub fn transform(&mut self) -> &Mat4 {
    if hint::unlikely(self.dirty) {
      let translation = self.translation_matrix();
      let rotation = self.rotation_matrix();
      let scale = self.scale_matrix();

      self.transform = &translation * &rotation * &scale;
      self.dirty = false;
    }
    &self.transform
  }
}

// Implementation

impl Transform {
  const fn scale_matrix(&self) -> Matrix4 {
    Matrix4::from_arrays([
      [self.scale.x, 0.0, 0.0, 0.0],
      [0.0, self.scale.y, 0.0, 0.0],
      [0.0, 0.0, self.scale.z, 0.0],
      [0.0, 0.0, 0.0, 1.0],
    ])
  }

  const fn translation_matrix(&self) -> Matrix4 {
    Matrix4::from_arrays([
      [1.0, 0.0, 0.0, self.translation.x],
      [0.0, 1.0, 0.0, self.translation.y],
      [0.0, 0.0, 1.0, self.translation.z],
      [0.0, 0.0, 0.0, 1.0],
    ])
  }

  fn rotation_matrix(&self) -> Matrix4 {
    self.rotation.to_matrix4()
  }
}

#[cfg(test)]
#[path = "transform.test.rs"]
mod test;
