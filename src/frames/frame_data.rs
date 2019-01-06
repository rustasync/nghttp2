use super::Header;
use crate::StreamId;

/// The DATA frame
#[derive(Debug, Clone)]
pub struct DataFrame {
  inner: libnghttp2_sys::nghttp2_data,
}

impl DataFrame {
  /// The length of the padding in this frame. This includes PAD_HIGH and PAD_LOW.
  #[inline]
  pub fn pad_len(&self) -> usize {
    self.inner.padlen
  }
}

impl Header for DataFrame {
  #[inline]
  fn flags(&self) -> u8 {
    self.inner.hd.flags
  }

  #[inline]
  fn stream_id(&self) -> StreamId {
    self.inner.hd.stream_id
  }
}

#[doc(hidden)]
impl From<libnghttp2_sys::nghttp2_data> for DataFrame {
  #[inline]
  fn from(inner: libnghttp2_sys::nghttp2_data) -> Self {
    Self { inner }
  }
}

#[doc(hidden)]
impl Into<libnghttp2_sys::nghttp2_data> for DataFrame {
  #[inline]
  fn into(self) -> libnghttp2_sys::nghttp2_data {
    self.inner
  }
}
