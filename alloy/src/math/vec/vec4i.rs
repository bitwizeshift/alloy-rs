use crate::math::vec::{Vec2i, Vec3i};

use std::borrow::{Borrow, BorrowMut};
use std::fmt;
use std::ops::{
  Add, AddAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Rem,
  RemAssign, Sub, SubAssign,
};
use std::slice::SliceIndex;

/// A 4-component non-owning view of an integral [Euclidean vector].
///
/// [Euclidean vector]: https://en.wikipedia.org/wiki/Euclidean_vector
///
/// # Guarantees
///
/// This type has the strict requirement that it can only reference a
/// 4-component slice of [`i32`] values. It is guaranteed to never refer to
/// more or less than 4 entries.
///
/// # Relation to [`Vector4i`]
///
/// [`Vec4i`] is a non-owning equivalent of [`Vector4i`]. This enables non-vector
/// types to either [`Deref`] or provide conversion-related utilities into
/// [`Vec4i`] types to be able to access and benefit from vector operations.
#[repr(transparent)]
#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct Vec4i([i32]);

impl Vec4i {
  /// Forms a reference to a [`Vec4i`] from a 4-component [`i32`] array.
  ///
  /// This function is identical to [`from_slice_unchecked`], except it is not
  /// marked `unsafe`.
  ///
  /// [`from_slice_unchecked`]: Self::from_slice_unchecked
  ///
  /// # Arguments
  ///
  /// * `array` - an array containing 4 [`i32`] values.
  ///
  /// # Examples
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::vec::Vec4i;
  /// let array: [i32; 4] = [1, 42, 314, 9];
  ///
  /// let vec = Vec4i::from_array(&array);
  ///
  /// assert_eq!(vec.as_ptr(), array.as_ptr());
  /// assert_eq!(vec.x(), array[0]);
  /// assert_eq!(vec.y(), array[1]);
  /// assert_eq!(vec.z(), array[2]);
  /// assert_eq!(vec.w(), array[3]);
  /// ```
  #[must_use]
  #[inline(always)]
  pub const fn from_array(array: &[i32; 4]) -> &Self {
    // SAFETY: `array` is guaranteed to be 4-components
    unsafe { std::mem::transmute(array.as_slice()) }
  }

  /// Forms a mutable reference to a [`Vec4i`] from a 4-component [`i32`] array.
  ///
  /// This function is identical to [`from_mut_slice_unchecked`], except it is
  /// not marked `unsafe`.
  ///
  /// [`from_mut_slice_unchecked`]: Self::from_mut_slice_unchecked
  ///
  /// # Arguments
  ///
  /// * `array` - an array containing 4 [`i32`] values.
  ///
  /// # Examples
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::vec::Vec4i;
  /// let mut array: [i32; 4] = [1, 42, 314, 9];
  ///
  /// let vec = Vec4i::from_mut_array(&mut array);
  ///
  /// assert_eq!(vec.as_ptr(), array.as_ptr());
  /// ```
  #[must_use]
  #[inline(always)]
  pub fn from_mut_array(array: &mut [i32; 4]) -> &mut Self {
    // SAFETY: `array` is guaranteed to be 4-components
    unsafe { std::mem::transmute(array.as_mut_slice()) }
  }

  /// Forms a reference to a [`Vec4i`] from a slice of [`i32`].
  ///
  /// This requires that `slice.len() == 4`, otherwise this returns [`None`].
  ///
  /// # Arguments
  ///
  /// * `slice` - the slice of [`i32`]s.
  ///
  /// # Examples
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::vec::Vec4i;
  /// let slice = &[1, 42, 314, 9];
  ///
  /// let vec = Vec4i::from_slice(slice).unwrap();
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
  /// # use alloy::math::vec::Vec4i;
  /// let slice = &[1];
  ///
  /// let vec = Vec4i::from_slice(slice);
  ///
  /// assert_eq!(vec, None);
  /// ```
  #[must_use]
  pub const fn from_slice(slice: &[i32]) -> Option<&Self> {
    if slice.len() == 4 {
      // SAFETY: Vec4 is transparent, and implemented directly in terms of a
      //         slice of i32s. The representation is the same, and thus valid.
      //         This is implemented symmetrically to `OsStr`.
      Some(unsafe { std::mem::transmute(slice) })
    } else {
      None
    }
  }

  /// Forms a mutable reference to a [`Vec4i`] from a mutable slice of [`i32`].
  ///
  /// This requires that `slice.len() == 4`, otherwise this returns [`None`].
  ///
  /// # Arguments
  ///
  /// * `slice` - the mutable slice of [`i32`]s.
  ///
  /// # Examples
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::vec::Vec4i;
  /// let slice = &mut [1, 42, 314, 9];
  ///
  /// let vec = Vec4i::from_mut_slice(slice).unwrap();
  ///
  /// assert_eq!(vec.as_ptr(), slice.as_ptr());
  /// ```
  ///
  /// Invalid size:
  ///
  /// ```rust
  /// # use alloy::math::vec::Vec4i;
  /// let slice = &mut [1];
  ///
  /// let vec = Vec4i::from_mut_slice(slice);
  ///
  /// assert_eq!(vec, None);
  /// ```
  #[must_use]
  pub fn from_mut_slice(slice: &mut [i32]) -> Option<&mut Self> {
    if slice.len() == 4 {
      // SAFETY: Vec4 is transparent, and implemented directly in terms of a
      //         slice of i32s. The representation is the same, and thus valid.
      //         This is implemented symmetrically to `OsStr`.
      Some(unsafe { std::mem::transmute(slice) })
    } else {
      None
    }
  }

  /// Forms a reference to a [`Vec4i`] from a slice of [`i32`] that is assumed to
  /// contain two values.
  ///
  /// # Arguments
  ///
  /// * `slice` - the slice of [`i32`]s.
  ///
  /// # Safety
  ///
  /// `slice.len()` must be equal to `2`.
  #[must_use]
  #[inline(always)]
  pub const unsafe fn from_slice_unchecked(slice: &[i32]) -> &Self {
    debug_assert!(slice.len() == 4);
    std::mem::transmute(slice)
  }

  /// Forms a mutable reference to a [`Vec4i`] from a slice of [`i32`] that is
  /// assumed to contain two values.
  ///
  /// # Arguments
  ///
  /// * `slice` - the mutable slice of [`i32`]s.
  ///
  /// # Safety
  ///
  /// `slice.len()` must be equal to `2`.
  #[must_use]
  #[inline(always)]
  pub unsafe fn from_mut_slice_unchecked(slice: &mut [i32]) -> &mut Self {
    debug_assert!(slice.len() == 4);
    std::mem::transmute(slice)
  }

  /// Forms a reference to a [`Vec2i`] from a pointer to a contiguous sequence
  /// of at least two [`i32`]s.
  ///
  /// # Arguments
  ///
  /// * `ptr` - the pointer to a sequence of [`i32`] values
  ///
  /// # Safety
  ///
  /// `ptr` must point to an allocated object that references at least two
  /// entries
  #[must_use]
  #[inline(always)]
  pub const unsafe fn from_ptr_unchecked<'a>(ptr: *const i32) -> &'a Vec4i {
    Vec4i::from_slice_unchecked(std::slice::from_raw_parts(ptr, 4))
  }

  /// Forms a mutable reference to a [`Vec3i`] from a pointer to a contiguous
  /// sequence of at least two [`i32`]s.
  ///
  /// # Arguments
  ///
  /// * `ptr` - the pointer to a sequence of [`i32`] values
  ///
  /// # Safety
  ///
  /// `ptr` must point to an allocated object that references at least two
  /// entries
  #[must_use]
  #[inline(always)]
  pub unsafe fn from_mut_ptr_unchecked<'a>(ptr: *mut i32) -> &'a mut Vec4i {
    Vec4i::from_mut_slice_unchecked(std::slice::from_raw_parts_mut(ptr, 4))
  }

  /// Returns this [`Vec4i`] as a slice of [`i32`].
  #[must_use]
  #[inline(always)]
  pub const fn as_slice(&self) -> &[i32] {
    &self.0
  }

  /// Returns this [`Vec4i`] as a mutable slice of [`i32`].
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

  /// Returns the X-coordinate of this 4-component vector.
  #[must_use]
  #[inline(always)]
  pub fn x(&self) -> i32 {
    // SAFETY: Vec4 being of size-4 is an internal invariant
    unsafe { *self.0.as_ptr() }
  }

  /// Returns the Y-coordinate of this 4-component vector.
  #[must_use]
  #[inline(always)]
  pub fn y(&self) -> i32 {
    // SAFETY: Vec4 being of size-4 is an internal invariant
    unsafe { *self.0.as_ptr().add(1) }
  }

  /// Returns the Z-coordinate of this 4-component vector.
  #[must_use]
  #[inline(always)]
  pub fn z(&self) -> i32 {
    // SAFETY: Vec4 being of size-4 is an internal invariant
    unsafe { *self.0.as_ptr().add(2) }
  }

  /// Returns the W-coordinate of this 4-component vector.
  #[must_use]
  #[inline(always)]
  pub fn w(&self) -> i32 {
    // SAFETY: Vec4 being of size-4 is an internal invariant
    unsafe { *self.0.as_ptr().add(3) }
  }

  /// Returns the xy coordinates of this vector as a [`Vec2i`].
  #[must_use]
  #[inline(always)]
  pub const fn xy(&self) -> &Vec2i {
    unsafe { Vec2i::from_ptr_unchecked(self.0.as_ptr()) }
  }

  /// Returns the yz coordinates of this vector as a [`Vec2i`].
  #[must_use]
  #[inline(always)]
  pub const fn yz(&self) -> &Vec2i {
    unsafe { Vec2i::from_ptr_unchecked(self.0.as_ptr().add(1)) }
  }

  /// Returns the zw coordinates of this vector as a [`Vec2i`].
  #[must_use]
  #[inline(always)]
  pub const fn zw(&self) -> &Vec2i {
    unsafe { Vec2i::from_ptr_unchecked(self.0.as_ptr().add(2)) }
  }

  /// Returns the xyz coordinates of this vector as a [`Vec2i`].
  #[must_use]
  #[inline(always)]
  pub const fn xyz(&self) -> &Vec3i {
    unsafe { Vec3i::from_ptr_unchecked(self.0.as_ptr()) }
  }

  /// Returns the yz coordinates of this vector as a [`Vec2i`].
  #[must_use]
  #[inline(always)]
  pub const fn yzw(&self) -> &Vec3i {
    unsafe { Vec3i::from_ptr_unchecked(self.0.as_ptr().add(1)) }
  }

  /// Returns a mutable reference to the X-coordinate of this 3-component vector.
  #[must_use]
  #[inline(always)]
  pub fn x_mut(&mut self) -> &mut i32 {
    unsafe { &mut *self.0.as_mut_ptr() }
  }

  /// Returns a mutable reference to the Y-coordinate of this 3-component vector.
  #[must_use]
  #[inline(always)]
  pub fn y_mut(&mut self) -> &mut i32 {
    unsafe { &mut *self.0.as_mut_ptr().add(1) }
  }

  /// Returns a mutable reference to the Z-coordinate of this 3-component vector.
  #[must_use]
  #[inline(always)]
  pub fn z_mut(&mut self) -> &mut i32 {
    unsafe { &mut *self.0.as_mut_ptr().add(2) }
  }

  /// Returns a mutable reference to the W-coordinate of this 3-component vector.
  #[must_use]
  #[inline(always)]
  pub fn w_mut(&mut self) -> &mut i32 {
    unsafe { &mut *self.0.as_mut_ptr().add(3) }
  }

  /// Returns a mutable reference to the xy coordinates of this vector.
  #[must_use]
  #[inline(always)]
  pub fn xy_mut(&mut self) -> &mut Vec2i {
    unsafe { Vec2i::from_mut_ptr_unchecked(self.0.as_mut_ptr()) }
  }

  /// Returns a mutable reference to the yz coordinates of this vector.
  #[must_use]
  #[inline(always)]
  pub fn yz_mut(&mut self) -> &mut Vec2i {
    unsafe { Vec2i::from_mut_ptr_unchecked(self.0.as_mut_ptr().add(1)) }
  }

  /// Returns a mutable reference to the zw coordinates of this vector.
  #[must_use]
  #[inline(always)]
  pub fn zw_mut(&mut self) -> &mut Vec2i {
    unsafe { Vec2i::from_mut_ptr_unchecked(self.0.as_mut_ptr().add(2)) }
  }

  /// Returns a mutable reference to the yz coordinates of this vector.
  #[must_use]
  #[inline(always)]
  pub fn xyz_mut(&mut self) -> &mut Vec3i {
    unsafe { Vec3i::from_mut_ptr_unchecked(self.0.as_mut_ptr()) }
  }

  /// Returns a mutable reference to the zw coordinates of this vector.
  #[must_use]
  #[inline(always)]
  pub fn yzw_mut(&mut self) -> &mut Vec3i {
    unsafe { Vec3i::from_mut_ptr_unchecked(self.0.as_mut_ptr().add(1)) }
  }

  /// Sets the x-component
  ///
  /// # Arguments
  ///
  /// * `x` - the X-component
  #[inline(always)]
  pub fn set_x(&mut self, x: i32) {
    unsafe { *self.0.as_mut_ptr() = x }
  }

  /// Sets the y-component
  ///
  /// # Arguments
  ///
  /// * `y` - the Y-component
  #[inline(always)]
  pub fn set_y(&mut self, y: i32) {
    unsafe { *self.0.as_mut_ptr().add(1) = y }
  }

  /// Sets the z-component
  ///
  /// # Arguments
  ///
  /// * `z` - theZ-component
  #[inline(always)]
  pub fn set_z(&mut self, z: i32) {
    unsafe { *self.0.as_mut_ptr().add(2) = z }
  }

  /// Sets the w-component
  ///
  /// # Arguments
  ///
  /// * `w` - the W-component
  #[inline(always)]
  pub fn set_w(&mut self, w: i32) {
    unsafe { *self.0.as_mut_ptr().add(3) = w }
  }

  /// Sets the X and Y components of this vector
  ///
  /// # Arguments
  ///
  /// * `xy` - the X and Y components of the [`Vec4i`]
  #[inline(always)]
  pub fn set_xy(&mut self, xy: &Vec2i) {
    self.xy_mut().set(xy)
  }

  /// Sets the Y and Z components of this vector
  ///
  /// # Arguments
  ///
  /// * `yz` - the Y and Z components of the [`Vec4i`]
  #[inline(always)]
  pub fn set_yz(&mut self, yz: &Vec2i) {
    self.yz_mut().set(yz)
  }

  /// Sets the Z and W components of this vector
  ///
  /// # Arguments
  ///
  /// * `zw` - the Z and W components of the [`Vec4i`]
  #[inline(always)]
  pub fn set_zw(&mut self, zw: &Vec2i) {
    self.zw_mut().set(zw)
  }

  /// Sets the X, Y, and Z components of this vector
  ///
  /// # Arguments
  ///
  /// * `xyz` - the X, Y, and Z components of the [`Vec4i`]`
  #[inline(always)]
  pub fn set_xyz(&mut self, xyz: &Vec3i) {
    self.xyz_mut().set(xyz)
  }

  /// Sets the Y, Z and W components of this vector
  ///
  /// # Arguments
  ///
  /// * `yzw` - the Y, Z, and W components of the [`Vec4i`]
  #[inline(always)]
  pub fn set_yzw(&mut self, yzw: &Vec3i) {
    self.yzw_mut().set(yzw)
  }

  /// Sets all the components of this vector the values from other.
  ///
  /// # Arguments
  ///
  /// * `other` - the other [`Vec3i`] to set.
  pub fn set(&mut self, other: &Vec4i) {
    let src_ptr = other.as_ptr();
    let dest_ptr = self.0.as_mut_ptr();

    unsafe {
      for i in 0..4 {
        *dest_ptr.add(i) = *src_ptr.add(i);
      }
    }
  }

  /// Computes the absolute value of `self`
  #[must_use]
  pub fn abs(&self) -> Vector4i {
    Vector4i {
      x: self.x().abs(),
      y: self.y().abs(),
      z: self.z().abs(),
      w: self.w().abs(),
    }
  }
}

impl AsRef<[i32]> for Vec4i {
  #[must_use]
  #[inline(always)]
  fn as_ref(&self) -> &[i32] {
    &self.0
  }
}

impl AsMut<[i32]> for Vec4i {
  #[must_use]
  #[inline(always)]
  fn as_mut(&mut self) -> &mut [i32] {
    &mut self.0
  }
}

impl Borrow<[i32]> for Vec4i {
  #[must_use]
  #[inline(always)]
  fn borrow(&self) -> &[i32] {
    &self.0
  }
}

impl BorrowMut<[i32]> for Vec4i {
  #[must_use]
  #[inline(always)]
  fn borrow_mut(&mut self) -> &mut [i32] {
    &mut self.0
  }
}

impl<I> Index<I> for Vec4i
where
  I: SliceIndex<[i32]>,
{
  type Output = I::Output;

  #[must_use]
  #[inline(always)]
  fn index(&self, index: I) -> &Self::Output {
    self.0.index(index)
  }
}

impl<I> IndexMut<I> for Vec4i
where
  I: SliceIndex<[i32]>,
{
  #[must_use]
  #[inline(always)]
  fn index_mut(&mut self, index: I) -> &mut Self::Output {
    self.0.index_mut(index)
  }
}

impl Add for &'_ Vec4i {
  type Output = Vector4i;

  #[must_use]
  fn add(self, rhs: Self) -> Self::Output {
    Vector4i {
      x: self.x() + rhs.x(),
      y: self.y() + rhs.y(),
      z: self.z() + rhs.z(),
      w: self.w() + rhs.w(),
    }
  }
}

impl AddAssign<&Vec4i> for Vec4i {
  fn add_assign(&mut self, rhs: &Vec4i) {
    let dest_ptr = self.0.as_mut_ptr();
    let src_ptr = rhs.0.as_ptr();

    unsafe {
      for i in 0..4 {
        *dest_ptr.add(i) += *src_ptr.add(i)
      }
    }
  }
}

impl Sub for &'_ Vec4i {
  type Output = Vector4i;

  #[must_use]
  fn sub(self, rhs: Self) -> Self::Output {
    Vector4i {
      x: self.x() - rhs.x(),
      y: self.y() - rhs.y(),
      z: self.z() - rhs.z(),
      w: self.w() - rhs.w(),
    }
  }
}

impl SubAssign<&Vec4i> for Vec4i {
  fn sub_assign(&mut self, rhs: &Vec4i) {
    let dest_ptr = self.0.as_mut_ptr();
    let src_ptr = rhs.0.as_ptr();

    unsafe {
      for i in 0..4 {
        *dest_ptr.add(i) -= *src_ptr.add(i)
      }
    }
  }
}

impl Mul<i32> for &'_ Vec4i {
  type Output = Vector4i;

  #[must_use]
  fn mul(self, rhs: i32) -> Self::Output {
    Vector4i {
      x: self.x() * rhs,
      y: self.y() * rhs,
      z: self.z() * rhs,
      w: self.w() * rhs,
    }
  }
}

impl Mul<&'_ Vec4i> for i32 {
  type Output = Vector4i;

  #[must_use]
  fn mul(self, rhs: &'_ Vec4i) -> Self::Output {
    Vector4i {
      x: self * rhs.x(),
      y: self * rhs.y(),
      z: self * rhs.z(),
      w: self * rhs.w(),
    }
  }
}

impl MulAssign<i32> for Vec4i {
  fn mul_assign(&mut self, rhs: i32) {
    let dest_ptr = self.0.as_mut_ptr();

    unsafe {
      for i in 0..4 {
        *dest_ptr.add(i) *= rhs
      }
    }
  }
}

impl Div<i32> for &'_ Vec4i {
  type Output = Vector4i;

  #[must_use]
  fn div(self, rhs: i32) -> Self::Output {
    Vector4i {
      x: self.x() / rhs,
      y: self.y() / rhs,
      z: self.z() / rhs,
      w: self.w() / rhs,
    }
  }
}

impl DivAssign<i32> for Vec4i {
  fn div_assign(&mut self, rhs: i32) {
    let dest_ptr = self.0.as_mut_ptr();

    unsafe {
      for i in 0..4 {
        *dest_ptr.add(i) /= rhs
      }
    }
  }
}

impl Rem<i32> for &'_ Vec4i {
  type Output = Vector4i;

  #[must_use]
  fn rem(self, rhs: i32) -> Self::Output {
    Vector4i {
      x: self.x().rem(rhs),
      y: self.y().rem(rhs),
      z: self.z().rem(rhs),
      w: self.w().rem(rhs),
    }
  }
}

impl RemAssign<i32> for Vec4i {
  fn rem_assign(&mut self, rhs: i32) {
    let dest_ptr = self.0.as_mut_ptr();

    unsafe {
      for i in 0..4 {
        *dest_ptr.add(i) %= rhs
      }
    }
  }
}

impl Neg for &'_ Vec4i {
  type Output = Vector4i;

  #[must_use]
  fn neg(self) -> Self::Output {
    Vector4i {
      x: -self.x(),
      y: -self.y(),
      z: -self.z(),
      w: -self.w(),
    }
  }
}

impl fmt::Debug for Vec4i {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Vec4i")
      .field("x", &self.x())
      .field("y", &self.y())
      .field("z", &self.z())
      .field("w", &self.w())
      .finish()
  }
}

impl fmt::Display for Vec4i {
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

/// An owning representation of an integral 4-component [Euclidean vector].
///
/// Like [`Vec4i`], the [`Vector4i`] object represents a [Euclidean vector] in
/// 4D. Unlike the [`Vec4i`], this is an owning representation that stores the
/// actual content of the vector.
///
/// [Euclidean vector]: https://en.wikipedia.org/wiki/Euclidean_vector
#[repr(C)]
#[repr(align(16))]
#[derive(Clone, Copy, Default, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
pub struct Vector4i {
  /// The X-component of the vector.
  pub x: i32,
  /// The Y-component of the vector.
  pub y: i32,
  /// The Z-component of the vector.
  pub z: i32,
  /// The W-component of the vector.
  pub w: i32,
}

impl Vector4i {
  /// A constant for a [Null vector], which has magnitude 0 and exists at the
  /// [origin].
  ///
  /// [Null vector]: https://en.wikipedia.org/wiki/Null_vector
  /// [origin]: https://en.wikipedia.org/wiki/Origin_(mathematics)
  pub const ZERO: Vector4i = Vector4i::new(0, 0, 0, 0);

  /// A constant for a [unit vector] in the positive X-direction.
  ///
  /// [unit vector]: https://en.wikipedia.org/wiki/Unit_vector
  pub const UNIT_X: Vector4i = Vector4i::new(1, 0, 0, 0);

  /// A constant for a [unit vector] in the positive Y-direction.
  ///
  /// [unit vector]: https://en.wikipedia.org/wiki/Unit_vector
  pub const UNIT_Y: Vector4i = Vector4i::new(0, 1, 0, 0);

  /// A constant for a [unit vector] in the positive Z-direction.
  ///
  /// [unit vector]: https://en.wikipedia.org/wiki/Unit_vector
  pub const UNIT_Z: Vector4i = Vector4i::new(0, 0, 1, 0);

  /// A constant for a [unit vector] in the positive W-direction.
  ///
  /// [unit vector]: https://en.wikipedia.org/wiki/Unit_vector
  pub const UNIT_W: Vector4i = Vector4i::new(0, 0, 0, 1);

  /// A constant for a [unit vector] in the negative X-direction.
  ///
  /// [unit vector]: https://en.wikipedia.org/wiki/Unit_vector
  pub const NEG_UNIT_X: Vector4i = Vector4i::new(-1, 0, 0, 0);

  /// A constant for a [unit vector] in the negative Y-direction.
  ///
  /// [unit vector]: https://en.wikipedia.org/wiki/Unit_vector
  pub const NEG_UNIT_Y: Vector4i = Vector4i::new(0, -1, 0, 0);

  /// A constant for a [unit vector] in the negative Z-direction.
  ///
  /// [unit vector]: https://en.wikipedia.org/wiki/Unit_vector
  pub const NEG_UNIT_Z: Vector4i = Vector4i::new(0, 0, -1, 0);

  /// A constant for a [unit vector] in the negative W-direction.
  ///
  /// [unit vector]: https://en.wikipedia.org/wiki/Unit_vector
  pub const NEG_UNIT_W: Vector4i = Vector4i::new(0, 0, 0, -1);

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
  pub const fn new(x: i32, y: i32, z: i32, w: i32) -> Self {
    Self { x, y, z, w }
  }

  /// Constructs this vector with a uniform value `v`.
  ///
  /// # Arguments
  ///
  /// * `v` - the value to uniformly apply
  #[must_use]
  #[inline(always)]
  pub const fn uniform(v: i32) -> Self {
    Self::new(v, v, v, v)
  }

  /// Constructs this vector from a 4-component [`i32`] array.
  ///
  /// # Arguments
  ///
  /// * `array` - an array containing 4 [`i32`] values.
  #[must_use]
  #[inline(always)]
  pub const fn from_array(array: &[i32; 4]) -> Self {
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
  pub const fn from_slice(slice: &[i32]) -> Option<Self> {
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
  pub const unsafe fn from_slice_unchecked(slice: &[i32]) -> Self {
    debug_assert!(slice.len() == 4);
    Self::from_ptr(slice.as_ptr())
  }

  /// Constructs this vector from a [`Vec4i`]
  ///
  /// # Arguments
  ///
  /// * `other` - the other vector
  #[must_use]
  pub fn from_vec4i(other: &Vec4i) -> Self {
    Self {
      x: other.x(),
      y: other.y(),
      z: other.z(),
      w: other.w(),
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
  /// contiguous sequence of 4 [`i32`] values.
  #[must_use]
  #[inline(always)]
  pub const unsafe fn from_ptr(ptr: *const i32) -> Self {
    Self::new(*ptr, *ptr.add(1), *ptr.add(2), *ptr.add(3))
  }

  /// Returns this vector as a [`Vec4i`].
  #[must_use]
  #[inline(always)]
  pub const fn as_vec4(&self) -> &Vec4i {
    // SAFETY:
    //
    //   Vector4i is repr(C) and thus points to two contiguous elements
    //   of type and align of `i32`. The only pointer capable of accessing both
    //   entries within its memory region is a pointer to itself (`*const _`).
    //   Thus, we alias this to `i32` -- which under `repr(C)` points to the
    //   first element, and has proper reachability into its neighbor-element.
    unsafe {
      std::mem::transmute(std::slice::from_raw_parts(
        self as *const _ as *const i32,
        4,
      ))
    }
  }

  /// Returns this vector as a mutable [`Vec4i`].
  #[must_use]
  #[inline(always)]
  pub fn as_mut_vec4(&mut self) -> &mut Vec4i {
    // SAFETY: See explanation in Borrow<Vec4>
    unsafe {
      std::mem::transmute(std::slice::from_raw_parts_mut(
        self as *mut _ as *mut i32,
        4,
      ))
    }
  }

  /// Returns this vector as a slice of [`i32`].
  #[must_use]
  #[inline(always)]
  pub const fn as_slice(&self) -> &[i32] {
    self.as_vec4().as_slice()
  }

  /// Returns this vector as a mutable slice of [`i32`].
  #[must_use]
  #[inline(always)]
  pub fn as_mut_slice(&mut self) -> &mut [i32] {
    self.as_mut_vec4().as_mut_slice()
  }
}

impl From<&'_ Vec4i> for Vector4i {
  #[must_use]
  #[inline(always)]
  fn from(value: &'_ Vec4i) -> Self {
    value.to_owned()
  }
}

impl Deref for Vector4i {
  type Target = Vec4i;

  #[must_use]
  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    self.borrow()
  }
}

impl DerefMut for Vector4i {
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.borrow_mut()
  }
}

impl AsRef<Vec4i> for Vector4i {
  #[must_use]
  #[inline(always)]
  fn as_ref(&self) -> &Vec4i {
    self.as_vec4()
  }
}

impl AsMut<Vec4i> for Vector4i {
  #[must_use]
  #[inline(always)]
  fn as_mut(&mut self) -> &mut Vec4i {
    self.as_mut_vec4()
  }
}

impl Borrow<Vec4i> for Vector4i {
  #[must_use]
  #[inline(always)]
  fn borrow(&self) -> &Vec4i {
    self.as_vec4()
  }
}

impl BorrowMut<Vec4i> for Vector4i {
  #[must_use]
  #[inline(always)]
  fn borrow_mut(&mut self) -> &mut Vec4i {
    self.as_mut_vec4()
  }
}

impl ToOwned for Vec4i {
  type Owned = Vector4i;

  #[must_use]
  #[inline(always)]
  fn to_owned(&self) -> Self::Owned {
    Vector4i {
      x: self.x(),
      y: self.y(),
      z: self.z(),
      w: self.w(),
    }
  }
}

impl Add for &Vector4i {
  type Output = Vector4i;

  #[must_use]
  #[inline(always)]
  fn add(self, rhs: Self) -> Self::Output {
    self.as_vec4().add(rhs.as_vec4())
  }
}

impl Add for Vector4i {
  type Output = Vector4i;

  #[must_use]
  #[inline(always)]
  fn add(self, rhs: Self) -> Self::Output {
    self.add(rhs.as_vec4())
  }
}

impl Add<&Vec4i> for &Vector4i {
  type Output = Vector4i;

  #[must_use]
  #[inline(always)]
  fn add(self, rhs: &Vec4i) -> Self::Output {
    self.as_vec4().add(rhs)
  }
}

impl Add<&Vector4i> for &'_ Vec4i {
  type Output = Vector4i;

  #[must_use]
  #[inline(always)]
  fn add(self, rhs: &Vector4i) -> Self::Output {
    self.add(rhs.as_vec4())
  }
}

impl Add<Vector4i> for &Vec4i {
  type Output = Vector4i;

  #[must_use]
  #[inline(always)]
  fn add(self, rhs: Vector4i) -> Self::Output {
    // Addition is commutative, so reordering operations is safe
    rhs.add(self)
  }
}

impl Add<&Vec4i> for Vector4i {
  type Output = Vector4i;

  #[must_use]
  fn add(mut self, rhs: &Vec4i) -> Self::Output {
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

impl Add<&Vector4i> for Vector4i {
  type Output = Vector4i;

  #[must_use]
  #[inline(always)]
  fn add(self, rhs: &Vector4i) -> Self::Output {
    // Addition is commutative, so reordering operations is safe
    rhs.add(self.as_vec4())
  }
}

impl Add<Vector4i> for &Vector4i {
  type Output = Vector4i;

  #[must_use]
  #[inline(always)]
  fn add(self, rhs: Vector4i) -> Self::Output {
    // Addition is commutative, so reordering operations is safe
    rhs.as_vec4().add(self)
  }
}

impl Sub for &Vector4i {
  type Output = Vector4i;

  #[must_use]
  #[inline(always)]
  fn sub(self, rhs: Self) -> Self::Output {
    self.as_vec4().sub(rhs.as_vec4())
  }
}

impl Sub for Vector4i {
  type Output = Vector4i;

  #[must_use]
  #[inline(always)]
  fn sub(self, rhs: Self) -> Self::Output {
    self.sub(rhs.as_vec4())
  }
}

impl Sub<&Vec4i> for &Vector4i {
  type Output = Vector4i;

  #[must_use]
  #[inline(always)]
  fn sub(self, rhs: &Vec4i) -> Self::Output {
    self.as_vec4().sub(rhs)
  }
}

impl Sub<&Vector4i> for &'_ Vec4i {
  type Output = Vector4i;

  #[must_use]
  #[inline(always)]
  fn sub(self, rhs: &Vector4i) -> Self::Output {
    self.sub(rhs.as_vec4())
  }
}

impl Sub<Vector4i> for &Vec4i {
  type Output = Vector4i;

  #[must_use]
  #[inline(always)]
  fn sub(self, rhs: Vector4i) -> Self::Output {
    self.sub(rhs.as_vec4())
  }
}

impl Sub<&Vec4i> for Vector4i {
  type Output = Vector4i;

  #[must_use]
  fn sub(mut self, rhs: &Vec4i) -> Self::Output {
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

impl Sub<&Vector4i> for Vector4i {
  type Output = Vector4i;

  #[must_use]
  #[inline(always)]
  fn sub(self, rhs: &Vector4i) -> Self::Output {
    self.sub(rhs.as_vec4())
  }
}

impl Sub<Vector4i> for &Vector4i {
  type Output = Vector4i;

  #[must_use]
  #[inline(always)]
  fn sub(self, rhs: Vector4i) -> Self::Output {
    self.sub(rhs.as_vec4())
  }
}

impl Mul<i32> for Vector4i {
  type Output = Vector4i;

  #[must_use]
  #[inline(always)]
  fn mul(mut self, rhs: i32) -> Self::Output {
    self.as_mut_vec4().mul_assign(rhs);
    self
  }
}

impl Mul<i32> for &Vector4i {
  type Output = Vector4i;

  #[must_use]
  #[inline(always)]
  fn mul(self, rhs: i32) -> Self::Output {
    self.as_vec4().mul(rhs)
  }
}

impl Mul<Vector4i> for i32 {
  type Output = Vector4i;

  #[must_use]
  #[inline(always)]
  fn mul(self, mut rhs: Vector4i) -> Self::Output {
    rhs.as_mut_vec4().mul_assign(self);
    rhs
  }
}

impl Mul<&Vector4i> for i32 {
  type Output = Vector4i;

  #[must_use]
  #[inline(always)]
  fn mul(self, rhs: &Vector4i) -> Self::Output {
    rhs.as_vec4().mul(self)
  }
}

impl Div<i32> for Vector4i {
  type Output = Vector4i;

  #[must_use]
  #[inline(always)]
  fn div(mut self, rhs: i32) -> Self::Output {
    self.as_mut_vec4().div_assign(rhs);
    self
  }
}

impl Div<i32> for &Vector4i {
  type Output = Vector4i;

  #[must_use]
  #[inline(always)]
  fn div(self, rhs: i32) -> Self::Output {
    self.as_vec4().div(rhs)
  }
}

impl Rem<i32> for Vector4i {
  type Output = Vector4i;

  #[must_use]
  #[inline(always)]
  fn rem(mut self, rhs: i32) -> Self::Output {
    self.as_mut_vec4().rem_assign(rhs);
    self
  }
}

impl Rem<i32> for &Vector4i {
  type Output = Vector4i;

  #[must_use]
  #[inline(always)]
  fn rem(self, rhs: i32) -> Self::Output {
    self.as_vec4().rem(rhs)
  }
}

impl AddAssign for Vector4i {
  #[inline(always)]
  fn add_assign(&mut self, rhs: Self) {
    self.as_mut_vec4().add_assign(&rhs)
  }
}

impl AddAssign<&Vector4i> for Vector4i {
  #[inline(always)]
  fn add_assign(&mut self, rhs: &Self) {
    self.as_mut_vec4().add_assign(rhs)
  }
}

impl AddAssign<&Vec4i> for Vector4i {
  #[inline(always)]
  fn add_assign(&mut self, rhs: &Vec4i) {
    self.as_mut_vec4().add_assign(rhs)
  }
}

impl SubAssign for Vector4i {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: Self) {
    self.as_mut_vec4().sub_assign(&rhs)
  }
}

impl SubAssign<&Vector4i> for Vector4i {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: &Self) {
    self.as_mut_vec4().sub_assign(rhs)
  }
}

impl SubAssign<&Vec4i> for Vector4i {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: &Vec4i) {
    self.as_mut_vec4().sub_assign(rhs)
  }
}

impl MulAssign<i32> for Vector4i {
  #[inline(always)]
  fn mul_assign(&mut self, rhs: i32) {
    self.as_mut_vec4().mul_assign(rhs)
  }
}

impl DivAssign<i32> for Vector4i {
  #[inline(always)]
  fn div_assign(&mut self, rhs: i32) {
    self.as_mut_vec4().div_assign(rhs)
  }
}

impl RemAssign<i32> for Vector4i {
  #[inline(always)]
  fn rem_assign(&mut self, rhs: i32) {
    self.as_mut_vec4().rem_assign(rhs)
  }
}

impl fmt::Display for Vector4i {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "[{}, {}, {}, {}]", self.x, self.y, self.z, self.w)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_identity() {
    let vec = Vector4i::new(4, 2, 0, 0);

    assert_eq!(vec.as_ptr(), &vec[0]);
    assert_eq!(vec.as_ptr(), &vec.x);
  }

  #[test]
  fn test_iter() {
    let mut vec = Vector4i::new(4, 2, 1, 0);

    for v in vec.iter_mut() {
      *v = *v * 2
    }

    assert_eq!(vec.x, 8);
    assert_eq!(vec.y, 4);
    assert_eq!(vec.z, 2);
    assert_eq!(vec.w, 0);
  }
}
