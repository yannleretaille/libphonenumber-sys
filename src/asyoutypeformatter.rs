#[allow(unused_imports)]
use {libc, cpp_utils, std};

pub mod i18n {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  pub mod phonenumbers {
    #[allow(unused_imports)]
    use {libc, cpp_utils, std};

    /// C++ type: <span style='color: green;'>```i18n::phonenumbers::AsYouTypeFormatter```</span>.
    #[repr(C)]
    pub struct AsYouTypeFormatter {
      _buffer: [u8; 736],
    }

    impl ::NewUninitialized for AsYouTypeFormatter {
      unsafe fn new_uninitialized() -> AsYouTypeFormatter {
        AsYouTypeFormatter { _buffer: std::mem::uninitialized() }
      }
    }

    impl AsYouTypeFormatter {
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::AsYouTypeFormatter::Clear()```</span>
      ///
      ///
      pub fn clear(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_AsYouTypeFormatter_Clear(self as *mut ::asyoutypeformatter::i18n::phonenumbers::AsYouTypeFormatter) }
      }

      /// C++ method: <span style='color: green;'>```int i18n::phonenumbers::AsYouTypeFormatter::GetRememberedPosition() const```</span>
      ///
      ///
      pub fn get_remembered_position(&self) -> libc::c_int {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_AsYouTypeFormatter_GetRememberedPosition(self as *const ::asyoutypeformatter::i18n::phonenumbers::AsYouTypeFormatter) }
      }

      /// C++ method: <span style='color: green;'>```const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& i18n::phonenumbers::AsYouTypeFormatter::InputDigit(int next_char, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* result)```</span>
      ///
      ///
      pub fn input_digit<'l0>
        (&'l0 mut self,
         next_char: libc::c_int,
         result: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef)
         -> &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_AsYouTypeFormatter_InputDigit(self as *mut ::asyoutypeformatter::i18n::phonenumbers::AsYouTypeFormatter, next_char, result) };
        unsafe { &*ffi_result }
      }

      /// C++ method: <span style='color: green;'>```const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& i18n::phonenumbers::AsYouTypeFormatter::InputDigitAndRememberPosition(int next_char, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* result)```</span>
      ///
      ///
      pub fn input_digit_and_remember_position<'l0>
        (&'l0 mut self,
         next_char: libc::c_int,
         result: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef)
         -> &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_AsYouTypeFormatter_InputDigitAndRememberPosition(self as *mut ::asyoutypeformatter::i18n::phonenumbers::AsYouTypeFormatter, next_char, result) };
        unsafe { &*ffi_result }
      }
    }

    impl Drop for AsYouTypeFormatter {
      /// C++ method: <span style='color: green;'>```[destructor] void i18n::phonenumbers::AsYouTypeFormatter::~AsYouTypeFormatter()```</span>
      ///
      ///
      fn drop(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_AsYouTypeFormatter_destructor(self as *mut ::asyoutypeformatter::i18n::phonenumbers::AsYouTypeFormatter) }
      }
    }

    impl cpp_utils::CppDeletable for AsYouTypeFormatter {
      fn deleter() -> cpp_utils::Deleter<Self> {
        ::ffi::libphonenumber_sys_c_i18n_phonenumbers_AsYouTypeFormatter_delete
      }
    }

  }

}
