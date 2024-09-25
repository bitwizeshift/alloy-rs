use std::ops::Neg;

use super::*;

#[test]
fn f32_abs() {
  struct TestCase {
    input: f32,
  }

  let test_cases = [
    TestCase { input: 1.0 },
    TestCase { input: -1.0 },
    TestCase { input: 0.0 },
    TestCase { input: -0.0 },
    TestCase {
      input: f32::INFINITY,
    },
    TestCase {
      input: f32::NEG_INFINITY,
    },
    TestCase { input: -42.371 },
  ];

  for test in test_cases {
    let expected = test.input.abs();
    let result = f32::abs(test.input);

    assert_eq!(result, expected);
  }
}

#[test]
fn f32_neg() {
  struct TestCase {
    input: f32,
  }

  let test_cases = [
    TestCase { input: 1.0 },
    TestCase { input: -1.0 },
    TestCase { input: 0.0 },
    TestCase { input: -0.0 },
    TestCase {
      input: f32::INFINITY,
    },
    TestCase {
      input: f32::NEG_INFINITY,
    },
    TestCase { input: -42.371 },
  ];

  for test in test_cases {
    let expected = test.input.neg();
    let result = f32::neg(test.input);

    assert_eq!(result, expected);
  }
}

#[test]
fn eq() {
  struct TestCase {
    lhs: f32,
    rhs: f32,
  }

  let test_cases = [
    TestCase { lhs: 1.0, rhs: 1.0 },
    TestCase {
      lhs: 1.0,
      rhs: -1.0,
    },
    TestCase { lhs: 0.0, rhs: 0.0 },
    TestCase {
      lhs: 0.0,
      rhs: -0.0,
    },
    TestCase {
      lhs: f32::INFINITY,
      rhs: f32::INFINITY,
    },
    TestCase {
      lhs: f32::INFINITY,
      rhs: f32::NEG_INFINITY,
    },
    TestCase {
      lhs: f32::NAN,
      rhs: f32::NAN,
    },
    TestCase {
      lhs: 42.371,
      rhs: 42.371,
    },
  ];

  for test in test_cases {
    let expected = test.lhs == test.rhs;
    let result = f32::eq(test.lhs, test.rhs);

    assert_eq!(result, expected, "{} == {}", test.lhs, test.rhs);
  }
}

#[test]
fn lt() {
  struct TestCase {
    lhs: f32,
    rhs: f32,
  }

  let test_cases = [
    TestCase { lhs: 1.0, rhs: 1.0 },
    TestCase {
      lhs: -1.0,
      rhs: 1.0,
    },
    TestCase {
      lhs: 1.0,
      rhs: -1.0,
    },
    TestCase {
      lhs: 0.0,
      rhs: -0.0,
    },
    TestCase {
      lhs: f32::INFINITY,
      rhs: f32::INFINITY,
    },
    TestCase {
      lhs: f32::INFINITY,
      rhs: f32::NEG_INFINITY,
    },
    TestCase {
      lhs: f32::NEG_INFINITY,
      rhs: f32::INFINITY,
    },
    TestCase {
      lhs: f32::NAN,
      rhs: f32::NAN,
    },
    TestCase {
      lhs: f32::NAN,
      rhs: 42.360,
    },
    TestCase {
      lhs: 42.360,
      rhs: f32::NAN,
    },
    TestCase {
      lhs: 42.371,
      rhs: 42.371,
    },
  ];

  for test in test_cases {
    let expected = test.lhs < test.rhs;
    let result = f32::lt(test.lhs, test.rhs);

    assert_eq!(result, expected, "{} < {}", test.lhs, test.rhs);
  }
}
