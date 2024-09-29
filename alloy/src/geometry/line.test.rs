use crate::{
  geometry::{Intersects, Line, Point3},
  math::vec::Vector3,
};

#[test]
fn line_intersects() {
  struct TestCase {
    lhs: Line,
    rhs: Line,
    expected: bool,
  }

  let test_cases = [
    TestCase {
      // Case: lines intersect at origin
      lhs: Line::new(Point3::new(0.0, 0.0, 0.0), &Vector3::new(1.0, 0.0, 0.0)),
      rhs: Line::new(Point3::new(0.0, 0.0, 0.0), &Vector3::new(0.0, 1.0, 0.0)),
      expected: true,
    },
    TestCase {
      // Case: lines are parallel
      lhs: Line::new(Point3::new(0.0, 0.0, 0.0), &Vector3::new(1.0, 0.0, 0.0)),
      rhs: Line::new(Point3::new(0.0, 0.0, 1.0), &Vector3::new(1.0, 0.0, 0.0)),
      expected: false,
    },
    TestCase {
      // Case: lines do not intersect
      lhs: Line::new(Point3::new(0.0, 0.0, 0.0), &Vector3::new(1.0, 0.0, 0.0)),
      rhs: Line::new(Point3::new(1.0, 1.0, 1.0), &Vector3::new(0.0, 1.0, 0.0)),
      expected: false,
    },
  ];

  for test in test_cases {
    let result = test.lhs.intersects(&test.rhs);

    assert_eq!(result, test.expected);
  }
}
