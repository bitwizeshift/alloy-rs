use super::*;

#[test]
fn vec2() {
  let vec = Vector2::new(4.0, 2.0);

  let magnitude = vec.square_magnitude();

  assert!(magnitude.almost_eq(&20.0))
}

#[test]
fn identity() {
  let vec = Vector2::new(4.0, 2.0);

  assert_eq!(vec.as_ptr(), &vec[0]);
  assert_eq!(vec.as_ptr(), &vec.x);
}

#[test]
fn iter() {
  let mut vec = Vector2::new(4.0, 2.0);

  for v in vec.iter_mut() {
    *v = *v * 2.0
  }

  assert!(vec.x.almost_eq(&8.0), "x = {}", vec.x);
  assert!(vec.y.almost_eq(&4.0), "y = {}", vec.y);
}

#[test]
fn add() {
  let a = Vector2 { x: 10.0, y: 10.0 };
  let b = Vector2 { x: 0.0, y: 10.0 };

  let c = a + b;

  assert!(c.almost_eq(&Vector2 { x: 10.0, y: 20.0 }))
}
