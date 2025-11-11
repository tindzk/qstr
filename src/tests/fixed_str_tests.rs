use core::mem;

use crate::{ExceedsCapacity, FStr8, FStr16, FStr24, FStr32, FStr64, FStr128};

#[test]
fn test_size() {
  assert_eq!(mem::size_of::<FStr8>(), 8);
  assert_eq!(mem::size_of::<FStr16>(), 16);
  assert_eq!(mem::size_of::<FStr24>(), 24);
  assert_eq!(mem::size_of::<FStr32>(), 32);
  assert_eq!(mem::size_of::<FStr64>(), 64);
  assert_eq!(mem::size_of::<FStr128>(), 128);
}

#[test]
fn test_alignment() {
  assert_eq!(mem::align_of::<FStr8>(), 8);
  assert_eq!(mem::align_of::<FStr16>(), 16);
  assert_eq!(mem::align_of::<FStr24>(), 8);
  assert_eq!(mem::align_of::<FStr32>(), 32);
  assert_eq!(mem::align_of::<FStr64>(), 64);
  assert_eq!(mem::align_of::<FStr128>(), 128);
}

const fn f() -> FStr24 {
  FStr24::const_try_from("abc").unwrap()
}

#[test]
fn test_const_try_from_zero_padded() {
  assert_eq!(f(), FStr24::try_from("abc").unwrap());
}

#[test]
fn test_as_str_zero_padded() {
  let str = FStr24::const_try_from("abc").unwrap();
  assert_eq!(str.as_str().len(), 24);
  assert_eq!(str.as_str_trimmed().len(), 3);
}

const fn g() -> FStr24 {
  FStr24::const_try_from("aaaaaaaaaaaaaaaaaaaaaaaa").unwrap()
}

#[test]
fn test_within_capacity() {
  // 24 bytes
  let result = g();
  let result2 = FStr24::try_from("aaaaaaaaaaaaaaaaaaaaaaaa").unwrap();

  assert_eq!(result.as_str(), "aaaaaaaaaaaaaaaaaaaaaaaa");
  assert_eq!(result2.as_str(), "aaaaaaaaaaaaaaaaaaaaaaaa");
  assert_eq!(result, result2);
}

const fn h() -> Option<FStr24> {
  FStr24::const_try_from("aaaaaaaaaaaaaaaaaaaaaaaaa")
}

#[test]
fn test_exceed_capacity() {
  // 25 bytes
  let result = h();
  let result2 = FStr24::try_from("aaaaaaaaaaaaaaaaaaaaaaaaa");

  assert_eq!(result, None);

  assert_eq!(
    result2,
    Err(ExceedsCapacity {
      length: 25,
      capacity: 24
    })
  );
}

#[cfg(feature = "std")]
mod std {
  use std::format;

  use crate::FStr32;

  #[test]
  fn test_debug() {
    let v = FStr32::try_from("abc").unwrap();
    assert_eq!(format!("{v:?}"), "abc");
  }
}

#[cfg(feature = "serde")]
mod serde_tests {
  use crate::FStr24;

  #[test]
  fn test_serialise() {
    let s = FStr24::try_from("aaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
    let json = serde_json::to_string(&s).unwrap();
    assert_eq!(json, r#""aaaaaaaaaaaaaaaaaaaaaaaa""#);
  }

  #[test]
  fn test_deserialise() {
    let json = r#""aaaaaaaaaaaaaaaaaaaaaaaa""#;
    let result = serde_json::from_str::<FStr24>(json).unwrap();
    assert_eq!(
      result,
      FStr24::try_from("aaaaaaaaaaaaaaaaaaaaaaaa").unwrap()
    );
  }
}
