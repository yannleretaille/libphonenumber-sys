#[allow(unused_imports)]
use {libc, cpp_utils, std};

pub mod i18n {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  pub mod phonenumbers {
    #[allow(unused_imports)]
    use {libc, cpp_utils, std};

    /// C++ type: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberMatch```</span>.
    #[repr(C)]
    pub struct PhoneNumberMatch {
      _buffer: [u8; 120],
    }

    impl ::NewUninitialized for PhoneNumberMatch {
      unsafe fn new_uninitialized() -> PhoneNumberMatch {
        PhoneNumberMatch { _buffer: std::mem::uninitialized() }
      }
    }

    impl PhoneNumberMatch {
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberMatch::CopyFrom(const i18n::phonenumbers::PhoneNumberMatch& number)```</span>
      ///
      ///
      pub fn copy_from(&mut self, number: &::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_CopyFrom(self as *mut ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch, number as *const ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch) }
      }

      /// C++ method: <span style='color: green;'>```int i18n::phonenumbers::PhoneNumberMatch::end() const```</span>
      ///
      ///
      pub fn end(&self) -> libc::c_int {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_end(self as *const ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneNumberMatch::Equals(const i18n::phonenumbers::PhoneNumberMatch& number) const```</span>
      ///
      ///
      pub fn equals(&self, number: &::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_Equals(self as *const ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch, number as *const ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch) }
      }

      /// C++ method: <span style='color: green;'>```int i18n::phonenumbers::PhoneNumberMatch::length() const```</span>
      ///
      ///
      pub fn length(&self) -> libc::c_int {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_length(self as *const ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberMatch::PhoneNumberMatch```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: <br>1) ```fn new(cpp_utils::AsStruct) -> ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch```<br>2) ```fn new(cpp_utils::AsBox) -> cpp_utils::CppBox<::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch>```<br>
      /// C++ method: <span style='color: green;'>```[constructor] void i18n::phonenumbers::PhoneNumberMatch::PhoneNumberMatch()```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: <br>1) ```fn new((libc::c_int, &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, &::phonenumber::i18n::phonenumbers::PhoneNumber, cpp_utils::AsStruct)) -> ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch```<br>2) ```fn new((libc::c_int, &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, &::phonenumber::i18n::phonenumbers::PhoneNumber, cpp_utils::AsBox)) -> cpp_utils::CppBox<::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch>```<br>
      /// C++ method: <span style='color: green;'>```[constructor] void i18n::phonenumbers::PhoneNumberMatch::PhoneNumberMatch(int start, const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& raw_string, const i18n::phonenumbers::PhoneNumber& number)```</span>
      ///
      ///
      pub fn new<Args>(args: Args) -> Args::ReturnType
        where Args: overloading::PhoneNumberMatchNewArgs
      {
        args.exec()
      }
      /// C++ method: <span style='color: green;'>```const i18n::phonenumbers::PhoneNumber& i18n::phonenumbers::PhoneNumberMatch::number() const```</span>
      ///
      ///
      pub fn number<'l0>(&'l0 self) -> &'l0 ::phonenumber::i18n::phonenumbers::PhoneNumber {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_number(self as *const ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch) };
        unsafe { &*ffi_result }
      }

      /// C++ method: <span style='color: green;'>```const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& i18n::phonenumbers::PhoneNumberMatch::raw_string() const```</span>
      ///
      ///
      pub fn raw_string<'l0>
        (&'l0 self)
         -> &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_raw_string(self as *const ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch) };
        unsafe { &*ffi_result }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberMatch::set_number(const i18n::phonenumbers::PhoneNumber& number)```</span>
      ///
      ///
      pub fn set_number(&mut self, number: &::phonenumber::i18n::phonenumbers::PhoneNumber) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_set_number(self as *mut ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch, number as *const ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberMatch::set_raw_string(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& raw_string)```</span>
      ///
      ///
pub fn set_raw_string(&mut self, raw_string: &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_set_raw_string(self as *mut ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch, raw_string as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberMatch::set_start(int start)```</span>
      ///
      ///
      pub fn set_start(&mut self, start: libc::c_int) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_set_start(self as *mut ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch, start) }
      }

      /// C++ method: <span style='color: green;'>```int i18n::phonenumbers::PhoneNumberMatch::start() const```</span>
      ///
      ///
      pub fn start(&self) -> libc::c_int {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_start(self as *const ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberMatch::ToString```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      /// Rust arguments: <br>1) ```fn to_string(&self, cpp_utils::AsBox) -> cpp_utils::CppBox<::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef>```<br>2) ```fn to_string(&self, cpp_utils::AsStruct) -> ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef```<br>
      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>> i18n::phonenumbers::PhoneNumberMatch::ToString() const```</span>
      ///
      ///
      pub fn to_string<'l0, Args>(&'l0 self, args: Args) -> Args::ReturnType
        where Args: overloading::PhoneNumberMatchToStringArgs<'l0>
      {
        args.exec(self)
      }
    }

    impl Drop for PhoneNumberMatch {
      /// C++ method: <span style='color: green;'>```[destructor] void i18n::phonenumbers::PhoneNumberMatch::~PhoneNumberMatch()```</span>
      ///
      ///
      fn drop(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_destructor(self as *mut ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch) }
      }
    }

    impl cpp_utils::CppDeletable for PhoneNumberMatch {
      fn deleter() -> cpp_utils::Deleter<Self> {
        ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_delete
      }
    }

    pub mod overloading {
      #[allow(unused_imports)]
      use {libc, cpp_utils, std};

      /// This trait represents a set of arguments accepted by [PhoneNumberMatch::new](../struct.PhoneNumberMatch.html#method.new) method.
      pub trait PhoneNumberMatchNewArgs {
        type ReturnType;
        fn exec(self) -> Self::ReturnType;
      }
      impl PhoneNumberMatchNewArgs for cpp_utils::AsStruct {
        type ReturnType = ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch;
        fn exec(self) -> ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch {

          {
            let mut object: ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch =
              unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_constructor_no_args(&mut object) }
            object
          }
        }
      }
      impl<'a> PhoneNumberMatchNewArgs for (libc::c_int,&'a ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,&'a ::phonenumber::i18n::phonenumbers::PhoneNumber,cpp_utils::AsStruct) {
  type ReturnType = ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch;
  fn exec(self, ) -> ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch {
    let start = self.0;
let raw_string = self.1;
let number = self.2;
    {
let mut object: ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch = unsafe { ::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_constructor_start_raw_string_number(start, raw_string as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, number as *const ::phonenumber::i18n::phonenumbers::PhoneNumber, &mut object) }object
}
  }
}
      impl PhoneNumberMatchNewArgs for cpp_utils::AsBox {
        type ReturnType = cpp_utils::CppBox<::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch>;
        fn exec(self) -> cpp_utils::CppBox<::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch> {

          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_new_no_args() };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
      impl<'a> PhoneNumberMatchNewArgs for (libc::c_int,&'a ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,&'a ::phonenumber::i18n::phonenumbers::PhoneNumber,cpp_utils::AsBox) {
  type ReturnType = cpp_utils::CppBox<::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch>;
  fn exec(self, ) -> cpp_utils::CppBox<::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch> {
    let start = self.0;
let raw_string = self.1;
let number = self.2;
    let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_new_start_raw_string_number(start, raw_string as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, number as *const ::phonenumber::i18n::phonenumbers::PhoneNumber) };
unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }
}
      /// This trait represents a set of arguments accepted by [PhoneNumberMatch::to_string](../struct.PhoneNumberMatch.html#method.to_string) method.
      pub trait PhoneNumberMatchToStringArgs<'l0> {
        type ReturnType;
        fn exec(self,
                original_self: &'l0 ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch)
                -> Self::ReturnType;
      }
      impl<'l0> PhoneNumberMatchToStringArgs<'l0> for cpp_utils::AsBox {
        type ReturnType = cpp_utils::CppBox<::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef>;
        fn exec
          (self,
           original_self: &'l0 ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch)
           -> cpp_utils::CppBox<::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef> {

          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_ToString_as_ptr(original_self as *const ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch) };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
      impl<'l0> PhoneNumberMatchToStringArgs<'l0> for cpp_utils::AsStruct {
        type ReturnType = ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
        fn exec(self,
                original_self: &'l0 ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch)
                -> ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {

          {
            let mut object: ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef =
              unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_ToString_to_output(original_self as *const ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch, &mut object) }
            object
          }
        }
      }
    }

  }

}
