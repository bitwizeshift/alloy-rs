use std::borrow::{Borrow, BorrowMut};
use std::fmt;
use std::ops::{
  Add, AddAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub,
  SubAssign,
};

use crate::core::hint;
use crate::math::vec::{Vec3, Vector3};
use crate::ops::Dot;

#[doc(inline)]
pub use crate::math::mat::col3::Col3;

/// A 3x3 non-owning view of a [matrix].
///
/// [matrix]: https://en.wikipedia.org/wiki/Matrix_(mathematics)
///
/// # Guarantees
///
/// This type has the strict requirement that it can only reference a
/// 3x3-component slice of [`f32`] values. It is guaranteed to never refer to
/// more or less than 2 entries.
///
/// This type is always in row-column order, meaning that the first index
/// is the row, and the second index is the column.
///
/// # Relation to [`Matrix3`]
///
/// This type is a non-owning view of a 3x3 matrix, and is used to provide
/// a way to reference a 3x3 matrix without taking ownership of the data.
#[repr(transparent)]
#[derive(PartialEq, PartialOrd)]
pub struct Mat3([f32]);

// Constructors

impl Mat3 {
  /// The number of columns this matrix contains
  pub const COLS: usize = 3;

  /// The number of rows this matrix contains.
  pub const ROWS: usize = 3;

  /// Creates a new 3x3 matrix view from a slice of 9 [`f32`] values.
  ///
  /// # Parameters
  ///
  /// * `data` - A slice of 9 [`f32`] values.
  #[must_use]
  #[inline(always)]
  pub const fn from_arrays(data: &[[f32; 3]; 3]) -> &Mat3 {
    // SAFETY: `[T]` is layout-identical to `[[T; N]; N]`
    unsafe { std::mem::transmute(std::slice::from_raw_parts(data.as_ptr().cast::<f32>(), 9)) }
  }

  /// Creates a new 3x3 matrix view from a mutable slice of 9 [`f32`] values.
  ///
  ///
  /// # Parameters
  ///
  /// * `data` - A slice of 9 [`f32`] values.
  #[must_use]
  #[inline(always)]
  pub fn from_mut_array(data: &mut [[f32; 3]; 3]) -> &mut Mat3 {
    // SAFETY: `[T]` is layout-identical to `[[T; N]; N]`
    unsafe {
      std::mem::transmute(std::slice::from_raw_parts_mut(
        data.as_mut_ptr().cast::<f32>(),
        9,
      ))
    }
  }

  /// Creates a new 3x3 matrix view from a slice of 9 [`f32`] values.
  /// If the length of the slice is not 9, this function will return [`None`].
  ///
  /// # Parameters
  ///
  /// * `data` - A slice of 9 [`f32`] values.
  pub const fn from_array_slice(data: &[[f32; 3]]) -> Option<&Mat3> {
    if data.len() != 3 {
      None
    } else {
      // SAFETY: The above check ensures that the length of the slice is 3.
      Some(unsafe { Self::from_array_slice_unchecked(data) })
    }
  }

  /// Creates a new 3x3 matrix view from a mutable slice of 9 [`f32`] values.
  /// If the length of the slice is not 9, this function will return [`None`].
  ///
  /// # Parameters
  ///
  /// * `data` - A slice of 9 [`f32`] values.
  pub fn from_mut_array_slice(data: &mut [[f32; 3]]) -> Option<&mut Mat3> {
    if data.len() != 3 {
      None
    } else {
      Some(unsafe { Self::from_mut_array_slice_unchecked(data) })
    }
  }

  /// Creates a new 3x3 matrix view from a slice of 9 [`f32`] values without
  /// performing bounds checking.
  ///
  /// # Safety
  ///
  /// This function is unsafe because it does not check that the given slice
  /// has a length of 9. If this constraint is violated, memory safety errors
  /// can occur.
  ///
  /// # Parameters
  ///
  /// * `data` - A slice of 9 [`f32`] values.
  #[must_use]
  #[inline(always)]
  pub const unsafe fn from_array_slice_unchecked(data: &[[f32; 3]]) -> &Mat3 {
    let len = data.len() * 3;

    // SAFETY: `[T]` is layout-identical to `[[T; N]; N]`
    unsafe { std::mem::transmute(std::slice::from_raw_parts(data.as_ptr().cast::<f32>(), len)) }
  }

  /// Creates a new 3x3 matrix view from a mutable slice of 9 [`f32`] values
  /// without performing bounds checking.
  ///
  /// # Safety
  ///
  /// This function is unsafe because it does not check that the given slice
  /// has a length of 9. If this constraint is violated, memory safety errors
  /// can occur.
  ///
  /// # Parameters
  ///
  /// * `data` - A slice of 9 [`f32`] values.
  #[must_use]
  #[inline(always)]
  pub unsafe fn from_mut_array_slice_unchecked(data: &mut [[f32; 3]]) -> &mut Mat3 {
    let len = data.len() * 3;

    // SAFETY: `[T]` is layout-identical to `[[T; N]; N]`
    unsafe {
      std::mem::transmute(std::slice::from_raw_parts_mut(
        data.as_mut_ptr().cast::<f32>(),
        len,
      ))
    }
  }

  /// Creates a new 3x3 matrix view from a slice of at least 9 [`f32`] values.
  /// If the length of the slice is less than 9, this function will return
  /// [`None`].
  ///
  /// # Parameters
  ///
  /// * `data` - A slice of at least 9 [`f32`] values.
  #[must_use]
  pub const fn from_slice(data: &[f32]) -> Option<&Mat3> {
    if data.len() != 9 {
      None
    } else {
      // SAFETY: The above check ensures that the length of the slice is 9.
      Some(unsafe { Self::from_slice_unchecked(data) })
    }
  }

  /// Creates a new 3x3 matrix view from a mutable slice of at least 9 [`f32`]
  /// values. If the length of the slice is less than 9, this function will
  /// return [`None`].
  ///
  /// # Parameters
  ///
  /// * `data` - A slice of at least 9 [`f32`] values.
  #[must_use]
  pub fn from_mut_slice(data: &mut [f32]) -> Option<&mut Mat3> {
    if data.len() != 9 {
      None
    } else {
      Some(unsafe { Self::from_mut_slice_unchecked(data) })
    }
  }

  /// Forms a reference to a [`Mat3`] from a flat slice of at least 9 [`f32`]
  /// values.
  ///
  /// # Safety
  ///
  /// The caller must guarantee that the slice has a length of 9.
  ///
  /// # Parameters
  ///
  /// * `data` - the slice of [`f32`] values
  #[must_use]
  #[inline(always)]
  pub const unsafe fn from_slice_unchecked(data: &[f32]) -> &Mat3 {
    // SAFETY: The caller must guarantee that the slice has a length of 9.
    std::mem::transmute(data)
  }

  /// Forms a mutable reference to a [`Mat3`] from a flat slice of at least 9
  /// [`f32`] values.
  ///
  /// # Safety
  ///
  /// The caller must guarantee that the slice has a length of 9.
  ///
  /// # Parameters
  ///
  /// * `data` - the slice of [`f32`] values
  #[must_use]
  #[inline(always)]
  pub unsafe fn from_mut_slice_unchecked(data: &mut [f32]) -> &mut Mat3 {
    // SAFETY: The caller must guarantee that the slice has a length of 9.
    std::mem::transmute(data)
  }

  /// Forms a reference to a [`Mat3`] from a pointer to a contiguous sequence
  /// of at least 9 [`f32`]s.
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
  pub const unsafe fn from_ptr_unchecked<'a>(ptr: *const f32) -> &'a Mat3 {
    Mat3::from_slice_unchecked(std::slice::from_raw_parts(ptr, 9))
  }

  /// Forms a mutable reference to a [`Mat3`] from a pointer to a contiguous
  /// sequence of at least 9 [`f32`]s.
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
  pub unsafe fn from_mut_ptr_unchecked<'a>(ptr: *mut f32) -> &'a mut Vec3 {
    Vec3::from_mut_slice_unchecked(std::slice::from_raw_parts_mut(ptr, 9))
  }
}

// Conversions

impl Mat3 {
  /// Returns this [`Mat3`] as a slice of [`f32`].
  #[must_use]
  #[inline(always)]
  pub const fn as_slice(&self) -> &[f32] {
    &self.0
  }

  /// Returns this [`Mat3`] as a mutable slice of [`f32`].
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

  /// Returns this [`Mat3`] as a slice of 3-component arrays of [`f32`].
  pub const fn as_array_slice(&self) -> &[[f32; 3]] {
    // SAFETY: The length of the slice is guaranteed to be 9.
    unsafe {
      std::mem::transmute(std::slice::from_raw_parts(
        self.0.as_ptr().cast::<[f32; 3]>(),
        3,
      ))
    }
  }

  /// Returns this [`Mat3`] as a mutable slice of 3-component arrays of [`f32`].
  pub fn as_mut_array_slice(&mut self) -> &mut [[f32; 3]] {
    // SAFETY: The length of the slice is guaranteed to be 9.
    unsafe {
      std::mem::transmute(std::slice::from_raw_parts_mut(
        self.0.as_mut_ptr().cast::<[f32; 3]>(),
        3,
      ))
    }
  }

  /// Returns this [`Mat3`] as a reference to a 3x3 array of [`f32`].
  pub const fn as_array_ref(&self) -> &[[f32; 3]; 3] {
    // SAFETY: The length of the slice is guaranteed to be 9.
    unsafe { std::mem::transmute(&*self.0.as_ptr()) }
  }

  /// Returns this [`Mat3`] as a mutable reference to a 3x3 array of [`f32`].
  pub fn as_mut_array_ref(&mut self) -> &mut [[f32; 3]; 3] {
    // SAFETY: The length of the slice is guaranteed to be 9.
    unsafe { std::mem::transmute(&mut *self.0.as_mut_ptr()) }
  }

  /// Returns this [`Mat3`] as a [`Matrix3`].
  pub const fn to_matrix3(&self) -> Matrix3 {
    Matrix3::from_mat3(self)
  }
}

impl ToOwned for Mat3 {
  type Owned = Matrix3;

  fn to_owned(&self) -> Self::Owned {
    let mut result = Matrix3::ZERO;

    for i in 0..3 {
      for j in 0..3 {
        result[i][j] = self[i][j];
      }
    }

    result
  }
}

// Accessors

impl Mat3 {
  /// Returns the row at the given index.
  ///
  /// This function will panic if the given index is out of bounds.
  ///
  /// # Parameters
  ///
  /// * `index` - The index of the row to return.
  #[must_use]
  #[inline(always)]
  pub const fn row(&self, index: usize) -> &Vec3 {
    if hint::unlikely(index >= 3) {
      panic!("index out of bounds")
    } else {
      // SAFETY: The above check ensures that the index is in bounds.
      unsafe { self.row_unchecked(index) }
    }
  }

  /// Returns the row at the given index without performing bounds checking.
  ///
  /// # Safety
  ///
  /// This function is unsafe because it does not check that the given index is
  /// less than 3. If this constraint is violated, memory safety errors can occur.
  ///
  /// # Parameters
  ///
  /// * `index` - The index of the row to return.
  #[must_use]
  #[inline(always)]
  pub const unsafe fn row_unchecked(&self, index: usize) -> &Vec3 {
    // SAFETY: The caller must guarantee that the index is in bounds.
    unsafe { Vec3::from_slice_unchecked(&self.as_array_ref()[index]) }
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
  pub fn mut_row(&mut self, index: usize) -> &mut Vec3 {
    if hint::unlikely(index >= 3) {
      panic!("index out of bounds")
    } else {
      // SAFETY: The above check ensures that the index is in bounds.
      unsafe { self.mut_row_unchecked(index) }
    }
  }

  /// Returns a mutable reference to the row at the given index without
  /// performing bounds checking.
  ///
  /// # Safety
  ///
  /// This function is unsafe because it does not check that the given index is
  /// less than 3. If this constraint is violated, memory safety errors can
  /// occur.
  ///
  /// # Parameters
  ///
  /// * `index` - The index of the row to return.
  #[must_use]
  #[inline(always)]
  pub unsafe fn mut_row_unchecked(&mut self, index: usize) -> &mut Vec3 {
    Vec3::from_mut_slice_unchecked(&mut self.as_mut_array_slice()[index])
  }

  /// Returns the column at the given index.
  ///
  /// This function will panic if the given index is out of bounds.
  ///
  /// # Parameters
  ///
  /// * `index` - The index of the column to return.
  #[must_use]
  pub const fn col(&self, index: usize) -> &Col3 {
    if hint::unlikely(index >= 3) {
      panic!("index out of bounds")
    } else {
      // SAFETY: The above check ensures that the index is in bounds.
      unsafe { self.col_unchecked(index) }
    }
  }

  /// Returns the column at the given index without performing bounds checking.
  ///
  /// **Note:** Unlike [`Mat3::row_unchecked`], this function returns an owning
  /// [`Vector3`] instead of a reference of [`Vec3`].
  ///
  /// # Safety
  ///
  /// This function is unsafe because it does not check that the given index is
  /// less than 3. If this constraint is violated, memory safety errors can occur.
  ///
  /// # Parameters
  ///
  /// * `index` - The index of the column to return.
  #[must_use]
  pub const unsafe fn col_unchecked(&self, index: usize) -> &Col3 {
    let ptr = self.as_ptr().add(index);
    let len = self.0.len() - index;
    Col3::from_raw_parts(ptr, len)
  }

  /// Returns a mutable reference to the column at the given index.
  ///
  /// This function will panic if the given index is out of bounds.
  ///
  /// # Parameters
  ///
  /// * `index` - The index of the column to return.
  #[must_use]
  pub fn mut_col(&mut self, index: usize) -> &mut Col3 {
    if hint::unlikely(index >= 3) {
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
  /// less than 3. If this constraint is violated, memory safety errors can occur.
  ///
  /// # Parameters
  ///
  /// * `index` - The index of the column to return.
  #[must_use]
  pub unsafe fn mut_col_unchecked(&mut self, index: usize) -> &mut Col3 {
    let ptr = self.as_mut_ptr().add(index);
    let len = self.0.len() - index;
    Col3::from_raw_parts_mut(ptr, len)
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
    if hint::unlikely(row >= 3 || col >= 3) {
      panic!("index out of bounds")
    } else {
      // SAFETY: bounds are checked above
      unsafe { self.get_unchecked(row, col) }
    }
  }

  /// Returns the value at the given column and row without performing bounds
  /// checking.
  ///
  /// # Safety
  ///
  /// This function is unsafe because it does not check that the given index is
  /// less than 3. If this constraint is violated, memory safety errors can occur.
  ///
  /// # Parameters
  ///
  /// * `row` - The index of the row to return.
  /// * `col` - The index of the column to return.
  #[must_use]
  #[inline(always)]
  pub const unsafe fn get_unchecked(&self, row: usize, col: usize) -> f32 {
    self.as_array_ref()[row][col]
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
    if hint::unlikely(row >= 3 || col >= 3) {
      panic!("index out of bounds")
    } else {
      // SAFETY: bounds are checked above
      unsafe { self.get_mut_unchecked(row, col) }
    }
  }

  /// Returns a mutable reference to the value at the given column and row
  /// without performing bounds checking.
  ///
  /// # Safety
  ///
  /// This function is unsafe because it does not check that the given index is
  /// less than 3. If this constraint is violated, memory safety errors can occur.
  ///
  /// # Parameters
  ///
  /// * `row` - The index of the row to return.
  /// * `col` - The index of the column to return.
  #[must_use]
  #[inline(always)]
  pub unsafe fn get_mut_unchecked(&mut self, row: usize, col: usize) -> &mut f32 {
    &mut self.as_mut_array_slice()[row][col]
  }
}

impl Index<(usize, usize)> for Mat3 {
  type Output = f32;

  fn index(&self, (row, col): (usize, usize)) -> &f32 {
    &self.0[row * 3 + col]
  }
}

impl IndexMut<(usize, usize)> for Mat3 {
  fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut f32 {
    &mut self.0[row * 3 + col]
  }
}

impl Index<usize> for Mat3 {
  type Output = Vec3;

  fn index(&self, index: usize) -> &Vec3 {
    self.row(index)
  }
}

impl IndexMut<usize> for Mat3 {
  fn index_mut(&mut self, index: usize) -> &mut Vec3 {
    self.mut_row(index)
  }
}

// Modifiers

impl Mat3 {
  /// Scales each component of the matrix by the given scalar.
  ///
  /// # Parameters
  ///
  /// * `scale` - The scalar to multiply each component by.
  pub fn scale(&mut self, scale: f32) {
    for row in 0..3 {
      for col in 0..3 {
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
  pub fn scale_vec(&mut self, scale: &Vec3) {
    for row in 0..3 {
      for col in 0..3 {
        self[row][col] *= scale[row];
      }
    }
  }

  /// Transposes all the elements of this matrix, swapping rows and columns.
  pub fn transpose(&mut self) {
    for row in 0..3 {
      for col in row + 1..3 {
        (self[row][col], self[col][row]) = (self[col][row], self[row][col]);
      }
    }
  }

  /// Inverts this matrix.
  pub fn invert(&mut self) {
    let mut inv = Matrix3::default(); // The resultant matrix

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
      for row in 0..3 {
        for col in 0..3 {
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

    for r in 0..3 {
      for c in 0..3 {
        self[r][c] = inv[r][c] * inv_det;
      }
    }
  }
}

// Properties

impl Mat3 {
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
  /// [`transpose`]: Matrix3::transpose
  pub fn transposed(&self) -> Matrix3 {
    let mut result = self.to_owned();
    result.transpose();
    result
  }

  /// Returns the inverse of the matrix.
  pub fn inverted(&self) -> Matrix3 {
    let mut result = self.to_owned();
    result.invert();
    result
  }
}

// Formatting

impl fmt::Debug for Mat3 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Mat3")
      .field("[0]", &self.row(0))
      .field("[1]", &self.row(1))
      .field("[2]", &self.row(2))
      .field("[3]", &self.row(3))
      .finish()
  }
}

impl fmt::Display for Mat3 {
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

impl Mat3 {
  /// Combines this matrix with another matrix by performing matrix multiplication.
  ///
  /// # Parameters
  ///
  /// * `other` - The matrix to combine with.
  pub fn mul_mat3(&self, other: &Mat3) -> Matrix3 {
    let mut result = Matrix3::ZERO;

    for r in 0..3 {
      // SAFETY: 0..3 is always within bounds.
      let row = unsafe { self.row_unchecked(r) };

      for c in 0..3 {
        // SAFETY: 0..3 is always within bounds.
        let col = unsafe { other.col_unchecked(c) };

        result[r][c] = row.dot(col.to_vector3().as_vec3());
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
  pub fn mul_col_vec3(&self, vec: &Vec3) -> Vector3 {
    let mut result = Vector3::default();

    for r in 0..3 {
      // SAFETY: 0..3 is always within bounds.
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
  pub fn mul_row_vec3(&self, vec: &Vec3) -> Vector3 {
    let mut result = Vector3::default();

    for c in 0..3 {
      // SAFETY: 0..3 is always within bounds.
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
  fn add_impl(lhs: &Self, rhs: &Self) -> Matrix3 {
    let mut result = Matrix3::ZERO;

    for r in 0..3 {
      for c in 0..3 {
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
  fn sub_impl(lhs: &Self, rhs: &Self) -> Matrix3 {
    let mut result = Matrix3::ZERO;

    for r in 0..3 {
      for c in 0..3 {
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
  fn mul_impl(lhs: &Self, rhs: &Self) -> Matrix3 {
    lhs.mul_mat3(rhs)
  }
}

impl Neg for &Mat3 {
  type Output = Matrix3;

  fn neg(self) -> Self::Output {
    let mut result = Matrix3::ZERO;

    for r in 0..3 {
      for c in 0..3 {
        result[r][c] = -self[r][c];
      }
    }

    result
  }
}

impl Add for &Mat3 {
  type Output = Matrix3;

  #[must_use]
  fn add(self, rhs: Self) -> Self::Output {
    let mut result = Matrix3::default();

    for r in 0..3 {
      for c in 0..3 {
        result[r][c] = rhs[r][c] + self[r][c];
      }
    }

    result
  }
}

impl Sub for &Mat3 {
  type Output = Matrix3;

  #[must_use]
  fn sub(self, rhs: Self) -> Self::Output {
    let mut result = Matrix3::default();

    for r in 0..3 {
      for c in 0..3 {
        result[r][c] = self[r][c] - rhs[r][c];
      }
    }

    result
  }
}

impl Mul for &Mat3 {
  type Output = Matrix3;

  #[must_use]
  fn mul(self, rhs: Self) -> Self::Output {
    self.mul_mat3(rhs)
  }
}

impl Mul<&Vec3> for &Mat3 {
  type Output = Vector3;

  #[must_use]
  fn mul(self, rhs: &Vec3) -> Self::Output {
    self.mul_col_vec3(rhs)
  }
}

impl Mul<f32> for &Mat3 {
  type Output = Matrix3;

  #[must_use]
  fn mul(self, rhs: f32) -> Self::Output {
    let mut result = Matrix3::default();

    for r in 0..3 {
      for c in 0..3 {
        result[r][c] = self[r][c] * rhs;
      }
    }

    result
  }
}

impl Mul<&Mat3> for f32 {
  type Output = Matrix3;

  #[must_use]
  #[inline(always)]
  fn mul(self, rhs: &Mat3) -> Self::Output {
    rhs * self
  }
}

impl Div<f32> for &Mat3 {
  type Output = Matrix3;

  #[must_use]
  fn div(self, rhs: f32) -> Self::Output {
    let mut result = Matrix3::default();

    let inverse = 1.0 / rhs;
    result.as_mut_mat3().mul_assign(inverse);

    result
  }
}

impl AddAssign<&Mat3> for Mat3 {
  fn add_assign(&mut self, rhs: &Mat3) {
    for r in 0..3 {
      for c in 0..3 {
        self[r][c] += rhs[r][c];
      }
    }
  }
}

impl SubAssign<&Mat3> for Mat3 {
  fn sub_assign(&mut self, rhs: &Mat3) {
    for r in 0..3 {
      for c in 0..3 {
        self[r][c] -= rhs[r][c];
      }
    }
  }
}

impl MulAssign<&Mat3> for Mat3 {
  fn mul_assign(&mut self, rhs: &Mat3) {
    for (i, v) in self.mul_mat3(rhs).as_slice().iter().enumerate() {
      self.0[i] = *v;
    }
  }
}

impl MulAssign<f32> for Mat3 {
  fn mul_assign(&mut self, rhs: f32) {
    for r in 0..3 {
      for c in 0..3 {
        self[r][c] *= rhs;
      }
    }
  }
}

impl DivAssign<f32> for Mat3 {
  fn div_assign(&mut self, rhs: f32) {
    let inverse = 1.0 / rhs;
    self.mul_assign(inverse);
  }
}

/// An owning representation of a 3x3 [matrix].
///
/// Like [`Mat3`], the [`Matrix3`] object represents a [matrix] in
/// 3D. Unlike the [`Mat3`], this is an owning representation that stores the
/// actual content of the vector.
///
/// [`Matrix3`] does not implement [`Copy`] to prevent accidental copying of
/// large amounts of data. If you need to copy the data, you can use the
/// [`Clone`] trait.
///
/// [matrix]: https://en.wikipedia.org/wiki/Matrix_(mathematics)
#[repr(C)]
#[derive(Clone, Default, PartialEq, PartialOrd)]
pub struct Matrix3([[f32; 3]; 3]);

// Constructors

impl Matrix3 {
  /// The zero matrix.
  pub const ZERO: Self = Self([[0.0; 3]; 3]);

  /// The identity matrix.
  pub const IDENTITY: Self = Self([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);

  /// Constructs a new 3x3 matrix with the identity matrix.
  #[must_use]
  #[inline(always)]
  pub const fn new() -> Self {
    Self::IDENTITY
  }

  /// Constructs a new 3x3 matrix as an identity matrix.
  #[must_use]
  #[inline(always)]
  pub const fn identity() -> Self {
    Self::IDENTITY
  }

  /// Constructs a new 3x3 matrix with the given value for the diagonal.
  ///
  /// # Parameters
  ///
  /// * `diagonal` - The value to set on the diagonal.
  #[must_use]
  pub const fn with_diagonal(diagonal: f32) -> Self {
    Self([
      [diagonal, 0.0, 0.0],
      [0.0, diagonal, 0.0],
      [0.0, 0.0, diagonal],
    ])
  }

  /// Constructs a new 3x3 matrix from the 3x3 array of [`f32`].
  ///
  /// # Parameters
  ///
  /// * `data` - The 3x3 array of [`f32`] values.
  #[must_use]
  #[inline(always)]
  pub const fn from_arrays(data: [[f32; 3]; 3]) -> Self {
    Self(data)
  }

  /// Constructs a new 3x3 matrix from a slice of 9 [`f32`] values.
  ///
  /// This function will panic if the slice does not have a length of at least 9.
  ///
  /// # Parameters
  ///
  /// * `data` - The slice of 9 [`f32`] values.
  #[must_use]
  pub const fn from_slice(data: &[f32]) -> Self {
    Self([
      [data[0], data[1], data[2]],
      [data[3], data[4], data[5]],
      [data[6], data[7], data[8]],
    ])
  }

  /// Constructs a new 3x3 matrix from a slice of 3-component arrays of [`f32`].
  ///
  /// This function will panic if the slice does not have a length of at least 3.
  ///
  /// # Parameters
  ///
  /// * `data` - The slice of 3-component arrays of [`f32`] values.
  pub const fn from_array_slice(data: &[[f32; 3]]) -> Self {
    Self([
      [data[0][0], data[0][1], data[0][2]],
      [data[1][0], data[1][1], data[1][2]],
      [data[2][0], data[2][1], data[2][2]],
    ])
  }

  /// Constructs a new 3x3 matrix from a mutable slice of 9 [`f32`] values.
  ///
  /// # Parameters
  ///
  /// * `data` - The mutable pointer to a sequence of 9 [`f32`] values.
  ///
  /// # Safety
  ///
  /// The caller must ensure that the pointer is valid and points to a sequence
  /// of 9 [`f32`] reachable values.
  #[must_use]
  #[inline(always)]
  pub const unsafe fn from_ptr_unchecked(ptr: *const f32) -> Self {
    Self::from_slice(std::slice::from_raw_parts(ptr, 9))
  }

  /// Constructs a new 3x3 matrix with the given [`Mat3`].
  ///
  /// # Parameters
  ///
  /// * `mat` - The [`Mat3`] to copy the values from.
  #[must_use]
  pub const fn from_mat3(mat: &Mat3) -> Self {
    Self([
      [mat.0[0], mat.0[1], mat.0[2]],
      [mat.0[3], mat.0[4], mat.0[5]],
      [mat.0[6], mat.0[7], mat.0[8]],
    ])
  }

  /// Creates a new 3x3 matrix from 3 row vectors.
  ///
  /// # Parameters
  ///
  /// * `r0` - The first row of the matrix.
  /// * `r1` - The second row of the matrix.
  /// * `r2` - The third row of the matrix.
  #[must_use]
  pub const fn from_rows(r0: &Vec3, r1: &Vec3, r2: &Vec3) -> Self {
    Self([
      [r0.x(), r0.y(), r0.z()],
      [r1.x(), r1.y(), r1.z()],
      [r2.x(), r2.y(), r2.z()],
    ])
  }

  /// Creates a new 3x3 matrix from 3 column vectors.
  ///
  /// # Parameters
  ///
  /// * `c0` - The first column of the matrix.
  /// * `c1` - The second column of the matrix.
  /// * `c2` - The third column of the matrix.
  #[must_use]
  pub const fn from_cols(c0: &Vec3, c1: &Vec3, c2: &Vec3) -> Self {
    Self([
      [c0.x(), c1.x(), c2.x()],
      [c0.y(), c1.y(), c2.y()],
      [c0.z(), c1.z(), c2.z()],
    ])
  }
}

// Conversions

impl Matrix3 {
  /// Returns the row at the given index.
  pub const fn as_mat3(&self) -> &Mat3 {
    Mat3::from_arrays(&self.0)
  }

  /// Returns the mutable [`Mat3`] reference.
  pub fn as_mut_mat3(&mut self) -> &mut Mat3 {
    Mat3::from_mut_array(&mut self.0)
  }
}

impl Deref for Matrix3 {
  type Target = Mat3;

  fn deref(&self) -> &Self::Target {
    Mat3::from_arrays(&self.0)
  }
}

impl DerefMut for Matrix3 {
  fn deref_mut(&mut self) -> &mut Self::Target {
    Mat3::from_mut_array(&mut self.0)
  }
}

impl Borrow<Mat3> for Matrix3 {
  #[must_use]
  #[inline(always)]
  fn borrow(&self) -> &Mat3 {
    self.as_mat3()
  }
}

impl BorrowMut<Mat3> for Matrix3 {
  #[must_use]
  #[inline(always)]
  fn borrow_mut(&mut self) -> &mut Mat3 {
    self.as_mut_mat3()
  }
}

impl AsRef<Mat3> for Matrix3 {
  #[must_use]
  #[inline(always)]
  fn as_ref(&self) -> &Mat3 {
    self.as_mat3()
  }
}

impl AsMut<Mat3> for Matrix3 {
  #[must_use]
  #[inline(always)]
  fn as_mut(&mut self) -> &mut Mat3 {
    self.as_mut_mat3()
  }
}

// Formatting

impl fmt::Debug for Matrix3 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Matrix3")
      .field("[0]", &self.row(0))
      .field("[1]", &self.row(1))
      .field("[2]", &self.row(2))
      .field("[3]", &self.row(3))
      .finish()
  }
}

impl fmt::Display for Matrix3 {
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
    impl $trait for Matrix3 {
      type Output = Matrix3;

      #[must_use]
      #[inline(always)]
      fn $func(self, rhs: Self) -> Self::Output {
        $logic(self.as_mat3(), rhs.as_mat3())
      }
    }

    impl $trait for &Matrix3 {
      type Output = Matrix3;

      #[must_use]
      #[inline(always)]
      fn $func(self, rhs: Self) -> Self::Output {
        $logic(self.as_mat3(), rhs.as_mat3())
      }
    }

    impl $trait<Matrix3> for &Matrix3 {
      type Output = Matrix3;

      #[must_use]
      #[inline(always)]
      fn $func(self, rhs: Matrix3) -> Self::Output {
        $logic(self.as_mat3(), rhs.as_mat3())
      }
    }

    impl $trait<&Matrix3> for Matrix3 {
      type Output = Matrix3;

      #[must_use]
      #[inline(always)]
      fn $func(self, rhs: &Matrix3) -> Self::Output {
        $logic(self.as_mat3(), rhs.as_mat3())
      }
    }

    impl $trait<&Mat3> for Matrix3 {
      type Output = Matrix3;

      #[must_use]
      #[inline(always)]
      fn $func(self, rhs: &Mat3) -> Self::Output {
        $logic(self.as_mat3(), rhs)
      }
    }

    impl $trait<&Mat3> for &Matrix3 {
      type Output = Matrix3;

      #[must_use]
      #[inline(always)]
      fn $func(self, rhs: &Mat3) -> Self::Output {
        $logic(self.as_mat3(), rhs)
      }
    }

    impl $trait<Matrix3> for &Mat3 {
      type Output = Matrix3;

      #[must_use]
      #[inline(always)]
      fn $func(self, rhs: Matrix3) -> Self::Output {
        $logic(self, rhs.as_mat3())
      }
    }

    impl $trait<&Matrix3> for &Mat3 {
      type Output = Matrix3;

      #[must_use]
      #[inline(always)]
      fn $func(self, rhs: &Matrix3) -> Self::Output {
        $logic(self, rhs.as_mat3())
      }
    }
  };
}

macro_rules! impl_op_assign {
  ($trait:ident, $func:ident, $op:tt) => {
    impl $trait for Matrix3 {
      #[inline(always)]
      fn $func(&mut self, rhs: Self) {
        *self.as_mut_mat3() $op rhs.as_mat3();
      }
    }

    impl $trait<&Matrix3> for Matrix3 {
      #[inline(always)]
      fn $func(&mut self, rhs: &Self) {
        *self.as_mut_mat3() $op rhs.as_mat3();
      }
    }

    impl $trait<&Mat3> for Matrix3 {
      #[inline(always)]
      fn $func(&mut self, rhs: &Mat3) {
        *self.as_mut_mat3() $op rhs;
      }
    }

    impl $trait<&Matrix3> for Mat3 {
      #[inline(always)]
      fn $func(&mut self, rhs: &Matrix3) {
        *self $op rhs.as_mat3();
      }
    }

    impl $trait<Matrix3> for Mat3 {
      #[inline(always)]
      fn $func(&mut self, rhs: Matrix3) {
        *self $op rhs.as_mat3();
      }
    }
  };
}

impl_op!(Add, add, +, Mat3::add_impl);
impl_op!(Sub, sub, -, Mat3::sub_impl);
impl_op!(Mul, mul, *, Mat3::mul_impl);

impl Mul<f32> for Matrix3 {
  type Output = Matrix3;

  #[must_use]
  #[inline(always)]
  fn mul(self, rhs: f32) -> Self::Output {
    self.as_mat3().mul(rhs)
  }
}

impl Div<f32> for &Matrix3 {
  type Output = Matrix3;

  #[must_use]
  #[inline(always)]
  fn div(self, rhs: f32) -> Self::Output {
    self.as_mat3().div(rhs)
  }
}

impl Div<&Matrix3> for f32 {
  type Output = Matrix3;

  #[must_use]
  #[inline(always)]
  fn div(self, rhs: &Matrix3) -> Self::Output {
    rhs.as_mat3().div(self)
  }
}

impl_op_assign!(AddAssign, add_assign, +=);
impl_op_assign!(SubAssign, sub_assign, -=);
impl_op_assign!(MulAssign, mul_assign, *=);

impl MulAssign<f32> for Matrix3 {
  fn mul_assign(&mut self, rhs: f32) {
    self.as_mut_mat3().mul_assign(rhs);
  }
}

impl DivAssign<f32> for Matrix3 {
  fn div_assign(&mut self, rhs: f32) {
    self.as_mut_mat3().div_assign(rhs);
  }
}

#[cfg(test)]
#[path = "mat3.test.rs"]
mod test;
