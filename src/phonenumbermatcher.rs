#[allow(unused_imports)]
use {libc, cpp_utils, std};

pub mod i18n {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  pub mod phonenumbers {
    #[allow(unused_imports)]
    use {libc, cpp_utils, std};

    /// C++ type: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberMatcher```</span>.
    #[repr(C)]
    pub struct PhoneNumberMatcher {
      _buffer: [u8; 120],
    }

    impl ::NewUninitialized for PhoneNumberMatcher {
      unsafe fn new_uninitialized() -> PhoneNumberMatcher {
        PhoneNumberMatcher { _buffer: std::mem::uninitialized() }
      }
    }

    impl PhoneNumberMatcher {
      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneNumberMatcher::HasNext()```</span>
      ///
      ///
      pub fn has_next(&mut self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatcher_HasNext(self as *mut ::phonenumbermatcher::i18n::phonenumbers::PhoneNumberMatcher) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberMatcher::PhoneNumberMatcher```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: <br>1) ```fn new((&::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, ::phonenumbermatcher::i18n::phonenumbers::phone_number_matcher::Leniency, libc::c_int, cpp_utils::AsStruct)) -> ::phonenumbermatcher::i18n::phonenumbers::PhoneNumberMatcher```<br>2) ```fn new((&::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, ::phonenumbermatcher::i18n::phonenumbers::phone_number_matcher::Leniency, libc::c_int, cpp_utils::AsBox)) -> cpp_utils::CppBox<::phonenumbermatcher::i18n::phonenumbers::PhoneNumberMatcher>```<br>
      /// C++ method: <span style='color: green;'>```[constructor] void i18n::phonenumbers::PhoneNumberMatcher::PhoneNumberMatcher(const i18n::phonenumbers::PhoneNumberUtil& util, const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& text, const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& region_code, i18n::phonenumbers::PhoneNumberMatcher::Leniency leniency, int max_tries)```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: <br>1) ```fn new((&::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, cpp_utils::AsStruct)) -> ::phonenumbermatcher::i18n::phonenumbers::PhoneNumberMatcher```<br>2) ```fn new((&::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, cpp_utils::AsBox)) -> cpp_utils::CppBox<::phonenumbermatcher::i18n::phonenumbers::PhoneNumberMatcher>```<br>
      /// C++ method: <span style='color: green;'>```[constructor] void i18n::phonenumbers::PhoneNumberMatcher::PhoneNumberMatcher(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& text, const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& region_code)```</span>
      ///
      ///
      pub fn new<Args>(args: Args) -> Args::ReturnType
        where Args: overloading::PhoneNumberMatcherNewArgs
      {
        args.exec()
      }
      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneNumberMatcher::Next(i18n::phonenumbers::PhoneNumberMatch* match)```</span>
      ///
      ///
      pub fn next(&mut self, match_: *mut ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatcher_Next(self as *mut ::phonenumbermatcher::i18n::phonenumbers::PhoneNumberMatcher, match_) }
      }
    }

    impl Drop for PhoneNumberMatcher {
      /// C++ method: <span style='color: green;'>```[destructor] void i18n::phonenumbers::PhoneNumberMatcher::~PhoneNumberMatcher()```</span>
      ///
      ///
      fn drop(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatcher_destructor(self as *mut ::phonenumbermatcher::i18n::phonenumbers::PhoneNumberMatcher) }
      }
    }

    impl cpp_utils::CppDeletable for PhoneNumberMatcher {
      fn deleter() -> cpp_utils::Deleter<Self> {
        ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatcher_delete
      }
    }

    pub mod overloading {
      #[allow(unused_imports)]
      use {libc, cpp_utils, std};

      /// This trait represents a set of arguments accepted by [PhoneNumberMatcher::new](../struct.PhoneNumberMatcher.html#method.new) method.
      pub trait PhoneNumberMatcherNewArgs {
        type ReturnType;
        fn exec(self) -> Self::ReturnType;
      }
      impl<'a> PhoneNumberMatcherNewArgs for (&'a ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,&'a ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,cpp_utils::AsStruct) {
  type ReturnType = ::phonenumbermatcher::i18n::phonenumbers::PhoneNumberMatcher;
  fn exec(self, ) -> ::phonenumbermatcher::i18n::phonenumbers::PhoneNumberMatcher {
    let text = self.0;
let region_code = self.1;
    {
let mut object: ::phonenumbermatcher::i18n::phonenumbers::PhoneNumberMatcher = unsafe { ::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatcher_constructor_text_region_code(text as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, region_code as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, &mut object) }object
}
  }
}
      impl<'a> PhoneNumberMatcherNewArgs for (&'a ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil,&'a ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,&'a ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,::phonenumbermatcher::i18n::phonenumbers::phone_number_matcher::Leniency,libc::c_int,cpp_utils::AsStruct) {
  type ReturnType = ::phonenumbermatcher::i18n::phonenumbers::PhoneNumberMatcher;
  fn exec(self, ) -> ::phonenumbermatcher::i18n::phonenumbers::PhoneNumberMatcher {
    let util = self.0;
let text = self.1;
let region_code = self.2;
let leniency = self.3;
let max_tries = self.4;
    {
let mut object: ::phonenumbermatcher::i18n::phonenumbers::PhoneNumberMatcher = unsafe { ::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatcher_constructor_util_text_region_code_leniency_max_tries(util as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, text as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, region_code as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, leniency, max_tries, &mut object) }object
}
  }
}
      impl<'a> PhoneNumberMatcherNewArgs for (&'a ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,&'a ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,cpp_utils::AsBox) {
  type ReturnType = cpp_utils::CppBox<::phonenumbermatcher::i18n::phonenumbers::PhoneNumberMatcher>;
  fn exec(self, ) -> cpp_utils::CppBox<::phonenumbermatcher::i18n::phonenumbers::PhoneNumberMatcher> {
    let text = self.0;
let region_code = self.1;
    let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatcher_new_text_region_code(text as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, region_code as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) };
unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }
}
      impl<'a> PhoneNumberMatcherNewArgs for (&'a ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil,&'a ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,&'a ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,::phonenumbermatcher::i18n::phonenumbers::phone_number_matcher::Leniency,libc::c_int,cpp_utils::AsBox) {
  type ReturnType = cpp_utils::CppBox<::phonenumbermatcher::i18n::phonenumbers::PhoneNumberMatcher>;
  fn exec(self, ) -> cpp_utils::CppBox<::phonenumbermatcher::i18n::phonenumbers::PhoneNumberMatcher> {
    let util = self.0;
let text = self.1;
let region_code = self.2;
let leniency = self.3;
let max_tries = self.4;
    let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatcher_new_util_text_region_code_leniency_max_tries(util as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, text as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, region_code as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, leniency, max_tries) };
unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }
}
    }

    pub mod phone_number_matcher {
      #[allow(unused_imports)]
      use {libc, cpp_utils, std};

      /// C++ type: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberMatcher::Leniency```</span>
      #[derive(Debug, PartialEq, Eq, Clone)]
      #[repr(C)]
      pub enum Leniency {
        /// C++ variant: POSSIBLE
        Possible = 0,
        /// C++ variant: VALID
        Valid = 1,
        /// C++ variant: STRICT_GROUPING
        StrictGrouping = 2,
        /// C++ variant: EXACT_GROUPING
        ExactGrouping = 3,
      }

    }

  }

}
