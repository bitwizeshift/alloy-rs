use super::*;

#[test]
fn strided_slice_index_usize() {
  let source: &[f32] = &[
    1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
  ];

  let sut = unsafe { Col4::from_slice(source) };

  assert_eq!(sut[0], 1.0);
  assert_eq!(sut[1], 5.0);
  assert_eq!(sut[2], 9.0);
  assert_eq!(sut[3], 13.0);
}
