use super::*;

#[test]
fn sphere_distance_to() {
  let sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0);
  let point = Point3::new(0.0, 0.0, 2.0);
  assert_eq!(sphere.distance_to(&point), 1.0);
}

#[test]
fn sphere_intersects() {
  struct TestCase {
    lhs: Sphere,
    rhs: Sphere,
    expected: bool,
  }

  let test_cases = [
    TestCase {
      lhs: Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0),
      rhs: Sphere::new(Point3::new(0.0, 0.0, 2.0), 1.0),
      expected: true,
    },
    TestCase {
      lhs: Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0),
      rhs: Sphere::new(Point3::new(0.0, 0.0, 2.0), 0.5),
      expected: false,
    },
  ];

  for test in test_cases {
    assert_eq!(test.lhs.intersects(&test.rhs), test.expected);
  }
}

#[test]
fn sphere_encloses_sphere() {
  struct TestCase {
    lhs: Sphere,
    rhs: Sphere,
    expected: bool,
  }

  let test_cases = [
    TestCase {
      // Case: lhs does not intersect rhs
      lhs: Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0),
      rhs: Sphere::new(Point3::new(0.0, 0.0, 2.0), 1.0),
      expected: false,
    },
    TestCase {
      // Case: lhs intersects rhs, but does not enclose
      lhs: Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0),
      rhs: Sphere::new(Point3::new(0.0, 0.0, 2.0), 1.0),
      expected: false,
    },
    TestCase {
      // Case: lhs encloses rhs
      lhs: Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0),
      rhs: Sphere::new(Point3::new(0.0, 0.0, 0.0), 0.5),
      expected: true,
    },
  ];

  for test in test_cases {
    assert_eq!(test.lhs.encloses(&test.rhs), test.expected);
  }
}

#[test]
fn sphere_encloses_point() {
  struct TestCase {
    lhs: Sphere,
    rhs: Point3,
    expected: bool,
  }

  let test_cases = [
    TestCase {
      // Case: point is outside of sphere
      lhs: Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0),
      rhs: Point3::new(0.0, 0.0, 2.0),
      expected: false,
    },
    TestCase {
      // Case: point is on the surface of the sphere
      lhs: Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0),
      rhs: Point3::new(0.0, 0.0, 1.0),
      expected: true,
    },
    TestCase {
      // Case: point is inside of the sphere
      lhs: Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0),
      rhs: Point3::new(0.0, 0.0, 0.5),
      expected: true,
    },
  ];

  for test in test_cases {
    assert_eq!(test.lhs.encloses(&test.rhs), test.expected);
  }
}
