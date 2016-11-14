#[allow(unused_imports)]
use {libc, cpp_utils};

pub mod std {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  /// C++ type: <span style='color: green;'>```std::vector<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>, std::allocator<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>>>```</span>.
  #[repr(C)]
  pub struct VectorBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRefAllocatorBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRefRef {
    _buffer: [u8; 24],
  }

  impl ::NewUninitialized for VectorBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRefAllocatorBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRefRef {
  unsafe fn new_uninitialized() -> VectorBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRefAllocatorBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRefRef {
    VectorBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRefAllocatorBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRefRef { _buffer: std::mem::uninitialized() }
  }
}

}
