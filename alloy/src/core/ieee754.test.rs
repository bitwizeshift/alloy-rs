use super::*;

#[test]
fn f32_abs() {
  struct TestCase {
    input: f32,
    expected: f32,
  }

  let test_cases = [
    TestCase {
      input: 1.0,
      expected: 1.0,
    },
    TestCase {
      input: -1.0,
      expected: 1.0,
    },
    TestCase {
      input: 0.0,
      expected: 0.0,
    },
    TestCase {
      input: -0.0,
      expected: 0.0,
    },
    TestCase {
      input: f32::INFINITY,
      expected: f32::INFINITY,
    },
    TestCase {
      input: f32::NEG_INFINITY,
      expected: f32::INFINITY,
    },
    TestCase {
      input: -42.371,
      expected: 42.371,
    },
  ];

  for test in test_cases {
    // SAFETY: The test cases are all valid floating point numbers.
    let result = unsafe { f32::abs(test.input) };

    assert_eq!(result, test.expected);
  }
}

#[test]
fn f32_neg() {
  struct TestCase {
    input: f32,
    expected: f32,
  }

  let test_cases = [
    TestCase {
      input: 1.0,
      expected: -1.0,
    },
    TestCase {
      input: -1.0,
      expected: 1.0,
    },
    TestCase {
      input: 0.0,
      expected: -0.0,
    },
    TestCase {
      input: -0.0,
      expected: 0.0,
    },
    TestCase {
      input: f32::INFINITY,
      expected: f32::NEG_INFINITY,
    },
    TestCase {
      input: f32::NEG_INFINITY,
      expected: f32::INFINITY,
    },
    TestCase {
      input: -42.371,
      expected: 42.371,
    },
  ];

  for test in test_cases {
    // SAFETY: The test cases are all valid floating point numbers.
    let result = unsafe { f32::neg(test.input) };

    assert_eq!(result, test.expected);
  }
}
