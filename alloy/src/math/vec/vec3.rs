use crate::cmp::{AlmostEq, Near};
use crate::math::vec::{Vec2, Vec3i, Vec3u};
use crate::math::Angle;
use crate::ops::{Cross, Dot, Midpoint};
use std::borrow::{Borrow, BorrowMut};
use std::ops::{
  Add, AddAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Rem,
  RemAssign, Sub, SubAssign,
};

/// A 3-component view of a vector-like object.
///
/// [`Vec3`] objects are to [`Vector3`] as [`str`] is to [`String`]; that is to
/// say that [`Vec3`] objects represent an immutable view of the owning
/// [`Vector3`] counter-part.
#[repr(transparent)]
#[derive(PartialEq, PartialOrd)]
pub struct Vec3([f32]);

impl Vec3 {
  /// Forms a reference to a [`Vec3`] from a slice of [`f32`].
  ///
  /// This requires that `slice.len() == 3`, otherwise this returns [`None`].
  ///
  /// # Arguments
  ///
  /// * `slice` - the slice of [`f32`]s.
  pub const fn from_slice(slice: &[f32]) -> Option<&Self> {
    if slice.len() == 3 {
      // SAFETY: Vec3 is transparent, and implemented directly in terms of a
      //         slice of f32s. The representation is the same, and thus valid.
      //         This is implemented symmetrically to `OsStr`.
      Some(unsafe { std::mem::transmute(slice) })
    } else {
      None
    }
  }

  /// Forms a mutable reference to a [`Vec3`] from a mutable slice of [`f32`].
  ///
  /// This requires that `slice.len() == 3`, otherwise this returns [`None`].
  ///
  /// # Arguments
  ///
  /// * `slice` - the mutable slice of [`f32`]s.
  pub fn from_mut_slice(slice: &mut [f32]) -> Option<&mut Self> {
    if slice.len() == 3 {
      // SAFETY: Vec3 is transparent, and implemented directly in terms of a
      //         slice of f32s. The representation is the same, and thus valid.
      //         This is implemented symmetrically to `OsStr`.
      Some(unsafe { std::mem::transmute(slice) })
    } else {
      None
    }
  }

  /// Forms a reference to a [`Vec3`] from a slice of [`f32`] that is assumed to
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
    debug_assert!(slice.len() == 3);
    // SAFETY:
    //   Vec3 is transparent, and implemented directly in terms of a
    //   slice of f32s. The representation is the same, and thus valid.
    //   This is implemented symmetrically to `OsStr`.
    unsafe { std::mem::transmute(slice) }
  }

  /// Forms a mutable reference to a [`Vec3`] from a slice of [`f32`] that is
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
    debug_assert!(slice.len() == 3);
    // SAFETY: See from_slice_unchecked
    unsafe { std::mem::transmute(slice) }
  }

  /// Forms a reference to a [`Vec3`] from a pointer to a contiguous sequence
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
  pub const unsafe fn from_ptr_unchecked<'a>(ptr: *const f32) -> &'a Vec3 {
    Vec3::from_slice_unchecked(std::slice::from_raw_parts(ptr, 3))
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
  #[inline(always)]
  pub unsafe fn from_mut_ptr_unchecked<'a>(ptr: *mut f32) -> &'a mut Vec3 {
    Vec3::from_mut_slice_unchecked(std::slice::from_raw_parts_mut(ptr, 3))
  }

  /// Returns this [`Vec3`] as a slice of [`f32`].
  #[inline(always)]
  pub const fn as_slice(&self) -> &[f32] {
    &self.0
  }

  /// Returns this [`Vec3`] as a mutable slice of [`f32`].
  #[inline(always)]
  pub fn as_mut_slice(&mut self) -> &mut [f32] {
    &mut self.0
  }

  /// Returns the X-coordinate of this 3-component vector.
  #[inline(always)]
  pub const fn x(&self) -> f32 {
    unsafe { *self.0.as_ptr() }
  }

  /// Returns the Y-coordinate of this 3-component vector.
  #[inline(always)]
  pub const fn y(&self) -> f32 {
    unsafe { *self.0.as_ptr().add(1) }
  }

  /// Returns the Z-coordinate of this 3-component vector.
  #[inline(always)]
  pub const fn z(&self) -> f32 {
    unsafe { *self.0.as_ptr().add(2) }
  }

  /// Returns the xy coordinates of this vector as a [`Vec3`].
  #[inline(always)]
  pub const fn xy(&self) -> &Vec2 {
    unsafe { Vec2::from_ptr_unchecked(self.0.as_ptr()) }
  }

  /// Returns the yz coordinates of this vector as a [`Vec3`].
  #[inline(always)]
  pub const fn yz(&self) -> &Vec2 {
    unsafe { Vec2::from_ptr_unchecked(self.0.as_ptr().add(1)) }
  }

  /// Returns a mutable reference to the X-coordinate of this 3-component vector.
  #[inline(always)]
  pub fn x_mut(&mut self) -> &mut f32 {
    unsafe { &mut *self.0.as_mut_ptr() }
  }

  /// Returns a mutable reference to the Y-coordinate of this 3-component vector.
  #[inline(always)]
  pub fn y_mut(&mut self) -> &mut f32 {
    unsafe { &mut *self.0.as_mut_ptr().add(1) }
  }

  /// Returns a mutable reference to the Z-coordinate of this 3-component vector.
  #[inline(always)]
  pub fn z_mut(&mut self) -> &mut f32 {
    unsafe { &mut *self.0.as_mut_ptr().add(2) }
  }

  /// Returns a mutable reference to the xy coordinates of this vector.
  #[inline(always)]
  pub fn xy_mut(&mut self) -> &mut Vec2 {
    unsafe { Vec2::from_mut_ptr_unchecked(self.0.as_mut_ptr()) }
  }

  /// Returns a mutable reference to the yz coordinates of this vector.
  #[inline(always)]
  pub fn yz_mut(&mut self) -> &mut Vec2 {
    unsafe { Vec2::from_mut_ptr_unchecked(self.0.as_mut_ptr().add(1)) }
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
  /// * `z` - the Z-component
  #[inline(always)]
  pub fn set_z(&mut self, z: f32) {
    unsafe { *self.0.as_mut_ptr().add(2) = z }
  }

  /// Sets the X and Y components of this vector
  ///
  /// # Arguments
  ///
  /// * `xy` - the X and Y components o
  #[inline(always)]
  pub fn set_xy(&mut self, xy: &Vec2) {
    self.xy_mut().set(xy)
  }

  /// Sets the Y and Z components of this vector
  ///
  /// # Arguments
  ///
  /// * `yz` - the Y and Z components of the [`Vec3`]
  #[inline(always)]
  pub fn set_yz(&mut self, xy: &Vec2) {
    self.yz_mut().set(xy)
  }

  /// Sets all the components of this vector the values from other.
  ///
  /// # Arguments
  ///
  /// * `other` - the other [`Vec3`] to set.
  pub fn set(&mut self, other: &Vec3) {
    let src_ptr = other.as_ptr();
    let dest_ptr = self.0.as_mut_ptr();

    unsafe {
      for i in 0..3 {
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
  /// Where possible, consider using [`Vec3::square_magnitude`] as this will
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

  /// Returns normalized vector.
  pub fn normalized(&self) -> Vector3 {
    self / self.magnitude()
  }

  /// Rotates this vector around the X axis by angle A.
  ///
  /// # Arguments
  ///
  /// * `angle` - the angle to rotate
  pub fn rotate_x<A: Angle>(&mut self, angle: A) {
    let (cos, sin) = angle.sin_cos();
    let y = cos * self.y() - sin * self.z();
    let z = sin * self.y() + cos * self.z();

    self.set_y(y);
    self.set_z(z);
  }

  /// Rotates this vector around the Y axis by angle A.
  ///
  /// # Arguments
  ///
  /// * `angle` - the angle to rotate
  pub fn rotate_y<A: Angle>(&mut self, angle: A) {
    let (cos, sin) = angle.sin_cos();
    let x = cos * self.x() + sin * self.z();
    let z = -sin * self.x() + cos * self.z();

    self.set_x(x);
    self.set_z(z);
  }

  /// Rotates this vector around the Y axis by angle A.
  ///
  /// # Arguments
  ///
  /// * `angle` - the angle to rotate
  pub fn rotate_z<A: Angle>(&mut self, angle: A) {
    let (cos, sin) = angle.sin_cos();
    let x = cos * self.x() - sin * self.y();
    let y = sin * self.x() + cos * self.y();

    self.set_x(x);
    self.set_y(y);
  }

  /// Returns whether all components of this Vec are finite.
  pub fn is_finite(&self) -> bool {
    self.x().is_finite() && self.y().is_finite() && self.z().is_finite()
  }

  /// Returns whether any component of this vec are infinite.
  pub fn is_infinite(&self) -> bool {
    self.x().is_infinite() || self.y().is_infinite() || self.z().is_infinite()
  }

  /// Returns whether any component of this vec are nan.
  pub fn is_nan(&self) -> bool {
    self.x().is_nan() || self.y().is_nan() || self.z().is_nan()
  }

  /// Computes the absolute value of `self`
  pub fn abs(&self) -> Vector3 {
    Vector3 {
      x: self.x().abs(),
      y: self.y().abs(),
      z: self.z().abs(),
    }
  }
}

impl Midpoint for Vec3 {
  type Output = Vector3;

  fn midpoint(&self, other: &Self) -> Self::Output {
    Vector3 {
      x: (self.x() + other.x()) * 0.5,
      y: (self.y() + other.y()) * 0.5,
      z: (self.z() + other.z()) * 0.5,
    }
  }
}

impl Near for Vec3 {
  fn near(&self, other: &Self, tolerance: &f32) -> bool {
    self.x().near(&other.x(), tolerance)
      && self.y().near(&other.y(), tolerance)
      && self.z().near(&other.z(), tolerance)
  }
}

impl AlmostEq for Vec3 {
  fn almost_eq(&self, other: &Self) -> bool {
    const EPSILON: f32 = 10.0 * std::f32::EPSILON;
    self.near(other, &EPSILON)
  }
}

impl<I> Index<I> for Vec3
where
  I: std::slice::SliceIndex<[f32]>,
{
  type Output = I::Output;

  #[inline(always)]
  fn index(&self, index: I) -> &Self::Output {
    self.0.index(index)
  }
}

impl<I> IndexMut<I> for Vec3
where
  I: std::slice::SliceIndex<[f32]>,
{
  #[inline(always)]
  fn index_mut(&mut self, index: I) -> &mut Self::Output {
    self.0.index_mut(index)
  }
}

impl Deref for Vec3 {
  type Target = [f32];

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Vec3 {
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl AsRef<[f32]> for Vec3 {
  #[inline(always)]
  fn as_ref(&self) -> &[f32] {
    &self.0
  }
}

impl AsMut<[f32]> for Vec3 {
  #[inline(always)]
  fn as_mut(&mut self) -> &mut [f32] {
    &mut self.0
  }
}

impl Dot for Vec3 {
  type Output = f32;
  fn dot(&self, rhs: &Self) -> Self::Output {
    self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
  }
}

impl Cross for Vec3 {
  type Output = Vector3;
  fn cross(&self, other: &Self) -> Self::Output {
    Vector3 {
      x: self.y() * other.z() - self.z() * other.y(),
      y: self.z() * other.x() - self.x() * other.z(),
      z: self.x() * other.y() - self.y() * other.x(),
    }
  }
}

impl Add for &'_ Vec3 {
  type Output = Vector3;

  fn add(self, rhs: Self) -> Self::Output {
    Vector3 {
      x: self.x() + rhs.x(),
      y: self.y() + rhs.y(),
      z: self.z() + rhs.z(),
    }
  }
}

impl AddAssign<&Vec3> for Vec3 {
  fn add_assign(&mut self, rhs: &Vec3) {
    let dest_ptr = self.0.as_mut_ptr();
    let src_ptr = rhs.0.as_ptr();

    unsafe {
      for i in 0..3 {
        *dest_ptr.add(i) += *src_ptr.add(i)
      }
    }
  }
}

impl Sub for &'_ Vec3 {
  type Output = Vector3;

  fn sub(self, rhs: Self) -> Self::Output {
    Vector3 {
      x: self.x() - rhs.x(),
      y: self.y() - rhs.y(),
      z: self.z() - rhs.z(),
    }
  }
}

impl SubAssign<&Vec3> for Vec3 {
  fn sub_assign(&mut self, rhs: &Vec3) {
    let dest_ptr = self.0.as_mut_ptr();
    let src_ptr = rhs.0.as_ptr();

    unsafe {
      for i in 0..3 {
        *dest_ptr.add(i) -= *src_ptr.add(i)
      }
    }
  }
}

impl Mul<f32> for &'_ Vec3 {
  type Output = Vector3;

  fn mul(self, rhs: f32) -> Self::Output {
    Vector3 {
      x: self.x() * rhs,
      y: self.y() * rhs,
      z: self.z() * rhs,
    }
  }
}

impl Mul<&'_ Vec3> for f32 {
  type Output = Vector3;

  fn mul(self, rhs: &'_ Vec3) -> Self::Output {
    Vector3 {
      x: self * rhs.x(),
      y: self * rhs.y(),
      z: self * rhs.z(),
    }
  }
}

impl MulAssign<f32> for Vec3 {
  fn mul_assign(&mut self, rhs: f32) {
    let dest_ptr = self.0.as_mut_ptr();

    unsafe {
      for i in 0..3 {
        *dest_ptr.add(i) *= rhs
      }
    }
  }
}

impl Div<f32> for &'_ Vec3 {
  type Output = Vector3;

  fn div(self, rhs: f32) -> Self::Output {
    let inverse = 1.0 / rhs;
    Vector3 {
      x: self.x() * inverse,
      y: self.y() * inverse,
      z: self.z() * inverse,
    }
  }
}

impl DivAssign<f32> for Vec3 {
  fn div_assign(&mut self, rhs: f32) {
    let dest_ptr = self.0.as_mut_ptr();

    let inverse = 1.0 / rhs;
    unsafe {
      for i in 0..3 {
        *dest_ptr.add(i) *= inverse
      }
    }
  }
}

impl Rem<f32> for &'_ Vec3 {
  type Output = Vector3;

  fn rem(self, rhs: f32) -> Self::Output {
    Vector3 {
      x: self.x().rem(rhs),
      y: self.y().rem(rhs),
      z: self.z().rem(rhs),
    }
  }
}

impl RemAssign<f32> for Vec3 {
  fn rem_assign(&mut self, rhs: f32) {
    let dest_ptr = self.0.as_mut_ptr();

    unsafe {
      for i in 0..3 {
        *dest_ptr.add(i) %= rhs
      }
    }
  }
}

impl Neg for &'_ Vec3 {
  type Output = Vector3;

  fn neg(self) -> Self::Output {
    Vector3 {
      x: -self.x(),
      y: -self.y(),
      z: -self.z(),
    }
  }
}

impl std::fmt::Debug for Vec3 {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Vec3")
      .field("x", &self.x())
      .field("y", &self.y())
      .field("z", &self.z())
      .finish()
  }
}

impl std::fmt::Display for Vec3 {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{{{}, {}, {}}}", self.x(), self.y(), self.z())
  }
}

/// An owning representation of a 2-dimensional Vector object.
///
/// Unlike [`Vec3`], which is solely referential, [`Vector3`] is an owning
/// instance.
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, PartialOrd, Debug)]
pub struct Vector3 {
  /// The X-component of the vector.
  pub x: f32,
  /// The Y-component of the vector.
  pub y: f32,
  /// The Z-component of the vector.
  pub z: f32,
}

impl Vector3 {
  /// A constant for a vector of magnitude 0 at the origin.
  pub const ZERO: Vector3 = Vector3::new(0.0, 0.0, 0.0);

  /// A constant for a unit vector in the positive X-direction.
  pub const UNIT_X: Vector3 = Vector3::new(1.0, 0.0, 0.0);

  /// A constant for a unit vector in the positive Y-direction.
  pub const UNIT_Y: Vector3 = Vector3::new(0.0, 1.0, 0.0);

  /// A constant for a unit vector in the positive Z-direction.
  pub const UNIT_Z: Vector3 = Vector3::new(0.0, 0.0, 1.0);

  /// A constant for a unit vector in the negative X-direction.
  pub const NEG_UNIT_X: Vector3 = Vector3::new(-1.0, 0.0, 0.0);

  /// A constant for a unit vector in the negative Y-direction.
  pub const NEG_UNIT_Y: Vector3 = Vector3::new(0.0, -1.0, 0.0);

  /// A constant for a unit vector in the negative Z-direction.
  pub const NEG_UNIT_Z: Vector3 = Vector3::new(0.0, 0.0, -1.0);

  /// Constructs this vector from an x, y, and z coordinate.
  ///
  /// # Arguments
  ///
  /// * `x` - the x-component
  /// * `y` - the y-component
  /// * `z` - the z-component
  #[inline(always)]
  pub const fn new(x: f32, y: f32, z: f32) -> Self {
    Self { x, y, z }
  }

  /// Constructs this vector with a uniform value `v`.
  ///
  /// # Arguments
  ///
  /// * `v` - the value to uniformly apply
  #[inline(always)]
  pub const fn uniform(v: f32) -> Self {
    Self::new(v, v, v)
  }

  /// Constructs this vector from a slice of floats.
  ///
  /// This will return [`None`] if `slice.len()` is not equal to 2.
  ///
  /// # Arguments
  ///
  /// * `slice` - the slice to read from
  pub const fn from_slice(slice: &[f32]) -> Option<Self> {
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
  pub const unsafe fn from_slice_unchecked(slice: &[f32]) -> Self {
    debug_assert!(slice.len() == 3);
    Self::from_ptr(slice.as_ptr())
  }

  /// Constructs this vector from a [`Vec3`]
  ///
  /// # Arguments
  ///
  /// * `other` - the other vector
  pub const fn from_vec3(other: &Vec3) -> Self {
    Self {
      x: other.x(),
      y: other.y(),
      z: other.z(),
    }
  }

  /// Constructs this vector from a [`Vec3i`]
  ///
  /// # Arguments
  ///
  /// * `other` - the other vector
  pub fn from_vec3i(other: &Vec3i) -> Self {
    Self {
      x: other.x() as f32,
      y: other.y() as f32,
      z: other.z() as f32,
    }
  }

  /// Constructs this vector from a [`Vec3u`]
  ///
  /// # Arguments
  ///
  /// * `other` - the other vector
  pub fn from_vec3u(other: &Vec3u) -> Self {
    Self {
      x: other.x() as f32,
      y: other.y() as f32,
      z: other.z() as f32,
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
  /// contiguous sequence of 3 [`f32`] values.
  pub const unsafe fn from_ptr(ptr: *const f32) -> Self {
    Self::new(*ptr, *ptr.add(1), *ptr.add(2))
  }

  /// Returns this vector as a [`Vec3`].
  #[inline(always)]
  pub const fn as_vec3(&self) -> &Vec3 {
    // SAFETY:
    //
    //   Vector3 is repr(C) and thus points to two contiguous elements
    //   of type and align of `f32`. The only pointer capable of accessing both
    //   entries within its memory region is a pointer to itself (`*const _`).
    //   Thus, we alias this to `f32` -- which under `repr(C)` points to the
    //   first element, and has proper reachability into its neighbor-element.
    unsafe {
      std::mem::transmute(std::slice::from_raw_parts(
        self as *const _ as *const f32,
        3,
      ))
    }
  }

  /// Returns this vector as a mutable [`Vec3`].
  #[inline(always)]
  pub fn as_mut_vec3(&mut self) -> &mut Vec3 {
    // SAFETY: See explanation in Borrow<Vec3>
    unsafe {
      std::mem::transmute(std::slice::from_raw_parts_mut(
        self as *mut _ as *mut f32,
        3,
      ))
    }
  }

  /// Returns this vector as a slice of [`f32`].
  #[inline(always)]
  pub const fn as_slice(&self) -> &[f32] {
    self.as_vec3().as_slice()
  }

  /// Returns this vector as a mutable slice of [`f32`].
  #[inline(always)]
  pub fn as_mut_slice(&mut self) -> &mut [f32] {
    self.as_mut_vec3().as_mut_slice()
  }
}

impl<Vec> From<Vec> for Vector3
where
  Vec: AsRef<Vec3>,
{
  #[inline(always)]
  fn from(value: Vec) -> Self {
    value.as_ref().to_owned()
  }
}

impl From<&'_ Vec3> for Vector3 {
  #[inline(always)]
  fn from(value: &'_ Vec3) -> Self {
    value.to_owned()
  }
}

impl Add for &Vector3 {
  type Output = Vector3;

  #[inline(always)]
  fn add(self, rhs: Self) -> Self::Output {
    self.as_vec3().add(rhs.as_vec3())
  }
}

impl Add for Vector3 {
  type Output = Vector3;

  #[inline(always)]
  fn add(self, rhs: Self) -> Self::Output {
    self.add(rhs.as_vec3())
  }
}

impl Add<&Vec3> for &Vector3 {
  type Output = Vector3;

  #[inline(always)]
  fn add(self, rhs: &Vec3) -> Self::Output {
    self.as_vec3().add(rhs)
  }
}

impl Add<&Vector3> for &'_ Vec3 {
  type Output = Vector3;

  #[inline(always)]
  fn add(self, rhs: &Vector3) -> Self::Output {
    self.add(rhs.as_vec3())
  }
}

impl Add<Vector3> for &Vec3 {
  type Output = Vector3;

  #[inline(always)]
  fn add(self, rhs: Vector3) -> Self::Output {
    // Addition is commutative, so reordering operations is safe
    rhs.add(self)
  }
}

impl Add<&Vec3> for Vector3 {
  type Output = Vector3;

  fn add(mut self, rhs: &Vec3) -> Self::Output {
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

impl Add<&Vector3> for Vector3 {
  type Output = Vector3;

  #[inline(always)]
  fn add(self, rhs: &Vector3) -> Self::Output {
    // Addition is commutative, so reordering operations is safe
    rhs.add(self.as_vec3())
  }
}

impl Add<Vector3> for &Vector3 {
  type Output = Vector3;

  #[inline(always)]
  fn add(self, rhs: Vector3) -> Self::Output {
    // Addition is commutative, so reordering operations is safe
    rhs.as_vec3().add(self)
  }
}

impl AddAssign for Vector3 {
  #[inline(always)]
  fn add_assign(&mut self, rhs: Self) {
    self.as_mut_vec3().add_assign(&rhs)
  }
}

impl AddAssign<&Vector3> for Vector3 {
  #[inline(always)]
  fn add_assign(&mut self, rhs: &Self) {
    self.as_mut_vec3().add_assign(rhs)
  }
}

impl AddAssign<&Vec3> for Vector3 {
  #[inline(always)]
  fn add_assign(&mut self, rhs: &Vec3) {
    self.as_mut_vec3().add_assign(rhs)
  }
}

impl Sub for &Vector3 {
  type Output = Vector3;

  #[inline(always)]
  fn sub(self, rhs: Self) -> Self::Output {
    self.as_vec3().sub(rhs.as_vec3())
  }
}

impl Sub for Vector3 {
  type Output = Vector3;

  #[inline(always)]
  fn sub(self, rhs: Self) -> Self::Output {
    self.sub(rhs.as_vec3())
  }
}

impl Sub<&Vec3> for &Vector3 {
  type Output = Vector3;

  #[inline(always)]
  fn sub(self, rhs: &Vec3) -> Self::Output {
    self.as_vec3().sub(rhs)
  }
}

impl Sub<&Vector3> for &'_ Vec3 {
  type Output = Vector3;

  #[inline(always)]
  fn sub(self, rhs: &Vector3) -> Self::Output {
    self.sub(rhs.as_vec3())
  }
}

impl Sub<Vector3> for &Vec3 {
  type Output = Vector3;

  #[inline(always)]
  fn sub(self, rhs: Vector3) -> Self::Output {
    self.sub(rhs.as_vec3())
  }
}

impl Sub<&Vec3> for Vector3 {
  type Output = Vector3;

  fn sub(mut self, rhs: &Vec3) -> Self::Output {
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

impl Sub<&Vector3> for Vector3 {
  type Output = Vector3;

  #[inline(always)]
  fn sub(self, rhs: &Vector3) -> Self::Output {
    self.sub(rhs.as_vec3())
  }
}

impl Sub<Vector3> for &Vector3 {
  type Output = Vector3;

  #[inline(always)]
  fn sub(self, rhs: Vector3) -> Self::Output {
    self.sub(rhs.as_vec3())
  }
}

impl SubAssign for Vector3 {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: Self) {
    self.as_mut_vec3().sub_assign(&rhs)
  }
}

impl SubAssign<&Vector3> for Vector3 {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: &Self) {
    self.as_mut_vec3().sub_assign(rhs)
  }
}

impl SubAssign<&Vec3> for Vector3 {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: &Vec3) {
    self.as_mut_vec3().sub_assign(rhs)
  }
}

impl Mul<f32> for Vector3 {
  type Output = Vector3;

  #[inline(always)]
  fn mul(mut self, rhs: f32) -> Self::Output {
    self.as_mut_vec3().mul_assign(rhs);
    self
  }
}

impl Mul<f32> for &Vector3 {
  type Output = Vector3;

  #[inline(always)]
  fn mul(self, rhs: f32) -> Self::Output {
    self.as_vec3().mul(rhs)
  }
}

impl Mul<Vector3> for f32 {
  type Output = Vector3;

  #[inline(always)]
  fn mul(self, mut rhs: Vector3) -> Self::Output {
    rhs.as_mut_vec3().mul_assign(self);
    rhs
  }
}

impl Mul<&Vector3> for f32 {
  type Output = Vector3;

  #[inline(always)]
  fn mul(self, rhs: &Vector3) -> Self::Output {
    rhs.as_vec3().mul(self)
  }
}

impl MulAssign<f32> for Vector3 {
  #[inline(always)]
  fn mul_assign(&mut self, rhs: f32) {
    self.as_mut_vec3().mul_assign(rhs)
  }
}

impl Div<f32> for Vector3 {
  type Output = Vector3;

  #[inline(always)]
  fn div(mut self, rhs: f32) -> Self::Output {
    self.as_mut_vec3().div_assign(rhs);
    self
  }
}

impl Div<f32> for &Vector3 {
  type Output = Vector3;

  #[inline(always)]
  fn div(self, rhs: f32) -> Self::Output {
    self.as_vec3().div(rhs)
  }
}

impl DivAssign<f32> for Vector3 {
  #[inline(always)]
  fn div_assign(&mut self, rhs: f32) {
    self.as_mut_vec3().div_assign(rhs)
  }
}

impl Rem<f32> for Vector3 {
  type Output = Vector3;

  #[inline(always)]
  fn rem(mut self, rhs: f32) -> Self::Output {
    self.as_mut_vec3().rem_assign(rhs);
    self
  }
}

impl Rem<f32> for &Vector3 {
  type Output = Vector3;

  #[inline(always)]
  fn rem(self, rhs: f32) -> Self::Output {
    self.as_vec3().rem(rhs)
  }
}

impl RemAssign<f32> for Vector3 {
  #[inline(always)]
  fn rem_assign(&mut self, rhs: f32) {
    self.as_mut_vec3().rem_assign(rhs)
  }
}

impl Deref for Vector3 {
  type Target = Vec3;

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    self.borrow()
  }
}

impl DerefMut for Vector3 {
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.borrow_mut()
  }
}

impl Borrow<Vec3> for Vector3 {
  #[inline(always)]
  fn borrow(&self) -> &Vec3 {
    self.as_vec3()
  }
}

impl BorrowMut<Vec3> for Vector3 {
  #[inline(always)]
  fn borrow_mut(&mut self) -> &mut Vec3 {
    self.as_mut_vec3()
  }
}

impl Borrow<[f32]> for Vector3 {
  #[inline(always)]
  fn borrow(&self) -> &[f32] {
    <Self as Borrow<Vec3>>::borrow(self).as_ref()
  }
}

impl BorrowMut<[f32]> for Vector3 {
  #[inline(always)]
  fn borrow_mut(&mut self) -> &mut [f32] {
    <Self as BorrowMut<Vec3>>::borrow_mut(self).as_mut()
  }
}

impl ToOwned for Vec3 {
  type Owned = Vector3;

  #[inline(always)]
  fn to_owned(&self) -> Self::Owned {
    Vector3 {
      x: self.x(),
      y: self.y(),
      z: self.z(),
    }
  }
}

impl std::fmt::Display for Vector3 {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{{{}, {}, {}}}", self.x, self.y, self.z)
  }
}

#[cfg(test)]
mod test {
  use crate::cmp::AlmostEq;

  use super::*;

  #[test]
  fn test_vec3() {
    let vec = Vector3::new(4.0, 2.0, 0.0);

    use crate::cmp::AlmostEq;
    let magnitude = vec.square_magnitude();

    assert!(magnitude.almost_eq(&20.0))
  }

  #[test]
  fn test_identity() {
    let vec = Vector3::new(4.0, 2.0, 0.0);

    assert_eq!(vec.as_ptr(), &vec[0]);
    assert_eq!(vec.as_ptr(), &vec.x);
  }

  #[test]
  fn test_iter() {
    let mut vec = Vector3::new(4.0, 2.0, 1.0);

    for v in vec.iter_mut() {
      *v = *v * 2.0
    }

    assert!(vec.x.almost_eq(&8.0), "x = {}", vec.x);
    assert!(vec.y.almost_eq(&4.0), "y = {}", vec.y);
    assert!(vec.z.almost_eq(&2.0), "z = {}", vec.z);
  }

  #[test]
  fn test_vec3_add() {
    let mut lhs = Vector3::new(4.0, 2.0, 1.0);
    lhs *= 4.0;
    lhs += lhs.clone();
    lhs += Vector3::new(1.0, 1.0, 1.0);
    assert!(lhs.dot(&Vector3::new(1.0, 1.0, 1.0)) != 0.0);
  }
}
