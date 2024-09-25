//! The simd crate provides basic wrapper around the builtin simd types in the
//! Rust standard library.

pub(crate) mod common;

// Neon support

#[cfg(target_arch = "aarch64")]
mod float32x4_neon;

#[doc(inline)]
#[cfg(target_arch = "aarch64")]
pub use float32x4_neon::*;

// SSE support

// TODO: Implement SSE support

// No supported

#[cfg(not(target_arch = "aarch64"))]
mod float32x4_none;

#[doc(inline)]
#[cfg(not(target_arch = "aarch64"))]
pub use float32x4_none::*;

impl Float32x4 {
  /// The alignment of the [Float32x4] type.
  pub const ALIGN: usize = 16;

  const ZERO: f32 = 0.0;

  /// Create a new [Float32x4] from the elements within the slice, taking care
  /// to handle both aligned and unaligned slices -- as well as padding slices
  /// that contain less than 4 elements with zeros.
  ///
  /// # Parameters
  ///
  /// * `slice` - the slice of [f32] values to create the vector from
  pub fn from_slice(slice: &[f32]) -> Self {
    match slice.len() {
      0 => Self::splat(Self::ZERO),
      1 => Self::new(slice[0], Self::ZERO, Self::ZERO, Self::ZERO),
      2 => Self::new(slice[0], slice[1], Self::ZERO, Self::ZERO),
      3 => Self::new(slice[0], slice[1], slice[2], Self::ZERO),
      _ => {
        if slice.as_ptr().align_offset(Self::ALIGN) == 0 {
          unsafe { Self::from_aligned_ptr(slice.as_ptr()) }
        } else {
          unsafe { Self::from_unaligned_ptr(slice.as_ptr()) }
        }
      }
    }
  }

  /// Create a new [Float32x4] from the elements within the aligned slice.
  ///
  /// # Parameters
  ///
  /// * `slice` - the aligned slice of [f32] values to create the vector from
  ///
  /// # Safety
  ///
  /// The slice must be aligned to 16 bytes
  pub unsafe fn from_aligned_slice(slice: &[f32]) -> Self {
    match slice.len() {
      0 => Self::splat(Self::ZERO),
      1 => Self::new(slice[0], Self::ZERO, Self::ZERO, Self::ZERO),
      2 => Self::new(slice[0], slice[1], Self::ZERO, Self::ZERO),
      3 => Self::new(slice[0], slice[1], slice[2], Self::ZERO),
      _ => Self::from_aligned_ptr(slice.as_ptr()),
    }
  }

  /// Create a new [Float32x4] from the elements within the aligned slice.
  ///
  /// # Parameters
  ///
  /// * `slice` - the aligned slice of [f32] values to create the vector from
  ///
  /// # Safety
  ///
  /// The slice must be aligned to 16 bytes, and the length of the slice must be
  /// a multiple of 4.
  #[inline(always)]
  pub unsafe fn from_aligned_slice_exact(slice: &[f32]) -> Self {
    Self::from_aligned_ptr(slice.as_ptr())
  }

  /// Create a new [Float32x4] from the elements within the aligned slice.
  ///
  /// # Parameters
  ///
  /// * `slice` - the aligned slice of [f32] values to create the vector from
  ///
  /// # Safety
  ///
  /// The slice must contain a multiple of 4 elements.
  pub unsafe fn from_unaligned_slice_exact(slice: &[f32]) -> Self {
    Self::from_unaligned_ptr(slice.as_ptr())
  }

  /// Create a new [Float32x4] from the elements within the unaligned slice.
  ///
  /// # Parameters
  ///
  /// * `slice` - the unaligned slice of [f32] values to create the vector from
  pub fn from_unaligned_slice(slice: &[f32]) -> Self {
    match slice.len() {
      0 => Self::splat(Self::ZERO),
      1 => Self::new(slice[0], Self::ZERO, Self::ZERO, Self::ZERO),
      2 => Self::new(slice[0], slice[1], Self::ZERO, Self::ZERO),
      3 => Self::new(slice[0], slice[1], slice[2], Self::ZERO),
      _ => unsafe { Self::from_unaligned_ptr(slice.as_ptr()) },
    }
  }

  /// Returns an iterator that yields [Float32x4] values from the given slice.
  ///
  /// If the slice is not divisible by 4, the remaining elements will be padded
  /// with zeros.
  ///
  /// # Parameters
  ///
  /// * `slice` - the slice of [f32] values to create the vector from
  pub fn iter_slice(slice: &[f32]) -> impl Iterator<Item = Self> + '_ {
    slice.chunks(4).map(Self::from_slice)
  }

  /// Returns an iterator that yields [Float32x4] values from the given slice.
  ///
  /// # Parameters
  ///
  /// * `slice` - the slice of [f32] values to create the vector from
  ///
  /// # Safety
  ///
  /// The slice must contain a multiple of 4 elements.
  pub unsafe fn iter_slice_exact(slice: &[f32]) -> impl Iterator<Item = Self> + '_ {
    Float32x4ExactIter { slice }
  }
}

struct Float32x4ExactIter<'a> {
  slice: &'a [f32],
}

impl<'a> Iterator for Float32x4ExactIter<'a> {
  type Item = Float32x4;

  fn next(&mut self) -> Option<Self::Item> {
    if self.slice.is_empty() {
      return None;
    }

    let vec = unsafe { Float32x4::from_aligned_slice_exact(self.slice) };
    self.slice = &self.slice[4..];
    Some(vec)
  }
}

impl FromIterator<f32> for Float32x4 {
  /// Create a new [Float32x4] from the first 4 elements within the iterator.
  ///
  /// # Parameters
  ///
  /// * `iter` - the iterator of [f32] values to create the vector from
  fn from_iter<I: IntoIterator<Item = f32>>(iter: I) -> Self {
    let mut arr = common::F32x4Array::new([Self::ZERO; 4]);
    for (i, val) in iter.into_iter().take(4).enumerate() {
      arr[i] = val;
    }
    unsafe { Self::from_aligned_ptr(arr.as_ptr()) }
  }
}

impl Default for Float32x4 {
  /// Create a new [Float32x4] with all components set to zero.
  #[inline(always)]
  fn default() -> Self {
    Self::splat(Self::ZERO)
  }
}

impl From<[f32; 4]> for Float32x4 {
  /// Create a new [Float32x4] from the given array of [f32] values.
  ///
  /// # Parameters
  ///
  /// * `arr` - the array of [f32] values to create the vector from
  #[inline(always)]
  fn from(arr: [f32; 4]) -> Self {
    Self::from_slice(&arr)
  }
}

impl From<&[f32]> for Float32x4 {
  /// Create a new [Float32x4] from the given slice of [f32] values.
  ///
  /// # Parameters
  ///
  /// * `slice` - the slice of [f32] values to create the vector from
  #[inline(always)]
  fn from(slice: &[f32]) -> Self {
    Self::from_slice(slice)
  }
}

impl From<Float32x4> for [f32; 4] {
  /// Create a new array of [f32] values from the given [Float32x4].
  ///
  /// # Parameters
  ///
  /// * `vec` - the [Float32x4] to create the array from
  fn from(vec: Float32x4) -> Self {
    let mut arr = [0.0; 4];

    // SAFETY: The array is always aligned, and contains 4 elements.
    unsafe { vec.store_aligned(arr.as_mut_ptr()) };
    arr
  }
}
