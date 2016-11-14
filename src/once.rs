#[allow(unused_imports)]
use {libc, cpp_utils, std};

pub mod google {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  pub mod protobuf {
    #[allow(unused_imports)]
    use {libc, cpp_utils, std};

    /// C++ type: <span style='color: green;'>```google::protobuf::GoogleOnceDynamic```</span>.
    #[repr(C)]
    pub struct GoogleOnceDynamic {
      _buffer: [u8; 8],
    }

    impl ::NewUninitialized for GoogleOnceDynamic {
      unsafe fn new_uninitialized() -> GoogleOnceDynamic {
        GoogleOnceDynamic { _buffer: std::mem::uninitialized() }
      }
    }

  }

}
