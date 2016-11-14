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

      /// C++ type: <span style='color: green;'>```google::protobuf::internal::SequenceNumber```</span>.
      #[repr(C)]
      pub struct SequenceNumber {
        _buffer: [u8; 8],
      }

      impl ::NewUninitialized for SequenceNumber {
        unsafe fn new_uninitialized() -> SequenceNumber {
          SequenceNumber { _buffer: std::mem::uninitialized() }
        }
      }

    }

  }

}
