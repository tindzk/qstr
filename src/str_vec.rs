use core::hash::{self, Hasher};

#[cfg(feature = "std")]
use std::fmt;

#[cfg(feature = "std")]
use std::{string::String, vec::Vec};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::ExceedsCapacity;
use crate::bitmap::Bitmap;

#[cfg(doc)]
use crate::StrVec28;
#[cfg(doc)]
use crate::StrVec56;
#[cfg(doc)]
use crate::StrVec112;

/// Stack-allocated, appendable string vector
///
/// The number of items as well as their lengths are variable. A StrVec can
/// contain up to `N` items and hold a maximum of `N` characters across all
/// items.
///
/// This structure is suitable for small string arrays with a known total
/// maximum length, where heap allocations should be avoided.
///
/// # Example
/// An example are topologies such as `aws:us:east:1`. Since all strings are
/// short (1..4) and there are few items (4), one could define a reasonable
/// upper bound for the total length (16). This assumption allows `StrVec<u16,
/// 16>` to represent various topologies.
///
/// # Comparison
/// Although `&[&str]` could be used, this is often impractical since the memory
/// is not owned and requires specifying a lifetime when embedded in a `struct`.
/// `[&str; N]` is an option if the total number of `N` is constant, but `&str`
/// still requires a lifetime.
///
/// # Limitations
/// An item cannot consist of a single NUL character which will be interpreted
/// as an empty string. As an empty string occupies 1 byte, a StrVec with `N`
/// items can effectively only contain empty items.
///
/// # Internal structure
/// A bitmap tracks every item's end position with a 1 bit. For example, the
/// bitmap corresponding to `[abc, def, ghi]` is: `0b001001001`
///
/// In terms of memory layout, all strings are stored contiguously
/// (`b"abcdefghi"`) which avoids pointer indirections when accessing elements
/// or iterating.
///
/// StrVec operations use efficient CPU instructions such as 'leading zeros' or
/// 'count ones'.
///
/// ## Size
/// The bitmap size must be below the capacity (`T::BITSIZE >= N`), for example
/// `StrVec<u16, 16>`. This value occupies 18 bytes (2-byte bitmap + 16-byte
/// data array). Since u16 is 2-byte aligned, the struct does not require any
/// internal or trailing padding.
///
/// # Usage
/// ```rust
/// # use qstr::StrVec28;
/// let mut vec = StrVec28::new();
///
/// assert!(vec.push("hello").is_ok());
/// assert!(vec.push("world").is_ok());
///
/// assert_eq!(vec.len(), 2);
///
/// assert_eq!(vec.get(0), Some("hello"));
/// assert_eq!(vec.get(1), Some("world"));
/// assert_eq!(vec.get(3), None);
/// ```
///
/// # Aliases
/// The following aliases that take into account cache line sizes are available:
/// [StrVec28], [StrVec56], [StrVec112]
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct StrVec<T: Bitmap, const N: usize, Alignment> {
  /// Marks each item's end position with a set bit
  pub(crate) bitmap: T,

  /// Strings are stored contiguously from left to right without delimiters.
  /// Unused space at the end is filled with NUL bytes. Empty items occupy a
  /// single byte, which will also be a NUL byte.
  pub(crate) data: [u8; N],

  align: [Alignment; 0],
}

impl<T: Bitmap, const N: usize, Alignment> StrVec<T, N, Alignment> {
  /// Create empty StrVec
  #[inline]
  pub fn new() -> Self {
    Self {
      bitmap: T::default(),
      data: [0u8; N],
      align: [],
    }
  }

  /// Create StrVec from an `&str` iterator
  ///
  /// # Safety
  /// This will panic if StrVec's capacity is exceeded
  pub fn from<'a, S>(values: S) -> Self
  where
    S: IntoIterator<Item = &'a str>,
  {
    Self::try_from(values).unwrap()
  }

  /// Attempts to create a StrVec from an `&str` iterator
  pub fn try_from<'a, S>(values: S) -> Result<Self, ExceedsCapacity>
  where
    S: IntoIterator<Item = &'a str>,
  {
    let mut result = StrVec::<T, N, Alignment>::new();

    for v in values {
      result.push(v)?;
    }

    Ok(result)
  }

  /// Create StrVec from a String iterator
  ///
  /// # Safety
  /// This will panic if StrVec's capacity is exceeded
  #[cfg(feature = "std")]
  pub fn from_owned<S>(values: S) -> Self
  where
    S: IntoIterator<Item = String>,
  {
    Self::try_from_owned(values).unwrap()
  }

  /// Attempts to create a StrVec from an iterator
  #[cfg(feature = "std")]
  pub fn try_from_owned<S>(values: S) -> Result<Self, ExceedsCapacity>
  where
    S: IntoIterator<Item = String>,
  {
    let mut result = Self::new();

    for v in values {
      result.push(&v)?;
    }

    Ok(result)
  }

  /// Number of items in O(1)
  #[inline]
  pub fn len(&self) -> usize {
    self.bitmap.count_ones()
  }

  /// Offset for inserting an item
  #[inline]
  pub(crate) fn next_offset(&self) -> usize {
    T::BITSIZE - self.bitmap.trailing_zeros()
  }

  /// Inserts given string at the end in O(1)
  ///
  /// Note: If s is `"\0"`, it corresponds to an empty string
  pub fn push(&mut self, s: &str) -> Result<(), ExceedsCapacity> {
    let s = if s.is_empty() { "\0" } else { s };

    let offset = self.next_offset();
    let str_len = s.len();

    if offset + str_len > N {
      return Err(ExceedsCapacity {
        length: offset + str_len,
        capacity: N,
      });
    }

    self.bitmap.set(offset + str_len - 1);
    self.data[offset..offset + str_len].copy_from_slice(s.as_bytes());

    Ok(())
  }

  /// Removes all elements
  pub fn clear(&mut self) {
    *self = Self::new();
  }

  /// Returns string at given index in O(N)
  pub fn get(&self, index: usize) -> Option<&str> {
    let (offset, end) = self.bitmap.find_nth_span(index)?;
    let span = &self.data[offset..end];

    if span == [0] {
      Some("")
    } else {
      // SAFETY: We trust that the stored bytes are valid UTF-8
      //         since we only store valid strings via push()
      Some(unsafe { core::str::from_utf8_unchecked(span) })
    }
  }

  /// Checks if there are no elements
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  /// Convert to an [Iterator]
  pub fn iter(&self) -> impl Iterator<Item = &str> {
    let mut offset = 0;
    let mut bitmap = self.bitmap;

    (0..self.len()).map(move |_| {
      let end = bitmap.leading_zeros();
      let span = &self.data[offset..(end + 1)];

      offset = end + 1;
      bitmap.unset(end);

      if span == [0] {
        ""
      } else {
        // SAFETY: We trust that the stored bytes are valid UTF-8
        //         since we only store valid strings via push()
        unsafe { core::str::from_utf8_unchecked(span) }
      }
    })
  }

  /// Convert to a [Vec]
  #[cfg(feature = "std")]
  pub fn to_vec(&self) -> Vec<&str> {
    self.iter().collect::<Vec<_>>()
  }
}

impl<T: Bitmap, const N: usize, Alignment> Default for StrVec<T, N, Alignment> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T: Bitmap, const N: usize, Alignment> hash::Hash for StrVec<T, N, Alignment> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.data.hash(state);
  }
}

impl<T: Bitmap + Eq, const N: usize, Alignment: Eq> Ord for StrVec<T, N, Alignment> {
  fn cmp(&self, other: &Self) -> core::cmp::Ordering {
    self.data.cmp(&other.data)
  }
}

impl<T: Bitmap + Eq, const N: usize, Alignment: Eq> PartialOrd for StrVec<T, N, Alignment> {
  fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

#[cfg(feature = "std")]
impl<T: Bitmap, const N: usize, Alignment> fmt::Debug for StrVec<T, N, Alignment> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.to_vec().fmt(f)
  }
}

#[cfg(feature = "serde")]
impl<T: Bitmap, const N: usize, Alignment> Serialize for StrVec<T, N, Alignment> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    self.to_vec().serialize(serializer)
  }
}

#[cfg(feature = "serde")]
impl<'de, T: Bitmap, const N: usize, Alignment> Deserialize<'de> for StrVec<T, N, Alignment> {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    let v = Vec::<String>::deserialize(deserializer)?;
    StrVec::try_from_owned(v).map_err(serde::de::Error::custom)
  }
}
