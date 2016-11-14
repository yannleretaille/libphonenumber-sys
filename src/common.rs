#[allow(unused_imports)]
use {libc, cpp_utils, std};

pub mod google {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  pub mod protobuf {
    #[allow(unused_imports)]
    use {libc, cpp_utils, std};

    /// C++ type: <span style='color: green;'>```google::protobuf::FatalException```</span>.
    #[repr(C)]
    pub struct FatalException {
      _buffer: [u8; 56],
    }

    impl ::NewUninitialized for FatalException {
      unsafe fn new_uninitialized() -> FatalException {
        FatalException { _buffer: std::mem::uninitialized() }
      }
    }

  }

}
