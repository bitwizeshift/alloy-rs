//! The line module defines a line that runs infinitely in both directions.

use crate::geometry::Point3;
use crate::math::vec::{Vec3, Vector3};
use crate::ops::{Cross, Dot};

use super::Intersects;

/// Representation of a line in 3D space that continues infinitely in both
/// directions.
pub struct Line {
  /// The origin of the line.
  origin: Point3,
  /// The direction of the line.
  direction: Vector3,
}

// Construction
impl Line {
  /// Create a new line from an origin and direction.
  ///
  /// The direction is assumed to be normalized
  ///
  /// # Safety
  ///
  /// The direction must be normalized, otherwise the behavior is undefined.
  ///
  /// # Parameters
  ///
  /// * `origin` - The origin of the line
  /// * `direction` - The direction of the line
  #[inline(always)]
  pub const unsafe fn new_unchecked(origin: Point3, direction: Vector3) -> Self {
    Self { origin, direction }
  }

  /// Create a new line from an origin and direction.
  ///
  /// The direction is normalized if it is not already.
  ///
  /// # Parameters
  ///
  /// * `origin` - The origin of the line
  /// * `direction` - The direction of the line
  #[inline(always)]
  pub fn new(origin: Point3, direction: &Vec3) -> Self {
    Self {
      origin,
      direction: direction.normalized(),
    }
  }
}

// Properties
impl Line {
  /// Get the origin of the line.
  #[inline(always)]
  pub const fn origin(&self) -> &Point3 {
    &self.origin
  }

  /// Get the direction of the line.
  #[inline(always)]
  pub const fn direction(&self) -> &Vec3 {
    self.direction.as_vec3()
  }

  /// Gets the point at the specified distance on the line
  ///
  /// A value of `0` for the distance will return the origin of the line.
  ///
  /// # Parameters
  ///
  /// * `distance` - The distance along the line to get the point at
  pub fn point_at_distance(&self, distance: f32) -> Point3 {
    Point3::from_vec3(&(self.origin.as_vec3() + self.direction.as_vec3() * distance))
  }

  /// Gets the distance between the line and a point
  ///
  /// # Parameters
  ///
  /// * `point` - The point to get the distance to
  pub fn distance_to(&self, point: &Point3) -> f32 {
    let origin_to_point = &self.origin - point;
    let projection = origin_to_point.dot(self.direction.as_vec3());

    origin_to_point.magnitude() - projection
  }

  /// Checks if the line contains a point
  ///
  /// # Parameters
  ///
  /// * `point` - The point to check
  pub fn contains_point(&self, point: &Point3) -> bool {
    let origin_to_point = point - &self.origin;

    // Compare ratios to check collinearity
    let t_x = if self.direction.x() != 0.0 {
      Some(origin_to_point.x() / self.direction.x)
    } else {
      None
    };
    let t_y = if self.direction.y() != 0.0 {
      Some(origin_to_point.y() / self.direction.y)
    } else {
      None
    };
    let t_z = if self.direction.z() != 0.0 {
      Some(origin_to_point.z() / self.direction.z)
    } else {
      None
    };

    t_x == t_y && t_y == t_z
  }
}

impl Intersects for Line {
  fn intersects(&self, other: &Self) -> bool {
    let cross = self.direction.as_vec3().cross(other.direction.as_vec3());

    // If the cross product is zero, the lines are parallel and can't intersect.
    if cross.square_magnitude() < 0.0001 {
      return false;
    }

    let delta = other.origin.as_vec3() - self.origin.as_vec3();
    let scalar_triple = delta.dot(cross.as_vec3());

    // If the scalar triple product is zero, or very near to zero, the lines are
    // coplanar and don't intersect.
    scalar_triple.abs() < 0.0001
  }
}

#[cfg(test)]
#[path = "line.test.rs"]
mod test;
