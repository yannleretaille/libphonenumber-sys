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

      /// C++ type: <span style='color: green;'>```google::protobuf::internal::big_```</span>.
      #[repr(C)]
      pub struct Big {
        _buffer: [u8; 2],
      }

      impl ::NewUninitialized for Big {
        unsafe fn new_uninitialized() -> Big {
          Big { _buffer: std::mem::uninitialized() }
        }
      }

    }

  }

}

pub mod i18n {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  pub mod phonenumbers {
    #[allow(unused_imports)]
    use {libc, cpp_utils, std};

    pub mod internal {
      #[allow(unused_imports)]
      use {libc, cpp_utils, std};

      /// C++ type: <span style='color: green;'>```i18n::phonenumbers::internal::ConvertHelper```</span>.
      #[repr(C)]
      pub struct ConvertHelper {
        _buffer: [u8; 1],
      }

      impl ::NewUninitialized for ConvertHelper {
        unsafe fn new_uninitialized() -> ConvertHelper {
          ConvertHelper { _buffer: std::mem::uninitialized() }
        }
      }

      /// C++ type: <span style='color: green;'>```i18n::phonenumbers::internal::IsClassHelper```</span>.
      #[repr(C)]
      pub struct IsClassHelper {
        _buffer: [u8; 1],
      }

      impl ::NewUninitialized for IsClassHelper {
        unsafe fn new_uninitialized() -> IsClassHelper {
          IsClassHelper { _buffer: std::mem::uninitialized() }
        }
      }

      /// C++ type: <span style='color: green;'>```i18n::phonenumbers::internal::NoType```</span>.
      #[repr(C)]
      pub struct NoType {
        _buffer: [u8; 2],
      }

      impl ::NewUninitialized for NoType {
        unsafe fn new_uninitialized() -> NoType {
          NoType { _buffer: std::mem::uninitialized() }
        }
      }

    }

  }

}
