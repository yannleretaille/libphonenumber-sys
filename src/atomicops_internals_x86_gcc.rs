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

      /// C++ type: <span style='color: green;'>```google::protobuf::internal::AtomicOps_x86CPUFeatureStruct```</span>.
      #[repr(C)]
      pub struct AtomicOpsX86CpufEatureStruct {
        _buffer: [u8; 2],
      }

      impl ::NewUninitialized for AtomicOpsX86CpufEatureStruct {
        unsafe fn new_uninitialized() -> AtomicOpsX86CpufEatureStruct {
          AtomicOpsX86CpufEatureStruct { _buffer: std::mem::uninitialized() }
        }
      }

    }

  }

}
