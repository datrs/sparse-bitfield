use proptest::proptest;

use sparse_bitfield::{Bitfield, Change};

fn model(bit: usize, page_sz: usize) {
  let mut bits = Bitfield::new(page_sz);
  assert_eq!(bits.set(bit, true), Change::Changed);
  assert_eq!(bits.get(bit), true);
}

proptest! {
  #[test]
  fn doesnt_crash(bit in 0usize..1_000_000, page_sz_exponent in 0usize..30) {
    model(bit, 1 << page_sz_exponent);
  }
}
