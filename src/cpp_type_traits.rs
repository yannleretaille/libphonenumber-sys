#[allow(unused_imports)]
use {libc, cpp_utils};

pub mod std {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  /// C++ type: <span style='color: green;'>```std::__false_type```</span>.
  #[repr(C)]
  pub struct FalseType {
    _buffer: [u8; 1],
  }

  impl ::NewUninitialized for FalseType {
    unsafe fn new_uninitialized() -> FalseType {
      FalseType { _buffer: std::mem::uninitialized() }
    }
  }

  /// C++ type: <span style='color: green;'>```std::__true_type```</span>.
  #[repr(C)]
  pub struct TrueType {
    _buffer: [u8; 1],
  }

  impl ::NewUninitialized for TrueType {
    unsafe fn new_uninitialized() -> TrueType {
      TrueType { _buffer: std::mem::uninitialized() }
    }
  }

}
