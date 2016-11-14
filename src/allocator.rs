#[allow(unused_imports)]
use {libc, cpp_utils};

pub mod std {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  /// C++ type: <span style='color: green;'>```std::allocator<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>>```</span>.
  #[repr(C)]
  pub struct AllocatorBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRef {
    _buffer: [u8; 1],
  }

  impl ::NewUninitialized for AllocatorBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRef {
    unsafe fn new_uninitialized() -> AllocatorBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRef {
      AllocatorBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRef { _buffer: std::mem::uninitialized() }
    }
  }

  /// C++ type: <span style='color: green;'>```std::allocator<char>```</span>.
  #[repr(C)]
  pub struct AllocatorCChar {
    _buffer: [u8; 1],
  }

  impl ::NewUninitialized for AllocatorCChar {
    unsafe fn new_uninitialized() -> AllocatorCChar {
      AllocatorCChar { _buffer: std::mem::uninitialized() }
    }
  }

  /// C++ type: <span style='color: green;'>```std::allocator<int>```</span>.
  #[repr(C)]
  pub struct AllocatorCInt {
    _buffer: [u8; 1],
  }

  impl ::NewUninitialized for AllocatorCInt {
    unsafe fn new_uninitialized() -> AllocatorCInt {
      AllocatorCInt { _buffer: std::mem::uninitialized() }
    }
  }

}
