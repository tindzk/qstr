//! Cache-efficient, stack-allocated string types.
//!
//! # Overview
//! qstr provides fixed-capacity string types optimised for locality and cache
//! efficiency. String data is stored on the stack, avoiding heap allocations
//! and pointer indirection. These types are suitable for embedded environments,
//! WebAssembly, parsers and other peformance-sensitive contexts.
//!
//! ## Available types
//! - Variable-length strings with fixed capacity ([BoundedStr])
//! - Fixed-length strings ([FixedStr])
//! - Fixed-capacity string vectors ([StrVec])
//!
//! ## Feature flags
//! - `std` (default): Disable for `no_std` compatibility
//! - `serde`: Support for serialisation/deserialisation with serde
//!
//! ## Minimum Supported Rust Version (MSRV)
//! Rust v1.87+ is required due to the use of [slice::copy_from_slice].
//!
//! # Example
//! ```rust
//! use qstr::BStr15;
//! use qstr::StrVec;
//! use qstr::Align16;
//!
//! let str: BStr15 = "aws:us:east:1".into();
//! let vec: StrVec<u16, 15, Align16> = str.split(":");
//!
//! assert_eq!(
//!   vec.iter().collect::<Vec<_>>(),
//!   vec!["aws", "us", "east", "1"]
//! );
//! ```
//!
//! # Aliases
//! Aliases are provided for common `N` byte sizes:
//! - `BStrN` types are aliases for `BoundedStr<N>`
//! - `FStrN` types are aliases for `FixedStr<N>`
//! - `StrVecN` types are aliases for `StrVec<Bitmap(N), N>`
//!
//! `N` always denotes the total number of storable characters rather than the
//! total `struct` size. The sizes were chosen with cache efficiency in mind
//! such that most values will fit into a single cache line.
//!
//! # Copy semantics
//! Unlike `String` and `Vec<String>`, all qstr reside fully on the stack and
//! therefore implement [Copy]. They can be passed by value or returned from
//! functions without cloning.
//!
//! # Safety
//! `unsafe` is required internally only for [str::from_utf8_unchecked] calls.
//! The correct usage is enforced at compile time by keeping the data buffers
//! private and marking [FixedStr::from_bytes] as `unsafe`.

#![no_std]
#![deny(missing_docs)]

#[cfg(feature = "std")]
extern crate std;

mod alignment;
mod alignment_resolver;
mod bitmap;
mod bitmap_resolver;
mod bounded_str;
mod errors;
mod fixed_str;
mod str_vec;

pub use errors::ExceedsCapacity;

pub use bounded_str::BoundedStr;

pub use alignment::Align8;
pub use alignment::Align16;
pub use alignment::Align32;
pub use alignment::Align64;
pub use alignment::Align128;

/// Variable-length string with a maximum capacity of 7 characters
///
/// Occupies 8 bytes
pub type BStr7 = BoundedStr<7, Align8>;

/// Variable-length string with a maximum capacity of 15 characters
///
/// Occupies 16 bytes
pub type BStr15 = BoundedStr<15, Align16>;

/// Variable-length string with a maximum capacity of 31 characters
///
/// Occupies 32 bytes
pub type BStr31 = BoundedStr<31, Align32>;

/// Variable-length string with a maximum capacity of 63 characters
///
/// Occupies 64 bytes
pub type BStr63 = BoundedStr<63, Align64>;

/// Variable-length string with a maximum capacity of 127 characters
///
/// Occupies 128 bytes
pub type BStr127 = BoundedStr<127, Align128>;

pub use fixed_str::FixedStr;

/// Fixed-length string with a capacity of 8 characters
///
/// Occupies 8 bytes
pub type FStr8 = FixedStr<8, Align8>;

/// Fixed-length string with a capacity of 16 characters
///
/// Occupies 16 bytes
pub type FStr16 = FixedStr<16, Align16>;

/// Fixed-length string with a capacity of 24 characters
///
/// Occupies 24 bytes
pub type FStr24 = FixedStr<24, Align8>;

/// Fixed-length string with a capacity of 32 characters
///
/// Occupies 32 bytes
pub type FStr32 = FixedStr<32, Align32>;

/// Fixed-length string with a capacity of 64 characters
///
/// Occupies 64 bytes
pub type FStr64 = FixedStr<64, Align64>;

/// Fixed-length string with a capacity of 128 characters
///
/// Occupies 128 bytes
pub type FStr128 = FixedStr<128, Align128>;

pub use str_vec::StrVec;

/// String vector supporting up to 28 items, with a combined capacity of 28
/// characters
///
/// Occupies 4 bytes (bitmap) + 28 bytes (data) = 32 bytes total
///
/// Two StrVec28 values will fit into a single cache line
pub type StrVec28 = StrVec<u32, 28, Align32>;

/// String vector supporting up to 56 items, with a combined capacity of 56
/// characters
///
/// Occupies 8 bytes (bitmap) + 56 bytes (data) = 64 bytes total
///
/// Fills a single cache line
pub type StrVec56 = StrVec<u64, 56, Align64>;

/// String vector supporting up to 112 items, with a combined capacity of 112
/// characters
///
/// Occupies 16 bytes (bitmap) + 112 bytes (data) = 128 bytes total
///
/// Fills two cache lines
pub type StrVec112 = StrVec<u128, 112, Align128>;

#[cfg(test)]
mod tests {
  mod bounded_str_tests;
  mod fixed_str_tests;
  mod str_vec_tests;
}
