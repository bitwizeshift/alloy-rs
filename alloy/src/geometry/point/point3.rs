use crate::math::vec::{Vec3, Vec4, Vector3};
use std::fmt;
use std::ops::{Add, Deref, DerefMut, Div, Mul, Sub};

/// A point in 3D space.
///
/// This type is used to disambiguate geometric uses of "points" representing
/// explicit points in space from "vectors" representing displacements or
/// directions.
///
/// Points can implicitly deref into [`Vec3`] to allow for easily using points
/// in places where vectors are expected.
#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Point3(Vector3);

// Constructors
impl Point3 {
  /// The origin point
  pub const ORIGIN: Point3 = Self::new(0.0, 0.0, 0.0);

  /// Create a new point
  #[inline(always)]
  pub const fn new(x: f32, y: f32, z: f32) -> Self {
    Self(Vector3::new(x, y, z))
  }

  /// Create a new point from a vector
  ///
  /// # Parameters
  ///
  /// * `v` - The vector to create the point from
  #[inline(always)]
  pub const fn from_vec3(v: &Vec3) -> Self {
    Self::new(v.x(), v.y(), v.z())
  }

  /// Create a new point from a 4-component vector
  ///
  /// This discards the w coordinate.
  ///
  /// # Parameters
  ///
  /// * `v` - The vector to create the point from
  #[inline(always)]
  pub const fn from_vec4(v: &Vec4) -> Self {
    Self::from_vec3(v.xyz())
  }

  /// Convert the point to a vector
  #[inline(always)]
  pub const fn to_vector3(&self) -> Vector3 {
    self.0
  }
}

impl From<&Vec3> for Point3 {
  #[inline(always)]
  fn from(v: &Vec3) -> Self {
    Self::from_vec3(v)
  }
}

impl From<Vector3> for Point3 {
  #[inline(always)]
  fn from(v: Vector3) -> Self {
    Self::from_vec3(&v)
  }
}

impl Vector3 {
  /// Create a new vector from two points
  ///
  /// # Parameters
  ///
  /// * `from` - The point to begin drawing the vector from
  /// * `to` - The point to end drawing the vector at`
  pub fn from_points(from: &Point3, to: &Point3) -> Self {
    let (from, to) = (from.as_vec3(), to.as_vec3());

    Vector3::new(to.x() - from.x(), to.y() - from.y(), to.z() - from.z())
  }
}

impl From<Point3> for Vector3 {
  #[inline(always)]
  fn from(p: Point3) -> Self {
    p.0
  }
}

// Conversion
impl Point3 {
  /// Get the x-coordinate of the point
  #[inline(always)]
  pub const fn as_vec3(&self) -> &Vec3 {
    self.0.as_vec3()
  }

  /// Get the x-coordinate of the point
  #[inline(always)]
  pub fn as_mut_vec3(&mut self) -> &mut Vec3 {
    self.0.as_mut_vec3()
  }
}

impl AsRef<Vec3> for Point3 {
  #[inline(always)]
  fn as_ref(&self) -> &Vec3 {
    self.as_vec3()
  }
}

impl AsMut<Vec3> for Point3 {
  #[inline(always)]
  fn as_mut(&mut self) -> &mut Vec3 {
    self.as_mut_vec3()
  }
}

impl Deref for Point3 {
  type Target = Vec3;

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    self.as_vec3()
  }
}

impl DerefMut for Point3 {
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.as_mut_vec3()
  }
}

// Properties
impl Point3 {
  /// Get the minimum values between two points
  ///
  /// # Parameters
  ///
  /// * `other` - The other point to compare against
  #[inline]
  pub fn min(&self, other: &Point3) -> Point3 {
    Self(self.0.min(&other.0))
  }

  /// Get the maximum values between two points
  ///
  /// # Parameters
  ///
  /// * `other` - The other point to compare against
  #[inline]
  pub fn max(&self, other: &Point3) -> Point3 {
    Self(self.0.max(&other.0))
  }

  /// Get the distance between two points
  ///
  /// # Parameters
  ///
  /// * `other` - The other point to get the distance to
  pub fn distance_to(&self, other: &Point3) -> f32 {
    (other.0 - self.0).magnitude()
  }

  /// Get the square distance between two points
  ///
  /// # Parameters
  ///
  /// * `other` - The other point to get the square distance to
  pub fn square_distance_to(&self, other: &Point3) -> f32 {
    (other.0 - self.0).square_magnitude()
  }
}

// Arithmetic

impl Add<&Vec3> for Point3 {
  type Output = Point3;

  #[inline(always)]
  fn add(self, rhs: &Vec3) -> Self::Output {
    Point3(self.0 + rhs)
  }
}

impl Add<Point3> for &Vec3 {
  type Output = Point3;

  #[inline(always)]
  fn add(self, rhs: Point3) -> Self::Output {
    rhs + self
  }
}

impl Add<&Point3> for &Vec3 {
  type Output = Point3;

  #[inline(always)]
  fn add(self, rhs: &Point3) -> Self::Output {
    *rhs + self
  }
}

impl Add for Point3 {
  type Output = Point3;

  #[inline(always)]
  fn add(self, rhs: Self) -> Self::Output {
    Point3(self.0 + rhs.0)
  }
}

impl Sub for Point3 {
  type Output = Vector3;

  #[inline(always)]
  fn sub(self, rhs: Self) -> Self::Output {
    self.0 - rhs.0
  }
}

impl Sub for &Point3 {
  type Output = Vector3;

  #[inline(always)]
  fn sub(self, rhs: Self) -> Self::Output {
    self.0 - rhs.0
  }
}

impl Sub<&Vec3> for Point3 {
  type Output = Point3;

  #[inline(always)]
  fn sub(self, rhs: &Vec3) -> Self::Output {
    Point3(self.0 - rhs)
  }
}

impl Sub<&Vec3> for &Point3 {
  type Output = Point3;

  #[inline(always)]
  fn sub(self, rhs: &Vec3) -> Self::Output {
    Point3(self.0 - rhs)
  }
}

impl Mul<f32> for Point3 {
  type Output = Point3;

  #[inline(always)]
  fn mul(self, rhs: f32) -> Self::Output {
    Point3(self.0 * rhs)
  }
}

impl Mul<f32> for &Point3 {
  type Output = Point3;

  #[inline(always)]
  fn mul(self, rhs: f32) -> Self::Output {
    Point3(self.0 * rhs)
  }
}

impl Mul<Point3> for f32 {
  type Output = Point3;

  #[inline(always)]
  fn mul(self, rhs: Point3) -> Self::Output {
    rhs * self
  }
}

impl Mul<&Point3> for f32 {
  type Output = Point3;

  #[inline(always)]
  fn mul(self, rhs: &Point3) -> Self::Output {
    rhs * self
  }
}

impl Div<f32> for Point3 {
  type Output = Point3;

  #[inline(always)]
  fn div(self, rhs: f32) -> Self::Output {
    Point3(self.0 / rhs)
  }
}

impl Div<f32> for &Point3 {
  type Output = Point3;

  #[inline(always)]
  fn div(self, rhs: f32) -> Self::Output {
    Point3(self.0 / rhs)
  }
}

impl fmt::Debug for Point3 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Point3")
      .field("x", &self.0.x())
      .field("y", &self.0.y())
      .field("z", &self.0.z())
      .finish()
  }
}

#[cfg(test)]
#[path = "point3.test.rs"]
mod test;
