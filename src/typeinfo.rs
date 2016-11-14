#[allow(unused_imports)]
use {libc, cpp_utils};

pub mod std {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  /// C++ type: <span style='color: green;'>```std::bad_cast```</span>.
  #[repr(C)]
  pub struct BadCast {
    _buffer: [u8; 8],
  }

  impl ::NewUninitialized for BadCast {
    unsafe fn new_uninitialized() -> BadCast {
      BadCast { _buffer: std::mem::uninitialized() }
    }
  }

  /// C++ type: <span style='color: green;'>```std::bad_typeid```</span>.
  #[repr(C)]
  pub struct BadTypeid {
    _buffer: [u8; 8],
  }

  impl ::NewUninitialized for BadTypeid {
    unsafe fn new_uninitialized() -> BadTypeid {
      BadTypeid { _buffer: std::mem::uninitialized() }
    }
  }

  /// C++ type: <span style='color: green;'>```std::type_info```</span>.
  #[repr(C)]
  pub struct TypeInfo {
    _buffer: [u8; 16],
  }

  impl ::NewUninitialized for TypeInfo {
    unsafe fn new_uninitialized() -> TypeInfo {
      TypeInfo { _buffer: std::mem::uninitialized() }
    }
  }

}
