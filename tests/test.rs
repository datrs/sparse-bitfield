extern crate sparse_bitfield;

use sparse_bitfield::{Bitfield, Change};

#[test]
fn can_create_bitfield() {
  let _bits = Bitfield::new(1024);
}

#[test]
fn basic_set_get() {
  let mut bits = Bitfield::new(1024);
  bits.set(0, true);
  assert!(bits.get(0), true);
}

#[test]
fn can_set_bits() {
  let mut bits = Bitfield::new(1024);
  bits.set(100, true);
  bits.set(1_000, true);
  bits.set(1_000_000, true);
  bits.set(1_000_000_000, true);
  // bits.set(1_000_000_000_000, true);
}

#[test]
fn can_get_bits() {
  let mut bits = Bitfield::new(1024);
  bits.set(0, true);
  bits.set(1, true);
  bits.set(1000, true);
  assert_eq!(bits.get(0), true);
  assert_eq!(bits.get(1), true);
}

#[test]
fn returns_if_flipped() {
  let mut bits = Bitfield::new(1024);
  assert_eq!(bits.set(0, true), Change::Changed);
  assert_eq!(bits.set(0, false), Change::Changed);
  assert_eq!(bits.set(0, true), Change::Changed);
  assert_eq!(bits.set(0, true), Change::Unchanged);
  assert_eq!(bits.set(0, true), Change::Unchanged);
}

#[test]
fn exposes_changed_unchanged_methods() {
  let mut bits = Bitfield::new(1024);
  assert!(bits.set(0, true).is_changed());
  assert!(bits.set(0, true).is_unchanged());
}

#[test]
fn can_iterate() {
  let mut bits = Bitfield::new(1024);

  bits.set(0, true);
  for bit in bits.iter() {
    assert_eq!(bit, true, "one bit, value true");
  }

  bits.set(1, false);
  for (i, bit) in bits.iter().enumerate() {
    match i {
      0 => assert!(bit, true),
      1 => assert!(bit, false),
      i => panic!("index {} out of bounds", i),
    }
  }
}
