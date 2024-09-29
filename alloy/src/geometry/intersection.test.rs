use crate::{
  geometry::{Intersects, Line, Point3, Sphere, AABB},
  math::vec::Vector3,
};

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

#[test]
fn sphere_intersects_line() {
  struct TestCase {
    lhs: Sphere,
    rhs: Line,
    expected: bool,
  }

  let test_cases = [
    TestCase {
      // Case: line touches edge of sphere
      lhs: Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0),
      rhs: Line::new(Point3::new(0.0, 0.0, 2.0), &Vector3::new(0.0, 0.0, -1.0)),
      expected: true,
    },
    TestCase {
      // Case: line passes through center of sphere
      lhs: Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0),
      rhs: Line::new(Point3::new(0.0, 0.0, 0.0), &Vector3::new(0.0, 0.0, -1.0)),
      expected: true,
    },
    TestCase {
      // Case: sphere does not intersect line
      lhs: Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0),
      rhs: Line::new(Point3::new(0.0, 0.0, 2.0), &Vector3::new(0.0, 1.0, 0.0)),
      expected: false,
    },
  ];

  for test in test_cases {
    assert_eq!(test.lhs.intersects(&test.rhs), test.expected);
  }
}

#[test]
fn aabb_intersects_line() {
  struct TestCase {
    lhs: AABB,
    rhs: Line,
    expected: bool,
  }

  let test_cases = [
    TestCase {
      // Case: line passes through center of AABB
      lhs: AABB::from_points(&Point3::new(0.0, 0.0, 0.0), &Point3::new(1.0, 1.0, 1.0)),
      rhs: Line::new(Point3::new(0.5, 0.5, 0.5), &Vector3::new(0.0, 0.0, -1.0)),
      expected: true,
    },
    TestCase {
      // Case: line touches edge of AABB
      lhs: AABB::from_points(&Point3::new(0.0, 0.0, 0.0), &Point3::new(1.0, 1.0, 1.0)),
      rhs: Line::new(Point3::new(0.0, 0.0, 1.0), &Vector3::new(0.0, 0.0, -1.0)),
      expected: true,
    },
    TestCase {
      // Case: line does not intersect AABB
      lhs: AABB::from_points(&Point3::new(0.0, 0.0, 0.0), &Point3::new(1.0, 1.0, 1.0)),
      rhs: Line::new(Point3::new(0.5, 0.5, 2.0), &Vector3::new(0.0, 1.0, 0.0)),
      expected: false,
    },
  ];

  for test in test_cases {
    assert_eq!(test.lhs.intersects(&test.rhs), test.expected);
  }
}
