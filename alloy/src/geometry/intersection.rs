use crate::geometry::{Intersects, Line, Sphere, AABB};
use crate::ops::Dot;

impl Intersects<Sphere> for AABB {
  fn intersects(&self, other: &Sphere) -> bool {
    let closest_point = self.nearest_point(other.center());
    let distance = other.center().distance_to(&closest_point);
    distance <= other.radius()
  }
}

impl Intersects<Sphere> for Line {
  fn intersects(&self, other: &Sphere) -> bool {
    let delta = self.origin() - other.center();
    let a = self.direction().dot(self.direction());
    let b = 2.0 * delta.dot(self.direction());
    let c = delta.dot(delta.as_vec3()) - other.radius() * other.radius();
    let discriminant = b * b - 4.0 * a * c;

    discriminant >= 0.0
  }
}

impl Intersects<Line> for Sphere {
  #[inline(always)]
  fn intersects(&self, other: &Line) -> bool {
    other.intersects(self)
  }
}

impl Intersects<Line> for AABB {
  #[inline(always)]
  fn intersects(&self, other: &Line) -> bool {
    other.intersects(self)
  }
}

impl Intersects<AABB> for Sphere {
  #[inline(always)]
  fn intersects(&self, other: &AABB) -> bool {
    <AABB as Intersects<Sphere>>::intersects(other, self)
  }
}

impl Intersects<AABB> for Line {
  fn intersects(&self, other: &AABB) -> bool {
    // Initialize t_min and t_max to represent the possible range of t where the line could intersect the AABB
    let mut t_min = f32::NEG_INFINITY;
    let mut t_max = f32::INFINITY;

    let line_origin = self.origin();
    let line_direction = self.direction();
    let aabb_min = other.min();
    let aabb_max = other.max();

    for i in 0..3 {
      let (origin, direction, min_bound, max_bound) = match i {
        0 => (
          line_origin.x(),
          line_direction.x(),
          aabb_min.x(),
          aabb_max.x(),
        ),
        1 => (
          line_origin.y(),
          line_direction.y(),
          aabb_min.y(),
          aabb_max.y(),
        ),
        2 => (
          line_origin.z(),
          line_direction.z(),
          aabb_min.z(),
          aabb_max.z(),
        ),
        // SAFETY: The loop is iterating over the range 0..3
        _ => unsafe { std::hint::unreachable_unchecked() },
      };

      // If the line is parallel to the slab (direction is zero)
      if direction.abs() < f32::EPSILON {
        if origin < min_bound || origin > max_bound {
          return false;
        }
      } else {
        let t1 = (min_bound - origin) / direction;
        let t2 = (max_bound - origin) / direction;

        let (t_near, t_far) = if t1 < t2 { (t1, t2) } else { (t2, t1) };

        t_min = t_min.max(t_near);
        t_max = t_max.min(t_far);

        if t_min > t_max {
          return false;
        }
      }
    }

    // If we reach this point, the line intersects the AABB
    true
  }
}

#[cfg(test)]
#[path = "intersection.test.rs"]
mod test;
