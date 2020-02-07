use failure::Error;
use sparse_bitfield::{Bitfield, Change};
use std::fs;

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
  for (i, bit) in bits.iter().enumerate() {
    match i {
      0 => assert_eq!(bit, true),
      _ => assert_eq!(bit, false),
    }
  }

  let arr: Vec<bool> = bits.iter().collect();
  assert_eq!(arr.len(), 8);

  bits.set(1, false);
  for (i, bit) in bits.iter().enumerate() {
    match i {
      0 => assert_eq!(bit, true),
      _ => assert_eq!(bit, false),
    }
  }

  let arr: Vec<bool> = bits.iter().collect();
  assert_eq!(arr.len(), 8);
}

#[test]
fn can_convert_to_bytes_buffer() {
  let mut bits = Bitfield::new(1024);

  assert_eq!(bits.to_bytes().unwrap(), vec![]);

  bits.set(0, true);

  assert_eq!(
    &bits.to_bytes().unwrap(),
    &bits.pages.get(0).unwrap().as_ref()
  );

  bits.set(9000, true);

  let mut concat_pages = bits.pages.get(0).unwrap().as_ref().to_vec();
  concat_pages.extend_from_slice(&bits.pages.get(1).unwrap().as_ref());
  assert_eq!(bits.to_bytes().unwrap(), concat_pages);
}

#[test]
fn from_file() -> Result<(), Error> {
  let page_size = 10;
  let mut file = fs::File::open("./tests/fixtures/40_normal.txt")?;
  let mut bits = Bitfield::from_file(&mut file, page_size, None)?;
  bits.set(100, false);
  assert_eq!(bits.page_len(), 4);
  assert_eq!(bits.len(), 320);
  assert_eq!(bits.get(100), false);
  Ok(())
}
