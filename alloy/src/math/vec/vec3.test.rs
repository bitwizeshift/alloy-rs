use super::*;

#[test]
fn vec3() {
  let vec = Vector3::new(4.0, 2.0, 0.0);

  let magnitude = vec.square_magnitude();

  assert!(magnitude.almost_eq(&20.0))
}

#[test]
fn identity() {
  let vec = Vector3::new(4.0, 2.0, 0.0);

  assert_eq!(vec.as_ptr(), &vec[0]);
  assert_eq!(vec.as_ptr(), &vec.x);
}

#[test]
fn iter() {
  let mut vec = Vector3::new(4.0, 2.0, 1.0);

  for v in vec.iter_mut() {
    *v = *v * 2.0
  }

  assert!(vec.x.almost_eq(&8.0), "x = {}", vec.x);
  assert!(vec.y.almost_eq(&4.0), "y = {}", vec.y);
  assert!(vec.z.almost_eq(&2.0), "z = {}", vec.z);
}

#[test]
fn vec3_add() {
  let mut lhs = Vector3::new(4.0, 2.0, 1.0);
  lhs *= 4.0;
  lhs += lhs.clone();
  lhs += Vector3::new(1.0, 1.0, 1.0);
  assert!(lhs.as_vec3().dot(Vector3::new(1.0, 1.0, 1.0).as_vec3()) != 0.0);
}
