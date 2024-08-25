use crate::math::simd::common::F32x4Array;

use std::arch::aarch64::float32x4_t;

/// A 128-bit SIMD vector with 4 [f32] components.
#[repr(align(16))]
#[derive(Clone, Copy, Debug)]
pub struct Float32x4(float32x4_t);

impl Float32x4 {
  /// Create a new vector with the given components.
  ///
  /// # Arguments
  ///
  /// * `v0` - the first component of the vector
  /// * `v1` - the second component of the vector
  /// * `v2` - the third component of the vector
  /// * `v3` - the fourth component of the vector
  #[inline]
  pub fn new(v0: f32, v1: f32, v2: f32, v3: f32) -> Self {
    let align = F32x4Array::new([v0, v1, v2, v3]);
    Self(unsafe { std::arch::aarch64::vld1q_f32(align.as_ptr()) })
  }

  /// Create a new vector with the given value in all components.
  ///
  /// # Arguments
  ///
  /// * `value` - the value to set all components to
  #[inline(always)]
  pub fn splat(value: f32) -> Self {
    Self(unsafe { std::arch::aarch64::vdupq_n_f32(value) })
  }

  /// Load a vector from a pointer to an aligned series of [f32] values.
  ///
  /// # Safety
  ///
  /// The pointer must be aligned to 16 bytes, and the pointer must be able to
  /// reach at least 4 [f32] values.
  ///
  /// # Arguments
  ///
  /// * `ptr` - the pointer to the slice of [f32] values
  #[inline(always)]
  pub unsafe fn from_aligned_ptr(ptr: *const f32) -> Self {
    Self(std::arch::aarch64::vld1q_f32(ptr))
  }

  /// Load a vector from a slice of [f32] values.
  ///
  /// # Safety
  ///
  /// The pointer must be able to reach at least 4 [f32] values.
  ///
  /// # Arguments
  ///
  /// * `ptr` - the pointer to the slice of [f32] values
  #[inline(always)]
  pub unsafe fn from_unaligned_ptr(ptr: *const f32) -> Self {
    Self(std::arch::aarch64::vld1q_f32(ptr))
  }

  /// Store the vector into an aligned [f32] pointer.
  ///
  /// # Arguments
  ///
  /// * `ptr` - the pointer to the [f32] values to store the vector into
  ///
  /// # Safety
  ///
  /// The pointer must be aligned to 16 bytes, and the pointer must be able to
  /// reach at least 4 [f32] values.
  #[inline(always)]
  pub unsafe fn store_aligned(&self, ptr: *mut f32) {
    std::arch::aarch64::vst1q_f32(ptr, self.0)
  }

  /// Calculates the minimum of two vectors component-wise.
  ///
  /// # Arguments
  ///
  /// * `other` - the vector to compare
  #[inline(always)]
  pub fn min(self, other: Self) -> Self {
    Self(unsafe { std::arch::aarch64::vminq_f32(self.0, other.0) })
  }

  /// Calculates the maximum of two vectors component-wise.
  ///
  /// # Arguments
  ///
  /// * `other` - the vector to compare
  #[inline(always)]
  pub fn max(self, other: Self) -> Self {
    Self(unsafe { std::arch::aarch64::vmaxq_f32(self.0, other.0) })
  }

  /// Calculates the square root of the vector component-wise.
  ///
  /// # Arguments
  ///
  /// * `other` - the vector to compare
  #[inline(always)]
  pub fn sqrt(self) -> Self {
    Self(unsafe { std::arch::aarch64::vsqrtq_f32(self.0) })
  }
}

impl std::ops::Add for Float32x4 {
  type Output = Self;

  #[inline(always)]
  fn add(self, other: Self) -> Self {
    Self(unsafe { std::arch::aarch64::vaddq_f32(self.0, other.0) })
  }
}

impl std::ops::Sub for Float32x4 {
  type Output = Self;

  #[inline(always)]
  fn sub(self, other: Self) -> Self {
    Self(unsafe { std::arch::aarch64::vsubq_f32(self.0, other.0) })
  }
}

impl std::ops::Mul for Float32x4 {
  type Output = Self;

  #[inline(always)]
  fn mul(self, other: Self) -> Self {
    Self(unsafe { std::arch::aarch64::vmulq_f32(self.0, other.0) })
  }
}

impl std::ops::Div for Float32x4 {
  type Output = Self;

  #[inline(always)]
  fn div(self, other: Self) -> Self {
    Self(unsafe { std::arch::aarch64::vdivq_f32(self.0, other.0) })
  }
}

impl std::ops::Neg for Float32x4 {
  type Output = Self;

  #[inline(always)]
  fn neg(self) -> Self {
    Self(unsafe { std::arch::aarch64::vnegq_f32(self.0) })
  }
}

impl std::ops::AddAssign for Float32x4 {
  #[inline(always)]
  fn add_assign(&mut self, other: Self) {
    self.0 = unsafe { std::arch::aarch64::vaddq_f32(self.0, other.0) }
  }
}

impl std::ops::SubAssign for Float32x4 {
  #[inline(always)]
  fn sub_assign(&mut self, other: Self) {
    self.0 = unsafe { std::arch::aarch64::vsubq_f32(self.0, other.0) }
  }
}

impl std::ops::MulAssign for Float32x4 {
  #[inline(always)]
  fn mul_assign(&mut self, other: Self) {
    self.0 = unsafe { std::arch::aarch64::vmulq_f32(self.0, other.0) }
  }
}

impl std::ops::DivAssign for Float32x4 {
  #[inline(always)]
  fn div_assign(&mut self, other: Self) {
    self.0 = unsafe { std::arch::aarch64::vdivq_f32(self.0, other.0) }
  }
}
