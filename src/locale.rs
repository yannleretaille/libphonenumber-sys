#[allow(unused_imports)]
use {libc, cpp_utils, std};

/// C++ type: <span style='color: green;'>```lconv```</span>.
#[repr(C)]
pub struct Lconv {
  _buffer: [u8; 96],
}

impl ::NewUninitialized for Lconv {
  unsafe fn new_uninitialized() -> Lconv {
    Lconv { _buffer: std::mem::uninitialized() }
  }
}
