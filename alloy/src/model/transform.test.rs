use super::*;

#[test]
fn scale_width() {
  let mut sut = Transform::default();
  let width = 2.0;
  let expected = Matrix4::from_arrays([
    [2.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
  ]);

  sut.scale_width(width);
  let mat = sut.transform().to_owned();

  assert_eq!(width, sut.scale_direction().x());
  assert_eq!(expected.as_mat4(), mat.as_mat4());
}

#[test]
fn scale_height() {
  let mut sut = Transform::default();
  let height = 2.0;
  let expected = Matrix4::from_arrays([
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 2.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
  ]);

  sut.scale_height(height);
  let mat = sut.transform().to_owned();

  assert_eq!(height, sut.scale_direction().y());
  assert_eq!(expected.as_mat4(), mat.as_mat4());
}

#[test]
fn scale_depth() {
  let mut sut = Transform::default();
  let depth = 2.0;
  let expected = Matrix4::from_arrays([
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 2.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
  ]);

  sut.scale_depth(depth);
  let mat = sut.transform().to_owned();

  assert_eq!(depth, sut.scale_direction().z());
  assert_eq!(expected.as_mat4(), mat.as_mat4());
}

#[test]
fn scale() {
  let mut sut = Transform::default();
  let scale = Vector3::new(2.0, 3.0, 4.0);
  let expected = Matrix4::from_arrays([
    [2.0, 0.0, 0.0, 0.0],
    [0.0, 3.0, 0.0, 0.0],
    [0.0, 0.0, 4.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
  ]);

  sut.scale(&scale);
  let mat = sut.transform().to_owned();

  assert_eq!(scale.as_vec3(), sut.scale_direction());
  assert_eq!(expected.as_mat4(), mat.as_mat4());
}

#[test]
fn scale_uniform() {
  let mut sut = Transform::default();
  let scale = 5.0;
  let expected = Matrix4::from_arrays([
    [5.0, 0.0, 0.0, 0.0],
    [0.0, 5.0, 0.0, 0.0],
    [0.0, 0.0, 5.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
  ]);

  sut.scale_uniform(scale);
  let mat = sut.transform().to_owned();

  assert_eq!(scale, sut.scale_direction().x());
  assert_eq!(scale, sut.scale_direction().y());
  assert_eq!(scale, sut.scale_direction().z());
  assert_eq!(expected.as_mat4(), mat.as_mat4());
}

#[test]
fn translate() {
  let mut sut = Transform::default();
  let translation = Vector3::new(1.0, 2.0, 3.0);
  let expected = Matrix4::from_arrays([
    [1.0, 0.0, 0.0, 1.0],
    [0.0, 1.0, 0.0, 2.0],
    [0.0, 0.0, 1.0, 3.0],
    [0.0, 0.0, 0.0, 1.0],
  ]);

  sut.translate(&translation);
  let mat = sut.transform().to_owned();
  let m = mat.as_mat4();

  assert_eq!(translation.as_vec3(), sut.translation());
  assert_eq!(expected.as_mat4(), m);
}
