extern crate sparse_bitfield;

use sparse_bitfield::Bitfield;

#[test]
fn can_create_bitfield() {
  let _bits = Bitfield::new(1024);
}

#[test]
fn can_set_bits() {
  let mut bits = Bitfield::new(1024);
  bits.set(0, true);
  bits.set(1, true);
  bits.set(1_000_000_000_000, true);
}

#[test]
fn can_get_bits() {
  let mut bits = Bitfield::new(1024);
  bits.set(0, true);
  bits.set(1, true);
  bits.set(1_000_000_000_000, true);
  assert_eq!(bits.get(0), true);
  assert_eq!(bits.get(1), true);
}
