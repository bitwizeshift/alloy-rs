//! This module provides utilities for forming projection matrices.

use std::ops::Deref;

use crate::math::mat::Mat4;
use crate::math::{mat::Matrix4, Angle, Radian};

use crate::model::clip::{ClipSpace, Depth};

/// Representation of a projection matrix.
///
/// Projection matrices may be identity projections, which are normalized device
/// coordinates, perspective projections, or orthographic projections.
pub struct Projection {
  matrix: Matrix4,
}

// Constructors

impl Projection {
  /// Creates a new projection matrix.
  pub const fn identity() -> Self {
    Self {
      matrix: Matrix4::identity(),
    }
  }

  /// Computes an orthographic projection from the specified clip-space information.
  ///
  /// # Arguments
  ///
  /// * `space` - The clip-space information.
  pub fn orthographic(space: ClipSpace) -> Self {
    let left = space.horizontal.left();
    let right = space.horizontal.right();
    let bottom = space.vertical.bottom();
    let top = space.vertical.top();
    let near = space.depth.near();
    let far = space.depth.far();

    let dx = right - left;
    let dy = top - bottom;
    let dz = far - near;

    let tx = -((right + left) / dx);
    let ty = -((top + bottom) / dy);
    let tz = -((far + near) / dz);

    let rx = 2.0 / dx;
    let ry = 2.0 / dy;
    let rz = -2.0 / dz;

    Self {
      matrix: Matrix4::from_arrays([
        [rx, 0.0, 0.0, tx],
        [0.0, ry, 0.0, ty],
        [0.0, 0.0, rz, tz],
        [0.0, 0.0, 0.0, 1.0],
      ]),
    }
  }

  /// Computes a perspective projection from the specified angle, aspect ratio, and depth.
  ///
  /// # Arguments
  ///
  /// * `angle` - The field of view angle.
  /// * `aspect_ratio` - The aspect ratio.
  /// * `depth` - The depth range.
  pub fn perspective<A: Angle>(fov: A, aspect_ratio: f32, depth: Depth) -> Self {
    let fov_radians = fov.to_angle::<Radian>();
    let half_tan_fov = (fov_radians * 0.5).tan();

    let dz = depth.far() - depth.near();

    let rx = 1.0 / (aspect_ratio * half_tan_fov);
    let ry = 1.0 / half_tan_fov;
    let rz = -(depth.far() + depth.near()) / dz;
    let factor = -(2.0 * depth.far() * depth.near()) / dz;

    Self {
      matrix: Matrix4::from_arrays([
        [rx, 0.0, 0.0, 0.0],
        [0.0, ry, 0.0, 0.0],
        [0.0, 0.0, rz, factor],
        [0.0, 0.0, -1.0, 0.0],
      ]),
    }
  }
}

impl Default for Projection {
  #[inline(always)]
  fn default() -> Self {
    Self::identity()
  }
}

// Conversions

impl Deref for Projection {
  type Target = Mat4;

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    &self.matrix
  }
}

impl AsRef<Mat4> for Projection {
  #[inline(always)]
  fn as_ref(&self) -> &Mat4 {
    &self.matrix
  }
}

// Properties

impl Projection {
  /// Returns the projection matrix.
  #[inline(always)]
  pub const fn as_matrix(&self) -> &Matrix4 {
    &self.matrix
  }

  /// Returns the inverse of the projection matrix.
  #[inline(always)]
  pub const fn as_mat4(&self) -> &Mat4 {
    self.matrix.as_mat4()
  }

  /// Returns a pointer to the projection matrix.
  pub const fn as_ptr(&self) -> *const f32 {
    self.matrix.as_mat4().as_ptr()
  }
}
