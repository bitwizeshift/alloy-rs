use crate::math::vec::{Vec2, Vec3, Vec4i, Vec4u};

use crate::cmp::{AlmostEq, Near};
use crate::ops::{Cross, Dot, Midpoint};
use std::borrow::{Borrow, BorrowMut};
use std::fmt;
use std::ops::{
  Add, AddAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Rem,
  RemAssign, Sub, SubAssign,
};
use std::slice::SliceIndex;

/// A 4-component non-owning view of a [Euclidean vector].
///
/// [Euclidean vector]: https://en.wikipedia.org/wiki/Euclidean_vector
///
/// # Guarantees
///
/// This type has the strict requirement that it can only reference a
/// 4-component slice of [`f32`] values. It is guaranteed to never refer to
/// more or less than 4 entries.
///
/// # Relation to [`Vector4`]
///
/// [`Vec4`] is a non-owning equivalent of [`Vector2`]. This enables non-vector
/// types to either [`Deref`] or provide conversion-related utilities into
/// [`Vec4`] types to be able to access and benefit from vector operations.
///
/// A basic example of where this can be useful is in something like a `Point2`
/// type, where it may not be implemented directly in terms of a [`Vector4`],
/// but can convert into a [`Vec4`] to benefit from operations on vectors.
#[repr(transparent)]
#[derive(PartialEq, PartialOrd)]
pub struct Vec4([f32]);

impl Vec4 {
  /// Forms a reference to a [`Vec4`] from a 4-component [`f32`] array.
  ///
  /// This function is identical to [`from_slice_unchecked`], except it is not
  /// marked `unsafe`.
  ///
  /// [`from_slice_unchecked`]: Self::from_slice_unchecked
  ///
  /// # Arguments
  ///
  /// * `array` - an array containing 4 [`f32`] values.
  ///
  /// # Examples
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::vec::Vec4;
  /// let array: [f32; 4] = [0.1, 0.42, 3.14, 0.9];
  ///
  /// let vec = Vec4::from_array(&array);
  ///
  /// assert_eq!(vec.as_ptr(), array.as_ptr());
  /// assert_eq!(vec.x(), array[0]);
  /// assert_eq!(vec.y(), array[1]);
  /// assert_eq!(vec.z(), array[2]);
  /// assert_eq!(vec.w(), array[3]);
  /// ```
  #[must_use]
  #[inline(always)]
  pub const fn from_array(array: &[f32; 4]) -> &Self {
    // SAFETY: `array` is guaranteed to be 2-components
    unsafe { std::mem::transmute(array.as_slice()) }
  }

  /// Forms a mutable reference to a [`Vec4`] from a 4-component [`f32`] array.
  ///
  /// This function is identical to [`from_mut_slice_unchecked`], except it is
  /// not marked `unsafe`.
  ///
  /// [`from_mut_slice_unchecked`]: Self::from_mut_slice_unchecked
  ///
  /// # Arguments
  ///
  /// * `array` - an array containing 4 [`f32`] values.
  ///
  /// # Examples
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::vec::Vec4;
  /// let mut array: [f32; 4] = [0.1, 42.0, 3.14, 0.9];
  ///
  /// let vec = Vec4::from_mut_array(&mut array);
  ///
  /// assert_eq!(vec.as_ptr(), array.as_ptr());
  /// ```
  #[must_use]
  #[inline(always)]
  pub fn from_mut_array(array: &mut [f32; 4]) -> &Self {
    // SAFETY: `array` is guaranteed to be 3-components
    unsafe { std::mem::transmute(array.as_mut_slice()) }
  }

  /// Forms a reference to a [`Vec4`] from a slice of [`f32`].
  ///
  /// This requires that `slice.len() == 4`, otherwise this returns [`None`].
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
  /// # use alloy::math::vec::Vec4;
  /// let slice = &[0.1, 0.42, 3.14, 0.9];
  ///
  /// let vec = Vec4::from_slice(slice).unwrap();
  ///
  /// assert_eq!(vec.as_ptr(), slice.as_ptr());
  /// assert_eq!(vec.x(), slice[0]);
  /// assert_eq!(vec.y(), slice[1]);
  /// assert_eq!(vec.z(), slice[2]);
  /// assert_eq!(vec.w(), slice[3]);
  /// ```
  ///
  /// Invalid size:
  ///
  /// ```rust
  /// # use alloy::math::vec::Vec4;
  /// let slice = &[0.1];
  ///
  /// let vec = Vec4::from_slice(slice);
  ///
  /// assert_eq!(vec, None);
  /// ```
  #[must_use]
  pub const fn from_slice(slice: &[f32]) -> Option<&Self> {
    if slice.len() == 4 {
      // SAFETY: Vec4 is transparent, and implemented directly in terms of a
      //         slice of f32s. The representation is the same, and thus valid.
      //         This is implemented symmetrically to `OsStr`.
      Some(unsafe { std::mem::transmute(slice) })
    } else {
      None
    }
  }

  /// Forms a mutable reference to a [`Vec4`] from a mutable slice of [`f32`].
  ///
  /// This requires that `slice.len() == 4`, otherwise this returns [`None`].
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
  /// # use alloy::math::vec::Vec4;
  /// let slice = &mut [0.1, 0.42, 3.14, 0.9];
  ///
  /// let vec = Vec4::from_mut_slice(slice).unwrap();
  ///
  /// assert_eq!(vec.as_ptr(), slice.as_ptr());
  /// ```
  ///
  /// Invalid size:
  ///
  /// ```rust
  /// # use alloy::math::vec::Vec4;
  /// let slice = &mut [0.1];
  ///
  /// let vec = Vec4::from_mut_slice(slice);
  ///
  /// assert_eq!(vec, None);
  /// ```
  #[must_use]
  pub fn from_mut_slice(slice: &mut [f32]) -> Option<&mut Self> {
    if slice.len() == 4 {
      // SAFETY: Vec4 is transparent, and implemented directly in terms of a
      //         slice of f32s. The representation is the same, and thus valid.
      //         This is implemented symmetrically to `OsStr`.
      Some(unsafe { std::mem::transmute(slice) })
    } else {
      None
    }
  }

  /// Forms a reference to a [`Vec4`] from a slice of [`f32`] that is assumed to
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
    debug_assert!(slice.len() == 4);
    std::mem::transmute(slice)
  }

  /// Forms a mutable reference to a [`Vec4`] from a slice of [`f32`] that is
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
    debug_assert!(slice.len() == 4);
    std::mem::transmute(slice)
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
  pub const unsafe fn from_ptr_unchecked<'a>(ptr: *const f32) -> &'a Vec4 {
    Vec4::from_slice_unchecked(std::slice::from_raw_parts(ptr, 4))
  }

  /// Forms a mutable reference to a [`Vec3`] from a pointer to a contiguous
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
  pub unsafe fn from_mut_ptr_unchecked<'a>(ptr: *mut f32) -> &'a mut Vec4 {
    Vec4::from_mut_slice_unchecked(std::slice::from_raw_parts_mut(ptr, 4))
  }

  /// Returns this [`Vec4`] as a slice of [`f32`].
  #[must_use]
  #[inline(always)]
  pub const fn as_slice(&self) -> &[f32] {
    &self.0
  }

  /// Returns this [`Vec4`] as a mutable slice of [`f32`].
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

  /// Returns the X-coordinate of this 4-component vector.
  #[must_use]
  #[inline(always)]
  pub const fn x(&self) -> f32 {
    // SAFETY: Vec4 being of size-4 is an internal invariant
    unsafe { *self.0.as_ptr() }
  }

  /// Returns the Y-coordinate of this 4-component vector.
  #[must_use]
  #[inline(always)]
  pub const fn y(&self) -> f32 {
    // SAFETY: Vec4 being of size-4 is an internal invariant
    unsafe { *self.0.as_ptr().add(1) }
  }

  /// Returns the Z-coordinate of this 4-component vector.
  #[must_use]
  #[inline(always)]
  pub const fn z(&self) -> f32 {
    // SAFETY: Vec4 being of size-4 is an internal invariant
    unsafe { *self.0.as_ptr().add(2) }
  }

  /// Returns the W-coordinate of this 4-component vector.
  #[must_use]
  #[inline(always)]
  pub const fn w(&self) -> f32 {
    // SAFETY: Vec4 being of size-4 is an internal invariant
    unsafe { *self.0.as_ptr().add(3) }
  }

  /// Returns the xy coordinates of this vector as a [`Vec2`].
  #[must_use]
  #[inline(always)]
  pub const fn xy(&self) -> &Vec2 {
    unsafe { Vec2::from_ptr_unchecked(self.0.as_ptr()) }
  }

  /// Returns the yz coordinates of this vector as a [`Vec2`].
  #[must_use]
  #[inline(always)]
  pub const fn yz(&self) -> &Vec2 {
    unsafe { Vec2::from_ptr_unchecked(self.0.as_ptr().add(1)) }
  }

  /// Returns the zw coordinates of this vector as a [`Vec2`].
  #[must_use]
  #[inline(always)]
  pub const fn zw(&self) -> &Vec2 {
    unsafe { Vec2::from_ptr_unchecked(self.0.as_ptr().add(2)) }
  }

  /// Returns the xyz coordinates of this vector as a [`Vec2`].
  #[must_use]
  #[inline(always)]
  pub const fn xyz(&self) -> &Vec3 {
    unsafe { Vec3::from_ptr_unchecked(self.0.as_ptr()) }
  }

  /// Returns the yz coordinates of this vector as a [`Vec2`].
  #[must_use]
  #[inline(always)]
  pub const fn yzw(&self) -> &Vec3 {
    unsafe { Vec3::from_ptr_unchecked(self.0.as_ptr().add(1)) }
  }

  /// Returns a mutable reference to the X-coordinate of this 3-component vector.
  #[must_use]
  #[inline(always)]
  pub fn x_mut(&mut self) -> &mut f32 {
    unsafe { &mut *self.0.as_mut_ptr() }
  }

  /// Returns a mutable reference to the Y-coordinate of this 3-component vector.
  #[must_use]
  #[inline(always)]
  pub fn y_mut(&mut self) -> &mut f32 {
    unsafe { &mut *self.0.as_mut_ptr().add(1) }
  }

  /// Returns a mutable reference to the Z-coordinate of this 3-component vector.
  #[must_use]
  #[inline(always)]
  pub fn z_mut(&mut self) -> &mut f32 {
    unsafe { &mut *self.0.as_mut_ptr().add(2) }
  }

  /// Returns a mutable reference to the W-coordinate of this 3-component vector.
  #[must_use]
  #[inline(always)]
  pub fn w_mut(&mut self) -> &mut f32 {
    unsafe { &mut *self.0.as_mut_ptr().add(3) }
  }

  /// Returns a mutable reference to the xy coordinates of this vector.
  #[must_use]
  #[inline(always)]
  pub fn xy_mut(&mut self) -> &mut Vec2 {
    unsafe { Vec2::from_mut_ptr_unchecked(self.0.as_mut_ptr()) }
  }

  /// Returns a mutable reference to the yz coordinates of this vector.
  #[must_use]
  #[inline(always)]
  pub fn yz_mut(&mut self) -> &mut Vec2 {
    unsafe { Vec2::from_mut_ptr_unchecked(self.0.as_mut_ptr().add(1)) }
  }

  /// Returns a mutable reference to the zw coordinates of this vector.
  #[must_use]
  #[inline(always)]
  pub fn zw_mut(&mut self) -> &mut Vec2 {
    unsafe { Vec2::from_mut_ptr_unchecked(self.0.as_mut_ptr().add(2)) }
  }

  /// Returns a mutable reference to the yz coordinates of this vector.
  #[must_use]
  #[inline(always)]
  pub fn xyz_mut(&mut self) -> &mut Vec3 {
    unsafe { Vec3::from_mut_ptr_unchecked(self.0.as_mut_ptr()) }
  }

  /// Returns a mutable reference to the zw coordinates of this vector.
  #[must_use]
  #[inline(always)]
  pub fn yzw_mut(&mut self) -> &mut Vec3 {
    unsafe { Vec3::from_mut_ptr_unchecked(self.0.as_mut_ptr().add(1)) }
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

  /// Sets the z-component
  ///
  /// # Arguments
  ///
  /// * `z` - theZ-component
  #[inline(always)]
  pub fn set_z(&mut self, z: f32) {
    unsafe { *self.0.as_mut_ptr().add(2) = z }
  }

  /// Sets the w-component
  ///
  /// # Arguments
  ///
  /// * `w` - the W-component
  #[inline(always)]
  pub fn set_w(&mut self, w: f32) {
    unsafe { *self.0.as_mut_ptr().add(3) = w }
  }

  /// Sets the X and Y components of this vector
  ///
  /// # Arguments
  ///
  /// * `xy` - the X and Y components of the [`Vec4`]
  #[inline(always)]
  pub fn set_xy(&mut self, xy: &Vec2) {
    self.xy_mut().set(xy)
  }

  /// Sets the Y and Z components of this vector
  ///
  /// # Arguments
  ///
  /// * `yz` - the Y and Z components of the [`Vec4`]
  #[inline(always)]
  pub fn set_yz(&mut self, yz: &Vec2) {
    self.yz_mut().set(yz)
  }

  /// Sets the Z and W components of this vector
  ///
  /// # Arguments
  ///
  /// * `zw` - the Z and W components of the [`Vec4`]
  #[inline(always)]
  pub fn set_zw(&mut self, zw: &Vec2) {
    self.zw_mut().set(zw)
  }

  /// Sets the X, Y, and Z components of this vector
  ///
  /// # Arguments
  ///
  /// * `xyz` - the X, Y, and Z components of the [`Vec4`]`
  #[inline(always)]
  pub fn set_xyz(&mut self, xyz: &Vec3) {
    self.xyz_mut().set(xyz)
  }

  /// Sets the Y, Z and W components of this vector
  ///
  /// # Arguments
  ///
  /// * `yzw` - the Y, Z, and W components of the [`Vec4`]
  #[inline(always)]
  pub fn set_yzw(&mut self, yzw: &Vec3) {
    self.yzw_mut().set(yzw)
  }

  /// Sets all the components of this vector the values from other.
  ///
  /// # Arguments
  ///
  /// * `other` - the other [`Vec3`] to set.
  pub fn set(&mut self, other: &Vec4) {
    let src_ptr = other.as_ptr();
    let dest_ptr = self.0.as_mut_ptr();

    unsafe {
      for i in 0..4 {
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
  /// Where possible, consider using [`Vec4::square_magnitude`] as this will
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

  /// Returns normalized vector.
  #[must_use]
  pub fn normalized(&self) -> Vector4 {
    self / self.magnitude()
  }

  /// Returns whether all components of this Vec are finite.
  ///
  /// # Example
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::vec::Vector4;
  ///
  /// let finite = Vector4::UNIT_X;
  /// let not_finite = Vector4::new(f32::INFINITY, 3.0, 0.0, -1.0);
  ///
  /// assert!(finite.is_finite());
  /// assert!(!not_finite.is_finite());
  /// ```
  #[must_use]
  pub fn is_finite(&self) -> bool {
    self.x().is_finite() && self.y().is_finite() && self.z().is_finite() && self.w().is_finite()
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
  /// # use alloy::math::vec::Vector4;
  ///
  /// let finite = Vector4::UNIT_X;
  /// let not_finite = Vector4::new(f32::INFINITY, 3.0, 0.0, -1.0);
  ///
  /// assert!(!finite.is_infinite());
  /// assert!(not_finite.is_infinite());
  /// ```
  #[must_use]
  pub fn is_infinite(&self) -> bool {
    self.x().is_infinite()
      || self.y().is_infinite()
      || self.z().is_infinite()
      || self.w().is_infinite()
  }

  /// Returns whether any component of this vec are nan.
  #[must_use]
  pub fn is_nan(&self) -> bool {
    self.x().is_nan() || self.y().is_nan() || self.z().is_nan() || self.w().is_nan()
  }

  /// Computes the absolute value of `self`
  #[must_use]
  pub fn abs(&self) -> Vector4 {
    Vector4 {
      x: self.x().abs(),
      y: self.y().abs(),
      z: self.z().abs(),
      w: self.w().abs(),
    }
  }
}

impl AsRef<[f32]> for Vec4 {
  #[must_use]
  #[inline(always)]
  fn as_ref(&self) -> &[f32] {
    &self.0
  }
}

impl AsMut<[f32]> for Vec4 {
  #[must_use]
  #[inline(always)]
  fn as_mut(&mut self) -> &mut [f32] {
    &mut self.0
  }
}

impl Borrow<[f32]> for Vec4 {
  fn borrow(&self) -> &[f32] {
    &self.0
  }
}

impl BorrowMut<[f32]> for Vec4 {
  fn borrow_mut(&mut self) -> &mut [f32] {
    &mut self.0
  }
}

impl Midpoint for Vec4 {
  type Output = Vector4;

  #[must_use]
  fn midpoint(&self, other: &Self) -> Self::Output {
    Vector4 {
      x: (self.x() + other.x()) * 0.5,
      y: (self.y() + other.y()) * 0.5,
      z: (self.z() + other.z()) * 0.5,
      w: (self.w() + other.w()) * 0.5,
    }
  }
}

impl Near for Vec4 {
  #[must_use]
  fn near(&self, other: &Self, tolerance: &f32) -> bool {
    self.x().near(&other.x(), tolerance)
      && self.y().near(&other.y(), tolerance)
      && self.z().near(&other.z(), tolerance)
      && self.w().near(&other.w(), tolerance)
  }
}

impl AlmostEq for Vec4 {
  #[must_use]
  fn almost_eq(&self, other: &Self) -> bool {
    const EPSILON: f32 = 10.0 * std::f32::EPSILON;
    self.near(other, &EPSILON)
  }
}

impl<I> Index<I> for Vec4
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

impl<I> IndexMut<I> for Vec4
where
  I: SliceIndex<[f32]>,
{
  #[must_use]
  #[inline(always)]
  fn index_mut(&mut self, index: I) -> &mut Self::Output {
    self.0.index_mut(index)
  }
}

impl Dot for Vec4 {
  type Output = f32;

  #[must_use]
  fn dot(&self, rhs: &Self) -> Self::Output {
    self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z() + self.w() * rhs.w()
  }
}

impl Cross for Vec4 {
  type Output = Vector4;

  #[must_use]
  fn cross(&self, other: &Self) -> Self::Output {
    Vector4 {
      x: self.y() * other.z() - self.z() * other.y(),
      y: self.z() * other.x() - self.x() * other.z(),
      z: self.x() * other.y() - self.y() * other.x(),
      w: 0.0,
    }
  }
}

impl Neg for &'_ Vec4 {
  type Output = Vector4;

  #[must_use]
  fn neg(self) -> Self::Output {
    Vector4 {
      x: -self.x(),
      y: -self.y(),
      z: -self.z(),
      w: -self.w(),
    }
  }
}

impl Add for &'_ Vec4 {
  type Output = Vector4;

  #[must_use]
  fn add(self, rhs: Self) -> Self::Output {
    Vector4 {
      x: self.x() + rhs.x(),
      y: self.y() + rhs.y(),
      z: self.z() + rhs.z(),
      w: self.w() + rhs.w(),
    }
  }
}

impl Sub for &'_ Vec4 {
  type Output = Vector4;

  #[must_use]
  fn sub(self, rhs: Self) -> Self::Output {
    Vector4 {
      x: self.x() - rhs.x(),
      y: self.y() - rhs.y(),
      z: self.z() - rhs.z(),
      w: self.w() - rhs.w(),
    }
  }
}

impl Mul<f32> for &'_ Vec4 {
  type Output = Vector4;

  #[must_use]
  fn mul(self, rhs: f32) -> Self::Output {
    Vector4 {
      x: self.x() * rhs,
      y: self.y() * rhs,
      z: self.z() * rhs,
      w: self.w() * rhs,
    }
  }
}

impl Mul<&'_ Vec4> for f32 {
  type Output = Vector4;

  #[must_use]
  fn mul(self, rhs: &'_ Vec4) -> Self::Output {
    Vector4 {
      x: self * rhs.x(),
      y: self * rhs.y(),
      z: self * rhs.z(),
      w: self * rhs.w(),
    }
  }
}

impl Div<f32> for &'_ Vec4 {
  type Output = Vector4;

  #[must_use]
  fn div(self, rhs: f32) -> Self::Output {
    let inverse = 1.0 / rhs;
    Vector4 {
      x: self.x() * inverse,
      y: self.y() * inverse,
      z: self.z() * inverse,
      w: self.w() * inverse,
    }
  }
}

impl Rem<f32> for &'_ Vec4 {
  type Output = Vector4;

  #[must_use]
  fn rem(self, rhs: f32) -> Self::Output {
    Vector4 {
      x: self.x().rem(rhs),
      y: self.y().rem(rhs),
      z: self.z().rem(rhs),
      w: self.w().rem(rhs),
    }
  }
}

impl AddAssign<&Vec4> for Vec4 {
  fn add_assign(&mut self, rhs: &Vec4) {
    let dest_ptr = self.0.as_mut_ptr();
    let src_ptr = rhs.0.as_ptr();

    unsafe {
      for i in 0..4 {
        *dest_ptr.add(i) += *src_ptr.add(i)
      }
    }
  }
}

impl SubAssign<&Vec4> for Vec4 {
  fn sub_assign(&mut self, rhs: &Vec4) {
    let dest_ptr = self.0.as_mut_ptr();
    let src_ptr = rhs.0.as_ptr();

    unsafe {
      for i in 0..4 {
        *dest_ptr.add(i) -= *src_ptr.add(i)
      }
    }
  }
}

impl MulAssign<f32> for Vec4 {
  fn mul_assign(&mut self, rhs: f32) {
    let dest_ptr = self.0.as_mut_ptr();

    unsafe {
      for i in 0..4 {
        *dest_ptr.add(i) *= rhs
      }
    }
  }
}

impl DivAssign<f32> for Vec4 {
  fn div_assign(&mut self, rhs: f32) {
    let dest_ptr = self.0.as_mut_ptr();

    let inverse = 1.0 / rhs;
    unsafe {
      for i in 0..4 {
        *dest_ptr.add(i) *= inverse
      }
    }
  }
}

impl RemAssign<f32> for Vec4 {
  fn rem_assign(&mut self, rhs: f32) {
    let dest_ptr = self.0.as_mut_ptr();

    unsafe {
      for i in 0..4 {
        *dest_ptr.add(i) %= rhs
      }
    }
  }
}

impl fmt::Debug for Vec4 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Vec4")
      .field("x", &self.x())
      .field("y", &self.y())
      .field("z", &self.z())
      .field("w", &self.w())
      .finish()
  }
}

impl fmt::Display for Vec4 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "[{}, {}, {}, {}]",
      self.x(),
      self.y(),
      self.z(),
      self.w()
    )
  }
}

/// An owning representation of a 4-component [Euclidean vector].
///
/// Like [`Vec4`], the [`Vector4`] object represents a [Euclidean vector] in
/// 4D. Unlike the [`Vec4`], this is an owning representation that stores the
/// actual content of the vector.
///
/// [Euclidean vector]: https://en.wikipedia.org/wiki/Euclidean_vector
#[repr(C)]
#[repr(align(16))]
#[derive(Clone, Copy, Default, PartialEq, PartialOrd, Debug)]
pub struct Vector4 {
  /// The X-component of the vector.
  pub x: f32,
  /// The Y-component of the vector.
  pub y: f32,
  /// The Z-component of the vector.
  pub z: f32,
  /// The W-component of the vector.
  pub w: f32,
}

impl Vector4 {
  /// A constant for a [Null vector], which has magnitude 0 and exists at the
  /// [origin].
  ///
  /// [Null vector]: https://en.wikipedia.org/wiki/Null_vector
  /// [origin]: https://en.wikipedia.org/wiki/Origin_(mathematics)
  pub const ZERO: Vector4 = Vector4::new(0.0, 0.0, 0.0, 0.0);

  /// A constant for a [unit vector] in the positive X-direction.
  ///
  /// [unit vector]: https://en.wikipedia.org/wiki/Unit_vector
  pub const UNIT_X: Vector4 = Vector4::new(1.0, 0.0, 0.0, 0.0);

  /// A constant for a [unit vector] in the positive Y-direction.
  ///
  /// [unit vector]: https://en.wikipedia.org/wiki/Unit_vector
  pub const UNIT_Y: Vector4 = Vector4::new(0.0, 1.0, 0.0, 0.0);

  /// A constant for a [unit vector] in the positive Z-direction.
  ///
  /// [unit vector]: https://en.wikipedia.org/wiki/Unit_vector
  pub const UNIT_Z: Vector4 = Vector4::new(0.0, 0.0, 1.0, 0.0);

  /// A constant for a [unit vector] in the positive W-direction.
  ///
  /// [unit vector]: https://en.wikipedia.org/wiki/Unit_vector
  pub const UNIT_W: Vector4 = Vector4::new(0.0, 0.0, 0.0, 1.0);

  /// A constant for a [unit vector] in the negative X-direction.
  ///
  /// [unit vector]: https://en.wikipedia.org/wiki/Unit_vector
  pub const NEG_UNIT_X: Vector4 = Vector4::new(-1.0, 0.0, 0.0, 0.0);

  /// A constant for a [unit vector] in the negative Y-direction.
  ///
  /// [unit vector]: https://en.wikipedia.org/wiki/Unit_vector
  pub const NEG_UNIT_Y: Vector4 = Vector4::new(0.0, -1.0, 0.0, 0.0);

  /// A constant for a [unit vector] in the negative Z-direction.
  ///
  /// [unit vector]: https://en.wikipedia.org/wiki/Unit_vector
  pub const NEG_UNIT_Z: Vector4 = Vector4::new(0.0, 0.0, -1.0, 0.0);

  /// A constant for a [unit vector] in the negative W-direction.
  ///
  /// [unit vector]: https://en.wikipedia.org/wiki/Unit_vector
  pub const NEG_UNIT_W: Vector4 = Vector4::new(0.0, 0.0, 0.0, -1.0);

  /// Constructs this vector from an x, y, z, and w coordinate.
  ///
  /// # Arguments
  ///
  /// * `x` - the x-component
  /// * `y` - the y-component
  /// * `z` - the z-component
  /// * `w` - the w-component
  #[must_use]
  #[inline(always)]
  pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
    Self { x, y, z, w }
  }

  /// Constructs this vector with a uniform value `v`.
  ///
  /// # Arguments
  ///
  /// * `v` - the value to uniformly apply
  #[must_use]
  #[inline(always)]
  pub const fn uniform(v: f32) -> Self {
    Self::new(v, v, v, v)
  }

  /// Constructs this vector from a 4-component [`f32`] array.
  ///
  /// # Arguments
  ///
  /// * `array` - an array containing 4 [`f32`] values.
  #[must_use]
  #[inline(always)]
  pub const fn from_array(array: &[f32; 4]) -> Self {
    Self::new(array[0], array[1], array[2], array[3])
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
    if slice.len() != 4 {
      None
    } else {
      Some(Self {
        x: slice[0],
        y: slice[1],
        z: slice[3],
        w: slice[4],
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
    debug_assert!(slice.len() == 4);
    Self::from_ptr(slice.as_ptr())
  }

  /// Constructs this vector from a [`Vec4`]
  ///
  /// # Arguments
  ///
  /// * `other` - the other vector
  #[must_use]
  pub const fn from_vec4(other: &Vec4) -> Self {
    Self {
      x: other.x(),
      y: other.y(),
      z: other.z(),
      w: other.w(),
    }
  }

  /// Constructs this vector from a [`Vec4i`]
  ///
  /// # Arguments
  ///
  /// * `other` - the other vector
  #[must_use]
  pub fn from_vec4i(other: &Vec4i) -> Self {
    Self {
      x: other.x() as f32,
      y: other.y() as f32,
      z: other.z() as f32,
      w: other.w() as f32,
    }
  }

  /// Constructs this vector from a [`Vec4u`]
  ///
  /// # Arguments
  ///
  /// * `other` - the other vector
  #[must_use]
  pub fn from_vec4u(other: &Vec4u) -> Self {
    Self {
      x: other.x() as f32,
      y: other.y() as f32,
      z: other.z() as f32,
      w: other.w() as f32,
    }
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
  /// contiguous sequence of 4 [`f32`] values.
  #[must_use]
  #[inline(always)]
  pub const unsafe fn from_ptr(ptr: *const f32) -> Self {
    Self::new(*ptr, *ptr.add(1), *ptr.add(2), *ptr.add(3))
  }

  /// Returns this vector as a [`Vec4`].
  #[must_use]
  #[inline(always)]
  pub const fn as_vec4(&self) -> &Vec4 {
    // SAFETY:
    //
    //   Vector4 is repr(C) and thus points to two contiguous elements
    //   of type and align of `f32`. The only pointer capable of accessing both
    //   entries within its memory region is a pointer to itself (`*const _`).
    //   Thus, we alias this to `f32` -- which under `repr(C)` points to the
    //   first element, and has proper reachability into its neighbor-element.
    unsafe {
      std::mem::transmute(std::slice::from_raw_parts(
        self as *const _ as *const f32,
        4,
      ))
    }
  }

  /// Returns this vector as a mutable [`Vec4`].
  #[must_use]
  #[inline(always)]
  pub fn as_mut_vec4(&mut self) -> &mut Vec4 {
    // SAFETY: See explanation in Borrow<Vec4>
    unsafe {
      std::mem::transmute(std::slice::from_raw_parts_mut(
        self as *mut _ as *mut f32,
        4,
      ))
    }
  }

  /// Returns this vector as a slice of [`f32`].
  #[must_use]
  #[inline(always)]
  pub const fn as_slice(&self) -> &[f32] {
    self.as_vec4().as_slice()
  }

  /// Returns this vector as a mutable slice of [`f32`].
  #[must_use]
  #[inline(always)]
  pub fn as_mut_slice(&mut self) -> &mut [f32] {
    self.as_mut_vec4().as_mut_slice()
  }
}

impl From<&'_ Vec4> for Vector4 {
  #[must_use]
  #[inline(always)]
  fn from(value: &'_ Vec4) -> Self {
    Self::from_vec4(value)
  }
}

impl From<&'_ Vec4i> for Vector4 {
  #[must_use]
  #[inline]
  fn from(value: &'_ Vec4i) -> Self {
    Self::from_vec4i(value)
  }
}

impl From<&'_ Vec4u> for Vector4 {
  #[must_use]
  #[inline]
  fn from(value: &'_ Vec4u) -> Self {
    Self::from_vec4u(value)
  }
}

impl Deref for Vector4 {
  type Target = Vec4;

  #[must_use]
  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    self.borrow()
  }
}

impl DerefMut for Vector4 {
  #[must_use]
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.borrow_mut()
  }
}

impl AsRef<Vec4> for Vector4 {
  #[must_use]
  #[inline(always)]
  fn as_ref(&self) -> &Vec4 {
    self.as_vec4()
  }
}

impl AsMut<Vec4> for Vector4 {
  #[inline(always)]
  fn as_mut(&mut self) -> &mut Vec4 {
    self.as_mut_vec4()
  }
}

impl Borrow<Vec4> for Vector4 {
  #[must_use]
  #[inline(always)]
  fn borrow(&self) -> &Vec4 {
    self.as_vec4()
  }
}

impl BorrowMut<Vec4> for Vector4 {
  #[must_use]
  #[inline(always)]
  fn borrow_mut(&mut self) -> &mut Vec4 {
    self.as_mut_vec4()
  }
}

impl ToOwned for Vec4 {
  type Owned = Vector4;

  #[must_use]
  #[inline(always)]
  fn to_owned(&self) -> Self::Owned {
    Vector4 {
      x: self.x(),
      y: self.y(),
      z: self.z(),
      w: self.w(),
    }
  }
}

impl Add for &Vector4 {
  type Output = Vector4;

  #[must_use]
  #[inline(always)]
  fn add(self, rhs: Self) -> Self::Output {
    self.as_vec4().add(rhs.as_vec4())
  }
}

impl Add for Vector4 {
  type Output = Vector4;

  #[must_use]
  #[inline(always)]
  fn add(self, rhs: Self) -> Self::Output {
    self.add(rhs.as_vec4())
  }
}

impl Add<&Vec4> for &Vector4 {
  type Output = Vector4;

  #[must_use]
  #[inline(always)]
  fn add(self, rhs: &Vec4) -> Self::Output {
    self.as_vec4().add(rhs)
  }
}

impl Add<&Vector4> for &'_ Vec4 {
  type Output = Vector4;

  #[must_use]
  #[inline(always)]
  fn add(self, rhs: &Vector4) -> Self::Output {
    self.add(rhs.as_vec4())
  }
}

impl Add<Vector4> for &Vec4 {
  type Output = Vector4;

  #[must_use]
  #[inline(always)]
  fn add(self, rhs: Vector4) -> Self::Output {
    // Addition is commutative, so reordering operations is safe
    rhs.add(self)
  }
}

impl Add<&Vec4> for Vector4 {
  type Output = Vector4;

  #[must_use]
  fn add(mut self, rhs: &Vec4) -> Self::Output {
    // Repurpose 'self' for the output, to save space (1 less lifetime)
    let dest_ptr = self.0.as_mut_ptr();
    let src_ptr = rhs.0.as_ptr();

    unsafe {
      for i in 0..4 {
        *dest_ptr.add(i) += *src_ptr.add(i)
      }
    }
    self
  }
}

impl Add<&Vector4> for Vector4 {
  type Output = Vector4;

  #[must_use]
  #[inline(always)]
  fn add(self, rhs: &Vector4) -> Self::Output {
    // Addition is commutative, so reordering operations is safe
    rhs.add(self.as_vec4())
  }
}

impl Add<Vector4> for &Vector4 {
  type Output = Vector4;

  #[must_use]
  #[inline(always)]
  fn add(self, rhs: Vector4) -> Self::Output {
    // Addition is commutative, so reordering operations is safe
    rhs.as_vec4().add(self)
  }
}

impl Sub for &Vector4 {
  type Output = Vector4;

  #[must_use]
  #[inline(always)]
  fn sub(self, rhs: Self) -> Self::Output {
    self.as_vec4().sub(rhs.as_vec4())
  }
}

impl Sub for Vector4 {
  type Output = Vector4;

  #[must_use]
  #[inline(always)]
  fn sub(self, rhs: Self) -> Self::Output {
    self.sub(rhs.as_vec4())
  }
}

impl Sub<&Vec4> for &Vector4 {
  type Output = Vector4;

  #[must_use]
  #[inline(always)]
  fn sub(self, rhs: &Vec4) -> Self::Output {
    self.as_vec4().sub(rhs)
  }
}

impl Sub<&Vector4> for &'_ Vec4 {
  type Output = Vector4;

  #[must_use]
  #[inline(always)]
  fn sub(self, rhs: &Vector4) -> Self::Output {
    self.sub(rhs.as_vec4())
  }
}

impl Sub<Vector4> for &Vec4 {
  type Output = Vector4;

  #[must_use]
  #[inline(always)]
  fn sub(self, rhs: Vector4) -> Self::Output {
    self.sub(rhs.as_vec4())
  }
}

impl Sub<&Vec4> for Vector4 {
  type Output = Vector4;

  #[must_use]
  fn sub(mut self, rhs: &Vec4) -> Self::Output {
    // Repurpose 'self' for the output, to save space (1 less lifetime)
    let dest_ptr = self.0.as_mut_ptr();
    let src_ptr = rhs.0.as_ptr();

    unsafe {
      for i in 0..4 {
        *dest_ptr.add(i) -= *src_ptr.add(i)
      }
    }
    self
  }
}

impl Sub<&Vector4> for Vector4 {
  type Output = Vector4;

  #[must_use]
  #[inline(always)]
  fn sub(self, rhs: &Vector4) -> Self::Output {
    self.sub(rhs.as_vec4())
  }
}

impl Sub<Vector4> for &Vector4 {
  type Output = Vector4;

  #[must_use]
  #[inline(always)]
  fn sub(self, rhs: Vector4) -> Self::Output {
    self.sub(rhs.as_vec4())
  }
}

impl Mul<f32> for Vector4 {
  type Output = Vector4;

  #[must_use]
  #[inline(always)]
  fn mul(mut self, rhs: f32) -> Self::Output {
    self.as_mut_vec4().mul_assign(rhs);
    self
  }
}

impl Mul<f32> for &Vector4 {
  type Output = Vector4;

  #[must_use]
  #[inline(always)]
  fn mul(self, rhs: f32) -> Self::Output {
    self.as_vec4().mul(rhs)
  }
}

impl Mul<Vector4> for f32 {
  type Output = Vector4;

  #[must_use]
  #[inline(always)]
  fn mul(self, mut rhs: Vector4) -> Self::Output {
    rhs.as_mut_vec4().mul_assign(self);
    rhs
  }
}

impl Mul<&Vector4> for f32 {
  type Output = Vector4;

  #[must_use]
  #[inline(always)]
  fn mul(self, rhs: &Vector4) -> Self::Output {
    rhs.as_vec4().mul(self)
  }
}

impl Div<f32> for Vector4 {
  type Output = Vector4;

  #[must_use]
  #[inline(always)]
  fn div(mut self, rhs: f32) -> Self::Output {
    self.as_mut_vec4().div_assign(rhs);
    self
  }
}

impl Div<f32> for &Vector4 {
  type Output = Vector4;

  #[must_use]
  #[inline(always)]
  fn div(self, rhs: f32) -> Self::Output {
    self.as_vec4().div(rhs)
  }
}

impl Rem<f32> for Vector4 {
  type Output = Vector4;

  #[must_use]
  #[inline(always)]
  fn rem(mut self, rhs: f32) -> Self::Output {
    self.as_mut_vec4().rem_assign(rhs);
    self
  }
}

impl Rem<f32> for &Vector4 {
  type Output = Vector4;

  #[must_use]
  #[inline(always)]
  fn rem(self, rhs: f32) -> Self::Output {
    self.as_vec4().rem(rhs)
  }
}

impl AddAssign for Vector4 {
  #[inline(always)]
  fn add_assign(&mut self, rhs: Self) {
    self.as_mut_vec4().add_assign(&rhs)
  }
}

impl AddAssign<&Vector4> for Vector4 {
  #[inline(always)]
  fn add_assign(&mut self, rhs: &Self) {
    self.as_mut_vec4().add_assign(rhs)
  }
}

impl AddAssign<&Vec4> for Vector4 {
  #[inline(always)]
  fn add_assign(&mut self, rhs: &Vec4) {
    self.as_mut_vec4().add_assign(rhs)
  }
}

impl SubAssign for Vector4 {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: Self) {
    self.as_mut_vec4().sub_assign(&rhs)
  }
}

impl SubAssign<&Vector4> for Vector4 {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: &Self) {
    self.as_mut_vec4().sub_assign(rhs)
  }
}

impl SubAssign<&Vec4> for Vector4 {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: &Vec4) {
    self.as_mut_vec4().sub_assign(rhs)
  }
}

impl MulAssign<f32> for Vector4 {
  #[inline(always)]
  fn mul_assign(&mut self, rhs: f32) {
    self.as_mut_vec4().mul_assign(rhs)
  }
}

impl DivAssign<f32> for Vector4 {
  #[inline(always)]
  fn div_assign(&mut self, rhs: f32) {
    self.as_mut_vec4().div_assign(rhs)
  }
}

impl RemAssign<f32> for Vector4 {
  #[inline(always)]
  fn rem_assign(&mut self, rhs: f32) {
    self.as_mut_vec4().rem_assign(rhs)
  }
}

impl fmt::Display for Vector4 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "[{}, {}, {}, {}]", self.x, self.y, self.z, self.w)
  }
}

#[cfg(test)]
mod test {
  use crate::cmp::AlmostEq;

  use super::*;

  #[test]
  fn test_vec4() {
    let vec = Vector4::new(4.0, 2.0, 0.0, 0.0);

    use crate::cmp::AlmostEq;
    let magnitude = vec.square_magnitude();

    assert!(magnitude.almost_eq(&20.0))
  }

  #[test]
  fn test_identity() {
    let vec = Vector4::new(4.0, 2.0, 0.0, 0.0);

    assert_eq!(vec.as_ptr(), &vec[0]);
    assert_eq!(vec.as_ptr(), &vec.x);
  }

  #[test]
  fn test_iter() {
    let mut vec = Vector4::new(4.0, 2.0, 1.0, 0.0);

    for v in vec.iter_mut() {
      *v = *v * 2.0
    }

    assert!(vec.x.almost_eq(&8.0), "x = {}", vec.x);
    assert!(vec.y.almost_eq(&4.0), "y = {}", vec.y);
    assert!(vec.z.almost_eq(&2.0), "z = {}", vec.z);
    assert!(vec.w.almost_eq(&0.0), "w = {}", vec.w);
  }
}
