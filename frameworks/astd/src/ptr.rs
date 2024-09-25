//! A module that provides utility support for Pointers.
//!
//! This is largely a module containing iterators, thoguh it also offers basic
//! functionality that is either missing from the standard crate, or is otherwise
//! unavailabile due to instability.`

use core::ffi::c_uint;
use std::ops::Range;

/// An iterator over a contiguous sequence created from a pointer and the size.
pub struct Iter<'a, T>(std::slice::Iter<'a, T>);

impl<'a, T> std::ops::Deref for Iter<'a, T> {
  type Target = std::slice::Iter<'a, T>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<'a, T> std::ops::DerefMut for Iter<'a, T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl<'a, T> Iterator for Iter<'a, T> {
  type Item = &'a T;
  fn next(&mut self) -> Option<Self::Item> {
    self.0.next()
  }
}

impl<'a, T> Iter<'a, T> {
  /// Constructs this [`PtrIter`] from a pointer and length.
  ///
  /// # Parameters
  ///
  /// * `ptr` - the pointer to the start of the contiguous data
  /// * `len` - the length of the contiguous sequence
  ///
  /// # Safety
  ///
  /// This iterator requires that `ptr` and `len` accurately model the amount
  /// of storage that is contiguously accessible.
  pub unsafe fn from_ptr_unchecked(ptr: *const T, len: c_uint) -> Self {
    let slice = std::slice::from_raw_parts(ptr, len as usize);
    Self::from_slice(slice)
  }

  /// Constructs this [`PtrIter`] from a range of pointer entries.
  ///
  /// # Parameters
  ///
  /// * `range` - the range of pointer entries
  ///
  /// # Safety
  ///
  /// This iterator requires that the `range.start <= range.end`, otherwise this
  /// will exhibit undefined behavior.
  pub unsafe fn from_ptr_range(range: Range<*const T>) -> Self {
    debug_assert!(range.start < range.end);
    Self::from_ptr_unchecked(range.start, range.start.offset_from(range.end) as c_uint)
  }

  /// Constructs this [`PtrIter`] from a slice of objects.
  ///
  /// # Parameters
  ///
  /// * `slice` - the slice to construct from
  #[inline(always)]
  pub fn from_slice(slice: &'a [T]) -> Self {
    Self(slice.iter())
  }
}

/// An iterator over a mutable contiguous sequence created from a pointer and
/// the size.
pub struct IterMut<'a, T>(std::slice::IterMut<'a, T>);

impl<'a, T> std::ops::Deref for IterMut<'a, T> {
  type Target = std::slice::IterMut<'a, T>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<'a, T> std::ops::DerefMut for IterMut<'a, T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl<'a, T> Iterator for IterMut<'a, T> {
  type Item = &'a mut T;
  fn next(&mut self) -> Option<Self::Item> {
    self.0.next()
  }
}

impl<'a, T> IterMut<'a, T> {
  /// Constructs this [`PtrIter`] from a pointer and length.
  ///
  /// # Parameters
  ///
  /// * `ptr` - the pointer to the start of the contiguous data
  /// * `len` - the length of the contiguous sequence
  ///
  /// # Safety
  ///
  /// This iterator requires that `ptr` and `len` accurately model the amount
  /// of storage that is contiguously accessible.
  pub unsafe fn from_ptr_unchecked(ptr: *mut T, len: c_uint) -> Self {
    let slice = std::slice::from_raw_parts_mut(ptr, len as usize);
    Self::from_slice(slice)
  }

  /// Constructs this [`PtrIter`] from a range of pointer entries.
  ///
  /// # Parameters
  ///
  /// * `range` - the range of pointer entries
  ///
  /// # Safety
  ///
  /// This iterator requires that the `range.start <= range.end`, otherwise this
  /// will exhibit undefined behavior.
  pub unsafe fn from_ptr_range(range: Range<*mut T>) -> Self {
    debug_assert!(range.start < range.end);
    Self::from_ptr_unchecked(range.start, range.start.offset_from(range.end) as c_uint)
  }

  /// Constructs this [`PtrIter`] from a slice of objects.
  ///
  /// # Parameters
  ///
  /// * `slice` - the slice to construct from
  #[inline(always)]
  pub fn from_slice(slice: &'a mut [T]) -> Self {
    Self(slice.iter_mut())
  }
}
