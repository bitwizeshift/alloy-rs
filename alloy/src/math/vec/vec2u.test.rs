use super::*;

#[test]
fn identity() {
  let vec = Vector2u::new(4, 2);

  assert_eq!(vec.as_ptr(), &vec[0]);
  assert_eq!(vec.as_ptr(), &vec.x);
}

#[test]
fn iter() {
  let mut vec = Vector2u::new(4, 2);

  for v in vec.iter_mut() {
    *v = *v * 2
  }

  assert_eq!(vec.x, 8);
  assert_eq!(vec.y, 4);
}

#[test]
fn add() {
  let a = Vector2u { x: 10, y: 10 };
  let b = Vector2u { x: 0, y: 10 };

  let c = a + b;

  assert_eq!(c, Vector2u { x: 10, y: 20 })
}
