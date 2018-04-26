extern crate sparse_bitfield;

use sparse_bitfield::{Bitfield, Change};

// src/lib.rs::is_even was incorrectly flagging 6 as odd.
#[test]
#[should_panic] // 6 is not a power of 2, which later became a constraint
fn regression_01() {
  let mut bits = Bitfield::new(6);
  assert_eq!(bits.set(0, true), Change::Changed);
  assert_eq!(bits.get(0), true);
}

// we learned that we actually need the Bitfield to have a
// page size that is a POWER of two, not a multiple.
#[test]
#[should_panic]
fn regression_02() {
  let mut bits = Bitfield::new(2566);
  assert_eq!(bits.set(332288, true), Change::Changed);
  assert_eq!(bits.get(332288), true);
}
