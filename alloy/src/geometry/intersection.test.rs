use crate::geometry::{Intersects, Point3, Sphere, AABB};

#[test]
fn aabb_intersects_sphere() {
  struct TestCase {
    lhs: AABB,
    rhs: Sphere,
    expected: bool,
  }

  let test_cases = [
    TestCase {
      // Case: lhs does intersect rhs
      lhs: AABB::from_points(&Point3::new(0.0, 0.0, 0.0), &Point3::new(1.0, 1.0, 1.0)),
      rhs: Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0),
      expected: true,
    },
    TestCase {
      // Case: lhs touches wall of rhs
      lhs: AABB::from_points(&Point3::new(0.0, 0.0, 0.0), &Point3::new(1.0, 1.0, 1.0)),
      rhs: Sphere::new(Point3::new(0.0, 0.0, 2.0), 1.0),
      expected: true,
    },
    TestCase {
      // Case: lhs does not intersect rhs
      lhs: AABB::from_points(&Point3::new(0.0, 0.0, 0.0), &Point3::new(1.0, 1.0, 1.0)),
      rhs: Sphere::new(Point3::new(0.0, 0.0, 3.0), 1.0),
      expected: false,
    },
  ];

  for test in test_cases {
    assert_eq!(test.lhs.intersects(&test.rhs), test.expected);
  }
}
