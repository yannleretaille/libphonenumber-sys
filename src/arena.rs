#[allow(unused_imports)]
use {libc, cpp_utils, std};

pub mod google {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  pub mod protobuf {
    #[allow(unused_imports)]
    use {libc, cpp_utils, std};

    /// C++ type: <span style='color: green;'>```google::protobuf::Arena```</span>.
    #[repr(C)]
    pub struct Arena {
      _buffer: [u8; 136],
    }

    impl ::NewUninitialized for Arena {
      unsafe fn new_uninitialized() -> Arena {
        Arena { _buffer: std::mem::uninitialized() }
      }
    }

    /// C++ type: <span style='color: green;'>```google::protobuf::ArenaOptions```</span>.
    #[repr(C)]
    pub struct ArenaOptions {
      _buffer: [u8; 80],
    }

    impl ::NewUninitialized for ArenaOptions {
      unsafe fn new_uninitialized() -> ArenaOptions {
        ArenaOptions { _buffer: std::mem::uninitialized() }
      }
    }

    pub mod arena {
      #[allow(unused_imports)]
      use {libc, cpp_utils, std};

      /// C++ type: <span style='color: green;'>```google::protobuf::Arena::Node```</span>.
      #[repr(C)]
      pub struct Node {
        _buffer: [u8; 24],
      }

      impl ::NewUninitialized for Node {
        unsafe fn new_uninitialized() -> Node {
          Node { _buffer: std::mem::uninitialized() }
        }
      }

    }

  }

}
