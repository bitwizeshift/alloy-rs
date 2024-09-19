use std::ops::{Deref, DerefMut, Index, IndexMut};

use crate::core::StridedSlice;
use crate::math::vec::{Vec4, Vector4};
use crate::ops::Dot;

/// A column of a 4x4 matrix.
#[repr(transparent)]
pub struct Col4(StridedSlice<f32, 4>);

// Constructors

impl Col4 {
  /// Creates a new column from a slice.
  ///
  /// # Safety
  ///
  /// This function is unsafe because the caller must ensure that the slice is
  /// at least 16 elements long.
  ///
  /// # Panics
  ///
  /// This function will panic if the slice is not at least 4 elements long.
  #[inline(always)]
  pub(crate) unsafe fn from_slice(slice: &[f32]) -> &Self {
    std::mem::transmute(StridedSlice::<f32, 4>::from_slice(slice))
  }

  /// Creates a new column from a mutable slice.
  ///
  /// # Safety
  ///
  /// This function is unsafe because the caller must ensure that the slice is
  /// at least 16 elements long.
  ///
  /// # Panics
  ///
  /// This function will panic if the slice is not at least 4 elements long.
  #[inline(always)]
  pub(crate) unsafe fn from_mut_slice(slice: &mut [f32]) -> &Self {
    std::mem::transmute(StridedSlice::<f32, 4>::from_mut_slice(slice))
  }

  /// Creates a new column from a raw pointer and a length.
  ///
  /// # Safety
  ///
  /// This function is unsafe because the caller must ensure that the pointer is
  /// valid for `len` elements.
  ///
  /// # Arguments
  ///
  /// * `ptr` - A raw pointer to the start of the column.
  /// * `len` - The number of elements in the column.
  #[inline(always)]
  pub(crate) const unsafe fn from_raw_parts<'a>(ptr: *const f32, len: usize) -> &'a Self {
    std::mem::transmute(StridedSlice::<f32, 4>::from_raw_parts(ptr, len))
  }

  /// Creates a new column from a raw pointer and a length.
  /// This function will return a mutable column.
  ///
  /// # Safety
  ///
  /// This function is unsafe because the caller must ensure that the pointer is
  /// valid for `len` elements.
  ///
  /// # Arguments
  ///
  /// * `ptr` - A raw pointer to the start of the column.
  /// * `len` - The number of elements in the column.
  #[inline(always)]
  pub(crate) unsafe fn from_raw_parts_mut<'a>(ptr: *mut f32, len: usize) -> &'a mut Self {
    std::mem::transmute(StridedSlice::<f32, 4>::from_raw_parts_mut(ptr, len))
  }
}

// Conversions

impl Col4 {
  /// Converts the column into a 4D owning vector.
  pub const fn to_vector4(&self) -> Vector4 {
    // Safety: every validly-constructed column has exactly 4 addressable elements.
    unsafe {
      Vector4::new(
        *self.index_unchecked(0),
        *self.index_unchecked(1),
        *self.index_unchecked(2),
        *self.index_unchecked(3),
      )
    }
  }
}

impl Deref for Col4 {
  type Target = StridedSlice<f32, 4>;

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Col4 {
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

// Indexing

impl Col4 {
  /// Returns the value at the specified index without doing any bounds checking.
  ///
  /// # Safety
  ///
  /// This function is unsafe because it does not check that the index is less
  /// than the length of the column.
  ///
  /// # Arguments
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
  /// # Arguments
  ///
  /// * `index` - The index of the value to return.
  pub unsafe fn index_mut_unchecked(&mut self, index: usize) -> &mut f32 {
    self.0.index_mut_unchecked(index)
  }
}

impl Index<usize> for Col4 {
  type Output = f32;

  #[inline(always)]
  fn index(&self, index: usize) -> &Self::Output {
    &self.0[index]
  }
}

impl IndexMut<usize> for Col4 {
  #[inline(always)]
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    &mut self.0[index]
  }
}

impl Index<core::ops::Range<usize>> for Col4 {
  type Output = Col4;

  #[inline(always)]
  fn index(&self, index: core::ops::Range<usize>) -> &Self::Output {
    // Safety: the column is guaranteed to have 4 elements.
    unsafe { std::mem::transmute(&self.0[index]) }
  }
}

impl IndexMut<core::ops::Range<usize>> for Col4 {
  #[inline(always)]
  fn index_mut(&mut self, index: core::ops::Range<usize>) -> &mut Self::Output {
    // Safety: the column is guaranteed to have 4 elements.
    unsafe { std::mem::transmute(&mut self.0[index]) }
  }
}

impl Index<core::ops::RangeTo<usize>> for Col4 {
  type Output = Col4;

  #[inline(always)]
  fn index(&self, index: core::ops::RangeTo<usize>) -> &Self::Output {
    // Safety: the column is guaranteed to have 4 elements.
    unsafe { std::mem::transmute(&self.0[index]) }
  }
}

impl IndexMut<core::ops::RangeTo<usize>> for Col4 {
  #[inline(always)]
  fn index_mut(&mut self, index: core::ops::RangeTo<usize>) -> &mut Self::Output {
    // Safety: the column is guaranteed to have 4 elements.
    unsafe { std::mem::transmute(&mut self.0[index]) }
  }
}

impl Index<core::ops::RangeFrom<usize>> for Col4 {
  type Output = Col4;

  #[inline(always)]
  fn index(&self, index: core::ops::RangeFrom<usize>) -> &Self::Output {
    // Safety: the column is guaranteed to have 4 elements.
    unsafe { std::mem::transmute(&self.0[index]) }
  }
}

impl IndexMut<core::ops::RangeFrom<usize>> for Col4 {
  #[inline(always)]
  fn index_mut(&mut self, index: core::ops::RangeFrom<usize>) -> &mut Self::Output {
    // Safety: the column is guaranteed to have 4 elements.
    unsafe { std::mem::transmute(&mut self.0[index]) }
  }
}

impl Index<core::ops::RangeInclusive<usize>> for Col4 {
  type Output = Col4;

  #[inline(always)]
  fn index(&self, index: core::ops::RangeInclusive<usize>) -> &Self::Output {
    // Safety: the column is guaranteed to have 4 elements.
    unsafe { std::mem::transmute(&self.0[index]) }
  }
}

impl IndexMut<core::ops::RangeInclusive<usize>> for Col4 {
  #[inline(always)]
  fn index_mut(&mut self, index: core::ops::RangeInclusive<usize>) -> &mut Self::Output {
    // Safety: the column is guaranteed to have 4 elements.
    unsafe { std::mem::transmute(&mut self.0[index]) }
  }
}

impl Index<core::ops::RangeFull> for Col4 {
  type Output = Col4;

  #[inline(always)]
  fn index(&self, _: core::ops::RangeFull) -> &Self::Output {
    self
  }
}

impl IndexMut<core::ops::RangeFull> for Col4 {
  #[inline(always)]
  fn index_mut(&mut self, _: core::ops::RangeFull) -> &mut Self::Output {
    self
  }
}

impl Index<core::ops::RangeToInclusive<usize>> for Col4 {
  type Output = Col4;

  #[inline(always)]
  fn index(&self, index: core::ops::RangeToInclusive<usize>) -> &Self::Output {
    // Safety: the column is guaranteed to have 4 elements.
    unsafe { std::mem::transmute(&self.0[index]) }
  }
}

impl IndexMut<core::ops::RangeToInclusive<usize>> for Col4 {
  #[inline(always)]
  fn index_mut(&mut self, index: core::ops::RangeToInclusive<usize>) -> &mut Self::Output {
    // Safety: the column is guaranteed to have 4 elements.
    unsafe { std::mem::transmute(&mut self.0[index]) }
  }
}

// Operation

impl Dot for Col4 {
  type Output = f32;

  #[inline(always)]
  fn dot(&self, other: &Self) -> Self::Output {
    self.iter().zip(other.iter()).map(|(a, b)| a * b).sum()
  }
}

impl Dot<Vec4> for Col4 {
  type Output = f32;

  #[inline(always)]
  fn dot(&self, other: &Vec4) -> Self::Output {
    self.iter().zip(other.iter()).map(|(a, b)| a * b).sum()
  }
}

impl Dot<Col4> for Vec4 {
  type Output = f32;

  #[inline(always)]
  fn dot(&self, other: &Col4) -> Self::Output {
    self.iter().zip(other.iter()).map(|(a, b)| a * b).sum()
  }
}

#[cfg(test)]
#[path = "col4.test.rs"]
mod test;
