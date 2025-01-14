use super::errors::VecError;
use crate::core::hint;
use crate::ops::Dot;

use std::borrow::{Borrow, BorrowMut};
use std::fmt;
use std::ops::{
  Add, AddAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Rem,
  RemAssign, Sub, SubAssign,
};
use std::slice::SliceIndex;

/// A 2-component non-owning view of an integral [Euclidean vector].
///
/// [Euclidean vector]: https://en.wikipedia.org/wiki/Euclidean_vector
///
/// # Guarantees
///
/// This type has the strict requirement that it can only reference a
/// 2-component slice of [`i32`] values. It is guaranteed to never refer to
/// more or less than 2 entries.
///
/// # Relation to [`Vector2i`]
///
/// [`Vec2i`] is a non-owning equivalent of [`Vector2i`]. This enables non-vector
/// types to either [`Deref`] or provide conversion-related utilities into
/// [`Vec2i`] types to be able to access and benefit from vector operations.
#[repr(transparent)]
#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct Vec2i([i32]);

impl Vec2i {
  /// Forms a reference to a [`Vec2i`] from a 2-component [`i32`] array.
  ///
  /// This function is identical to [`from_slice_unchecked`], except it is not
  /// marked `unsafe`.
  ///
  /// [`from_slice_unchecked`]: Self::from_slice_unchecked
  ///
  /// # Parameters
  ///
  /// * `array` - an array containing 2 [`i32`] values.
  ///
  /// # Examples
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::vec::Vec2i;
  /// let array: [i32; 2] = [1, 42];
  ///
  /// let vec = Vec2i::from_array(&array);
  ///
  /// assert_eq!(vec.as_ptr(), array.as_ptr());
  /// assert_eq!(vec.x(), array[0]);
  /// assert_eq!(vec.y(), array[1]);
  /// ```
  #[must_use]
  #[inline(always)]
  pub const fn from_array(array: &[i32; 2]) -> &Self {
    // SAFETY: `array` is guaranteed to be 2-components
    unsafe { std::mem::transmute(array.as_slice()) }
  }

  /// Forms a mutable reference to a [`Vec2i`] from a 2-component [`i32`] array.
  ///
  /// This function is identical to [`from_mut_slice_unchecked`], except it is
  /// not marked `unsafe`.
  ///
  /// [`from_mut_slice_unchecked`]: Self::from_mut_slice_unchecked
  ///
  /// # Parameters
  ///
  /// * `array` - an array containing 2 [`i32`] values.
  ///
  /// # Examples
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::vec::Vec2i;
  /// let mut array: [i32; 2] = [1, 42];
  ///
  /// let vec = Vec2i::from_mut_array(&mut array);
  ///
  /// assert_eq!(vec.as_ptr(), array.as_ptr());
  /// ```
  #[must_use]
  #[inline(always)]
  pub fn from_mut_array(array: &mut [i32; 2]) -> &mut Self {
    // SAFETY: `array` is guaranteed to be 2-components
    unsafe { std::mem::transmute(array.as_mut_slice()) }
  }

  /// Forms a reference to a [`Vec2`] from a slice of [`i32`].
  ///
  /// This requires that `slice.len() == 2`, otherwise this returns [`VecError`].
  ///
  /// # Parameters
  ///
  /// * `slice` - the slice of [`i32`]s.
  ///
  /// # Examples
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::vec::Vec2i;
  /// let slice = &[1, 02];
  ///
  /// let vec = Vec2i::from_slice(slice).unwrap();
  ///
  /// assert_eq!(vec.as_ptr(), slice.as_ptr());
  /// assert_eq!(vec.x(), slice[0]);
  /// assert_eq!(vec.y(), slice[1]);
  /// ```
  ///
  /// Invalid size:
  ///
  /// ```rust
  /// # use alloy::math::vec::Vec2i;
  /// let slice = &[1];
  ///
  /// let vec = Vec2i::from_slice(slice);
  ///
  /// assert!(vec.is_err());
  /// ```
  pub const fn from_slice(slice: &[i32]) -> Result<&Self, VecError> {
    if hint::likely(slice.len() == 2) {
      // SAFETY: slice is checked to have exactly 2 elements
      Ok(unsafe { Self::from_slice_unchecked(slice) })
    } else {
      Err(VecError::new(2, slice.len()))
    }
  }

  /// Forms a mutable reference to a [`Vec2`] from a mutable slice of [`i32`].
  ///
  /// This requires that `slice.len() == 2`, otherwise this returns [`VecError`].
  ///
  /// # Parameters
  ///
  /// * `slice` - the mutable slice of [`i32`]s.
  ///
  /// # Examples
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::vec::Vec2i;
  /// let slice = &mut [1, 42];
  ///
  /// let vec = Vec2i::from_mut_slice(slice).unwrap();
  ///
  /// assert_eq!(vec.as_ptr(), slice.as_ptr());
  /// ```
  ///
  /// Invalid size:
  ///
  /// ```rust
  /// # use alloy::math::vec::Vec2i;
  /// let slice = &mut [1];
  ///
  /// let vec = Vec2i::from_mut_slice(slice);
  ///
  /// assert!(vec.is_err());
  /// ```
  pub fn from_mut_slice(slice: &mut [i32]) -> Result<&mut Self, VecError> {
    if hint::likely(slice.len() == 2) {
      // SAFETY: slice is checked to have exactly 2 elements
      Ok(unsafe { Self::from_mut_slice_unchecked(slice) })
    } else {
      Err(VecError::new(2, slice.len()))
    }
  }

  /// Forms a reference to a [`Vec2`] from a slice of [`i32`] that is assumed to
  /// contain two values.
  ///
  /// # Parameters
  ///
  /// * `slice` - the slice of [`i32`]s.
  ///
  /// # Safety
  ///
  /// `slice.len()` must be equal to `2`.
  #[must_use]
  #[inline(always)]
  pub const unsafe fn from_slice_unchecked(slice: &[i32]) -> &Self {
    debug_assert!(slice.len() == 2);
    // SAFETY:
    //   Vec2 is transparent, and implemented directly in terms of a
    //   slice of i32s. The representation is the same, and thus valid.
    //   This is implemented symmetrically to `OsStr`.
    unsafe { std::mem::transmute(slice) }
  }

  /// Forms a mutable reference to a [`Vec2`] from a slice of [`i32`] that is
  /// assumed to contain two values.
  ///
  /// # Parameters
  ///
  /// * `slice` - the mutable slice of [`i32`]s.
  ///
  /// # Safety
  ///
  /// `slice.len()` must be equal to `2`.
  #[must_use]
  #[inline(always)]
  pub unsafe fn from_mut_slice_unchecked(slice: &mut [i32]) -> &mut Self {
    debug_assert!(slice.len() == 2);
    // SAFETY: See from_slice_unchecked
    unsafe { std::mem::transmute(slice) }
  }

  /// Forms a reference to a [`Vec2`] from a pointer to a contiguous sequence
  /// of at least two [`i32`]s.
  ///
  /// # Parameters
  ///
  /// * `ptr` - the pointer to a sequence of [`i32`] values
  ///
  /// # Safety
  ///
  /// `ptr` must point to an allocated object that references at least two
  /// entries
  #[must_use]
  #[inline(always)]
  pub const unsafe fn from_ptr_unchecked<'a>(ptr: *const i32) -> &'a Vec2i {
    Vec2i::from_slice_unchecked(std::slice::from_raw_parts(ptr, 2))
  }

  /// Forms a mutable reference to a [`Vec2`] from a pointer to a contiguous
  /// sequence of at least two [`i32`]s.
  ///
  /// # Parameters
  ///
  /// * `ptr` - the pointer to a sequence of [`i32`] values
  ///
  /// # Safety
  ///
  /// `ptr` must point to an allocated object that references at least two
  /// entries
  #[must_use]
  #[inline(always)]
  pub unsafe fn from_mut_ptr_unchecked<'a>(ptr: *mut i32) -> &'a mut Vec2i {
    Vec2i::from_mut_slice_unchecked(std::slice::from_raw_parts_mut(ptr, 2))
  }

  /// Returns this [`Vec2`] as a slice of [`i32`].
  #[must_use]
  #[inline(always)]
  pub const fn as_slice(&self) -> &[i32] {
    &self.0
  }

  /// Returns this [`Vec2`] as a mutable slice of [`i32`].
  #[must_use]
  #[inline(always)]
  pub fn as_mut_slice(&mut self) -> &mut [i32] {
    &mut self.0
  }

  /// Returns a raw pointer to [`i32`] of the vector's buffer.
  #[must_use]
  #[inline(always)]
  pub const fn as_ptr(&self) -> *const i32 {
    self.as_slice().as_ptr()
  }

  /// Returns a mutable raw pointer to [`i32`] of the vector's buffer.
  #[must_use]
  #[inline(always)]
  pub fn as_mut_ptr(&mut self) -> *mut i32 {
    self.as_mut_slice().as_mut_ptr()
  }

  /// Returns an iterator over the vector.
  #[inline(always)]
  pub fn iter(&self) -> impl Iterator<Item = &i32> {
    self.as_slice().iter()
  }

  /// Returns a mutable iterator over the vector.
  #[inline(always)]
  pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut i32> {
    self.as_mut_slice().iter_mut()
  }

  /// Returns the X-coordinate of this 2-component vector.
  #[must_use]
  #[inline(always)]
  pub const fn x(&self) -> i32 {
    unsafe { *self.0.as_ptr() }
  }

  /// Returns the Y-coordinate of this 2-component vector.
  #[must_use]
  #[inline(always)]
  pub const fn y(&self) -> i32 {
    unsafe { *self.0.as_ptr().add(1) }
  }

  /// Returns a mutable reference to the X-coordinate of this 2-component vector.
  #[must_use]
  #[inline(always)]
  pub fn x_mut(&mut self) -> &mut i32 {
    unsafe { &mut *self.0.as_mut_ptr() }
  }

  /// Returns a mutable reference to the Y-coordinate of this 2-component vector.
  #[must_use]
  #[inline(always)]
  pub fn y_mut(&mut self) -> &mut i32 {
    unsafe { &mut *self.0.as_mut_ptr().add(1) }
  }

  /// Sets the x-component
  ///
  /// # Parameters
  ///
  /// * `x` - the X-component
  #[inline(always)]
  pub fn set_x(&mut self, x: i32) {
    unsafe { *self.0.as_mut_ptr() = x }
  }

  /// Sets the y-component
  ///
  /// # Parameters
  ///
  /// * `y` - the Y-component
  #[inline(always)]
  pub fn set_y(&mut self, y: i32) {
    unsafe { *self.0.as_mut_ptr().add(1) = y }
  }

  /// Sets all the components of this vector the values from other.
  ///
  /// # Parameters
  ///
  /// * `other` - the other [`Vec2`] to set.
  pub fn set(&mut self, other: &Vec2i) {
    let src_ptr = other.as_ptr();
    let dest_ptr = self.0.as_mut_ptr();

    unsafe {
      for i in 0..2 {
        *dest_ptr.add(i) = *src_ptr.add(i);
      }
    }
  }

  /// Computes the absolute value of `self`
  #[must_use]
  pub fn abs(&self) -> Vector2i {
    Vector2i {
      x: self.x().abs(),
      y: self.y().abs(),
    }
  }

  /// Computes the minimum of each vector.
  ///
  /// # Parameters
  ///
  /// * `lhs` - the left vector
  /// * `rhs` - the right vector
  pub fn min(&self, rhs: &Vec2i) -> Vector2i {
    Vector2i {
      x: self.x().min(rhs.x()),
      y: self.y().min(rhs.y()),
    }
  }

  /// Computes the maximum of each vector.
  ///
  /// # Parameters
  ///
  /// * `lhs` - the left vector
  /// * `rhs` - the right vector
  pub fn max(&self, rhs: &Vec2i) -> Vector2i {
    Vector2i {
      x: self.x().max(rhs.x()),
      y: self.y().max(rhs.y()),
    }
  }
}

impl AsRef<[i32]> for Vec2i {
  #[must_use]
  #[inline(always)]
  fn as_ref(&self) -> &[i32] {
    &self.0
  }
}

impl AsMut<[i32]> for Vec2i {
  #[must_use]
  #[inline(always)]
  fn as_mut(&mut self) -> &mut [i32] {
    &mut self.0
  }
}

impl Borrow<[i32]> for Vec2i {
  fn borrow(&self) -> &[i32] {
    &self.0
  }
}

impl BorrowMut<[i32]> for Vec2i {
  fn borrow_mut(&mut self) -> &mut [i32] {
    &mut self.0
  }
}

impl<I> Index<I> for Vec2i
where
  I: SliceIndex<[i32]>,
{
  type Output = I::Output;

  #[must_use]
  #[inline(always)]
  fn index(&self, index: I) -> &Self::Output {
    // SAFETY: All Vec4 objects are guaranteed to have 2 components
    unsafe { crate::core::hint::fixed_size(&self.0, 2) };

    self.0.index(index)
  }
}

impl<I> IndexMut<I> for Vec2i
where
  I: SliceIndex<[i32]>,
{
  #[must_use]
  #[inline(always)]
  fn index_mut(&mut self, index: I) -> &mut Self::Output {
    // SAFETY: All Vec4 objects are guaranteed to have 2 components
    unsafe { crate::core::hint::fixed_size(&self.0, 2) };

    self.0.index_mut(index)
  }
}

impl Dot for Vec2i {
  type Output = i32;
  #[must_use]
  fn dot(&self, rhs: &Self) -> Self::Output {
    self.x() * rhs.x() + self.y() * rhs.y()
  }
}

impl Neg for &'_ Vec2i {
  type Output = Vector2i;

  #[must_use]
  fn neg(self) -> Self::Output {
    Vector2i {
      x: -self.x(),
      y: -self.y(),
    }
  }
}

impl Add for &'_ Vec2i {
  type Output = Vector2i;

  #[must_use]
  fn add(self, rhs: Self) -> Self::Output {
    Vector2i {
      x: self.x() + rhs.x(),
      y: self.y() + rhs.y(),
    }
  }
}

impl Sub for &'_ Vec2i {
  type Output = Vector2i;

  #[must_use]
  fn sub(self, rhs: Self) -> Self::Output {
    Vector2i {
      x: self.x() - rhs.x(),
      y: self.y() - rhs.y(),
    }
  }
}

impl Mul<i32> for &'_ Vec2i {
  type Output = Vector2i;

  #[must_use]
  fn mul(self, rhs: i32) -> Self::Output {
    Vector2i {
      x: self.x() * rhs,
      y: self.y() * rhs,
    }
  }
}

impl Mul<&'_ Vec2i> for i32 {
  type Output = Vector2i;

  #[must_use]
  fn mul(self, rhs: &'_ Vec2i) -> Self::Output {
    Vector2i {
      x: self * rhs.x(),
      y: self * rhs.y(),
    }
  }
}

impl Div<i32> for &'_ Vec2i {
  type Output = Vector2i;

  #[must_use]
  fn div(self, rhs: i32) -> Self::Output {
    Vector2i {
      x: self.x() / rhs,
      y: self.y() / rhs,
    }
  }
}

impl Rem<i32> for &'_ Vec2i {
  type Output = Vector2i;

  #[must_use]
  fn rem(self, rhs: i32) -> Self::Output {
    Vector2i {
      x: self.x().rem(rhs),
      y: self.y().rem(rhs),
    }
  }
}

impl AddAssign<&Vec2i> for Vec2i {
  fn add_assign(&mut self, rhs: &Vec2i) {
    let dest_ptr = self.0.as_mut_ptr();
    let src_ptr = rhs.0.as_ptr();

    unsafe {
      *dest_ptr += *src_ptr;
      *dest_ptr.add(1) += *src_ptr.add(1);
    }
  }
}

impl SubAssign<&Vec2i> for Vec2i {
  fn sub_assign(&mut self, rhs: &Vec2i) {
    let dest_ptr = self.0.as_mut_ptr();
    let src_ptr = rhs.0.as_ptr();

    unsafe {
      *dest_ptr -= *src_ptr;
      *dest_ptr.add(1) -= *src_ptr.add(1);
    }
  }
}

impl MulAssign<i32> for Vec2i {
  fn mul_assign(&mut self, rhs: i32) {
    let dest_ptr = self.0.as_mut_ptr();

    unsafe {
      *dest_ptr *= rhs;
      *dest_ptr.add(1) *= rhs;
    }
  }
}

impl DivAssign<i32> for Vec2i {
  fn div_assign(&mut self, rhs: i32) {
    let dest_ptr = self.0.as_mut_ptr();

    unsafe {
      *dest_ptr /= rhs;
      *dest_ptr.add(1) /= rhs;
    }
  }
}

impl RemAssign<i32> for Vec2i {
  fn rem_assign(&mut self, rhs: i32) {
    let dest_ptr = self.0.as_mut_ptr();

    unsafe {
      *dest_ptr %= rhs;
      *dest_ptr.add(1) %= rhs;
    }
  }
}

impl fmt::Debug for Vec2i {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Vec2i")
      .field("x", &self.x())
      .field("y", &self.y())
      .finish()
  }
}

impl fmt::Display for Vec2i {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "[{}, {}]", self.x(), self.y())
  }
}

/// An owning representation of an integral 2-component [Euclidean vector].
///
/// Like [`Vec2i`], the [`Vector2i`] object represents a [Euclidean vector] in
/// 2D. Unlike the [`Vec2i`], this is an owning representation that stores the
/// actual content of the vector.
///
/// [Euclidean vector]: https://en.wikipedia.org/wiki/Euclidean_vector
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, PartialOrd, Debug, Eq, Ord)]
pub struct Vector2i {
  /// The X-component of the vector.
  pub x: i32,
  /// The Y-component of the vector.
  pub y: i32,
}

impl Vector2i {
  /// A constant for a [Null vector], which has magnitude 0 and exists at the
  /// [origin].
  ///
  /// [Null vector]: https://en.wikipedia.org/wiki/Null_vector
  /// [origin]: https://en.wikipedia.org/wiki/Origin_(mathematics)
  pub const ZERO: Vector2i = Vector2i::new(0, 0);

  /// A constant for a [unit vector] in the positive X-direction.
  ///
  /// [unit vector]: https://en.wikipedia.org/wiki/Unit_vector
  pub const UNIT_X: Vector2i = Vector2i::new(1, 0);

  /// A constant for a [unit vector] in the positive Y-direction.
  ///
  /// [unit vector]: https://en.wikipedia.org/wiki/Unit_vector
  pub const UNIT_Y: Vector2i = Vector2i::new(0, 1);

  /// A constant for a [unit vector] in the negative X-direction.
  ///
  /// [unit vector]: https://en.wikipedia.org/wiki/Unit_vector
  pub const NEG_UNIT_X: Vector2i = Vector2i::new(-1, 0);

  /// A constant for a [unit vector] in the negative Y-direction.
  ///
  /// [unit vector]: https://en.wikipedia.org/wiki/Unit_vector
  pub const NEG_UNIT_Y: Vector2i = Vector2i::new(0, -1);

  /// Constructs this vector from an x and y coordinate.
  ///
  /// # Parameters
  ///
  /// * `x` - the x-component
  /// * `y` - the y-component
  #[must_use]
  #[inline(always)]
  pub const fn new(x: i32, y: i32) -> Self {
    Self { x, y }
  }

  /// Constructs this vector with a uniform value `v`.
  ///
  /// # Parameters
  ///
  /// * `v` - the value to uniformly apply
  #[must_use]
  #[inline(always)]
  pub const fn uniform(v: i32) -> Self {
    Self::new(v, v)
  }

  /// Constructs this vector from a 2-component [`i32`] array.
  ///
  /// # Parameters
  ///
  /// * `array` - an array containing 2 [`i32`] values.
  #[must_use]
  #[inline(always)]
  pub const fn from_array(array: &[i32; 2]) -> Self {
    Self::new(array[0], array[1])
  }

  /// Constructs this vector from a slice of floats.
  ///
  /// This will return [`VecError`] if `slice.len()` is not equal to 2.
  ///
  /// # Parameters
  ///
  /// * `slice` - the slice to read from
  pub const fn from_slice(slice: &[i32]) -> Result<Self, VecError> {
    if hint::likely(slice.len() == 2) {
      // SAFETY: slice is checked to have exactly 2 elements
      Ok(unsafe { Self::from_slice_unchecked(slice) })
    } else {
      Err(VecError::new(2, slice.len()))
    }
  }

  /// Constructs this vector from a slice of floats.
  ///
  /// # Parameters
  ///
  /// * `slice` - the slice to read from
  ///
  /// # Safety
  ///
  /// `slice.len()` must be greater or equal to `2`, otherwise this will
  /// access an out-of-bounds entry and `panic`.
  #[must_use]
  #[inline(always)]
  pub const unsafe fn from_slice_unchecked(slice: &[i32]) -> Self {
    debug_assert!(slice.len() == 2);
    Self::from_ptr(slice.as_ptr())
  }

  /// Constructs this vector from a [`Vec2i`]
  ///
  /// # Parameters
  ///
  /// * `other` - the other vector
  #[must_use]
  #[inline(always)]
  pub const fn from_vec2i(other: &Vec2i) -> Self {
    Self {
      x: other.x(),
      y: other.y(),
    }
  }

  /// Constructs this vector from a pointer to floating point values.
  ///
  /// # Parameters
  ///
  /// * `ptr` - the pointer to the start of a contiguous sequence of floats
  ///
  /// # Safety
  ///
  /// This function requires that `ptr` be non-null and point to the start of a
  /// contiguous sequence of 2 [`i32`] values.
  #[must_use]
  #[inline(always)]
  pub const unsafe fn from_ptr(ptr: *const i32) -> Self {
    Self::new(*ptr, *ptr.add(1))
  }

  /// Returns this vector as a [`Vec2`].
  #[must_use]
  #[inline(always)]
  pub const fn as_vec2i(&self) -> &Vec2i {
    // SAFETY:
    //
    //   Vector2 is repr(C) and thus points to two contiguous elements
    //   of type and align of `i32`. The only pointer capable of accessing both
    //   entries within its memory region is a pointer to itself (`*const _`).
    //   Thus, we alias this to `i32` -- which under `repr(C)` points to the
    //   first element, and has proper reachability into its neighbor-element.
    unsafe {
      std::mem::transmute(std::slice::from_raw_parts(
        self as *const _ as *const i32,
        2,
      ))
    }
  }

  /// Returns this vector as a mutable [`Vec2`].
  #[must_use]
  #[inline(always)]
  pub fn as_mut_vec2i(&mut self) -> &mut Vec2i {
    // SAFETY: See explanation in Self::as_vec2i
    unsafe {
      std::mem::transmute(std::slice::from_raw_parts_mut(
        self as *mut _ as *mut i32,
        2,
      ))
    }
  }

  /// Returns this vector as a slice of [`i32`].
  #[must_use]
  #[inline(always)]
  pub const fn as_slice(&self) -> &[i32] {
    self.as_vec2i().as_slice()
  }

  /// Returns this vector as a mutable slice of [`i32`].
  #[must_use]
  #[inline(always)]
  pub fn as_mut_slice(&mut self) -> &mut [i32] {
    self.as_mut_vec2i().as_mut_slice()
  }
}

impl From<&'_ Vec2i> for Vector2i {
  #[must_use]
  #[inline(always)]
  fn from(value: &'_ Vec2i) -> Self {
    value.to_owned()
  }
}

impl Deref for Vector2i {
  type Target = Vec2i;

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    self.borrow()
  }
}

impl DerefMut for Vector2i {
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.borrow_mut()
  }
}

impl AsRef<Vec2i> for Vector2i {
  #[must_use]
  #[inline(always)]
  fn as_ref(&self) -> &Vec2i {
    self.as_vec2i()
  }
}

impl AsMut<Vec2i> for Vector2i {
  #[inline(always)]
  fn as_mut(&mut self) -> &mut Vec2i {
    self.as_mut_vec2i()
  }
}

impl Borrow<Vec2i> for Vector2i {
  #[inline(always)]
  fn borrow(&self) -> &Vec2i {
    self.as_vec2i()
  }
}

impl BorrowMut<Vec2i> for Vector2i {
  #[inline(always)]
  fn borrow_mut(&mut self) -> &mut Vec2i {
    self.as_mut_vec2i()
  }
}

impl ToOwned for Vec2i {
  type Owned = Vector2i;

  #[inline(always)]
  fn to_owned(&self) -> Self::Owned {
    Vector2i {
      x: self.x(),
      y: self.y(),
    }
  }
}

impl Add for &Vector2i {
  type Output = Vector2i;

  #[must_use]
  #[inline(always)]
  fn add(self, rhs: Self) -> Self::Output {
    self.as_vec2i().add(rhs.as_vec2i())
  }
}

impl Add for Vector2i {
  type Output = Vector2i;

  #[must_use]
  #[inline(always)]
  fn add(self, rhs: Self) -> Self::Output {
    self.add(rhs.as_vec2i())
  }
}

impl Add<&Vec2i> for &Vector2i {
  type Output = Vector2i;

  #[must_use]
  #[inline(always)]
  fn add(self, rhs: &Vec2i) -> Self::Output {
    self.as_vec2i().add(rhs)
  }
}

impl Add<&Vector2i> for &'_ Vec2i {
  type Output = Vector2i;

  #[must_use]
  #[inline(always)]
  fn add(self, rhs: &Vector2i) -> Self::Output {
    self.add(rhs.as_vec2i())
  }
}

impl Add<Vector2i> for &Vec2i {
  type Output = Vector2i;

  #[must_use]
  #[inline(always)]
  fn add(self, rhs: Vector2i) -> Self::Output {
    // Addition is commutative, so reordering operations is safe
    rhs.add(self)
  }
}

impl Add<&Vec2i> for Vector2i {
  type Output = Vector2i;

  #[must_use]
  fn add(mut self, rhs: &Vec2i) -> Self::Output {
    // Repurpose 'self' for the output, to save space (1 less lifetime)
    let dest_ptr = self.0.as_mut_ptr();
    let src_ptr = rhs.0.as_ptr();

    unsafe {
      for i in 0..2 {
        *dest_ptr.add(i) += *src_ptr.add(i)
      }
    }
    self
  }
}

impl Add<&Vector2i> for Vector2i {
  type Output = Vector2i;

  #[must_use]
  #[inline(always)]
  fn add(self, rhs: &Vector2i) -> Self::Output {
    // Addition is commutative, so reordering operations is safe
    rhs.add(self.as_vec2i())
  }
}

impl Add<Vector2i> for &Vector2i {
  type Output = Vector2i;

  #[must_use]
  #[inline(always)]
  fn add(self, rhs: Vector2i) -> Self::Output {
    // Addition is commutative, so reordering operations is safe
    rhs.as_vec2i().add(self)
  }
}

impl Sub for &Vector2i {
  type Output = Vector2i;

  #[inline(always)]
  fn sub(self, rhs: Self) -> Self::Output {
    self.as_vec2i().sub(rhs.as_vec2i())
  }
}

impl Sub for Vector2i {
  type Output = Vector2i;

  #[inline(always)]
  fn sub(self, rhs: Self) -> Self::Output {
    self.sub(rhs.as_vec2i())
  }
}

impl Sub<&Vec2i> for &Vector2i {
  type Output = Vector2i;

  #[inline(always)]
  fn sub(self, rhs: &Vec2i) -> Self::Output {
    self.as_vec2i().sub(rhs)
  }
}

impl Sub<&Vector2i> for &'_ Vec2i {
  type Output = Vector2i;

  #[inline(always)]
  fn sub(self, rhs: &Vector2i) -> Self::Output {
    self.sub(rhs.as_vec2i())
  }
}

impl Sub<Vector2i> for &Vec2i {
  type Output = Vector2i;

  #[inline(always)]
  fn sub(self, rhs: Vector2i) -> Self::Output {
    self.sub(rhs.as_vec2i())
  }
}

impl Sub<&Vec2i> for Vector2i {
  type Output = Vector2i;

  fn sub(mut self, rhs: &Vec2i) -> Self::Output {
    // Repurpose 'self' for the output, to save space (1 less lifetime)
    let dest_ptr = self.0.as_mut_ptr();
    let src_ptr = rhs.0.as_ptr();

    unsafe {
      for i in 0..2 {
        *dest_ptr.add(i) -= *src_ptr.add(i)
      }
    }
    self
  }
}

impl Sub<&Vector2i> for Vector2i {
  type Output = Vector2i;

  #[inline(always)]
  fn sub(self, rhs: &Vector2i) -> Self::Output {
    self.sub(rhs.as_vec2i())
  }
}

impl Sub<Vector2i> for &Vector2i {
  type Output = Vector2i;

  #[inline(always)]
  fn sub(self, rhs: Vector2i) -> Self::Output {
    self.sub(rhs.as_vec2i())
  }
}

impl Mul<i32> for Vector2i {
  type Output = Vector2i;

  #[inline(always)]
  fn mul(mut self, rhs: i32) -> Self::Output {
    self.as_mut_vec2i().mul_assign(rhs);
    self
  }
}

impl Mul<i32> for &Vector2i {
  type Output = Vector2i;

  #[inline(always)]
  fn mul(self, rhs: i32) -> Self::Output {
    self.as_vec2i().mul(rhs)
  }
}

impl Mul<Vector2i> for i32 {
  type Output = Vector2i;

  #[inline(always)]
  fn mul(self, mut rhs: Vector2i) -> Self::Output {
    rhs.as_mut_vec2i().mul_assign(self);
    rhs
  }
}

impl Mul<&Vector2i> for i32 {
  type Output = Vector2i;

  #[inline(always)]
  fn mul(self, rhs: &Vector2i) -> Self::Output {
    rhs.as_vec2i().mul(self)
  }
}

impl Div<i32> for Vector2i {
  type Output = Vector2i;

  #[inline(always)]
  fn div(mut self, rhs: i32) -> Self::Output {
    self.as_mut_vec2i().div_assign(rhs);
    self
  }
}

impl Div<i32> for &Vector2i {
  type Output = Vector2i;

  #[inline(always)]
  fn div(self, rhs: i32) -> Self::Output {
    self.as_vec2i().div(rhs)
  }
}

impl Rem<i32> for Vector2i {
  type Output = Vector2i;

  #[inline(always)]
  fn rem(mut self, rhs: i32) -> Self::Output {
    self.as_mut_vec2i().rem_assign(rhs);
    self
  }
}

impl Rem<i32> for &Vector2i {
  type Output = Vector2i;

  #[inline(always)]
  fn rem(self, rhs: i32) -> Self::Output {
    self.as_vec2i().rem(rhs)
  }
}

impl AddAssign for Vector2i {
  #[inline(always)]
  fn add_assign(&mut self, rhs: Self) {
    self.as_mut_vec2i().add_assign(&rhs)
  }
}

impl AddAssign<&Vector2i> for Vector2i {
  #[inline(always)]
  fn add_assign(&mut self, rhs: &Self) {
    self.as_mut_vec2i().add_assign(rhs)
  }
}

impl AddAssign<&Vec2i> for Vector2i {
  #[inline(always)]
  fn add_assign(&mut self, rhs: &Vec2i) {
    self.as_mut_vec2i().add_assign(rhs)
  }
}

impl SubAssign for Vector2i {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: Self) {
    self.as_mut_vec2i().sub_assign(&rhs)
  }
}

impl SubAssign<&Vector2i> for Vector2i {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: &Self) {
    self.as_mut_vec2i().sub_assign(rhs)
  }
}

impl SubAssign<&Vec2i> for Vector2i {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: &Vec2i) {
    self.as_mut_vec2i().sub_assign(rhs)
  }
}

impl MulAssign<i32> for Vector2i {
  #[inline(always)]
  fn mul_assign(&mut self, rhs: i32) {
    self.as_mut_vec2i().mul_assign(rhs)
  }
}

impl DivAssign<i32> for Vector2i {
  #[inline(always)]
  fn div_assign(&mut self, rhs: i32) {
    self.as_mut_vec2i().div_assign(rhs)
  }
}

impl RemAssign<i32> for Vector2i {
  #[inline(always)]
  fn rem_assign(&mut self, rhs: i32) {
    self.as_mut_vec2i().rem_assign(rhs)
  }
}

impl fmt::Display for Vector2i {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "[{}, {}]", self.x, self.y)
  }
}

#[cfg(test)]
#[path = "vec2i.test.rs"]
mod test;
