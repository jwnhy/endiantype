//! # Endian Type
//!
//! `endian_type` is an `no-std`, endian-aware implementation of primitive types like `u8`, `u16`, etc.
//!
//! All endian types are implemented in a transparent way that it can directly replace the original
//! types with almost zero overhead.
//!
//! ## Usage
//! Add this to your Cargo.toml
//! ```
//! endian_type = "0.1.0"
//! ```
//! and import endian-ware types from this crate.
//! ```
//! use endian_type::types::*;
//! ```
//!
//! ## Features
//! Here is some features of `endian_type` crate.
//! ### [no-std] Support
//! This crate can be used without `std` support with no requirements or additional features needed
//! as it only relies on `core`.
//! ### Drop-in replacement
//! This crate provides sufficient default operations for endian-aware types.
//!
//! For example, you can directly compare a primitive type with a endian-aware type.
//! ```
//! use endian_type::*;
//! let num_le = u16_le::from_native(10);
//! assert!(num_le < 11);
//! ```
//! Other bit-wise ops like `&`, `|` and arithmetic ops like `+`, `-` are also supported.
//! ```
//! use endian_type::*;
//! let a = u32_le::from_native(1);
//! let b = u32_be::from_native(2);
//! assert!(a+b == 3);
//! ```

use core::cmp::Ordering;
use core::ops::{Add, BitAnd, BitOr, BitXor, Sub};

pub use types::*;

/// # Little endian types
/// ## Example
/// ```
/// use endian_type::*;
/// // endian types can be created from native types;
/// let deadbeef = u32_le::from_native(0xdeadbeef);
/// assert!(deadbeef == 0xdeadbeef);
/// // or use `From` trait of native types;
/// let deadbeef: u32_le = 0xdeadbeef.into();
/// assert!(deadbeef == 0xdeadbeef);
/// // or use `From` trait of another endian types;
/// let deadbeef: u32_le = u32_be::from_native(0xdeadbeef).into();
/// assert!(deadbeef == 0xdeadbeef);
/// ```
#[derive(Copy, Clone, Debug, Hash)]
#[repr(transparent)]
pub struct LittleEndian<T>(T);

/// # Big endian types
/// ## Example
/// ```
/// use endian_type::*;
/// // endian types can be created from native types;
/// let deadbeef = u32_be::from_native(0xdeadbeef);
/// assert!(deadbeef == 0xdeadbeef);
/// // or use `From` trait of native types;
/// let deadbeef: u32_be = 0xdeadbeef.into();
/// assert!(deadbeef == 0xdeadbeef);
/// // or use `From` trait of another endian types;
/// let deadbeef: u32_be = u32_le::from_native(0xdeadbeef).into();
/// assert!(deadbeef == 0xdeadbeef);
/// ```
#[derive(Copy, Clone, Debug, Hash)]
#[repr(transparent)]
pub struct BigEndian<T>(T);

macro_rules! impl_endian {
    ($type_name:ident) => {
        impl_endian_base!($type_name);
        impl_endian_from_native!($type_name, LittleEndian);
        impl_endian_from_native!($type_name, BigEndian);
        impl_endian_from_each!($type_name);
        impl_endian_op!($type_name, BitAnd, bitand);
        impl_endian_op!($type_name, BitOr, bitor);
        impl_endian_op!($type_name, BitXor, bitxor);
        impl_endian_op!($type_name, Add, add);
        impl_endian_op!($type_name, Sub, sub);
        impl_endian_cmp!($type_name, PartialEq, eq, bool);
        impl_endian_cmp!($type_name, PartialOrd, partial_cmp, Option<Ordering>);
    };
}
macro_rules! impl_endian_base {
    ($type_name: ident) => {
        impl BigEndian<$type_name> {
            pub const fn from_native(data: $type_name) -> Self {
                Self(data.to_be())
            }

            pub const fn new(data: $type_name) -> Self {
                Self(data)
            }

            pub fn to_native(&self) -> $type_name {
                match () {
                    #[cfg(target_endian = "big")]
                    () => self.0,
                    #[cfg(target_endian = "little")]
                    () => self.0.swap_bytes(),
                }
            }
        }

        impl LittleEndian<$type_name> {
            pub const fn from_native(data: $type_name) -> Self {
                Self(data.to_le())
            }
            pub const fn new(data: $type_name) -> Self {
                Self(data)
            }

            pub fn to_native(&self) -> $type_name {
                match () {
                    #[cfg(target_endian = "big")]
                    () => self.0.swap_bytes(),
                    #[cfg(target_endian = "little")]
                    () => self.0,
                }
            }
        }
    };
}

macro_rules! impl_endian_from_native {
    ($type_name: ident, $endian_name: ident) => {
        impl From<$endian_name<$type_name>> for $type_name {
            #[inline]
            fn from(data: $endian_name<$type_name>) -> Self {
                data.to_native()
            }
        }

        impl From<$type_name> for $endian_name<$type_name> {
            #[inline]
            fn from(data: $type_name) -> Self {
                $endian_name::<$type_name>::from_native(data)
            }
        }
    };
}

macro_rules! impl_endian_from_each {
    ($type_name: ident) => {
        impl From<LittleEndian<$type_name>> for BigEndian<$type_name> {
            #[inline]
            fn from(data: LittleEndian<$type_name>) -> Self {
                Self(data.0.swap_bytes())
            }
        }

        impl From<BigEndian<$type_name>> for LittleEndian<$type_name> {
            #[inline]
            fn from(data: BigEndian<$type_name>) -> Self {
                Self(data.0.swap_bytes())
            }
        }
    };
}

macro_rules! impl_endian_cmp_each {
    ($type_name: ident, $endian_name: ident, $other_endian_name: ident, $trait_name: ident, $trait_func_name: ident, $return_type: ty) => {
        impl $trait_name<$other_endian_name<$type_name>> for $endian_name<$type_name> {
            #[inline]
            fn $trait_func_name(&self, rhs: &$other_endian_name<$type_name>) -> $return_type {
                self.to_native().$trait_func_name(&rhs.to_native())
            }
        }
    };
}

macro_rules! impl_endian_cmp_native {
    ($type_name: ident, $endian_name: ident, $trait_name: ident, $trait_func_name: ident, $return_type: ty) => {
        impl $trait_name<$type_name> for $endian_name<$type_name> {
            #[inline]
            fn $trait_func_name(&self, rhs: &$type_name) -> $return_type {
                self.to_native().$trait_func_name(rhs)
            }
        }
        impl $trait_name<$endian_name<$type_name>> for $type_name {
            #[inline]
            fn $trait_func_name(&self, rhs: &$endian_name<$type_name>) -> $return_type {
                self.$trait_func_name(&rhs.to_native())
            }
        }
    };
}

macro_rules! impl_endian_op_each {
    ($type_name: ident, $endian_name: ident, $other_endian_name: ident, $trait_name: ident, $trait_func_name: ident) => {
        impl $trait_name<$other_endian_name<$type_name>> for $endian_name<$type_name> {
            type Output = $endian_name<$type_name>;
            #[inline]
            fn $trait_func_name(self, rhs: $other_endian_name<$type_name>) -> Self {
                $endian_name::<$type_name>::from_native(
                    self.to_native().$trait_func_name(rhs.to_native()),
                )
            }
        }
    };
}

macro_rules! impl_endian_op_native {
    ($type_name: ident, $endian_name: ident, $trait_name: ident, $trait_func_name: ident) => {
        impl $trait_name<$type_name> for $endian_name<$type_name> {
            type Output = $endian_name<$type_name>;
            #[inline]
            fn $trait_func_name(self, rhs: $type_name) -> Self {
                $endian_name::<$type_name>::from_native(self.to_native().$trait_func_name(rhs))
            }
        }
        impl $trait_name<$endian_name<$type_name>> for $type_name {
            type Output = $type_name;
            #[inline]
            fn $trait_func_name(self, rhs: $endian_name<$type_name>) -> Self {
                self.$trait_func_name(rhs.to_native())
            }
        }
    };
}

macro_rules! impl_endian_op {
    ($type_name: ident, $trait_name: ident, $trait_func_name: ident) => {
        impl_endian_op_each!(
            $type_name,
            BigEndian,
            BigEndian,
            $trait_name,
            $trait_func_name
        );
        impl_endian_op_each!(
            $type_name,
            LittleEndian,
            LittleEndian,
            $trait_name,
            $trait_func_name
        );
        impl_endian_op_each!(
            $type_name,
            BigEndian,
            LittleEndian,
            $trait_name,
            $trait_func_name
        );
        impl_endian_op_each!(
            $type_name,
            LittleEndian,
            BigEndian,
            $trait_name,
            $trait_func_name
        );
        impl_endian_op_native!($type_name, BigEndian, $trait_name, $trait_func_name);
        impl_endian_op_native!($type_name, LittleEndian, $trait_name, $trait_func_name);
    };
}

macro_rules! impl_endian_cmp {
    ($type_name: ident, $trait_name: ident, $trait_func_name: ident, $return_type: ty) => {
        impl_endian_cmp_each!(
            $type_name,
            BigEndian,
            LittleEndian,
            $trait_name,
            $trait_func_name,
            $return_type
        );
        impl_endian_cmp_each!(
            $type_name,
            LittleEndian,
            BigEndian,
            $trait_name,
            $trait_func_name,
            $return_type
        );
        impl_endian_cmp_each!(
            $type_name,
            LittleEndian,
            LittleEndian,
            $trait_name,
            $trait_func_name,
            $return_type
        );
        impl_endian_cmp_each!(
            $type_name,
            BigEndian,
            BigEndian,
            $trait_name,
            $trait_func_name,
            $return_type
        );
        impl_endian_cmp_native!(
            $type_name,
            BigEndian,
            $trait_name,
            $trait_func_name,
            $return_type
        );
        impl_endian_cmp_native!(
            $type_name,
            LittleEndian,
            $trait_name,
            $trait_func_name,
            $return_type
        );
    };
}

impl_endian!(u8);
impl_endian!(u16);
impl_endian!(u32);
impl_endian!(u64);
impl_endian!(u128);
impl_endian!(usize);
impl_endian!(i8);
impl_endian!(i16);
impl_endian!(i32);
impl_endian!(i64);
impl_endian!(i128);
impl_endian!(isize);

#[allow(non_camel_case_types)]
pub mod types {
    pub type u8_le = super::LittleEndian<u8>;
    pub type u16_le = super::LittleEndian<u16>;
    pub type u32_le = super::LittleEndian<u32>;
    pub type u64_le = super::LittleEndian<u64>;
    pub type u128_le = super::LittleEndian<u128>;
    pub type usize_le = super::LittleEndian<usize>;
    pub type i8_le = super::LittleEndian<i8>;
    pub type i16_le = super::LittleEndian<i16>;
    pub type i32_le = super::LittleEndian<i32>;
    pub type i64_le = super::LittleEndian<i64>;
    pub type i128_le = super::LittleEndian<i128>;
    pub type isize_le = super::LittleEndian<isize>;
    pub type u8_be = super::BigEndian<u8>;
    pub type u16_be = super::BigEndian<u16>;
    pub type u32_be = super::BigEndian<u32>;
    pub type u64_be = super::BigEndian<u64>;
    pub type u128_be = super::BigEndian<u128>;
    pub type usize_be = super::BigEndian<usize>;
    pub type i8_be = super::BigEndian<i8>;
    pub type i16_be = super::BigEndian<i16>;
    pub type i32_be = super::BigEndian<i32>;
    pub type i64_be = super::BigEndian<i64>;
    pub type i128_be = super::BigEndian<i128>;
    pub type isize_be = super::BigEndian<isize>;
}
