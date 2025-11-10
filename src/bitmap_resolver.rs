use crate::bitmap::Bitmap;

/// Marker type for which [BitmapForLength] instances are provided
pub type BitmapMarker = ();

/// Associates a BStr length `N` with a suitable bitmap type
pub trait BitmapForLength<const N: usize> {
  type Output: Bitmap;
}

impl BitmapForLength<7> for BitmapMarker {
  type Output = u8;
}

impl BitmapForLength<15> for BitmapMarker {
  type Output = u16;
}

impl BitmapForLength<31> for BitmapMarker {
  type Output = u32;
}

impl BitmapForLength<63> for BitmapMarker {
  type Output = u64;
}

impl BitmapForLength<127> for BitmapMarker {
  type Output = u128;
}

/// Resolves the [Bitmap] type for the given BStr length `N`
pub type BitmapType<const N: usize> = <BitmapMarker as BitmapForLength<N>>::Output;
