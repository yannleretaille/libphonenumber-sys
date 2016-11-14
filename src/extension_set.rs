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

      /// C++ type: <span style='color: green;'>```google::protobuf::internal::ExtensionFinder```</span>.
      #[repr(C)]
      pub struct ExtensionFinder {
        _buffer: [u8; 8],
      }

      impl ::NewUninitialized for ExtensionFinder {
        unsafe fn new_uninitialized() -> ExtensionFinder {
          ExtensionFinder { _buffer: std::mem::uninitialized() }
        }
      }

      /// C++ type: <span style='color: green;'>```google::protobuf::internal::ExtensionInfo```</span>.
      #[repr(C)]
      pub struct ExtensionInfo {
        _buffer: [u8; 32],
      }

      impl ::NewUninitialized for ExtensionInfo {
        unsafe fn new_uninitialized() -> ExtensionInfo {
          ExtensionInfo { _buffer: std::mem::uninitialized() }
        }
      }

      /// C++ type: <span style='color: green;'>```google::protobuf::internal::ExtensionSet```</span>.
      #[repr(C)]
      pub struct ExtensionSet {
        _buffer: [u8; 56],
      }

      impl ::NewUninitialized for ExtensionSet {
        unsafe fn new_uninitialized() -> ExtensionSet {
          ExtensionSet { _buffer: std::mem::uninitialized() }
        }
      }

      /// C++ type: <span style='color: green;'>```google::protobuf::internal::GeneratedExtensionFinder```</span>.
      #[repr(C)]
      pub struct GeneratedExtensionFinder {
        _buffer: [u8; 16],
      }

      impl ::NewUninitialized for GeneratedExtensionFinder {
        unsafe fn new_uninitialized() -> GeneratedExtensionFinder {
          GeneratedExtensionFinder { _buffer: std::mem::uninitialized() }
        }
      }

      /// C++ type: <span style='color: green;'>```google::protobuf::internal::RepeatedMessageGenericTypeTraits```</span>.
      #[repr(C)]
      pub struct RepeatedMessageGenericTypeTraits {
        _buffer: [u8; 1],
      }

      impl ::NewUninitialized for RepeatedMessageGenericTypeTraits {
        unsafe fn new_uninitialized() -> RepeatedMessageGenericTypeTraits {
          RepeatedMessageGenericTypeTraits { _buffer: std::mem::uninitialized() }
        }
      }

      /// C++ type: <span style='color: green;'>```google::protobuf::internal::RepeatedPrimitiveGenericTypeTraits```</span>.
      #[repr(C)]
      pub struct RepeatedPrimitiveGenericTypeTraits {
        _buffer: [u8; 1],
      }

      impl ::NewUninitialized for RepeatedPrimitiveGenericTypeTraits {
        unsafe fn new_uninitialized() -> RepeatedPrimitiveGenericTypeTraits {
          RepeatedPrimitiveGenericTypeTraits { _buffer: std::mem::uninitialized() }
        }
      }

      /// C++ type: <span style='color: green;'>```google::protobuf::internal::RepeatedStringTypeTraits```</span>.
      #[repr(C)]
      pub struct RepeatedStringTypeTraits {
        _buffer: [u8; 1],
      }

      impl ::NewUninitialized for RepeatedStringTypeTraits {
        unsafe fn new_uninitialized() -> RepeatedStringTypeTraits {
          RepeatedStringTypeTraits { _buffer: std::mem::uninitialized() }
        }
      }

      /// C++ type: <span style='color: green;'>```google::protobuf::internal::StringTypeTraits```</span>.
      #[repr(C)]
      pub struct StringTypeTraits {
        _buffer: [u8; 1],
      }

      impl ::NewUninitialized for StringTypeTraits {
        unsafe fn new_uninitialized() -> StringTypeTraits {
          StringTypeTraits { _buffer: std::mem::uninitialized() }
        }
      }

      pub mod extension_info {
        #[allow(unused_imports)]
        use {libc, cpp_utils, std};

        /// C++ type: <span style='color: green;'>```google::protobuf::internal::ExtensionInfo::EnumValidityCheck```</span>.
        #[repr(C)]
        pub struct EnumValidityCheck {
          _buffer: [u8; 16],
        }

        impl ::NewUninitialized for EnumValidityCheck {
          unsafe fn new_uninitialized() -> EnumValidityCheck {
            EnumValidityCheck { _buffer: std::mem::uninitialized() }
          }
        }

      }

    }

  }

}
