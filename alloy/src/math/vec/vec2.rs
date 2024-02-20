use crate::math::vec::{Vec2i, Vec2u};

use crate::cmp::{AlmostEq, Near};
use crate::math::Angle;
use crate::ops::{Cross, Dot, Midpoint};
use std::borrow::{Borrow, BorrowMut};
use std::fmt;
use std::ops::{
  Add, AddAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Rem,
  RemAssign, Sub, SubAssign,
};
use std::slice::SliceIndex;

/// A 2-component non-owning view of a [Euclidean vector].
///
/// [Euclidean vector]: https://en.wikipedia.org/wiki/Euclidean_vector
///
/// # Guarantees
///
/// This type has the strict requirement that it can only reference a
/// 2-component slice of [`f32`] values. It is guaranteed to never refer to
/// more or less than 2 entries.
///
/// # Relation to [`Vector2`]
///
/// [`Vec2`] is a non-owning equivalent of [`Vector2`]. This enables non-vector
/// types to either [`Deref`] or provide conversion-related utilities into
/// [`Vec2`] types to be able to access and benefit from vector operations.
///
/// A basic example of where this can be useful is in something like a `Point2`
/// type, where it may not be implemented directly in terms of a [`Vector2`],
/// but can convert into a [`Vec2`] to benefit from operations on vectors.
#[repr(transparent)]
#[derive(PartialEq, PartialOrd)]
pub struct Vec2([f32]);

impl Vec2 {
  /// Forms a reference to a [`Vec2`] from a 2-component [`f32`] array.
  ///
  /// This function is identical to [`from_slice_unchecked`], except it is not
  /// marked `unsafe`.
  ///
  /// [`from_slice_unchecked`]: Self::from_slice_unchecked
  ///
  /// # Arguments
  ///
  /// * `array` - an array containing 2 [`f32`] values.
  ///
  /// # Examples
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::vec::Vec2;
  /// let array: [f32; 2] = [0.1, 0.42];
  ///
  /// let vec = Vec2::from_array(&array);
  ///
  /// assert_eq!(vec.as_ptr(), array.as_ptr());
  /// assert_eq!(vec.x(), array[0]);
  /// assert_eq!(vec.y(), array[1]);
  /// ```
  #[must_use]
  #[inline(always)]
  pub const fn from_array(array: &[f32; 2]) -> &Self {
    // SAFETY: `array` is guaranteed to be 2-components
    unsafe { std::mem::transmute(array.as_slice()) }
  }

  /// Forms a mutable reference to a [`Vec2`] from a 2-component [`f32`] array.
  ///
  /// This function is identical to [`from_mut_slice_unchecked`], except it is
  /// not marked `unsafe`.
  ///
  /// [`from_mut_slice_unchecked`]: Self::from_mut_slice_unchecked
  ///
  /// # Arguments
  ///
  /// * `array` - an array containing 2 [`f32`] values.
  ///
  /// # Examples
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::vec::Vec2;
  /// let mut array: [f32; 2] = [0.1, 0.42];
  ///
  /// let vec = Vec2::from_mut_array(&mut array);
  ///
  /// assert_eq!(vec.as_ptr(), array.as_ptr());
  /// ```
  #[must_use]
  #[inline(always)]
  pub fn from_mut_array(array: &mut [f32; 2]) -> &Self {
    // SAFETY: `array` is guaranteed to be 2-components
    unsafe { std::mem::transmute(array.as_mut_slice()) }
  }

  /// Forms a reference to a [`Vec2`] from a slice of [`f32`].
  ///
  /// This requires that `slice.len() == 2`, otherwise this returns [`None`].
  ///
  /// # Arguments
  ///
  /// * `slice` - the slice of [`f32`]s.
  ///
  /// # Examples
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::vec::Vec2;
  /// let slice = &[0.1, 0.42];
  ///
  /// let vec = Vec2::from_slice(slice).unwrap();
  ///
  /// assert_eq!(vec.as_ptr(), slice.as_ptr());
  /// assert_eq!(vec.x(), slice[0]);
  /// assert_eq!(vec.y(), slice[1]);
  /// ```
  ///
  /// Invalid size:
  ///
  /// ```rust
  /// # use alloy::math::vec::Vec2;
  /// let slice = &[0.1];
  ///
  /// let vec = Vec2::from_slice(slice);
  ///
  /// assert_eq!(vec, None);
  /// ```
  #[must_use]
  pub const fn from_slice(slice: &[f32]) -> Option<&Self> {
    if slice.len() == 2 {
      // SAFETY: Vec2 is transparent, and implemented directly in terms of a
      //         slice of f32s. The representation is the same, and thus valid.
      //         This is implemented symmetrically to `OsStr`.
      Some(unsafe { std::mem::transmute(slice) })
    } else {
      None
    }
  }

  /// Forms a mutable reference to a [`Vec2`] from a mutable slice of [`f32`].
  ///
  /// This requires that `slice.len() == 2`, otherwise this returns [`None`].
  ///
  /// # Arguments
  ///
  /// * `slice` - the mutable slice of [`f32`]s.
  ///
  /// # Examples
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::vec::Vec2;
  /// let slice = &mut [0.1, 0.42];
  ///
  /// let vec = Vec2::from_mut_slice(slice).unwrap();
  ///
  /// assert_eq!(vec.as_ptr(), slice.as_ptr());
  /// ```
  ///
  /// Invalid size:
  ///
  /// ```rust
  /// # use alloy::math::vec::Vec2;
  /// let slice = &mut [0.1];
  ///
  /// let vec = Vec2::from_mut_slice(slice);
  ///
  /// assert_eq!(vec, None);
  /// ```
  #[must_use]
  pub fn from_mut_slice(slice: &mut [f32]) -> Option<&mut Self> {
    if slice.len() == 2 {
      // SAFETY: Vec2 is transparent, and implemented directly in terms of a
      //         slice of f32s. The representation is the same, and thus valid.
      //         This is implemented symmetrically to `OsStr`.
      Some(unsafe { std::mem::transmute(slice) })
    } else {
      None
    }
  }

  /// Forms a reference to a [`Vec2`] from a slice of [`f32`] that is assumed to
  /// contain two values.
  ///
  /// # Arguments
  ///
  /// * `slice` - the slice of [`f32`]s.
  ///
  /// # Safety
  ///
  /// `slice.len()` must be equal to `2`.
  #[must_use]
  #[inline(always)]
  pub const unsafe fn from_slice_unchecked(slice: &[f32]) -> &Self {
    debug_assert!(slice.len() == 2);
    // SAFETY:
    //   Vec2 is transparent, and implemented directly in terms of a
    //   slice of f32s. The representation is the same, and thus valid.
    //   This is implemented symmetrically to `OsStr`.
    unsafe { std::mem::transmute(slice) }
  }

  /// Forms a mutable reference to a [`Vec2`] from a slice of [`f32`] that is
  /// assumed to contain two values.
  ///
  /// # Arguments
  ///
  /// * `slice` - the mutable slice of [`f32`]s.
  ///
  /// # Safety
  ///
  /// `slice.len()` must be equal to `2`.
  #[must_use]
  #[inline(always)]
  pub unsafe fn from_mut_slice_unchecked(slice: &mut [f32]) -> &mut Self {
    debug_assert!(slice.len() == 2);
    // SAFETY: See from_slice_unchecked
    unsafe { std::mem::transmute(slice) }
  }

  /// Forms a reference to a [`Vec2`] from a pointer to a contiguous sequence
  /// of at least two [`f32`]s.
  ///
  /// # Arguments
  ///
  /// * `ptr` - the pointer to a sequence of [`f32`] values
  ///
  /// # Safety
  ///
  /// `ptr` must point to an allocated object that references at least two
  /// entries
  #[must_use]
  #[inline(always)]
  pub const unsafe fn from_ptr_unchecked<'a>(ptr: *const f32) -> &'a Vec2 {
    Vec2::from_slice_unchecked(std::slice::from_raw_parts(ptr, 2))
  }

  /// Forms a mutable reference to a [`Vec2`] from a pointer to a contiguous
  /// sequence of at least two [`f32`]s.
  ///
  /// # Arguments
  ///
  /// * `ptr` - the pointer to a sequence of [`f32`] values
  ///
  /// # Safety
  ///
  /// `ptr` must point to an allocated object that references at least two
  /// entries
  #[must_use]
  #[inline(always)]
  pub unsafe fn from_mut_ptr_unchecked<'a>(ptr: *mut f32) -> &'a mut Vec2 {
    Vec2::from_mut_slice_unchecked(std::slice::from_raw_parts_mut(ptr, 2))
  }

  /// Returns this [`Vec2`] as a slice of [`f32`].
  #[must_use]
  #[inline(always)]
  pub const fn as_slice(&self) -> &[f32] {
    &self.0
  }

  /// Returns this [`Vec2`] as a mutable slice of [`f32`].
  #[must_use]
  #[inline(always)]
  pub fn as_mut_slice(&mut self) -> &mut [f32] {
    &mut self.0
  }

  /// Returns a raw pointer to [`f32`] of the vector's buffer.
  #[must_use]
  #[inline(always)]
  pub const fn as_ptr(&self) -> *const f32 {
    self.as_slice().as_ptr()
  }

  /// Returns a raw pointer to [`f32`] of the vector's buffer.
  #[must_use]
  #[inline(always)]
  pub fn as_mut_ptr(&mut self) -> *mut f32 {
    self.as_mut_slice().as_mut_ptr()
  }

  /// Returns an iterator over the vector.
  #[inline(always)]
  pub fn iter(&self) -> impl Iterator<Item = &f32> {
    self.as_slice().iter()
  }

  /// Returns a mutable iterator over the vector.
  #[inline(always)]
  pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f32> {
    self.as_mut_slice().iter_mut()
  }

  /// Returns the X-coordinate of this 2-component vector.
  #[must_use]
  #[inline(always)]
  pub const fn x(&self) -> f32 {
    unsafe { *self.0.as_ptr() }
  }

  /// Returns the Y-coordinate of this 2-component vector.
  #[must_use]
  #[inline(always)]
  pub const fn y(&self) -> f32 {
    unsafe { *self.0.as_ptr().add(1) }
  }

  /// Returns a mutable reference to the X-coordinate of this 2-component vector.
  #[must_use]
  #[inline(always)]
  pub fn x_mut(&mut self) -> &mut f32 {
    unsafe { &mut *self.0.as_mut_ptr() }
  }

  /// Returns a mutable reference to the Y-coordinate of this 2-component vector.
  #[must_use]
  #[inline(always)]
  pub fn y_mut(&mut self) -> &mut f32 {
    unsafe { &mut *self.0.as_mut_ptr().add(1) }
  }

  /// Sets the x-component
  ///
  /// # Arguments
  ///
  /// * `x` - the X-component
  #[inline(always)]
  pub fn set_x(&mut self, x: f32) {
    unsafe { *self.0.as_mut_ptr() = x }
  }

  /// Sets the y-component
  ///
  /// # Arguments
  ///
  /// * `y` - the Y-component
  #[inline(always)]
  pub fn set_y(&mut self, y: f32) {
    unsafe { *self.0.as_mut_ptr().add(1) = y }
  }

  /// Sets all the components of this vector the values from other.
  ///
  /// # Arguments
  ///
  /// * `other` - the other [`Vec2`] to set.
  pub fn set(&mut self, other: &Vec2) {
    let src_ptr = other.as_ptr();
    let dest_ptr = self.0.as_mut_ptr();

    unsafe {
      for i in 0..2 {
        *dest_ptr.add(i) = *src_ptr.add(i);
      }
    }
  }

  /// Computes the square magnitude of this two-component vector.
  #[must_use]
  #[inline(always)]
  pub fn square_magnitude(&self) -> f32 {
    self.dot(self)
  }

  /// Computes the magnitude of this vector.
  ///
  /// Where possible, consider using [`Vec2::square_magnitude`] as this will
  /// avoid the need to compute the square-root.
  #[must_use]
  pub fn magnitude(&self) -> f32 {
    self.square_magnitude().sqrt()
  }

  /// Queries whether this vector is normalized (e.g. has a magnitude of 1).
  #[must_use]
  pub fn is_normalized(&self) -> bool {
    self.square_magnitude().almost_eq(&1.0)
  }

  /// Normalizes this vector.
  pub fn normalize(&mut self) {
    *self /= self.magnitude()
  }

  /// Returns a normalized version of this vector.
  #[must_use]
  pub fn normalized(&self) -> Vector2 {
    self / self.magnitude()
  }

  /// Rotates this vector around the origin by angle A.
  ///
  /// # Arguments
  ///
  /// * `angle` - the angle to rotate
  pub fn rotate<A: Angle>(&mut self, angle: A) {
    let (cos, sin) = angle.sin_cos();
    let x = cos * self.x() - sin * self.y();
    let y = sin * self.x() + cos * self.y();

    self.set_x(x);
    self.set_y(y);
  }

  /// Returns a vector rotated by the specified `angle`
  ///
  /// # Arguments
  ///
  /// * `angle` - the angle to rotate
  #[must_use]
  pub fn rotated<A: Angle>(&self, angle: A) -> Vector2 {
    let mut result = self.to_owned();
    result.rotate(angle);
    result
  }

  /// Returns whether all components of this Vec are finite.
  ///
  /// # Example
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::vec::Vector2;
  ///
  /// let finite = Vector2::UNIT_X;
  /// let not_finite = Vector2::new(f32::INFINITY, 3.0);
  ///
  /// assert!(finite.is_finite());
  /// assert!(!not_finite.is_finite());
  /// ```
  #[must_use]
  pub fn is_finite(&self) -> bool {
    self.x().is_finite() && self.y().is_finite()
  }

  /// Returns whether any component of this vec are infinite.
  ///
  /// This is the inverse operation of [`is_finite`].
  ///
  /// [`is_finite`]: Self::is_finite
  ///
  /// # Example
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::vec::Vector2;
  ///
  /// let finite = Vector2::UNIT_X;
  /// let not_finite = Vector2::new(f32::INFINITY, 3.0);
  ///
  /// assert!(!finite.is_infinite());
  /// assert!(not_finite.is_infinite());
  /// ```
  #[must_use]
  pub fn is_infinite(&self) -> bool {
    self.x().is_infinite() || self.y().is_infinite()
  }

  /// Returns whether any component of this vec are nan.
  ///
  /// # Example
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::vec::Vector2;
  ///
  /// let not_nan = Vector2::UNIT_X;
  /// let nan = Vector2::new(f32::NAN, 3.0);
  ///
  /// assert!(!not_nan.is_nan());
  /// assert!(nan.is_nan());
  /// ```
  #[must_use]
  pub fn is_nan(&self) -> bool {
    self.x().is_nan() || self.y().is_nan()
  }

  /// Computes the absolute value of `self`
  ///
  /// # Example
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::vec::Vector2;
  ///
  /// let vec = Vector2::new(-3.0, 4.0);
  /// let abs = vec.abs();
  ///
  /// assert_eq!(abs, Vector2::new(3.0, 4.0));
  /// ```
  #[must_use]
  pub fn abs(&self) -> Vector2 {
    Vector2 {
      x: self.x().abs(),
      y: self.y().abs(),
    }
  }

  fn add_impl(lhs: &Vec2, rhs: &Vec2) -> Vector2 {
    Vector2 {
      x: lhs.x() + rhs.x(),
      y: lhs.y() + rhs.y(),
    }
  }

  fn sub_impl(lhs: &Vec2, rhs: &Vec2) -> Vector2 {
    Vector2 {
      x: lhs.x() - rhs.x(),
      y: lhs.y() - rhs.y(),
    }
  }
}

impl AsRef<[f32]> for Vec2 {
  #[must_use]
  #[inline(always)]
  fn as_ref(&self) -> &[f32] {
    &self.0
  }
}

impl AsMut<[f32]> for Vec2 {
  #[must_use]
  #[inline(always)]
  fn as_mut(&mut self) -> &mut [f32] {
    &mut self.0
  }
}

impl Borrow<[f32]> for Vec2 {
  fn borrow(&self) -> &[f32] {
    &self.0
  }
}

impl BorrowMut<[f32]> for Vec2 {
  fn borrow_mut(&mut self) -> &mut [f32] {
    &mut self.0
  }
}

impl Near for Vec2 {
  #[must_use]
  fn near(&self, other: &Self, tolerance: &f32) -> bool {
    self.x().near(&other.x(), tolerance) && self.y().near(&other.y(), tolerance)
  }
}

impl AlmostEq for Vec2 {
  #[must_use]
  fn almost_eq(&self, other: &Self) -> bool {
    const EPSILON: f32 = 10.0 * std::f32::EPSILON;
    self.near(other, &EPSILON)
  }
}

impl<I> Index<I> for Vec2
where
  I: SliceIndex<[f32]>,
{
  type Output = I::Output;

  #[must_use]
  #[inline(always)]
  fn index(&self, index: I) -> &Self::Output {
    self.0.index(index)
  }
}

impl<I> IndexMut<I> for Vec2
where
  I: SliceIndex<[f32]>,
{
  #[must_use]
  #[inline(always)]
  fn index_mut(&mut self, index: I) -> &mut Self::Output {
    self.0.index_mut(index)
  }
}

impl Midpoint for Vec2 {
  type Output = Vector2;

  #[must_use]
  fn midpoint(&self, other: &Self) -> Self::Output {
    Vector2 {
      x: (self.x() + other.x()) / 2.0,
      y: (self.y() + other.y()) / 2.0,
    }
  }
}

impl Dot for Vec2 {
  type Output = f32;

  #[must_use]
  fn dot(&self, rhs: &Self) -> Self::Output {
    self.x() * rhs.x() + self.y() * rhs.y()
  }
}

impl Cross for Vec2 {
  type Output = f32;

  #[must_use]
  fn cross(&self, other: &Self) -> Self::Output {
    self.x() * other.y() - self.y() * other.x()
  }
}

impl Neg for &'_ Vec2 {
  type Output = Vector2;

  #[must_use]
  fn neg(self) -> Self::Output {
    Vector2 {
      x: -self.x(),
      y: -self.y(),
    }
  }
}

impl Add for &'_ Vec2 {
  type Output = Vector2;

  #[must_use]
  fn add(self, rhs: Self) -> Self::Output {
    Vec2::add_impl(self, rhs)
  }
}

impl Sub for &'_ Vec2 {
  type Output = Vector2;

  #[must_use]
  fn sub(self, rhs: Self) -> Self::Output {
    Vec2::sub_impl(self, rhs)
  }
}

impl Mul<f32> for &'_ Vec2 {
  type Output = Vector2;

  #[must_use]
  fn mul(self, rhs: f32) -> Self::Output {
    Vector2 {
      x: self.x() * rhs,
      y: self.y() * rhs,
    }
  }
}

impl Mul<&'_ Vec2> for f32 {
  type Output = Vector2;

  #[must_use]
  fn mul(self, rhs: &'_ Vec2) -> Self::Output {
    Vector2 {
      x: self * rhs.x(),
      y: self * rhs.y(),
    }
  }
}

impl Div<f32> for &'_ Vec2 {
  type Output = Vector2;

  #[must_use]
  fn div(self, rhs: f32) -> Self::Output {
    let inverse = 1.0 / rhs;
    Vector2 {
      x: self.x() * inverse,
      y: self.y() * inverse,
    }
  }
}

impl Rem<f32> for &'_ Vec2 {
  type Output = Vector2;

  #[must_use]
  fn rem(self, rhs: f32) -> Self::Output {
    Vector2 {
      x: self.x().rem(rhs),
      y: self.y().rem(rhs),
    }
  }
}

impl AddAssign<&Vec2> for Vec2 {
  fn add_assign(&mut self, rhs: &Vec2) {
    let dest_ptr = self.0.as_mut_ptr();
    let src_ptr = rhs.0.as_ptr();

    unsafe {
      *dest_ptr += *src_ptr;
      *dest_ptr.add(1) += *src_ptr.add(1);
    }
  }
}

impl SubAssign<&Vec2> for Vec2 {
  fn sub_assign(&mut self, rhs: &Vec2) {
    let dest_ptr = self.0.as_mut_ptr();
    let src_ptr = rhs.0.as_ptr();

    unsafe {
      *dest_ptr -= *src_ptr;
      *dest_ptr.add(1) -= *src_ptr.add(1);
    }
  }
}

impl MulAssign<f32> for Vec2 {
  fn mul_assign(&mut self, rhs: f32) {
    let dest_ptr = self.0.as_mut_ptr();

    unsafe {
      *dest_ptr *= rhs;
      *dest_ptr.add(1) *= rhs;
    }
  }
}

impl DivAssign<f32> for Vec2 {
  fn div_assign(&mut self, rhs: f32) {
    let dest_ptr = self.0.as_mut_ptr();

    let inverse = 1.0 / rhs;
    unsafe {
      *dest_ptr *= inverse;
      *dest_ptr.add(1) *= inverse;
    }
  }
}

impl RemAssign<f32> for Vec2 {
  fn rem_assign(&mut self, rhs: f32) {
    let dest_ptr = self.0.as_mut_ptr();

    unsafe {
      *dest_ptr %= rhs;
      *dest_ptr.add(1) %= rhs;
    }
  }
}

impl fmt::Debug for Vec2 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Vec2")
      .field("x", &self.x())
      .field("y", &self.y())
      .finish()
  }
}

impl fmt::Display for Vec2 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "[{}, {}]", self.x(), self.y())
  }
}

/// An owning representation of a 2-component [Euclidean vector].
///
/// Like [`Vec2`], the [`Vector2`] object represents a [Euclidean vector] in
/// 2D. Unlike the [`Vec2`], this is an owning representation that stores the
/// actual content of the vector.
///
/// [Euclidean vector]: https://en.wikipedia.org/wiki/Euclidean_vector
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, PartialOrd, Debug)]
pub struct Vector2 {
  /// The X-component of the vector.
  pub x: f32,
  /// The Y-component of the vector.
  pub y: f32,
}

impl Vector2 {
  /// A constant for a [Null vector], which has magnitude 0 and exists at the
  /// [origin].
  ///
  /// [Null vector]: https://en.wikipedia.org/wiki/Null_vector
  /// [origin]: https://en.wikipedia.org/wiki/Origin_(mathematics)
  pub const ZERO: Vector2 = Vector2::new(0.0, 0.0);

  /// A constant for a [unit vector] in the positive X-direction.
  ///
  /// [unit vector]: https://en.wikipedia.org/wiki/Unit_vector
  pub const UNIT_X: Vector2 = Vector2::new(1.0, 0.0);

  /// A constant for a [unit vector] in the positive Y-direction.
  ///
  /// [unit vector]: https://en.wikipedia.org/wiki/Unit_vector
  pub const UNIT_Y: Vector2 = Vector2::new(0.0, 1.0);

  /// A constant for a [unit vector] in the negative X-direction.
  ///
  /// [unit vector]: https://en.wikipedia.org/wiki/Unit_vector
  pub const NEG_UNIT_X: Vector2 = Vector2::new(-1.0, 0.0);

  /// A constant for a [unit vector] in the negative Y-direction.
  ///
  /// [unit vector]: https://en.wikipedia.org/wiki/Unit_vector
  pub const NEG_UNIT_Y: Vector2 = Vector2::new(0.0, -1.0);

  /// Constructs this vector from an x and y coordinate.
  ///
  /// # Arguments
  ///
  /// * `x` - the x-component
  /// * `y` - the y-component
  #[must_use]
  #[inline(always)]
  pub const fn new(x: f32, y: f32) -> Self {
    Self { x, y }
  }

  /// Constructs this vector with a uniform value `v`.
  ///
  /// # Arguments
  ///
  /// * `v` - the value to uniformly apply
  #[must_use]
  #[inline(always)]
  pub const fn uniform(v: f32) -> Self {
    Self::new(v, v)
  }

  /// Constructs this vector from a 2-component [`f32`] array.
  ///
  /// # Arguments
  ///
  /// * `array` - an array containing 2 [`f32`] values.
  #[must_use]
  #[inline(always)]
  pub const fn from_array(array: &[f32; 2]) -> Self {
    Self::new(array[0], array[1])
  }

  /// Constructs this vector from a slice of floats.
  ///
  /// This will return [`None`] if `slice.len()` is not equal to 2.
  ///
  /// # Arguments
  ///
  /// * `slice` - the slice to read from
  #[must_use]
  pub const fn from_slice(slice: &[f32]) -> Option<Self> {
    if slice.len() != 2 {
      None
    } else {
      Some(Self {
        x: slice[0],
        y: slice[1],
      })
    }
  }

  /// Constructs this vector from a slice of floats.
  ///
  /// # Arguments
  ///
  /// * `slice` - the slice to read from
  ///
  /// # Safety
  ///
  /// `slice.len()` must be greater or equal to `2`, otherwise this will
  /// access an out-of-bounds entry and `panic`.
  #[must_use]
  #[inline(always)]
  pub const unsafe fn from_slice_unchecked(slice: &[f32]) -> Self {
    debug_assert!(slice.len() == 2);
    Self::from_ptr(slice.as_ptr())
  }

  /// Constructs this vector from a pointer to floating point values.
  ///
  /// # Arguments
  ///
  /// * `ptr` - the pointer to the start of a contiguous sequence of floats
  ///
  /// # Safety
  ///
  /// This function requires that `ptr` be non-null and point to the start of a
  /// contiguous sequence of 2 [`f32`] values.
  #[must_use]
  #[inline(always)]
  pub const unsafe fn from_ptr(ptr: *const f32) -> Self {
    Self::new(*ptr, *ptr.add(1))
  }

  /// Constructs this vector from a [`Vec2`]
  ///
  /// # Arguments
  ///
  /// * `other` - the other vector
  #[must_use]
  #[inline(always)]
  pub const fn from_vec2(other: &Vec2) -> Self {
    Self {
      x: other.x(),
      y: other.y(),
    }
  }

  /// Constructs this vector from a [`Vec2i`]
  ///
  /// # Arguments
  ///
  /// * `other` - the other vector
  #[must_use]
  pub fn from_vec2i(other: &Vec2i) -> Self {
    Self {
      x: other.x() as f32,
      y: other.y() as f32,
    }
  }

  /// Constructs this vector from a [`Vec2u`]
  ///
  /// # Arguments
  ///
  /// * `other` - the other vector
  #[must_use]
  pub fn from_vec2u(other: &Vec2u) -> Self {
    Self {
      x: other.x() as f32,
      y: other.y() as f32,
    }
  }

  /// Returns this vector as a [`Vec2`].
  #[must_use]
  #[inline(always)]
  pub const fn as_vec2(&self) -> &Vec2 {
    // SAFETY:
    //
    //   Vector2 is repr(C) and thus points to two contiguous elements
    //   of type and align of `f32`. The only pointer capable of accessing both
    //   entries within its memory region is a pointer to itself (`*const _`).
    //   Thus, we alias this to `f32` -- which under `repr(C)` points to the
    //   first element, and has proper reachability into its neighbor-element.
    unsafe {
      std::mem::transmute(std::slice::from_raw_parts(
        self as *const _ as *const f32,
        2,
      ))
    }
  }

  /// Returns this vector as a mutable [`Vec2`].
  #[must_use]
  #[inline(always)]
  pub fn as_mut_vec2(&mut self) -> &mut Vec2 {
    // SAFETY: See explanation in Borrow<Vec2>
    unsafe {
      std::mem::transmute(std::slice::from_raw_parts_mut(
        self as *mut _ as *mut f32,
        2,
      ))
    }
  }

  /// Returns this vector as a slice of [`f32`].
  #[must_use]
  #[inline(always)]
  pub const fn as_slice(&self) -> &[f32] {
    self.as_vec2().as_slice()
  }

  /// Returns this vector as a mutable slice of [`f32`].
  #[must_use]
  #[inline(always)]
  pub fn as_mut_slice(&mut self) -> &mut [f32] {
    self.as_mut_vec2().as_mut_slice()
  }
}

impl From<&'_ Vec2> for Vector2 {
  #[must_use]
  #[inline(always)]
  fn from(value: &'_ Vec2) -> Self {
    Self::from_vec2(value)
  }
}

impl From<&'_ Vec2i> for Vector2 {
  #[must_use]
  #[inline]
  fn from(value: &'_ Vec2i) -> Self {
    Self::from_vec2i(value)
  }
}

impl From<&'_ Vec2u> for Vector2 {
  #[must_use]
  #[inline]
  fn from(value: &'_ Vec2u) -> Self {
    Self::from_vec2u(value)
  }
}

impl Deref for Vector2 {
  type Target = Vec2;

  #[must_use]
  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    self.borrow()
  }
}

impl DerefMut for Vector2 {
  #[must_use]
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.borrow_mut()
  }
}

impl AsRef<Vec2> for Vector2 {
  #[must_use]
  #[inline(always)]
  fn as_ref(&self) -> &Vec2 {
    self.as_vec2()
  }
}

impl AsMut<Vec2> for Vector2 {
  #[inline(always)]
  fn as_mut(&mut self) -> &mut Vec2 {
    self.as_mut_vec2()
  }
}

impl Borrow<Vec2> for Vector2 {
  #[must_use]
  #[inline(always)]
  fn borrow(&self) -> &Vec2 {
    self.as_vec2()
  }
}

impl BorrowMut<Vec2> for Vector2 {
  #[must_use]
  #[inline(always)]
  fn borrow_mut(&mut self) -> &mut Vec2 {
    self.as_mut_vec2()
  }
}

impl ToOwned for Vec2 {
  type Owned = Vector2;

  #[must_use]
  #[inline(always)]
  fn to_owned(&self) -> Self::Owned {
    Vector2 {
      x: self.x(),
      y: self.y(),
    }
  }
}

impl Add for &Vector2 {
  type Output = Vector2;

  #[must_use]
  #[inline(always)]
  fn add(self, rhs: Self) -> Self::Output {
    Vec2::add_impl(self.as_vec2(), rhs.as_vec2())
  }
}

impl Add for Vector2 {
  type Output = Vector2;

  #[must_use]
  #[inline(always)]
  fn add(self, rhs: Self) -> Self::Output {
    Vec2::add_impl(self.as_vec2(), rhs.as_vec2())
  }
}

impl Add<&Vec2> for &Vector2 {
  type Output = Vector2;

  #[must_use]
  #[inline(always)]
  fn add(self, rhs: &Vec2) -> Self::Output {
    Vec2::add_impl(self.as_vec2(), rhs)
  }
}

impl Add<&Vector2> for &'_ Vec2 {
  type Output = Vector2;

  #[must_use]
  #[inline(always)]
  fn add(self, rhs: &Vector2) -> Self::Output {
    Vec2::add_impl(self, rhs.as_vec2())
  }
}

impl Add<Vector2> for &Vec2 {
  type Output = Vector2;

  #[must_use]
  #[inline(always)]
  fn add(self, rhs: Vector2) -> Self::Output {
    Vec2::add_impl(self, rhs.as_vec2())
  }
}

impl Add<&Vec2> for Vector2 {
  type Output = Vector2;

  #[must_use]
  fn add(self, rhs: &Vec2) -> Self::Output {
    Vec2::add_impl(self.as_vec2(), rhs)
  }
}

impl Add<&Vector2> for Vector2 {
  type Output = Vector2;

  #[must_use]
  #[inline(always)]
  fn add(self, rhs: &Vector2) -> Self::Output {
    Vec2::add_impl(self.as_vec2(), rhs.as_vec2())
  }
}

impl Add<Vector2> for &Vector2 {
  type Output = Vector2;

  #[must_use]
  #[inline(always)]
  fn add(self, rhs: Vector2) -> Self::Output {
    Vec2::add_impl(self.as_vec2(), rhs.as_vec2())
  }
}

impl Sub for &Vector2 {
  type Output = Vector2;

  #[must_use]
  #[inline(always)]
  fn sub(self, rhs: Self) -> Self::Output {
    Vec2::sub_impl(self.as_vec2(), rhs.as_vec2())
  }
}

impl Sub for Vector2 {
  type Output = Vector2;

  #[must_use]
  #[inline(always)]
  fn sub(self, rhs: Self) -> Self::Output {
    Vec2::sub_impl(self.as_vec2(), rhs.as_vec2())
  }
}

impl Sub<&Vec2> for &Vector2 {
  type Output = Vector2;

  #[must_use]
  #[inline(always)]
  fn sub(self, rhs: &Vec2) -> Self::Output {
    Vec2::sub_impl(self.as_vec2(), rhs)
  }
}

impl Sub<&Vector2> for &'_ Vec2 {
  type Output = Vector2;

  #[must_use]
  #[inline(always)]
  fn sub(self, rhs: &Vector2) -> Self::Output {
    Vec2::sub_impl(self, rhs.as_vec2())
  }
}

impl Sub<Vector2> for &Vec2 {
  type Output = Vector2;

  #[must_use]
  #[inline(always)]
  fn sub(self, rhs: Vector2) -> Self::Output {
    Vec2::sub_impl(self, rhs.as_vec2())
  }
}

impl Sub<&Vec2> for Vector2 {
  type Output = Vector2;

  #[must_use]
  fn sub(self, rhs: &Vec2) -> Self::Output {
    Vec2::sub_impl(self.as_vec2(), rhs)
  }
}

impl Sub<&Vector2> for Vector2 {
  type Output = Vector2;

  #[must_use]
  #[inline(always)]
  fn sub(self, rhs: &Vector2) -> Self::Output {
    Vec2::sub_impl(self.as_vec2(), rhs.as_vec2())
  }
}

impl Sub<Vector2> for &Vector2 {
  type Output = Vector2;

  #[must_use]
  #[inline(always)]
  fn sub(self, rhs: Vector2) -> Self::Output {
    Vec2::sub_impl(self.as_vec2(), rhs.as_vec2())
  }
}

impl Mul<f32> for Vector2 {
  type Output = Vector2;

  #[must_use]
  #[inline(always)]
  fn mul(mut self, rhs: f32) -> Self::Output {
    self.as_mut_vec2().mul_assign(rhs);
    self
  }
}

impl Mul<f32> for &Vector2 {
  type Output = Vector2;

  #[must_use]
  #[inline(always)]
  fn mul(self, rhs: f32) -> Self::Output {
    self.as_vec2().mul(rhs)
  }
}

impl Mul<Vector2> for f32 {
  type Output = Vector2;

  #[must_use]
  #[inline(always)]
  fn mul(self, mut rhs: Vector2) -> Self::Output {
    rhs.as_mut_vec2().mul_assign(self);
    rhs
  }
}

impl Mul<&Vector2> for f32 {
  type Output = Vector2;

  #[must_use]
  #[inline(always)]
  fn mul(self, rhs: &Vector2) -> Self::Output {
    rhs.as_vec2().mul(self)
  }
}

impl Div<f32> for Vector2 {
  type Output = Vector2;

  #[must_use]
  #[inline(always)]
  fn div(mut self, rhs: f32) -> Self::Output {
    self.as_mut_vec2().div_assign(rhs);
    self
  }
}

impl Div<f32> for &Vector2 {
  type Output = Vector2;

  #[must_use]
  #[inline(always)]
  fn div(self, rhs: f32) -> Self::Output {
    self.as_vec2().div(rhs)
  }
}

impl Rem<f32> for Vector2 {
  type Output = Vector2;

  #[must_use]
  #[inline(always)]
  fn rem(mut self, rhs: f32) -> Self::Output {
    self.as_mut_vec2().rem_assign(rhs);
    self
  }
}

impl Rem<f32> for &Vector2 {
  type Output = Vector2;

  #[must_use]
  #[inline(always)]
  fn rem(self, rhs: f32) -> Self::Output {
    self.as_vec2().rem(rhs)
  }
}

impl AddAssign for Vector2 {
  #[inline(always)]
  fn add_assign(&mut self, rhs: Self) {
    self.as_mut_vec2().add_assign(&rhs)
  }
}

impl AddAssign<&Vector2> for Vector2 {
  #[inline(always)]
  fn add_assign(&mut self, rhs: &Self) {
    self.as_mut_vec2().add_assign(rhs)
  }
}

impl AddAssign<&Vec2> for Vector2 {
  #[inline(always)]
  fn add_assign(&mut self, rhs: &Vec2) {
    self.as_mut_vec2().add_assign(rhs)
  }
}

impl SubAssign for Vector2 {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: Self) {
    self.as_mut_vec2().sub_assign(&rhs)
  }
}

impl SubAssign<&Vector2> for Vector2 {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: &Self) {
    self.as_mut_vec2().sub_assign(rhs)
  }
}

impl SubAssign<&Vec2> for Vector2 {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: &Vec2) {
    self.as_mut_vec2().sub_assign(rhs)
  }
}

impl MulAssign<f32> for Vector2 {
  #[inline(always)]
  fn mul_assign(&mut self, rhs: f32) {
    self.as_mut_vec2().mul_assign(rhs)
  }
}

impl DivAssign<f32> for Vector2 {
  #[inline(always)]
  fn div_assign(&mut self, rhs: f32) {
    self.as_mut_vec2().div_assign(rhs)
  }
}

impl RemAssign<f32> for Vector2 {
  #[inline(always)]
  fn rem_assign(&mut self, rhs: f32) {
    self.as_mut_vec2().rem_assign(rhs)
  }
}

impl fmt::Display for Vector2 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "[{}, {}]", self.x, self.y)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_vec2() {
    let vec = Vector2::new(4.0, 2.0);

    let magnitude = vec.square_magnitude();

    assert!(magnitude.almost_eq(&20.0))
  }

  #[test]
  fn test_identity() {
    let vec = Vector2::new(4.0, 2.0);

    assert_eq!(vec.as_ptr(), &vec[0]);
    assert_eq!(vec.as_ptr(), &vec.x);
  }

  #[test]
  fn test_iter() {
    let mut vec = Vector2::new(4.0, 2.0);

    for v in vec.iter_mut() {
      *v = *v * 2.0
    }

    assert!(vec.x.almost_eq(&8.0), "x = {}", vec.x);
    assert!(vec.y.almost_eq(&4.0), "y = {}", vec.y);
  }

  #[test]
  fn test_add() {
    let a = Vector2 { x: 10.0, y: 10.0 };
    let b = Vector2 { x: 0.0, y: 10.0 };

    let c = a + b;

    assert!(c.almost_eq(&Vector2 { x: 10.0, y: 20.0 }))
  }
}
