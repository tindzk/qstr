use core::fmt;

/// Length exceeds string's capacity
#[derive(PartialEq, Eq)]
pub struct ExceedsCapacity {
  /// Length
  pub length: usize,

  /// Capacity
  pub capacity: usize,
}

impl fmt::Debug for ExceedsCapacity {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_fmt(format_args!(
      "String length ({}) exceeds capacity ({})",
      self.length, self.capacity
    ))
  }
}

impl core::fmt::Display for ExceedsCapacity {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_fmt(format_args!(
      "String length ({}) exceeds capacity ({})",
      self.length, self.capacity
    ))
  }
}

impl core::error::Error for ExceedsCapacity {}
