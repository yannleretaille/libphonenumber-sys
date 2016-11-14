#[allow(unused_imports)]
use {libc, cpp_utils};

pub mod std {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  /// C++ type: <span style='color: green;'>```std::char_traits<char>```</span>.
  #[repr(C)]
  pub struct CharTraitsCChar {
    _buffer: [u8; 1],
  }

  impl ::NewUninitialized for CharTraitsCChar {
    unsafe fn new_uninitialized() -> CharTraitsCChar {
      CharTraitsCChar { _buffer: std::mem::uninitialized() }
    }
  }

}
