use crate::math::simd::common::F32x4Array;

/// A 128-bit SIMD vector with 4 [f32] components.
pub struct Float32x4(F32x4Array);

impl Float32x4 {
  /// Create a new vector with the given components.
  ///
  /// # Parameters
  ///
  /// * `v0` - the first component of the vector
  /// * `v1` - the second component of the vector
  /// * `v2` - the third component of the vector
  /// * `v3` - the fourth component of the vector
  pub const fn new(v0: f32, v1: f32, v2: f32, v3: f32) -> Self {
    Self(F32x4Array::new([v0, v1, v2, v3]))
  }

  /// Create a new vector with the given value in all components.
  ///
  /// # Parameters
  ///
  /// * `value` - the value to set all components to
  pub const fn splat(value: f32) -> Self {
    Self(F32x4Array::new([value, value, value, value]))
  }

  /// Load a vector from a pointer to an aligned series of [`f32`] values.
  ///
  /// # Safety
  ///
  /// The pointer must be aligned to 16 bytes, and the pointer must be able to
  /// reach at least 4 [`f32`] values.
  ///
  /// # Parameters
  ///
  /// * `ptr` - the pointer to the slice of [`f32`] values
  pub const unsafe fn from_aligned_ptr(ptr: *const f32) -> Self {
    Self::from_unaligned_ptr(ptr)
  }

  /// Load a vector from a slice of [`f32`] values.
  ///
  /// # Safety
  ///
  /// The pointer must be able to reach at least 4 [`f32`] values.
  ///
  /// # Parameters
  ///
  /// * `ptr` - the pointer to the slice of [`f32`] values
  pub const unsafe fn from_unaligned_ptr(ptr: *const f32) -> Self {
    Self(F32x4Array::new([
      *ptr,
      *ptr.add(1),
      *ptr.add(2),
      *ptr.add(3),
    ]))
  }

  /// Store the vector into a slice of [`f32`] values.
  ///
  /// # Safety
  ///
  /// The the pointer must be able to reach at least 4 [`f32`] values.
  pub unsafe fn store_aligned(&self, ptr: *mut f32) {
    ptr.copy_from_nonoverlapping(self.0.as_ptr(), 4);
  }
}
