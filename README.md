# Endian Type

`endian_type` is an `no-std`, endian-aware implementation of primitive types like `u8`, `u16`, etc.

All endian types are implemented in a transparent way that it can directly replace the original
types with almost zero overhead.

## Usage
Add this to your Cargo.toml
```
endian_type = "0.1.0"
```
and import endian-ware types from this crate.
```
use endian_type::types::*;
```

## Features
Here is some features of `endian_type` crate.
### [no-std] Support
This crate can be used without `std` support with no requirements or additional features needed
as it only relies on `core`.
### Drop-in replacement
This crate provides sufficient default operations for endian-aware types.

For example, you can directly compare a primitive type with a endian-aware type.
```
use endian_type::*;
let num_le = u16_le::from_native(10);
assert!(num_le < 11);
```
Other bit-wise ops like `&`, `|` and arithmetic ops like `+`, `-` are also supported.
```
use endian_type::*;
let a = u32_le::from_native(1);
let b = u32_be::from_native(2);
assert!(a+b == 3);
```


