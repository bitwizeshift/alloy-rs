use super::*;

#[test]
fn vector3_from_points() {
  let sut = Vector3::from_points(&Point3::new(0.0, 0.0, 0.0), &Point3::new(1.0, 1.0, 1.0));

  assert_eq!(sut, Vector3::new(1.0, 1.0, 1.0));
}
