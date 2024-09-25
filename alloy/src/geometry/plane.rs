use std::ops::{Deref, DerefMut};

use crate::cmp::AlmostEq;
use crate::math::vec::{Vec2, Vec3, Vec4, Vector4};
use crate::ops::{Cross, Dot};

use super::Point3;

/// A value type that represents a plane in 3D space with the equation:
/// `a*x + b*y + c*z + d = 0`.
///
/// Points can implicitly deref into [`Vec4`] to allow for easily using points
/// in places where vectors are expected.
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, PartialOrd, Debug)]
pub struct Plane {
  data: Vector4,
}

// Construction

impl Plane {
  /// Creates a new plane with the given coefficients.
  ///
  /// # Parameters
  ///
  /// * `a` - The first component of the plane
  /// * `b` - The second component of the plane
  /// * `c` - The third component of the plane
  /// * `d` - The fourth component of the plane
  #[inline(always)]
  pub const fn new(a: f32, b: f32, c: f32, d: f32) -> Self {
    Self {
      data: Vector4::new(a, b, c, d),
    }
  }

  /// Create a new plane from a normal and distance
  ///
  /// # Parameters
  ///
  /// * `normal` - The normal of the plane
  /// * `distance` - The distance of the plane
  #[inline]
  pub const fn from_normal_distance(normal: &Vec3, distance: f32) -> Self {
    Self {
      data: Vector4::new(normal.x(), normal.y(), normal.z(), distance),
    }
  }

  /// Create a new plane from a point and normal
  ///
  /// # Parameters
  ///
  /// * `point` - The point on the plane
  /// * `normal` - The normal of the plane
  pub fn from_point_and_normal(point: &Vec3, normal: &Vec3) -> Self {
    let d = -point.dot(normal);
    Self {
      data: Vector4::new(normal.x(), normal.y(), normal.z(), d),
    }
  }

  /// Create a new plane from three points in counter-clockwise order.
  ///
  /// # Parameters
  ///
  /// * `p0` - The first point
  /// * `p1` - The second point
  /// * `p2` - The third point
  pub fn from_points_clockwise(p0: &Vec3, p1: &Vec3, p2: &Vec3) -> Self {
    let normal = (p2 - p0).cross(&(p1 - p0)).normalized();

    Self::from_point_and_normal(p0, &normal)
  }

  /// Create a new plane from three points in counter-clockwise order.
  /// This is the same as `from_points_clockwise` but with the points in a different order.
  ///
  /// # Parameters
  ///
  /// * `p0` - The first point
  /// * `p1` - The second point
  /// * `p2` - The third point
  pub fn from_points_counter_clockwise(p0: &Vec3, p1: &Vec3, p2: &Vec3) -> Self {
    let normal = (p1 - p0).cross(&(p2 - p0)).normalized();

    Self::from_point_and_normal(p0, &normal)
  }
}

// Getters / Setters
impl Plane {
  /// Gets the 'a' component of the plane
  #[inline(always)]
  pub const fn a(&self) -> f32 {
    self.data.as_vec4().x()
  }

  /// Gets the 'b' component of the plane
  #[inline(always)]
  pub const fn b(&self) -> f32 {
    self.data.as_vec4().y()
  }

  /// Gets the 'c' component of the plane
  #[inline(always)]
  pub const fn c(&self) -> f32 {
    self.data.as_vec4().z()
  }

  /// Gets the 'd' component of the plane
  #[inline(always)]
  pub const fn d(&self) -> f32 {
    self.data.as_vec4().w()
  }

  /// Gets the 'a' and 'b' components of the plane
  /// as a 2D vector
  #[inline(always)]
  pub const fn ab(&self) -> &Vec2 {
    self.data.as_vec4().xy()
  }

  /// Gets the 'b' and 'c' components of the plane
  /// as a 2D vector
  #[inline(always)]
  pub const fn bc(&self) -> &Vec2 {
    self.data.as_vec4().yz()
  }

  /// Gets the 'c' and 'd' components of the plane
  /// as a 2D vector
  #[inline(always)]
  pub const fn cd(&self) -> &Vec2 {
    self.data.as_vec4().zw()
  }

  /// Gets the 'a', 'b', and 'c' components of the plane
  /// as a 3D vector
  #[inline(always)]
  pub const fn abc(&self) -> &Vec3 {
    self.data.as_vec4().xyz()
  }

  /// Gets the 'b', 'c', and 'd' components of the plane
  /// as a 3D vector
  #[inline(always)]
  pub const fn bcd(&self) -> &Vec3 {
    self.data.as_vec4().yzw()
  }

  /// Gets the 'a', 'b', 'c', and 'd' components of the plane
  /// as a 4D vector
  #[inline(always)]
  pub const fn abcd(&self) -> &Vec4 {
    self.data.as_vec4()
  }

  /// Gets the normal of this plane
  #[inline(always)]
  pub const fn normal(&self) -> &Vec3 {
    self.data.as_vec4().xyz()
  }

  /// Gets the 'a' component of the plane
  #[inline(always)]
  pub fn a_mut(&mut self) -> &mut f32 {
    self.data.as_mut_vec4().x_mut()
  }

  /// Gets the 'b' component of the plane
  #[inline(always)]
  pub fn b_mut(&mut self) -> &mut f32 {
    self.data.as_mut_vec4().y_mut()
  }

  /// Gets the 'c' component of the plane
  #[inline(always)]
  pub fn c_mut(&mut self) -> &mut f32 {
    self.data.as_mut_vec4().z_mut()
  }

  /// Gets the 'd' component of the plane
  #[inline(always)]
  pub fn d_mut(&mut self) -> &mut f32 {
    self.data.as_mut_vec4().w_mut()
  }

  /// Gets the 'a' and 'b' components of the plane
  /// as a 2D vector
  #[inline(always)]
  pub fn ab_mut(&mut self) -> &mut Vec2 {
    self.data.as_mut_vec4().xy_mut()
  }

  /// Gets the 'b' and 'c' components of the plane
  /// as a 2D vector
  #[inline(always)]
  pub fn bc_mut(&mut self) -> &mut Vec2 {
    self.data.as_mut_vec4().yz_mut()
  }

  /// Gets the 'c' and 'd' components of the plane
  /// as a 2D vector
  #[inline(always)]
  pub fn cd_mut(&mut self) -> &mut Vec2 {
    self.data.as_mut_vec4().zw_mut()
  }

  /// Gets the 'a', 'b', and 'c' components of the plane
  /// as a 3D vector
  #[inline(always)]
  pub fn abc_mut(&mut self) -> &mut Vec3 {
    self.data.as_mut_vec4().xyz_mut()
  }

  /// Gets the 'b', 'c', and 'd' components of the plane
  /// as a 3D vector
  #[inline(always)]
  pub fn bcd_mut(&mut self) -> &mut Vec3 {
    self.data.as_mut_vec4().yzw_mut()
  }

  /// Gets the 'a', 'b', 'c', and 'd' components of the plane
  /// as a 4D vector
  #[inline(always)]
  pub fn abcd_mut(&mut self) -> &mut Vec4 {
    self.data.as_mut_vec4()
  }

  /// Sets the 'a' component of the plane
  ///
  /// # Parameters
  ///
  /// * `a` - The new value of the 'a' component
  #[inline(always)]
  pub fn set_a(&mut self, a: f32) {
    self.data.set_x(a)
  }

  /// Sets the 'b' component of the plane
  ///
  /// # Parameters
  ///
  /// * `b` - The new value of the 'b' component
  #[inline(always)]
  pub fn set_b(&mut self, b: f32) {
    self.data.set_y(b)
  }

  /// Sets the 'c' component of the plane
  ///
  /// # Parameters
  ///
  /// * `c` - The new value of the 'c' component
  #[inline(always)]
  pub fn set_c(&mut self, c: f32) {
    self.data.set_z(c)
  }

  /// Sets the 'd' component of the plane
  ///
  /// # Parameters
  ///
  /// * `d` - The new value of the 'd' component
  #[inline(always)]
  pub fn set_d(&mut self, d: f32) {
    self.data.set_w(d)
  }

  /// Gets the 'a' and 'b' components of the plane
  ///
  /// # Parameters
  ///
  /// * `ab` - The new value of the 'a' and 'b' components
  #[inline(always)]
  pub fn set_ab(&mut self, ab: &Vec2) {
    self.data.set_xy(ab)
  }

  /// Gets the 'b' and 'c' components of the plane
  ///
  /// # Parameters
  ///
  /// * `bc` - The new value of the 'b' and 'c' components
  #[inline(always)]
  pub fn set_bc(&mut self, bc: &Vec2) {
    self.data.set_yz(bc)
  }

  /// Gets the 'c' and 'd' components of the plane
  ///
  /// # Parameters
  ///
  /// * `cd` - The new value of the 'c' and 'd' components
  #[inline(always)]
  pub fn set_cd(&mut self, cd: &Vec2) {
    self.data.set_yz(cd)
  }

  /// Gets the 'a', 'b', and 'c' components of the plane
  ///
  /// # Parameters
  ///
  /// * `abc` - The new value of the 'a', 'b', and 'c' components
  #[inline(always)]
  pub fn set_abc(&mut self, abc: &Vec3) {
    self.data.set_xyz(abc)
  }

  /// Gets the 'b', 'c', and 'd' components of the plane
  ///
  /// # Parameters
  ///
  /// * `bcd` - The new value of the 'b', 'c', and 'd' components
  #[inline(always)]
  pub fn set_bcd(&mut self, bcd: &Vec3) {
    self.data.set_yzw(bcd)
  }

  /// Gets the 'a', 'b', 'c', and 'd' components of the plane
  ///
  /// # Parameters
  ///
  /// * `abcd` - The new value of the 'a', 'b', 'c', and 'd' components
  #[inline(always)]
  pub fn set(&mut self, abcd: &Vec4) {
    self.data.set(abcd)
  }

  /// Gets the normal of this plane
  #[inline(always)]
  pub const fn as_vec4(&self) -> &Vec4 {
    self.data.as_vec4()
  }

  /// Gets the normal of this plane
  #[inline(always)]
  pub fn as_mut_vec4(&mut self) -> &mut Vec4 {
    self.data.as_mut_vec4()
  }
}

impl AsRef<Vec4> for Plane {
  #[inline(always)]
  fn as_ref(&self) -> &Vec4 {
    self.as_vec4()
  }
}

impl AsMut<Vec4> for Plane {
  #[inline(always)]
  fn as_mut(&mut self) -> &mut Vec4 {
    self.as_mut_vec4()
  }
}

impl Deref for Plane {
  type Target = Vec4;

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    self.as_vec4()
  }
}

impl DerefMut for Plane {
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.as_mut_vec4()
  }
}

// Queries
impl Plane {
  /// Gets the distance to the point from this plane
  ///
  /// # Parameters
  ///
  /// * `point` - The point to get the distance to
  pub fn distance_to_point(&self, point: &Point3) -> f32 {
    use crate::ops::Dot;
    let abc = self.data.as_vec4().xyz();

    abc.dot(point) + self.d()
  }

  /// Checks if the plane contains the point within the default epsilon.
  ///
  /// # Parameters
  ///
  /// * `point` - The point to check
  pub fn contains(&self, point: &Point3) -> bool {
    self.distance_to_point(point).almost_eq(&0.0)
  }

  /// Checks if the point exists over the plane
  ///
  /// # Parameters
  ///
  /// * `point` - The point to check
  pub fn is_point_over_plane(&self, point: &Point3) -> bool {
    self.data.as_vec4().xyz().dot(point) > self.d()
  }

  /// Checks if the point exists under the plane
  ///
  /// # Parameters
  ///
  /// * `point` - The point to check
  pub fn is_point_under_plane(&self, point: &Point3) -> bool {
    self.data.as_vec4().xyz().dot(point) < self.d()
  }

  /// Gets the nearest point on the plane to the given point
  ///
  /// # Parameters
  ///
  /// * `point` - The point to get the nearest point to
  pub fn nearest_point(&self, point: &Point3) -> Point3 {
    let abc = self.data.as_vec4().xyz();

    let t = -(abc.dot(point) + self.d());
    let scaled_normal = abc * t;

    *point + scaled_normal.as_vec3()
  }
}

// Modifiers
impl Plane {
  /// Computes and returns the plane that is this plane's inverse.
  pub fn inverted(&self) -> Self {
    Self::new(-self.a(), -self.b(), -self.c(), -self.d())
  }
}

#[cfg(test)]
#[path = "plane.test.rs"]
mod test;
