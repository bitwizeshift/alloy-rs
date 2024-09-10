use crate::math::vec::Vector3;

use crate::geometry::{Plane, Point3};

/// Axis-aligned bounding box
pub struct AABB {
  min: Point3,
  max: Point3,
}

// Construction
impl AABB {
  /// Create a new axis-aligned bounding box
  pub fn from_points(p0: &Point3, p1: &Point3) -> Self {
    Self {
      min: p0.min(p1),
      max: p0.max(p1),
    }
  }

  /// Create a new axis-aligned bounding box
  pub const fn from_corners(top_left: &Point3, bottom_right: &Point3) -> Self {
    Self {
      min: *top_left,
      max: *bottom_right,
    }
  }
}

// Properties
impl AABB {
  /// Get the normal for the top of the box.
  pub const TOP_NORMAL: Vector3 = Vector3::UNIT_Y;

  /// Get the normal for the bottom of the box.
  pub const BOTTOM_NORMAL: Vector3 = Vector3::NEG_UNIT_Y;

  /// Get the normal for the front of the box.
  pub const FRONT_NORMAL: Vector3 = Vector3::UNIT_Z;

  /// Get the normal for the back of the box.
  pub const BACK_NORMAL: Vector3 = Vector3::NEG_UNIT_Z;

  /// Get the normal for the left of the box.
  pub const LEFT_NORMAL: Vector3 = Vector3::NEG_UNIT_X;

  /// Get the normal for the right of the box.
  pub const RIGHT_NORMAL: Vector3 = Vector3::UNIT_X;

  /// Get the top-left corner of the bounding box
  #[inline(always)]
  pub const fn top_plane(&self) -> Plane {
    Plane::from_normal_distance(Vector3::UNIT_Y.as_vec3(), self.max.as_vec3().y())
  }

  /// Get the bottom-right corner of the bounding box
  #[inline(always)]
  pub const fn bottom_plane(&self) -> Plane {
    Plane::from_normal_distance(Vector3::NEG_UNIT_Y.as_vec3(), self.min.as_vec3().y())
  }

  /// Get the front plane of the bounding box
  #[inline(always)]
  pub const fn front_plane(&self) -> Plane {
    Plane::from_normal_distance(Vector3::UNIT_Z.as_vec3(), self.min.as_vec3().z())
  }

  /// Get the back plane of the bounding box
  #[inline(always)]
  pub const fn back_plane(&self) -> Plane {
    Plane::from_normal_distance(Vector3::NEG_UNIT_Z.as_vec3(), self.max.as_vec3().z())
  }

  /// Get the left plane of the bounding box
  #[inline(always)]
  pub fn left_plane(&self) -> Plane {
    Plane::from_normal_distance(Vector3::NEG_UNIT_X.as_vec3(), self.min.as_vec3().x())
  }

  /// Get the right plane of the bounding box
  #[inline(always)]
  pub const fn right_plane(&self) -> Plane {
    Plane::from_normal_distance(Vector3::UNIT_X.as_vec3(), self.max.as_vec3().x())
  }

  /// Get a vector of the sized normal of the top-face of the bounding box.
  pub const fn top(&self) -> Vector3 {
    Vector3::new(0.0, self.max.as_vec3().y(), 0.0)
  }

  /// Get a vector of the sized normal of the bottom-face of the bounding box.
  pub const fn bottom(&self) -> Vector3 {
    Vector3::new(0.0, self.min.as_vec3().y(), 0.0)
  }

  /// Get a vector of the sized normal of the front-face of the bounding box.
  pub const fn front(&self) -> Vector3 {
    Vector3::new(0.0, 0.0, self.min.as_vec3().z())
  }

  /// Get a vector of the sized normal of the back-face of the bounding box.
  pub const fn back(&self) -> Vector3 {
    Vector3::new(0.0, 0.0, self.max.as_vec3().z())
  }

  /// Get a vector of the sized normal of the left-face of the bounding box.
  pub const fn left(&self) -> Vector3 {
    Vector3::new(self.min.as_vec3().x(), 0.0, 0.0)
  }

  /// Get a vector of the sized normal of the right-face of the bounding box.
  pub const fn right(&self) -> Vector3 {
    Vector3::new(self.max.as_vec3().x(), 0.0, 0.0)
  }

  /// Get the center of the bounding box
  pub fn center(&self) -> Point3 {
    const HALF: f32 = 0.5;
    Point3::new(
      (self.min.x() + self.max.x()) * HALF,
      (self.min.y() + self.max.y()) * HALF,
      (self.min.z() + self.max.z()) * HALF,
    )
  }

  /// Get the size of the bounding box
  pub fn size(&self) -> Vector3 {
    Vector3::from_points(&self.min, &self.max)
  }

  /// Get the volume of the bounding box
  pub fn volume(&self) -> f32 {
    let size = self.size();
    size.x() * size.y() * size.z()
  }

  /// Get the planes of the bounding box.
  ///
  /// No particular order is guaranteed, and this may change in the future.
  pub fn planes(&self) -> [Plane; 6] {
    [
      self.top_plane(),
      self.bottom_plane(),
      self.front_plane(),
      self.back_plane(),
      self.left_plane(),
      self.right_plane(),
    ]
  }

  /// Get the vectors of the bounding box.
  ///
  /// No particular order is guaranteed, and this may change in the future.
  pub fn vectors(&self) -> [Vector3; 6] {
    [
      self.top(),
      self.bottom(),
      self.front(),
      self.back(),
      self.left(),
      self.right(),
    ]
  }
}

// Intersection
impl AABB {
  /// Check if the bounding box contains a point
  pub fn contains_point(&self, point: &Point3) -> bool {
    self.min.x() <= point.x()
      && self.min.y() <= point.y()
      && self.min.z() <= point.z()
      && self.max.x() >= point.x()
      && self.max.y() >= point.y()
      && self.max.z() >= point.z()
  }

  /// Check if the bounding box intersects with another bounding box
  ///
  /// # Arguments
  ///
  /// * `other` - The other bounding box to check against
  pub fn intersects(&self, other: &Self) -> bool {
    self.min.x() <= other.max.x()
      && self.min.y() <= other.max.y()
      && self.min.z() <= other.max.z()
      && self.max.x() >= other.min.x()
      && self.max.y() >= other.min.y()
      && self.max.z() >= other.min.z()
  }

  /// Check if the bounding box encloses another bounding box
  ///
  /// # Arguments
  ///
  /// * `other` - The other bounding box to check against
  pub fn encloses(&self, other: &Self) -> bool {
    self.min.x() <= other.min.x()
      && self.min.y() <= other.min.y()
      && self.min.z() <= other.min.z()
      && self.max.x() >= other.max.x()
      && self.max.y() >= other.max.y()
      && self.max.z() >= other.max.z()
  }
}

// Queries
impl AABB {
  /// Get the nearest point on the bounding box to a point
  ///
  /// # Arguments
  ///
  /// * `point` - The point to find the nearest point to
  pub fn nearest_point(&self, point: &Point3) -> Point3 {
    Point3::new(
      point.x().max(self.min.x()).min(self.max.x()),
      point.y().max(self.min.y()).min(self.max.y()),
      point.z().max(self.min.z()).min(self.max.z()),
    )
  }

  /// Get the distance to the nearest point on the bounding box to a point
  ///
  /// # Arguments
  ///
  /// * `point` - The point to find the nearest point to
  pub fn distance_to(&self, point: &Point3) -> f32 {
    Vector3::from_points(&self.nearest_point(point), point).magnitude()
  }

  /// Get the square distance to the nearest point on the bounding box to a
  /// point
  ///
  /// # Arguments
  ///
  /// * `point` - The point to find the nearest point to
  pub fn square_distance_to(&self, point: &Point3) -> f32 {
    Vector3::from_points(&self.nearest_point(point), point).square_magnitude()
  }
}

#[cfg(test)]
#[path = "aabb.test.rs"]
mod test;
