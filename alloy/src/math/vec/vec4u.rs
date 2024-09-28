use crate::math::vec::{Vec2u, Vec3u};

use std::borrow::{Borrow, BorrowMut};
use std::fmt;
use std::ops::{
  Add, AddAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Rem, RemAssign,
  Sub, SubAssign,
};
use std::slice::SliceIndex;

/// A 4-component non-owning view of an unsigned [Euclidean vector].
///
/// [Euclidean vector]: https://en.wikipedia.org/wiki/Euclidean_vector
///
/// # Guarantees
///
/// This type has the strict requirement that it can only reference a
/// 4-component slice of [`u32`] values. It is guaranteed to never refer to
/// more or less than 4 entries.
///
/// # Relation to [`Vector4u`]
///
/// [`Vec4u`] is a non-owning equivalent of [`Vector4u`]. This enables non-vector
/// types to either [`Deref`] or provide conversion-related utilities into
/// [`Vec4u`] types to be able to access and benefit from vector operations.
#[repr(transparent)]
#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct Vec4u([u32]);

impl Vec4u {
  /// Forms a reference to a [`Vec4u`] from a 4-component [`u32`] array.
  ///
  /// This function is identical to [`from_slice_unchecked`], except it is not
  /// marked `unsafe`.
  ///
  /// [`from_slice_unchecked`]: Self::from_slice_unchecked
  ///
  /// # Parameters
  ///
  /// * `array` - an array containing 4 [`u32`] values.
  ///
  /// # Examples
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::vec::Vec4u;
  /// let array: [u32; 4] = [1, 42, 314, 9];
  ///
  /// let vec = Vec4u::from_array(&array);
  ///
  /// assert_eq!(vec.as_ptr(), array.as_ptr());
  /// assert_eq!(vec.x(), array[0]);
  /// assert_eq!(vec.y(), array[1]);
  /// assert_eq!(vec.z(), array[2]);
  /// assert_eq!(vec.w(), array[3]);
  /// ```
  #[must_use]
  #[inline(always)]
  pub const fn from_array(array: &[u32; 4]) -> &Self {
    // SAFETY: `array` is guaranteed to be 4-components
    unsafe { std::mem::transmute(array.as_slice()) }
  }

  /// Forms a mutable reference to a [`Vec4u`] from a 4-component [`u32`] array.
  ///
  /// This function is identical to [`from_mut_slice_unchecked`], except it is
  /// not marked `unsafe`.
  ///
  /// [`from_mut_slice_unchecked`]: Self::from_mut_slice_unchecked
  ///
  /// # Parameters
  ///
  /// * `array` - an array containing 4 [`u32`] values.
  ///
  /// # Examples
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::vec::Vec4u;
  /// let mut array: [u32; 4] = [1, 420, 314, 9];
  ///
  /// let vec = Vec4u::from_mut_array(&mut array);
  ///
  /// assert_eq!(vec.as_ptr(), array.as_ptr());
  /// ```
  #[must_use]
  #[inline(always)]
  pub fn from_mut_array(array: &mut [u32; 4]) -> &mut Self {
    // SAFETY: `array` is guaranteed to be 4-components
    unsafe { std::mem::transmute(array.as_mut_slice()) }
  }

  /// Forms a reference to a [`Vec4u`] from a slice of [`u32`].
  ///
  /// This requires that `slice.len() == 4`, otherwise this returns [`None`].
  ///
  /// # Parameters
  ///
  /// * `slice` - the slice of [`u32`]s.
  ///
  /// # Examples
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::vec::Vec4u;
  /// let slice = &[1, 42, 314, 9];
  ///
  /// let vec = Vec4u::from_slice(slice).unwrap();
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
  /// # use alloy::math::vec::Vec4u;
  /// let slice = &[1];
  ///
  /// let vec = Vec4u::from_slice(slice);
  ///
  /// assert_eq!(vec, None);
  /// ```
  #[must_use]
  pub const fn from_slice(slice: &[u32]) -> Option<&Self> {
    if slice.len() == 4 {
      // SAFETY: Vec4 is transparent, and implemented directly in terms of a
      //         slice of u32s. The representation is the same, and thus valid.
      //         This is implemented symmetrically to `OsStr`.
      Some(unsafe { Self::from_slice_unchecked(slice) })
    } else {
      None
    }
  }

  /// Forms a mutable reference to a [`Vec4u`] from a mutable slice of [`u32`].
  ///
  /// This requires that `slice.len() == 4`, otherwise this returns [`None`].
  ///
  /// # Parameters
  ///
  /// * `slice` - the mutable slice of [`u32`]s.
  ///
  /// # Examples
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use alloy::math::vec::Vec4u;
  /// let slice = &mut [1, 42, 314, 9];
  ///
  /// let vec = Vec4u::from_mut_slice(slice).unwrap();
  ///
  /// assert_eq!(vec.as_ptr(), slice.as_ptr());
  /// ```
  ///
  /// Invalid size:
  ///
  /// ```rust
  /// # use alloy::math::vec::Vec4u;
  /// let slice = &mut [1];
  ///
  /// let vec = Vec4u::from_mut_slice(slice);
  ///
  /// assert_eq!(vec, None);
  /// ```
  #[must_use]
  pub fn from_mut_slice(slice: &mut [u32]) -> Option<&mut Self> {
    if slice.len() == 4 {
      // SAFETY: Vec4 is transparent, and implemented directly in terms of a
      //         slice of u32s. The representation is the same, and thus valid.
      //         This is implemented symmetrically to `OsStr`.
      Some(unsafe { Self::from_mut_slice_unchecked(slice) })
    } else {
      None
    }
  }

  /// Forms a reference to a [`Vec4u`] from a slice of [`u32`] that is assumed to
  /// contain two values.
  ///
  /// # Parameters
  ///
  /// * `slice` - the slice of [`u32`]s.
  ///
  /// # Safety
  ///
  /// `slice.len()` must be equal to `2`.
  #[must_use]
  #[inline(always)]
  pub const unsafe fn from_slice_unchecked(slice: &[u32]) -> &Self {
    debug_assert!(slice.len() == 4);
    std::mem::transmute(slice)
  }

  /// Forms a mutable reference to a [`Vec4u`] from a slice of [`u32`] that is
  /// assumed to contain two values.
  ///
  /// # Parameters
  ///
  /// * `slice` - the mutable slice of [`u32`]s.
  ///
  /// # Safety
  ///
  /// `slice.len()` must be equal to `2`.
  #[must_use]
  #[inline(always)]
  pub unsafe fn from_mut_slice_unchecked(slice: &mut [u32]) -> &mut Self {
    debug_assert!(slice.len() == 4);
    std::mem::transmute(slice)
  }

  /// Forms a reference to a [`Vec2u`] from a pointer to a contiguous sequence
  /// of at least two [`u32`]s.
  ///
  /// # Parameters
  ///
  /// * `ptr` - the pointer to a sequence of [`u32`] values
  ///
  /// # Safety
  ///
  /// `ptr` must point to an allocated object that references at least two
  /// entries
  #[must_use]
  #[inline(always)]
  pub const unsafe fn from_ptr_unchecked<'a>(ptr: *const u32) -> &'a Vec4u {
    Vec4u::from_slice_unchecked(std::slice::from_raw_parts(ptr, 4))
  }

  /// Forms a mutable reference to a [`Vec3u`] from a pointer to a contiguous
  /// sequence of at least two [`u32`]s.
  ///
  /// # Parameters
  ///
  /// * `ptr` - the pointer to a sequence of [`u32`] values
  ///
  /// # Safety
  ///
  /// `ptr` must point to an allocated object that references at least two
  /// entries
  #[must_use]
  #[inline(always)]
  pub unsafe fn from_mut_ptr_unchecked<'a>(ptr: *mut u32) -> &'a mut Vec4u {
    Vec4u::from_mut_slice_unchecked(std::slice::from_raw_parts_mut(ptr, 4))
  }

  /// Returns this [`Vec4u`] as a slice of [`u32`].
  #[must_use]
  #[inline(always)]
  pub const fn as_slice(&self) -> &[u32] {
    &self.0
  }

  /// Returns this [`Vec4u`] as a mutable slice of [`u32`].
  #[must_use]
  #[inline(always)]
  pub fn as_mut_slice(&mut self) -> &mut [u32] {
    &mut self.0
  }

  /// Returns a raw pointer to [`u32`] of the vector's buffer.
  #[must_use]
  #[inline(always)]
  pub const fn as_ptr(&self) -> *const u32 {
    self.as_slice().as_ptr()
  }

  /// Returns a mutable raw pointer to [`u32`] of the vector's buffer.
  #[must_use]
  #[inline(always)]
  pub fn as_mut_ptr(&mut self) -> *mut u32 {
    self.as_mut_slice().as_mut_ptr()
  }

  /// Returns an iterator over the vector.
  #[inline(always)]
  pub fn iter(&self) -> impl Iterator<Item = &u32> {
    self.as_slice().iter()
  }

  /// Returns a mutable iterator over the vector.
  #[inline(always)]
  pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut u32> {
    self.as_mut_slice().iter_mut()
  }

  /// Returns the X-coordinate of this 4-component vector.
  #[must_use]
  #[inline(always)]
  pub fn x(&self) -> u32 {
    // SAFETY: Vec4 being of size-4 is an internal invariant
    unsafe { *self.0.as_ptr() }
  }

  /// Returns the Y-coordinate of this 4-component vector.
  #[must_use]
  #[inline(always)]
  pub fn y(&self) -> u32 {
    // SAFETY: Vec4 being of size-4 is an internal invariant
    unsafe { *self.0.as_ptr().add(1) }
  }

  /// Returns the Z-coordinate of this 4-component vector.
  #[must_use]
  #[inline(always)]
  pub fn z(&self) -> u32 {
    // SAFETY: Vec4 being of size-4 is an internal invariant
    unsafe { *self.0.as_ptr().add(2) }
  }

  /// Returns the W-coordinate of this 4-component vector.
  #[must_use]
  #[inline(always)]
  pub fn w(&self) -> u32 {
    // SAFETY: Vec4 being of size-4 is an internal invariant
    unsafe { *self.0.as_ptr().add(3) }
  }

  /// Returns the xy coordinates of this vector as a [`Vec2u`].
  #[must_use]
  #[inline(always)]
  pub const fn xy(&self) -> &Vec2u {
    unsafe { Vec2u::from_ptr_unchecked(self.0.as_ptr()) }
  }

  /// Returns the yz coordinates of this vector as a [`Vec2u`].
  #[must_use]
  #[inline(always)]
  pub const fn yz(&self) -> &Vec2u {
    unsafe { Vec2u::from_ptr_unchecked(self.0.as_ptr().add(1)) }
  }

  /// Returns the zw coordinates of this vector as a [`Vec2u`].
  #[must_use]
  #[inline(always)]
  pub const fn zw(&self) -> &Vec2u {
    unsafe { Vec2u::from_ptr_unchecked(self.0.as_ptr().add(2)) }
  }

  /// Returns the xyz coordinates of this vector as a [`Vec2u`].
  #[must_use]
  #[inline(always)]
  pub const fn xyz(&self) -> &Vec3u {
    unsafe { Vec3u::from_ptr_unchecked(self.0.as_ptr()) }
  }

  /// Returns the yz coordinates of this vector as a [`Vec2u`].
  #[must_use]
  #[inline(always)]
  pub const fn yzw(&self) -> &Vec3u {
    unsafe { Vec3u::from_ptr_unchecked(self.0.as_ptr().add(1)) }
  }

  /// Returns a mutable reference to the X-coordinate of this 3-component vector.
  #[must_use]
  #[inline(always)]
  pub fn x_mut(&mut self) -> &mut u32 {
    unsafe { &mut *self.0.as_mut_ptr() }
  }

  /// Returns a mutable reference to the Y-coordinate of this 3-component vector.
  #[must_use]
  #[inline(always)]
  pub fn y_mut(&mut self) -> &mut u32 {
    unsafe { &mut *self.0.as_mut_ptr().add(1) }
  }

  /// Returns a mutable reference to the Z-coordinate of this 3-component vector.
  #[must_use]
  #[inline(always)]
  pub fn z_mut(&mut self) -> &mut u32 {
    unsafe { &mut *self.0.as_mut_ptr().add(2) }
  }

  /// Returns a mutable reference to the W-coordinate of this 3-component vector.
  #[must_use]
  #[inline(always)]
  pub fn w_mut(&mut self) -> &mut u32 {
    unsafe { &mut *self.0.as_mut_ptr().add(3) }
  }

  /// Returns a mutable reference to the xy coordinates of this vector.
  #[must_use]
  #[inline(always)]
  pub fn xy_mut(&mut self) -> &mut Vec2u {
    unsafe { Vec2u::from_mut_ptr_unchecked(self.0.as_mut_ptr()) }
  }

  /// Returns a mutable reference to the yz coordinates of this vector.
  #[must_use]
  #[inline(always)]
  pub fn yz_mut(&mut self) -> &mut Vec2u {
    unsafe { Vec2u::from_mut_ptr_unchecked(self.0.as_mut_ptr().add(1)) }
  }

  /// Returns a mutable reference to the zw coordinates of this vector.
  #[must_use]
  #[inline(always)]
  pub fn zw_mut(&mut self) -> &mut Vec2u {
    unsafe { Vec2u::from_mut_ptr_unchecked(self.0.as_mut_ptr().add(2)) }
  }

  /// Returns a mutable reference to the yz coordinates of this vector.
  #[must_use]
  #[inline(always)]
  pub fn xyz_mut(&mut self) -> &mut Vec3u {
    unsafe { Vec3u::from_mut_ptr_unchecked(self.0.as_mut_ptr()) }
  }

  /// Returns a mutable reference to the zw coordinates of this vector.
  #[must_use]
  #[inline(always)]
  pub fn yzw_mut(&mut self) -> &mut Vec3u {
    unsafe { Vec3u::from_mut_ptr_unchecked(self.0.as_mut_ptr().add(1)) }
  }

  /// Sets the x-component
  ///
  /// # Parameters
  ///
  /// * `x` - the X-component
  #[inline(always)]
  pub fn set_x(&mut self, x: u32) {
    unsafe { *self.0.as_mut_ptr() = x }
  }

  /// Sets the y-component
  ///
  /// # Parameters
  ///
  /// * `y` - the Y-component
  #[inline(always)]
  pub fn set_y(&mut self, y: u32) {
    unsafe { *self.0.as_mut_ptr().add(1) = y }
  }

  /// Sets the z-component
  ///
  /// # Parameters
  ///
  /// * `z` - theZ-component
  #[inline(always)]
  pub fn set_z(&mut self, z: u32) {
    unsafe { *self.0.as_mut_ptr().add(2) = z }
  }

  /// Sets the w-component
  ///
  /// # Parameters
  ///
  /// * `w` - the W-component
  #[inline(always)]
  pub fn set_w(&mut self, w: u32) {
    unsafe { *self.0.as_mut_ptr().add(3) = w }
  }

  /// Sets the X and Y components of this vector
  ///
  /// # Parameters
  ///
  /// * `xy` - the X and Y components of the [`Vec4u`]
  #[inline(always)]
  pub fn set_xy(&mut self, xy: &Vec2u) {
    self.xy_mut().set(xy)
  }

  /// Sets the Y and Z components of this vector
  ///
  /// # Parameters
  ///
  /// * `yz` - the Y and Z components of the [`Vec4u`]
  #[inline(always)]
  pub fn set_yz(&mut self, yz: &Vec2u) {
    self.yz_mut().set(yz)
  }

  /// Sets the Z and W components of this vector
  ///
  /// # Parameters
  ///
  /// * `zw` - the Z and W components of the [`Vec4u`]
  #[inline(always)]
  pub fn set_zw(&mut self, zw: &Vec2u) {
    self.zw_mut().set(zw)
  }

  /// Sets the X, Y, and Z components of this vector
  ///
  /// # Parameters
  ///
  /// * `xyz` - the X, Y, and Z components of the [`Vec4u`]`
  #[inline(always)]
  pub fn set_xyz(&mut self, xyz: &Vec3u) {
    self.xyz_mut().set(xyz)
  }

  /// Sets the Y, Z and W components of this vector
  ///
  /// # Parameters
  ///
  /// * `yzw` - the Y, Z, and W components of the [`Vec4u`]
  #[inline(always)]
  pub fn set_yzw(&mut self, yzw: &Vec3u) {
    self.yzw_mut().set(yzw)
  }

  /// Sets all the components of this vector the values from other.
  ///
  /// # Parameters
  ///
  /// * `other` - the other [`Vec3u`] to set.
  pub fn set(&mut self, other: &Vec4u) {
    let src_ptr = other.as_ptr();
    let dest_ptr = self.0.as_mut_ptr();

    unsafe {
      for i in 0..4 {
        *dest_ptr.add(i) = *src_ptr.add(i);
      }
    }
  }

  /// Computes the minimum of each vector.
  ///
  /// # Parameters
  ///
  /// * `lhs` - the left vector
  /// * `rhs` - the right vector
  pub fn min(&self, rhs: &Vec4u) -> Vector4u {
    Vector4u {
      x: self.x().min(rhs.x()),
      y: self.y().min(rhs.y()),
      z: self.z().min(rhs.z()),
      w: self.w().min(rhs.w()),
    }
  }

  /// Computes the maximum of each vector.
  ///
  /// # Parameters
  ///
  /// * `lhs` - the left vector
  /// * `rhs` - the right vector
  pub fn max(&self, rhs: &Vec4u) -> Vector4u {
    Vector4u {
      x: self.x().max(rhs.x()),
      y: self.y().max(rhs.y()),
      z: self.z().max(rhs.z()),
      w: self.w().max(rhs.w()),
    }
  }
}

impl AsRef<[u32]> for Vec4u {
  #[must_use]
  #[inline(always)]
  fn as_ref(&self) -> &[u32] {
    &self.0
  }
}

impl AsMut<[u32]> for Vec4u {
  #[must_use]
  #[inline(always)]
  fn as_mut(&mut self) -> &mut [u32] {
    &mut self.0
  }
}

impl Borrow<[u32]> for Vec4u {
  #[must_use]
  #[inline(always)]
  fn borrow(&self) -> &[u32] {
    &self.0
  }
}

impl BorrowMut<[u32]> for Vec4u {
  #[must_use]
  #[inline(always)]
  fn borrow_mut(&mut self) -> &mut [u32] {
    &mut self.0
  }
}

impl<I> Index<I> for Vec4u
where
  I: SliceIndex<[u32]>,
{
  type Output = I::Output;

  #[must_use]
  #[inline(always)]
  fn index(&self, index: I) -> &Self::Output {
    // SAFETY: All Vec4 objects are guaranteed to have 4 components
    unsafe { crate::core::hint::fixed_size(&self.0, 4) };

    self.0.index(index)
  }
}

impl<I> IndexMut<I> for Vec4u
where
  I: SliceIndex<[u32]>,
{
  #[must_use]
  #[inline(always)]
  fn index_mut(&mut self, index: I) -> &mut Self::Output {
    // SAFETY: All Vec4 objects are guaranteed to have 4 components
    unsafe { crate::core::hint::fixed_size(&self.0, 4) };

    self.0.index_mut(index)
  }
}

impl Add for &'_ Vec4u {
  type Output = Vector4u;

  #[must_use]
  fn add(self, rhs: Self) -> Self::Output {
    Vector4u {
      x: self.x() + rhs.x(),
      y: self.y() + rhs.y(),
      z: self.z() + rhs.z(),
      w: self.w() + rhs.w(),
    }
  }
}

impl Sub for &'_ Vec4u {
  type Output = Vector4u;

  #[must_use]
  fn sub(self, rhs: Self) -> Self::Output {
    Vector4u {
      x: self.x() - rhs.x(),
      y: self.y() - rhs.y(),
      z: self.z() - rhs.z(),
      w: self.w() - rhs.w(),
    }
  }
}

impl Mul<u32> for &'_ Vec4u {
  type Output = Vector4u;

  #[must_use]
  fn mul(self, rhs: u32) -> Self::Output {
    Vector4u {
      x: self.x() * rhs,
      y: self.y() * rhs,
      z: self.z() * rhs,
      w: self.w() * rhs,
    }
  }
}

impl Mul<&'_ Vec4u> for u32 {
  type Output = Vector4u;

  #[must_use]
  fn mul(self, rhs: &'_ Vec4u) -> Self::Output {
    Vector4u {
      x: self * rhs.x(),
      y: self * rhs.y(),
      z: self * rhs.z(),
      w: self * rhs.w(),
    }
  }
}

impl Rem<u32> for &'_ Vec4u {
  type Output = Vector4u;

  #[must_use]
  fn rem(self, rhs: u32) -> Self::Output {
    Vector4u {
      x: self.x().rem(rhs),
      y: self.y().rem(rhs),
      z: self.z().rem(rhs),
      w: self.w().rem(rhs),
    }
  }
}

impl Div<u32> for &'_ Vec4u {
  type Output = Vector4u;

  #[must_use]
  fn div(self, rhs: u32) -> Self::Output {
    Vector4u {
      x: self.x() / rhs,
      y: self.y() / rhs,
      z: self.z() / rhs,
      w: self.w() / rhs,
    }
  }
}

impl AddAssign<&Vec4u> for Vec4u {
  fn add_assign(&mut self, rhs: &Vec4u) {
    let dest_ptr = self.0.as_mut_ptr();
    let src_ptr = rhs.0.as_ptr();

    unsafe {
      for i in 0..4 {
        *dest_ptr.add(i) += *src_ptr.add(i)
      }
    }
  }
}

impl SubAssign<&Vec4u> for Vec4u {
  fn sub_assign(&mut self, rhs: &Vec4u) {
    let dest_ptr = self.0.as_mut_ptr();
    let src_ptr = rhs.0.as_ptr();

    unsafe {
      for i in 0..4 {
        *dest_ptr.add(i) -= *src_ptr.add(i)
      }
    }
  }
}

impl MulAssign<u32> for Vec4u {
  fn mul_assign(&mut self, rhs: u32) {
    let dest_ptr = self.0.as_mut_ptr();

    unsafe {
      for i in 0..4 {
        *dest_ptr.add(i) *= rhs
      }
    }
  }
}

impl DivAssign<u32> for Vec4u {
  fn div_assign(&mut self, rhs: u32) {
    let dest_ptr = self.0.as_mut_ptr();

    unsafe {
      for i in 0..4 {
        *dest_ptr.add(i) /= rhs
      }
    }
  }
}

impl RemAssign<u32> for Vec4u {
  fn rem_assign(&mut self, rhs: u32) {
    let dest_ptr = self.0.as_mut_ptr();

    unsafe {
      for i in 0..4 {
        *dest_ptr.add(i) %= rhs
      }
    }
  }
}

impl fmt::Debug for Vec4u {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Vec4u")
      .field("x", &self.x())
      .field("y", &self.y())
      .field("z", &self.z())
      .field("w", &self.w())
      .finish()
  }
}

impl fmt::Display for Vec4u {
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

/// An owning representation of an unsigned 4-component [Euclidean vector].
///
/// Like [`Vec4u`], the [`Vector4u`] object represents a [Euclidean vector] in
/// 4D. Unlike the [`Vec4u`], this is an owning representation that stores the
/// actual content of the vector.
///
/// [Euclidean vector]: https://en.wikipedia.org/wiki/Euclidean_vector
#[repr(C)]
#[repr(align(16))]
#[derive(Clone, Copy, Default, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
pub struct Vector4u {
  /// The X-component of the vector.
  pub x: u32,
  /// The Y-component of the vector.
  pub y: u32,
  /// The Z-component of the vector.
  pub z: u32,
  /// The W-component of the vector.
  pub w: u32,
}

impl Vector4u {
  /// A constant for a [Null vector], which has magnitude 0 and exists at the
  /// [origin].
  ///
  /// [Null vector]: https://en.wikipedia.org/wiki/Null_vector
  /// [origin]: https://en.wikipedia.org/wiki/Origin_(mathematics)
  pub const ZERO: Vector4u = Vector4u::new(0, 0, 0, 0);

  /// A constant for a [unit vector] in the positive X-direction.
  ///
  /// [unit vector]: https://en.wikipedia.org/wiki/Unit_vector
  pub const UNIT_X: Vector4u = Vector4u::new(1, 0, 0, 0);

  /// A constant for a [unit vector] in the positive Y-direction.
  ///
  /// [unit vector]: https://en.wikipedia.org/wiki/Unit_vector
  pub const UNIT_Y: Vector4u = Vector4u::new(0, 1, 0, 0);

  /// A constant for a [unit vector] in the positive Z-direction.
  ///
  /// [unit vector]: https://en.wikipedia.org/wiki/Unit_vector
  pub const UNIT_Z: Vector4u = Vector4u::new(0, 0, 1, 0);

  /// A constant for a [unit vector] in the positive W-direction.
  ///
  /// [unit vector]: https://en.wikipedia.org/wiki/Unit_vector
  pub const UNIT_W: Vector4u = Vector4u::new(0, 0, 0, 1);

  /// Constructs this vector from an x, y, z, and w coordinate.
  ///
  /// # Parameters
  ///
  /// * `x` - the x-component
  /// * `y` - the y-component
  /// * `z` - the z-component
  /// * `w` - the w-component
  #[must_use]
  #[inline(always)]
  pub const fn new(x: u32, y: u32, z: u32, w: u32) -> Self {
    Self { x, y, z, w }
  }

  /// Constructs this vector with a uniform value `v`.
  ///
  /// # Parameters
  ///
  /// * `v` - the value to uniformly apply
  #[must_use]
  #[inline(always)]
  pub const fn uniform(v: u32) -> Self {
    Self::new(v, v, v, v)
  }
  /// Constructs this vector from a 4-component [`u32`] array.
  ///
  /// # Parameters
  ///
  /// * `array` - an array containing 4 [`u32`] values.
  #[must_use]
  #[inline(always)]
  pub const fn from_array(array: &[u32; 4]) -> Self {
    Self::new(array[0], array[1], array[2], array[3])
  }

  /// Constructs this vector from a slice of floats.
  ///
  /// This will return [`None`] if `slice.len()` is not equal to 2.
  ///
  /// # Parameters
  ///
  /// * `slice` - the slice to read from
  #[must_use]
  pub const fn from_slice(slice: &[u32]) -> Option<Self> {
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
  pub const unsafe fn from_slice_unchecked(slice: &[u32]) -> Self {
    debug_assert!(slice.len() == 4);
    Self::from_ptr(slice.as_ptr())
  }

  /// Constructs this vector from a [`Vec4u`]
  ///
  /// # Parameters
  ///
  /// * `other` - the other vector
  #[must_use]
  pub fn from_vec4u(other: &Vec4u) -> Self {
    Self {
      x: other.x(),
      y: other.y(),
      z: other.z(),
      w: other.w(),
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
  /// contiguous sequence of 3 [`u32`] values.
  #[must_use]
  #[inline(always)]
  pub const unsafe fn from_ptr(ptr: *const u32) -> Self {
    Self::new(*ptr, *ptr.add(1), *ptr.add(2), *ptr.add(3))
  }

  /// Returns this vector as a [`Vec4u`].
  #[must_use]
  #[inline(always)]
  pub const fn as_vec4(&self) -> &Vec4u {
    // SAFETY:
    //
    //   Vector4u is repr(C) and thus points to two contiguous elements
    //   of type and align of `u32`. The only pointer capable of accessing both
    //   entries within its memory region is a pointer to itself (`*const _`).
    //   Thus, we alias this to `u32` -- which under `repr(C)` points to the
    //   first element, and has proper reachability into its neighbor-element.
    unsafe {
      std::mem::transmute(std::slice::from_raw_parts(
        self as *const _ as *const u32,
        4,
      ))
    }
  }

  /// Returns this vector as a mutable [`Vec4u`].
  #[must_use]
  #[inline(always)]
  pub fn as_mut_vec4(&mut self) -> &mut Vec4u {
    // SAFETY: See explanation in Borrow<Vec4>
    unsafe {
      std::mem::transmute(std::slice::from_raw_parts_mut(
        self as *mut _ as *mut u32,
        4,
      ))
    }
  }

  /// Returns this vector as a slice of [`u32`].
  #[must_use]
  #[inline(always)]
  pub const fn as_slice(&self) -> &[u32] {
    self.as_vec4().as_slice()
  }

  /// Returns this vector as a mutable slice of [`u32`].
  #[must_use]
  #[inline(always)]
  pub fn as_mut_slice(&mut self) -> &mut [u32] {
    self.as_mut_vec4().as_mut_slice()
  }
}

impl From<&'_ Vec4u> for Vector4u {
  #[must_use]
  #[inline(always)]
  fn from(value: &'_ Vec4u) -> Self {
    value.to_owned()
  }
}

impl Deref for Vector4u {
  type Target = Vec4u;

  #[must_use]
  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    self.borrow()
  }
}

impl DerefMut for Vector4u {
  #[must_use]
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.borrow_mut()
  }
}

impl AsRef<Vec4u> for Vector4u {
  #[must_use]
  #[inline(always)]
  fn as_ref(&self) -> &Vec4u {
    self.as_vec4()
  }
}

impl AsMut<Vec4u> for Vector4u {
  #[must_use]
  #[inline(always)]
  fn as_mut(&mut self) -> &mut Vec4u {
    self.as_mut_vec4()
  }
}

impl Borrow<Vec4u> for Vector4u {
  #[must_use]
  #[inline(always)]
  fn borrow(&self) -> &Vec4u {
    self.as_vec4()
  }
}

impl BorrowMut<Vec4u> for Vector4u {
  #[must_use]
  #[inline(always)]
  fn borrow_mut(&mut self) -> &mut Vec4u {
    self.as_mut_vec4()
  }
}

impl Borrow<[u32]> for Vector4u {
  #[must_use]
  #[inline(always)]
  fn borrow(&self) -> &[u32] {
    <Self as Borrow<Vec4u>>::borrow(self).as_ref()
  }
}

impl BorrowMut<[u32]> for Vector4u {
  #[must_use]
  #[inline(always)]
  fn borrow_mut(&mut self) -> &mut [u32] {
    <Self as BorrowMut<Vec4u>>::borrow_mut(self).as_mut()
  }
}

impl ToOwned for Vec4u {
  type Owned = Vector4u;

  #[must_use]
  #[inline(always)]
  fn to_owned(&self) -> Self::Owned {
    Vector4u {
      x: self.x(),
      y: self.y(),
      z: self.z(),
      w: self.w(),
    }
  }
}

impl Add for &Vector4u {
  type Output = Vector4u;

  #[must_use]
  #[inline(always)]
  fn add(self, rhs: Self) -> Self::Output {
    self.as_vec4().add(rhs.as_vec4())
  }
}

impl Add for Vector4u {
  type Output = Vector4u;

  #[must_use]
  #[inline(always)]
  fn add(self, rhs: Self) -> Self::Output {
    self.add(rhs.as_vec4())
  }
}

impl Add<&Vec4u> for &Vector4u {
  type Output = Vector4u;

  #[must_use]
  #[inline(always)]
  fn add(self, rhs: &Vec4u) -> Self::Output {
    self.as_vec4().add(rhs)
  }
}

impl Add<&Vector4u> for &'_ Vec4u {
  type Output = Vector4u;

  #[must_use]
  #[inline(always)]
  fn add(self, rhs: &Vector4u) -> Self::Output {
    self.add(rhs.as_vec4())
  }
}

impl Add<Vector4u> for &Vec4u {
  type Output = Vector4u;

  #[must_use]
  #[inline(always)]
  fn add(self, rhs: Vector4u) -> Self::Output {
    // Addition is commutative, so reordering operations is safe
    rhs.add(self)
  }
}

impl Add<&Vec4u> for Vector4u {
  type Output = Vector4u;

  #[must_use]
  fn add(mut self, rhs: &Vec4u) -> Self::Output {
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

impl Add<&Vector4u> for Vector4u {
  type Output = Vector4u;

  #[must_use]
  #[inline(always)]
  fn add(self, rhs: &Vector4u) -> Self::Output {
    // Addition is commutative, so reordering operations is safe
    rhs.add(self.as_vec4())
  }
}

impl Add<Vector4u> for &Vector4u {
  type Output = Vector4u;

  #[must_use]
  #[inline(always)]
  fn add(self, rhs: Vector4u) -> Self::Output {
    // Addition is commutative, so reordering operations is safe
    rhs.as_vec4().add(self)
  }
}

impl Sub for &Vector4u {
  type Output = Vector4u;

  #[must_use]
  #[inline(always)]
  fn sub(self, rhs: Self) -> Self::Output {
    self.as_vec4().sub(rhs.as_vec4())
  }
}

impl Sub for Vector4u {
  type Output = Vector4u;

  #[must_use]
  #[inline(always)]
  fn sub(self, rhs: Self) -> Self::Output {
    self.sub(rhs.as_vec4())
  }
}

impl Sub<&Vec4u> for &Vector4u {
  type Output = Vector4u;

  #[must_use]
  #[inline(always)]
  fn sub(self, rhs: &Vec4u) -> Self::Output {
    self.as_vec4().sub(rhs)
  }
}

impl Sub<&Vector4u> for &'_ Vec4u {
  type Output = Vector4u;

  #[must_use]
  #[inline(always)]
  fn sub(self, rhs: &Vector4u) -> Self::Output {
    self.sub(rhs.as_vec4())
  }
}

impl Sub<Vector4u> for &Vec4u {
  type Output = Vector4u;

  #[must_use]
  #[inline(always)]
  fn sub(self, rhs: Vector4u) -> Self::Output {
    self.sub(rhs.as_vec4())
  }
}

impl Sub<&Vec4u> for Vector4u {
  type Output = Vector4u;

  #[must_use]
  fn sub(mut self, rhs: &Vec4u) -> Self::Output {
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

impl Sub<&Vector4u> for Vector4u {
  type Output = Vector4u;

  #[must_use]
  #[inline(always)]
  fn sub(self, rhs: &Vector4u) -> Self::Output {
    self.sub(rhs.as_vec4())
  }
}

impl Sub<Vector4u> for &Vector4u {
  type Output = Vector4u;

  #[must_use]
  #[inline(always)]
  fn sub(self, rhs: Vector4u) -> Self::Output {
    self.sub(rhs.as_vec4())
  }
}

impl Mul<u32> for Vector4u {
  type Output = Vector4u;

  #[must_use]
  #[inline(always)]
  fn mul(mut self, rhs: u32) -> Self::Output {
    self.as_mut_vec4().mul_assign(rhs);
    self
  }
}

impl Mul<u32> for &Vector4u {
  type Output = Vector4u;

  #[must_use]
  #[inline(always)]
  fn mul(self, rhs: u32) -> Self::Output {
    self.as_vec4().mul(rhs)
  }
}

impl Mul<Vector4u> for u32 {
  type Output = Vector4u;

  #[must_use]
  #[inline(always)]
  fn mul(self, mut rhs: Vector4u) -> Self::Output {
    rhs.as_mut_vec4().mul_assign(self);
    rhs
  }
}

impl Mul<&Vector4u> for u32 {
  type Output = Vector4u;

  #[must_use]
  #[inline(always)]
  fn mul(self, rhs: &Vector4u) -> Self::Output {
    rhs.as_vec4().mul(self)
  }
}

impl Div<u32> for Vector4u {
  type Output = Vector4u;

  #[must_use]
  #[inline(always)]
  fn div(mut self, rhs: u32) -> Self::Output {
    self.as_mut_vec4().div_assign(rhs);
    self
  }
}

impl Div<u32> for &Vector4u {
  type Output = Vector4u;

  #[must_use]
  #[inline(always)]
  fn div(self, rhs: u32) -> Self::Output {
    self.as_vec4().div(rhs)
  }
}

impl Rem<u32> for Vector4u {
  type Output = Vector4u;

  #[must_use]
  #[inline(always)]
  fn rem(mut self, rhs: u32) -> Self::Output {
    self.as_mut_vec4().rem_assign(rhs);
    self
  }
}

impl Rem<u32> for &Vector4u {
  type Output = Vector4u;

  #[must_use]
  #[inline(always)]
  fn rem(self, rhs: u32) -> Self::Output {
    self.as_vec4().rem(rhs)
  }
}

impl AddAssign for Vector4u {
  #[inline(always)]
  fn add_assign(&mut self, rhs: Self) {
    self.as_mut_vec4().add_assign(&rhs)
  }
}

impl AddAssign<&Vector4u> for Vector4u {
  #[inline(always)]
  fn add_assign(&mut self, rhs: &Self) {
    self.as_mut_vec4().add_assign(rhs)
  }
}

impl AddAssign<&Vec4u> for Vector4u {
  #[inline(always)]
  fn add_assign(&mut self, rhs: &Vec4u) {
    self.as_mut_vec4().add_assign(rhs)
  }
}

impl SubAssign for Vector4u {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: Self) {
    self.as_mut_vec4().sub_assign(&rhs)
  }
}

impl SubAssign<&Vector4u> for Vector4u {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: &Self) {
    self.as_mut_vec4().sub_assign(rhs)
  }
}

impl SubAssign<&Vec4u> for Vector4u {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: &Vec4u) {
    self.as_mut_vec4().sub_assign(rhs)
  }
}

impl MulAssign<u32> for Vector4u {
  #[inline(always)]
  fn mul_assign(&mut self, rhs: u32) {
    self.as_mut_vec4().mul_assign(rhs)
  }
}

impl DivAssign<u32> for Vector4u {
  #[inline(always)]
  fn div_assign(&mut self, rhs: u32) {
    self.as_mut_vec4().div_assign(rhs)
  }
}

impl RemAssign<u32> for Vector4u {
  #[inline(always)]
  fn rem_assign(&mut self, rhs: u32) {
    self.as_mut_vec4().rem_assign(rhs)
  }
}

impl fmt::Display for Vector4u {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "[{}, {}, {}, {}]", self.x, self.y, self.z, self.w)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_identity() {
    let vec = Vector4u::new(4, 2, 0, 0);

    assert_eq!(vec.as_ptr(), &vec[0]);
    assert_eq!(vec.as_ptr(), &vec.x);
  }

  #[test]
  fn test_iter() {
    let mut vec = Vector4u::new(4, 2, 1, 0);

    for v in vec.iter_mut() {
      *v = *v * 2
    }

    assert_eq!(vec.x, 8);
    assert_eq!(vec.y, 4);
    assert_eq!(vec.z, 2);
    assert_eq!(vec.w, 0);
  }
}
