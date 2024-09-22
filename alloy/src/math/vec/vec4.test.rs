use super::*;

#[test]
fn square_magnitude() {
  let vec = Vector4::new(4.0, 2.0, 0.0, 0.0);

  let magnitude = vec.square_magnitude();

  assert!(magnitude.almost_eq(&20.0))
}

#[test]
fn identity() {
  let vec = Vector4::new(4.0, 2.0, 0.0, 0.0);

  assert_eq!(vec.as_ptr(), &vec[0]);
  assert_eq!(vec.as_ptr(), &vec.x);
}

#[test]
fn iter() {
  let mut vec = Vector4::new(4.0, 2.0, 1.0, 0.0);

  for v in vec.iter_mut() {
    *v = *v * 2.0
  }

  assert!(vec.x.almost_eq(&8.0), "x = {}", vec.x);
  assert!(vec.y.almost_eq(&4.0), "y = {}", vec.y);
  assert!(vec.z.almost_eq(&2.0), "z = {}", vec.z);
  assert!(vec.w.almost_eq(&0.0), "w = {}", vec.w);
}

#[test]
fn lerp() {
  struct TestCase {
    lhs: Vector4,
    rhs: Vector4,
    alpha: f32,
    expected: Vector4,
  }

  let test_cases = [
    TestCase {
      lhs: Vector4::new(0.0, 0.0, 0.0, 0.0),
      rhs: Vector4::new(1.0, 1.0, 1.0, 1.0),
      alpha: 0.5,
      expected: Vector4::new(0.5, 0.5, 0.5, 0.5),
    },
    TestCase {
      lhs: Vector4::new(0.0, 0.0, 0.0, 0.0),
      rhs: Vector4::new(4.0, 6.0, 8.0, 10.0),
      alpha: 0.0,
      expected: Vector4::new(0.0, 0.0, 0.0, 0.0),
    },
    TestCase {
      lhs: Vector4::new(0.0, 0.0, 0.0, 0.0),
      rhs: Vector4::new(4.0, 6.0, 8.0, 10.0),
      alpha: 1.0,
      expected: Vector4::new(4.0, 6.0, 8.0, 10.0),
    },
    TestCase {
      lhs: Vector4::new(0.0, 0.0, 0.0, 0.0),
      rhs: Vector4::new(4.0, 6.0, 8.0, 10.0),
      alpha: 0.25,
      expected: Vector4::new(1.0, 1.5, 2.0, 2.5),
    },
  ];

  for test in test_cases {
    let result = test.lhs.lerp(&test.rhs, test.alpha);

    assert!(result.almost_eq(&test.expected), "result = {}", result);
  }
}
