# sparse-bitfield
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Bitfield that allocates a series of small buffers. Adapted from
[mafintosh/sparse-bitfield].

- [Documentation][8]
- [Crates.io][2]

## Usage
```rust
extern crate sparse_bitfield;

use sparse_bitfield::{Bitfield};

let bits = Bitfield::new(1024);
bits.set(0, true); // set first bit
bits.set(1, true); // set second bit
bits.set(1_000_000_000_000, true); // set the trillionth bit
assert!(bits.get(1));
```

## Installation
```sh
$ cargo add sparse-bitfield
```

## License
[Apache-2.0](./LICENSE)

[1]: https://img.shields.io/crates/v/sparse-bitfield.svg?style=flat-square
[2]: https://crates.io/crates/sparse-bitfield
[3]: https://img.shields.io/travis/datrs/sparse-bitfield.svg?style=flat-square
[4]: https://travis-ci.org/datrs/sparse-bitfield
[5]: https://img.shields.io/crates/d/sparse-bitfield.svg?style=flat-square
[6]: https://crates.io/crates/sparse-bitfield
[7]: https://docs.rs/sparse-bitfield/badge.svg
[8]: https://docs.rs/sparse-bitfield

[mafintosh/sparse-bitfield]: https://github.com/mafintosh/sparse-bitfield
