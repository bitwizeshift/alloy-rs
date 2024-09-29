use super::{Encloses, Intersects, Point3};

/// A sphere in 3D space.
pub struct Sphere {
  center: Point3,
  radius: f32,
}

// Construction
impl Sphere {
  /// Create a new sphere
  ///
  /// # Parameters
  ///
  /// * `center` - The center of the sphere
  /// * `radius` - The radius of the sphere
  #[inline(always)]
  #[must_use]
  pub const fn new(center: Point3, radius: f32) -> Self {
    Self { center, radius }
  }
}

// Properties
impl Sphere {
  /// Get the center of the sphere
  #[inline(always)]
  #[must_use]
  pub const fn center(&self) -> &Point3 {
    &self.center
  }

  /// Get the radius of the sphere
  #[inline(always)]
  #[must_use]
  pub const fn radius(&self) -> f32 {
    self.radius
  }
}

// Queries
impl Sphere {
  /// Get the distance to the nearest point on the sphere to a point
  ///
  /// # Parameters
  ///
  /// * `point` - The point to find the nearest point to
  #[inline(always)]
  pub fn distance_to(&self, point: &Point3) -> f32 {
    let distance = self.center.distance_to(point);
    distance - self.radius
  }
}

impl Intersects for Sphere {
  fn intersects(&self, other: &Self) -> bool {
    let delta_x = self.center.x() - other.center.x();
    let delta_y = self.center.y() - other.center.y();
    let delta_z = self.center.z() - other.center.z();
    let square_distance = delta_x * delta_x + delta_y * delta_y + delta_z * delta_z;

    let radii_sum = self.radius + other.radius;
    let square_radii_sum = radii_sum * radii_sum;

    square_distance <= square_radii_sum
  }
}

impl Encloses for Sphere {
  fn encloses(&self, other: &Self) -> bool {
    let delta_x = self.center.x() - other.center.x();
    let delta_y = self.center.y() - other.center.y();
    let delta_z = self.center.z() - other.center.z();
    let square_distance = delta_x * delta_x + delta_y * delta_y + delta_z * delta_z;

    let radii_diff = self.radius - other.radius;
    let square_radii_diff = radii_diff * radii_diff;

    square_distance <= square_radii_diff
  }
}

impl Encloses<Point3> for Sphere {
  fn encloses(&self, point: &Point3) -> bool {
    self.center.square_distance_to(point) <= (self.radius * self.radius)
  }
}

#[cfg(test)]
#[path = "sphere.test.rs"]
mod test;
