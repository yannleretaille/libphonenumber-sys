#[allow(unused_imports)]
use {libc, cpp_utils, std};

pub mod google {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  pub mod protobuf {
    #[allow(unused_imports)]
    use {libc, cpp_utils, std};

    /// C++ type: <span style='color: green;'>```google::protobuf::Closure```</span>.
    #[repr(C)]
    pub struct Closure {
      _buffer: [u8; 8],
    }

    impl ::NewUninitialized for Closure {
      unsafe fn new_uninitialized() -> Closure {
        Closure { _buffer: std::mem::uninitialized() }
      }
    }

    pub mod internal {
      #[allow(unused_imports)]
      use {libc, cpp_utils, std};

      /// C++ type: <span style='color: green;'>```google::protobuf::internal::FunctionClosure0```</span>.
      #[repr(C)]
      pub struct FunctionClosure0 {
        _buffer: [u8; 24],
      }

      impl ::NewUninitialized for FunctionClosure0 {
        unsafe fn new_uninitialized() -> FunctionClosure0 {
          FunctionClosure0 { _buffer: std::mem::uninitialized() }
        }
      }

    }

  }

}
