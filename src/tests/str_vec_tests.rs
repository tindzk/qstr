#![allow(clippy::unusual_byte_groupings)]
use core::mem;

use crate::{ExceedsCapacity, StrVec, StrVec28, StrVec56, StrVec112};

#[test]
fn test_type() {
  let _ = StrVec::<u16, 16>::new();
}

#[test]
fn test_len() {
  let mut vec = StrVec28::new();
  assert_eq!(vec.len(), 0);

  vec.push("abc").unwrap();
  assert_eq!(vec.len(), 1);

  vec.push("def").unwrap();
  assert_eq!(vec.len(), 2);
}

#[test]
fn test_get() {
  let mut v = StrVec28::new();
  v.push("ab").unwrap();
  v.push("cdefg").unwrap();

  assert_eq!(v.get(0), Some("ab"));
  assert_eq!(v.get(1), Some("cdefg"));
}

#[test]
fn test_push_empty() {
  let mut vec = StrVec28::new();
  vec.push("").unwrap();

  assert_eq!(vec.len(), 1);
  assert_eq!(vec.get(0), Some(""));

  // Note: The bitmap has 32 bits, but only the first 28 are in use
  assert_eq!(vec.bitmap, 0b1000000000000000000000000000_0000);
  assert_eq!(vec.data, [0; 28]);
}

#[test]
fn test_fill() {
  let mut vec = StrVec28::new();
  for _ in 0..28 {
    vec.push("").unwrap();
  }

  assert_eq!(vec.len(), 28);
  assert_eq!(vec.get(27), Some(""));

  assert_eq!(vec.bitmap, 0b1111111111111111111111111111_0000);
  assert_eq!(vec.data, [0; 28]);

  assert_eq!(
    vec.push(""),
    Err(ExceedsCapacity {
      length: 29,
      capacity: 28
    })
  );
}

#[test]
fn test_capacity() {
  assert_eq!(
    StrVec56::new().push("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
    Err(ExceedsCapacity {
      length: 57,
      capacity: 56
    })
  );
}

#[test]
fn test_push_short() {
  let mut vec = StrVec28::new();

  vec.push("a").unwrap();
  vec.push("").unwrap();
  vec.push("b").unwrap();

  assert_eq!(vec.len(), 3);

  assert_eq!(vec.get(0), Some("a"));
  assert_eq!(vec.get(1), Some(""));
  assert_eq!(vec.get(2), Some("b"));
  assert_eq!(vec.get(3), None);

  assert_eq!(vec.bitmap, 0b1110000000000000000000000000_0000);
  assert_eq!(
    &vec.data,
    b"a\0b\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"
  );
}

#[test]
fn test_push_long() {
  let mut vec = StrVec56::new();

  // Fill 54 characters
  vec.push("aaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
  vec.push("aaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
  vec.push("aaaaaa").unwrap();

  assert_eq!(vec.len(), 3);

  assert_eq!(vec.get(0), Some("aaaaaaaaaaaaaaaaaaaaaaaa"));
  assert_eq!(vec.get(1), Some("aaaaaaaaaaaaaaaaaaaaaaaa"));
  assert_eq!(vec.get(2), Some("aaaaaa"));
  assert_eq!(vec.get(3), None);

  // Note: The bitmap's last 10 bits are unused
  assert_eq!(
    vec.bitmap,
    0b000000000000000000000001000000000000000000000001000001_0000000000
  );

  assert_eq!(
    &vec.data,
    b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\0\0"
  );
}

#[test]
fn test_push_nul() {
  let mut vec = StrVec28::new();

  vec.push("\0\0\0").unwrap();
  vec.push("\0\0").unwrap();
  vec.push("\0").unwrap();

  assert_eq!(vec.get(0).unwrap(), "\0\0\0");
  assert_eq!(vec.get(1).unwrap(), "\0\0");

  // Cannot determine whether NUL or empty string
  assert_eq!(vec.get(2).unwrap(), "");

  assert_eq!(vec.bitmap, 0b0010110000000000000000000000_0000);
  assert_eq!(vec.data, [0; 28]);
}

#[test]
fn test_size() {
  assert_eq!(mem::size_of::<StrVec28>(), 32);
  assert_eq!(mem::size_of::<StrVec56>(), 64);
  assert_eq!(mem::size_of::<StrVec112>(), 128);
}

#[test]
fn test_next_offset() {
  let mut v = StrVec28::new();
  assert_eq!(v.next_offset(), 0);

  v.push("a").unwrap();
  assert_eq!(v.next_offset(), 1);

  v.push("ab").unwrap();
  assert_eq!(v.next_offset(), 1 + 2);

  v.push("abc").unwrap();
  assert_eq!(v.next_offset(), 1 + 2 + 3);
}

#[cfg(feature = "std")]
mod std {
  use std::collections::HashSet;
  use std::format;
  use std::vec;
  use std::{collections::BTreeSet, vec::Vec};

  use crate::{StrVec28, StrVec56, StrVec112};

  #[test]
  fn test_hash() {
    let mut s1 = StrVec112::new();
    s1.push("ab").unwrap();

    let mut s2 = StrVec112::new();
    s2.push("ab").unwrap();
    s2.push("cdefg").unwrap();

    let mut set = HashSet::new();
    set.insert(s1);
    set.insert(s2);

    assert_eq!(set.len(), 2);
  }

  #[test]
  fn test_ord() {
    let s1 = StrVec112::try_from(["ab", "a1"]).unwrap();
    let s2 = StrVec112::try_from(["ab", "a2"]).unwrap();
    let s3 = StrVec112::try_from(["ab", "a3"]).unwrap();

    let mut set = BTreeSet::new();
    set.insert(s3);
    set.insert(s2);
    set.insert(s1);

    assert_eq!(set.iter().collect::<Vec<_>>(), vec![&s1, &s2, &s3]);
  }

  #[test]
  fn test_iter() {
    let mut vec = StrVec56::new();

    vec.push("hello").unwrap();
    vec.push("world").unwrap();

    let strings: Vec<&str> = vec.iter().collect();
    assert_eq!(strings, ["hello", "world"]);
  }

  #[test]
  fn test_capacity() {
    let mut vec = StrVec56::new();

    let insert = (0..56)
      .map(|v| format!("{}", v % 10)) // Limit to 1 character
      .collect::<Vec<_>>();

    for s in &insert {
      vec.push(s).unwrap();
    }

    assert_eq!(vec.len(), 56);

    assert!(vec.push("0").is_err());
    assert!(vec.push("").is_err());

    for (i, s) in insert.iter().enumerate() {
      assert_eq!(vec.get(i), Some(s as &str));
    }

    let collected = vec.iter().collect::<Vec<_>>();
    assert_eq!(collected.len(), 56);
    assert_eq!(collected, insert);
  }

  #[test]
  fn test_debug() {
    let v = StrVec28::try_from(["a", "b", "c"]).unwrap();
    assert_eq!(format!("{:?}", v), r#"["a", "b", "c"]"#);
  }
}

#[cfg(feature = "serde")]
mod serde_tests {
  use serde_json::Value;
  use std::{string::ToString, vec};

  use crate::StrVec56;

  #[test]
  fn test_deserialise_borrowed_string() {
    let json = r#"["admin"]"#;
    let result = serde_json::from_str::<StrVec56>(json).unwrap();
    assert_eq!(result, StrVec56::try_from(["admin"]).unwrap());
  }

  #[test]
  fn test_deserialise_owned_string() {
    let json = Value::Array(vec![Value::String("admin".to_string())]);
    let result = serde_json::from_value::<StrVec56>(json).unwrap();
    assert_eq!(result, StrVec56::try_from(["admin"]).unwrap());
  }
}
