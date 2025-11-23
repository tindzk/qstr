/// Zero-sized type (ZST) to enforce 8-byte memory alignment
#[repr(align(8))]
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct Align8;

/// Zero-sized type (ZST) to enforce 16-byte memory alignment
#[repr(align(16))]
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct Align16;

/// Zero-sized type (ZST) to enforce 32-byte memory alignment
#[repr(align(32))]
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct Align32;

/// Zero-sized type (ZST) to enforce 64-byte memory alignment
#[repr(align(64))]
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct Align64;

/// Zero-sized type (ZST) to enforce 128-byte memory alignment
#[repr(align(128))]
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct Align128;
