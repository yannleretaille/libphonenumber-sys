#[allow(unused_imports)]
use {libc, cpp_utils, std};

pub mod i18n {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  pub mod phonenumbers {
    #[allow(unused_imports)]
    use {libc, cpp_utils, std};

    /// C++ type: <span style='color: green;'>```i18n::phonenumbers::Singleton<i18n::phonenumbers::PhoneNumberUtil>```</span>.
    #[repr(C)]
    pub struct SingletonPhoneNumberUtilRef {
      _buffer: [u8; 8],
    }

    impl ::NewUninitialized for SingletonPhoneNumberUtilRef {
      unsafe fn new_uninitialized() -> SingletonPhoneNumberUtilRef {
        SingletonPhoneNumberUtilRef { _buffer: std::mem::uninitialized() }
      }
    }

    impl SingletonPhoneNumberUtilRef {
      /// C++ method: <span style='color: green;'>```static i18n::phonenumbers::PhoneNumberUtil* i18n::phonenumbers::Singleton<i18n::phonenumbers::PhoneNumberUtil>::GetInstance()```</span>
      ///
      ///
      pub fn get_instance() -> *mut ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil {
        unsafe {
          ::ffi::libphonenumber_sys_c_i18n_phonenumbers_Singleton_i18n_phonenumbers_PhoneNumberUtil_GetInstance()
        }
      }
    }

    impl Drop for SingletonPhoneNumberUtilRef {
      /// C++ method: <span style='color: green;'>```virtual [destructor] void i18n::phonenumbers::Singleton<i18n::phonenumbers::PhoneNumberUtil>::~Singleton()```</span>
      ///
      ///
      fn drop(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_Singleton_i18n_phonenumbers_PhoneNumberUtil_destructor(self as *mut ::singleton_posix::i18n::phonenumbers::SingletonPhoneNumberUtilRef) }
      }
    }

    impl cpp_utils::CppDeletable for SingletonPhoneNumberUtilRef {
      fn deleter() -> cpp_utils::Deleter<Self> {
        ::ffi::libphonenumber_sys_c_i18n_phonenumbers_Singleton_i18n_phonenumbers_PhoneNumberUtil_delete
      }
    }

  }

}
