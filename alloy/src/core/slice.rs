//! The `slice` module provides utilities for operating with slices, which
//! extends logic provided in the standard library.
use std::cmp::Ordering;
use std::fmt;
use std::ops::{
  Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};

/// A strided slice is a slice that skips over a fixed number of elements
/// between each element.
///
/// This enables a "vertical" view of a contiguous slice, where the stride
/// is the number of elements to skip between each element.
///
/// This is a referential type, only intended to be used as `&StridedSlice<T, N>`,
/// or in larger compositions -- since it enables projecting a 2D-array view on
/// a 1D slice.
#[repr(transparent)]
pub struct StridedSlice<T, const N: usize>([T]);

// Constructors

impl<T, const N: usize> StridedSlice<T, N> {
  /// Constructs this strided slice from the underlying slice
  ///
  /// # Parameters
  ///
  /// * `slice` - the slice to construct from
  #[inline]
  #[must_use]
  pub const fn from_slice(slice: &[T]) -> &Self {
    unsafe { std::mem::transmute(slice) }
  }

  /// Constructs this strided slice from the underlying slice
  ///
  /// # Parameters
  ///
  /// * `slice` - the slice to construct from
  #[inline]
  #[must_use]
  pub fn from_mut_slice(slice: &mut [T]) -> &mut Self {
    unsafe { std::mem::transmute(slice) }
  }

  /// Constructs this strided slice from a raw pointer and length
  ///
  /// # Parameters
  ///
  /// * `ptr` - the pointer to the slice
  /// * `len` - the length of the slice
  ///
  /// # Safety
  ///
  /// The caller must ensure that the pointer is valid and that the length
  /// is within the bounds of the slice.
  pub const unsafe fn from_raw_parts<'a>(ptr: *const T, len: usize) -> &'a Self {
    std::mem::transmute(std::slice::from_raw_parts(ptr, len))
  }

  /// Constructs this mutable strided slice from a raw pointer and length
  ///
  /// # Parameters
  ///
  /// * `ptr` - the pointer to the slice
  /// * `len` - the length of the slice
  ///
  /// # Safety
  ///
  /// The caller must ensure that the pointer is valid and that the length
  /// is within the bounds of the slice.
  pub unsafe fn from_raw_parts_mut<'a>(ptr: *mut T, len: usize) -> &'a mut Self {
    std::mem::transmute(std::slice::from_raw_parts_mut(ptr, len))
  }
}

// Properties

impl<T, const N: usize> StridedSlice<T, N> {
  /// Returns the number of elements in the strided slice.
  #[inline]
  #[must_use]
  pub const fn len(&self) -> usize {
    (self.0.len() + N - 1) / N
  }

  /// Returns `true` if the slice has a length of 0.
  #[inline]
  #[must_use]
  pub const fn is_empty(&self) -> bool {
    self.len() == 0
  }

  /// Returns a raw pointer to the slice's buffer.
  ///
  /// The caller must ensure that the slice outlives the pointer this
  /// function returns, or else it will end up pointing to garbage.
  ///
  /// The caller must also ensure that the memory the pointer (non-transitively) points to
  /// is never written to (except inside an `UnsafeCell`) using this pointer or any pointer
  /// derived from it. If you need to mutate the contents of the slice, use [`as_mut_ptr`].
  ///
  /// Modifying the container referenced by this slice may cause its buffer
  /// to be reallocated, which would also make any pointers to it invalid.
  #[inline(always)]
  #[must_use]
  pub const fn as_ptr(&self) -> *const T {
    &self.0 as *const [T] as *const T
  }

  /// Returns an unsafe mutable pointer to the slice's buffer.
  ///
  /// The caller must ensure that the slice outlives the pointer this
  /// function returns, or else it will end up pointing to garbage.
  ///
  /// Modifying the container referenced by this slice may cause its buffer
  /// to be reallocated, which would also make any pointers to it invalid.
  #[inline(always)]
  #[must_use]
  pub fn as_mut_ptr(&mut self) -> *mut T {
    &mut self.0 as *mut [T] as *mut T
  }
}

// Operations

impl<T, const N: usize> StridedSlice<T, N> {
  /// Returns a reference to the first element of the slice, or `None` if it
  /// is empty.
  #[inline]
  #[must_use]
  pub const fn first(&self) -> Option<&T> {
    if let [first, ..] = &self.0 {
      Some(first)
    } else {
      None
    }
  }

  /// Returns a mutable pointer to the first element of the slice, or `None` if
  /// it is empty.
  #[inline]
  #[must_use]
  pub fn first_mut(&mut self) -> Option<&mut T> {
    if let [first, ..] = &mut self.0 {
      Some(first)
    } else {
      None
    }
  }

  /// Returns a reference to the last item in the slice, or `None` if it is
  /// empty.
  #[inline]
  #[must_use]
  pub const fn last(&self) -> Option<&T> {
    if let [.., last] = &self.0 {
      Some(last)
    } else {
      None
    }
  }

  /// Returns a mutable pointer to the last item in the slice.
  #[inline]
  #[must_use]
  pub fn last_mut(&mut self) -> Option<&mut T> {
    if let [.., last] = &mut self.0 {
      Some(last)
    } else {
      None
    }
  }

  /// Returns an iterator over the slice.
  #[inline(always)]
  pub fn iter(&self) -> impl Iterator<Item = &T> {
    self.0.iter().step_by(N)
  }

  /// Returns a mutable iterator over the slice.
  #[inline(always)]
  pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
    self.0.iter_mut().step_by(N)
  }

  /// Retrieves the value at the given index without doing proper bounds
  /// checking.
  ///
  /// # Parameters
  ///
  /// * `index` - the index to access
  ///
  /// # Safety
  ///
  /// `index` must be within the reachable bounds of the slice, or the result
  /// is undefined behavior.
  pub const unsafe fn index_unchecked(&self, index: usize) -> &T {
    let ptr = self.as_ptr();
    &*ptr.add(index * N)
  }

  /// Retrieves the a mutable reference to the given index without doing proper
  /// bounds checking.
  ///
  /// # Parameters
  ///
  /// * `index` - the index to access
  ///
  /// # Safety
  ///
  /// `index` must be within the reachable bounds of the slice, or the result
  /// is undefined behavior.
  pub unsafe fn index_mut_unchecked(&mut self, index: usize) -> &mut T {
    let ptr = self.as_mut_ptr();
    &mut *ptr.add(index * N)
  }
}

impl<T, const S: usize> PartialEq for &StridedSlice<T, S>
where
  T: PartialEq<T>,
{
  fn eq(&self, other: &Self) -> bool {
    if self.len() != other.len() {
      false
    } else {
      self.iter().eq(other.iter())
    }
  }
}

impl<T, const N: usize> PartialEq<&[T]> for StridedSlice<T, N>
where
  T: PartialEq<T>,
{
  fn eq(&self, other: &&[T]) -> bool {
    if self.len() != other.len() {
      false
    } else {
      self.iter().eq(other.iter())
    }
  }
}

impl<T, const N: usize> PartialEq<StridedSlice<T, N>> for &[T]
where
  T: PartialEq<T>,
{
  fn eq(&self, other: &StridedSlice<T, N>) -> bool {
    if self.len() != other.len() {
      false
    } else {
      self.iter().eq(other.iter())
    }
  }
}

impl<T, const N: usize> PartialOrd<&[T]> for StridedSlice<T, N>
where
  T: PartialOrd<T>,
{
  fn partial_cmp(&self, other: &&[T]) -> Option<Ordering> {
    let (lhs_len, rhs_len) = (self.len(), other.len());
    if lhs_len != rhs_len {
      lhs_len.partial_cmp(&rhs_len)
    } else {
      self.iter().partial_cmp(other.iter())
    }
  }
}

impl<T, const N: usize> PartialOrd<StridedSlice<T, N>> for &[T]
where
  T: PartialOrd<T>,
{
  fn partial_cmp(&self, other: &StridedSlice<T, N>) -> Option<Ordering> {
    let (lhs_len, rhs_len) = (self.len(), other.len());
    if lhs_len != rhs_len {
      lhs_len.partial_cmp(&rhs_len)
    } else {
      self.iter().partial_cmp(other.iter())
    }
  }
}

impl<T, const N: usize> Index<Range<usize>> for StridedSlice<T, N> {
  type Output = StridedSlice<T, N>;

  #[inline(always)]
  fn index(&self, index: Range<usize>) -> &Self::Output {
    unsafe { std::mem::transmute(Self::from_slice(&self.0[index.start * N..index.end * N])) }
  }
}

impl<T, const N: usize> IndexMut<Range<usize>> for StridedSlice<T, N> {
  #[inline(always)]
  fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
    unsafe {
      std::mem::transmute(Self::from_mut_slice(
        &mut self.0[index.start * N..index.end * N],
      ))
    }
  }
}

impl<T, const N: usize> Index<RangeFrom<usize>> for StridedSlice<T, N> {
  type Output = StridedSlice<T, N>;

  #[inline(always)]
  fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
    unsafe { std::mem::transmute(Self::from_slice(&self.0[index.start * N..])) }
  }
}

impl<T, const N: usize> IndexMut<RangeFrom<usize>> for StridedSlice<T, N> {
  #[inline(always)]
  fn index_mut(&mut self, index: RangeFrom<usize>) -> &mut Self::Output {
    unsafe { std::mem::transmute(Self::from_mut_slice(&mut self.0[index.start * N..])) }
  }
}

impl<T, const N: usize> Index<RangeTo<usize>> for StridedSlice<T, N> {
  type Output = StridedSlice<T, N>;

  #[inline(always)]
  fn index(&self, index: RangeTo<usize>) -> &Self::Output {
    unsafe {
      std::mem::transmute(Self::from_slice(
        &self.0[..(index.end * N).min(self.0.len())],
      ))
    }
  }
}

impl<T, const N: usize> IndexMut<RangeTo<usize>> for StridedSlice<T, N> {
  #[inline(always)]
  fn index_mut(&mut self, index: RangeTo<usize>) -> &mut Self::Output {
    unsafe {
      let len = self.0.len();
      std::mem::transmute(Self::from_mut_slice(
        &mut self.0[..(index.end * N).min(len)],
      ))
    }
  }
}

impl<T, const N: usize> Index<RangeInclusive<usize>> for StridedSlice<T, N> {
  type Output = StridedSlice<T, N>;

  #[inline(always)]
  fn index(&self, index: RangeInclusive<usize>) -> &Self::Output {
    unsafe {
      std::mem::transmute(Self::from_slice(
        &self.0[index.start() * N..=index.end() * N],
      ))
    }
  }
}

impl<T, const N: usize> IndexMut<RangeInclusive<usize>> for StridedSlice<T, N> {
  #[inline(always)]
  fn index_mut(&mut self, index: RangeInclusive<usize>) -> &mut Self::Output {
    unsafe {
      std::mem::transmute(Self::from_mut_slice(
        &mut self.0[index.start() * N..=index.end() * N],
      ))
    }
  }
}

impl<T, const N: usize> Index<RangeFull> for StridedSlice<T, N> {
  type Output = StridedSlice<T, N>;

  #[inline(always)]
  fn index(&self, _: RangeFull) -> &Self::Output {
    self
  }
}

impl<T, const N: usize> IndexMut<RangeFull> for StridedSlice<T, N> {
  #[inline(always)]
  fn index_mut(&mut self, _: RangeFull) -> &mut Self::Output {
    self
  }
}

impl<T, const N: usize> Index<RangeToInclusive<usize>> for StridedSlice<T, N> {
  type Output = StridedSlice<T, N>;

  #[inline(always)]
  fn index(&self, index: RangeToInclusive<usize>) -> &Self::Output {
    unsafe {
      std::mem::transmute(Self::from_slice(
        &self.0[..=(index.end * N).min(self.0.len())],
      ))
    }
  }
}

impl<T, const N: usize> IndexMut<RangeToInclusive<usize>> for StridedSlice<T, N> {
  #[inline(always)]
  fn index_mut(&mut self, index: RangeToInclusive<usize>) -> &mut Self::Output {
    unsafe {
      let len = self.0.len();
      std::mem::transmute(Self::from_mut_slice(
        &mut self.0[..=(index.end * N).min(len)],
      ))
    }
  }
}

impl<T, const N: usize> Index<usize> for StridedSlice<T, N> {
  type Output = T;
  #[inline(always)]
  fn index(&self, index: usize) -> &Self::Output {
    self.0.index(index * N)
  }
}

impl<T, const N: usize> IndexMut<usize> for StridedSlice<T, N> {
  #[inline(always)]
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    self.0.index_mut(index * N)
  }
}

impl<T, const N: usize> fmt::Debug for StridedSlice<T, N>
where
  T: fmt::Debug,
{
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_list().entries(self.iter()).finish()
  }
}

#[cfg(test)]
#[path = "slice.test.rs"]
mod test;
