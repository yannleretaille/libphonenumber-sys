#[allow(unused_imports)]
use {libc, cpp_utils, std};

/// C++ type: <span style='color: green;'>```sigaction```</span>.
#[repr(C)]
pub struct Sigaction {
  _buffer: [u8; 152],
}

impl ::NewUninitialized for Sigaction {
  unsafe fn new_uninitialized() -> Sigaction {
    Sigaction { _buffer: std::mem::uninitialized() }
  }
}
