use crate::cmp::{AlmostEq, Near};
use crate::math::vec::{Vec2i, Vec2u};
use crate::math::Angle;
use crate::ops::{Dot, Midpoint};
use std::borrow::{Borrow, BorrowMut};
use std::ops::{
  Add, AddAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Rem,
  RemAssign, Sub, SubAssign,
};
use std::slice::SliceIndex;

/// A 2-component view of a vector-like object.
///
/// [`Vec2`] objects are to [`Vector2`] as [`str`] is to [`String`]; that is to
/// say that [`Vec2`] objects represent an immutable view of the owning
/// [`Vector2`] counter-part.
#[repr(transparent)]
#[derive(PartialEq, PartialOrd)]
pub struct Vec2([f32]);

impl Vec2 {
  /// Forms a reference to a [`Vec2`] from a slice of [`f32`].
  ///
  /// This requires that `slice.len() == 2`, otherwise this returns [`None`].
  ///
  /// # Arguments
  ///
  /// * `slice` - the slice of [`f32`]s.
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
  #[inline(always)]
  pub unsafe fn from_mut_ptr_unchecked<'a>(ptr: *mut f32) -> &'a mut Vec2 {
    Vec2::from_mut_slice_unchecked(std::slice::from_raw_parts_mut(ptr, 2))
  }

  /// Returns this [`Vec2`] as a slice of [`f32`].
  #[inline(always)]
  pub const fn as_slice(&self) -> &[f32] {
    &self.0
  }

  /// Returns this [`Vec2`] as a mutable slice of [`f32`].
  #[inline(always)]
  pub fn as_mut_slice(&mut self) -> &mut [f32] {
    &mut self.0
  }

  /// Returns the X-coordinate of this 2-component vector.
  #[inline(always)]
  pub const fn x(&self) -> f32 {
    unsafe { *self.0.as_ptr() }
  }

  /// Returns the Y-coordinate of this 2-component vector.
  #[inline(always)]
  pub const fn y(&self) -> f32 {
    unsafe { *self.0.as_ptr().add(1) }
  }

  /// Returns a mutable reference to the X-coordinate of this 2-component vector.
  #[inline(always)]
  pub fn x_mut(&mut self) -> &mut f32 {
    unsafe { &mut *self.0.as_mut_ptr() }
  }

  /// Returns a mutable reference to the Y-coordinate of this 2-component vector.
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
  #[inline(always)]
  pub fn square_magnitude(&self) -> f32 {
    self.dot(self)
  }

  /// Computes the magnitude of this vector.
  ///
  /// Where possible, consider using [`Vec2::square_magnitude`] as this will
  /// avoid the need to compute the square-root.
  pub fn magnitude(&self) -> f32 {
    self.square_magnitude().sqrt()
  }

  /// Queries whether this vector is normalized (e.g. has a magnitude of 1).
  pub fn is_normalized(&self) -> bool {
    self.square_magnitude().almost_eq(&1.0)
  }

  /// Normalizes this vector.
  pub fn normalize(&mut self) {
    *self /= self.magnitude()
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
}

impl Midpoint for Vec2 {
  type Output = Vector2;

  fn midpoint(&self, other: &Self) -> Self::Output {
    Vector2 {
      x: (self.x() + other.x()) / 2.0,
      y: (self.y() + other.y()) / 2.0,
    }
  }
}

impl Near for Vec2 {
  fn near(&self, other: &Self, tolerance: &f32) -> bool {
    self.x().near(&other.x(), tolerance) && self.y().near(&other.y(), tolerance)
  }
}

impl AlmostEq for Vec2 {
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

  #[inline(always)]
  fn index(&self, index: I) -> &Self::Output {
    self.0.index(index)
  }
}

impl<I> IndexMut<I> for Vec2
where
  I: SliceIndex<[f32]>,
{
  #[inline(always)]
  fn index_mut(&mut self, index: I) -> &mut Self::Output {
    self.0.index_mut(index)
  }
}

impl Deref for Vec2 {
  type Target = [f32];

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Vec2 {
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl AsRef<[f32]> for Vec2 {
  #[inline(always)]
  fn as_ref(&self) -> &[f32] {
    &self.0
  }
}

impl AsMut<[f32]> for Vec2 {
  #[inline(always)]
  fn as_mut(&mut self) -> &mut [f32] {
    &mut self.0
  }
}

impl Dot for Vec2 {
  type Output = f32;
  fn dot(&self, rhs: &Self) -> Self::Output {
    self.x() * rhs.x() + self.y() * rhs.y()
  }
}

impl Add for &'_ Vec2 {
  type Output = Vector2;

  fn add(self, rhs: Self) -> Self::Output {
    Vector2 {
      x: self.x() + rhs.x(),
      y: self.y() + rhs.y(),
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

impl Sub for &'_ Vec2 {
  type Output = Vector2;

  fn sub(self, rhs: Self) -> Self::Output {
    Vector2 {
      x: self.x() - rhs.x(),
      y: self.y() - rhs.y(),
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

impl Mul<f32> for &'_ Vec2 {
  type Output = Vector2;

  fn mul(self, rhs: f32) -> Self::Output {
    Vector2 {
      x: self.x() * rhs,
      y: self.y() * rhs,
    }
  }
}

impl Mul<&'_ Vec2> for f32 {
  type Output = Vector2;

  fn mul(self, rhs: &'_ Vec2) -> Self::Output {
    Vector2 {
      x: self * rhs.x(),
      y: self * rhs.y(),
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

impl Div<f32> for &'_ Vec2 {
  type Output = Vector2;

  fn div(self, rhs: f32) -> Self::Output {
    let inverse = 1.0 / rhs;
    Vector2 {
      x: self.x() * inverse,
      y: self.y() * inverse,
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

impl Rem<f32> for &'_ Vec2 {
  type Output = Vector2;

  fn rem(self, rhs: f32) -> Self::Output {
    Vector2 {
      x: self.x().rem(rhs),
      y: self.y().rem(rhs),
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

impl Neg for &'_ Vec2 {
  type Output = Vector2;

  fn neg(self) -> Self::Output {
    Vector2 {
      x: -self.x(),
      y: -self.y(),
    }
  }
}

impl std::fmt::Debug for Vec2 {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Vec2")
      .field("x", &self.x())
      .field("y", &self.y())
      .finish()
  }
}

impl std::fmt::Display for Vec2 {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{{{}, {}}}", self.x(), self.y())
  }
}

/// An owning representation of a 2-dimensional Vector object.
///
/// Unlike [`Vec2`], which is solely referential, [`Vector2`] is an owning
/// instance.
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, PartialOrd, Debug)]
pub struct Vector2 {
  /// The X-component of the vector.
  pub x: f32,
  /// The Y-component of the vector.
  pub y: f32,
}

impl Vector2 {
  /// A constant for a unit vector in the positive X-direction.
  pub const UNIT_X: Vector2 = Vector2::new(1.0, 0.0);

  /// A constant for a unit vector in the positive Y-direction.
  pub const UNIT_Y: Vector2 = Vector2::new(0.0, 1.0);

  /// A constant for a unit vector in the negative X-direction.
  pub const NEG_UNIT_X: Vector2 = Vector2::new(-1.0, 0.0);

  /// A constant for a unit vector in the negative Y-direction.
  pub const NEG_UNIT_Y: Vector2 = Vector2::new(0.0, -1.0);

  /// Constructs this vector from an x and y coordinate.
  ///
  /// # Arguments
  ///
  /// * `x` - the x-component
  /// * `y` - the y-component
  #[inline(always)]
  pub const fn new(x: f32, y: f32) -> Self {
    Self { x, y }
  }

  /// Constructs this vector with a uniform value `v`.
  ///
  /// # Arguments
  ///
  /// * `v` - the value to uniformly apply
  #[inline(always)]
  pub const fn uniform(v: f32) -> Self {
    Self::new(v, v)
  }

  /// Constructs this vector from a slice of floats.
  ///
  /// This will return [`None`] if `slice.len()` is not equal to 2.
  ///
  /// # Arguments
  ///
  /// * `slice` - the slice to read from
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
  pub const unsafe fn from_ptr(ptr: *const f32) -> Self {
    Self::new(*ptr, *ptr.add(1))
  }

  /// Constructs this vector from a [`Vec2`]
  ///
  /// # Arguments
  ///
  /// * `other` - the other vector
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
  pub fn from_vec2u(other: &Vec2u) -> Self {
    Self {
      x: other.x() as f32,
      y: other.y() as f32,
    }
  }

  /// Returns this vector as a [`Vec2`].
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
  #[inline(always)]
  pub const fn as_slice(&self) -> &[f32] {
    self.as_vec2().as_slice()
  }

  /// Returns this vector as a mutable slice of [`f32`].
  #[inline(always)]
  pub fn as_mut_slice(&mut self) -> &mut [f32] {
    self.as_mut_vec2().as_mut_slice()
  }
}

impl From<&'_ Vec2> for Vector2 {
  #[inline(always)]
  fn from(value: &'_ Vec2) -> Self {
    value.to_owned()
  }
}

impl<Vec> From<Vec> for Vector2
where
  Vec: AsRef<Vec2>,
{
  #[inline(always)]
  fn from(value: Vec) -> Self {
    value.as_ref().to_owned()
  }
}

impl Add for &Vector2 {
  type Output = Vector2;

  #[inline(always)]
  fn add(self, rhs: Self) -> Self::Output {
    self.as_vec2().add(rhs.as_vec2())
  }
}

impl Add for Vector2 {
  type Output = Vector2;

  #[inline(always)]
  fn add(self, rhs: Self) -> Self::Output {
    self.add(rhs.as_vec2())
  }
}

impl Add<&Vec2> for &Vector2 {
  type Output = Vector2;

  #[inline(always)]
  fn add(self, rhs: &Vec2) -> Self::Output {
    self.as_vec2().add(rhs)
  }
}

impl Add<&Vector2> for &'_ Vec2 {
  type Output = Vector2;

  #[inline(always)]
  fn add(self, rhs: &Vector2) -> Self::Output {
    self.add(rhs.as_vec2())
  }
}

impl Add<Vector2> for &Vec2 {
  type Output = Vector2;

  #[inline(always)]
  fn add(self, rhs: Vector2) -> Self::Output {
    // Addition is commutative, so reordering operations is safe
    rhs.add(self)
  }
}

impl Add<&Vec2> for Vector2 {
  type Output = Vector2;

  fn add(mut self, rhs: &Vec2) -> Self::Output {
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

impl Add<&Vector2> for Vector2 {
  type Output = Vector2;

  #[inline(always)]
  fn add(self, rhs: &Vector2) -> Self::Output {
    // Addition is commutative, so reordering operations is safe
    rhs.add(self.as_vec2())
  }
}

impl Add<Vector2> for &Vector2 {
  type Output = Vector2;

  #[inline(always)]
  fn add(self, rhs: Vector2) -> Self::Output {
    // Addition is commutative, so reordering operations is safe
    rhs.as_vec2().add(self)
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

impl Sub for &Vector2 {
  type Output = Vector2;

  #[inline(always)]
  fn sub(self, rhs: Self) -> Self::Output {
    self.as_vec2().sub(rhs.as_vec2())
  }
}

impl Sub for Vector2 {
  type Output = Vector2;

  #[inline(always)]
  fn sub(self, rhs: Self) -> Self::Output {
    self.sub(rhs.as_vec2())
  }
}

impl Sub<&Vec2> for &Vector2 {
  type Output = Vector2;

  #[inline(always)]
  fn sub(self, rhs: &Vec2) -> Self::Output {
    self.as_vec2().sub(rhs)
  }
}

impl Sub<&Vector2> for &'_ Vec2 {
  type Output = Vector2;

  #[inline(always)]
  fn sub(self, rhs: &Vector2) -> Self::Output {
    self.sub(rhs.as_vec2())
  }
}

impl Sub<Vector2> for &Vec2 {
  type Output = Vector2;

  #[inline(always)]
  fn sub(self, rhs: Vector2) -> Self::Output {
    self.sub(rhs.as_vec2())
  }
}

impl Sub<&Vec2> for Vector2 {
  type Output = Vector2;

  fn sub(mut self, rhs: &Vec2) -> Self::Output {
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

impl Sub<&Vector2> for Vector2 {
  type Output = Vector2;

  #[inline(always)]
  fn sub(self, rhs: &Vector2) -> Self::Output {
    self.sub(rhs.as_vec2())
  }
}

impl Sub<Vector2> for &Vector2 {
  type Output = Vector2;

  #[inline(always)]
  fn sub(self, rhs: Vector2) -> Self::Output {
    self.sub(rhs.as_vec2())
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

impl Mul<f32> for Vector2 {
  type Output = Vector2;

  #[inline(always)]
  fn mul(mut self, rhs: f32) -> Self::Output {
    self.as_mut_vec2().mul_assign(rhs);
    self
  }
}

impl Mul<f32> for &Vector2 {
  type Output = Vector2;

  #[inline(always)]
  fn mul(self, rhs: f32) -> Self::Output {
    self.as_vec2().mul(rhs)
  }
}

impl Mul<Vector2> for f32 {
  type Output = Vector2;

  #[inline(always)]
  fn mul(self, mut rhs: Vector2) -> Self::Output {
    rhs.as_mut_vec2().mul_assign(self);
    rhs
  }
}

impl Mul<&Vector2> for f32 {
  type Output = Vector2;

  #[inline(always)]
  fn mul(self, rhs: &Vector2) -> Self::Output {
    rhs.as_vec2().mul(self)
  }
}

impl MulAssign<f32> for Vector2 {
  #[inline(always)]
  fn mul_assign(&mut self, rhs: f32) {
    self.as_mut_vec2().mul_assign(rhs)
  }
}

impl Div<f32> for Vector2 {
  type Output = Vector2;

  #[inline(always)]
  fn div(mut self, rhs: f32) -> Self::Output {
    self.as_mut_vec2().div_assign(rhs);
    self
  }
}

impl Div<f32> for &Vector2 {
  type Output = Vector2;

  #[inline(always)]
  fn div(self, rhs: f32) -> Self::Output {
    self.as_vec2().div(rhs)
  }
}

impl DivAssign<f32> for Vector2 {
  #[inline(always)]
  fn div_assign(&mut self, rhs: f32) {
    self.as_mut_vec2().div_assign(rhs)
  }
}

impl Rem<f32> for Vector2 {
  type Output = Vector2;

  #[inline(always)]
  fn rem(mut self, rhs: f32) -> Self::Output {
    self.as_mut_vec2().rem_assign(rhs);
    self
  }
}

impl Rem<f32> for &Vector2 {
  type Output = Vector2;

  #[inline(always)]
  fn rem(self, rhs: f32) -> Self::Output {
    self.as_vec2().rem(rhs)
  }
}

impl RemAssign<f32> for Vector2 {
  #[inline(always)]
  fn rem_assign(&mut self, rhs: f32) {
    self.as_mut_vec2().rem_assign(rhs)
  }
}

impl Deref for Vector2 {
  type Target = Vec2;

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    self.borrow()
  }
}

impl DerefMut for Vector2 {
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.borrow_mut()
  }
}

impl Borrow<Vec2> for Vector2 {
  #[inline(always)]
  fn borrow(&self) -> &Vec2 {
    self.as_vec2()
  }
}

impl BorrowMut<Vec2> for Vector2 {
  #[inline(always)]
  fn borrow_mut(&mut self) -> &mut Vec2 {
    self.as_mut_vec2()
  }
}

impl Borrow<[f32]> for Vector2 {
  #[inline(always)]
  fn borrow(&self) -> &[f32] {
    <Self as Borrow<Vec2>>::borrow(self).as_ref()
  }
}

impl BorrowMut<[f32]> for Vector2 {
  #[inline(always)]
  fn borrow_mut(&mut self) -> &mut [f32] {
    <Self as BorrowMut<Vec2>>::borrow_mut(self).as_mut()
  }
}

impl ToOwned for Vec2 {
  type Owned = Vector2;

  #[inline(always)]
  fn to_owned(&self) -> Self::Owned {
    Vector2 {
      x: self.x(),
      y: self.y(),
    }
  }
}

impl std::fmt::Display for Vector2 {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{{{}, {}}}", self.x, self.y)
  }
}

#[cfg(test)]
mod test {
  use crate::cmp::AlmostEq;

  use super::*;

  #[test]
  fn test_vec2() {
    let vec = Vector2::new(4.0, 2.0);

    use crate::cmp::AlmostEq;
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
