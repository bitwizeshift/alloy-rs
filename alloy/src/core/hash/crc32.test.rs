use super::*;

#[test]
fn checksum() {
  let bytes = b"hello-world";
  let checksum = Crc32::checksum(bytes);
  assert_eq!(checksum, 0xb1d4025b);
}
