#![deny(missing_docs)]
#![feature(external_doc)]
#![doc(include = "../README.md")]
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(clippy))]

extern crate memory_pager;
use memory_pager::Pager;

/// Bitfield instance.
pub struct Bitfield {
  /// The [`page_size`] of the `Page` instances stored in [memory-pager].
  ///
  /// [`page_size`]: https://docs.rs/memory-pager/0.1.0/memory_pager/struct.Pager.html#structfield.page_size
  /// [memory-pager]: https://docs.rs/memory-pager/
  pub page_size: usize,

  /// A [memory-pager] instance.
  ///
  /// [memory-pager]: https://docs.rs/memory-pager/
  pub pages: Pager,

  page_mask: usize,
  byte_length: usize,
  length: usize,
}

/// Create a new instance with a `page_size` of `1kb`.
impl Default for Bitfield {
  fn default() -> Self {
    let page_size = 1024;
    Bitfield::new(page_size)
  }
}

impl Bitfield {
  /// Create a new instance.
  pub fn new(page_size: usize) -> Self {
    assert!(is_even(page_size));
    let pages = Pager::new(page_size);
    let byte_length = pages.page_size * page_size;
    Bitfield {
      page_size,
      pages,
      byte_length,
      page_mask: page_size - 1,
      length: 8 * byte_length,
    }
  }

  /// Set a bit to true or false.
  pub fn set(&mut self, index: usize, value: bool) {
    let masked_index = index & 7;
    let j = (index - masked_index) / 8;
    let b = self.get_byte(j);

    if value {
      self.set_byte(j, b | (128 >> masked_index));
    } else {
      self.set_byte(j, b & (255 ^ (128 >> masked_index)));
    }
  }

  /// Get the value of a bit.
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
  fn get_byte(&mut self, index: usize) -> u8 {
    let masked_index = index & self.page_mask;
    let page_num = index / self.page_size;
    match self.pages.get_mut(page_num) {
      Some(page) => page[masked_index],
      None => 0,
    }
  }

  /// Set a byte to the right value inside our internal buffers.
  fn set_byte(&mut self, index: usize, byte: u8) -> bool {
    let masked_index = index & self.page_mask;
    let page_num = (index - masked_index) / self.page_size;
    let page = self.pages.get_mut_or_alloc(page_num);

    if index >= self.byte_length {
      self.byte_length = index + 1;
      self.length = self.byte_length * 8;
    }

    if page[masked_index] == byte {
      return false;
    }

    page[masked_index] = byte;
    if index >= self.byte_length {
      self.byte_length = index + 1;
      self.length = self.byte_length * 8;
    }

    true
  }

  /// Get the amount of bits stored. Includes sparse spaces.
  pub fn len(&self) -> usize {
    self.length
  }

  /// Check if `length` is zero.
  pub fn is_empty(&self) -> bool {
    self.length == 0
  }
}

#[inline]
fn is_even(x: usize) -> bool {
  (x & (x - 1)) == 0
}
