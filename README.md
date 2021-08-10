# Endian Type
[![Crate](https://img.shields.io/crates/v/endiantype.svg)](https://crates.io/crates/endiantype)
[![API](https://docs.rs/endiantype/badge.svg)](https://docs.rs/endiantype)

`endiantype` is an `no-std`, endian-aware implementation of primitive types like `u8`, `u16`, etc.

All endian types are implemented in a transparent way that it can directly replace the original
types with almost zero overhead.

## Usage
Add this to your Cargo.toml
```
endiantype = "0.1.2"
```
to use in a `[no_std]` environment, you need to disable default features.
```
endiantype = { version = "0.1.3", default-features = false}
```
and import endian-ware types from this crate.
```rust
use endiantype::types::*;
```

## Features
Here is some features of `endiantype` crate.
### [no-std] Support
This crate can be used without `std` support with no requirements or additional features needed
as it only relies on `core`.
### Drop-in replacement
This crate provides sufficient default operations for endian-aware types.

For example, you can directly compare a primitive type with a endian-aware type.
```rust
use endiantype::*;
let num_le = u16_le::from_native(10);
assert!(num_le < 11);
```
Other bit-wise ops like `&`, `|` and arithmetic ops like `+`, `-` are also supported.
```rust
use endiantype::*;
let a = u32_le::from_native(1);
let b = u32_be::from_native(2);
assert!(a+b == 3);
```


