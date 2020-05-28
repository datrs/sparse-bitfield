#![cfg_attr(nightly, deny(missing_docs))]
#![cfg_attr(nightly, feature(external_doc))]
#![cfg_attr(nightly, doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]

mod change;
mod iter;

pub use crate::change::Change;
pub use crate::iter::Iter;

use memory_pager::Pager;
use std::cell::RefMut;
use std::fs::File;
use std::io;

/// Bitfield instance.
#[derive(Debug)]
pub struct Bitfield<'p> {
  pages: &'p Pager,
  byte_length: usize,
  page_length: usize,
}

impl<'p> Bitfield<'p> {
  /// Create a new instance.
  ///
  /// ## Panics
  /// The page size must be a multiple of 2, and bigger than 0.
  pub fn new(page_size: usize) -> Self {
    assert!(page_size.is_power_of_two());
    let pages = Pager::new(page_size);
    Bitfield {
      pages: &pages,
      page_length: 0,
      byte_length: 0,
    }
  }

  /// Create a new instance from a `File`.
  pub fn from_file(
    file: &mut File,
    page_size: usize,
    offset: Option<usize>,
  ) -> io::Result<Self> {
    let pages = Pager::from_file(file, page_size, offset)?;
    let page_length = pages.len();

    // NOTE: empty pages are initialized as `0` filled. So when we reinitialize
    // a page, in essence our byte length becomes the amount of bytes we have
    // times the amount of pages we have.
    let byte_length = page_length * page_size;

    Ok(Self {
      pages: &pages,
      page_length,
      byte_length,
    })
  }

  /// Create an instance from a `Pager` instance.
  pub fn from_pager(pages: RefMut<'p, Pager>) -> Result<Self, Error> {
    let page_length = pages.len();
    let byte_length = page_length * pages.page_size();

    Ok(Self {
      pages: &pages,
      page_length,
      byte_length,
    })
  }

  /// Set a bit to true or false. Returns a boolean indicating if the value was
  /// changed.
  #[inline]
  pub fn set(&mut self, index: usize, value: bool) -> Change {
    let index_mask = index & 7;
    let byte_index = (index - index_mask) / 8;
    let byte = self.get_byte(byte_index);

    if value {
      // Mask the byte to flip a bit to `1`.
      let byte = byte | (128 >> index_mask);
      self.set_byte(byte_index, byte)
    } else {
      // Mask the byte to flip a bit to `0`.
      let byte = byte & (255 ^ (128 >> index_mask));
      self.set_byte(byte_index, byte)
    }
  }

  /// Get the value of a bit.
  #[inline]
  pub fn get(&mut self, index: usize) -> bool {
    let byte_offset = index & 7;
    let j = (index - byte_offset) / 8;

    let num = self.get_byte(j) & (128 >> byte_offset);
    match num {
      0 => false,
      _ => true,
    }
  }

  /// Get a byte from our internal buffers.
  #[inline]
  pub fn get_byte(&self, index: usize) -> u8 {
    let byte_offset = self.page_mask(index);
    let page_num = index / self.page_size();
    match self.pages.get(page_num) {
      Some(page) => page[byte_offset],
      None => 0,
    }
  }

  /// Set a byte to the right value inside our internal buffers.
  #[inline]
  pub fn set_byte(&mut self, index: usize, byte: u8) -> Change {
    let byte_offset = self.page_mask(index);
    let page_num = index / self.page_size();
    let page = self.pages.get_mut_or_alloc(page_num);

    if index >= self.byte_length {
      self.byte_length = index + 1;
    }

    if page[byte_offset] == byte {
      Change::Unchanged
    } else {
      page[byte_offset] = byte;
      Change::Changed
    }
  }

  /// Get the memory page size in bytes.
  #[inline]
  pub fn page_size(&self) -> usize {
    self.pages.page_size()
  }

  /// Get the amount of bits in the bitfield.
  ///
  /// ## Examples
  /// ```rust
  /// # extern crate sparse_bitfield;
  /// # use sparse_bitfield::Bitfield;
  /// let mut bits = Bitfield::new(1024);
  /// assert_eq!(bits.len(), 0);
  /// bits.set(0, true);
  /// assert_eq!(bits.len(), 8);
  /// bits.set(1, true);
  /// assert_eq!(bits.len(), 8);
  /// bits.set(9, false);
  /// assert_eq!(bits.len(), 16);
  /// ```
  #[inline]
  pub fn len(&self) -> usize {
    self.byte_length * 8
  }

  /// Get the amount of bytes in the bitfield.
  ///
  /// ## Examples
  /// ```rust
  /// # extern crate sparse_bitfield;
  /// # use sparse_bitfield::Bitfield;
  /// let mut bits = Bitfield::new(1024);
  /// assert_eq!(bits.byte_len(), 0);
  /// bits.set(0, true);
  /// assert_eq!(bits.byte_len(), 1);
  /// bits.set(1, true);
  /// assert_eq!(bits.byte_len(), 1);
  /// bits.set(9, false);
  /// assert_eq!(bits.byte_len(), 2);
  /// ```
  #[inline]
  pub fn byte_len(&self) -> usize {
    self.byte_length
  }

  /// Get the amount of memory pages in the bitfield.
  ///
  /// ## Examples
  /// ```rust
  /// # extern crate sparse_bitfield;
  /// # use sparse_bitfield::Bitfield;
  /// let mut bits = Bitfield::new(1024);
  /// assert_eq!(bits.page_len(), 0);
  /// bits.set(0, true);
  /// assert_eq!(bits.page_len(), 1);
  /// bits.set(1, true);
  /// assert_eq!(bits.page_len(), 1);
  /// bits.set(2, false);
  /// assert_eq!(bits.page_len(), 1);
  /// bits.set(1024 * 8 + 1, true);
  /// assert_eq!(bits.page_len(), 2);
  /// ```
  #[inline]
  pub fn page_len(&self) -> usize {
    self.pages.len()
  }

  /// Returns `true` if no bits are stored.
  ///
  /// ## Examples
  /// ```rust
  /// # extern crate sparse_bitfield;
  /// # use sparse_bitfield::Bitfield;
  /// let mut bits = Bitfield::new(1024);
  /// assert!(bits.is_empty());
  /// bits.set(0, true);
  /// assert!(!bits.is_empty());
  /// ```
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.pages.is_empty()
  }

  /// Create an `Iterator` that iterates over all pages.
  #[inline]
  pub fn iter<'b: 'p>(&'p mut self) -> Iter<'b, 'p> {
    Iter::new(self)
  }

  #[inline]
  /// Find which page we should write to.
  fn page_mask(&self, index: usize) -> usize {
    index & (self.page_size() - 1)
  }

  /// Based on [Bitfield.prototype.toBuffer](https://github.com/mafintosh/sparse-bitfield/blob/master/index.js#L54-L64)
  pub fn to_bytes(&self) -> std::io::Result<Vec<u8>> {
    use std::io::{Cursor, Write};

    let mut all =
      Cursor::new(Vec::with_capacity(self.page_len() * self.page_size()));

    for index in 0..self.page_len() {
      let next = self.pages.get(index);
      if let Some(page) = next {
        let all_offset = index * self.page_size();
        all.set_position(all_offset as u64);
        all.write_all(&page)?;
      }
    }

    Ok(all.into_inner())
  }
}

/// Create a new instance with a `page_size` of `1kb`.
impl<'p> Default for Bitfield<'p> {
  #[inline]
  fn default() -> Self {
    let page_size = 1024;
    Bitfield::new(page_size)
  }
}
