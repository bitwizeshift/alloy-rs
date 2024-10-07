use crate::core::ClampedF32;

#[test]
fn clampedf32_clamp() {
  struct TestCase {
    value: f32,
    expected: f32,
  }

  let test_cases = [
    TestCase {
      value: 0.0,
      expected: 0.0,
    },
    TestCase {
      value: 0.5,
      expected: 0.5,
    },
    TestCase {
      value: 1.0,
      expected: 1.0,
    },
    TestCase {
      value: -0.5,
      expected: 0.0,
    },
    TestCase {
      value: 1.5,
      expected: 1.0,
    },
  ];

  for test in test_cases {
    let sut = ClampedF32::clamp(test.value);

    assert_eq!(sut, test.expected);
  }
}

#[test]
fn clamped32_saturating_add() {
  struct TestCase {
    lhs: f32,
    rhs: f32,
    expected: f32,
  }

  let test_cases = [
    TestCase {
      lhs: 0.0,
      rhs: 0.0,
      expected: 0.0,
    },
    TestCase {
      lhs: 0.5,
      rhs: 0.5,
      expected: 1.0,
    },
    TestCase {
      lhs: 1.0,
      rhs: 0.5,
      expected: 1.0,
    },
    TestCase {
      lhs: 0.5,
      rhs: 1.0,
      expected: 1.0,
    },
    TestCase {
      lhs: 0.0,
      rhs: 1.0,
      expected: 1.0,
    },
    TestCase {
      lhs: 1.0,
      rhs: 1.0,
      expected: 1.0,
    },
    TestCase {
      lhs: 0.0,
      rhs: -1.0,
      expected: 0.0,
    },
  ];

  for test in test_cases {
    let sut = ClampedF32::clamp(test.lhs);

    let result = sut.saturating_add(ClampedF32::clamp(test.rhs));

    assert_eq!(result, test.expected);
  }
}

#[test]
fn clamped32_saturating_sub() {
  struct TestCase {
    lhs: f32,
    rhs: f32,
    expected: f32,
  }

  let test_cases = [
    TestCase {
      lhs: 0.0,
      rhs: 0.0,
      expected: 0.0,
    },
    TestCase {
      lhs: 0.5,
      rhs: 0.5,
      expected: 0.0,
    },
    TestCase {
      lhs: 1.0,
      rhs: 0.5,
      expected: 0.5,
    },
    TestCase {
      lhs: 0.5,
      rhs: 1.0,
      expected: 0.0,
    },
    TestCase {
      lhs: 0.0,
      rhs: 1.0,
      expected: 0.0,
    },
    TestCase {
      lhs: 1.0,
      rhs: 1.0,
      expected: 0.0,
    },
  ];

  for test in test_cases {
    let sut = ClampedF32::clamp(test.lhs);

    let result = sut.saturating_sub(ClampedF32::clamp(test.rhs));

    assert_eq!(result, test.expected);
  }
}

#[test]
fn clamped32_saturating_mul() {
  struct TestCase {
    lhs: f32,
    rhs: f32,
    expected: f32,
  }

  let test_cases = [
    TestCase {
      lhs: 0.0,
      rhs: 0.0,
      expected: 0.0,
    },
    TestCase {
      lhs: 0.5,
      rhs: 0.5,
      expected: 0.25,
    },
    TestCase {
      lhs: 1.0,
      rhs: 0.5,
      expected: 0.5,
    },
    TestCase {
      lhs: 0.5,
      rhs: 1.0,
      expected: 0.5,
    },
    TestCase {
      lhs: 0.0,
      rhs: 1.0,
      expected: 0.0,
    },
    TestCase {
      lhs: 1.0,
      rhs: 1.0,
      expected: 1.0,
    },
  ];

  for test in test_cases {
    let sut = ClampedF32::clamp(test.lhs);

    let result = sut.saturating_mul(ClampedF32::clamp(test.rhs));

    assert_eq!(result, test.expected);
  }
}

#[test]
fn clamped32_saturating_div() {
  struct TestCase {
    lhs: f32,
    rhs: f32,
    expected: f32,
  }

  let test_cases = [
    TestCase {
      lhs: 0.0,
      rhs: 0.1,
      expected: 0.0,
    },
    TestCase {
      lhs: 0.5,
      rhs: 0.5,
      expected: 1.0,
    },
    TestCase {
      lhs: 1.0,
      rhs: 0.5,
      expected: 1.0,
    },
    TestCase {
      lhs: 0.5,
      rhs: 1.0,
      expected: 0.5,
    },
    TestCase {
      lhs: 0.0,
      rhs: 1.0,
      expected: 0.0,
    },
    TestCase {
      lhs: 1.0,
      rhs: 1.0,
      expected: 1.0,
    },
  ];

  for test in test_cases {
    let sut = ClampedF32::clamp(test.lhs);

    let result = sut.saturating_div(ClampedF32::clamp(test.rhs));

    assert_eq!(
      result, test.expected,
      "{} / {} = {}",
      test.lhs, test.rhs, result
    );
  }
}
