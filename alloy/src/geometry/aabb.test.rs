use super::*;

#[test]
fn aabb_contains_point() {
  let sut = AABB::from_points(&Point3::new(-1.0, -1.0, -1.0), &Point3::new(1.0, 1.0, 1.0));

  // Included
  // Dead Center
  assert!(sut.contains_point(&Point3::new(0.0, 0.0, 0.0)));

  // On the right wall
  assert!(sut.contains_point(&Point3::new(1.0, 0.0, 0.0)));

  // On the left wall
  assert!(sut.contains_point(&Point3::new(-1.0, 0.0, 0.0)));

  // On the top wall
  assert!(sut.contains_point(&Point3::new(0.0, 1.0, 0.0)));

  // On the bottom wall
  assert!(sut.contains_point(&Point3::new(0.0, -1.0, 0.0)));

  // On the front wall
  assert!(sut.contains_point(&Point3::new(0.0, 0.0, 1.0)));

  // On the back wall
  assert!(sut.contains_point(&Point3::new(0.0, 0.0, -1.0)));

  // Excluded
  // Outside the right wall
  assert!(!sut.contains_point(&Point3::new(2.0, 0.0, 0.0)));

  // Outside the left wall
  assert!(!sut.contains_point(&Point3::new(-2.0, 0.0, 0.0)));

  // Outside the top wall
  assert!(!sut.contains_point(&Point3::new(0.0, 2.0, 0.0)));

  // Outside the bottom wall
  assert!(!sut.contains_point(&Point3::new(0.0, -2.0, 0.0)));

  // Outside the front wall
  assert!(!sut.contains_point(&Point3::new(0.0, 0.0, 2.0)));

  // Outside the back wall
  assert!(!sut.contains_point(&Point3::new(0.0, 0.0, -2.0)));
}

#[test]
fn aabb_intersects() {
  let aabb1 = AABB::from_points(&Point3::new(-1.0, -1.0, -1.0), &Point3::new(1.0, 1.0, 1.0));
  let aabb2 = AABB::from_points(&Point3::new(0.0, 0.0, 0.0), &Point3::new(2.0, 2.0, 2.0));
  let aabb3 = AABB::from_points(&Point3::new(2.0, 2.0, 2.0), &Point3::new(3.0, 3.0, 3.0));
  let aabb4 = AABB::from_points(&Point3::new(-0.5, -0.5, -0.5), &Point3::new(0.5, 0.5, 0.5));

  // Intersection
  // Identity intersection
  assert!(aabb1.intersects(&aabb1));

  // Intersects Other
  assert!(aabb1.intersects(&aabb2));

  // Encloses
  assert!(aabb1.intersects(&aabb4));

  // No Intersection
  // Touches wall, but does not intersect
  assert!(!aabb1.intersects(&aabb3));

  assert!(!aabb1.intersects(&aabb3));
}

#[test]
fn aabb_encloses() {
  let aabb1 = AABB::from_points(&Point3::new(-1.0, -1.0, -1.0), &Point3::new(1.0, 1.0, 1.0));
  let aabb2 = AABB::from_points(&Point3::new(-0.5, -0.5, -0.5), &Point3::new(0.5, 0.5, 0.5));
  let aabb3 = AABB::from_points(&Point3::new(0.0, 0.0, 0.0), &Point3::new(2.0, 2.0, 2.0));

  // Identity Enclosure
  assert!(aabb1.encloses(&aabb1));

  // Encloses Other
  assert!(aabb1.encloses(&aabb2));

  // Intersects but does not enclose
  assert!(!aabb1.encloses(&aabb3));
}

#[test]
fn aabb_nearest_point() {
  let sut = AABB::from_points(&Point3::new(-1.0, -1.0, -1.0), &Point3::new(1.0, 1.0, 1.0));

  // Dead Center
  assert_eq!(
    sut.nearest_point(&Point3::new(0.0, 0.0, 0.0)),
    Point3::new(0.0, 0.0, 0.0)
  );

  // On the right wall
  assert_eq!(
    sut.nearest_point(&Point3::new(2.0, 0.0, 0.0)),
    Point3::new(1.0, 0.0, 0.0)
  );

  // On the left wall
  assert_eq!(
    sut.nearest_point(&Point3::new(-2.0, 0.0, 0.0)),
    Point3::new(-1.0, 0.0, 0.0)
  );

  // On the top wall
  assert_eq!(
    sut.nearest_point(&Point3::new(0.0, 2.0, 0.0)),
    Point3::new(0.0, 1.0, 0.0)
  );

  // On the bottom wall
  assert_eq!(
    sut.nearest_point(&Point3::new(0.0, -2.0, 0.0)),
    Point3::new(0.0, -1.0, 0.0)
  );

  // On the front wall
  assert_eq!(
    sut.nearest_point(&Point3::new(0.0, 0.0, 2.0)),
    Point3::new(0.0, 0.0, 1.0)
  );

  // On the back wall
  assert_eq!(
    sut.nearest_point(&Point3::new(0.0, 0.0, -2.0)),
    Point3::new(0.0, 0.0, -1.0)
  );
}

#[test]
fn aabb_distance_to() {
  let sut = AABB::from_points(&Point3::new(-1.0, -1.0, -1.0), &Point3::new(1.0, 1.0, 1.0));

  // Inside the box
  assert_eq!(sut.distance_to(&Point3::new(0.0, 0.0, 0.0)), 0.0);

  // Distance to right wall
  assert_eq!(sut.distance_to(&Point3::new(2.0, 0.0, 0.0)), 1.0);

  // Distance to left wall
  assert_eq!(sut.distance_to(&Point3::new(-2.0, 0.0, 0.0)), 1.0);

  // Distance to top wall
  assert_eq!(sut.distance_to(&Point3::new(0.0, 2.0, 0.0)), 1.0);

  // Distance to bottom wall
  assert_eq!(sut.distance_to(&Point3::new(0.0, -2.0, 0.0)), 1.0);

  // Distance to front wall
  assert_eq!(sut.distance_to(&Point3::new(0.0, 0.0, 2.0)), 1.0);

  // Distance to back wall
  assert_eq!(sut.distance_to(&Point3::new(0.0, 0.0, -2.0)), 1.0);
}
