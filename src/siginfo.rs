#[allow(unused_imports)]
use {libc, cpp_utils, std};

/// C++ type: <span style='color: green;'>```sigevent```</span>.
#[repr(C)]
pub struct Sigevent {
  _buffer: [u8; 64],
}

impl ::NewUninitialized for Sigevent {
  unsafe fn new_uninitialized() -> Sigevent {
    Sigevent { _buffer: std::mem::uninitialized() }
  }
}
