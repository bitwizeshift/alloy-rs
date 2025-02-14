use std::borrow::{Borrow, BorrowMut};
use std::fmt;
use std::ops::{
  Add, AddAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub,
  SubAssign,
};

use crate::core::hint;
use crate::math::vec::{Vec4, Vector4};
use crate::ops::Dot;

#[doc(inline)]
pub use crate::math::mat::col4::Col4;

/// A 4x4 non-owning view of a [matrix].
///
/// [matrix]: https://en.wikipedia.org/wiki/Matrix_(mathematics)
///
/// # Guarantees
///
/// This type has the strict requirement that it can only reference a
/// 4x4-component slice of [`f32`] values. It is guaranteed to never refer to
/// more or less than 2 entries.
///
/// This type is always in row-column order, meaning that the first index
/// is the row, and the second index is the column.
///
/// # Relation to [`Matrix4`]
///
/// This type is a non-owning view of a 4x4 matrix, and is used to provide
/// a way to reference a 4x4 matrix without taking ownership of the data.
#[repr(transparent)]
#[derive(PartialEq, PartialOrd)]
pub struct Mat4([f32]);

// Constructors

impl Mat4 {
  /// The number of columns this matrix contains
  pub const COLS: usize = 4;

  /// The number of rows this matrix contains.
  pub const ROWS: usize = 4;

  /// Creates a new 4x4 matrix view from a slice of 16 [`f32`] values.
  ///
  /// # Parameters
  ///
  /// * `data` - A slice of 16 [`f32`] values.
  #[must_use]
  #[inline(always)]
  pub const fn from_arrays(data: &[[f32; 4]; 4]) -> &Mat4 {
    // SAFETY: `[T]` is layout-identical to `[[T; N]; N]`
    unsafe { std::mem::transmute(std::slice::from_raw_parts(data.as_ptr().cast::<f32>(), 16)) }
  }

  /// Creates a new 4x4 matrix view from a mutable slice of 16 [`f32`] values.
  ///
  ///
  /// # Parameters
  ///
  /// * `data` - A slice of 16 [`f32`] values.
  #[must_use]
  #[inline(always)]
  pub fn from_mut_array(data: &mut [[f32; 4]; 4]) -> &mut Mat4 {
    // SAFETY: `[T]` is layout-identical to `[[T; N]; N]`
    unsafe {
      std::mem::transmute(std::slice::from_raw_parts_mut(
        data.as_mut_ptr().cast::<f32>(),
        16,
      ))
    }
  }

  /// Creates a new 4x4 matrix view from a slice of 16 [`f32`] values.
  /// If the length of the slice is not 16, this function will return [`None`].
  ///
  /// # Parameters
  ///
  /// * `data` - A slice of 16 [`f32`] values.
  pub const fn from_array_slice(data: &[[f32; 4]]) -> Option<&Mat4> {
    if data.len() != 4 {
      None
    } else {
      // SAFETY: The above check ensures that the length of the slice is 4.
      Some(unsafe { Self::from_array_slice_unchecked(data) })
    }
  }

  /// Creates a new 4x4 matrix view from a mutable slice of 16 [`f32`] values.
  /// If the length of the slice is not 16, this function will return [`None`].
  ///
  /// # Parameters
  ///
  /// * `data` - A slice of 16 [`f32`] values.
  pub fn from_mut_array_slice(data: &mut [[f32; 4]]) -> Option<&mut Mat4> {
    if data.len() != 4 {
      None
    } else {
      Some(unsafe { Self::from_mut_array_slice_unchecked(data) })
    }
  }

  /// Creates a new 4x4 matrix view from a slice of 16 [`f32`] values without
  /// performing bounds checking.
  ///
  /// # Safety
  ///
  /// This function is unsafe because it does not check that the given slice
  /// has a length of 16. If this constraint is violated, memory safety errors
  /// can occur.
  ///
  /// # Parameters
  ///
  /// * `data` - A slice of 16 [`f32`] values.
  #[must_use]
  #[inline(always)]
  pub const unsafe fn from_array_slice_unchecked(data: &[[f32; 4]]) -> &Mat4 {
    let len = data.len() * 4;

    // SAFETY: `[T]` is layout-identical to `[[T; N]; N]`
    unsafe { std::mem::transmute(std::slice::from_raw_parts(data.as_ptr().cast::<f32>(), len)) }
  }

  /// Creates a new 4x4 matrix view from a mutable slice of 16 [`f32`] values
  /// without performing bounds checking.
  ///
  /// # Safety
  ///
  /// This function is unsafe because it does not check that the given slice
  /// has a length of 16. If this constraint is violated, memory safety errors
  /// can occur.
  ///
  /// # Parameters
  ///
  /// * `data` - A slice of 16 [`f32`] values.
  #[must_use]
  #[inline(always)]
  pub unsafe fn from_mut_array_slice_unchecked(data: &mut [[f32; 4]]) -> &mut Mat4 {
    let len = data.len() * 4;

    // SAFETY: `[T]` is layout-identical to `[[T; N]; N]`
    unsafe {
      std::mem::transmute(std::slice::from_raw_parts_mut(
        data.as_mut_ptr().cast::<f32>(),
        len,
      ))
    }
  }

  /// Creates a new 4x4 matrix view from a slice of at least 16 [`f32`] values.
  /// If the length of the slice is less than 16, this function will return
  /// [`None`].
  ///
  /// # Parameters
  ///
  /// * `data` - A slice of at least 16 [`f32`] values.
  #[must_use]
  pub const fn from_slice(data: &[f32]) -> Option<&Mat4> {
    if data.len() != 16 {
      None
    } else {
      // SAFETY: The above check ensures that the length of the slice is 16.
      Some(unsafe { Self::from_slice_unchecked(data) })
    }
  }

  /// Creates a new 4x4 matrix view from a mutable slice of at least 16 [`f32`]
  /// values. If the length of the slice is less than 16, this function will
  /// return [`None`].
  ///
  /// # Parameters
  ///
  /// * `data` - A slice of at least 16 [`f32`] values.
  #[must_use]
  pub fn from_mut_slice(data: &mut [f32]) -> Option<&mut Mat4> {
    if data.len() != 16 {
      None
    } else {
      Some(unsafe { Self::from_mut_slice_unchecked(data) })
    }
  }

  /// Forms a reference to a [`Mat4`] from a flat slice of at least 16 [`f32`]
  /// values.
  ///
  /// # Safety
  ///
  /// The caller must guarantee that the slice has a length of 16.
  ///
  /// # Parameters
  ///
  /// * `data` - the slice of [`f32`] values
  #[must_use]
  #[inline(always)]
  pub const unsafe fn from_slice_unchecked(data: &[f32]) -> &Mat4 {
    // SAFETY: The caller must guarantee that the slice has a length of 16.
    std::mem::transmute(data)
  }

  /// Forms a mutable reference to a [`Mat4`] from a flat slice of at least 16
  /// [`f32`] values.
  ///
  /// # Safety
  ///
  /// The caller must guarantee that the slice has a length of 16.
  ///
  /// # Parameters
  ///
  /// * `data` - the slice of [`f32`] values
  #[must_use]
  #[inline(always)]
  pub unsafe fn from_mut_slice_unchecked(data: &mut [f32]) -> &mut Mat4 {
    // SAFETY: The caller must guarantee that the slice has a length of 16.
    std::mem::transmute(data)
  }

  /// Forms a reference to a [`Mat4`] from a pointer to a contiguous sequence
  /// of at least 16 [`f32`]s.
  ///
  /// # Parameters
  ///
  /// * `ptr` - the pointer to a sequence of [`f32`] values
  ///
  /// # Safety
  ///
  /// `ptr` must point to an allocated object that references at least two
  /// entries
  #[must_use]
  #[inline(always)]
  pub const unsafe fn from_ptr_unchecked<'a>(ptr: *const f32) -> &'a Mat4 {
    Mat4::from_slice_unchecked(std::slice::from_raw_parts(ptr, 16))
  }

  /// Forms a mutable reference to a [`Mat4`] from a pointer to a contiguous
  /// sequence of at least 16 [`f32`]s.
  ///
  /// # Parameters
  ///
  /// * `ptr` - the pointer to a sequence of [`f32`] values
  ///
  /// # Safety
  ///
  /// `ptr` must point to an allocated object that references at least two
  /// entries
  #[must_use]
  #[inline(always)]
  pub unsafe fn from_mut_ptr_unchecked<'a>(ptr: *mut f32) -> &'a mut Vec4 {
    Vec4::from_mut_slice_unchecked(std::slice::from_raw_parts_mut(ptr, 16))
  }
}

// Conversions

impl Mat4 {
  /// Returns this [`Mat4`] as a slice of [`f32`].
  #[must_use]
  #[inline(always)]
  pub const fn as_slice(&self) -> &[f32] {
    &self.0
  }

  /// Returns this [`Mat4`] as a mutable slice of [`f32`].
  #[must_use]
  #[inline(always)]
  pub fn as_mut_slice(&mut self) -> &mut [f32] {
    &mut self.0
  }

  /// Returns this vector as a ptr of [`f32`].
  #[must_use]
  #[inline(always)]
  pub const fn as_ptr(&self) -> *const f32 {
    self.as_slice().as_ptr()
  }

  /// Returns this vector as a mutable ptr of [`f32`].
  #[must_use]
  #[inline(always)]
  pub fn as_mut_ptr(&mut self) -> *mut f32 {
    self.as_mut_slice().as_mut_ptr()
  }

  /// Returns this [`Mat4`] as a slice of 4-component arrays of [`f32`].
  #[must_use]
  #[inline(always)]
  pub const fn as_array_slice(&self) -> &[[f32; 4]] {
    self.as_array_ref().as_slice()
  }

  /// Returns this [`Mat4`] as a mutable slice of 4-component arrays of [`f32`].
  #[must_use]
  #[inline(always)]
  pub fn as_mut_array_slice(&mut self) -> &mut [[f32; 4]] {
    self.as_mut_array_ref().as_mut_slice()
  }

  /// Returns this [`Mat4`] as a reference to a 4x4 array of [`f32`].
  #[must_use]
  #[inline(always)]
  pub const fn as_array_ref(&self) -> &[[f32; 4]; 4] {
    // SAFETY: The length of the slice is guaranteed to be 16.
    unsafe { crate::core::hint::fixed_size(&self.0, 16) };

    // Note: This was previously defined in terms of [`transmute`], but miri
    // had issues with it. This does not flag miri and is equivalent.
    let ptr = self.0.as_ptr() as *const [[f32; 4]; 4];

    // SAFETY: The length of the slice is guaranteed to be 16.
    unsafe { &*ptr }
  }

  /// Returns this [`Mat4`] as a mutable reference to a 4x4 array of [`f32`].
  #[must_use]
  #[inline(always)]
  pub fn as_mut_array_ref(&mut self) -> &mut [[f32; 4]; 4] {
    // SAFETY: The length of the slice is guaranteed to be 16.
    unsafe { crate::core::hint::fixed_size(&self.0, 16) };

    // Note: This was previously defined in terms of [`transmute`], but miri
    // had issues with it. This does not flag miri and is equivalent.
    let ptr = self.0.as_mut_ptr() as *mut [[f32; 4]; 4];

    // SAFETY: The length of the slice is guaranteed to be 16.
    unsafe { &mut *ptr }
  }

  /// Returns this [`Mat4`] as a [`Matrix4`].
  ///
  /// This function is `const`, unlike the equivalent [`From`] or [`ToOwned`]
  /// implementations.
  pub const fn to_matrix4(&self) -> Matrix4 {
    Matrix4::from_mat4(self)
  }
}

impl ToOwned for Mat4 {
  type Owned = Matrix4;

  #[must_use]
  #[inline]
  fn to_owned(&self) -> Self::Owned {
    self.to_matrix4()
  }
}

// Accessors

impl Mat4 {
  /// Returns the row at the given index.
  ///
  /// This function will panic if the given index is out of bounds.
  ///
  /// # Parameters
  ///
  /// * `index` - The index of the row to return.
  #[must_use]
  #[inline(always)]
  pub const fn row(&self, index: usize) -> &Vec4 {
    // SAFETY: the length of the slice is always 4, or it panics.
    unsafe { Vec4::from_slice_unchecked(&self.as_array_ref()[index]) }
  }

  /// Returns the row at the given index without performing bounds checking.
  ///
  /// # Safety
  ///
  /// This function is unsafe because it does not check that the given index is
  /// less than 4. If this constraint is violated, memory safety errors can occur.
  ///
  /// # Parameters
  ///
  /// * `index` - The index of the row to return.
  #[must_use]
  #[inline(always)]
  pub const unsafe fn row_unchecked(&self, index: usize) -> &Vec4 {
    // SAFETY: The caller must guarantee that the index is in bounds.
    unsafe { Vec4::from_slice_unchecked(&*self.as_array_ref().as_ptr().add(index)) }
  }

  /// Returns a mutable reference to the row at the given index.
  ///
  /// This function will panic if the given index is out of bounds.
  ///
  /// # Parameters
  ///
  /// * `index` - The index of the row to return.
  #[must_use]
  #[inline(always)]
  pub fn mut_row(&mut self, index: usize) -> &mut Vec4 {
    // SAFETY: the length of the slice is always 4, or it panics.
    unsafe { Vec4::from_mut_slice_unchecked(&mut self.as_mut_array_ref()[index]) }
  }

  /// Returns a mutable reference to the row at the given index without
  /// performing bounds checking.
  ///
  /// # Safety
  ///
  /// This function is unsafe because it does not check that the given index is
  /// less than 4. If this constraint is violated, memory safety errors can
  /// occur.
  ///
  /// # Parameters
  ///
  /// * `index` - The index of the row to return.
  #[must_use]
  #[inline(always)]
  pub unsafe fn mut_row_unchecked(&mut self, index: usize) -> &mut Vec4 {
    Vec4::from_mut_slice_unchecked(&mut *self.as_mut_array_ref().as_mut_ptr().add(index))
  }

  /// Returns the column at the given index.
  ///
  /// This function will panic if the given index is out of bounds.
  ///
  /// # Parameters
  ///
  /// * `index` - The index of the column to return.
  #[must_use]
  pub const fn col(&self, index: usize) -> &Col4 {
    if hint::unlikely(index >= 4) {
      panic!("index out of bounds")
    } else {
      // SAFETY: The above check ensures that the index is in bounds.
      unsafe { self.col_unchecked(index) }
    }
  }

  /// Returns the column at the given index without performing bounds checking.
  ///
  /// **Note:** Unlike [`Mat4::row_unchecked`], this function returns an owning
  /// [`Vector4`] instead of a reference of [`Vec4`].
  ///
  /// # Safety
  ///
  /// This function is unsafe because it does not check that the given index is
  /// less than 4. If this constraint is violated, memory safety errors can occur.
  ///
  /// # Parameters
  ///
  /// * `index` - The index of the column to return.
  #[must_use]
  pub const unsafe fn col_unchecked(&self, index: usize) -> &Col4 {
    let ptr = self.as_ptr().add(index);
    let len = self.0.len() - index;
    Col4::from_raw_parts(ptr, len)
  }

  /// Returns a mutable reference to the column at the given index.
  ///
  /// This function will panic if the given index is out of bounds.
  ///
  /// # Parameters
  ///
  /// * `index` - The index of the column to return.
  #[must_use]
  pub fn mut_col(&mut self, index: usize) -> &mut Col4 {
    if hint::unlikely(index >= 4) {
      panic!("index out of bounds")
    } else {
      // SAFETY: The above check ensures that the index is in bounds.
      unsafe { self.mut_col_unchecked(index) }
    }
  }

  /// Returns a mutable reference to the column at the given index without
  /// performing bounds checking.
  ///
  /// # Safety
  ///
  /// This function is unsafe because it does not check that the given index is
  /// less than 4. If this constraint is violated, memory safety errors can occur.
  ///
  /// # Parameters
  ///
  /// * `index` - The index of the column to return.
  #[must_use]
  pub unsafe fn mut_col_unchecked(&mut self, index: usize) -> &mut Col4 {
    let ptr = self.as_mut_ptr().add(index);
    let len = self.0.len() - index;
    Col4::from_raw_parts_mut(ptr, len)
  }

  /// Returns the value at the given column and row.
  ///
  /// This function will panic if the given index is out of bounds.
  ///
  /// # Parameters
  ///
  /// * `row` - The index of the row to return.
  /// * `col` - The index of the column to return.
  #[must_use]
  pub const fn get(&self, row: usize, col: usize) -> f32 {
    self.as_array_ref()[row][col]
  }

  /// Returns the value at the given column and row without performing bounds
  /// checking.
  ///
  /// # Safety
  ///
  /// This function is unsafe because it does not check that the given index is
  /// less than 4. If this constraint is violated, memory safety errors can occur.
  ///
  /// # Parameters
  ///
  /// * `row` - The index of the row to return.
  /// * `col` - The index of the column to return.
  #[must_use]
  #[inline(always)]
  pub const unsafe fn get_unchecked(&self, row: usize, col: usize) -> f32 {
    // This awful word-salad of code helps to enable unchecked indexing
    // for faster access-times without introducing branches.
    let row = &*self.as_array_ref().as_ptr().add(row);

    row.as_ptr().add(col).read()
  }

  /// Returns a mutable reference to the value at the given column and row.
  ///
  /// This function will panic if the given index is out of bounds.
  ///
  /// # Parameters
  ///
  /// * `row` - The index of the row to return.
  /// * `col` - The index of the column to return.
  #[must_use]
  pub fn get_mut(&mut self, row: usize, col: usize) -> &mut f32 {
    &mut self.as_mut_array_ref()[row][col]
  }

  /// Returns a mutable reference to the value at the given column and row
  /// without performing bounds checking.
  ///
  /// # Safety
  ///
  /// This function is unsafe because it does not check that the given index is
  /// less than 4. If this constraint is violated, memory safety errors can occur.
  ///
  /// # Parameters
  ///
  /// * `row` - The index of the row to return.
  /// * `col` - The index of the column to return.
  #[must_use]
  #[inline(always)]
  pub unsafe fn get_mut_unchecked(&mut self, row: usize, col: usize) -> &mut f32 {
    let row = &mut *self.as_mut_array_ref().as_mut_ptr().add(row);

    &mut *row.as_mut_ptr().add(col)
  }
}

impl Index<(usize, usize)> for Mat4 {
  type Output = f32;

  fn index(&self, (row, col): (usize, usize)) -> &f32 {
    &self.0[row * 4 + col]
  }
}

impl IndexMut<(usize, usize)> for Mat4 {
  fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut f32 {
    &mut self.0[row * 4 + col]
  }
}

impl Index<usize> for Mat4 {
  type Output = Vec4;

  fn index(&self, index: usize) -> &Vec4 {
    self.row(index)
  }
}

impl IndexMut<usize> for Mat4 {
  fn index_mut(&mut self, index: usize) -> &mut Vec4 {
    self.mut_row(index)
  }
}

// Modifiers

impl Mat4 {
  /// Scales each component of the matrix by the given scalar.
  ///
  /// # Parameters
  ///
  /// * `scale` - The scalar to multiply each component by.
  pub fn scale(&mut self, scale: f32) {
    for row in 0..4 {
      for col in 0..4 {
        self[row][col] *= scale;
      }
    }
  }

  /// Scales each row of the matrix by the given vector.
  /// This is equivalent to multiplying the matrix by a diagonal matrix with
  /// the given vector on the diagonal.
  ///
  /// # Parameters
  ///
  /// * `scale` - The vector to multiply each row by.
  pub fn scale_vec(&mut self, scale: &Vec4) {
    for row in 0..4 {
      for col in 0..4 {
        self[row][col] *= scale[row];
      }
    }
  }

  /// Transposes all the elements of this matrix, swapping rows and columns.
  pub fn transpose(&mut self) {
    for row in 0..4 {
      for col in row + 1..4 {
        (self[row][col], self[col][row]) = (self[col][row], self[row][col]);
      }
    }
  }

  /// Inverts this matrix.
  pub fn invert(&mut self) {
    let mut inv = Matrix4::default(); // The resultant matrix

    inv[0][0] = self[1][1] * self[2][2] * self[3][3]
      - self[1][1] * self[2][3] * self[3][2]
      - self[2][1] * self[1][2] * self[3][3]
      + self[2][1] * self[1][3] * self[3][2]
      + self[3][1] * self[1][2] * self[2][3]
      - self[3][1] * self[1][3] * self[2][2];

    inv[1][0] = -self[1][0] * self[2][2] * self[3][3]
      + self[1][0] * self[2][3] * self[3][2]
      + self[2][0] * self[1][2] * self[3][3]
      - self[2][0] * self[1][3] * self[3][2]
      - self[3][0] * self[1][2] * self[2][3]
      + self[3][0] * self[1][3] * self[2][2];

    inv[2][0] = self[1][0] * self[2][1] * self[3][3]
      - self[1][0] * self[2][3] * self[3][1]
      - self[2][0] * self[1][1] * self[3][3]
      + self[2][0] * self[1][3] * self[3][1]
      + self[3][0] * self[1][1] * self[2][3]
      - self[3][0] * self[1][3] * self[2][1];

    inv[3][0] = -self[1][0] * self[2][1] * self[3][2]
      + self[1][0] * self[2][2] * self[3][1]
      + self[2][0] * self[1][1] * self[3][2]
      - self[2][0] * self[1][2] * self[3][1]
      - self[3][0] * self[1][1] * self[2][2]
      + self[3][0] * self[1][2] * self[2][1];

    let det = self[0][0] * inv[0][0]
      + self[0][1] * inv[1][0]
      + self[0][2] * inv[2][0]
      + self[0][3] * inv[3][0];

    // If determinant is zero, just return the identity matrix
    if det == 0.0 {
      for row in 0..4 {
        for col in 0..4 {
          self[row][col] = (row == col) as i32 as f32;
        }
      }
      return;
    }

    inv[0][1] = -self[0][1] * self[2][2] * self[3][3]
      + self[0][1] * self[2][3] * self[3][2]
      + self[2][1] * self[0][2] * self[3][3]
      - self[2][1] * self[0][3] * self[3][2]
      - self[3][1] * self[0][2] * self[2][3]
      + self[3][1] * self[0][3] * self[2][2];

    inv[1][1] = self[0][0] * self[2][2] * self[3][3]
      - self[0][0] * self[2][3] * self[3][2]
      - self[2][0] * self[0][2] * self[3][3]
      + self[2][0] * self[0][3] * self[3][2]
      + self[3][0] * self[0][2] * self[2][3]
      - self[3][0] * self[0][3] * self[2][2];

    inv[2][1] = -self[0][0] * self[2][1] * self[3][3]
      + self[0][0] * self[2][3] * self[3][1]
      + self[2][0] * self[0][1] * self[3][3]
      - self[2][0] * self[0][3] * self[3][1]
      - self[3][0] * self[0][1] * self[2][3]
      + self[3][0] * self[0][3] * self[2][1];

    inv[3][1] = self[0][0] * self[2][1] * self[3][2]
      - self[0][0] * self[2][2] * self[3][1]
      - self[2][0] * self[0][1] * self[3][2]
      + self[2][0] * self[0][2] * self[3][1]
      + self[3][0] * self[0][1] * self[2][2]
      - self[3][0] * self[0][2] * self[2][1];

    inv[0][2] = self[0][1] * self[1][2] * self[3][3]
      - self[0][1] * self[1][3] * self[3][2]
      - self[1][1] * self[0][2] * self[3][3]
      + self[1][1] * self[0][3] * self[3][2]
      + self[3][1] * self[0][2] * self[1][3]
      - self[3][1] * self[0][3] * self[1][2];

    inv[1][2] = -self[0][0] * self[1][2] * self[3][3]
      + self[0][0] * self[1][3] * self[3][2]
      + self[1][0] * self[0][2] * self[3][3]
      - self[1][0] * self[0][3] * self[3][2]
      - self[3][0] * self[0][2] * self[1][3]
      + self[3][0] * self[0][3] * self[1][2];

    inv[2][2] = self[0][0] * self[1][1] * self[3][3]
      - self[0][0] * self[1][3] * self[3][1]
      - self[1][0] * self[0][1] * self[3][3]
      + self[1][0] * self[0][3] * self[3][1]
      + self[3][0] * self[0][1] * self[1][3]
      - self[3][0] * self[0][3] * self[1][1];

    inv[3][2] = -self[0][0] * self[1][1] * self[3][2]
      + self[0][0] * self[1][2] * self[3][1]
      + self[1][0] * self[0][1] * self[3][2]
      - self[1][0] * self[0][2] * self[3][1]
      - self[3][0] * self[0][1] * self[1][2]
      + self[3][0] * self[0][2] * self[1][1];

    inv[0][3] = -self[0][1] * self[1][2] * self[2][3]
      + self[0][1] * self[1][3] * self[2][2]
      + self[1][1] * self[0][2] * self[2][3]
      - self[1][1] * self[0][3] * self[2][2]
      - self[2][1] * self[0][2] * self[1][3]
      + self[2][1] * self[0][3] * self[1][2];

    inv[1][3] = self[0][0] * self[1][2] * self[2][3]
      - self[0][0] * self[1][3] * self[2][2]
      - self[1][0] * self[0][2] * self[2][3]
      + self[1][0] * self[0][3] * self[2][2]
      + self[2][0] * self[0][2] * self[1][3]
      - self[2][0] * self[0][3] * self[1][2];

    inv[2][3] = -self[0][0] * self[1][1] * self[2][3]
      + self[0][0] * self[1][3] * self[2][1]
      + self[1][0] * self[0][1] * self[2][3]
      - self[1][0] * self[0][3] * self[2][1]
      - self[2][0] * self[0][1] * self[1][3]
      + self[2][0] * self[0][3] * self[1][1];

    inv[3][3] = self[0][0] * self[1][1] * self[2][2]
      - self[0][0] * self[1][2] * self[2][1]
      - self[1][0] * self[0][1] * self[2][2]
      + self[1][0] * self[0][2] * self[2][1]
      + self[2][0] * self[0][1] * self[1][2]
      - self[2][0] * self[0][2] * self[1][1];

    let inv_det = 1.0 / det;

    for r in 0..4 {
      for c in 0..4 {
        self[r][c] = inv[r][c] * inv_det;
      }
    }
  }
}

// Properties

impl Mat4 {
  /// Returns the determinant of the matrix.
  #[must_use]
  pub fn determinant(&self) -> f32 {
    let mat = self.as_array_ref();

    mat[0][0]
      * (mat[1][1] * mat[2][2] * mat[3][3]
        - mat[1][1] * mat[2][3] * mat[3][2]
        - mat[2][1] * mat[1][2] * mat[3][3]
        + mat[2][1] * mat[1][3] * mat[3][2]
        + mat[3][1] * mat[1][2] * mat[2][3]
        - mat[3][1] * mat[1][3] * mat[2][2])
      + mat[0][1]
        * (-mat[1][0] * mat[2][2] * mat[3][3]
          + mat[1][0] * mat[2][3] * mat[3][2]
          + mat[2][0] * mat[1][2] * mat[3][3]
          - mat[2][0] * mat[1][3] * mat[3][2]
          - mat[3][0] * mat[1][2] * mat[2][3]
          + mat[3][0] * mat[1][3] * mat[2][2])
      + mat[0][2]
        * (mat[1][0] * mat[2][1] * mat[3][3]
          - mat[1][0] * mat[2][3] * mat[3][1]
          - mat[2][0] * mat[1][1] * mat[3][3]
          + mat[2][0] * mat[1][3] * mat[3][1]
          + mat[3][0] * mat[1][1] * mat[2][3]
          - mat[3][0] * mat[1][3] * mat[2][1])
      + mat[0][3]
        * (-mat[1][0] * mat[2][1] * mat[3][2]
          + mat[1][0] * mat[2][2] * mat[3][1]
          + mat[2][0] * mat[1][1] * mat[3][2]
          - mat[2][0] * mat[1][2] * mat[3][1]
          - mat[3][0] * mat[1][1] * mat[2][2]
          + mat[3][0] * mat[1][2] * mat[2][1])
  }

  /// Returns the trace of the matrix.
  #[must_use]
  pub fn trace(&self) -> f32 {
    let mat = self.as_array_ref();

    mat[0][0] + mat[1][1] + mat[2][2] + mat[3][3]
  }

  /// Returns the [`transpose`] of the matrix.
  ///
  /// [`transpose`]: Mat4::transpose
  pub const fn transposed(&self) -> Matrix4 {
    const fn at(arr: &Mat4, r: usize, c: usize) -> f32 {
      unsafe { arr.get_unchecked(c, r) }
    }

    Matrix4::from_arrays([
      [
        at(self, 0, 0),
        at(self, 1, 0),
        at(self, 2, 0),
        at(self, 3, 0),
      ],
      [
        at(self, 0, 1),
        at(self, 1, 1),
        at(self, 2, 1),
        at(self, 3, 1),
      ],
      [
        at(self, 0, 2),
        at(self, 1, 2),
        at(self, 2, 2),
        at(self, 3, 2),
      ],
      [
        at(self, 0, 3),
        at(self, 1, 3),
        at(self, 2, 3),
        at(self, 3, 3),
      ],
    ])
  }

  /// Returns the inverse of the matrix.
  pub fn inverted(&self) -> Matrix4 {
    let mut result = self.to_matrix4();
    result.invert();
    result
  }
}

// Formatting

impl fmt::Debug for Mat4 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Mat4")
      .field("[0]", &self.row(0))
      .field("[1]", &self.row(1))
      .field("[2]", &self.row(2))
      .field("[3]", &self.row(3))
      .finish()
  }
}

impl fmt::Display for Mat4 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "[{}, {}, {}, {}]",
      self.row(0),
      self.row(1),
      self.row(2),
      self.row(3)
    )
  }
}

// Arithmetic Operations

impl Mat4 {
  /// Combines this matrix with another matrix by performing matrix multiplication.
  ///
  /// # Parameters
  ///
  /// * `other` - The matrix to combine with.
  pub fn mul_mat4(&self, other: &Mat4) -> Matrix4 {
    let mut result = Matrix4::ZERO;

    for r in 0..4 {
      // SAFETY: 0..4 is always within bounds.
      let row = unsafe { self.row_unchecked(r) };

      for c in 0..4 {
        // SAFETY: 0..4 is always within bounds.
        let col = unsafe { other.col_unchecked(c) };

        result[r][c] = row.dot(col.to_vector4().as_vec4());
      }
    }
    result
  }

  /// Combines this matrix with a column vector, returning the resultant vector.
  ///
  /// This implements matrix-vector of:
  ///
  /// ```raw
  /// | a b c d |   | x |   | ax + by + cz + dw |
  /// | e f g h | * | y | = | ex + fy + gz + hw |
  /// | i j k l |   | z |   | ix + jy + kz + lw |
  /// | m n o p |   | w |   | mx + ny + oz + pw |
  /// ```
  ///
  /// # Parameters
  ///
  /// * `vec` - The vector to combine with.
  pub fn mul_col_vec4(&self, vec: &Vec4) -> Vector4 {
    let mut result = Vector4::default();

    for r in 0..4 {
      // SAFETY: 0..4 is always within bounds.
      let row = unsafe { self.row_unchecked(r) };

      result[r] = row.dot(vec);
    }
    result
  }

  /// Combines this matrix with a row vector, returning the resultant vector.
  ///
  /// This implements vector-matrix multiplication of:
  /// ```raw
  /// | x y z w |   | a b c d |   | ax + ey + iz + mw |
  ///             * | e f g h | = | bx + fy + jz + nw |
  ///               | i j k l |   | cx + gy + kz + ow |
  ///               | m n o p |   | dx + hy + lz + pw |
  /// ```
  pub fn mul_row_vec4(&self, vec: &Vec4) -> Vector4 {
    let mut result = Vector4::default();

    for c in 0..4 {
      // SAFETY: 0..4 is always within bounds.
      let col = unsafe { self.col_unchecked(c) };

      result[c] = col.dot(vec);
    }
    result
  }

  /// A private helper function for the [`Add`] trait implementations.
  ///
  /// # Parameters
  ///
  /// * `lhs` - The left-hand side matrix.
  /// * `rhs` - The right-hand side matrix.
  fn add_impl(lhs: &Self, rhs: &Self) -> Matrix4 {
    let mut result = Matrix4::ZERO;

    for r in 0..4 {
      for c in 0..4 {
        result[r][c] = lhs[r][c] + rhs[r][c];
      }
    }

    result
  }

  /// A private helper function for the [`Sub`] trait implementations.
  ///
  /// # Parameters
  ///
  /// * `lhs` - The left-hand side matrix.
  /// * `rhs` - The right-hand side matrix.
  fn sub_impl(lhs: &Self, rhs: &Self) -> Matrix4 {
    let mut result = Matrix4::ZERO;

    for r in 0..4 {
      for c in 0..4 {
        result[r][c] = lhs[r][c] - rhs[r][c];
      }
    }

    result
  }

  /// A private helper function for the [`Mul`] trait implementations.
  ///
  /// # Parameters
  ///
  /// * `lhs` - The left-hand side matrix.
  /// * `rhs` - The right-hand side matrix.
  #[inline]
  fn mul_impl(lhs: &Self, rhs: &Self) -> Matrix4 {
    lhs.mul_mat4(rhs)
  }
}

impl Neg for &Mat4 {
  type Output = Matrix4;

  fn neg(self) -> Self::Output {
    let mut result = Matrix4::ZERO;

    for r in 0..4 {
      for c in 0..4 {
        result[r][c] = -self[r][c];
      }
    }

    result
  }
}

impl Add for &Mat4 {
  type Output = Matrix4;

  #[must_use]
  fn add(self, rhs: Self) -> Self::Output {
    let mut result = Matrix4::default();

    for r in 0..4 {
      for c in 0..4 {
        result[r][c] = rhs[r][c] + self[r][c];
      }
    }

    result
  }
}

impl Sub for &Mat4 {
  type Output = Matrix4;

  #[must_use]
  fn sub(self, rhs: Self) -> Self::Output {
    let mut result = Matrix4::default();

    for r in 0..4 {
      for c in 0..4 {
        result[r][c] = self[r][c] - rhs[r][c];
      }
    }

    result
  }
}

impl Mul for &Mat4 {
  type Output = Matrix4;

  #[must_use]
  fn mul(self, rhs: Self) -> Self::Output {
    self.mul_mat4(rhs)
  }
}

impl Mul<&Vec4> for &Mat4 {
  type Output = Vector4;

  #[must_use]
  fn mul(self, rhs: &Vec4) -> Self::Output {
    self.mul_col_vec4(rhs)
  }
}

impl Mul<f32> for &Mat4 {
  type Output = Matrix4;

  #[must_use]
  fn mul(self, rhs: f32) -> Self::Output {
    let mut result = Matrix4::default();

    for r in 0..4 {
      for c in 0..4 {
        result[r][c] = self[r][c] * rhs;
      }
    }

    result
  }
}

impl Mul<&Mat4> for f32 {
  type Output = Matrix4;

  #[must_use]
  #[inline(always)]
  fn mul(self, rhs: &Mat4) -> Self::Output {
    rhs * self
  }
}

impl Div<f32> for &Mat4 {
  type Output = Matrix4;

  #[must_use]
  fn div(self, rhs: f32) -> Self::Output {
    let mut result = Matrix4::default();

    let inverse = 1.0 / rhs;
    result.as_mut_mat4().mul_assign(inverse);

    result
  }
}

impl AddAssign<&Mat4> for Mat4 {
  fn add_assign(&mut self, rhs: &Mat4) {
    for r in 0..4 {
      for c in 0..4 {
        self[r][c] += rhs[r][c];
      }
    }
  }
}

impl SubAssign<&Mat4> for Mat4 {
  fn sub_assign(&mut self, rhs: &Mat4) {
    for r in 0..4 {
      for c in 0..4 {
        self[r][c] -= rhs[r][c];
      }
    }
  }
}

impl MulAssign<&Mat4> for Mat4 {
  fn mul_assign(&mut self, rhs: &Mat4) {
    for (i, v) in self.mul_mat4(rhs).as_slice().iter().enumerate() {
      self.0[i] = *v;
    }
  }
}

impl MulAssign<f32> for Mat4 {
  fn mul_assign(&mut self, rhs: f32) {
    for r in 0..4 {
      for c in 0..4 {
        self[r][c] *= rhs;
      }
    }
  }
}

impl DivAssign<f32> for Mat4 {
  fn div_assign(&mut self, rhs: f32) {
    let inverse = 1.0 / rhs;
    self.mul_assign(inverse);
  }
}

/// An owning representation of a 4x4 [matrix].
///
/// Like [`Mat4`], the [`Matrix4`] object represents a [matrix] in
/// 4D. Unlike the [`Mat4`], this is an owning representation that stores the
/// actual content of the vector.
///
/// [`Matrix4`] does not implement [`Copy`] to prevent accidental copying of
/// large amounts of data. If you need to copy the data, you can use the
/// [`Clone`] trait.
///
/// [matrix]: https://en.wikipedia.org/wiki/Matrix_(mathematics)
#[repr(C)]
#[repr(align(64))]
#[derive(Clone, Default, PartialEq, PartialOrd)]
pub struct Matrix4([[f32; 4]; 4]);

// Constructors

impl Matrix4 {
  /// The zero matrix.
  pub const ZERO: Self = Self([[0.0; 4]; 4]);

  /// The identity matrix.
  pub const IDENTITY: Self = Self([
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
  ]);

  /// Constructs a new 4x4 matrix with the identity matrix.
  #[must_use]
  #[inline(always)]
  pub const fn new() -> Self {
    Self::IDENTITY
  }

  /// Constructs a new 4x4 matrix as an identity matrix.
  #[must_use]
  #[inline(always)]
  pub const fn identity() -> Self {
    Self::IDENTITY
  }

  /// Constructs a new 4x4 matrix with the given value for the diagonal.
  ///
  /// # Parameters
  ///
  /// * `diagonal` - The value to set on the diagonal.
  #[must_use]
  pub const fn with_diagonal(diagonal: f32) -> Self {
    Self([
      [diagonal, 0.0, 0.0, 0.0],
      [0.0, diagonal, 0.0, 0.0],
      [0.0, 0.0, diagonal, 0.0],
      [0.0, 0.0, 0.0, diagonal],
    ])
  }

  /// Constructs a new 4x4 matrix from the 4x4 array of [`f32`].
  ///
  /// # Parameters
  ///
  /// * `data` - The 4x4 array of [`f32`] values.
  #[must_use]
  #[inline(always)]
  pub const fn from_arrays(data: [[f32; 4]; 4]) -> Self {
    Self(data)
  }

  /// Constructs a new 4x4 matrix from a slice of 16 [`f32`] values.
  ///
  /// This function will panic if the slice does not have a length of at least 16.
  ///
  /// # Parameters
  ///
  /// * `data` - The slice of 16 [`f32`] values.
  #[must_use]
  pub const fn from_slice(data: &[f32]) -> Self {
    Self([
      [data[0], data[1], data[2], data[3]],
      [data[4], data[5], data[6], data[7]],
      [data[8], data[9], data[10], data[11]],
      [data[12], data[13], data[14], data[15]],
    ])
  }

  /// Constructs a new 4x4 matrix from a slice of 4-component arrays of [`f32`].
  ///
  /// This function will panic if the slice does not have a length of at least 4.
  ///
  /// # Parameters
  ///
  /// * `data` - The slice of 4-component arrays of [`f32`] values.
  pub const fn from_array_slice(data: &[[f32; 4]]) -> Self {
    Self([
      [data[0][0], data[0][1], data[0][2], data[0][3]],
      [data[1][0], data[1][1], data[1][2], data[1][3]],
      [data[2][0], data[2][1], data[2][2], data[2][3]],
      [data[3][0], data[3][1], data[3][2], data[3][3]],
    ])
  }

  /// Constructs a new 4x4 matrix from a mutable slice of 16 [`f32`] values.
  ///
  /// # Parameters
  ///
  /// * `data` - The mutable pointer to a sequence of 16 [`f32`] values.
  ///
  /// # Safety
  ///
  /// The caller must ensure that the pointer is valid and points to a sequence
  /// of 16 [`f32`] reachable values.
  #[must_use]
  #[inline(always)]
  pub const unsafe fn from_ptr_unchecked(ptr: *const f32) -> Self {
    Self::from_slice(std::slice::from_raw_parts(ptr, 16))
  }

  /// Constructs a new 4x4 matrix with the given [`Mat4`].
  ///
  /// # Parameters
  ///
  /// * `mat` - The [`Mat4`] to copy the values from.
  #[must_use]
  pub const fn from_mat4(mat: &Mat4) -> Self {
    Self([
      [mat.0[0], mat.0[1], mat.0[2], mat.0[3]],
      [mat.0[4], mat.0[5], mat.0[6], mat.0[7]],
      [mat.0[8], mat.0[9], mat.0[10], mat.0[11]],
      [mat.0[12], mat.0[13], mat.0[14], mat.0[15]],
    ])
  }

  /// Creates a new 4x4 matrix from 4 row vectors.
  ///
  /// # Parameters
  ///
  /// * `r0` - The first row of the matrix.
  /// * `r1` - The second row of the matrix.
  /// * `r2` - The third row of the matrix.
  /// * `r3` - The fourth row of the matrix.
  #[must_use]
  pub const fn from_rows(r0: &Vec4, r1: &Vec4, r2: &Vec4, r3: &Vec4) -> Self {
    Self([
      [r0.x(), r0.y(), r0.z(), r0.w()],
      [r1.x(), r1.y(), r1.z(), r1.w()],
      [r2.x(), r2.y(), r2.z(), r2.w()],
      [r3.x(), r3.y(), r3.z(), r3.w()],
    ])
  }

  /// Creates a new 4x4 matrix from 4 column vectors.
  ///
  /// # Parameters
  ///
  /// * `c0` - The first column of the matrix.
  /// * `c1` - The second column of the matrix.
  /// * `c2` - The third column of the matrix.
  /// * `c3` - The fourth column of the matrix.
  #[must_use]
  pub const fn from_cols(c0: &Vec4, c1: &Vec4, c2: &Vec4, c3: &Vec4) -> Self {
    Self([
      [c0.x(), c1.x(), c2.x(), c3.x()],
      [c0.y(), c1.y(), c2.y(), c3.y()],
      [c0.z(), c1.z(), c2.z(), c3.z()],
      [c0.w(), c1.w(), c2.w(), c3.w()],
    ])
  }
}

// Conversions

impl Matrix4 {
  /// Returns the row at the given index.
  pub const fn as_mat4(&self) -> &Mat4 {
    Mat4::from_arrays(&self.0)
  }

  /// Returns the mutable [`Mat4`] reference.
  pub fn as_mut_mat4(&mut self) -> &mut Mat4 {
    Mat4::from_mut_array(&mut self.0)
  }
}

impl Deref for Matrix4 {
  type Target = Mat4;

  fn deref(&self) -> &Self::Target {
    Mat4::from_arrays(&self.0)
  }
}

impl DerefMut for Matrix4 {
  fn deref_mut(&mut self) -> &mut Self::Target {
    Mat4::from_mut_array(&mut self.0)
  }
}

impl Borrow<Mat4> for Matrix4 {
  #[must_use]
  #[inline(always)]
  fn borrow(&self) -> &Mat4 {
    self.as_mat4()
  }
}

impl BorrowMut<Mat4> for Matrix4 {
  #[must_use]
  #[inline(always)]
  fn borrow_mut(&mut self) -> &mut Mat4 {
    self.as_mut_mat4()
  }
}

impl AsRef<Mat4> for Matrix4 {
  #[must_use]
  #[inline(always)]
  fn as_ref(&self) -> &Mat4 {
    self.as_mat4()
  }
}

impl AsMut<Mat4> for Matrix4 {
  #[must_use]
  #[inline(always)]
  fn as_mut(&mut self) -> &mut Mat4 {
    self.as_mut_mat4()
  }
}

// Formatting

impl fmt::Debug for Matrix4 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Matrix4")
      .field("[0]", &self.row(0))
      .field("[1]", &self.row(1))
      .field("[2]", &self.row(2))
      .field("[3]", &self.row(3))
      .finish()
  }
}

impl fmt::Display for Matrix4 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "[{}, {}, {}, {}]",
      self.row(0),
      self.row(1),
      self.row(2),
      self.row(3)
    )
  }
}

// Arithmetic Operations

macro_rules! impl_op {
  ($trait:ident, $func:ident, $op:tt, $logic:expr) => {
    impl $trait for Matrix4 {
      type Output = Matrix4;

      #[must_use]
      #[inline(always)]
      fn $func(self, rhs: Self) -> Self::Output {
        $logic(self.as_mat4(), rhs.as_mat4())
      }
    }

    impl $trait for &Matrix4 {
      type Output = Matrix4;

      #[must_use]
      #[inline(always)]
      fn $func(self, rhs: Self) -> Self::Output {
        $logic(self.as_mat4(), rhs.as_mat4())
      }
    }

    impl $trait<Matrix4> for &Matrix4 {
      type Output = Matrix4;

      #[must_use]
      #[inline(always)]
      fn $func(self, rhs: Matrix4) -> Self::Output {
        $logic(self.as_mat4(), rhs.as_mat4())
      }
    }

    impl $trait<&Matrix4> for Matrix4 {
      type Output = Matrix4;

      #[must_use]
      #[inline(always)]
      fn $func(self, rhs: &Matrix4) -> Self::Output {
        $logic(self.as_mat4(), rhs.as_mat4())
      }
    }

    impl $trait<&Mat4> for Matrix4 {
      type Output = Matrix4;

      #[must_use]
      #[inline(always)]
      fn $func(self, rhs: &Mat4) -> Self::Output {
        $logic(self.as_mat4(), rhs)
      }
    }

    impl $trait<&Mat4> for &Matrix4 {
      type Output = Matrix4;

      #[must_use]
      #[inline(always)]
      fn $func(self, rhs: &Mat4) -> Self::Output {
        $logic(self.as_mat4(), rhs)
      }
    }

    impl $trait<Matrix4> for &Mat4 {
      type Output = Matrix4;

      #[must_use]
      #[inline(always)]
      fn $func(self, rhs: Matrix4) -> Self::Output {
        $logic(self, rhs.as_mat4())
      }
    }

    impl $trait<&Matrix4> for &Mat4 {
      type Output = Matrix4;

      #[must_use]
      #[inline(always)]
      fn $func(self, rhs: &Matrix4) -> Self::Output {
        $logic(self, rhs.as_mat4())
      }
    }
  };
}

macro_rules! impl_op_assign {
  ($trait:ident, $func:ident, $op:tt) => {
    impl $trait for Matrix4 {
      #[inline(always)]
      fn $func(&mut self, rhs: Self) {
        *self.as_mut_mat4() $op rhs.as_mat4();
      }
    }

    impl $trait<&Matrix4> for Matrix4 {
      #[inline(always)]
      fn $func(&mut self, rhs: &Self) {
        *self.as_mut_mat4() $op rhs.as_mat4();
      }
    }

    impl $trait<&Mat4> for Matrix4 {
      #[inline(always)]
      fn $func(&mut self, rhs: &Mat4) {
        *self.as_mut_mat4() $op rhs;
      }
    }

    impl $trait<&Matrix4> for Mat4 {
      #[inline(always)]
      fn $func(&mut self, rhs: &Matrix4) {
        *self $op rhs.as_mat4();
      }
    }

    impl $trait<Matrix4> for Mat4 {
      #[inline(always)]
      fn $func(&mut self, rhs: Matrix4) {
        *self $op rhs.as_mat4();
      }
    }
  };
}

impl_op!(Add, add, +, Mat4::add_impl);
impl_op!(Sub, sub, -, Mat4::sub_impl);
impl_op!(Mul, mul, *, Mat4::mul_impl);

impl Mul<f32> for Matrix4 {
  type Output = Matrix4;

  #[must_use]
  #[inline(always)]
  fn mul(self, rhs: f32) -> Self::Output {
    self.as_mat4().mul(rhs)
  }
}

impl Div<f32> for &Matrix4 {
  type Output = Matrix4;

  #[must_use]
  #[inline(always)]
  fn div(self, rhs: f32) -> Self::Output {
    self.as_mat4().div(rhs)
  }
}

impl Div<&Matrix4> for f32 {
  type Output = Matrix4;

  #[must_use]
  #[inline(always)]
  fn div(self, rhs: &Matrix4) -> Self::Output {
    rhs.as_mat4().div(self)
  }
}

impl_op_assign!(AddAssign, add_assign, +=);
impl_op_assign!(SubAssign, sub_assign, -=);
impl_op_assign!(MulAssign, mul_assign, *=);

impl MulAssign<f32> for Matrix4 {
  fn mul_assign(&mut self, rhs: f32) {
    self.as_mut_mat4().mul_assign(rhs);
  }
}

impl DivAssign<f32> for Matrix4 {
  fn div_assign(&mut self, rhs: f32) {
    self.as_mut_mat4().div_assign(rhs);
  }
}

#[cfg(test)]
#[path = "mat4.test.rs"]
mod test;
