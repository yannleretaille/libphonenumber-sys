#[allow(unused_imports)]
use {libc, cpp_utils};

pub mod std {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  pub mod cxx11 {
    #[allow(unused_imports)]
    use {libc, cpp_utils, std};

    /// C++ type: <span style='color: green;'>```std::__cxx11::list<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>, std::allocator<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>>>```</span>.
    #[repr(C)]
    pub struct ListBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRefAllocatorBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRefRef {
      _buffer: [u8; 24],
    }

    impl ::NewUninitialized for ListBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRefAllocatorBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRefRef {
  unsafe fn new_uninitialized() -> ListBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRefAllocatorBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRefRef {
    ListBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRefAllocatorBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRefRef { _buffer: std::mem::uninitialized() }
  }
}

  }

  pub mod detail {
    #[allow(unused_imports)]
    use {libc, cpp_utils, std};

    /// C++ type: <span style='color: green;'>```std::__detail::_List_node_base```</span>.
    #[repr(C)]
    pub struct ListNodeBase {
      _buffer: [u8; 16],
    }

    impl ::NewUninitialized for ListNodeBase {
      unsafe fn new_uninitialized() -> ListNodeBase {
        ListNodeBase { _buffer: std::mem::uninitialized() }
      }
    }

  }

}
