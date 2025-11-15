use core::fmt;

#[cfg(feature = "std")]
use std::string::String;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::alignment_resolver::{AlignmentForLength, AlignmentMarker, AlignmentType};
use crate::bitmap_resolver::{BitmapForLength, BitmapMarker, BitmapType};
use crate::errors::ExceedsCapacity;
use crate::str_vec::StrVec;

#[cfg(doc)]
use crate::BStr7;
#[cfg(doc)]
use crate::BStr15;
#[cfg(doc)]
use crate::BStr31;
#[cfg(doc)]
use crate::BStr63;
#[cfg(doc)]
use crate::BStr127;
#[cfg(doc)]
use crate::FStr64;

/// Bounded stack-allocated string
///
/// A BoundedStr is a variable-length string with a fixed capacity N.
///
/// # Internal structure
/// The length is stored in the first byte, and the remaining bytes contain the
/// string content.
///
/// # Caveat
/// While [FStr64] fits into a cache line on most CPUs, a 64-byte [BoundedStr]
/// would not. Since BoundedStr reserves the first byte for the length, only
/// [BStr63] exists, but not BStr64. To encode a variable-length string with a
/// capacity of 64 bytes, the next larger size can be used, i.e. [BStr127].
///
/// # Aliases
/// To avoid unnecessary cache-line straddling, aliases are provided with
/// capacities of `2ᴺ - 1` for `N ∈ [3, 7]`.
///
/// See also: [BStr7], [BStr15], [BStr31], [BStr63], [BStr127]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BoundedStr<const N: usize, Alignment> {
  length: u8,
  data: [u8; N],
  align: [Alignment; 0],
}

impl<const N: usize, Alignment> Default for BoundedStr<N, Alignment> {
  fn default() -> Self {
    Self::new()
  }
}

impl<const N: usize, Alignment> BoundedStr<N, Alignment> {
  /// Create an empty BoundedStr
  #[inline]
  pub const fn new() -> Self {
    Self {
      length: 0,
      data: [0; N],
      align: [],
    }
  }

  /// Builds a BoundedStr in a const context
  ///
  /// # Safety
  /// This will panic if the string exceeds the capacity.
  pub const fn const_from(src: &str) -> Self {
    let bytes = src.as_bytes();
    let length = bytes.len();

    if length > N {
      panic!("String length exceeds capacity");
    }

    let mut data = [0u8; N];

    {
      let (left, _) = data.split_at_mut(length);
      left.copy_from_slice(bytes);
    }

    BoundedStr {
      length: length as u8,
      data,
      align: [],
    }
  }

  /// Attempt to construct BoundedStr in a const context
  #[inline]
  pub const fn const_try_from(src: &str) -> Option<Self> {
    let bytes = src.as_bytes();
    let length = bytes.len();

    if length > N {
      None
    } else {
      let mut data = [0u8; N];

      {
        let (left, _) = data.split_at_mut(length);
        left.copy_from_slice(bytes);
      }

      Some(BoundedStr {
        length: length as u8,
        data,
        align: [],
      })
    }
  }

  /// Attempt to construct BoundedStr
  #[inline]
  pub fn try_from(s: &str) -> Result<Self, ExceedsCapacity> {
    let bytes = s.as_bytes();
    let length = bytes.len();

    if length > N {
      return Err(ExceedsCapacity {
        length,
        capacity: N,
      });
    }

    let mut data = [0u8; N];
    data[0..length].copy_from_slice(bytes);

    Ok(BoundedStr {
      length: length as u8,
      data,
      align: [],
    })
  }

  /// Returns string length
  #[inline]
  pub fn len(&self) -> usize {
    self.length as usize
  }

  /// Checks if the BoundedStr is empty
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  /// Appends a string slice to the BoundedStr
  ///
  /// Returns `Err` if there is not enough capacity.
  #[inline]
  pub fn push_str(&mut self, s: &str) -> Result<(), ExceedsCapacity> {
    let bytes = s.as_bytes();
    let length = self.length as usize;
    let new_len = length + bytes.len();

    if new_len > N {
      return Err(ExceedsCapacity {
        length: new_len,
        capacity: N,
      });
    }

    self.data[length..new_len].copy_from_slice(bytes);
    self.length = new_len as u8;

    Ok(())
  }

  /// Appends a single character to the BoundedStr
  ///
  /// Returns `Err` if there is not enough capacity.
  #[inline]
  pub fn push(&mut self, c: char) -> Result<(), ExceedsCapacity> {
    let mut buf = [0u8; 4];
    let s = c.encode_utf8(&mut buf);
    self.push_str(s)
  }

  /// Convert BoundedStr to `&str`
  #[inline]
  pub fn as_str(&self) -> &str {
    // SAFETY: Since `data` is private and objects are only constructed in new() from
    //         valid &str values, from_utf8_unchecked() is safe to be used here
    unsafe { core::str::from_utf8_unchecked(&self.data[..self.length as usize]) }
  }

  /// Splits BoundedStr by delimiter
  ///
  /// # Note
  /// This function is only available for common N values (7, 15, 31 etc.) since
  /// the corresponding bitmap size for StrVec is resolved at compile time using a
  /// type-level mapping.
  pub fn split(&self, delimiter: &str) -> StrVec<BitmapType<N>, N, AlignmentType<N>>
  where
    BitmapMarker: BitmapForLength<N>,
    AlignmentMarker: AlignmentForLength<N>,
  {
    let s = self.as_str();

    let mut result = StrVec::new();
    let mut start = 0;
    let mut i = 0;

    while i < self.length as usize {
      if &self.data[i..i + delimiter.len()] == delimiter.as_bytes() {
        result.push(&s[start..i]).unwrap();
        start = i + delimiter.len();
        i += delimiter.len();
      } else {
        i += 1;
      }
    }

    if start <= self.length as usize {
      result.push(&s[start..]).unwrap();
    }

    result
  }
}

impl<const N: usize, Alignment> fmt::Display for BoundedStr<N, Alignment> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

impl<const N: usize, Alignment> fmt::Debug for BoundedStr<N, Alignment> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

impl<const N: usize, Alignment> From<&str> for BoundedStr<N, Alignment> {
  fn from(s: &str) -> Self {
    Self::try_from(s).unwrap()
  }
}

#[cfg(feature = "std")]
impl<const N: usize, Alignment> From<&String> for BoundedStr<N, Alignment> {
  fn from(s: &String) -> Self {
    Self::try_from(s).unwrap()
  }
}

#[cfg(feature = "std")]
impl<const N: usize, Alignment> From<String> for BoundedStr<N, Alignment> {
  fn from(s: String) -> Self {
    Self::try_from(&s).unwrap()
  }
}

#[cfg(feature = "serde")]
impl<const N: usize, Alignment> Serialize for BoundedStr<N, Alignment> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    self.as_str().serialize(serializer)
  }
}

#[cfg(feature = "serde")]
impl<'de, const N: usize, Alignment> Deserialize<'de> for BoundedStr<N, Alignment> {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;
    BoundedStr::try_from(&s).map_err(serde::de::Error::custom)
  }
}
