use super::*;

#[test]
fn mat4_from_array() {
  const INPUT: [[f32; 4]; 4] = [
    [1.0, 2.0, 3.0, 4.0],
    [5.0, 6.0, 7.0, 8.0],
    [9.0, 10.0, 11.0, 12.0],
    [13.0, 14.0, 15.0, 16.0],
  ];

  const SUT: &Mat4 = Mat4::from_arrays(&INPUT);

  for row in 0..4 {
    for col in 0..4 {
      assert_eq!(SUT[(row, col)], INPUT[row][col]);
    }
  }
}

#[test]
fn mat4_from_array_slice() {
  const INPUT: &[[f32; 4]] = &[
    [1.0, 2.0, 3.0, 4.0],
    [5.0, 6.0, 7.0, 8.0],
    [9.0, 10.0, 11.0, 12.0],
    [13.0, 14.0, 15.0, 16.0],
  ];

  // SAFETY: the above slice has exactly 4 elements.
  const SUT: &Mat4 = unsafe { Mat4::from_array_slice_unchecked(&INPUT) };

  for row in 0..4 {
    for col in 0..4 {
      assert_eq!(SUT[(row, col)], INPUT[row][col]);
    }
  }
}

#[test]
fn mat4_from_slice_unchecked() {
  const INPUT: &[f32] = &[
    1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
  ];

  // SAFETY: the above slice has exactly 16 elements.
  const SUT: &Mat4 = unsafe { Mat4::from_slice_unchecked(INPUT) };

  for row in 0..4 {
    for col in 0..4 {
      assert_eq!(SUT[(row, col)], INPUT[row * 4 + col]);
    }
  }
}

#[test]
fn mat4_as_array_slice() {
  const BEFORE: &[[f32; 4]] = &[
    [1.0, 2.0, 3.0, 4.0],
    [5.0, 6.0, 7.0, 8.0],
    [9.0, 10.0, 11.0, 12.0],
    [13.0, 14.0, 15.0, 16.0],
  ];

  // SAFETY: the above slice has exactly 4 elements.
  const AFTER: &[[f32; 4]] = unsafe { Mat4::from_array_slice_unchecked(&BEFORE) }.as_array_slice();

  assert_eq!(BEFORE, AFTER);
  assert_eq!(BEFORE.as_ptr(), AFTER.as_ptr());
  assert_eq!(BEFORE.len(), AFTER.len());
}

#[test]
fn mat4_combine_matrix() {
  struct TestCase {
    lhs: Matrix4,
    rhs: Matrix4,
    expected: Matrix4,
  }

  let cases = [
    TestCase {
      lhs: Matrix4::IDENTITY,
      rhs: Matrix4::IDENTITY,
      expected: Matrix4::IDENTITY,
    },
    TestCase {
      lhs: Matrix4::IDENTITY,
      rhs: Matrix4::ZERO,
      expected: Matrix4::ZERO,
    },
    TestCase {
      lhs: Matrix4::ZERO,
      rhs: Matrix4::IDENTITY,
      expected: Matrix4::ZERO,
    },
    TestCase {
      lhs: Matrix4::ZERO,
      rhs: Matrix4::ZERO,
      expected: Matrix4::ZERO,
    },
    TestCase {
      lhs: Matrix4::IDENTITY,
      rhs: Matrix4::from_arrays([
        [2.0, 0.0, 0.0, 0.0],
        [0.0, 3.0, 0.0, 0.0],
        [0.0, 0.0, 4.0, 0.0],
        [0.0, 0.0, 0.0, 5.0],
      ]),
      expected: Matrix4::from_arrays([
        [2.0, 0.0, 0.0, 0.0],
        [0.0, 3.0, 0.0, 0.0],
        [0.0, 0.0, 4.0, 0.0],
        [0.0, 0.0, 0.0, 5.0],
      ]),
    },
    TestCase {
      lhs: Matrix4::from_arrays([
        [1.0, 2.0, 3.0, 4.0],
        [7.0, 6.0, 5.0, 4.0],
        [1.0, 2.0, 3.0, 4.0],
        [7.0, 6.0, 5.0, 4.0],
      ]),
      rhs: Matrix4::from_arrays([
        [4.0, 5.0, 6.0, 7.0],
        [4.0, 3.0, 2.0, 1.0],
        [4.0, 5.0, 6.0, 7.0],
        [4.0, 3.0, 2.0, 1.0],
      ]),
      expected: Matrix4::from_arrays([
        [40.0, 38.0, 36.0, 34.0],
        [88.0, 90.0, 92.0, 94.0],
        [40.0, 38.0, 36.0, 34.0],
        [88.0, 90.0, 92.0, 94.0],
      ]),
    },
  ];

  for case in cases.iter() {
    let result = case.lhs.mul_mat4(&case.rhs);

    assert_eq!(result, case.expected);
  }
}

#[test]
fn mat4_mul_col_vec() {
  struct TestCase {
    lhs: Matrix4,
    rhs: Vector4,
    expected: Vector4,
  }

  let cases = [
    TestCase {
      lhs: Matrix4::IDENTITY,
      rhs: Vector4::ZERO,
      expected: Vector4::ZERO,
    },
    TestCase {
      lhs: Matrix4::from_arrays([
        [1.0, 2.0, 3.0, 4.0],
        [7.0, 6.0, 5.0, 4.0],
        [1.0, 2.0, 3.0, 4.0],
        [7.0, 6.0, 5.0, 4.0],
      ]),
      rhs: Vector4::new(1.0, 2.0, 3.0, 4.0),
      expected: Vector4::new(30.0, 50.0, 30.0, 50.0),
    },
  ];

  for case in cases.iter() {
    let result = case.lhs.mul_col_vec4(&case.rhs);

    assert_eq!(result, case.expected);
  }
}

#[test]
fn mat4_mul_row_vec() {
  struct TestCase {
    lhs: Matrix4,
    rhs: Vector4,
    expected: Vector4,
  }

  let cases = [
    TestCase {
      lhs: Matrix4::IDENTITY,
      rhs: Vector4::ZERO,
      expected: Vector4::ZERO,
    },
    TestCase {
      lhs: Matrix4::from_arrays([
        [1.0, 2.0, 3.0, 4.0],
        [7.0, 6.0, 5.0, 4.0],
        [1.0, 2.0, 3.0, 4.0],
        [7.0, 6.0, 5.0, 4.0],
      ]),
      rhs: Vector4::new(1.0, 2.0, 3.0, 4.0),
      expected: Vector4::new(46.0, 44.0, 42.0, 40.0),
    },
  ];

  for case in cases.iter() {
    let result = case.lhs.mul_row_vec4(&case.rhs);

    assert_eq!(result, case.expected);
  }
}

#[test]
fn mat4_determinant() {
  struct TestCase {
    input: Matrix4,
    expected: f32,
  }

  let cases = [
    TestCase {
      input: Matrix4::IDENTITY,
      expected: 1.0,
    },
    TestCase {
      input: Matrix4::ZERO,
      expected: 0.0,
    },
    TestCase {
      input: Matrix4::from_arrays([
        [1.0, 2.0, 3.0, 0.0],
        [3.0, 2.0, 1.0, 0.0],
        [2.0, 1.0, 3.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
      ]),
      expected: -12.0,
    },
    TestCase {
      input: Matrix4::from_arrays([
        [1.0, 2.0, 3.0, 4.0],
        [7.0, 6.0, 5.0, 4.0],
        [1.0, 2.0, 3.0, 4.0],
        [7.0, 6.0, 5.0, 4.0],
      ]),
      expected: 0.0,
    },
  ];

  for case in cases.iter() {
    assert_eq!(case.input.determinant(), case.expected);
  }
}

#[test]
fn mat4_transpose() {
  struct TestCase {
    input: Matrix4,
    expected: Matrix4,
  }

  let cases = [
    TestCase {
      input: Matrix4::IDENTITY,
      expected: Matrix4::IDENTITY,
    },
    TestCase {
      input: Matrix4::ZERO,
      expected: Matrix4::ZERO,
    },
    TestCase {
      input: Matrix4::from_arrays([
        [1.0, 2.0, 3.0, 0.0],
        [3.0, 2.0, 1.0, 0.0],
        [2.0, 1.0, 3.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
      ]),
      expected: Matrix4::from_arrays([
        [1.0, 3.0, 2.0, 0.0],
        [2.0, 2.0, 1.0, 0.0],
        [3.0, 1.0, 3.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
      ]),
    },
    TestCase {
      input: Matrix4::from_arrays([
        [1.0, 2.0, 3.0, 4.0],
        [7.0, 6.0, 5.0, 4.0],
        [1.0, 2.0, 3.0, 4.0],
        [7.0, 6.0, 5.0, 4.0],
      ]),
      expected: Matrix4::from_arrays([
        [1.0, 7.0, 1.0, 7.0],
        [2.0, 6.0, 2.0, 6.0],
        [3.0, 5.0, 3.0, 5.0],
        [4.0, 4.0, 4.0, 4.0],
      ]),
    },
  ];

  for case in cases.iter() {
    let mut result = case.input.clone();

    result.transpose();

    assert_eq!(result, case.expected);
  }
}

#[test]
fn mat4_invert() {
  struct TestCase {
    input: Matrix4,
    expected: Matrix4,
  }

  let cases = [
    TestCase {
      input: Matrix4::IDENTITY,
      expected: Matrix4::IDENTITY,
    },
    TestCase {
      input: Matrix4::ZERO,
      expected: Matrix4::IDENTITY,
    },
    TestCase {
      input: Matrix4::from_arrays([
        [1.0, 2.0, 0.0, 1.0],
        [0.0, 2.0, 0.0, 0.0],
        [1.0, 0.0, 2.0, 0.0],
        [0.0, 2.0, 0.0, 1.0],
      ]),
      expected: Matrix4::from_arrays([
        [1.0, 0.0, 0.0, -1.0],
        [0.0, 0.5, 0.0, 0.0],
        [-0.5, 0.0, 0.5, 0.5],
        [0.0, -1.0, 0.0, 1.0],
      ]),
    },
    TestCase {
      input: Matrix4::from_arrays([
        [1.0, 2.0, 3.0, 4.0],
        [7.0, 6.0, 5.0, 4.0],
        [1.0, 2.0, 3.0, 4.0],
        [7.0, 6.0, 5.0, 4.0],
      ]),
      expected: Matrix4::IDENTITY,
    },
  ];

  for case in cases.iter() {
    let mut result = case.input.clone();

    result.invert();

    assert_eq!(result, case.expected);
  }
}

#[test]
fn mat4_trace() {
  struct TestCase {
    input: Matrix4,
    expected: f32,
  }

  let cases = [
    TestCase {
      input: Matrix4::IDENTITY,
      expected: 4.0,
    },
    TestCase {
      input: Matrix4::ZERO,
      expected: 0.0,
    },
    TestCase {
      input: Matrix4::from_arrays([
        [1.0, 2.0, 3.0, 0.0],
        [3.0, 2.0, 1.0, 0.0],
        [2.0, 1.0, 3.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
      ]),
      expected: 7.0,
    },
    TestCase {
      input: Matrix4::from_arrays([
        [1.0, 2.0, 3.0, 4.0],
        [7.0, 6.0, 5.0, 4.0],
        [1.0, 2.0, 3.0, 4.0],
        [7.0, 6.0, 5.0, 4.0],
      ]),
      expected: 14.0,
    },
  ];

  for case in cases.iter() {
    assert_eq!(case.input.trace(), case.expected);
  }
}

#[test]
fn mat4_col() {
  let matrix = Matrix4::from_arrays([
    [1.0, 2.0, 3.0, 4.0],
    [7.0, 6.0, 5.0, 4.0],
    [1.0, 2.0, 3.0, 4.0],
    [7.0, 6.0, 5.0, 4.0],
  ]);

  struct TestCase {
    index: usize,
    expected: Vector4,
  }

  let cases = [
    TestCase {
      index: 0,
      expected: Vector4::new(1.0, 7.0, 1.0, 7.0),
    },
    TestCase {
      index: 1,
      expected: Vector4::new(2.0, 6.0, 2.0, 6.0),
    },
    TestCase {
      index: 2,
      expected: Vector4::new(3.0, 5.0, 3.0, 5.0),
    },
    TestCase {
      index: 3,
      expected: Vector4::new(4.0, 4.0, 4.0, 4.0),
    },
  ];

  for case in cases.iter() {
    let col = matrix.col(case.index);

    assert_eq!(col.to_vector4().as_vec4(), case.expected.as_vec4());
  }
}
