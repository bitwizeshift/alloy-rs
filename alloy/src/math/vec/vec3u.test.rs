use super::*;

#[test]
fn identity() {
  let vec = Vector3u::new(4, 2, 0);

  assert_eq!(vec.as_ptr(), &vec[0]);
  assert_eq!(vec.as_ptr(), &vec.x);
}

#[test]
fn iter() {
  let mut vec = Vector3u::new(4, 2, 1);

  for v in vec.iter_mut() {
    *v = *v * 2
  }

  assert_eq!(vec.x, 8);
  assert_eq!(vec.y, 4);
  assert_eq!(vec.z, 2);
}
