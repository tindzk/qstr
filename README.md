# qstr
[![Crates.io](https://img.shields.io/crates/v/qstr.svg)](https://crates.io/crates/qstr)
[![Docs.rs](https://img.shields.io/docsrs/qstr)](https://docs.rs/qstr)
[![Licence](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Build Status](https://img.shields.io/github/actions/workflow/status/tindzk/qstr/tests)](https://github.com/tindzk/qstr/actions)
[![no_std](https://img.shields.io/badge/no__std-%E2%9C%94-blue)](#)
[![Minimum Rust Version](https://img.shields.io/badge/rustc-1.87+-orange.svg)](#)

qstr is a small, `no_std` Rust library providing cache-efficient, stack-allocated string types.

It is suitable for embedded environments, WebAssembly, parsers and other peformance-sensitive contexts.

## Motivation
In many cases, string lengths are bounded and their maximum size is known in advance. Storing such strings on the stack eliminates allocator overhead and improves cache locality.

Because qstr types implement `Copy`, they can be passed by value without cloning.

This library provides types for common sizes, optimised for cache-line efficiency.

## Features
- Stack-allocated string types
  - Variable-length strings with fixed capacity
  - Fixed-length strings
  - Fixed-capacity string vectors
- All types implement `Copy`
- Usable in `const` contexts
- Optional `serde` support
- `no_std` compatible
- Zero dependencies

## Example
```rust
use qstr::BStr15;
use qstr::StrVec;

let str: BStr15 = "aws:us:east:1".into();
let vec: StrVec<u16, 15> = str.split(":");

assert_eq!(
  vec.iter().collect::<Vec<_>>(),
  vec!["aws", "us", "east", "1"]
);
```

# Licence
qstr is licensed under the terms of the Apache License, Version 2.0.
