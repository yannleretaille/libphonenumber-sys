#[allow(unused_imports)]
use {libc, cpp_utils};

pub mod std {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  /// C++ type: <span style='color: green;'>```std::less<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>>```</span>.
  #[repr(C)]
  pub struct LessBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRef {
    _buffer: [u8; 1],
  }

  impl ::NewUninitialized for LessBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRef {
    unsafe fn new_uninitialized() -> LessBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRef {
      LessBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRef { _buffer: std::mem::uninitialized() }
    }
  }

  /// C++ type: <span style='color: green;'>```std::less<int>```</span>.
  #[repr(C)]
  pub struct LessCInt {
    _buffer: [u8; 1],
  }

  impl ::NewUninitialized for LessCInt {
    unsafe fn new_uninitialized() -> LessCInt {
      LessCInt { _buffer: std::mem::uninitialized() }
    }
  }

}
