/// Bitmap manipulation and traversal functions
///
/// All functions assume that the left-most bit denotes the 0-th bit position.
pub trait Bitmap
where
  Self: Default + Copy,
{
  /// Number of available bits
  const BITSIZE: usize;

  /// Sets the bit at the specified position
  ///
  /// # Safety
  ///
  /// Requires that `offset < BITSIZE`
  fn set(&mut self, offset: usize);

  /// Unsets the bit at the specified position
  ///
  /// # Safety
  ///
  /// Requires that `offset < BITSIZE`
  fn unset(&mut self, offset: usize);

  fn leading_zeros(&self) -> usize;
  fn trailing_zeros(&self) -> usize;

  /// Counts the number of set bits
  fn count_ones(&self) -> usize;

  /// Returns the `(start, end)` range of the n-th span in O(N)
  ///
  /// The bitmap is interpreted as a sequence of spans. Each 1 marks the end of
  /// a span.
  ///
  /// Returns `None` if the span does not exist.
  fn find_nth_span(&self, n: usize) -> Option<(usize, usize)>;
}

macro_rules! impl_bitmap_for {
  ($t:ty, $bits:expr) => {
    impl Bitmap for $t {
      const BITSIZE: usize = $bits;

      #[inline]
      fn set(&mut self, offset: usize) {
        *self |= 1 as $t << ($bits - 1 - offset);
      }

      #[inline]
      fn unset(&mut self, offset: usize) {
        *self &= !(1 as $t << ($bits - 1 - offset));
      }

      #[inline]
      fn leading_zeros(&self) -> usize {
        <$t>::leading_zeros(*self) as usize
      }

      #[inline]
      fn trailing_zeros(&self) -> usize {
        <$t>::trailing_zeros(*self) as usize
      }

      #[inline]
      fn count_ones(&self) -> usize {
        <$t>::count_ones(*self) as usize
      }

      fn find_nth_span(&self, n: usize) -> Option<(usize, usize)> {
        let mut value = *self;
        let mut start = 0;
        let mut count = 0;

        while value != 0 {
          let end = value.leading_zeros();

          if count == n {
            return Some((start as usize, end as usize + 1));
          }

          // Clear bit
          value &= !(1 as $t << ($bits - 1 - end));

          start = end + 1;
          count += 1;
        }

        None
      }
    }
  };
}

// Implementations for all unsigned integer types
impl_bitmap_for!(u8, 8);
impl_bitmap_for!(u16, 16);
impl_bitmap_for!(u32, 32);
impl_bitmap_for!(u64, 64);
impl_bitmap_for!(u128, 128);
