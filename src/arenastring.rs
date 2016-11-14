#[allow(unused_imports)]
use {libc, cpp_utils, std};

pub mod google {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  pub mod protobuf {
    #[allow(unused_imports)]
    use {libc, cpp_utils, std};

    pub mod internal {
      #[allow(unused_imports)]
      use {libc, cpp_utils, std};

      /// C++ type: <span style='color: green;'>```google::protobuf::internal::ArenaStringPtr```</span>.
      #[repr(C)]
      pub struct ArenaStringPtr {
        _buffer: [u8; 8],
      }

      impl ::NewUninitialized for ArenaStringPtr {
        unsafe fn new_uninitialized() -> ArenaStringPtr {
          ArenaStringPtr { _buffer: std::mem::uninitialized() }
        }
      }

    }

  }

}
