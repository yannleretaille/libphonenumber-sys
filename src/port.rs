#[allow(unused_imports)]
use {libc, cpp_utils, std};

pub mod google {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  pub mod protobuf {
    #[allow(unused_imports)]
    use {libc, cpp_utils, std};

    /// C++ type: <span style='color: green;'>```google::protobuf::BigEndian```</span>.
    #[repr(C)]
    pub struct BigEndian {
      _buffer: [u8; 1],
    }

    impl ::NewUninitialized for BigEndian {
      unsafe fn new_uninitialized() -> BigEndian {
        BigEndian { _buffer: std::mem::uninitialized() }
      }
    }

  }

}
