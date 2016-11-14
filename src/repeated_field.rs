#[allow(unused_imports)]
use {libc, cpp_utils, std};

pub mod google {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  pub mod protobuf {
    #[allow(unused_imports)]
    use {libc, cpp_utils, std};

    /// C++ type: <span style='color: green;'>```google::protobuf::RepeatedPtrField<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>>```</span>.
    #[repr(C)]
    pub struct RepeatedPtrFieldBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRef {
      _buffer: [u8; 24],
    }

    impl ::NewUninitialized for RepeatedPtrFieldBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRef {
      unsafe fn new_uninitialized() -> RepeatedPtrFieldBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRef {
        RepeatedPtrFieldBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRef { _buffer: std::mem::uninitialized() }
      }
    }

    /// C++ type: <span style='color: green;'>```google::protobuf::RepeatedPtrField<i18n::phonenumbers::NumberFormat>```</span>.
    #[repr(C)]
    pub struct RepeatedPtrFieldNumberFormatRef {
      _buffer: [u8; 24],
    }

    impl ::NewUninitialized for RepeatedPtrFieldNumberFormatRef {
      unsafe fn new_uninitialized() -> RepeatedPtrFieldNumberFormatRef {
        RepeatedPtrFieldNumberFormatRef { _buffer: std::mem::uninitialized() }
      }
    }

    /// C++ type: <span style='color: green;'>```google::protobuf::RepeatedPtrField<i18n::phonenumbers::PhoneMetadata>```</span>.
    #[repr(C)]
    pub struct RepeatedPtrFieldPhoneMetadataRef {
      _buffer: [u8; 24],
    }

    impl ::NewUninitialized for RepeatedPtrFieldPhoneMetadataRef {
      unsafe fn new_uninitialized() -> RepeatedPtrFieldPhoneMetadataRef {
        RepeatedPtrFieldPhoneMetadataRef { _buffer: std::mem::uninitialized() }
      }
    }

    pub mod internal {
      #[allow(unused_imports)]
      use {libc, cpp_utils, std};

      /// C++ type: <span style='color: green;'>```google::protobuf::internal::RepeatedPtrFieldBase```</span>.
      #[repr(C)]
      pub struct RepeatedPtrFieldBase {
        _buffer: [u8; 24],
      }

      impl ::NewUninitialized for RepeatedPtrFieldBase {
        unsafe fn new_uninitialized() -> RepeatedPtrFieldBase {
          RepeatedPtrFieldBase { _buffer: std::mem::uninitialized() }
        }
      }

      /// C++ type: <span style='color: green;'>```google::protobuf::internal::StringTypeHandler```</span>.
      #[repr(C)]
      pub struct StringTypeHandler {
        _buffer: [u8; 1],
      }

      impl ::NewUninitialized for StringTypeHandler {
        unsafe fn new_uninitialized() -> StringTypeHandler {
          StringTypeHandler { _buffer: std::mem::uninitialized() }
        }
      }

      /// C++ type: <span style='color: green;'>```google::protobuf::internal::StringTypeHandlerBase```</span>.
      #[repr(C)]
      pub struct StringTypeHandlerBase {
        _buffer: [u8; 1],
      }

      impl ::NewUninitialized for StringTypeHandlerBase {
        unsafe fn new_uninitialized() -> StringTypeHandlerBase {
          StringTypeHandlerBase { _buffer: std::mem::uninitialized() }
        }
      }

    }

  }

}
