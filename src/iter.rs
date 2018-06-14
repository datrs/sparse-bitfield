use super::Bitfield;
use std::iter;

/// Iterate over a `Bitfield` instance.
pub struct Iter<'a> {
  pub(crate) inner: &'a mut Bitfield,
  pub(crate) cursor: usize,
}

impl<'a> iter::Iterator for Iter<'a> {
  type Item = bool;

  fn next(&mut self) -> Option<Self::Item> {
    let cursor = self.cursor;
    self.cursor += 1;

    if cursor >= self.inner.len() {
      None
    } else {
      Some(self.inner.get(cursor))
    }
  }
}
