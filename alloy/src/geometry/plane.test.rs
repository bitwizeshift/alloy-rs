use super::*;

#[test]
fn plane_distance_to_point() {
  let sut = Plane::new(0.0, 1.0, 0.0, 0.0);

  // On the plane
  assert_eq!(sut.distance_to_point(&Point3::new(0.0, 0.0, 0.0)), 0.0);

  // Above the plane
  assert_eq!(sut.distance_to_point(&Point3::new(0.0, 1.0, 0.0)), 1.0);

  // Below the plane
  assert_eq!(sut.distance_to_point(&Point3::new(0.0, -1.0, 0.0)), -1.0);
}

#[test]
fn plane_contains() {
  let sut = Plane::new(0.0, 1.0, 0.0, 0.0);

  // On the plane
  assert!(sut.contains(&Point3::new(0.0, 0.0, 0.0)));

  // Above the plane
  assert!(!sut.contains(&Point3::new(0.0, 1.0, 0.0)));

  // Below the plane
  assert!(!sut.contains(&Point3::new(0.0, -1.0, 0.0)));
}

#[test]
fn plane_nearest_point() {
  let sut = Plane::new(0.0, 1.0, 0.0, 0.0);

  // On the plane
  assert_eq!(
    sut.nearest_point(&Point3::new(0.0, 0.0, 0.0)),
    Point3::new(0.0, 0.0, 0.0)
  );

  // Above the plane
  assert_eq!(
    sut.nearest_point(&Point3::new(0.0, 1.0, 0.0)),
    Point3::new(0.0, 0.0, 0.0)
  );

  // Below the plane
  assert_eq!(
    sut.nearest_point(&Point3::new(0.0, -1.0, 0.0)),
    Point3::new(0.0, 0.0, 0.0)
  );
}

#[test]
fn plane_is_point_under_plane() {
  let sut = Plane::new(0.0, 1.0, 0.0, 0.0);

  // On the plane
  assert!(!sut.is_point_under_plane(&Point3::new(0.0, 0.0, 0.0)));

  // Above the plane
  assert!(!sut.is_point_under_plane(&Point3::new(0.0, 1.0, 0.0)));

  // Below the plane
  assert!(sut.is_point_under_plane(&Point3::new(0.0, -1.0, 0.0)));
}

#[test]
fn plane_is_point_over_plane() {
  let sut = Plane::new(0.0, 1.0, 0.0, 0.0);

  // On the plane
  assert!(!sut.is_point_over_plane(&Point3::new(0.0, 0.0, 0.0)));

  // Above the plane
  assert!(sut.is_point_over_plane(&Point3::new(0.0, 1.0, 0.0)));

  // Below the plane
  assert!(!sut.is_point_over_plane(&Point3::new(0.0, -1.0, 0.0)));
}
