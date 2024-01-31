use crate::ops::Dot;
use std::borrow::{Borrow, BorrowMut};
use std::ops::{
  Add, AddAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Rem, RemAssign,
  Sub, SubAssign,
};
use std::slice::SliceIndex;

/// A 2-component view of a vector-like object.
///
/// [`Vec2u`] objects are to [`Vector2u`] as [`str`] is to [`String`]; that is to
/// say that [`Vec2`] objects represent an immutable view of the owning
/// [`Vector2u`] counter-part.
#[repr(transparent)]
#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct Vec2u([u32]);

impl Vec2u {
  /// Forms a reference to a [`Vec2`] from a slice of [`u32`].
  ///
  /// This requires that `slice.len() == 2`, otherwise this returns [`None`].
  ///
  /// # Arguments
  ///
  /// * `slice` - the slice of [`u32`]s.
  pub const fn from_slice(slice: &[u32]) -> Option<&Self> {
    if slice.len() == 2 {
      // SAFETY: Vec2 is transparent, and implemented directly in terms of a
      //         slice of u32s. The representation is the same, and thus valid.
      //         This is implemented symmetrically to `OsStr`.
      Some(unsafe { std::mem::transmute(slice) })
    } else {
      None
    }
  }

  /// Forms a mutable reference to a [`Vec2`] from a mutable slice of [`u32`].
  ///
  /// This requires that `slice.len() == 2`, otherwise this returns [`None`].
  ///
  /// # Arguments
  ///
  /// * `slice` - the mutable slice of [`u32`]s.
  pub fn from_mut_slice(slice: &mut [u32]) -> Option<&mut Self> {
    if slice.len() == 2 {
      // SAFETY: Vec2 is transparent, and implemented directly in terms of a
      //         slice of u32s. The representation is the same, and thus valid.
      //         This is implemented symmetrically to `OsStr`.
      Some(unsafe { std::mem::transmute(slice) })
    } else {
      None
    }
  }

  /// Forms a reference to a [`Vec2`] from a slice of [`u32`] that is assumed to
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
    debug_assert!(slice.len() == 2);
    // SAFETY:
    //   Vec2 is transparent, and implemented directly in terms of a
    //   slice of u32s. The representation is the same, and thus valid.
    //   This is implemented symmetrically to `OsStr`.
    unsafe { std::mem::transmute(slice) }
  }

  /// Forms a mutable reference to a [`Vec2`] from a slice of [`u32`] that is
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
    debug_assert!(slice.len() == 2);
    // SAFETY: See from_slice_unchecked
    unsafe { std::mem::transmute(slice) }
  }

  /// Forms a reference to a [`Vec2`] from a pointer to a contiguous sequence
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
  pub const unsafe fn from_ptr_unchecked<'a>(ptr: *const u32) -> &'a Vec2u {
    Vec2u::from_slice_unchecked(std::slice::from_raw_parts(ptr, 2))
  }

  /// Forms a mutable reference to a [`Vec2`] from a pointer to a contiguous
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
  pub unsafe fn from_mut_ptr_unchecked<'a>(ptr: *mut u32) -> &'a mut Vec2u {
    Vec2u::from_mut_slice_unchecked(std::slice::from_raw_parts_mut(ptr, 2))
  }

  /// Returns this [`Vec2`] as a slice of [`u32`].
  #[inline(always)]
  pub const fn as_slice(&self) -> &[u32] {
    &self.0
  }

  /// Returns this [`Vec2`] as a mutable slice of [`u32`].
  #[inline(always)]
  pub fn as_mut_slice(&mut self) -> &mut [u32] {
    &mut self.0
  }

  /// Returns the X-coordinate of this 2-component vector.
  #[inline(always)]
  pub const fn x(&self) -> u32 {
    unsafe { *self.0.as_ptr() }
  }

  /// Returns the Y-coordinate of this 2-component vector.
  #[inline(always)]
  pub const fn y(&self) -> u32 {
    unsafe { *self.0.as_ptr().add(1) }
  }

  /// Returns a mutable reference to the X-coordinate of this 2-component vector.
  #[inline(always)]
  pub fn x_mut(&mut self) -> &mut u32 {
    unsafe { &mut *self.0.as_mut_ptr() }
  }

  /// Returns a mutable reference to the Y-coordinate of this 2-component vector.
  #[inline(always)]
  pub fn y_mut(&mut self) -> &mut u32 {
    unsafe { &mut *self.0.as_mut_ptr().add(1) }
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

  /// Sets all the components of this vector the values from other.
  ///
  /// # Arguments
  ///
  /// * `other` - the other [`Vec2`] to set.
  pub fn set(&mut self, other: &Vec2u) {
    let src_ptr = other.as_ptr();
    let dest_ptr = self.0.as_mut_ptr();

    unsafe {
      for i in 0..2 {
        *dest_ptr.add(i) = *src_ptr.add(i);
      }
    }
  }
}

impl<I> Index<I> for Vec2u
where
  I: SliceIndex<[u32]>,
{
  type Output = I::Output;

  #[inline(always)]
  fn index(&self, index: I) -> &Self::Output {
    self.0.index(index)
  }
}

impl<I> IndexMut<I> for Vec2u
where
  I: SliceIndex<[u32]>,
{
  #[inline(always)]
  fn index_mut(&mut self, index: I) -> &mut Self::Output {
    self.0.index_mut(index)
  }
}

impl Deref for Vec2u {
  type Target = [u32];

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Vec2u {
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl AsRef<[u32]> for Vec2u {
  #[inline(always)]
  fn as_ref(&self) -> &[u32] {
    &self.0
  }
}

impl AsMut<[u32]> for Vec2u {
  #[inline(always)]
  fn as_mut(&mut self) -> &mut [u32] {
    &mut self.0
  }
}

impl Dot for Vec2u {
  type Output = u32;
  fn dot(&self, rhs: &Self) -> Self::Output {
    self.x() * rhs.x() + self.y() * rhs.y()
  }
}

impl Add for &'_ Vec2u {
  type Output = Vector2u;

  fn add(self, rhs: Self) -> Self::Output {
    Vector2u {
      x: self.x() + rhs.x(),
      y: self.y() + rhs.y(),
    }
  }
}

impl AddAssign<&Vec2u> for Vec2u {
  fn add_assign(&mut self, rhs: &Vec2u) {
    let dest_ptr = self.0.as_mut_ptr();
    let src_ptr = rhs.0.as_ptr();

    unsafe {
      *dest_ptr += *src_ptr;
      *dest_ptr.add(1) += *src_ptr.add(1);
    }
  }
}

impl Sub for &'_ Vec2u {
  type Output = Vector2u;

  fn sub(self, rhs: Self) -> Self::Output {
    Vector2u {
      x: self.x() - rhs.x(),
      y: self.y() - rhs.y(),
    }
  }
}

impl SubAssign<&Vec2u> for Vec2u {
  fn sub_assign(&mut self, rhs: &Vec2u) {
    let dest_ptr = self.0.as_mut_ptr();
    let src_ptr = rhs.0.as_ptr();

    unsafe {
      *dest_ptr -= *src_ptr;
      *dest_ptr.add(1) -= *src_ptr.add(1);
    }
  }
}

impl Mul<u32> for &'_ Vec2u {
  type Output = Vector2u;

  fn mul(self, rhs: u32) -> Self::Output {
    Vector2u {
      x: self.x() * rhs,
      y: self.y() * rhs,
    }
  }
}

impl Mul<&'_ Vec2u> for u32 {
  type Output = Vector2u;

  fn mul(self, rhs: &'_ Vec2u) -> Self::Output {
    Vector2u {
      x: self * rhs.x(),
      y: self * rhs.y(),
    }
  }
}

impl MulAssign<u32> for Vec2u {
  fn mul_assign(&mut self, rhs: u32) {
    let dest_ptr = self.0.as_mut_ptr();

    unsafe {
      *dest_ptr *= rhs;
      *dest_ptr.add(1) *= rhs;
    }
  }
}

impl Div<u32> for &'_ Vec2u {
  type Output = Vector2u;

  fn div(self, rhs: u32) -> Self::Output {
    Vector2u {
      x: self.x() / rhs,
      y: self.y() / rhs,
    }
  }
}

impl DivAssign<u32> for Vec2u {
  fn div_assign(&mut self, rhs: u32) {
    let dest_ptr = self.0.as_mut_ptr();

    unsafe {
      *dest_ptr /= rhs;
      *dest_ptr.add(1) /= rhs;
    }
  }
}

impl Rem<u32> for &'_ Vec2u {
  type Output = Vector2u;

  fn rem(self, rhs: u32) -> Self::Output {
    Vector2u {
      x: self.x().rem(rhs),
      y: self.y().rem(rhs),
    }
  }
}

impl RemAssign<u32> for Vec2u {
  fn rem_assign(&mut self, rhs: u32) {
    let dest_ptr = self.0.as_mut_ptr();

    unsafe {
      *dest_ptr %= rhs;
      *dest_ptr.add(1) %= rhs;
    }
  }
}

impl std::fmt::Debug for Vec2u {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Vec2u")
      .field("x", &self.x())
      .field("y", &self.y())
      .finish()
  }
}

impl std::fmt::Display for Vec2u {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{{{}, {}}}", self.x(), self.y())
  }
}

/// An owning representation of a 2-dimensional Vector object.
///
/// Unlike [`Vec2`], which is solely referential, [`Vector2`] is an owning
/// instance.
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, PartialOrd, Debug, Eq, Ord)]
pub struct Vector2u {
  /// The X-component of the vector.
  pub x: u32,
  /// The Y-component of the vector.
  pub y: u32,
}

impl Vector2u {
  /// Constructs this vector from an x and y coordinate.
  ///
  /// # Arguments
  ///
  /// * `x` - the x-component
  /// * `y` - the y-component
  #[inline(always)]
  pub const fn new(x: u32, y: u32) -> Self {
    Self { x, y }
  }

  /// Constructs this vector with a uniform value `v`.
  ///
  /// # Arguments
  ///
  /// * `v` - the value to uniformly apply
  #[inline(always)]
  pub const fn uniform(v: u32) -> Self {
    Self::new(v, v)
  }

  /// Constructs this vector from a slice of floats.
  ///
  /// This will return [`None`] if `slice.len()` is not equal to 2.
  ///
  /// # Arguments
  ///
  /// * `slice` - the slice to read from
  pub const fn from_slice(slice: &[u32]) -> Option<Self> {
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
  #[inline(always)]
  pub const unsafe fn from_slice_unchecked(slice: &[u32]) -> Self {
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
  /// contiguous sequence of 2 [`u32`] values.
  pub const unsafe fn from_ptr(ptr: *const u32) -> Self {
    Self::new(*ptr, *ptr.add(1))
  }

  /// Returns this vector as a [`Vec2`].
  #[inline(always)]
  pub const fn as_vec2u(&self) -> &Vec2u {
    // SAFETY:
    //
    //   Vector2 is repr(C) and thus points to two contiguous elements
    //   of type and align of `u32`. The only pointer capable of accessing both
    //   entries within its memory region is a pointer to itself (`*const _`).
    //   Thus, we alias this to `u32` -- which under `repr(C)` points to the
    //   first element, and has proper reachability into its neighbor-element.
    unsafe {
      std::mem::transmute(std::slice::from_raw_parts(
        self as *const _ as *const u32,
        2,
      ))
    }
  }

  /// Returns this vector as a mutable [`Vec2`].
  #[inline(always)]
  pub fn as_mut_vec2u(&mut self) -> &mut Vec2u {
    // SAFETY: See explanation in Self::as_vec2u
    unsafe {
      std::mem::transmute(std::slice::from_raw_parts_mut(
        self as *mut _ as *mut u32,
        2,
      ))
    }
  }

  /// Returns this vector as a slice of [`u32`].
  #[inline(always)]
  pub const fn as_slice(&self) -> &[u32] {
    self.as_vec2u().as_slice()
  }

  /// Returns this vector as a mutable slice of [`u32`].
  #[inline(always)]
  pub fn as_mut_slice(&mut self) -> &mut [u32] {
    self.as_mut_vec2u().as_mut_slice()
  }
}

impl From<&'_ Vec2u> for Vector2u {
  #[inline(always)]
  fn from(value: &'_ Vec2u) -> Self {
    value.to_owned()
  }
}

impl<Vec> From<Vec> for Vector2u
where
  Vec: AsRef<Vec2u>,
{
  #[inline(always)]
  fn from(value: Vec) -> Self {
    value.as_ref().to_owned()
  }
}

impl Add for &Vector2u {
  type Output = Vector2u;

  #[inline(always)]
  fn add(self, rhs: Self) -> Self::Output {
    self.as_vec2u().add(rhs.as_vec2u())
  }
}

impl Add for Vector2u {
  type Output = Vector2u;

  #[inline(always)]
  fn add(self, rhs: Self) -> Self::Output {
    self.add(rhs.as_vec2u())
  }
}

impl Add<&Vec2u> for &Vector2u {
  type Output = Vector2u;

  #[inline(always)]
  fn add(self, rhs: &Vec2u) -> Self::Output {
    self.as_vec2u().add(rhs)
  }
}

impl Add<&Vector2u> for &'_ Vec2u {
  type Output = Vector2u;

  #[inline(always)]
  fn add(self, rhs: &Vector2u) -> Self::Output {
    self.add(rhs.as_vec2u())
  }
}

impl Add<Vector2u> for &Vec2u {
  type Output = Vector2u;

  #[inline(always)]
  fn add(self, rhs: Vector2u) -> Self::Output {
    // Addition is commutative, so reordering operations is safe
    rhs.add(self)
  }
}

impl Add<&Vec2u> for Vector2u {
  type Output = Vector2u;

  fn add(mut self, rhs: &Vec2u) -> Self::Output {
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

impl Add<&Vector2u> for Vector2u {
  type Output = Vector2u;

  #[inline(always)]
  fn add(self, rhs: &Vector2u) -> Self::Output {
    // Addition is commutative, so reordering operations is safe
    rhs.add(self.as_vec2u())
  }
}

impl Add<Vector2u> for &Vector2u {
  type Output = Vector2u;

  #[inline(always)]
  fn add(self, rhs: Vector2u) -> Self::Output {
    // Addition is commutative, so reordering operations is safe
    rhs.as_vec2u().add(self)
  }
}

impl AddAssign for Vector2u {
  #[inline(always)]
  fn add_assign(&mut self, rhs: Self) {
    self.as_mut_vec2u().add_assign(&rhs)
  }
}

impl AddAssign<&Vector2u> for Vector2u {
  #[inline(always)]
  fn add_assign(&mut self, rhs: &Self) {
    self.as_mut_vec2u().add_assign(rhs)
  }
}

impl AddAssign<&Vec2u> for Vector2u {
  #[inline(always)]
  fn add_assign(&mut self, rhs: &Vec2u) {
    self.as_mut_vec2u().add_assign(rhs)
  }
}

impl Sub for &Vector2u {
  type Output = Vector2u;

  #[inline(always)]
  fn sub(self, rhs: Self) -> Self::Output {
    self.as_vec2u().sub(rhs.as_vec2u())
  }
}

impl Sub for Vector2u {
  type Output = Vector2u;

  #[inline(always)]
  fn sub(self, rhs: Self) -> Self::Output {
    self.sub(rhs.as_vec2u())
  }
}

impl Sub<&Vec2u> for &Vector2u {
  type Output = Vector2u;

  #[inline(always)]
  fn sub(self, rhs: &Vec2u) -> Self::Output {
    self.as_vec2u().sub(rhs)
  }
}

impl Sub<&Vector2u> for &'_ Vec2u {
  type Output = Vector2u;

  #[inline(always)]
  fn sub(self, rhs: &Vector2u) -> Self::Output {
    self.sub(rhs.as_vec2u())
  }
}

impl Sub<Vector2u> for &Vec2u {
  type Output = Vector2u;

  #[inline(always)]
  fn sub(self, rhs: Vector2u) -> Self::Output {
    self.sub(rhs.as_vec2u())
  }
}

impl Sub<&Vec2u> for Vector2u {
  type Output = Vector2u;

  fn sub(mut self, rhs: &Vec2u) -> Self::Output {
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

impl Sub<&Vector2u> for Vector2u {
  type Output = Vector2u;

  #[inline(always)]
  fn sub(self, rhs: &Vector2u) -> Self::Output {
    self.sub(rhs.as_vec2u())
  }
}

impl Sub<Vector2u> for &Vector2u {
  type Output = Vector2u;

  #[inline(always)]
  fn sub(self, rhs: Vector2u) -> Self::Output {
    self.sub(rhs.as_vec2u())
  }
}

impl SubAssign for Vector2u {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: Self) {
    self.as_mut_vec2u().sub_assign(&rhs)
  }
}

impl SubAssign<&Vector2u> for Vector2u {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: &Self) {
    self.as_mut_vec2u().sub_assign(rhs)
  }
}

impl SubAssign<&Vec2u> for Vector2u {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: &Vec2u) {
    self.as_mut_vec2u().sub_assign(rhs)
  }
}

impl Mul<u32> for Vector2u {
  type Output = Vector2u;

  #[inline(always)]
  fn mul(mut self, rhs: u32) -> Self::Output {
    self.as_mut_vec2u().mul_assign(rhs);
    self
  }
}

impl Mul<u32> for &Vector2u {
  type Output = Vector2u;

  #[inline(always)]
  fn mul(self, rhs: u32) -> Self::Output {
    self.as_vec2u().mul(rhs)
  }
}

impl Mul<Vector2u> for u32 {
  type Output = Vector2u;

  #[inline(always)]
  fn mul(self, mut rhs: Vector2u) -> Self::Output {
    rhs.as_mut_vec2u().mul_assign(self);
    rhs
  }
}

impl Mul<&Vector2u> for u32 {
  type Output = Vector2u;

  #[inline(always)]
  fn mul(self, rhs: &Vector2u) -> Self::Output {
    rhs.as_vec2u().mul(self)
  }
}

impl MulAssign<u32> for Vector2u {
  #[inline(always)]
  fn mul_assign(&mut self, rhs: u32) {
    self.as_mut_vec2u().mul_assign(rhs)
  }
}

impl Div<u32> for Vector2u {
  type Output = Vector2u;

  #[inline(always)]
  fn div(mut self, rhs: u32) -> Self::Output {
    self.as_mut_vec2u().div_assign(rhs);
    self
  }
}

impl Div<u32> for &Vector2u {
  type Output = Vector2u;

  #[inline(always)]
  fn div(self, rhs: u32) -> Self::Output {
    self.as_vec2u().div(rhs)
  }
}

impl DivAssign<u32> for Vector2u {
  #[inline(always)]
  fn div_assign(&mut self, rhs: u32) {
    self.as_mut_vec2u().div_assign(rhs)
  }
}

impl Rem<u32> for Vector2u {
  type Output = Vector2u;

  #[inline(always)]
  fn rem(mut self, rhs: u32) -> Self::Output {
    self.as_mut_vec2u().rem_assign(rhs);
    self
  }
}

impl Rem<u32> for &Vector2u {
  type Output = Vector2u;

  #[inline(always)]
  fn rem(self, rhs: u32) -> Self::Output {
    self.as_vec2u().rem(rhs)
  }
}

impl RemAssign<u32> for Vector2u {
  #[inline(always)]
  fn rem_assign(&mut self, rhs: u32) {
    self.as_mut_vec2u().rem_assign(rhs)
  }
}

impl Deref for Vector2u {
  type Target = Vec2u;

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    self.borrow()
  }
}

impl DerefMut for Vector2u {
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.borrow_mut()
  }
}

impl Borrow<Vec2u> for Vector2u {
  #[inline(always)]
  fn borrow(&self) -> &Vec2u {
    self.as_vec2u()
  }
}

impl BorrowMut<Vec2u> for Vector2u {
  #[inline(always)]
  fn borrow_mut(&mut self) -> &mut Vec2u {
    self.as_mut_vec2u()
  }
}

impl Borrow<[u32]> for Vector2u {
  #[inline(always)]
  fn borrow(&self) -> &[u32] {
    <Self as Borrow<Vec2u>>::borrow(self).as_ref()
  }
}

impl BorrowMut<[u32]> for Vector2u {
  #[inline(always)]
  fn borrow_mut(&mut self) -> &mut [u32] {
    <Self as BorrowMut<Vec2u>>::borrow_mut(self).as_mut()
  }
}

impl ToOwned for Vec2u {
  type Owned = Vector2u;

  #[inline(always)]
  fn to_owned(&self) -> Self::Owned {
    Vector2u {
      x: self.x(),
      y: self.y(),
    }
  }
}

impl std::fmt::Display for Vector2u {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{{{}, {}}}", self.x, self.y)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_identity() {
    let vec = Vector2u::new(4, 2);

    assert_eq!(vec.as_ptr(), &vec[0]);
    assert_eq!(vec.as_ptr(), &vec.x);
  }

  #[test]
  fn test_iter() {
    let mut vec = Vector2u::new(4, 2);

    for v in vec.iter_mut() {
      *v = *v * 2
    }

    assert_eq!(vec.x, 8);
    assert_eq!(vec.y, 4);
  }

  #[test]
  fn test_add() {
    let a = Vector2u { x: 10, y: 10 };
    let b = Vector2u { x: 0, y: 10 };

    let c = a + b;

    assert_eq!(c, Vector2u { x: 10, y: 20 })
  }
}
