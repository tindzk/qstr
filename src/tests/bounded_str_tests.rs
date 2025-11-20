use core::mem;

use crate::BStr7;
use crate::BStr15;
use crate::BStr31;
use crate::BStr63;
use crate::BStr127;
use crate::ExceedsCapacity;

#[test]
fn test_size() {
  assert_eq!(mem::size_of::<BStr7>(), 8);
  assert_eq!(mem::size_of::<BStr15>(), 16);
  assert_eq!(mem::size_of::<BStr31>(), 32);
  assert_eq!(mem::size_of::<BStr63>(), 64);
  assert_eq!(mem::size_of::<BStr127>(), 128);
}

#[test]
fn test_alignment() {
  assert_eq!(mem::align_of::<BStr7>(), 8);
  assert_eq!(mem::align_of::<BStr15>(), 16);
  assert_eq!(mem::align_of::<BStr31>(), 32);
  assert_eq!(mem::align_of::<BStr63>(), 64);
  assert_eq!(mem::align_of::<BStr127>(), 128);
}

#[test]
fn test_push_str() {
  let mut s = BStr15::new();
  s.push_str("abc").unwrap();
  s.push_str("def").unwrap();

  assert_eq!(s.len(), 6);
  assert_eq!(s.as_str(), "abcdef");
}

#[test]
fn test_push_str_exceeds_capacity() {
  let mut s = BStr7::new();

  assert_eq!(
    s.push_str("00000000"),
    Err(ExceedsCapacity {
      length: 8,
      capacity: 7
    })
  );
}

#[test]
fn test_push() {
  let mut s = BStr15::new();
  s.push('a').unwrap();
  s.push('b').unwrap();
  s.push('c').unwrap();

  assert_eq!(s.len(), 3);
  assert_eq!(s.as_str(), "abc");
}

const fn f() -> BStr15 {
  BStr15::const_try_from("abc").unwrap()
}

#[test]
fn test_const_try_from() {
  assert_eq!(f(), BStr15::try_from("abc").unwrap());
}

#[test]
fn test_into() {
  let _v: BStr7 = "asdf".into();
}

#[cfg(feature = "std")]
mod std {
  use std::format;
  use std::string::String;
  use std::vec;
  use std::vec::Vec;

  use crate::{Align8, Align64, BStr7, BStr63, ExceedsCapacity, StrVec};

  #[test]
  fn test_into_panic() {
    let Err(error) = std::panic::catch_unwind(|| {
      let _v: BStr7 = "000000000000".into();
    }) else {
      panic!()
    };

    assert_eq!(
      error.downcast_ref::<String>().unwrap(),
      r#"called `Result::unwrap()` on an `Err` value: String length (12) exceeds capacity (7)"#
    );
  }

  #[test]
  fn test_split() {
    let v: BStr7 = "a,b,c,d".into();
    let split: StrVec<u8, 7, Align8> = v.split(",");

    assert_eq!(split.iter().collect::<Vec<_>>(), vec!["a", "b", "c", "d"]);
  }

  #[test]
  fn test_split2() {
    let v: BStr7 = ",".into();
    assert_eq!(
      v.split(",").iter().collect::<Vec<_>>(),
      v.as_str().split(",").collect::<Vec<_>>()
    );
  }

  #[test]
  fn test_split3() {
    let v: BStr63 = "us:aws:east:1:worker0000000000000000000000000000000000000000000".into();
    let split: StrVec<u64, 63, Align64> = v.split(":");

    assert_eq!(
      split.iter().collect::<Vec<_>>(),
      vec![
        "us",
        "aws",
        "east",
        "1",
        "worker0000000000000000000000000000000000000000000"
      ]
    );
  }

  #[test]
  fn test_push_str_exceeds_capacity() {
    let mut s = BStr7::new();
    let exceeds_capacity = s.push_str("00000000").unwrap_err();

    assert_eq!(
      exceeds_capacity,
      ExceedsCapacity {
        length: 8,
        capacity: 7
      }
    );

    assert_eq!(
      format!("{}", exceeds_capacity),
      "String length (8) exceeds capacity (7)"
    );
  }
}

#[cfg(feature = "serde")]
mod serde_tests {
  use crate::BStr63;

  #[test]
  fn test_serialise() {
    let s =
      BStr63::try_from("000000000000000000000000000000000000000000000000000000000000000").unwrap();

    let json = serde_json::to_string(&s).unwrap();

    assert_eq!(
      json,
      r#""000000000000000000000000000000000000000000000000000000000000000""#
    );
  }

  #[test]
  fn test_deserialise() {
    let json = r#""000000000000000000000000000000000000000000000000000000000000000""#;
    let result = serde_json::from_str::<BStr63>(json).unwrap();
    assert_eq!(
      result,
      BStr63::try_from("000000000000000000000000000000000000000000000000000000000000000").unwrap()
    );
  }
}
