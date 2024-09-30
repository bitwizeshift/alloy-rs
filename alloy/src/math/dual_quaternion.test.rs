use crate::cmp::AlmostEq;
use crate::math::vec::Vector3;
use crate::math::{Degree, DualQuaternion, Quaternion};

#[test]
fn from_translation() {
  let translation = DualQuaternion::from_translation(&Vector3::new(1.0, 2.0, 3.0));
  let expected = DualQuaternion::new(Quaternion::identity(), Quaternion::new(0.0, 0.5, 1.0, 1.5));

  assert_eq!(translation, expected);
}

#[test]
fn translation() {
  let translation = DualQuaternion::from_translation(&Vector3::new(1.0, 2.0, 3.0));
  let expected = Vector3::new(1.0, 2.0, 3.0);

  assert_eq!(translation.translation(), expected);
}

#[test]
fn translate() {
  struct TestCase {
    rotation: Quaternion,
    translation: Vector3,
  }

  let test_cases = [
    TestCase {
      rotation: Quaternion::identity(),
      translation: Vector3::new(1.0, 2.0, 3.0),
    },
    TestCase {
      rotation: Quaternion::from_euler_angles(
        Degree::new(90.0),
        Degree::new(0.0),
        Degree::new(0.0),
      ),
      translation: Vector3::new(0.0, 0.0, 1.0),
    },
    TestCase {
      rotation: Quaternion::from_euler_angles(
        Degree::new(0.0),
        Degree::new(90.0),
        Degree::new(0.0),
      ),
      translation: Vector3::new(0.0, 1.0, 0.0),
    },
    TestCase {
      rotation: Quaternion::from_euler_angles(
        Degree::new(0.0),
        Degree::new(0.0),
        Degree::new(90.0),
      ),
      translation: Vector3::new(1.0, 0.0, 0.0),
    },
  ];

  for test in test_cases {
    let mut dual_quaternion = DualQuaternion::from_rotation(test.rotation);

    dual_quaternion.translate(&test.translation);
    let translation = dual_quaternion.translation();
    let rotation = dual_quaternion.rotation();
    let _world = dual_quaternion.world_translation();

    assert!(
      translation.almost_eq(&test.translation),
      "{} ~= {}",
      translation,
      test.translation
    );
    assert!(
      rotation.almost_eq(&test.rotation),
      "{} ~= {}",
      rotation,
      test.rotation
    )
  }
}

#[test]
fn translate_world() {
  struct TestCase {
    rotation: Quaternion,
    translation: Vector3,
  }

  let test_cases = [
    TestCase {
      rotation: Quaternion::identity(),
      translation: Vector3::new(1.0, 2.0, 3.0),
    },
    TestCase {
      rotation: Quaternion::from_euler_angles(
        Degree::new(90.0),
        Degree::new(0.0),
        Degree::new(0.0),
      ),
      translation: Vector3::new(0.0, 0.0, 1.0),
    },
    TestCase {
      rotation: Quaternion::from_euler_angles(
        Degree::new(0.0),
        Degree::new(90.0),
        Degree::new(0.0),
      ),
      translation: Vector3::new(0.0, 1.0, 0.0),
    },
    TestCase {
      rotation: Quaternion::from_euler_angles(
        Degree::new(0.0),
        Degree::new(0.0),
        Degree::new(90.0),
      ),
      translation: Vector3::new(1.0, 0.0, 0.0),
    },
  ];

  for test in test_cases {
    let mut dual_quaternion = DualQuaternion::from_rotation(test.rotation);

    dual_quaternion.translate_world(&test.translation);
    let translation = dual_quaternion.world_translation();
    let rotation = dual_quaternion.rotation();

    assert!(
      translation.almost_eq(&test.translation),
      "{} ~= {}",
      translation,
      test.translation
    );
    assert!(
      rotation.almost_eq(&test.rotation),
      "{} ~= {}",
      rotation,
      test.rotation
    )
  }
}
