use crate::geometry::{Intersects, Sphere, AABB};

impl Intersects<Sphere> for AABB {
  fn intersects(&self, other: &Sphere) -> bool {
    let closest_point = self.nearest_point(other.center());
    let distance = other.center().distance_to(&closest_point);
    distance <= other.radius()
  }
}

impl Intersects<AABB> for Sphere {
  #[inline(always)]
  fn intersects(&self, other: &AABB) -> bool {
    <AABB as Intersects<Sphere>>::intersects(other, self)
  }
}

#[cfg(test)]
#[path = "intersection.test.rs"]
mod test;
