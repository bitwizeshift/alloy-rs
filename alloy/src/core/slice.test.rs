use super::*;

#[test]
fn strided_slice_index_usize() {
  let source: &[i32] = &[1, 2, 3, 4, 5, 6, 7, 8, 9];

  let stride = StridedSlice::<i32, 3>::from_slice(source);

  assert_eq!(stride[0], 1);
  assert_eq!(stride[1], 4);
  assert_eq!(stride[2], 7);
}

#[test]
fn strided_slice_index_range() {
  let source: &[i32] = &[1, 2, 3, 4, 5, 6, 7, 8, 9];
  let expected: &[i32] = &[1, 4, 7];

  let stride = StridedSlice::<i32, 3>::from_slice(source);

  assert_eq!(stride[0..3], expected);
}

#[test]
fn stride_slice_index_range_from() {
  let source: &[i32] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
  let expected: &[i32] = &[1, 4, 7, 10];

  let stride = StridedSlice::<i32, 3>::from_slice(source);

  assert_eq!(stride[0..], expected);
}

#[test]
fn stride_slice_index_range_to() {
  let source: &[i32] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
  let expected: &[i32] = &[1, 4, 7];

  let stride = StridedSlice::<i32, 3>::from_slice(source);

  assert_eq!(stride[..3], expected);
}

#[test]
fn strided_slice_index_range_inclusive() {
  let source: &[i32] = &[1, 2, 3, 4, 5, 6, 7, 8, 9];
  let expected: &[i32] = &[1, 4, 7];

  let stride = StridedSlice::<i32, 3>::from_slice(source);

  assert_eq!(stride[0..=2], expected);
}

#[test]
fn strided_slice_index_mut_usize() {
  let source: &mut [i32] = &mut [1, 2, 3, 4, 5, 6, 7, 8, 9];

  let stride = StridedSlice::<i32, 3>::from_mut_slice(source);

  assert_eq!(stride[0], 1);
  assert_eq!(stride[1], 4);
  assert_eq!(stride[2], 7);

  stride[0] = 10;
  stride[1] = 11;
  stride[2] = 12;

  assert_eq!(stride[0], 10);
  assert_eq!(stride[1], 11);
  assert_eq!(stride[2], 12);
}
