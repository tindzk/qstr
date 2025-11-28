#[cfg(feature = "std")]
mod std {
  use ::std::boxed::Box;
  use core::error::Error;

  use crate::ExceedsCapacity;

  #[test]
  fn test_error() {
    let _: Box<dyn Error> = Box::new(ExceedsCapacity {
      length: 16,
      capacity: 8,
    });
  }
}
