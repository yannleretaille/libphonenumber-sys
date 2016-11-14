#[allow(unused_imports)]
use {libc, cpp_utils};

pub mod std {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  /// C++ type: <span style='color: green;'>```std::__num_base```</span>.
  #[repr(C)]
  pub struct NumBase {
    _buffer: [u8; 1],
  }

  impl ::NewUninitialized for NumBase {
    unsafe fn new_uninitialized() -> NumBase {
      NumBase { _buffer: std::mem::uninitialized() }
    }
  }

}
