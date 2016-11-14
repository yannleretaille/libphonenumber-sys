#[allow(unused_imports)]
use {libc, cpp_utils};

pub mod std {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  /// C++ type: <span style='color: green;'>```std::bad_exception```</span>.
  #[repr(C)]
  pub struct BadException {
    _buffer: [u8; 8],
  }

  impl ::NewUninitialized for BadException {
    unsafe fn new_uninitialized() -> BadException {
      BadException { _buffer: std::mem::uninitialized() }
    }
  }

  /// C++ type: <span style='color: green;'>```std::exception```</span>.
  #[repr(C)]
  pub struct Exception {
    _buffer: [u8; 8],
  }

  impl ::NewUninitialized for Exception {
    unsafe fn new_uninitialized() -> Exception {
      Exception { _buffer: std::mem::uninitialized() }
    }
  }

}
