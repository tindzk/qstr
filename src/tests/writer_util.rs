use core::fmt;

/// From https://stackoverflow.com/a/64726826/13300239
pub struct ByteMutWriter<'a> {
  buf: &'a mut [u8],
  cursor: usize,
}

impl<'a> ByteMutWriter<'a> {
  pub fn new(buf: &'a mut [u8]) -> Self {
    ByteMutWriter { buf, cursor: 0 }
  }

  pub fn as_str(&self) -> &str {
    str::from_utf8(&self.buf[0..self.cursor]).unwrap()
  }

  #[inline]
  pub fn capacity(&self) -> usize {
    self.buf.len()
  }
}

impl fmt::Write for ByteMutWriter<'_> {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    let cap = self.capacity();

    for (i, &b) in self.buf[self.cursor..cap]
      .iter_mut()
      .zip(s.as_bytes().iter())
    {
      *i = b;
    }

    self.cursor = usize::min(cap, self.cursor + s.len());
    Ok(())
  }
}
