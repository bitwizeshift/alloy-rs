use std::borrow::{Borrow, BorrowMut};
use std::ops::{
  Add, AddAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Rem,
  RemAssign, Sub, SubAssign,
};

use super::Vec2i;

/// A 3-component view of a vector-like object.
///
/// [`Vec3i`] objects are to [`Vector3i`] as [`str`] is to [`String`]; that is to
/// say that [`Vec3i`] objects represent an immutable view of the owning
/// [`Vector3i`] counter-part.
#[repr(transparent)]
#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct Vec3i([i32]);

impl Vec3i {
  /// Forms a reference to a [`Vec3`] from a slice of [`i32`].
  ///
  /// This requires that `slice.len() == 3`, otherwise this returns [`None`].
  ///
  /// # Arguments
  ///
  /// * `slice` - the slice of [`i32`]s.
  pub const fn from_slice(slice: &[i32]) -> Option<&Self> {
    if slice.len() == 3 {
      // SAFETY: Vec3 is transparent, and implemented directly in terms of a
      //         slice of i32s. The representation is the same, and thus valid.
      //         This is implemented symmetrically to `OsStr`.
      Some(unsafe { std::mem::transmute(slice) })
    } else {
      None
    }
  }

  /// Forms a mutable reference to a [`Vec3`] from a mutable slice of [`i32`].
  ///
  /// This requires that `slice.len() == 3`, otherwise this returns [`None`].
  ///
  /// # Arguments
  ///
  /// * `slice` - the mutable slice of [`i32`]s.
  pub fn from_mut_slice(slice: &mut [i32]) -> Option<&mut Self> {
    if slice.len() == 3 {
      // SAFETY: Vec3 is transparent, and implemented directly in terms of a
      //         slice of i32s. The representation is the same, and thus valid.
      //         This is implemented symmetrically to `OsStr`.
      Some(unsafe { std::mem::transmute(slice) })
    } else {
      None
    }
  }

  /// Forms a reference to a [`Vec3`] from a slice of [`i32`] that is assumed to
  /// contain two values.
  ///
  /// # Arguments
  ///
  /// * `slice` - the slice of [`i32`]s.
  ///
  /// # Safety
  ///
  /// `slice.len()` must be equal to `2`.
  #[inline(always)]
  pub const unsafe fn from_slice_unchecked(slice: &[i32]) -> &Self {
    debug_assert!(slice.len() == 3);
    // SAFETY:
    //   Vec3 is transparent, and implemented directly in terms of a
    //   slice of i32s. The representation is the same, and thus valid.
    //   This is implemented symmetrically to `OsStr`.
    unsafe { std::mem::transmute(slice) }
  }

  /// Forms a mutable reference to a [`Vec3`] from a slice of [`i32`] that is
  /// assumed to contain two values.
  ///
  /// # Arguments
  ///
  /// * `slice` - the mutable slice of [`i32`]s.
  ///
  /// # Safety
  ///
  /// `slice.len()` must be equal to `2`.
  #[inline(always)]
  pub unsafe fn from_mut_slice_unchecked(slice: &mut [i32]) -> &mut Self {
    debug_assert!(slice.len() == 3);
    // SAFETY: See from_slice_unchecked
    unsafe { std::mem::transmute(slice) }
  }

  /// Forms a reference to a [`Vec3`] from a pointer to a contiguous sequence
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
  #[inline(always)]
  pub const unsafe fn from_ptr_unchecked<'a>(ptr: *const i32) -> &'a Vec3i {
    Vec3i::from_slice_unchecked(std::slice::from_raw_parts(ptr, 3))
  }

  /// Forms a mutable reference to a [`Vec3`] from a pointer to a contiguous
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
  #[inline(always)]
  pub unsafe fn from_mut_ptr_unchecked<'a>(ptr: *mut i32) -> &'a mut Vec3i {
    Vec3i::from_mut_slice_unchecked(std::slice::from_raw_parts_mut(ptr, 3))
  }

  /// Returns this [`Vec3`] as a slice of [`i32`].
  #[inline(always)]
  pub const fn as_slice(&self) -> &[i32] {
    &self.0
  }

  /// Returns this [`Vec3`] as a mutable slice of [`i32`].
  #[inline(always)]
  pub fn as_mut_slice(&mut self) -> &mut [i32] {
    &mut self.0
  }

  /// Returns the X-coordinate of this 3-component vector.
  #[inline(always)]
  pub const fn x(&self) -> i32 {
    unsafe { *self.0.as_ptr() }
  }

  /// Returns the Y-coordinate of this 3-component vector.
  #[inline(always)]
  pub const fn y(&self) -> i32 {
    unsafe { *self.0.as_ptr().add(1) }
  }

  /// Returns the Z-coordinate of this 3-component vector.
  #[inline(always)]
  pub fn z(&self) -> i32 {
    unsafe { *self.0.as_ptr().add(2) }
  }

  /// Returns the xy coordinates of this vector as a [`Vec3`].
  #[inline(always)]
  pub const fn xy(&self) -> &Vec2i {
    unsafe { Vec2i::from_ptr_unchecked(self.0.as_ptr()) }
  }

  /// Returns the yz coordinates of this vector as a [`Vec3`].
  #[inline(always)]
  pub const fn yz(&self) -> &Vec2i {
    unsafe { Vec2i::from_ptr_unchecked(self.0.as_ptr().add(1)) }
  }

  /// Returns a mutable reference to the X-coordinate of this 3-component vector.
  #[inline(always)]
  pub fn x_mut(&mut self) -> &mut i32 {
    unsafe { &mut *self.0.as_mut_ptr() }
  }

  /// Returns a mutable reference to the Y-coordinate of this 3-component vector.
  #[inline(always)]
  pub fn y_mut(&mut self) -> &mut i32 {
    unsafe { &mut *self.0.as_mut_ptr().add(1) }
  }

  /// Returns a mutable reference to the Z-coordinate of this 3-component vector.
  #[inline(always)]
  pub fn z_mut(&mut self) -> &mut i32 {
    unsafe { &mut *self.0.as_mut_ptr().add(2) }
  }

  /// Returns a mutable reference to the xy coordinates of this vector.
  #[inline(always)]
  pub fn xy_mut(&mut self) -> &mut Vec2i {
    unsafe { Vec2i::from_mut_ptr_unchecked(self.0.as_mut_ptr()) }
  }

  /// Returns a mutable reference to the yz coordinates of this vector.
  #[inline(always)]
  pub fn yz_mut(&mut self) -> &mut Vec2i {
    unsafe { Vec2i::from_mut_ptr_unchecked(self.0.as_mut_ptr().add(1)) }
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
  /// * `z` - the Z-component
  #[inline(always)]
  pub fn set_z(&mut self, z: i32) {
    unsafe { *self.0.as_mut_ptr().add(2) = z }
  }

  /// Sets the X and Y components of this vector
  ///
  /// # Arguments
  ///
  /// * `xy` - the X and Y components o
  #[inline(always)]
  pub fn set_xy(&mut self, xy: &Vec2i) {
    self.xy_mut().set(xy)
  }

  /// Sets the Y and Z components of this vector
  ///
  /// # Arguments
  ///
  /// * `yz` - the Y and Z components of the [`Vec3`]
  #[inline(always)]
  pub fn set_yz(&mut self, xy: &Vec2i) {
    self.yz_mut().set(xy)
  }

  /// Sets all the components of this vector the values from other.
  ///
  /// # Arguments
  ///
  /// * `other` - the other [`Vec3`] to set.
  pub fn set(&mut self, other: &Vec3i) {
    let src_ptr = other.as_ptr();
    let dest_ptr = self.0.as_mut_ptr();

    unsafe {
      for i in 0..3 {
        *dest_ptr.add(i) = *src_ptr.add(i);
      }
    }
  }

  /// Computes the absolute value of `self`
  pub fn abs(&self) -> Vector3i {
    Vector3i {
      x: self.x().abs(),
      y: self.y().abs(),
      z: self.z().abs(),
    }
  }
}

impl<I> Index<I> for Vec3i
where
  I: std::slice::SliceIndex<[i32]>,
{
  type Output = I::Output;

  #[inline(always)]
  fn index(&self, index: I) -> &Self::Output {
    self.0.index(index)
  }
}

impl<I> IndexMut<I> for Vec3i
where
  I: std::slice::SliceIndex<[i32]>,
{
  #[inline(always)]
  fn index_mut(&mut self, index: I) -> &mut Self::Output {
    self.0.index_mut(index)
  }
}

impl Deref for Vec3i {
  type Target = [i32];

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Vec3i {
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl AsRef<[i32]> for Vec3i {
  #[inline(always)]
  fn as_ref(&self) -> &[i32] {
    &self.0
  }
}

impl AsMut<[i32]> for Vec3i {
  #[inline(always)]
  fn as_mut(&mut self) -> &mut [i32] {
    &mut self.0
  }
}

impl Add for &'_ Vec3i {
  type Output = Vector3i;

  fn add(self, rhs: Self) -> Self::Output {
    Vector3i {
      x: self.x() + rhs.x(),
      y: self.y() + rhs.y(),
      z: self.z() + rhs.z(),
    }
  }
}

impl AddAssign<&Vec3i> for Vec3i {
  fn add_assign(&mut self, rhs: &Vec3i) {
    let dest_ptr = self.0.as_mut_ptr();
    let src_ptr = rhs.0.as_ptr();

    unsafe {
      for i in 0..3 {
        *dest_ptr.add(i) += *src_ptr.add(i)
      }
    }
  }
}

impl Sub for &'_ Vec3i {
  type Output = Vector3i;

  fn sub(self, rhs: Self) -> Self::Output {
    Vector3i {
      x: self.x() - rhs.x(),
      y: self.y() - rhs.y(),
      z: self.z() - rhs.z(),
    }
  }
}

impl SubAssign<&Vec3i> for Vec3i {
  fn sub_assign(&mut self, rhs: &Vec3i) {
    let dest_ptr = self.0.as_mut_ptr();
    let src_ptr = rhs.0.as_ptr();

    unsafe {
      for i in 0..3 {
        *dest_ptr.add(i) -= *src_ptr.add(i)
      }
    }
  }
}

impl Mul<i32> for &'_ Vec3i {
  type Output = Vector3i;

  fn mul(self, rhs: i32) -> Self::Output {
    Vector3i {
      x: self.x() * rhs,
      y: self.y() * rhs,
      z: self.z() * rhs,
    }
  }
}

impl Mul<&'_ Vec3i> for i32 {
  type Output = Vector3i;

  fn mul(self, rhs: &'_ Vec3i) -> Self::Output {
    Vector3i {
      x: self * rhs.x(),
      y: self * rhs.y(),
      z: self * rhs.z(),
    }
  }
}

impl MulAssign<i32> for Vec3i {
  fn mul_assign(&mut self, rhs: i32) {
    let dest_ptr = self.0.as_mut_ptr();

    unsafe {
      for i in 0..3 {
        *dest_ptr.add(i) *= rhs
      }
    }
  }
}

impl Div<i32> for &'_ Vec3i {
  type Output = Vector3i;

  fn div(self, rhs: i32) -> Self::Output {
    Vector3i {
      x: self.x() / rhs,
      y: self.y() / rhs,
      z: self.z() / rhs,
    }
  }
}

impl DivAssign<i32> for Vec3i {
  fn div_assign(&mut self, rhs: i32) {
    let dest_ptr = self.0.as_mut_ptr();

    unsafe {
      for i in 0..3 {
        *dest_ptr.add(i) /= rhs
      }
    }
  }
}

impl Rem<i32> for &'_ Vec3i {
  type Output = Vector3i;

  fn rem(self, rhs: i32) -> Self::Output {
    Vector3i {
      x: self.x().rem(rhs),
      y: self.y().rem(rhs),
      z: self.z().rem(rhs),
    }
  }
}

impl RemAssign<i32> for Vec3i {
  fn rem_assign(&mut self, rhs: i32) {
    let dest_ptr = self.0.as_mut_ptr();

    unsafe {
      for i in 0..3 {
        *dest_ptr.add(i) %= rhs
      }
    }
  }
}

impl Neg for &'_ Vec3i {
  type Output = Vector3i;

  fn neg(self) -> Self::Output {
    Vector3i {
      x: -self.x(),
      y: -self.y(),
      z: -self.z(),
    }
  }
}

impl std::fmt::Debug for Vec3i {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Vec3i")
      .field("x", &self.x())
      .field("y", &self.y())
      .field("z", &self.z())
      .finish()
  }
}

impl std::fmt::Display for Vec3i {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{{{}, {}, {}}}", self.x(), self.y(), self.z())
  }
}

/// An owning representation of a 2-dimensional Vector object.
///
/// Unlike [`Vec3`], which is solely referential, [`Vector3i`] is an owning
/// instance.
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct Vector3i {
  /// The X-component of the vector.
  pub x: i32,
  /// The Y-component of the vector.
  pub y: i32,
  /// The Z-component of the vector.
  pub z: i32,
}

impl Vector3i {
  /// A constant for a vector of magnitude 0 at the origin.
  pub const ZERO: Vector3i = Vector3i::new(0, 0, 0);

  /// A constant for a unit vector in the positive X-direction.
  pub const UNIT_X: Vector3i = Vector3i::new(1, 0, 0);

  /// A constant for a unit vector in the positive Y-direction.
  pub const UNIT_Y: Vector3i = Vector3i::new(0, 1, 0);

  /// A constant for a unit vector in the positive Z-direction.
  pub const UNIT_Z: Vector3i = Vector3i::new(0, 0, 1);

  /// A constant for a unit vector in the negative X-direction.
  pub const NEG_UNIT_X: Vector3i = Vector3i::new(-1, 0, 0);

  /// A constant for a unit vector in the negative Y-direction.
  pub const NEG_UNIT_Y: Vector3i = Vector3i::new(0, -1, 0);

  /// A constant for a unit vector in the negative Z-direction.
  pub const NEG_UNIT_Z: Vector3i = Vector3i::new(0, 0, -1);

  /// Constructs this vector from an x, y, and z coordinate.
  ///
  /// # Arguments
  ///
  /// * `x` - the x-component
  /// * `y` - the y-component
  /// * `z` - the z-component
  #[inline(always)]
  pub const fn new(x: i32, y: i32, z: i32) -> Self {
    Self { x, y, z }
  }

  /// Constructs this vector with a uniform value `v`.
  ///
  /// # Arguments
  ///
  /// * `v` - the value to uniformly apply
  #[inline(always)]
  pub const fn uniform(v: i32) -> Self {
    Self::new(v, v, v)
  }

  /// Constructs this vector from a slice of floats.
  ///
  /// This will return [`None`] if `slice.len()` is not equal to 2.
  ///
  /// # Arguments
  ///
  /// * `slice` - the slice to read from
  pub const fn from_slice(slice: &[i32]) -> Option<Self> {
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
  pub const unsafe fn from_slice_unchecked(slice: &[i32]) -> Self {
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
  /// contiguous sequence of 3 [`i32`] values.
  pub const unsafe fn from_ptr(ptr: *const i32) -> Self {
    Self::new(*ptr, *ptr.add(1), *ptr.add(2))
  }

  /// Returns this vector as a [`Vec3`].
  #[inline(always)]
  pub const fn as_vec3i(&self) -> &Vec3i {
    // SAFETY:
    //
    //   Vector3i is repr(C) and thus points to two contiguous elements
    //   of type and align of `i32`. The only pointer capable of accessing both
    //   entries within its memory region is a pointer to itself (`*const _`).
    //   Thus, we alias this to `i32` -- which under `repr(C)` points to the
    //   first element, and has proper reachability into its neighbor-element.
    unsafe {
      std::mem::transmute(std::slice::from_raw_parts(
        self as *const _ as *const i32,
        3,
      ))
    }
  }

  /// Returns this vector as a mutable [`Vec3`].
  #[inline(always)]
  pub fn as_mut_vec3i(&mut self) -> &mut Vec3i {
    // SAFETY: See explanation in Self::as_vec3
    unsafe {
      std::mem::transmute(std::slice::from_raw_parts_mut(
        self as *mut _ as *mut i32,
        3,
      ))
    }
  }

  /// Returns this vector as a slice of [`i32`].
  #[inline(always)]
  pub const fn as_slice(&self) -> &[i32] {
    self.as_vec3i().as_slice()
  }

  /// Returns this vector as a mutable slice of [`i32`].
  #[inline(always)]
  pub fn as_mut_slice(&mut self) -> &mut [i32] {
    self.as_mut_vec3i().as_mut_slice()
  }
}

impl<Vec> From<Vec> for Vector3i
where
  Vec: AsRef<Vec3i>,
{
  #[inline(always)]
  fn from(value: Vec) -> Self {
    value.as_ref().to_owned()
  }
}

impl From<&'_ Vec3i> for Vector3i {
  #[inline(always)]
  fn from(value: &'_ Vec3i) -> Self {
    value.to_owned()
  }
}

impl Add for &Vector3i {
  type Output = Vector3i;

  #[inline(always)]
  fn add(self, rhs: Self) -> Self::Output {
    self.as_vec3i().add(rhs.as_vec3i())
  }
}

impl Add for Vector3i {
  type Output = Vector3i;

  #[inline(always)]
  fn add(self, rhs: Self) -> Self::Output {
    self.add(rhs.as_vec3i())
  }
}

impl Add<&Vec3i> for &Vector3i {
  type Output = Vector3i;

  #[inline(always)]
  fn add(self, rhs: &Vec3i) -> Self::Output {
    self.as_vec3i().add(rhs)
  }
}

impl Add<&Vector3i> for &'_ Vec3i {
  type Output = Vector3i;

  #[inline(always)]
  fn add(self, rhs: &Vector3i) -> Self::Output {
    self.add(rhs.as_vec3i())
  }
}

impl Add<Vector3i> for &Vec3i {
  type Output = Vector3i;

  #[inline(always)]
  fn add(self, rhs: Vector3i) -> Self::Output {
    // Addition is commutative, so reordering operations is safe
    rhs.add(self)
  }
}

impl Add<&Vec3i> for Vector3i {
  type Output = Vector3i;

  fn add(mut self, rhs: &Vec3i) -> Self::Output {
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

impl Add<&Vector3i> for Vector3i {
  type Output = Vector3i;

  #[inline(always)]
  fn add(self, rhs: &Vector3i) -> Self::Output {
    // Addition is commutative, so reordering operations is safe
    rhs.add(self.as_vec3i())
  }
}

impl Add<Vector3i> for &Vector3i {
  type Output = Vector3i;

  #[inline(always)]
  fn add(self, rhs: Vector3i) -> Self::Output {
    // Addition is commutative, so reordering operations is safe
    rhs.as_vec3i().add(self)
  }
}

impl AddAssign for Vector3i {
  #[inline(always)]
  fn add_assign(&mut self, rhs: Self) {
    self.as_mut_vec3i().add_assign(&rhs)
  }
}

impl AddAssign<&Vector3i> for Vector3i {
  #[inline(always)]
  fn add_assign(&mut self, rhs: &Self) {
    self.as_mut_vec3i().add_assign(rhs)
  }
}

impl AddAssign<&Vec3i> for Vector3i {
  #[inline(always)]
  fn add_assign(&mut self, rhs: &Vec3i) {
    self.as_mut_vec3i().add_assign(rhs)
  }
}

impl Sub for &Vector3i {
  type Output = Vector3i;

  #[inline(always)]
  fn sub(self, rhs: Self) -> Self::Output {
    self.as_vec3i().sub(rhs.as_vec3i())
  }
}

impl Sub for Vector3i {
  type Output = Vector3i;

  #[inline(always)]
  fn sub(self, rhs: Self) -> Self::Output {
    self.sub(rhs.as_vec3i())
  }
}

impl Sub<&Vec3i> for &Vector3i {
  type Output = Vector3i;

  #[inline(always)]
  fn sub(self, rhs: &Vec3i) -> Self::Output {
    self.as_vec3i().sub(rhs)
  }
}

impl Sub<&Vector3i> for &'_ Vec3i {
  type Output = Vector3i;

  #[inline(always)]
  fn sub(self, rhs: &Vector3i) -> Self::Output {
    self.sub(rhs.as_vec3i())
  }
}

impl Sub<Vector3i> for &Vec3i {
  type Output = Vector3i;

  #[inline(always)]
  fn sub(self, rhs: Vector3i) -> Self::Output {
    self.sub(rhs.as_vec3i())
  }
}

impl Sub<&Vec3i> for Vector3i {
  type Output = Vector3i;

  fn sub(mut self, rhs: &Vec3i) -> Self::Output {
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

impl Sub<&Vector3i> for Vector3i {
  type Output = Vector3i;

  #[inline(always)]
  fn sub(self, rhs: &Vector3i) -> Self::Output {
    self.sub(rhs.as_vec3i())
  }
}

impl Sub<Vector3i> for &Vector3i {
  type Output = Vector3i;

  #[inline(always)]
  fn sub(self, rhs: Vector3i) -> Self::Output {
    self.sub(rhs.as_vec3i())
  }
}

impl SubAssign for Vector3i {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: Self) {
    self.as_mut_vec3i().sub_assign(&rhs)
  }
}

impl SubAssign<&Vector3i> for Vector3i {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: &Self) {
    self.as_mut_vec3i().sub_assign(rhs)
  }
}

impl SubAssign<&Vec3i> for Vector3i {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: &Vec3i) {
    self.as_mut_vec3i().sub_assign(rhs)
  }
}

impl Mul<i32> for Vector3i {
  type Output = Vector3i;

  #[inline(always)]
  fn mul(mut self, rhs: i32) -> Self::Output {
    self.as_mut_vec3i().mul_assign(rhs);
    self
  }
}

impl Mul<i32> for &Vector3i {
  type Output = Vector3i;

  #[inline(always)]
  fn mul(self, rhs: i32) -> Self::Output {
    self.as_vec3i().mul(rhs)
  }
}

impl Mul<Vector3i> for i32 {
  type Output = Vector3i;

  #[inline(always)]
  fn mul(self, mut rhs: Vector3i) -> Self::Output {
    rhs.as_mut_vec3i().mul_assign(self);
    rhs
  }
}

impl Mul<&Vector3i> for i32 {
  type Output = Vector3i;

  #[inline(always)]
  fn mul(self, rhs: &Vector3i) -> Self::Output {
    rhs.as_vec3i().mul(self)
  }
}

impl MulAssign<i32> for Vector3i {
  #[inline(always)]
  fn mul_assign(&mut self, rhs: i32) {
    self.as_mut_vec3i().mul_assign(rhs)
  }
}

impl Div<i32> for Vector3i {
  type Output = Vector3i;

  #[inline(always)]
  fn div(mut self, rhs: i32) -> Self::Output {
    self.as_mut_vec3i().div_assign(rhs);
    self
  }
}

impl Div<i32> for &Vector3i {
  type Output = Vector3i;

  #[inline(always)]
  fn div(self, rhs: i32) -> Self::Output {
    self.as_vec3i().div(rhs)
  }
}

impl DivAssign<i32> for Vector3i {
  #[inline(always)]
  fn div_assign(&mut self, rhs: i32) {
    self.as_mut_vec3i().div_assign(rhs)
  }
}

impl Rem<i32> for Vector3i {
  type Output = Vector3i;

  #[inline(always)]
  fn rem(mut self, rhs: i32) -> Self::Output {
    self.as_mut_vec3i().rem_assign(rhs);
    self
  }
}

impl Rem<i32> for &Vector3i {
  type Output = Vector3i;

  #[inline(always)]
  fn rem(self, rhs: i32) -> Self::Output {
    self.as_vec3i().rem(rhs)
  }
}

impl RemAssign<i32> for Vector3i {
  #[inline(always)]
  fn rem_assign(&mut self, rhs: i32) {
    self.as_mut_vec3i().rem_assign(rhs)
  }
}

impl Deref for Vector3i {
  type Target = Vec3i;

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    self.borrow()
  }
}

impl DerefMut for Vector3i {
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.borrow_mut()
  }
}

impl Borrow<Vec3i> for Vector3i {
  #[inline(always)]
  fn borrow(&self) -> &Vec3i {
    self.as_vec3i()
  }
}

impl BorrowMut<Vec3i> for Vector3i {
  #[inline(always)]
  fn borrow_mut(&mut self) -> &mut Vec3i {
    self.as_mut_vec3i()
  }
}

impl Borrow<[i32]> for Vector3i {
  #[inline(always)]
  fn borrow(&self) -> &[i32] {
    <Self as Borrow<Vec3i>>::borrow(self).as_ref()
  }
}

impl BorrowMut<[i32]> for Vector3i {
  #[inline(always)]
  fn borrow_mut(&mut self) -> &mut [i32] {
    <Self as BorrowMut<Vec3i>>::borrow_mut(self).as_mut()
  }
}

impl ToOwned for Vec3i {
  type Owned = Vector3i;

  #[inline(always)]
  fn to_owned(&self) -> Self::Owned {
    Vector3i {
      x: self.x(),
      y: self.y(),
      z: self.z(),
    }
  }
}

impl std::fmt::Display for Vector3i {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{{{}, {}, {}}}", self.x, self.y, self.z)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_identity() {
    let vec = Vector3i::new(4, 2, 0);

    assert_eq!(vec.as_ptr(), &vec[0]);
    assert_eq!(vec.as_ptr(), &vec.x);
  }

  #[test]
  fn test_iter() {
    let mut vec = Vector3i::new(4, 2, 1);

    for v in vec.iter_mut() {
      *v = *v * 2
    }

    assert_eq!(vec.x, 8);
    assert_eq!(vec.y, 4);
    assert_eq!(vec.z, 2);
  }
}
