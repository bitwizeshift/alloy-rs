use std::borrow::{Borrow, BorrowMut};
use std::ops::{
  Add, AddAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Rem, RemAssign,
  Sub, SubAssign,
};

use super::Vec2u;

/// A 3-component view of a vector-like object.
///
/// [`Vec3u`] objects are to [`Vector3u`] as [`str`] is to [`String`]; that is to
/// say that [`Vec3u`] objects represent an immutable view of the owning
/// [`Vector3u`] counter-part.
#[repr(transparent)]
#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct Vec3u([u32]);

impl Vec3u {
  /// Forms a reference to a [`Vec3`] from a slice of [`u32`].
  ///
  /// This requires that `slice.len() == 3`, otherwise this returns [`None`].
  ///
  /// # Arguments
  ///
  /// * `slice` - the slice of [`u32`]s.
  pub const fn from_slice(slice: &[u32]) -> Option<&Self> {
    if slice.len() == 3 {
      // SAFETY: Vec3 is transparent, and implemented directly in terms of a
      //         slice of u32s. The representation is the same, and thus valid.
      //         This is implemented symmetrically to `OsStr`.
      Some(unsafe { std::mem::transmute(slice) })
    } else {
      None
    }
  }

  /// Forms a mutable reference to a [`Vec3`] from a mutable slice of [`u32`].
  ///
  /// This requires that `slice.len() == 3`, otherwise this returns [`None`].
  ///
  /// # Arguments
  ///
  /// * `slice` - the mutable slice of [`u32`]s.
  pub fn from_mut_slice(slice: &mut [u32]) -> Option<&mut Self> {
    if slice.len() == 3 {
      // SAFETY: Vec3 is transparent, and implemented directly in terms of a
      //         slice of u32s. The representation is the same, and thus valid.
      //         This is implemented symmetrically to `OsStr`.
      Some(unsafe { std::mem::transmute(slice) })
    } else {
      None
    }
  }

  /// Forms a reference to a [`Vec3`] from a slice of [`u32`] that is assumed to
  /// contain two values.
  ///
  /// # Arguments
  ///
  /// * `slice` - the slice of [`u32`]s.
  ///
  /// # Safety
  ///
  /// `slice.len()` must be equal to `2`.
  #[inline(always)]
  pub const unsafe fn from_slice_unchecked(slice: &[u32]) -> &Self {
    debug_assert!(slice.len() == 3);
    // SAFETY:
    //   Vec3 is transparent, and implemented directly in terms of a
    //   slice of u32s. The representation is the same, and thus valid.
    //   This is implemented symmetrically to `OsStr`.
    unsafe { std::mem::transmute(slice) }
  }

  /// Forms a mutable reference to a [`Vec3`] from a slice of [`u32`] that is
  /// assumed to contain two values.
  ///
  /// # Arguments
  ///
  /// * `slice` - the mutable slice of [`u32`]s.
  ///
  /// # Safety
  ///
  /// `slice.len()` must be equal to `2`.
  #[inline(always)]
  pub unsafe fn from_mut_slice_unchecked(slice: &mut [u32]) -> &mut Self {
    debug_assert!(slice.len() == 3);
    // SAFETY: See from_slice_unchecked
    unsafe { std::mem::transmute(slice) }
  }

  /// Forms a reference to a [`Vec3`] from a pointer to a contiguous sequence
  /// of at least two [`u32`]s.
  ///
  /// # Arguments
  ///
  /// * `ptr` - the pointer to a sequence of [`u32`] values
  ///
  /// # Safety
  ///
  /// `ptr` must point to an allocated object that references at least two
  /// entries
  #[inline(always)]
  pub const unsafe fn from_ptr_unchecked<'a>(ptr: *const u32) -> &'a Vec3u {
    Vec3u::from_slice_unchecked(std::slice::from_raw_parts(ptr, 3))
  }

  /// Forms a mutable reference to a [`Vec3`] from a pointer to a contiguous
  /// sequence of at least two [`u32`]s.
  ///
  /// # Arguments
  ///
  /// * `ptr` - the pointer to a sequence of [`u32`] values
  ///
  /// # Safety
  ///
  /// `ptr` must point to an allocated object that references at least two
  /// entries
  #[inline(always)]
  pub unsafe fn from_mut_ptr_unchecked<'a>(ptr: *mut u32) -> &'a mut Vec3u {
    Vec3u::from_mut_slice_unchecked(std::slice::from_raw_parts_mut(ptr, 3))
  }

  /// Returns this [`Vec3`] as a slice of [`u32`].
  #[inline(always)]
  pub const fn as_slice(&self) -> &[u32] {
    &self.0
  }

  /// Returns this [`Vec3`] as a mutable slice of [`u32`].
  #[inline(always)]
  pub fn as_mut_slice(&mut self) -> &mut [u32] {
    &mut self.0
  }

  /// Returns the X-coordinate of this 3-component vector.
  #[inline(always)]
  pub const fn x(&self) -> u32 {
    unsafe { *self.0.as_ptr() }
  }

  /// Returns the Y-coordinate of this 3-component vector.
  #[inline(always)]
  pub const fn y(&self) -> u32 {
    unsafe { *self.0.as_ptr().add(1) }
  }

  /// Returns the Z-coordinate of this 3-component vector.
  #[inline(always)]
  pub fn z(&self) -> u32 {
    unsafe { *self.0.as_ptr().add(2) }
  }

  /// Returns the xy coordinates of this vector as a [`Vec3`].
  #[inline(always)]
  pub const fn xy(&self) -> &Vec2u {
    unsafe { Vec2u::from_ptr_unchecked(self.0.as_ptr()) }
  }

  /// Returns the yz coordinates of this vector as a [`Vec3`].
  #[inline(always)]
  pub const fn yz(&self) -> &Vec2u {
    unsafe { Vec2u::from_ptr_unchecked(self.0.as_ptr().add(1)) }
  }

  /// Returns a mutable reference to the X-coordinate of this 3-component vector.
  #[inline(always)]
  pub fn x_mut(&mut self) -> &mut u32 {
    unsafe { &mut *self.0.as_mut_ptr() }
  }

  /// Returns a mutable reference to the Y-coordinate of this 3-component vector.
  #[inline(always)]
  pub fn y_mut(&mut self) -> &mut u32 {
    unsafe { &mut *self.0.as_mut_ptr().add(1) }
  }

  /// Returns a mutable reference to the Z-coordinate of this 3-component vector.
  #[inline(always)]
  pub fn z_mut(&mut self) -> &mut u32 {
    unsafe { &mut *self.0.as_mut_ptr().add(2) }
  }

  /// Returns a mutable reference to the xy coordinates of this vector.
  #[inline(always)]
  pub fn xy_mut(&mut self) -> &mut Vec2u {
    unsafe { Vec2u::from_mut_ptr_unchecked(self.0.as_mut_ptr()) }
  }

  /// Returns a mutable reference to the yz coordinates of this vector.
  #[inline(always)]
  pub fn yz_mut(&mut self) -> &mut Vec2u {
    unsafe { Vec2u::from_mut_ptr_unchecked(self.0.as_mut_ptr().add(1)) }
  }

  /// Sets the x-component
  ///
  /// # Arguments
  ///
  /// * `x` - the X-component
  #[inline(always)]
  pub fn set_x(&mut self, x: u32) {
    unsafe { *self.0.as_mut_ptr() = x }
  }

  /// Sets the y-component
  ///
  /// # Arguments
  ///
  /// * `y` - the Y-component
  #[inline(always)]
  pub fn set_y(&mut self, y: u32) {
    unsafe { *self.0.as_mut_ptr().add(1) = y }
  }

  /// Sets the z-component
  ///
  /// # Arguments
  ///
  /// * `z` - the Z-component
  #[inline(always)]
  pub fn set_z(&mut self, z: u32) {
    unsafe { *self.0.as_mut_ptr().add(2) = z }
  }

  /// Sets the X and Y components of this vector
  ///
  /// # Arguments
  ///
  /// * `xy` - the X and Y components o
  #[inline(always)]
  pub fn set_xy(&mut self, xy: &Vec2u) {
    self.xy_mut().set(xy)
  }

  /// Sets the Y and Z components of this vector
  ///
  /// # Arguments
  ///
  /// * `yz` - the Y and Z components of the [`Vec3`]
  #[inline(always)]
  pub fn set_yz(&mut self, xy: &Vec2u) {
    self.yz_mut().set(xy)
  }

  /// Sets all the components of this vector the values from other.
  ///
  /// # Arguments
  ///
  /// * `other` - the other [`Vec3`] to set.
  pub fn set(&mut self, other: &Vec3u) {
    let src_ptr = other.as_ptr();
    let dest_ptr = self.0.as_mut_ptr();

    unsafe {
      for i in 0..3 {
        *dest_ptr.add(i) = *src_ptr.add(i);
      }
    }
  }
}

impl<I> Index<I> for Vec3u
where
  I: std::slice::SliceIndex<[u32]>,
{
  type Output = I::Output;

  #[inline(always)]
  fn index(&self, index: I) -> &Self::Output {
    self.0.index(index)
  }
}

impl<I> IndexMut<I> for Vec3u
where
  I: std::slice::SliceIndex<[u32]>,
{
  #[inline(always)]
  fn index_mut(&mut self, index: I) -> &mut Self::Output {
    self.0.index_mut(index)
  }
}

impl Deref for Vec3u {
  type Target = [u32];

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Vec3u {
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl AsRef<[u32]> for Vec3u {
  #[inline(always)]
  fn as_ref(&self) -> &[u32] {
    &self.0
  }
}

impl AsMut<[u32]> for Vec3u {
  #[inline(always)]
  fn as_mut(&mut self) -> &mut [u32] {
    &mut self.0
  }
}

impl Add for &'_ Vec3u {
  type Output = Vector3u;

  fn add(self, rhs: Self) -> Self::Output {
    Vector3u {
      x: self.x() + rhs.x(),
      y: self.y() + rhs.y(),
      z: self.z() + rhs.z(),
    }
  }
}

impl AddAssign<&Vec3u> for Vec3u {
  fn add_assign(&mut self, rhs: &Vec3u) {
    let dest_ptr = self.0.as_mut_ptr();
    let src_ptr = rhs.0.as_ptr();

    unsafe {
      for i in 0..3 {
        *dest_ptr.add(i) += *src_ptr.add(i)
      }
    }
  }
}

impl Sub for &'_ Vec3u {
  type Output = Vector3u;

  fn sub(self, rhs: Self) -> Self::Output {
    Vector3u {
      x: self.x() - rhs.x(),
      y: self.y() - rhs.y(),
      z: self.z() - rhs.z(),
    }
  }
}

impl SubAssign<&Vec3u> for Vec3u {
  fn sub_assign(&mut self, rhs: &Vec3u) {
    let dest_ptr = self.0.as_mut_ptr();
    let src_ptr = rhs.0.as_ptr();

    unsafe {
      for i in 0..3 {
        *dest_ptr.add(i) -= *src_ptr.add(i)
      }
    }
  }
}

impl Mul<u32> for &'_ Vec3u {
  type Output = Vector3u;

  fn mul(self, rhs: u32) -> Self::Output {
    Vector3u {
      x: self.x() * rhs,
      y: self.y() * rhs,
      z: self.z() * rhs,
    }
  }
}

impl Mul<&'_ Vec3u> for u32 {
  type Output = Vector3u;

  fn mul(self, rhs: &'_ Vec3u) -> Self::Output {
    Vector3u {
      x: self * rhs.x(),
      y: self * rhs.y(),
      z: self * rhs.z(),
    }
  }
}

impl MulAssign<u32> for Vec3u {
  fn mul_assign(&mut self, rhs: u32) {
    let dest_ptr = self.0.as_mut_ptr();

    unsafe {
      for i in 0..3 {
        *dest_ptr.add(i) *= rhs
      }
    }
  }
}

impl Div<u32> for &'_ Vec3u {
  type Output = Vector3u;

  fn div(self, rhs: u32) -> Self::Output {
    Vector3u {
      x: self.x() / rhs,
      y: self.y() / rhs,
      z: self.z() / rhs,
    }
  }
}

impl DivAssign<u32> for Vec3u {
  fn div_assign(&mut self, rhs: u32) {
    let dest_ptr = self.0.as_mut_ptr();

    unsafe {
      for i in 0..3 {
        *dest_ptr.add(i) /= rhs
      }
    }
  }
}

impl Rem<u32> for &'_ Vec3u {
  type Output = Vector3u;

  fn rem(self, rhs: u32) -> Self::Output {
    Vector3u {
      x: self.x().rem(rhs),
      y: self.y().rem(rhs),
      z: self.z().rem(rhs),
    }
  }
}

impl RemAssign<u32> for Vec3u {
  fn rem_assign(&mut self, rhs: u32) {
    let dest_ptr = self.0.as_mut_ptr();

    unsafe {
      for i in 0..3 {
        *dest_ptr.add(i) %= rhs
      }
    }
  }
}

impl std::fmt::Debug for Vec3u {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Vec3u")
      .field("x", &self.x())
      .field("y", &self.y())
      .field("z", &self.z())
      .finish()
  }
}

impl std::fmt::Display for Vec3u {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{{{}, {}, {}}}", self.x(), self.y(), self.z())
  }
}

/// An owning representation of a 2-dimensional Vector object.
///
/// Unlike [`Vec3`], which is solely referential, [`Vector3u`] is an owning
/// instance.
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct Vector3u {
  /// The X-component of the vector.
  pub x: u32,
  /// The Y-component of the vector.
  pub y: u32,
  /// The Z-component of the vector.
  pub z: u32,
}

impl Vector3u {
  /// A constant for a vector of magnitude 0 at the origin.
  pub const ZERO: Vector3u = Vector3u::new(0, 0, 0);

  /// A constant for a unit vector in the positive X-direction.
  pub const UNIT_X: Vector3u = Vector3u::new(1, 0, 0);

  /// A constant for a unit vector in the positive Y-direction.
  pub const UNIT_Y: Vector3u = Vector3u::new(0, 1, 0);

  /// A constant for a unit vector in the positive Z-direction.
  pub const UNIT_Z: Vector3u = Vector3u::new(0, 0, 1);

  /// Constructs this vector from an x, y, and z coordinate.
  ///
  /// # Arguments
  ///
  /// * `x` - the x-component
  /// * `y` - the y-component
  /// * `z` - the z-component
  #[inline(always)]
  pub const fn new(x: u32, y: u32, z: u32) -> Self {
    Self { x, y, z }
  }

  /// Constructs this vector with a uniform value `v`.
  ///
  /// # Arguments
  ///
  /// * `v` - the value to uniformly apply
  #[inline(always)]
  pub const fn uniform(v: u32) -> Self {
    Self::new(v, v, v)
  }

  /// Constructs this vector from a slice of floats.
  ///
  /// This will return [`None`] if `slice.len()` is not equal to 2.
  ///
  /// # Arguments
  ///
  /// * `slice` - the slice to read from
  pub const fn from_slice(slice: &[u32]) -> Option<Self> {
    if slice.len() != 3 {
      None
    } else {
      Some(Self {
        x: slice[0],
        y: slice[1],
        z: slice[3],
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
  #[inline(always)]
  pub const unsafe fn from_slice_unchecked(slice: &[u32]) -> Self {
    debug_assert!(slice.len() == 3);
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
  /// contiguous sequence of 3 [`u32`] values.
  pub const unsafe fn from_ptr(ptr: *const u32) -> Self {
    Self::new(*ptr, *ptr.add(1), *ptr.add(2))
  }

  /// Returns this vector as a [`Vec3`].
  #[inline(always)]
  pub const fn as_vec3u(&self) -> &Vec3u {
    // SAFETY:
    //
    //   Vector3u is repr(C) and thus points to two contiguous elements
    //   of type and align of `u32`. The only pointer capable of accessing both
    //   entries within its memory region is a pointer to itself (`*const _`).
    //   Thus, we alias this to `u32` -- which under `repr(C)` points to the
    //   first element, and has proper reachability into its neighbor-element.
    unsafe {
      std::mem::transmute(std::slice::from_raw_parts(
        self as *const _ as *const u32,
        3,
      ))
    }
  }

  /// Returns this vector as a mutable [`Vec3`].
  #[inline(always)]
  pub fn as_mut_vec3u(&mut self) -> &mut Vec3u {
    // SAFETY: See explanation in Self::as_vec3
    unsafe {
      std::mem::transmute(std::slice::from_raw_parts_mut(
        self as *mut _ as *mut u32,
        3,
      ))
    }
  }

  /// Returns this vector as a slice of [`u32`].
  #[inline(always)]
  pub const fn as_slice(&self) -> &[u32] {
    self.as_vec3u().as_slice()
  }

  /// Returns this vector as a mutable slice of [`u32`].
  #[inline(always)]
  pub fn as_mut_slice(&mut self) -> &mut [u32] {
    self.as_mut_vec3u().as_mut_slice()
  }
}

impl<Vec> From<Vec> for Vector3u
where
  Vec: AsRef<Vec3u>,
{
  #[inline(always)]
  fn from(value: Vec) -> Self {
    value.as_ref().to_owned()
  }
}

impl From<&'_ Vec3u> for Vector3u {
  #[inline(always)]
  fn from(value: &'_ Vec3u) -> Self {
    value.to_owned()
  }
}

impl Add for &Vector3u {
  type Output = Vector3u;

  #[inline(always)]
  fn add(self, rhs: Self) -> Self::Output {
    self.as_vec3u().add(rhs.as_vec3u())
  }
}

impl Add for Vector3u {
  type Output = Vector3u;

  #[inline(always)]
  fn add(self, rhs: Self) -> Self::Output {
    self.add(rhs.as_vec3u())
  }
}

impl Add<&Vec3u> for &Vector3u {
  type Output = Vector3u;

  #[inline(always)]
  fn add(self, rhs: &Vec3u) -> Self::Output {
    self.as_vec3u().add(rhs)
  }
}

impl Add<&Vector3u> for &'_ Vec3u {
  type Output = Vector3u;

  #[inline(always)]
  fn add(self, rhs: &Vector3u) -> Self::Output {
    self.add(rhs.as_vec3u())
  }
}

impl Add<Vector3u> for &Vec3u {
  type Output = Vector3u;

  #[inline(always)]
  fn add(self, rhs: Vector3u) -> Self::Output {
    // Addition is commutative, so reordering operations is safe
    rhs.add(self)
  }
}

impl Add<&Vec3u> for Vector3u {
  type Output = Vector3u;

  fn add(mut self, rhs: &Vec3u) -> Self::Output {
    // Repurpose 'self' for the output, to save space (1 less lifetime)
    let dest_ptr = self.0.as_mut_ptr();
    let src_ptr = rhs.0.as_ptr();

    unsafe {
      for i in 0..3 {
        *dest_ptr.add(i) += *src_ptr.add(i)
      }
    }
    self
  }
}

impl Add<&Vector3u> for Vector3u {
  type Output = Vector3u;

  #[inline(always)]
  fn add(self, rhs: &Vector3u) -> Self::Output {
    // Addition is commutative, so reordering operations is safe
    rhs.add(self.as_vec3u())
  }
}

impl Add<Vector3u> for &Vector3u {
  type Output = Vector3u;

  #[inline(always)]
  fn add(self, rhs: Vector3u) -> Self::Output {
    // Addition is commutative, so reordering operations is safe
    rhs.as_vec3u().add(self)
  }
}

impl AddAssign for Vector3u {
  #[inline(always)]
  fn add_assign(&mut self, rhs: Self) {
    self.as_mut_vec3u().add_assign(&rhs)
  }
}

impl AddAssign<&Vector3u> for Vector3u {
  #[inline(always)]
  fn add_assign(&mut self, rhs: &Self) {
    self.as_mut_vec3u().add_assign(rhs)
  }
}

impl AddAssign<&Vec3u> for Vector3u {
  #[inline(always)]
  fn add_assign(&mut self, rhs: &Vec3u) {
    self.as_mut_vec3u().add_assign(rhs)
  }
}

impl Sub for &Vector3u {
  type Output = Vector3u;

  #[inline(always)]
  fn sub(self, rhs: Self) -> Self::Output {
    self.as_vec3u().sub(rhs.as_vec3u())
  }
}

impl Sub for Vector3u {
  type Output = Vector3u;

  #[inline(always)]
  fn sub(self, rhs: Self) -> Self::Output {
    self.sub(rhs.as_vec3u())
  }
}

impl Sub<&Vec3u> for &Vector3u {
  type Output = Vector3u;

  #[inline(always)]
  fn sub(self, rhs: &Vec3u) -> Self::Output {
    self.as_vec3u().sub(rhs)
  }
}

impl Sub<&Vector3u> for &'_ Vec3u {
  type Output = Vector3u;

  #[inline(always)]
  fn sub(self, rhs: &Vector3u) -> Self::Output {
    self.sub(rhs.as_vec3u())
  }
}

impl Sub<Vector3u> for &Vec3u {
  type Output = Vector3u;

  #[inline(always)]
  fn sub(self, rhs: Vector3u) -> Self::Output {
    self.sub(rhs.as_vec3u())
  }
}

impl Sub<&Vec3u> for Vector3u {
  type Output = Vector3u;

  fn sub(mut self, rhs: &Vec3u) -> Self::Output {
    // Repurpose 'self' for the output, to save space (1 less lifetime)
    let dest_ptr = self.0.as_mut_ptr();
    let src_ptr = rhs.0.as_ptr();

    unsafe {
      for i in 0..3 {
        *dest_ptr.add(i) -= *src_ptr.add(i)
      }
    }
    self
  }
}

impl Sub<&Vector3u> for Vector3u {
  type Output = Vector3u;

  #[inline(always)]
  fn sub(self, rhs: &Vector3u) -> Self::Output {
    self.sub(rhs.as_vec3u())
  }
}

impl Sub<Vector3u> for &Vector3u {
  type Output = Vector3u;

  #[inline(always)]
  fn sub(self, rhs: Vector3u) -> Self::Output {
    self.sub(rhs.as_vec3u())
  }
}

impl SubAssign for Vector3u {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: Self) {
    self.as_mut_vec3u().sub_assign(&rhs)
  }
}

impl SubAssign<&Vector3u> for Vector3u {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: &Self) {
    self.as_mut_vec3u().sub_assign(rhs)
  }
}

impl SubAssign<&Vec3u> for Vector3u {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: &Vec3u) {
    self.as_mut_vec3u().sub_assign(rhs)
  }
}

impl Mul<u32> for Vector3u {
  type Output = Vector3u;

  #[inline(always)]
  fn mul(mut self, rhs: u32) -> Self::Output {
    self.as_mut_vec3u().mul_assign(rhs);
    self
  }
}

impl Mul<u32> for &Vector3u {
  type Output = Vector3u;

  #[inline(always)]
  fn mul(self, rhs: u32) -> Self::Output {
    self.as_vec3u().mul(rhs)
  }
}

impl Mul<Vector3u> for u32 {
  type Output = Vector3u;

  #[inline(always)]
  fn mul(self, mut rhs: Vector3u) -> Self::Output {
    rhs.as_mut_vec3u().mul_assign(self);
    rhs
  }
}

impl Mul<&Vector3u> for u32 {
  type Output = Vector3u;

  #[inline(always)]
  fn mul(self, rhs: &Vector3u) -> Self::Output {
    rhs.as_vec3u().mul(self)
  }
}

impl MulAssign<u32> for Vector3u {
  #[inline(always)]
  fn mul_assign(&mut self, rhs: u32) {
    self.as_mut_vec3u().mul_assign(rhs)
  }
}

impl Div<u32> for Vector3u {
  type Output = Vector3u;

  #[inline(always)]
  fn div(mut self, rhs: u32) -> Self::Output {
    self.as_mut_vec3u().div_assign(rhs);
    self
  }
}

impl Div<u32> for &Vector3u {
  type Output = Vector3u;

  #[inline(always)]
  fn div(self, rhs: u32) -> Self::Output {
    self.as_vec3u().div(rhs)
  }
}

impl DivAssign<u32> for Vector3u {
  #[inline(always)]
  fn div_assign(&mut self, rhs: u32) {
    self.as_mut_vec3u().div_assign(rhs)
  }
}

impl Rem<u32> for Vector3u {
  type Output = Vector3u;

  #[inline(always)]
  fn rem(mut self, rhs: u32) -> Self::Output {
    self.as_mut_vec3u().rem_assign(rhs);
    self
  }
}

impl Rem<u32> for &Vector3u {
  type Output = Vector3u;

  #[inline(always)]
  fn rem(self, rhs: u32) -> Self::Output {
    self.as_vec3u().rem(rhs)
  }
}

impl RemAssign<u32> for Vector3u {
  #[inline(always)]
  fn rem_assign(&mut self, rhs: u32) {
    self.as_mut_vec3u().rem_assign(rhs)
  }
}

impl Deref for Vector3u {
  type Target = Vec3u;

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    self.borrow()
  }
}

impl DerefMut for Vector3u {
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.borrow_mut()
  }
}

impl Borrow<Vec3u> for Vector3u {
  #[inline(always)]
  fn borrow(&self) -> &Vec3u {
    self.as_vec3u()
  }
}

impl BorrowMut<Vec3u> for Vector3u {
  #[inline(always)]
  fn borrow_mut(&mut self) -> &mut Vec3u {
    self.as_mut_vec3u()
  }
}

impl Borrow<[u32]> for Vector3u {
  #[inline(always)]
  fn borrow(&self) -> &[u32] {
    <Self as Borrow<Vec3u>>::borrow(self).as_ref()
  }
}

impl BorrowMut<[u32]> for Vector3u {
  #[inline(always)]
  fn borrow_mut(&mut self) -> &mut [u32] {
    <Self as BorrowMut<Vec3u>>::borrow_mut(self).as_mut()
  }
}

impl ToOwned for Vec3u {
  type Owned = Vector3u;

  #[inline(always)]
  fn to_owned(&self) -> Self::Owned {
    Vector3u {
      x: self.x(),
      y: self.y(),
      z: self.z(),
    }
  }
}

impl std::fmt::Display for Vector3u {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{{{}, {}, {}}}", self.x, self.y, self.z)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_identity() {
    let vec = Vector3u::new(4, 2, 0);

    assert_eq!(vec.as_ptr(), &vec[0]);
    assert_eq!(vec.as_ptr(), &vec.x);
  }

  #[test]
  fn test_iter() {
    let mut vec = Vector3u::new(4, 2, 1);

    for v in vec.iter_mut() {
      *v = *v * 2
    }

    assert_eq!(vec.x, 8);
    assert_eq!(vec.y, 4);
    assert_eq!(vec.z, 2);
  }
}
