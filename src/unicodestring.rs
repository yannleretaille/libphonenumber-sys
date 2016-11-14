#[allow(unused_imports)]
use {libc, cpp_utils, std};

pub mod i18n {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  pub mod phonenumbers {
    #[allow(unused_imports)]
    use {libc, cpp_utils, std};

    /// C++ type: <span style='color: green;'>```i18n::phonenumbers::UnicodeString```</span>.
    #[repr(C)]
    pub struct UnicodeString {
      _buffer: [u8; 40],
    }

    impl ::NewUninitialized for UnicodeString {
      unsafe fn new_uninitialized() -> UnicodeString {
        UnicodeString { _buffer: std::mem::uninitialized() }
      }
    }

    //added
    impl cpp_utils::CppDeletable for UnicodeString {
      fn deleter() -> cpp_utils::Deleter<Self> { 
			unsafe extern "C" fn deleter_func(this_ptr: *mut UnicodeString) {
				drop(this_ptr);
			};
			deleter_func
		}
    }

    impl UnicodeString {
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeString::append```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn append(&mut self, &::unicodestring::i18n::phonenumbers::UnicodeString) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::UnicodeString::append(const i18n::phonenumbers::UnicodeString& unicode_string)```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn append(&mut self, libc::c_int) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::UnicodeString::append(int codepoint)```</span>
      ///
      ///
      pub fn append<'l0, Args>(&'l0 mut self, args: Args) -> ()
        where Args: overloading::UnicodeStringAppendArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeString::begin```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      /// Rust arguments: <br>1) ```fn begin(&self, cpp_utils::AsBox) -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator>```<br>2) ```fn begin(&self, cpp_utils::AsStruct) -> ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator```<br>
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText::const_iterator i18n::phonenumbers::UnicodeString::begin() const```</span>
      ///
      ///
      pub fn begin<'l0, Args>(&'l0 self, args: Args) -> Args::ReturnType
        where Args: overloading::UnicodeStringBeginArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeString::end```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      /// Rust arguments: <br>1) ```fn end(&self, cpp_utils::AsBox) -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator>```<br>2) ```fn end(&self, cpp_utils::AsStruct) -> ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator```<br>
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText::const_iterator i18n::phonenumbers::UnicodeString::end() const```</span>
      ///
      ///
      pub fn end<'l0, Args>(&'l0 self, args: Args) -> Args::ReturnType
        where Args: overloading::UnicodeStringEndArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```int i18n::phonenumbers::UnicodeString::indexOf(int codepoint) const```</span>
      ///
      ///
      pub fn index_of(&self, codepoint: libc::c_int) -> libc::c_int {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_indexOf(self as *const ::unicodestring::i18n::phonenumbers::UnicodeString, codepoint) }
      }

      /// C++ method: <span style='color: green;'>```int i18n::phonenumbers::UnicodeString::length() const```</span>
      ///
      ///
      pub fn length(&self) -> libc::c_int {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_length(self as *const ::unicodestring::i18n::phonenumbers::UnicodeString) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeString::UnicodeString```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: <br>1) ```fn new(cpp_utils::AsStruct) -> ::unicodestring::i18n::phonenumbers::UnicodeString```<br>2) ```fn new(cpp_utils::AsBox) -> cpp_utils::CppBox<::unicodestring::i18n::phonenumbers::UnicodeString>```<br>
      /// C++ method: <span style='color: green;'>```[constructor] void i18n::phonenumbers::UnicodeString::UnicodeString()```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: <br>1) ```fn new((*const libc::c_char, cpp_utils::AsStruct)) -> ::unicodestring::i18n::phonenumbers::UnicodeString```<br>2) ```fn new((*const libc::c_char, cpp_utils::AsBox)) -> cpp_utils::CppBox<::unicodestring::i18n::phonenumbers::UnicodeString>```<br>
      /// C++ method: <span style='color: green;'>```[constructor] void i18n::phonenumbers::UnicodeString::UnicodeString(const char* utf8)```</span>
      ///
      ///
      ///
      /// ## Variant 3
      ///
      /// Rust arguments: <br>1) ```fn new((&::unicodestring::i18n::phonenumbers::UnicodeString, cpp_utils::AsStruct)) -> ::unicodestring::i18n::phonenumbers::UnicodeString```<br>2) ```fn new((&::unicodestring::i18n::phonenumbers::UnicodeString, cpp_utils::AsBox)) -> cpp_utils::CppBox<::unicodestring::i18n::phonenumbers::UnicodeString>```<br>
      /// C++ method: <span style='color: green;'>```[constructor] void i18n::phonenumbers::UnicodeString::UnicodeString(const i18n::phonenumbers::UnicodeString& src)```</span>
      ///
      ///
      ///
      /// ## Variant 4
      ///
      /// Rust arguments: <br>1) ```fn new((libc::c_int, cpp_utils::AsStruct)) -> ::unicodestring::i18n::phonenumbers::UnicodeString```<br>2) ```fn new((libc::c_int, cpp_utils::AsBox)) -> cpp_utils::CppBox<::unicodestring::i18n::phonenumbers::UnicodeString>```<br>
      /// C++ method: <span style='color: green;'>```[constructor] void i18n::phonenumbers::UnicodeString::UnicodeString(int codepoint)```</span>
      ///
      ///
      pub fn new<Args>(args: Args) -> Args::ReturnType
        where Args: overloading::UnicodeStringNewArgs
      {
        args.exec()
      }
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeString& i18n::phonenumbers::UnicodeString::operator=(const i18n::phonenumbers::UnicodeString& src)```</span>
      ///
      ///
      pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                                 src: &'l1 ::unicodestring::i18n::phonenumbers::UnicodeString)
                                 -> &'l0 mut ::unicodestring::i18n::phonenumbers::UnicodeString {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_operator_assign(self as *mut ::unicodestring::i18n::phonenumbers::UnicodeString, src as *const ::unicodestring::i18n::phonenumbers::UnicodeString) };
        unsafe { &mut *ffi_result }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::UnicodeString::operator==(const i18n::phonenumbers::UnicodeString& rhs) const```</span>
      ///
      ///
      pub fn op_eq(&self, rhs: &::unicodestring::i18n::phonenumbers::UnicodeString) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_operator_eq(self as *const ::unicodestring::i18n::phonenumbers::UnicodeString, rhs as *const ::unicodestring::i18n::phonenumbers::UnicodeString) }
      }

      /// C++ method: <span style='color: green;'>```int i18n::phonenumbers::UnicodeString::operator[](int index) const```</span>
      ///
      ///
      pub fn op_index(&self, index: libc::c_int) -> libc::c_int {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_operator_index(self as *const ::unicodestring::i18n::phonenumbers::UnicodeString, index) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::UnicodeString::remove()```</span>
      ///
      ///
      pub fn remove(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_remove(self as *mut ::unicodestring::i18n::phonenumbers::UnicodeString) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::UnicodeString::replace(int start, int length, const i18n::phonenumbers::UnicodeString& src)```</span>
      ///
      ///
      pub fn replace(&mut self,
                     start: libc::c_int,
                     length: libc::c_int,
                     src: &::unicodestring::i18n::phonenumbers::UnicodeString) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_replace(self as *mut ::unicodestring::i18n::phonenumbers::UnicodeString, start, length, src as *const ::unicodestring::i18n::phonenumbers::UnicodeString) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::UnicodeString::setCharAt(int pos, int c)```</span>
      ///
      ///
      pub fn set_char_at(&mut self, pos: libc::c_int, c: libc::c_int) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_setCharAt(self as *mut ::unicodestring::i18n::phonenumbers::UnicodeString, pos, c) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::UnicodeString::setTo(const char* s, unsigned long len)```</span>
      ///
      ///
      pub fn set_to(&mut self, s: *const libc::c_char, len: libc::c_ulong) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_setTo(self as *mut ::unicodestring::i18n::phonenumbers::UnicodeString, s, len) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeString::tempSubString```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      /// Rust arguments: <br>1) ```fn temp_sub_string(&self, (libc::c_int, cpp_utils::AsBox)) -> cpp_utils::CppBox<::unicodestring::i18n::phonenumbers::UnicodeString>```<br>2) ```fn temp_sub_string(&self, (libc::c_int, libc::c_int, cpp_utils::AsBox)) -> cpp_utils::CppBox<::unicodestring::i18n::phonenumbers::UnicodeString>```<br>3) ```fn temp_sub_string(&self, (libc::c_int, cpp_utils::AsStruct)) -> ::unicodestring::i18n::phonenumbers::UnicodeString```<br>4) ```fn temp_sub_string(&self, (libc::c_int, libc::c_int, cpp_utils::AsStruct)) -> ::unicodestring::i18n::phonenumbers::UnicodeString```<br>
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeString i18n::phonenumbers::UnicodeString::tempSubString(int start, int length = ?) const```</span>
      ///
      ///
      pub fn temp_sub_string<'l0, Args>(&'l0 self, args: Args) -> Args::ReturnType
        where Args: overloading::UnicodeStringTempSubStringArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::UnicodeString::toUTF8String(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& out) const```</span>
      ///
      ///
pub fn to_utf_8_string(&self, out: &mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_toUTF8String(self as *const ::unicodestring::i18n::phonenumbers::UnicodeString, out as *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
      }
    }

    pub mod overloading {
      #[allow(unused_imports)]
      use {libc, cpp_utils, std};

      /// This trait represents a set of arguments accepted by [UnicodeString::append](../struct.UnicodeString.html#method.append) method.
      pub trait UnicodeStringAppendArgs<'l0> {
        fn exec(self, original_self: &'l0 mut ::unicodestring::i18n::phonenumbers::UnicodeString) -> ();
      }
      impl<'l0> UnicodeStringAppendArgs<'l0> for libc::c_int {
        fn exec(self, original_self: &'l0 mut ::unicodestring::i18n::phonenumbers::UnicodeString) -> () {
          let codepoint = self;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_append_codepoint(original_self as *mut ::unicodestring::i18n::phonenumbers::UnicodeString, codepoint) }
        }
      }
      impl<'l0> UnicodeStringAppendArgs<'l0> for &'l0 ::unicodestring::i18n::phonenumbers::UnicodeString {
        fn exec(self, original_self: &'l0 mut ::unicodestring::i18n::phonenumbers::UnicodeString) -> () {
          let unicode_string = self;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_append_unicode_string(original_self as *mut ::unicodestring::i18n::phonenumbers::UnicodeString, unicode_string as *const ::unicodestring::i18n::phonenumbers::UnicodeString) }
        }
      }
      /// This trait represents a set of arguments accepted by [UnicodeString::begin](../struct.UnicodeString.html#method.begin) method.
      pub trait UnicodeStringBeginArgs<'l0> {
        type ReturnType;
        fn exec(self, original_self: &'l0 ::unicodestring::i18n::phonenumbers::UnicodeString) -> Self::ReturnType;
      }
      impl<'l0> UnicodeStringBeginArgs<'l0> for cpp_utils::AsBox {
        type ReturnType = cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator>;
        fn exec(self,
                original_self: &'l0 ::unicodestring::i18n::phonenumbers::UnicodeString)
                -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator> {

          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_begin_as_ptr(original_self as *const ::unicodestring::i18n::phonenumbers::UnicodeString) };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
      impl<'l0> UnicodeStringBeginArgs<'l0> for cpp_utils::AsStruct {
        type ReturnType = ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator;
        fn exec(self,
                original_self: &'l0 ::unicodestring::i18n::phonenumbers::UnicodeString)
                -> ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator {

          {
            let mut object: ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator =
              unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_begin_to_output(original_self as *const ::unicodestring::i18n::phonenumbers::UnicodeString, &mut object) }
            object
          }
        }
      }
      /// This trait represents a set of arguments accepted by [UnicodeString::end](../struct.UnicodeString.html#method.end) method.
      pub trait UnicodeStringEndArgs<'l0> {
        type ReturnType;
        fn exec(self, original_self: &'l0 ::unicodestring::i18n::phonenumbers::UnicodeString) -> Self::ReturnType;
      }
      impl<'l0> UnicodeStringEndArgs<'l0> for cpp_utils::AsBox {
        type ReturnType = cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator>;
        fn exec(self,
                original_self: &'l0 ::unicodestring::i18n::phonenumbers::UnicodeString)
                -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator> {

          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_end_as_ptr(original_self as *const ::unicodestring::i18n::phonenumbers::UnicodeString) };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
      impl<'l0> UnicodeStringEndArgs<'l0> for cpp_utils::AsStruct {
        type ReturnType = ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator;
        fn exec(self,
                original_self: &'l0 ::unicodestring::i18n::phonenumbers::UnicodeString)
                -> ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator {

          {
            let mut object: ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator =
              unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_end_to_output(original_self as *const ::unicodestring::i18n::phonenumbers::UnicodeString, &mut object) }
            object
          }
        }
      }
      /// This trait represents a set of arguments accepted by [UnicodeString::new](../struct.UnicodeString.html#method.new) method.
      pub trait UnicodeStringNewArgs {
        type ReturnType;
        fn exec(self) -> Self::ReturnType;
      }
      impl UnicodeStringNewArgs for (libc::c_int, cpp_utils::AsStruct) {
        type ReturnType = ::unicodestring::i18n::phonenumbers::UnicodeString;
        fn exec(self) -> ::unicodestring::i18n::phonenumbers::UnicodeString {
          let codepoint = self.0;
          {
            let mut object: ::unicodestring::i18n::phonenumbers::UnicodeString =
              unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe {
              ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_constructor_codepoint(codepoint, &mut object)
            }
            object
          }
        }
      }
      impl UnicodeStringNewArgs for cpp_utils::AsStruct {
        type ReturnType = ::unicodestring::i18n::phonenumbers::UnicodeString;
        fn exec(self) -> ::unicodestring::i18n::phonenumbers::UnicodeString {

          {
            let mut object: ::unicodestring::i18n::phonenumbers::UnicodeString =
              unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_constructor_no_args(&mut object) }
            object
          }
        }
      }
      impl<'a> UnicodeStringNewArgs for (&'a ::unicodestring::i18n::phonenumbers::UnicodeString, cpp_utils::AsStruct) {
        type ReturnType = ::unicodestring::i18n::phonenumbers::UnicodeString;
        fn exec(self) -> ::unicodestring::i18n::phonenumbers::UnicodeString {
          let src = self.0;
          {
            let mut object: ::unicodestring::i18n::phonenumbers::UnicodeString =
              unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_constructor_src(src as *const ::unicodestring::i18n::phonenumbers::UnicodeString, &mut object) }
            object
          }
        }
      }
      impl UnicodeStringNewArgs for (*const libc::c_char, cpp_utils::AsStruct) {
        type ReturnType = ::unicodestring::i18n::phonenumbers::UnicodeString;
        fn exec(self) -> ::unicodestring::i18n::phonenumbers::UnicodeString {
          let utf8 = self.0;
          {
            let mut object: ::unicodestring::i18n::phonenumbers::UnicodeString =
              unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_constructor_utf8(utf8, &mut object) }
            object
          }
        }
      }
      impl UnicodeStringNewArgs for (libc::c_int, cpp_utils::AsBox) {
        type ReturnType = cpp_utils::CppBox<::unicodestring::i18n::phonenumbers::UnicodeString>;
        fn exec(self) -> cpp_utils::CppBox<::unicodestring::i18n::phonenumbers::UnicodeString> {
          let codepoint = self.0;
          let ffi_result =
            unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_new_codepoint(codepoint) };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
      impl UnicodeStringNewArgs for cpp_utils::AsBox {
        type ReturnType = cpp_utils::CppBox<::unicodestring::i18n::phonenumbers::UnicodeString>;
        fn exec(self) -> cpp_utils::CppBox<::unicodestring::i18n::phonenumbers::UnicodeString> {

          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_new_no_args() };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
      impl<'a> UnicodeStringNewArgs for (&'a ::unicodestring::i18n::phonenumbers::UnicodeString, cpp_utils::AsBox) {
        type ReturnType = cpp_utils::CppBox<::unicodestring::i18n::phonenumbers::UnicodeString>;
        fn exec(self) -> cpp_utils::CppBox<::unicodestring::i18n::phonenumbers::UnicodeString> {
          let src = self.0;
          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_new_src(src as *const ::unicodestring::i18n::phonenumbers::UnicodeString) };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
      impl UnicodeStringNewArgs for (*const libc::c_char, cpp_utils::AsBox) {
        type ReturnType = cpp_utils::CppBox<::unicodestring::i18n::phonenumbers::UnicodeString>;
        fn exec(self) -> cpp_utils::CppBox<::unicodestring::i18n::phonenumbers::UnicodeString> {
          let utf8 = self.0;
          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_new_utf8(utf8) };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
      /// This trait represents a set of arguments accepted by [UnicodeString::temp_sub_string](../struct.UnicodeString.html#method.temp_sub_string) method.
      pub trait UnicodeStringTempSubStringArgs<'l0> {
        type ReturnType;
        fn exec(self, original_self: &'l0 ::unicodestring::i18n::phonenumbers::UnicodeString) -> Self::ReturnType;
      }
      impl<'l0> UnicodeStringTempSubStringArgs<'l0> for (libc::c_int, cpp_utils::AsBox) {
        type ReturnType = cpp_utils::CppBox<::unicodestring::i18n::phonenumbers::UnicodeString>;
        fn exec(self,
                original_self: &'l0 ::unicodestring::i18n::phonenumbers::UnicodeString)
                -> cpp_utils::CppBox<::unicodestring::i18n::phonenumbers::UnicodeString> {
          let start = self.0;
          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_tempSubString_as_ptr_start(original_self as *const ::unicodestring::i18n::phonenumbers::UnicodeString, start) };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
      impl<'l0> UnicodeStringTempSubStringArgs<'l0> for (libc::c_int, libc::c_int, cpp_utils::AsBox) {
        type ReturnType = cpp_utils::CppBox<::unicodestring::i18n::phonenumbers::UnicodeString>;
        fn exec(self,
                original_self: &'l0 ::unicodestring::i18n::phonenumbers::UnicodeString)
                -> cpp_utils::CppBox<::unicodestring::i18n::phonenumbers::UnicodeString> {
          let start = self.0;
          let length = self.1;
          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_tempSubString_as_ptr_start_length(original_self as *const ::unicodestring::i18n::phonenumbers::UnicodeString, start, length) };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
      impl<'l0> UnicodeStringTempSubStringArgs<'l0> for (libc::c_int, cpp_utils::AsStruct) {
        type ReturnType = ::unicodestring::i18n::phonenumbers::UnicodeString;
        fn exec(self,
                original_self: &'l0 ::unicodestring::i18n::phonenumbers::UnicodeString)
                -> ::unicodestring::i18n::phonenumbers::UnicodeString {
          let start = self.0;
          {
            let mut object: ::unicodestring::i18n::phonenumbers::UnicodeString =
              unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_tempSubString_to_output_start(original_self as *const ::unicodestring::i18n::phonenumbers::UnicodeString, start, &mut object) }
            object
          }
        }
      }
      impl<'l0> UnicodeStringTempSubStringArgs<'l0> for (libc::c_int, libc::c_int, cpp_utils::AsStruct) {
        type ReturnType = ::unicodestring::i18n::phonenumbers::UnicodeString;
        fn exec(self,
                original_self: &'l0 ::unicodestring::i18n::phonenumbers::UnicodeString)
                -> ::unicodestring::i18n::phonenumbers::UnicodeString {
          let start = self.0;
          let length = self.1;
          {
            let mut object: ::unicodestring::i18n::phonenumbers::UnicodeString =
              unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_tempSubString_to_output_start_length(original_self as *const ::unicodestring::i18n::phonenumbers::UnicodeString, start, length, &mut object) }
            object
          }
        }
      }
    }

  }

}
