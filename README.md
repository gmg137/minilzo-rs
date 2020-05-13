# minilzo-rs

A pure rust implementation bound to the C version of minilzo.

![License](https://img.shields.io/github/license/gmg137/minilzo-rs.svg)
[![Actions Status](https://github.com/gmg137/minilzo-rs/workflows/CI/badge.svg)](https://github.com/gmg137/minilzo-rs/actions)

## Functions

- compress
- decompress
- adler32

## Usage
```rust
    // test compress
    let mut lzo = minilzo_rs::LZO::new().unwrap();
    let input = [0x00u8; 1024];
    let out = lzo.compress(&input).unwrap();

    // test decompress
    let input = lzo.decompress_safe(&out[..], 1024);
    let input = input.unwrap();
    assert_eq!(input.len(), 1024);
```

## License
This project's source code and documentation is licensed under the  [GNU General Public License](LICENSE) (GPL v3).

LZO itself is licensed under the terms of the [GNU General Public License](http://www.oberhumer.com/opensource/gpl.html) (GPL v2+).
