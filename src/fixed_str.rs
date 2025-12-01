use core::fmt;

#[cfg(feature = "std")]
use std::string::String;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(doc)]
use crate::BStr63;
#[cfg(doc)]
use crate::BStr127;
#[cfg(doc)]
use crate::BoundedStr;
#[cfg(doc)]
use crate::FStr8;
#[cfg(doc)]
use crate::FStr16;
#[cfg(doc)]
use crate::FStr24;
#[cfg(doc)]
use crate::FStr32;
#[cfg(doc)]
use crate::FStr64;
#[cfg(doc)]
use crate::FStr128;

use crate::ExceedsCapacity;

/// Fixed stack-allocated string
///
/// The characters are stored within FixedStr's internal buffer. It is suitable
/// for any fixed-length data having a valid UTF-8 representation. Common uses
/// for FixedStr include identifiers and hashes.
///
/// Unlike [BoundedStr], the length is not encoded and is assumed to match the
/// capacity `N`. However, if the string is shorter than `N`, the remaining
/// bytes will be NUL-padded.
///
/// # Aliases
/// See also: [FStr8], [FStr16], [FStr24], [FStr32], [FStr64], [FStr128]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FixedStr<const N: usize, Alignment> {
  data: [u8; N],
  align: [Alignment; 0],
}

impl<const N: usize, Alignment> FixedStr<N, Alignment> {
  /// Creates a NUL-padded FixedStr. Equivalent to FixedStr::default().
  #[inline]
  pub const fn new() -> Self {
    FixedStr {
      data: [0; N],
      align: [],
    }
  }

  /// # Safety
  /// This function requires that the provided bytes can be represented by a UTF-8 string.
  /// Otherwise, [Self::as_str] and [Self::as_str_trimmed] are not well-defined.
  #[inline]
  pub const unsafe fn from_bytes(data: [u8; N]) -> Self {
    FixedStr { data, align: [] }
  }

  /// It is possible to construct a FixedStr shorter than its capacity, in which
  /// case the missing bytes will be filled with NULs.
  #[inline]
  pub fn try_from(s: &str) -> Result<Self, ExceedsCapacity> {
    let length = s.len();

    if length > N {
      return Err(ExceedsCapacity {
        length,
        capacity: N,
      });
    }

    let mut data = [0u8; N];
    let bytes = s.as_bytes();
    data[0..length].copy_from_slice(bytes);

    Ok(FixedStr { data, align: [] })
  }

  /// Builds FixedStr within a const context
  pub const fn const_from(s: &str) -> Self {
    let length = s.len();

    if length > N {
      panic!("String length exceeds capacity");
    }

    let bytes = s.as_bytes();

    let mut t = FixedStr {
      data: [0; N],
      align: [],
    };

    {
      let (left, _) = t.data.split_at_mut(length);
      left.copy_from_slice(bytes);
    }

    t
  }

  /// Attempts to build FixedStr within a const context
  #[inline]
  pub const fn const_try_from(s: &str) -> Option<Self> {
    let length = s.len();

    if length > N {
      None
    } else {
      let bytes = s.as_bytes();

      let mut t = FixedStr {
        data: [0; N],
        align: [],
      };

      {
        let (left, _) = t.data.split_at_mut(length);
        left.copy_from_slice(bytes);
      }

      Some(t)
    }
  }

  /// Returns underlying byte buffer
  #[inline]
  pub fn as_bytes(&self) -> &[u8; N] {
    &self.data
  }

  /// Converts FixedStr to `&str`
  #[inline]
  pub fn as_str(&self) -> &str {
    unsafe { core::str::from_utf8_unchecked(&self.data) }
  }

  /// Converts FixedStr to a &str. If the value is NUL-padded or contains NULs,
  /// this stops at the first NUL byte.
  #[inline]
  pub fn as_str_trimmed(&self) -> &str {
    let length = self
      .data
      .iter()
      .position(|&b| b == 0)
      .unwrap_or(self.data.len());

    unsafe { core::str::from_utf8_unchecked(&self.data[..length]) }
  }

  // Note: to_string() is provided via fmt::Display

  /// Converts FixedStr to a String. If the value is NUL-padded or contains
  /// NULs, this stops at the first NUL byte.
  #[cfg(feature = "std")]
  pub fn to_string_trimmed(&self) -> String {
    String::from(self.as_str_trimmed())
  }
}

impl<const N: usize, Alignment> Default for FixedStr<N, Alignment> {
  fn default() -> Self {
    FixedStr::new()
  }
}

impl<const N: usize, Alignment> fmt::Display for FixedStr<N, Alignment> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

impl<const N: usize, Alignment> fmt::Debug for FixedStr<N, Alignment> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str_trimmed())
  }
}

impl<const N: usize, Alignment> From<&str> for FixedStr<N, Alignment> {
  #[track_caller]
  fn from(s: &str) -> Self {
    Self::try_from(s).unwrap()
  }
}

#[cfg(feature = "std")]
impl<const N: usize, Alignment> From<&String> for FixedStr<N, Alignment> {
  #[track_caller]
  fn from(s: &String) -> Self {
    Self::try_from(s).unwrap()
  }
}

#[cfg(feature = "std")]
impl<const N: usize, Alignment> From<String> for FixedStr<N, Alignment> {
  #[track_caller]
  fn from(s: String) -> Self {
    Self::try_from(&s).unwrap()
  }
}

#[cfg(feature = "serde")]
impl<const N: usize, Alignment> Serialize for FixedStr<N, Alignment> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    self.as_str().serialize(serializer)
  }
}

#[cfg(feature = "serde")]
impl<'de, const N: usize, Alignment> Deserialize<'de> for FixedStr<N, Alignment> {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    let v = String::deserialize(deserializer)?;
    FixedStr::try_from(&v).map_err(serde::de::Error::custom)
  }
}
