use crate::ops::Dot;
use std::borrow::{Borrow, BorrowMut};
use std::ops::{
  Add, AddAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Rem,
  RemAssign, Sub, SubAssign,
};
use std::slice::SliceIndex;

/// A 2-component view of a vector-like object.
///
/// [`Vec2i`] objects are to [`Vector2i`] as [`str`] is to [`String`]; that is to
/// say that [`Vec2`] objects represent an immutable view of the owning
/// [`Vector2i`] counter-part.
#[repr(transparent)]
#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct Vec2i([i32]);

impl Vec2i {
  /// Forms a reference to a [`Vec2`] from a slice of [`i32`].
  ///
  /// This requires that `slice.len() == 2`, otherwise this returns [`None`].
  ///
  /// # Arguments
  ///
  /// * `slice` - the slice of [`i32`]s.
  pub const fn from_slice(slice: &[i32]) -> Option<&Self> {
    if slice.len() == 2 {
      // SAFETY: Vec2 is transparent, and implemented directly in terms of a
      //         slice of i32s. The representation is the same, and thus valid.
      //         This is implemented symmetrically to `OsStr`.
      Some(unsafe { std::mem::transmute(slice) })
    } else {
      None
    }
  }

  /// Forms a mutable reference to a [`Vec2`] from a mutable slice of [`i32`].
  ///
  /// This requires that `slice.len() == 2`, otherwise this returns [`None`].
  ///
  /// # Arguments
  ///
  /// * `slice` - the mutable slice of [`i32`]s.
  pub fn from_mut_slice(slice: &mut [i32]) -> Option<&mut Self> {
    if slice.len() == 2 {
      // SAFETY: Vec2 is transparent, and implemented directly in terms of a
      //         slice of i32s. The representation is the same, and thus valid.
      //         This is implemented symmetrically to `OsStr`.
      Some(unsafe { std::mem::transmute(slice) })
    } else {
      None
    }
  }

  /// Forms a reference to a [`Vec2`] from a slice of [`i32`] that is assumed to
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
  /// # Arguments
  ///
  /// * `slice` - the mutable slice of [`i32`]s.
  ///
  /// # Safety
  ///
  /// `slice.len()` must be equal to `2`.
  #[inline(always)]
  pub unsafe fn from_mut_slice_unchecked(slice: &mut [i32]) -> &mut Self {
    debug_assert!(slice.len() == 2);
    // SAFETY: See from_slice_unchecked
    unsafe { std::mem::transmute(slice) }
  }

  /// Forms a reference to a [`Vec2`] from a pointer to a contiguous sequence
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
  pub const unsafe fn from_ptr_unchecked<'a>(ptr: *const i32) -> &'a Vec2i {
    Vec2i::from_slice_unchecked(std::slice::from_raw_parts(ptr, 2))
  }

  /// Forms a mutable reference to a [`Vec2`] from a pointer to a contiguous
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
  pub unsafe fn from_mut_ptr_unchecked<'a>(ptr: *mut i32) -> &'a mut Vec2i {
    Vec2i::from_mut_slice_unchecked(std::slice::from_raw_parts_mut(ptr, 2))
  }

  /// Returns this [`Vec2`] as a slice of [`i32`].
  #[inline(always)]
  pub const fn as_slice(&self) -> &[i32] {
    &self.0
  }

  /// Returns this [`Vec2`] as a mutable slice of [`i32`].
  #[inline(always)]
  pub fn as_mut_slice(&mut self) -> &mut [i32] {
    &mut self.0
  }

  /// Returns the X-coordinate of this 2-component vector.
  #[inline(always)]
  pub const fn x(&self) -> i32 {
    unsafe { *self.0.as_ptr() }
  }

  /// Returns the Y-coordinate of this 2-component vector.
  #[inline(always)]
  pub const fn y(&self) -> i32 {
    unsafe { *self.0.as_ptr().add(1) }
  }

  /// Returns a mutable reference to the X-coordinate of this 2-component vector.
  #[inline(always)]
  pub fn x_mut(&mut self) -> &mut i32 {
    unsafe { &mut *self.0.as_mut_ptr() }
  }

  /// Returns a mutable reference to the Y-coordinate of this 2-component vector.
  #[inline(always)]
  pub fn y_mut(&mut self) -> &mut i32 {
    unsafe { &mut *self.0.as_mut_ptr().add(1) }
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

  /// Sets all the components of this vector the values from other.
  ///
  /// # Arguments
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
}

impl<I> Index<I> for Vec2i
where
  I: SliceIndex<[i32]>,
{
  type Output = I::Output;

  #[inline(always)]
  fn index(&self, index: I) -> &Self::Output {
    self.0.index(index)
  }
}

impl<I> IndexMut<I> for Vec2i
where
  I: SliceIndex<[i32]>,
{
  #[inline(always)]
  fn index_mut(&mut self, index: I) -> &mut Self::Output {
    self.0.index_mut(index)
  }
}

impl Deref for Vec2i {
  type Target = [i32];

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Vec2i {
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl AsRef<[i32]> for Vec2i {
  #[inline(always)]
  fn as_ref(&self) -> &[i32] {
    &self.0
  }
}

impl AsMut<[i32]> for Vec2i {
  #[inline(always)]
  fn as_mut(&mut self) -> &mut [i32] {
    &mut self.0
  }
}

impl Dot for Vec2i {
  type Output = i32;
  fn dot(&self, rhs: &Self) -> Self::Output {
    self.x() * rhs.x() + self.y() * rhs.y()
  }
}

impl Add for &'_ Vec2i {
  type Output = Vector2i;

  fn add(self, rhs: Self) -> Self::Output {
    Vector2i {
      x: self.x() + rhs.x(),
      y: self.y() + rhs.y(),
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

impl Sub for &'_ Vec2i {
  type Output = Vector2i;

  fn sub(self, rhs: Self) -> Self::Output {
    Vector2i {
      x: self.x() - rhs.x(),
      y: self.y() - rhs.y(),
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

impl Mul<i32> for &'_ Vec2i {
  type Output = Vector2i;

  fn mul(self, rhs: i32) -> Self::Output {
    Vector2i {
      x: self.x() * rhs,
      y: self.y() * rhs,
    }
  }
}

impl Mul<&'_ Vec2i> for i32 {
  type Output = Vector2i;

  fn mul(self, rhs: &'_ Vec2i) -> Self::Output {
    Vector2i {
      x: self * rhs.x(),
      y: self * rhs.y(),
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

impl Div<i32> for &'_ Vec2i {
  type Output = Vector2i;

  fn div(self, rhs: i32) -> Self::Output {
    Vector2i {
      x: self.x() / rhs,
      y: self.y() / rhs,
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

impl Rem<i32> for &'_ Vec2i {
  type Output = Vector2i;

  fn rem(self, rhs: i32) -> Self::Output {
    Vector2i {
      x: self.x().rem(rhs),
      y: self.y().rem(rhs),
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

impl Neg for &'_ Vec2i {
  type Output = Vector2i;

  fn neg(self) -> Self::Output {
    Vector2i {
      x: -self.x(),
      y: -self.y(),
    }
  }
}

impl std::fmt::Debug for Vec2i {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Vec2i")
      .field("x", &self.x())
      .field("y", &self.y())
      .finish()
  }
}

impl std::fmt::Display for Vec2i {
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
pub struct Vector2i {
  /// The X-component of the vector.
  pub x: i32,
  /// The Y-component of the vector.
  pub y: i32,
}

impl Vector2i {
  /// Constructs this vector from an x and y coordinate.
  ///
  /// # Arguments
  ///
  /// * `x` - the x-component
  /// * `y` - the y-component
  #[inline(always)]
  pub const fn new(x: i32, y: i32) -> Self {
    Self { x, y }
  }

  /// Constructs this vector with a uniform value `v`.
  ///
  /// # Arguments
  ///
  /// * `v` - the value to uniformly apply
  #[inline(always)]
  pub const fn uniform(v: i32) -> Self {
    Self::new(v, v)
  }

  /// Constructs this vector from a slice of floats.
  ///
  /// This will return [`None`] if `slice.len()` is not equal to 2.
  ///
  /// # Arguments
  ///
  /// * `slice` - the slice to read from
  pub const fn from_slice(slice: &[i32]) -> Option<Self> {
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
  pub const unsafe fn from_slice_unchecked(slice: &[i32]) -> Self {
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
  /// contiguous sequence of 2 [`i32`] values.
  pub const unsafe fn from_ptr(ptr: *const i32) -> Self {
    Self::new(*ptr, *ptr.add(1))
  }

  /// Returns this vector as a [`Vec2`].
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
  #[inline(always)]
  pub const fn as_slice(&self) -> &[i32] {
    self.as_vec2i().as_slice()
  }

  /// Returns this vector as a mutable slice of [`i32`].
  #[inline(always)]
  pub fn as_mut_slice(&mut self) -> &mut [i32] {
    self.as_mut_vec2i().as_mut_slice()
  }
}

impl From<&'_ Vec2i> for Vector2i {
  #[inline(always)]
  fn from(value: &'_ Vec2i) -> Self {
    value.to_owned()
  }
}

impl<Vec> From<Vec> for Vector2i
where
  Vec: AsRef<Vec2i>,
{
  #[inline(always)]
  fn from(value: Vec) -> Self {
    value.as_ref().to_owned()
  }
}

impl Add for &Vector2i {
  type Output = Vector2i;

  #[inline(always)]
  fn add(self, rhs: Self) -> Self::Output {
    self.as_vec2i().add(rhs.as_vec2i())
  }
}

impl Add for Vector2i {
  type Output = Vector2i;

  #[inline(always)]
  fn add(self, rhs: Self) -> Self::Output {
    self.add(rhs.as_vec2i())
  }
}

impl Add<&Vec2i> for &Vector2i {
  type Output = Vector2i;

  #[inline(always)]
  fn add(self, rhs: &Vec2i) -> Self::Output {
    self.as_vec2i().add(rhs)
  }
}

impl Add<&Vector2i> for &'_ Vec2i {
  type Output = Vector2i;

  #[inline(always)]
  fn add(self, rhs: &Vector2i) -> Self::Output {
    self.add(rhs.as_vec2i())
  }
}

impl Add<Vector2i> for &Vec2i {
  type Output = Vector2i;

  #[inline(always)]
  fn add(self, rhs: Vector2i) -> Self::Output {
    // Addition is commutative, so reordering operations is safe
    rhs.add(self)
  }
}

impl Add<&Vec2i> for Vector2i {
  type Output = Vector2i;

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

  #[inline(always)]
  fn add(self, rhs: &Vector2i) -> Self::Output {
    // Addition is commutative, so reordering operations is safe
    rhs.add(self.as_vec2i())
  }
}

impl Add<Vector2i> for &Vector2i {
  type Output = Vector2i;

  #[inline(always)]
  fn add(self, rhs: Vector2i) -> Self::Output {
    // Addition is commutative, so reordering operations is safe
    rhs.as_vec2i().add(self)
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

impl MulAssign<i32> for Vector2i {
  #[inline(always)]
  fn mul_assign(&mut self, rhs: i32) {
    self.as_mut_vec2i().mul_assign(rhs)
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

impl DivAssign<i32> for Vector2i {
  #[inline(always)]
  fn div_assign(&mut self, rhs: i32) {
    self.as_mut_vec2i().div_assign(rhs)
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

impl RemAssign<i32> for Vector2i {
  #[inline(always)]
  fn rem_assign(&mut self, rhs: i32) {
    self.as_mut_vec2i().rem_assign(rhs)
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

impl Borrow<[i32]> for Vector2i {
  #[inline(always)]
  fn borrow(&self) -> &[i32] {
    <Self as Borrow<Vec2i>>::borrow(self).as_ref()
  }
}

impl BorrowMut<[i32]> for Vector2i {
  #[inline(always)]
  fn borrow_mut(&mut self) -> &mut [i32] {
    <Self as BorrowMut<Vec2i>>::borrow_mut(self).as_mut()
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

impl std::fmt::Display for Vector2i {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{{{}, {}}}", self.x, self.y)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_identity() {
    let vec = Vector2i::new(4, 2);

    assert_eq!(vec.as_ptr(), &vec[0]);
    assert_eq!(vec.as_ptr(), &vec.x);
  }

  #[test]
  fn test_iter() {
    let mut vec = Vector2i::new(4, 2);

    for v in vec.iter_mut() {
      *v = *v * 2
    }

    assert_eq!(vec.x, 8);
    assert_eq!(vec.y, 4);
  }

  #[test]
  fn test_add() {
    let a = Vector2i { x: 10, y: 10 };
    let b = Vector2i { x: 0, y: 10 };

    let c = a + b;

    assert_eq!(c, Vector2i { x: 10, y: 20 })
  }
}
