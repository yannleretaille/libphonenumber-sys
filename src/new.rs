#[allow(unused_imports)]
use {libc, cpp_utils};

pub mod std {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  /// C++ type: <span style='color: green;'>```std::bad_alloc```</span>.
  #[repr(C)]
  pub struct BadAlloc {
    _buffer: [u8; 8],
  }

  impl ::NewUninitialized for BadAlloc {
    unsafe fn new_uninitialized() -> BadAlloc {
      BadAlloc { _buffer: std::mem::uninitialized() }
    }
  }

  /// C++ type: <span style='color: green;'>```std::nothrow_t```</span>.
  #[repr(C)]
  pub struct NothrowT {
    _buffer: [u8; 1],
  }

  impl ::NewUninitialized for NothrowT {
    unsafe fn new_uninitialized() -> NothrowT {
      NothrowT { _buffer: std::mem::uninitialized() }
    }
  }

}
