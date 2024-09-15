use crate::math::vec::Vector3;

use super::*;

#[test]
fn color_from_vec3() {
  let input = Vector3::new(1.0, 0.5, 0.25);
  let color = Color::from_vec3(&input);

  assert_eq!(color.rgb(), input.as_vec3());
  assert_eq!(color.a(), 1.0);
}

#[test]
fn color_from_vec4() {
  let input = Vector4::new(1.0, 0.5, 0.25, 0.75);

  let color = Color::from_vec4(&input);

  assert_eq!(color.as_vec4(), input.as_vec4());
}

#[test]
fn color_from_rgba32() {
  let before = 0x11_22_33_44;

  let color = Color::from_rgba32(0x11223344);

  let after = color.to_rgba32();

  assert_eq!(before, after);
}

#[test]
fn color_from_argb32() {
  let before = 0x11_22_33_44;

  let color = Color::from_argb32(0x11223344);

  let after = color.to_argb32();

  assert_eq!(before, after);
}

#[test]
fn color_from_rgba64() {
  let before = 0x1122_3344_5566_7788;

  let color = Color::from_rgba64(before);

  let after = color.to_rgba64();

  assert_eq!(before, after);
}

#[test]
fn color_from_argb64() {
  let before = 0x1122_3344_5566_7788;

  let color = Color::from_argb64(before);

  let after = color.to_argb64();

  assert_eq!(before, after);
}
