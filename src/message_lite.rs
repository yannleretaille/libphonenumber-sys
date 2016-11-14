#[allow(unused_imports)]
use {libc, cpp_utils, std};

pub mod google {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  pub mod protobuf {
    #[allow(unused_imports)]
    use {libc, cpp_utils, std};

    /// C++ type: <span style='color: green;'>```google::protobuf::MessageLite```</span>.
    #[repr(C)]
    pub struct MessageLite {
      _buffer: [u8; 8],
    }

    impl ::NewUninitialized for MessageLite {
      unsafe fn new_uninitialized() -> MessageLite {
        MessageLite { _buffer: std::mem::uninitialized() }
      }
    }

  }

}
