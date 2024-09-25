use std::ops::{Deref, DerefMut, Index, IndexMut};

use crate::core::StridedSlice;
use crate::math::vec::{Vec3, Vector3};
use crate::ops::Dot;

/// A column of a 3x3 matrix.
#[repr(transparent)]
pub struct Col3(StridedSlice<f32, 3>);

// Constructors

impl Col3 {
  /// Creates a new column from a slice.
  ///
  /// # Safety
  ///
  /// This function is unsafe because the caller must ensure that the slice is
  /// at least 16 elements long.
  ///
  /// # Panics
  ///
  /// This function will panic if the slice is not at least 3 elements long.
  #[inline(always)]
  #[allow(unused)] // This is used in tests
  pub(crate) unsafe fn from_slice(slice: &[f32]) -> &Self {
    std::mem::transmute(StridedSlice::<f32, 3>::from_slice(slice))
  }

  /// Creates a new column from a raw pointer and a length.
  ///
  /// # Safety
  ///
  /// This function is unsafe because the caller must ensure that the pointer is
  /// valid for `len` elements.
  ///
  /// # Parameters
  ///
  /// * `ptr` - A raw pointer to the start of the column.
  /// * `len` - The number of elements in the column.
  #[inline(always)]
  pub(crate) const unsafe fn from_raw_parts<'a>(ptr: *const f32, len: usize) -> &'a Self {
    std::mem::transmute(StridedSlice::<f32, 3>::from_raw_parts(ptr, len))
  }

  /// Creates a new column from a raw pointer and a length.
  /// This function will return a mutable column.
  ///
  /// # Safety
  ///
  /// This function is unsafe because the caller must ensure that the pointer is
  /// valid for `len` elements.
  ///
  /// # Parameters
  ///
  /// * `ptr` - A raw pointer to the start of the column.
  /// * `len` - The number of elements in the column.
  #[inline(always)]
  pub(crate) unsafe fn from_raw_parts_mut<'a>(ptr: *mut f32, len: usize) -> &'a mut Self {
    std::mem::transmute(StridedSlice::<f32, 3>::from_raw_parts_mut(ptr, len))
  }
}

// Conversions

impl Col3 {
  /// Converts the column into a 3D owning vector.
  pub const fn to_vector3(&self) -> Vector3 {
    // Safety: every validly-constructed column has exactly 3 addressable elements.
    unsafe {
      Vector3::new(
        *self.index_unchecked(0),
        *self.index_unchecked(1),
        *self.index_unchecked(2),
      )
    }
  }
}

impl Deref for Col3 {
  type Target = StridedSlice<f32, 3>;

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Col3 {
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

// Modifiers

impl Col3 {
  /// Sets the column to the content of the other vector.
  ///
  /// # Parameters
  ///
  /// * `vec` - The vector to set the column to.
  pub fn set_vec3(&mut self, vec: &Vec3) {
    self.0[0] = vec.x();
    self.0[1] = vec.y();
    self.0[2] = vec.z();
  }
}

// Indexing

impl Col3 {
  /// Returns the value at the specified index without doing any bounds checking.
  ///
  /// # Safety
  ///
  /// This function is unsafe because it does not check that the index is less
  /// than the length of the column.
  ///
  /// # Parameters
  ///
  /// * `index` - The index of the value to return.
  pub const unsafe fn index_unchecked(&self, index: usize) -> &f32 {
    self.0.index_unchecked(index)
  }

  /// Returns a mutable reference to the value at the specified index without doing
  /// any bounds checking.
  ///
  /// # Safety
  ///
  /// This function is unsafe because it does not check that the index is less than
  /// the length of the column.
  ///
  /// # Parameters
  ///
  /// * `index` - The index of the value to return.
  pub unsafe fn index_mut_unchecked(&mut self, index: usize) -> &mut f32 {
    self.0.index_mut_unchecked(index)
  }
}

impl Index<usize> for Col3 {
  type Output = f32;

  #[inline(always)]
  fn index(&self, index: usize) -> &Self::Output {
    &self.0[index]
  }
}

impl IndexMut<usize> for Col3 {
  #[inline(always)]
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    &mut self.0[index]
  }
}

impl Index<core::ops::Range<usize>> for Col3 {
  type Output = Col3;

  #[inline(always)]
  fn index(&self, index: core::ops::Range<usize>) -> &Self::Output {
    // Safety: the column is guaranteed to have 3 elements.
    unsafe { std::mem::transmute(&self.0[index]) }
  }
}

impl IndexMut<core::ops::Range<usize>> for Col3 {
  #[inline(always)]
  fn index_mut(&mut self, index: core::ops::Range<usize>) -> &mut Self::Output {
    // Safety: the column is guaranteed to have 3 elements.
    unsafe { std::mem::transmute(&mut self.0[index]) }
  }
}

impl Index<core::ops::RangeTo<usize>> for Col3 {
  type Output = Col3;

  #[inline(always)]
  fn index(&self, index: core::ops::RangeTo<usize>) -> &Self::Output {
    // Safety: the column is guaranteed to have 3 elements.
    unsafe { std::mem::transmute(&self.0[index]) }
  }
}

impl IndexMut<core::ops::RangeTo<usize>> for Col3 {
  #[inline(always)]
  fn index_mut(&mut self, index: core::ops::RangeTo<usize>) -> &mut Self::Output {
    // Safety: the column is guaranteed to have 3 elements.
    unsafe { std::mem::transmute(&mut self.0[index]) }
  }
}

impl Index<core::ops::RangeFrom<usize>> for Col3 {
  type Output = Col3;

  #[inline(always)]
  fn index(&self, index: core::ops::RangeFrom<usize>) -> &Self::Output {
    // Safety: the column is guaranteed to have 3 elements.
    unsafe { std::mem::transmute(&self.0[index]) }
  }
}

impl IndexMut<core::ops::RangeFrom<usize>> for Col3 {
  #[inline(always)]
  fn index_mut(&mut self, index: core::ops::RangeFrom<usize>) -> &mut Self::Output {
    // Safety: the column is guaranteed to have 3 elements.
    unsafe { std::mem::transmute(&mut self.0[index]) }
  }
}

impl Index<core::ops::RangeInclusive<usize>> for Col3 {
  type Output = Col3;

  #[inline(always)]
  fn index(&self, index: core::ops::RangeInclusive<usize>) -> &Self::Output {
    // Safety: the column is guaranteed to have 3 elements.
    unsafe { std::mem::transmute(&self.0[index]) }
  }
}

impl IndexMut<core::ops::RangeInclusive<usize>> for Col3 {
  #[inline(always)]
  fn index_mut(&mut self, index: core::ops::RangeInclusive<usize>) -> &mut Self::Output {
    // Safety: the column is guaranteed to have 3 elements.
    unsafe { std::mem::transmute(&mut self.0[index]) }
  }
}

impl Index<core::ops::RangeFull> for Col3 {
  type Output = Col3;

  #[inline(always)]
  fn index(&self, _: core::ops::RangeFull) -> &Self::Output {
    self
  }
}

impl IndexMut<core::ops::RangeFull> for Col3 {
  #[inline(always)]
  fn index_mut(&mut self, _: core::ops::RangeFull) -> &mut Self::Output {
    self
  }
}

impl Index<core::ops::RangeToInclusive<usize>> for Col3 {
  type Output = Col3;

  #[inline(always)]
  fn index(&self, index: core::ops::RangeToInclusive<usize>) -> &Self::Output {
    // Safety: the column is guaranteed to have 3 elements.
    unsafe { std::mem::transmute(&self.0[index]) }
  }
}

impl IndexMut<core::ops::RangeToInclusive<usize>> for Col3 {
  #[inline(always)]
  fn index_mut(&mut self, index: core::ops::RangeToInclusive<usize>) -> &mut Self::Output {
    // Safety: the column is guaranteed to have 3 elements.
    unsafe { std::mem::transmute(&mut self.0[index]) }
  }
}

// Operation

impl Dot for Col3 {
  type Output = f32;

  #[inline(always)]
  fn dot(&self, other: &Self) -> Self::Output {
    self.iter().zip(other.iter()).map(|(a, b)| a * b).sum()
  }
}

impl Dot<Vec3> for Col3 {
  type Output = f32;

  #[inline(always)]
  fn dot(&self, other: &Vec3) -> Self::Output {
    self.iter().zip(other.iter()).map(|(a, b)| a * b).sum()
  }
}

impl Dot<Col3> for Vec3 {
  type Output = f32;

  #[inline(always)]
  fn dot(&self, other: &Col3) -> Self::Output {
    self.iter().zip(other.iter()).map(|(a, b)| a * b).sum()
  }
}

#[cfg(test)]
#[path = "col3.test.rs"]
mod test;
