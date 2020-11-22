# one-of

[![version](https://img.shields.io/crates/v/one-of?logo=rust&style=flat-square)](https://crates.io/crates/one-of)
[![dependencies](https://img.shields.io/librariesio/release/cargo/one-of?style=flat-square)](https://libraries.io/cargo/one-of)
[![license](https://img.shields.io/badge/license-MPL--2.0-blue?style=flat-square)](https://www.mozilla.org/en-US/MPL/2.0)
[![ci](https://img.shields.io/github/workflow/status/figsoda/one-of/ci?label=ci&logo=github-actions&style=flat-square)](https://github.com/figsoda/one-of/actions?query=workflow:ci)

Macro to represent a type that can be converted either `From` or `Into` the given types

[Documentation](https://docs.rs/one-of)


## Usage

```rust
use one_of::one_of;

// either `u32` or `char`
let x: one_of!(u32, char) = 42.into();
assert_eq!(Some(42u32), x.into());
assert_eq!(Option::<char>::None, x.into());

// some type of integer
// integer literal defaults to `i32` in rust
let x: one_of!(i8, i16, i32, i64, u8, u16, u32, u64) = 42.into();
assert_eq!(Option::<i8>::None, x.into());
assert_eq!(Option::<i16>::None, x.into());
assert_eq!(Some(42i32), x.into());
assert_eq!(Option::<i64>::None, x.into());
assert_eq!(Option::<u8>::None, x.into());
assert_eq!(Option::<u16>::None, x.into());
assert_eq!(Option::<u32>::None, x.into());
assert_eq!(Option::<u64>::None, x.into());
```


## Changelog

See [CHANGELOG.md](https://github.com/figsoda/one-of/blob/main/CHANGELOG.md)
