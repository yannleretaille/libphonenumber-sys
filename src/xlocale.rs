#[allow(unused_imports)]
use {libc, cpp_utils, std};

/// C++ type: <span style='color: green;'>```__locale_struct```</span>.
#[repr(C)]
pub struct LocaleStruct {
  _buffer: [u8; 232],
}

impl ::NewUninitialized for LocaleStruct {
  unsafe fn new_uninitialized() -> LocaleStruct {
    LocaleStruct { _buffer: std::mem::uninitialized() }
  }
}
