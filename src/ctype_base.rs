#[allow(unused_imports)]
use {libc, cpp_utils};

pub mod std {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  /// C++ type: <span style='color: green;'>```std::ctype_base```</span>.
  #[repr(C)]
  pub struct CtypeBase {
    _buffer: [u8; 1],
  }

  impl ::NewUninitialized for CtypeBase {
    unsafe fn new_uninitialized() -> CtypeBase {
      CtypeBase { _buffer: std::mem::uninitialized() }
    }
  }

}
