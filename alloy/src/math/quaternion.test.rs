use crate::cmp::Near;
use crate::math::Degree;

use super::*;

const EPSILON: f32 = 0.001;

#[test]
fn quaternion_from_angle_axis() {
  struct TestCase {
    angle: Radian,
    axis: Vector3,
    expected: Quaternion,
  }

  let test_cases = [TestCase {
    angle: Radian::from(Degree::new(90.0)),
    axis: Vector3::new(1.0, 0.0, 0.0),
    expected: Quaternion::new(0.70710677, 0.70710677, 0.0, 0.0),
  }];

  for case in test_cases.iter() {
    let result = Quaternion::from_angle_axis(case.angle, &case.axis);

    assert_eq!(result, case.expected);
  }
}

#[test]
fn quaternion_mul_quaternion() {
  struct TestCase {
    lhs: Quaternion,
    rhs: Quaternion,
    expected: Quaternion,
  }

  let test_cases = [
    //
    TestCase {
      lhs: Quaternion::new(1.0, 2.0, 3.0, 4.0),
      rhs: Quaternion::new(4.0, 3.0, 2.0, 1.0),
      expected: Quaternion::new(-12.0, 6.0, 24.0, 12.0),
    },
  ];

  for case in test_cases.iter() {
    let result = case.lhs * case.rhs;

    assert_eq!(result, case.expected);
  }
}

#[test]
fn quaternion_angle_axis() {
  struct TestCase {
    angle: Radian,
    axis: Vector3,
  }

  let test_cases = [
    TestCase {
      angle: Radian::from(Degree::new(90.0)),
      axis: Vector3::new(1.0, 0.0, 0.0),
    },
    TestCase {
      angle: Radian::ONE_REVOLUTION / 2.0,
      axis: Vector3::new(0.0, 1.0, 0.0),
    },
  ];

  for case in test_cases.iter() {
    let quaternion = Quaternion::from_angle_axis(case.angle, &case.axis);

    let (result_angle, result_axis) = quaternion.angle_axis::<Radian>();

    assert_eq!(result_angle, case.angle);
    assert_eq!(result_axis, case.axis);
  }
}

#[test]
fn quaternion_euler_angles() {
  struct TestCase {
    yaw: Radian,
    pitch: Radian,
    roll: Radian,
  }

  let test_cases = [
    TestCase {
      yaw: Radian::ONE_REVOLUTION / 8.0,
      pitch: Radian::new(0.0),
      roll: Radian::new(0.0),
    },
    TestCase {
      yaw: Radian::new(0.0),
      pitch: Radian::ONE_REVOLUTION / 8.0,
      roll: Radian::new(0.0),
    },
    TestCase {
      yaw: Radian::new(0.0),
      pitch: Radian::new(0.0),
      roll: Radian::ONE_REVOLUTION / 8.0,
    },
    TestCase {
      yaw: Radian::ONE_REVOLUTION / 4.0,
      pitch: Radian::new(0.0),
      roll: Radian::new(0.0),
    },
    TestCase {
      yaw: Radian::new(0.0),
      pitch: Radian::ONE_REVOLUTION / 4.0,
      roll: Radian::new(0.0),
    },
    TestCase {
      yaw: Radian::new(0.0),
      pitch: Radian::new(0.0),
      roll: Radian::ONE_REVOLUTION / 4.0,
    },
    TestCase {
      yaw: Radian::ONE_REVOLUTION / 8.0,
      pitch: Radian::ONE_REVOLUTION / 16.0,
      roll: Radian::new(0.0),
    },
    TestCase {
      yaw: Radian::ONE_REVOLUTION / 8.0,
      pitch: Radian::new(0.0),
      roll: Radian::ONE_REVOLUTION / 16.0,
    },
    TestCase {
      yaw: Radian::new(0.0),
      pitch: Radian::ONE_REVOLUTION / 8.0,
      roll: Radian::ONE_REVOLUTION / 16.0,
    },
  ];

  for case in test_cases.iter() {
    let quaternion = Quaternion::from_euler_angles(case.yaw, case.pitch, case.roll);

    let (result_yaw, result_pitch, result_roll) = quaternion.euler_angles::<Radian>();

    assert!(
      result_yaw.near(&case.yaw, &EPSILON),
      "Yaw {} = {}",
      result_yaw,
      case.yaw
    );
    assert!(
      result_pitch.near(&case.pitch, &EPSILON),
      "Pitch {} = {}",
      result_pitch,
      case.pitch
    );
    assert!(
      result_roll.near(&case.roll, &EPSILON),
      "Roll {} = {}",
      result_roll,
      case.roll
    );
  }
}

#[test]
fn quaternion_rotate_vec3() {
  struct TestCase {
    yaw: Radian,
    pitch: Radian,
    roll: Radian,
    vector: Vector3,
    expected: Vector3,
  }

  let test_cases = [
    TestCase {
      yaw: Radian::ONE_REVOLUTION / 4.0,
      pitch: Radian::new(0.0),
      roll: Radian::new(0.0),
      vector: Vector3::new(1.0, 0.0, 0.0),
      expected: Vector3::new(0.0, 1.0, 0.0),
    },
    TestCase {
      yaw: Radian::ONE_REVOLUTION / -4.0,
      pitch: Radian::new(0.0),
      roll: Radian::new(0.0),
      vector: Vector3::new(1.0, 0.0, 0.0),
      expected: Vector3::new(0.0, -1.0, 0.0),
    },
    TestCase {
      yaw: Radian::new(0.0),
      pitch: Radian::ONE_REVOLUTION / 4.0,
      roll: Radian::new(0.0),
      vector: Vector3::new(1.0, 0.0, 0.0),
      expected: Vector3::new(0.0, 0.0, -1.0),
    },
    TestCase {
      yaw: Radian::new(0.0),
      pitch: Radian::ONE_REVOLUTION / -4.0,
      roll: Radian::new(0.0),
      vector: Vector3::new(1.0, 0.0, 0.0),
      expected: Vector3::new(0.0, 0.0, 1.0),
    },
    TestCase {
      yaw: Radian::new(0.0),
      pitch: Radian::new(0.0),
      roll: Radian::ONE_REVOLUTION / 4.0,
      vector: Vector3::new(1.0, 0.0, 0.0),
      expected: Vector3::new(1.0, 0.0, 0.0),
    },
    TestCase {
      yaw: Radian::new(0.0),
      pitch: Radian::new(0.0),
      roll: Radian::ONE_REVOLUTION / -4.0,
      vector: Vector3::new(1.0, 0.0, 0.0),
      expected: Vector3::new(1.0, 0.0, 0.0),
    },
  ];

  for case in test_cases.iter() {
    let quaternion = Quaternion::from_euler_angles(case.yaw, case.pitch, case.roll);

    let result = quaternion.rotate_vec3(&case.vector);

    assert!(
      result.near(&case.expected, &EPSILON),
      "{} = {}",
      result,
      case.expected
    );
  }
}

#[test]
fn quaternion_rotate_vec4() {
  struct TestCase {
    yaw: Radian,
    pitch: Radian,
    roll: Radian,
    vector: Vector4,
    expected: Vector4,
  }

  let test_cases = [
    TestCase {
      yaw: Radian::ONE_REVOLUTION / 4.0,
      pitch: Radian::new(0.0),
      roll: Radian::new(0.0),
      vector: Vector4::new(1.0, 0.0, 0.0, 1.0),
      expected: Vector4::new(0.0, 1.0, 0.0, 1.0),
    },
    TestCase {
      yaw: Radian::ONE_REVOLUTION / -4.0,
      pitch: Radian::new(0.0),
      roll: Radian::new(0.0),
      vector: Vector4::new(1.0, 0.0, 0.0, 1.0),
      expected: Vector4::new(0.0, -1.0, 0.0, 1.0),
    },
    TestCase {
      yaw: Radian::new(0.0),
      pitch: Radian::ONE_REVOLUTION / 4.0,
      roll: Radian::new(0.0),
      vector: Vector4::new(1.0, 0.0, 0.0, 1.0),
      expected: Vector4::new(0.0, 0.0, -1.0, 1.0),
    },
    TestCase {
      yaw: Radian::new(0.0),
      pitch: Radian::ONE_REVOLUTION / -4.0,
      roll: Radian::new(0.0),
      vector: Vector4::new(1.0, 0.0, 0.0, 1.0),
      expected: Vector4::new(0.0, 0.0, 1.0, 1.0),
    },
    TestCase {
      yaw: Radian::new(0.0),
      pitch: Radian::new(0.0),
      roll: Radian::ONE_REVOLUTION / 4.0,
      vector: Vector4::new(1.0, 0.0, 0.0, 1.0),
      expected: Vector4::new(1.0, 0.0, 0.0, 1.0),
    },
    TestCase {
      yaw: Radian::new(0.0),
      pitch: Radian::new(0.0),
      roll: Radian::ONE_REVOLUTION / -4.0,
      vector: Vector4::new(1.0, 0.0, 0.0, 1.0),
      expected: Vector4::new(1.0, 0.0, 0.0, 1.0),
    },
  ];

  for case in test_cases.iter() {
    let quaternion = Quaternion::from_euler_angles(case.yaw, case.pitch, case.roll);

    let result = quaternion.rotate_vec4(&case.vector);

    assert!(
      result.near(&case.expected, &EPSILON),
      "{} = {}",
      result,
      case.expected
    );
  }
}
