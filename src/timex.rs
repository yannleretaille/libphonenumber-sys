#[allow(unused_imports)]
use {libc, cpp_utils, std};

/// C++ type: <span style='color: green;'>```timex```</span>.
#[repr(C)]
pub struct Timex {
  _buffer: [u8; 208],
}

impl ::NewUninitialized for Timex {
  unsafe fn new_uninitialized() -> Timex {
    Timex { _buffer: std::mem::uninitialized() }
  }
}
