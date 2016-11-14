#[allow(unused_imports)]
use {libc, cpp_utils, std};

pub mod cxxabiv1 {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  /// C++ type: <span style='color: green;'>```__cxxabiv1::__forced_unwind```</span>.
  #[repr(C)]
  pub struct ForcedUnwind {
    _buffer: [u8; 8],
  }

  impl ::NewUninitialized for ForcedUnwind {
    unsafe fn new_uninitialized() -> ForcedUnwind {
      ForcedUnwind { _buffer: std::mem::uninitialized() }
    }
  }

}
