use super::Bitfield;
use std::iter;

/// Iterate over a `Bitfield` instance.
// TODO: the most efficient way to iterate this is to get a page at the time &
// cache it. Then get a bit at the time & cache it. That should allow us to
// return values much faster, rather than doing a heap lookup for every bit.
//
// The CPU's (L3) cache might kick in here to store a page at the time, but
// there's no guarantees that it does. So it's up to us to eventually optimize
// this.
pub struct Iter<'p, 'b> {
  pub(crate) inner: &'b mut Bitfield<'p>,
  pub(crate) cursor: usize,
}

impl<'p, 'b> Iter<'p, 'b> {
  #[inline]
  pub(crate) fn new(inner: &'b mut Bitfield<'p>) -> Self {
    Self { inner, cursor: 0 }
  }
}

impl<'p, 'b> iter::Iterator for Iter<'p, 'b> {
  type Item = bool;

  fn next(&mut self) -> Option<Self::Item> {
    let cursor = self.cursor;
    self.cursor += 1;

    // Each byte contains 8 bits, so we must iterate over each bit.
    if cursor >= self.inner.len() {
      None
    } else {
      Some(self.inner.get(cursor))
    }
  }
}
