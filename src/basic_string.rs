#[allow(unused_imports)]
use {libc, cpp_utils};

pub mod std {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  pub mod cxx11 {
    #[allow(unused_imports)]
    use {libc, cpp_utils, std};

    /// C++ type: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>```</span>.
    #[repr(C)]
    pub struct BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
      _buffer: [u8; 32],
    }

    impl ::NewUninitialized for BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
      unsafe fn new_uninitialized() -> BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        BasicStringCCharCharTraitsCCharRefAllocatorCCharRef { _buffer: std::mem::uninitialized() }
      }
    }
    //added
    impl cpp_utils::CppDeletable for BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
      fn deleter() -> cpp_utils::Deleter<Self> { 
			unsafe extern "C" fn deleter_func(this_ptr: *mut BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {
				drop(this_ptr);
			};
			deleter_func
		}
    }

  }

}
