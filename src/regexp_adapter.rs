#[allow(unused_imports)]
use {libc, cpp_utils, std};

pub mod i18n {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  pub mod phonenumbers {
    #[allow(unused_imports)]
    use {libc, cpp_utils, std};

    /// C++ type: <span style='color: green;'>```i18n::phonenumbers::AbstractRegExpFactory```</span>.
    #[repr(C)]
    pub struct AbstractRegExpFactory {
      _buffer: [u8; 8],
    }

    impl ::NewUninitialized for AbstractRegExpFactory {
      unsafe fn new_uninitialized() -> AbstractRegExpFactory {
        AbstractRegExpFactory { _buffer: std::mem::uninitialized() }
      }
    }

    impl AbstractRegExpFactory {
      /// C++ method: <span style='color: green;'>```pure virtual i18n::phonenumbers::RegExpInput* i18n::phonenumbers::AbstractRegExpFactory::CreateInput(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& utf8_input) const```</span>
      ///
      ///
pub fn create_input(&self, utf8_input: &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> *mut ::regexp_adapter::i18n::phonenumbers::RegExpInput {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_AbstractRegExpFactory_CreateInput(self as *const ::regexp_adapter::i18n::phonenumbers::AbstractRegExpFactory, utf8_input as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
      }

      /// C++ method: <span style='color: green;'>```pure virtual i18n::phonenumbers::RegExp* i18n::phonenumbers::AbstractRegExpFactory::CreateRegExp(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& utf8_regexp) const```</span>
      ///
      ///
pub fn create_reg_exp(&self, utf8_regexp: &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> *mut ::regexp_adapter::i18n::phonenumbers::RegExp {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_AbstractRegExpFactory_CreateRegExp(self as *const ::regexp_adapter::i18n::phonenumbers::AbstractRegExpFactory, utf8_regexp as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
      }
    }

    impl Drop for AbstractRegExpFactory {
      /// C++ method: <span style='color: green;'>```virtual [destructor] void i18n::phonenumbers::AbstractRegExpFactory::~AbstractRegExpFactory()```</span>
      ///
      ///
      fn drop(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_AbstractRegExpFactory_destructor(self as *mut ::regexp_adapter::i18n::phonenumbers::AbstractRegExpFactory) }
      }
    }

    impl cpp_utils::CppDeletable for AbstractRegExpFactory {
      fn deleter() -> cpp_utils::Deleter<Self> {
        ::ffi::libphonenumber_sys_c_i18n_phonenumbers_AbstractRegExpFactory_delete
      }
    }

    /// C++ type: <span style='color: green;'>```i18n::phonenumbers::RegExp```</span>.
    #[repr(C)]
    pub struct RegExp {
      _buffer: [u8; 8],
    }

    impl ::NewUninitialized for RegExp {
      unsafe fn new_uninitialized() -> RegExp {
        RegExp { _buffer: std::mem::uninitialized() }
      }
    }

    impl RegExp {
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::RegExp::Consume```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn consume(&self, *mut ::regexp_adapter::i18n::phonenumbers::RegExpInput) -> bool```<br>
      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::RegExp::Consume(i18n::phonenumbers::RegExpInput* input_string) const```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn consume(&self, (*mut ::regexp_adapter::i18n::phonenumbers::RegExpInput, *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef)) -> bool```<br>
      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::RegExp::Consume(i18n::phonenumbers::RegExpInput* input_string, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* matched_string) const```</span>
      ///
      ///
      ///
      /// ## Variant 3
      ///
      /// Rust arguments: ```fn consume(&self, (*mut ::regexp_adapter::i18n::phonenumbers::RegExpInput, *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef)) -> bool```<br>
      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::RegExp::Consume(i18n::phonenumbers::RegExpInput* input_string, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* matched_string1, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* matched_string2) const```</span>
      ///
      ///
      ///
      /// ## Variant 4
      ///
      /// Rust arguments: ```fn consume(&self, (*mut ::regexp_adapter::i18n::phonenumbers::RegExpInput, *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef)) -> bool```<br>
      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::RegExp::Consume(i18n::phonenumbers::RegExpInput* input_string, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* matched_string1, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* matched_string2, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* matched_string3) const```</span>
      ///
      ///
      ///
      /// ## Variant 5
      ///
      /// Rust arguments: ```fn consume(&self, (*mut ::regexp_adapter::i18n::phonenumbers::RegExpInput, bool, *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef)) -> bool```<br>
      /// C++ method: <span style='color: green;'>```pure virtual bool i18n::phonenumbers::RegExp::Consume(i18n::phonenumbers::RegExpInput* input_string, bool anchor_at_start, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* matched_string1, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* matched_string2, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* matched_string3) const```</span>
      ///
      ///
      pub fn consume<'l0, Args>(&'l0 self, args: Args) -> bool
        where Args: overloading::RegExpConsumeArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::RegExp::FindAndConsume(i18n::phonenumbers::RegExpInput* input_string, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* matched_string) const```</span>
      ///
      ///
pub fn find_and_consume(&self, input_string: *mut ::regexp_adapter::i18n::phonenumbers::RegExpInput, matched_string: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_RegExp_FindAndConsume(self as *const ::regexp_adapter::i18n::phonenumbers::RegExp, input_string, matched_string) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::RegExp::FullMatch```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn full_match(&self, &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> bool```<br>
      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::RegExp::FullMatch(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& input_string) const```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn full_match(&self, (&::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef)) -> bool```<br>
      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::RegExp::FullMatch(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& input_string, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* matched_string) const```</span>
      ///
      ///
      pub fn full_match<'l0, Args>(&'l0 self, args: Args) -> bool
        where Args: overloading::RegExpFullMatchArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::RegExp::GlobalReplace(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* string_to_process, const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& replacement_string) const```</span>
      ///
      ///
pub fn global_replace(&self, string_to_process: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, replacement_string: &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_RegExp_GlobalReplace(self as *const ::regexp_adapter::i18n::phonenumbers::RegExp, string_to_process, replacement_string as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
      }

      /// C++ method: <span style='color: green;'>```pure virtual bool i18n::phonenumbers::RegExp::Match(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& input_string, bool full_match, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* matched_string) const```</span>
      ///
      ///
pub fn match_(&self, input_string: &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, full_match: bool, matched_string: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_RegExp_Match(self as *const ::regexp_adapter::i18n::phonenumbers::RegExp, input_string as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, full_match, matched_string) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::RegExp::PartialMatch```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn partial_match(&self, &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> bool```<br>
      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::RegExp::PartialMatch(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& input_string) const```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn partial_match(&self, (&::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef)) -> bool```<br>
      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::RegExp::PartialMatch(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& input_string, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* matched_string) const```</span>
      ///
      ///
      pub fn partial_match<'l0, Args>(&'l0 self, args: Args) -> bool
        where Args: overloading::RegExpPartialMatchArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::RegExp::Replace```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn replace(&self, (*mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef)) -> bool```<br>
      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::RegExp::Replace(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* string_to_process, const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& replacement_string) const```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn replace(&self, (*mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, bool, &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef)) -> bool```<br>
      /// C++ method: <span style='color: green;'>```pure virtual bool i18n::phonenumbers::RegExp::Replace(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* string_to_process, bool global, const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& replacement_string) const```</span>
      ///
      ///
      pub fn replace<'l0, Args>(&'l0 self, args: Args) -> bool
        where Args: overloading::RegExpReplaceArgs<'l0>
      {
        args.exec(self)
      }
    }

    impl Drop for RegExp {
      /// C++ method: <span style='color: green;'>```virtual [destructor] void i18n::phonenumbers::RegExp::~RegExp()```</span>
      ///
      ///
      fn drop(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_RegExp_destructor(self as *mut ::regexp_adapter::i18n::phonenumbers::RegExp) }
      }
    }

    impl cpp_utils::CppDeletable for RegExp {
      fn deleter() -> cpp_utils::Deleter<Self> {
        ::ffi::libphonenumber_sys_c_i18n_phonenumbers_RegExp_delete
      }
    }

    /// C++ type: <span style='color: green;'>```i18n::phonenumbers::RegExpInput```</span>.
    #[repr(C)]
    pub struct RegExpInput {
      _buffer: [u8; 8],
    }

    impl ::NewUninitialized for RegExpInput {
      unsafe fn new_uninitialized() -> RegExpInput {
        RegExpInput { _buffer: std::mem::uninitialized() }
      }
    }

    impl RegExpInput {
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::RegExpInput::ToString```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      /// Rust arguments: <br>1) ```fn to_string(&self, cpp_utils::AsBox) -> cpp_utils::CppBox<::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef>```<br>2) ```fn to_string(&self, cpp_utils::AsStruct) -> ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef```<br>
      /// C++ method: <span style='color: green;'>```pure virtual std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>> i18n::phonenumbers::RegExpInput::ToString() const```</span>
      ///
      ///
      pub fn to_string<'l0, Args>(&'l0 self, args: Args) -> Args::ReturnType
        where Args: overloading::RegExpInputToStringArgs<'l0>
      {
        args.exec(self)
      }
    }

    impl Drop for RegExpInput {
      /// C++ method: <span style='color: green;'>```virtual [destructor] void i18n::phonenumbers::RegExpInput::~RegExpInput()```</span>
      ///
      ///
      fn drop(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_RegExpInput_destructor(self as *mut ::regexp_adapter::i18n::phonenumbers::RegExpInput) }
      }
    }

    impl cpp_utils::CppDeletable for RegExpInput {
      fn deleter() -> cpp_utils::Deleter<Self> {
        ::ffi::libphonenumber_sys_c_i18n_phonenumbers_RegExpInput_delete
      }
    }

    pub mod overloading {
      #[allow(unused_imports)]
      use {libc, cpp_utils, std};

      /// This trait represents a set of arguments accepted by [RegExp::consume](../struct.RegExp.html#method.consume) method.
      pub trait RegExpConsumeArgs<'l0> {
        fn exec(self, original_self: &'l0 ::regexp_adapter::i18n::phonenumbers::RegExp) -> bool;
      }
      impl<'l0> RegExpConsumeArgs<'l0> for *mut ::regexp_adapter::i18n::phonenumbers::RegExpInput {
        fn exec(self, original_self: &'l0 ::regexp_adapter::i18n::phonenumbers::RegExp) -> bool {
          let input_string = self;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_RegExp_Consume_input_string(original_self as *const ::regexp_adapter::i18n::phonenumbers::RegExp, input_string) }
        }
      }
      impl<'l0> RegExpConsumeArgs<'l0> for (*mut ::regexp_adapter::i18n::phonenumbers::RegExpInput,bool,*mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,*mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,*mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {

  fn exec(self, original_self: &'l0 ::regexp_adapter::i18n::phonenumbers::RegExp) -> bool {
    let input_string = self.0;
let anchor_at_start = self.1;
let matched_string1 = self.2;
let matched_string2 = self.3;
let matched_string3 = self.4;
    unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_RegExp_Consume_input_string_anchor_at_start_matched_string1_matched_string2_matched_string3(original_self as *const ::regexp_adapter::i18n::phonenumbers::RegExp, input_string, anchor_at_start, matched_string1, matched_string2, matched_string3) }
  }
}
      impl<'l0> RegExpConsumeArgs<'l0> for (*mut ::regexp_adapter::i18n::phonenumbers::RegExpInput,*mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {

  fn exec(self, original_self: &'l0 ::regexp_adapter::i18n::phonenumbers::RegExp) -> bool {
    let input_string = self.0;
let matched_string = self.1;
    unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_RegExp_Consume_input_string_matched_string(original_self as *const ::regexp_adapter::i18n::phonenumbers::RegExp, input_string, matched_string) }
  }
}
      impl<'l0> RegExpConsumeArgs<'l0> for (*mut ::regexp_adapter::i18n::phonenumbers::RegExpInput,*mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,*mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {

  fn exec(self, original_self: &'l0 ::regexp_adapter::i18n::phonenumbers::RegExp) -> bool {
    let input_string = self.0;
let matched_string1 = self.1;
let matched_string2 = self.2;
    unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_RegExp_Consume_input_string_matched_string1_matched_string2(original_self as *const ::regexp_adapter::i18n::phonenumbers::RegExp, input_string, matched_string1, matched_string2) }
  }
}
      impl<'l0> RegExpConsumeArgs<'l0> for (*mut ::regexp_adapter::i18n::phonenumbers::RegExpInput,*mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,*mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,*mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {

  fn exec(self, original_self: &'l0 ::regexp_adapter::i18n::phonenumbers::RegExp) -> bool {
    let input_string = self.0;
let matched_string1 = self.1;
let matched_string2 = self.2;
let matched_string3 = self.3;
    unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_RegExp_Consume_input_string_matched_string1_matched_string2_matched_string3(original_self as *const ::regexp_adapter::i18n::phonenumbers::RegExp, input_string, matched_string1, matched_string2, matched_string3) }
  }
}
      /// This trait represents a set of arguments accepted by [RegExp::full_match](../struct.RegExp.html#method.full_match) method.
      pub trait RegExpFullMatchArgs<'l0> {
        fn exec(self, original_self: &'l0 ::regexp_adapter::i18n::phonenumbers::RegExp) -> bool;
      }
      impl<'l0> RegExpFullMatchArgs<'l0> for &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {

  fn exec(self, original_self: &'l0 ::regexp_adapter::i18n::phonenumbers::RegExp) -> bool {
    let input_string = self;
    unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_RegExp_FullMatch_input_string(original_self as *const ::regexp_adapter::i18n::phonenumbers::RegExp, input_string as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
  }
}
      impl<'l0> RegExpFullMatchArgs<'l0> for (&'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,*mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {

  fn exec(self, original_self: &'l0 ::regexp_adapter::i18n::phonenumbers::RegExp) -> bool {
    let input_string = self.0;
let matched_string = self.1;
    unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_RegExp_FullMatch_input_string_matched_string(original_self as *const ::regexp_adapter::i18n::phonenumbers::RegExp, input_string as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, matched_string) }
  }
}
      /// This trait represents a set of arguments accepted by [RegExpInput::to_string](../struct.RegExpInput.html#method.to_string) method.
      pub trait RegExpInputToStringArgs<'l0> {
        type ReturnType;
        fn exec(self, original_self: &'l0 ::regexp_adapter::i18n::phonenumbers::RegExpInput) -> Self::ReturnType;
      }
      impl<'l0> RegExpInputToStringArgs<'l0> for cpp_utils::AsBox {
        type ReturnType = cpp_utils::CppBox<::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef>;
        fn exec
          (self,
           original_self: &'l0 ::regexp_adapter::i18n::phonenumbers::RegExpInput)
           -> cpp_utils::CppBox<::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef> {

          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_RegExpInput_ToString_as_ptr(original_self as *const ::regexp_adapter::i18n::phonenumbers::RegExpInput) };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
      impl<'l0> RegExpInputToStringArgs<'l0> for cpp_utils::AsStruct {
        type ReturnType = ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
        fn exec(self,
                original_self: &'l0 ::regexp_adapter::i18n::phonenumbers::RegExpInput)
                -> ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {

          {
            let mut object: ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef =
              unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_RegExpInput_ToString_to_output(original_self as *const ::regexp_adapter::i18n::phonenumbers::RegExpInput, &mut object) }
            object
          }
        }
      }
      /// This trait represents a set of arguments accepted by [RegExp::partial_match](../struct.RegExp.html#method.partial_match) method.
      pub trait RegExpPartialMatchArgs<'l0> {
        fn exec(self, original_self: &'l0 ::regexp_adapter::i18n::phonenumbers::RegExp) -> bool;
      }
      impl<'l0> RegExpPartialMatchArgs<'l0> for &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {

  fn exec(self, original_self: &'l0 ::regexp_adapter::i18n::phonenumbers::RegExp) -> bool {
    let input_string = self;
    unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_RegExp_PartialMatch_input_string(original_self as *const ::regexp_adapter::i18n::phonenumbers::RegExp, input_string as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
  }
}
      impl<'l0> RegExpPartialMatchArgs<'l0> for (&'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,*mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {

  fn exec(self, original_self: &'l0 ::regexp_adapter::i18n::phonenumbers::RegExp) -> bool {
    let input_string = self.0;
let matched_string = self.1;
    unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_RegExp_PartialMatch_input_string_matched_string(original_self as *const ::regexp_adapter::i18n::phonenumbers::RegExp, input_string as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, matched_string) }
  }
}
      /// This trait represents a set of arguments accepted by [RegExp::replace](../struct.RegExp.html#method.replace) method.
      pub trait RegExpReplaceArgs<'l0> {
        fn exec(self, original_self: &'l0 ::regexp_adapter::i18n::phonenumbers::RegExp) -> bool;
      }
      impl<'l0> RegExpReplaceArgs<'l0> for (*mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,bool,&'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {

  fn exec(self, original_self: &'l0 ::regexp_adapter::i18n::phonenumbers::RegExp) -> bool {
    let string_to_process = self.0;
let global = self.1;
let replacement_string = self.2;
    unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_RegExp_Replace_string_to_process_global_replacement_string(original_self as *const ::regexp_adapter::i18n::phonenumbers::RegExp, string_to_process, global, replacement_string as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
  }
}
      impl<'l0> RegExpReplaceArgs<'l0> for (*mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,&'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {

  fn exec(self, original_self: &'l0 ::regexp_adapter::i18n::phonenumbers::RegExp) -> bool {
    let string_to_process = self.0;
let replacement_string = self.1;
    unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_RegExp_Replace_string_to_process_replacement_string(original_self as *const ::regexp_adapter::i18n::phonenumbers::RegExp, string_to_process, replacement_string as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
  }
}
    }

  }

}
