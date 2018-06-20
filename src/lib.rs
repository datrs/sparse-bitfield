#![cfg_attr(nightly, deny(missing_docs))]
#![cfg_attr(nightly, feature(external_doc))]
#![cfg_attr(nightly, doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]

extern crate memory_pager;

mod change;
mod iter;

pub use change::Change;
pub use iter::Iter;
use memory_pager::Pager;

/// Bitfield instance.
#[derive(Debug)]
pub struct Bitfield {
  /// A [memory-pager] instance.
  ///
  /// [memory-pager]: https://docs.rs/memory-pager/
  pub pages: Pager,

  length: usize,
  page_length: usize,
}

/// Create a new instance with a `page_size` of `1kb`.
impl Default for Bitfield {
  #[inline]
  fn default() -> Self {
    let page_size = 1024;
    Bitfield::new(page_size)
  }
}

impl Bitfield {
  /// Create a new instance.
  ///
  /// ## Panics
  /// Panics if the page size is not a power of two (2, 4, 8, etc.)
  pub fn new(page_size: usize) -> Self {
    assert!(is_power_of_two(page_size));
    Bitfield {
      pages: Pager::new(page_size),
      page_length: 0,
      length: 0,
    }
  }

  /// Set a byte to true or false. Returns a boolean indicating if the value was
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
    let masked_index = index & 7;
    let j = (index - masked_index) / 8;

    let num = self.get_byte(j) & (128 >> masked_index);
    match num {
      0 => false,
      _ => true,
    }
  }

  /// Get a byte from our internal buffers.
  #[inline]
  pub fn get_byte(&self, index: usize) -> u8 {
    let masked_index = self.page_mask(index);
    let page_num = index / self.page_size();
    match self.pages.get(page_num) {
      Some(page) => page[masked_index],
      None => 0,
    }
  }

  /// Set a byte to the right value inside our internal buffers.
  #[inline]
  pub fn set_byte(&mut self, index: usize, byte: u8) -> Change {
    let masked_index = self.page_mask(index);
    let page_num = (index - masked_index) / self.page_size();
    let page = self.pages.get_mut_or_alloc(page_num);

    if index >= self.length {
      self.length = index + 1;
    }

    if page[masked_index] == byte {
      Change::Unchanged
    } else {
      page[masked_index] = byte;
      Change::Changed
    }
  }

  /// Get the memory page size in bytes.
  #[inline]
  pub fn page_size(&self) -> usize {
    self.pages.page_size()
  }

  /// Get the amount of bytes in the bitfield.
  ///
  /// ## Examples
  /// ```rust
  /// # extern crate sparse_bitfield;
  /// # use sparse_bitfield::Bitfield;
  /// let mut bits = Bitfield::new(1024);
  /// assert_eq!(bits.len(), 0);
  /// bits.set(0, true);
  /// assert_eq!(bits.len(), 1);
  /// bits.set(1, true);
  /// assert_eq!(bits.len(), 1);
  /// bits.set(9, false);
  /// assert_eq!(bits.len(), 2);
  /// ```
  #[inline]
  pub fn len(&self) -> usize {
    self.length
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
    self.len() == 0
  }

  /// Create an `Iterator` that iterates over all pages.
  pub fn iter(&mut self) -> Iter {
    Iter::new(self)
  }

  #[inline]
  fn page_mask(&self, index: usize) -> usize {
    index & (self.page_size() - 1)
  }
}

#[inline]
fn is_power_of_two(x: usize) -> bool {
  x.count_ones() == 1
}
