use crate::{Align8, Align16, Align32, Align64, Align128};

/// Marker type for which [AlignmentForLength] instances are provided
pub type AlignmentMarker = ();

/// Associates a BStr length `N` with a suitable alignment
pub trait AlignmentForLength<const N: usize> {
  type Output;
}

impl AlignmentForLength<7> for AlignmentMarker {
  type Output = Align8;
}

impl AlignmentForLength<15> for AlignmentMarker {
  type Output = Align16;
}

impl AlignmentForLength<31> for AlignmentMarker {
  type Output = Align32;
}

impl AlignmentForLength<63> for AlignmentMarker {
  type Output = Align64;
}

impl AlignmentForLength<127> for AlignmentMarker {
  type Output = Align128;
}

/// Resolves the alignment type for the given BStr length `N`
pub type AlignmentType<const N: usize> = <AlignmentMarker as AlignmentForLength<N>>::Output;
