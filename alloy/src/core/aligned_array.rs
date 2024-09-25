use crate::core::{Align, Alignment};

/// An array that is aligned to a particular boundary.
pub struct AlignedArray<T, const N: usize, const A: usize>(Align<A>, [T; N])
where
  Align<A>: Alignment;

impl<T, const N: usize, const A: usize> Default for AlignedArray<T, N, A>
where
  Align<A>: Alignment,
  [T; N]: Default,
{
  fn default() -> Self {
    Self(Align::default(), <[T; N]>::default())
  }
}

impl<T, const N: usize, const A: usize> Copy for AlignedArray<T, N, A>
where
  Align<A>: Alignment,
  [T; N]: Copy,
{
}

impl<T, const N: usize, const A: usize> Clone for AlignedArray<T, N, A>
where
  Align<A>: Alignment,
  [T; N]: Clone,
{
  fn clone(&self) -> Self {
    Self(self.0, self.1.clone())
  }
}

impl<T, const N: usize, const A: usize> std::fmt::Debug for AlignedArray<T, N, A>
where
  Align<A>: Alignment,
  [T; N]: std::fmt::Debug,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_tuple("AlignedArray").field(&self.1).finish()
  }
}

impl<T, const N: usize, const A: usize> PartialEq for AlignedArray<T, N, A>
where
  Align<A>: Alignment,
  [T; N]: PartialEq,
{
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.1 == other.1
  }
}

impl<T, const N: usize, const A: usize> Eq for AlignedArray<T, N, A>
where
  Align<A>: Alignment,
  [T; N]: Eq,
{
}

impl<T, const N: usize, const A: usize> std::hash::Hash for AlignedArray<T, N, A>
where
  Align<A>: Alignment,
  [T; N]: std::hash::Hash,
{
  #[inline]
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.1.hash(state)
  }
}

impl<T, const N: usize, const A: usize> PartialOrd for AlignedArray<T, N, A>
where
  Align<A>: Alignment,
  [T; N]: PartialOrd,
{
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    self.1.partial_cmp(&other.1)
  }
}

impl<T, const N: usize, const A: usize> Ord for AlignedArray<T, N, A>
where
  Align<A>: Alignment,
  [T; N]: Ord,
{
  #[inline]
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.1.cmp(&other.1)
  }
}

impl<T, const N: usize, const A: usize> From<[T; N]> for AlignedArray<T, N, A>
where
  Align<A>: Alignment,
{
  #[inline]
  fn from(elements: [T; N]) -> Self {
    Self(Align::new(), elements)
  }
}

impl<T, const N: usize, const A: usize> AsRef<[T]> for AlignedArray<T, N, A>
where
  Align<A>: Alignment,
{
  #[inline]
  fn as_ref(&self) -> &[T] {
    &self.1
  }
}

impl<T, const N: usize, const A: usize> AsMut<[T]> for AlignedArray<T, N, A>
where
  Align<A>: Alignment,
{
  #[inline]
  fn as_mut(&mut self) -> &mut [T] {
    &mut self.1
  }
}

impl<T, const N: usize, const A: usize> std::ops::Deref for AlignedArray<T, N, A>
where
  Align<A>: Alignment,
{
  type Target = [T; N];

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.1
  }
}

impl<T, const N: usize, const A: usize> std::ops::DerefMut for AlignedArray<T, N, A>
where
  Align<A>: Alignment,
{
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.1
  }
}

impl<T, const N: usize, const A: usize> std::ops::Index<usize> for AlignedArray<T, N, A>
where
  Align<A>: Alignment,
{
  type Output = T;

  #[inline]
  fn index(&self, index: usize) -> &Self::Output {
    &self.1[index]
  }
}

impl<T, const N: usize, const A: usize> std::ops::IndexMut<usize> for AlignedArray<T, N, A>
where
  Align<A>: Alignment,
{
  #[inline]
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    &mut self.1[index]
  }
}

impl<T, const N: usize, const A: usize> AlignedArray<T, N, A>
where
  Align<A>: Alignment,
{
  /// Create a new aligned array with the given elements.
  ///
  /// # Parameters
  ///
  /// * `elements` - the elements to store in the array
  pub const fn new(elements: [T; N]) -> Self {
    Self(Align::new(), elements)
  }

  /// Get a pointer to the first element in the array.
  pub const fn as_ptr(&self) -> *const T {
    self.1.as_ptr()
  }

  /// Get a mutable pointer to the first element in the array.
  pub fn as_mut_ptr(&mut self) -> *mut T {
    self.1.as_mut_ptr()
  }

  /// Get the inner array.
  pub fn into_inner(self) -> [T; N] {
    self.1
  }
}
