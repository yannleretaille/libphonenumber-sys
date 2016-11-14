#[allow(unused_imports)]
use {libc, cpp_utils};

pub mod std {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  /// C++ type: <span style='color: green;'>```std::set<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>, std::less<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>>, std::allocator<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>>>```</span>.
  #[repr(C)]
  pub struct SetBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRefLessBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRefRefAllocatorBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRefRef {
    _buffer: [u8; 48],
  }

  impl ::NewUninitialized for SetBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRefLessBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRefRefAllocatorBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRefRef {
  unsafe fn new_uninitialized() -> SetBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRefLessBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRefRefAllocatorBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRefRef {
    SetBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRefLessBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRefRefAllocatorBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRefRef { _buffer: std::mem::uninitialized() }
  }
}

  /// C++ type: <span style='color: green;'>```std::set<int, std::less<int>, std::allocator<int>>```</span>.
  #[repr(C)]
  pub struct SetCIntLessCIntRefAllocatorCIntRef {
    _buffer: [u8; 48],
  }

  impl ::NewUninitialized for SetCIntLessCIntRefAllocatorCIntRef {
    unsafe fn new_uninitialized() -> SetCIntLessCIntRefAllocatorCIntRef {
      SetCIntLessCIntRefAllocatorCIntRef { _buffer: std::mem::uninitialized() }
    }
  }

}
