#[allow(unused_imports)]
use {libc, cpp_utils, std};

pub mod i18n {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  pub mod phonenumbers {
    #[allow(unused_imports)]
    use {libc, cpp_utils, std};

    /// C++ type: <span style='color: green;'>```i18n::phonenumbers::UnicodeText```</span>.
    #[repr(C)]
    pub struct UnicodeText {
      _buffer: [u8; 24],
    }

    impl ::NewUninitialized for UnicodeText {
      unsafe fn new_uninitialized() -> UnicodeText {
        UnicodeText { _buffer: std::mem::uninitialized() }
      }
    }

    impl UnicodeText {
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText::append```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn append(&mut self, &'l1 ::unicodetext::i18n::phonenumbers::UnicodeText) -> &'l0 mut ::unicodetext::i18n::phonenumbers::UnicodeText```<br>
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText& i18n::phonenumbers::UnicodeText::append(const i18n::phonenumbers::UnicodeText& source)```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn append(&mut self, (&'l1 ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, &'l2 ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator)) -> &'l0 mut ::unicodetext::i18n::phonenumbers::UnicodeText```<br>
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText& i18n::phonenumbers::UnicodeText::append(const i18n::phonenumbers::UnicodeText::const_iterator& first, const i18n::phonenumbers::UnicodeText::const_iterator& last)```</span>
      ///
      ///
      pub fn append<'l0, Args>(&'l0 mut self, args: Args) -> &'l0 mut ::unicodetext::i18n::phonenumbers::UnicodeText
        where Args: overloading::UnicodeTextAppendArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText& i18n::phonenumbers::UnicodeText::assign(const i18n::phonenumbers::UnicodeText& src)```</span>
      ///
      ///
      pub fn assign<'l0, 'l1>(&'l0 mut self,
                              src: &'l1 ::unicodetext::i18n::phonenumbers::UnicodeText)
                              -> &'l0 mut ::unicodetext::i18n::phonenumbers::UnicodeText {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_assign(self as *mut ::unicodetext::i18n::phonenumbers::UnicodeText, src as *const ::unicodetext::i18n::phonenumbers::UnicodeText) };
        unsafe { &mut *ffi_result }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText::begin```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      /// Rust arguments: <br>1) ```fn begin(&self, cpp_utils::AsBox) -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator>```<br>2) ```fn begin(&self, cpp_utils::AsStruct) -> ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator```<br>
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText::const_iterator i18n::phonenumbers::UnicodeText::begin() const```</span>
      ///
      ///
      pub fn begin<'l0, Args>(&'l0 self, args: Args) -> Args::ReturnType
        where Args: overloading::UnicodeTextBeginArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::UnicodeText::clear()```</span>
      ///
      ///
      pub fn clear(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_clear(self as *mut ::unicodetext::i18n::phonenumbers::UnicodeText) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText& i18n::phonenumbers::UnicodeText::Copy(const i18n::phonenumbers::UnicodeText& src)```</span>
      ///
      ///
      pub fn copy<'l0, 'l1>(&'l0 mut self,
                            src: &'l1 ::unicodetext::i18n::phonenumbers::UnicodeText)
                            -> &'l0 mut ::unicodetext::i18n::phonenumbers::UnicodeText {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_Copy(self as *mut ::unicodetext::i18n::phonenumbers::UnicodeText, src as *const ::unicodetext::i18n::phonenumbers::UnicodeText) };
        unsafe { &mut *ffi_result }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText& i18n::phonenumbers::UnicodeText::CopyUTF8(const char* utf8_buffer, int byte_length)```</span>
      ///
      ///
      pub fn copy_u_t_f8<'l0>(&'l0 mut self,
                              utf8_buffer: *const libc::c_char,
                              byte_length: libc::c_int)
                              -> &'l0 mut ::unicodetext::i18n::phonenumbers::UnicodeText {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_CopyUTF8(self as *mut ::unicodetext::i18n::phonenumbers::UnicodeText, utf8_buffer, byte_length) };
        unsafe { &mut *ffi_result }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText::DebugString```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      /// Rust arguments: <br>1) ```fn debug_string(&self, cpp_utils::AsBox) -> cpp_utils::CppBox<::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef>```<br>2) ```fn debug_string(&self, cpp_utils::AsStruct) -> ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef```<br>
      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>> i18n::phonenumbers::UnicodeText::DebugString() const```</span>
      ///
      ///
      pub fn debug_string<'l0, Args>(&'l0 self, args: Args) -> Args::ReturnType
        where Args: overloading::UnicodeTextDebugStringArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::UnicodeText::empty()```</span>
      ///
      ///
      pub fn empty(&mut self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_empty(self as *mut ::unicodetext::i18n::phonenumbers::UnicodeText) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText::end```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      /// Rust arguments: <br>1) ```fn end(&self, cpp_utils::AsBox) -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator>```<br>2) ```fn end(&self, cpp_utils::AsStruct) -> ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator```<br>
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText::const_iterator i18n::phonenumbers::UnicodeText::end() const```</span>
      ///
      ///
      pub fn end<'l0, Args>(&'l0 self, args: Args) -> Args::ReturnType
        where Args: overloading::UnicodeTextEndArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText::find```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: <br>1) ```fn find(&self, (&::unicodetext::i18n::phonenumbers::UnicodeText, cpp_utils::AsBox)) -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator>```<br>2) ```fn find(&self, (&::unicodetext::i18n::phonenumbers::UnicodeText, cpp_utils::AsStruct)) -> ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator```<br>
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText::const_iterator i18n::phonenumbers::UnicodeText::find(const i18n::phonenumbers::UnicodeText& look) const```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: <br>1) ```fn find(&self, (&::unicodetext::i18n::phonenumbers::UnicodeText, &::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, cpp_utils::AsBox)) -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator>```<br>2) ```fn find(&self, (&::unicodetext::i18n::phonenumbers::UnicodeText, &::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, cpp_utils::AsStruct)) -> ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator```<br>
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText::const_iterator i18n::phonenumbers::UnicodeText::find(const i18n::phonenumbers::UnicodeText& look, i18n::phonenumbers::UnicodeText::const_iterator start_pos) const```</span>
      ///
      ///
      pub fn find<'l0, Args>(&'l0 self, args: Args) -> Args::ReturnType
        where Args: overloading::UnicodeTextFindArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::UnicodeText::HasReplacementChar() const```</span>
      ///
      ///
      pub fn has_replacement_char(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_HasReplacementChar(self as *const ::unicodetext::i18n::phonenumbers::UnicodeText) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText::MakeIterator```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      /// Rust arguments: <br>1) ```fn make_iterator(&self, (*const libc::c_char, cpp_utils::AsBox)) -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator>```<br>2) ```fn make_iterator(&self, (*const libc::c_char, cpp_utils::AsStruct)) -> ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator```<br>
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText::const_iterator i18n::phonenumbers::UnicodeText::MakeIterator(const char* p) const```</span>
      ///
      ///
      pub fn make_iterator<'l0, Args>(&'l0 self, args: Args) -> Args::ReturnType
        where Args: overloading::UnicodeTextMakeIteratorArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText::UnicodeText```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: <br>1) ```fn new(cpp_utils::AsStruct) -> ::unicodetext::i18n::phonenumbers::UnicodeText```<br>2) ```fn new(cpp_utils::AsBox) -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::UnicodeText>```<br>
      /// C++ method: <span style='color: green;'>```[constructor] void i18n::phonenumbers::UnicodeText::UnicodeText()```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: <br>1) ```fn new((&::unicodetext::i18n::phonenumbers::UnicodeText, cpp_utils::AsStruct)) -> ::unicodetext::i18n::phonenumbers::UnicodeText```<br>2) ```fn new((&::unicodetext::i18n::phonenumbers::UnicodeText, cpp_utils::AsBox)) -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::UnicodeText>```<br>
      /// C++ method: <span style='color: green;'>```[constructor] void i18n::phonenumbers::UnicodeText::UnicodeText(const i18n::phonenumbers::UnicodeText& src)```</span>
      ///
      ///
      ///
      /// ## Variant 3
      ///
      /// Rust arguments: <br>1) ```fn new((&::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, &::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, cpp_utils::AsStruct)) -> ::unicodetext::i18n::phonenumbers::UnicodeText```<br>2) ```fn new((&::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, &::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, cpp_utils::AsBox)) -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::UnicodeText>```<br>
      /// C++ method: <span style='color: green;'>```[constructor] void i18n::phonenumbers::UnicodeText::UnicodeText(const i18n::phonenumbers::UnicodeText::const_iterator& first, const i18n::phonenumbers::UnicodeText::const_iterator& last)```</span>
      ///
      ///
      pub fn new<Args>(args: Args) -> Args::ReturnType
        where Args: overloading::UnicodeTextNewArgs
      {
        args.exec()
      }
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText& i18n::phonenumbers::UnicodeText::operator=(const i18n::phonenumbers::UnicodeText& src)```</span>
      ///
      ///
      pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                                 src: &'l1 ::unicodetext::i18n::phonenumbers::UnicodeText)
                                 -> &'l0 mut ::unicodetext::i18n::phonenumbers::UnicodeText {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_operator_assign(self as *mut ::unicodetext::i18n::phonenumbers::UnicodeText, src as *const ::unicodetext::i18n::phonenumbers::UnicodeText) };
        unsafe { &mut *ffi_result }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText::PointTo```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn point_to(&mut self, &'l1 ::unicodetext::i18n::phonenumbers::UnicodeText) -> &'l0 mut ::unicodetext::i18n::phonenumbers::UnicodeText```<br>
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText& i18n::phonenumbers::UnicodeText::PointTo(const i18n::phonenumbers::UnicodeText& src)```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn point_to(&mut self, (&'l1 ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, &'l2 ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator)) -> &'l0 mut ::unicodetext::i18n::phonenumbers::UnicodeText```<br>
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText& i18n::phonenumbers::UnicodeText::PointTo(const i18n::phonenumbers::UnicodeText::const_iterator& first, const i18n::phonenumbers::UnicodeText::const_iterator& last)```</span>
      ///
      ///
      pub fn point_to<'l0, Args>(&'l0 mut self, args: Args) -> &'l0 mut ::unicodetext::i18n::phonenumbers::UnicodeText
        where Args: overloading::UnicodeTextPointToArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText& i18n::phonenumbers::UnicodeText::PointToUTF8(const char* utf8_buffer, int byte_length)```</span>
      ///
      ///
      pub fn point_to_u_t_f8<'l0>(&'l0 mut self,
                                  utf8_buffer: *const libc::c_char,
                                  byte_length: libc::c_int)
                                  -> &'l0 mut ::unicodetext::i18n::phonenumbers::UnicodeText {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_PointToUTF8(self as *mut ::unicodetext::i18n::phonenumbers::UnicodeText, utf8_buffer, byte_length) };
        unsafe { &mut *ffi_result }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::UnicodeText::push_back(int codepoint)```</span>
      ///
      ///
      pub fn push_back(&mut self, codepoint: libc::c_int) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_push_back(self as *mut ::unicodetext::i18n::phonenumbers::UnicodeText, codepoint) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText::rbegin```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      /// Rust arguments: <br>1) ```fn rbegin(&self, cpp_utils::AsBox) -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::unicode_text::ConstReverseIterator>```<br>2) ```fn rbegin(&self, cpp_utils::AsStruct) -> ::unicodetext::i18n::phonenumbers::unicode_text::ConstReverseIterator```<br>
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText::const_reverse_iterator i18n::phonenumbers::UnicodeText::rbegin() const```</span>
      ///
      ///
      pub fn rbegin<'l0, Args>(&'l0 self, args: Args) -> Args::ReturnType
        where Args: overloading::UnicodeTextRbeginArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText::rend```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      /// Rust arguments: <br>1) ```fn rend(&self, cpp_utils::AsBox) -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::unicode_text::ConstReverseIterator>```<br>2) ```fn rend(&self, cpp_utils::AsStruct) -> ::unicodetext::i18n::phonenumbers::unicode_text::ConstReverseIterator```<br>
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText::const_reverse_iterator i18n::phonenumbers::UnicodeText::rend() const```</span>
      ///
      ///
      pub fn rend<'l0, Args>(&'l0 self, args: Args) -> Args::ReturnType
        where Args: overloading::UnicodeTextRendArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```int i18n::phonenumbers::UnicodeText::size() const```</span>
      ///
      ///
      pub fn size(&self) -> libc::c_int {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_size(self as *const ::unicodetext::i18n::phonenumbers::UnicodeText) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText& i18n::phonenumbers::UnicodeText::TakeOwnershipOfUTF8(char* utf8_buffer, int byte_length, int byte_capacity)```</span>
      ///
      ///
      pub fn take_ownership_of_u_t_f8<'l0>(&'l0 mut self,
                                           utf8_buffer: *mut libc::c_char,
                                           byte_length: libc::c_int,
                                           byte_capacity: libc::c_int)
                                           -> &'l0 mut ::unicodetext::i18n::phonenumbers::UnicodeText {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_TakeOwnershipOfUTF8(self as *mut ::unicodetext::i18n::phonenumbers::UnicodeText, utf8_buffer, byte_length, byte_capacity) };
        unsafe { &mut *ffi_result }
      }

      /// C++ method: <span style='color: green;'>```int i18n::phonenumbers::UnicodeText::utf8_capacity() const```</span>
      ///
      ///
      pub fn utf8_capacity(&self) -> libc::c_int {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_utf8_capacity(self as *const ::unicodetext::i18n::phonenumbers::UnicodeText) }
      }

      /// C++ method: <span style='color: green;'>```const char* i18n::phonenumbers::UnicodeText::utf8_data() const```</span>
      ///
      ///
      pub fn utf8_data(&self) -> *const libc::c_char {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_utf8_data(self as *const ::unicodetext::i18n::phonenumbers::UnicodeText) }
      }

      /// C++ method: <span style='color: green;'>```int i18n::phonenumbers::UnicodeText::utf8_length() const```</span>
      ///
      ///
      pub fn utf8_length(&self) -> libc::c_int {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_utf8_length(self as *const ::unicodetext::i18n::phonenumbers::UnicodeText) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText::UTF8Substring```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      /// Rust arguments: <br>1) ```fn utf_8_substring((&::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, &::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, cpp_utils::AsBox)) -> cpp_utils::CppBox<::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef>```<br>2) ```fn utf_8_substring((&::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, &::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, cpp_utils::AsStruct)) -> ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef```<br>
      /// C++ method: <span style='color: green;'>```static std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>> i18n::phonenumbers::UnicodeText::UTF8Substring(const i18n::phonenumbers::UnicodeText::const_iterator& first, const i18n::phonenumbers::UnicodeText::const_iterator& last)```</span>
      ///
      ///
      pub fn utf_8_substring<Args>(args: Args) -> Args::ReturnType
        where Args: overloading::UnicodeTextUtf8SubstringArgs
      {
        args.exec()
      }
    }

    impl Drop for UnicodeText {
      /// C++ method: <span style='color: green;'>```[destructor] void i18n::phonenumbers::UnicodeText::~UnicodeText()```</span>
      ///
      ///
      fn drop(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_destructor(self as *mut ::unicodetext::i18n::phonenumbers::UnicodeText) }
      }
    }

    impl cpp_utils::CppDeletable for UnicodeText {
      fn deleter() -> cpp_utils::Deleter<Self> {
        ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_delete
      }
    }

    /// C++ method: <span style='color: green;'>```i18n::phonenumbers::MakeUnicodeTextAcceptingOwnership```</span>
    ///
    /// This is an overloaded function. Available variants:
    ///
    /// Rust arguments: <br>1) ```fn make_unicode_text_accepting_ownership((*mut libc::c_char, libc::c_int, libc::c_int, cpp_utils::AsBox)) -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::UnicodeText>```<br>2) ```fn make_unicode_text_accepting_ownership((*mut libc::c_char, libc::c_int, libc::c_int, cpp_utils::AsStruct)) -> ::unicodetext::i18n::phonenumbers::UnicodeText```<br>
    /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText i18n::phonenumbers::MakeUnicodeTextAcceptingOwnership(char* utf8_buffer, int byte_length, int byte_capacity)```</span>
    ///
    ///
    pub fn make_unicode_text_accepting_ownership<Args>(args: Args) -> Args::ReturnType
      where Args: overloading::MakeUnicodeTextAcceptingOwnershipArgs
    {
      args.exec()
    }
    /// C++ method: <span style='color: green;'>```i18n::phonenumbers::MakeUnicodeTextWithoutAcceptingOwnership```</span>
    ///
    /// This is an overloaded function. Available variants:
    ///
    /// Rust arguments: <br>1) ```fn make_unicode_text_without_accepting_ownership((*const libc::c_char, libc::c_int, cpp_utils::AsBox)) -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::UnicodeText>```<br>2) ```fn make_unicode_text_without_accepting_ownership((*const libc::c_char, libc::c_int, cpp_utils::AsStruct)) -> ::unicodetext::i18n::phonenumbers::UnicodeText```<br>
    /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText i18n::phonenumbers::MakeUnicodeTextWithoutAcceptingOwnership(const char* utf8_buffer, int byte_length)```</span>
    ///
    ///
    pub fn make_unicode_text_without_accepting_ownership<Args>(args: Args) -> Args::ReturnType
      where Args: overloading::MakeUnicodeTextWithoutAcceptingOwnershipArgs
    {
      args.exec()
    }
    /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::UnicodeTextRangeIsEmpty(const std::pair<i18n::phonenumbers::UnicodeText::const_iterator, i18n::phonenumbers::UnicodeText::const_iterator>& r)```</span>
    ///
    ///
    pub fn unicode_text_range_is_empty(r: &::stl_pair::std::PairConstIteratorRefConstIteratorRef) -> bool {
      unsafe { ::ffi::libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UnicodeTextRangeIsEmpty(r as *const ::stl_pair::std::PairConstIteratorRefConstIteratorRef) }
    }

    /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeTextToUTF8```</span>
    ///
    /// This is an overloaded function. Available variants:
    ///
    /// Rust arguments: <br>1) ```fn unicode_text_to_u_t_f8((&::unicodetext::i18n::phonenumbers::UnicodeText, cpp_utils::AsBox)) -> cpp_utils::CppBox<::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef>```<br>2) ```fn unicode_text_to_u_t_f8((&::unicodetext::i18n::phonenumbers::UnicodeText, cpp_utils::AsStruct)) -> ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef```<br>
    /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>> i18n::phonenumbers::UnicodeTextToUTF8(const i18n::phonenumbers::UnicodeText& t)```</span>
    ///
    ///
    pub fn unicode_text_to_u_t_f8<Args>(args: Args) -> Args::ReturnType
      where Args: overloading::UnicodeTextToUTF8Args
    {
      args.exec()
    }
    /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UTF8ToUnicodeText```</span>
    ///
    /// This is an overloaded function. Available variants:
    ///
    ///
    ///
    /// ## Variant 1
    ///
    /// Rust arguments: <br>1) ```fn utf_8_to_unicode_text((*const libc::c_char, libc::c_int, cpp_utils::AsBox)) -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::UnicodeText>```<br>2) ```fn utf_8_to_unicode_text((*const libc::c_char, libc::c_int, cpp_utils::AsStruct)) -> ::unicodetext::i18n::phonenumbers::UnicodeText```<br>
    /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText i18n::phonenumbers::UTF8ToUnicodeText(const char* utf8_buf, int len)```</span>
    ///
    ///
    ///
    /// ## Variant 2
    ///
    /// Rust arguments: <br>1) ```fn utf_8_to_unicode_text((*const libc::c_char, libc::c_int, bool, cpp_utils::AsBox)) -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::UnicodeText>```<br>2) ```fn utf_8_to_unicode_text((*const libc::c_char, libc::c_int, bool, cpp_utils::AsStruct)) -> ::unicodetext::i18n::phonenumbers::UnicodeText```<br>
    /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText i18n::phonenumbers::UTF8ToUnicodeText(const char* utf8_buf, int len, bool do_copy)```</span>
    ///
    ///
    ///
    /// ## Variant 3
    ///
    /// Rust arguments: <br>1) ```fn utf_8_to_unicode_text((&::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, cpp_utils::AsBox)) -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::UnicodeText>```<br>2) ```fn utf_8_to_unicode_text((&::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, cpp_utils::AsStruct)) -> ::unicodetext::i18n::phonenumbers::UnicodeText```<br>
    /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText i18n::phonenumbers::UTF8ToUnicodeText(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& utf8_string)```</span>
    ///
    ///
    ///
    /// ## Variant 4
    ///
    /// Rust arguments: <br>1) ```fn utf_8_to_unicode_text((&::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, bool, cpp_utils::AsBox)) -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::UnicodeText>```<br>2) ```fn utf_8_to_unicode_text((&::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, bool, cpp_utils::AsStruct)) -> ::unicodetext::i18n::phonenumbers::UnicodeText```<br>
    /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText i18n::phonenumbers::UTF8ToUnicodeText(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& utf_string, bool do_copy)```</span>
    ///
    ///
    pub fn utf_8_to_unicode_text<Args>(args: Args) -> Args::ReturnType
      where Args: overloading::Utf8ToUnicodeTextArgs
    {
      args.exec()
    }
    pub mod overloading {
      #[allow(unused_imports)]
      use {libc, cpp_utils, std};

      /// This trait represents a set of arguments accepted by [make_unicode_text_accepting_ownership](../fn.make_unicode_text_accepting_ownership.html) method.
      pub trait MakeUnicodeTextAcceptingOwnershipArgs {
        type ReturnType;
        fn exec(self) -> Self::ReturnType;
      }
      impl MakeUnicodeTextAcceptingOwnershipArgs for (*mut libc::c_char, libc::c_int, libc::c_int, cpp_utils::AsBox) {
        type ReturnType = cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::UnicodeText>;
        fn exec(self) -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::UnicodeText> {
          let utf8_buffer = self.0;
          let byte_length = self.1;
          let byte_capacity = self.2;
          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_MakeUnicodeTextAcceptingOwnership_as_ptr(utf8_buffer, byte_length, byte_capacity) };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
      impl MakeUnicodeTextAcceptingOwnershipArgs for (*mut libc::c_char, libc::c_int, libc::c_int, cpp_utils::AsStruct) {
        type ReturnType = ::unicodetext::i18n::phonenumbers::UnicodeText;
        fn exec(self) -> ::unicodetext::i18n::phonenumbers::UnicodeText {
          let utf8_buffer = self.0;
          let byte_length = self.1;
          let byte_capacity = self.2;
          {
            let mut object: ::unicodetext::i18n::phonenumbers::UnicodeText =
              unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe { ::ffi::libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_MakeUnicodeTextAcceptingOwnership_to_output(utf8_buffer, byte_length, byte_capacity, &mut object) }
            object
          }
        }
      }
      /// This trait represents a set of arguments accepted by [make_unicode_text_without_accepting_ownership](../fn.make_unicode_text_without_accepting_ownership.html) method.
      pub trait MakeUnicodeTextWithoutAcceptingOwnershipArgs {
        type ReturnType;
        fn exec(self) -> Self::ReturnType;
      }
      impl MakeUnicodeTextWithoutAcceptingOwnershipArgs for (*const libc::c_char, libc::c_int, cpp_utils::AsBox) {
        type ReturnType = cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::UnicodeText>;
        fn exec(self) -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::UnicodeText> {
          let utf8_buffer = self.0;
          let byte_length = self.1;
          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_MakeUnicodeTextWithoutAcceptingOwnership_as_ptr(utf8_buffer, byte_length) };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
      impl MakeUnicodeTextWithoutAcceptingOwnershipArgs for (*const libc::c_char, libc::c_int, cpp_utils::AsStruct) {
        type ReturnType = ::unicodetext::i18n::phonenumbers::UnicodeText;
        fn exec(self) -> ::unicodetext::i18n::phonenumbers::UnicodeText {
          let utf8_buffer = self.0;
          let byte_length = self.1;
          {
            let mut object: ::unicodetext::i18n::phonenumbers::UnicodeText =
              unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe { ::ffi::libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_MakeUnicodeTextWithoutAcceptingOwnership_to_output(utf8_buffer, byte_length, &mut object) }
            object
          }
        }
      }
      /// This trait represents a set of arguments accepted by [UnicodeText::append](../struct.UnicodeText.html#method.append) method.
      pub trait UnicodeTextAppendArgs<'l0> {
        fn exec(self,
                original_self: &'l0 mut ::unicodetext::i18n::phonenumbers::UnicodeText)
                -> &'l0 mut ::unicodetext::i18n::phonenumbers::UnicodeText;
      }
      impl<'l0> UnicodeTextAppendArgs<'l0> for (&'l0 ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator,&'l0 ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator) {

  fn exec(self, original_self: &'l0 mut ::unicodetext::i18n::phonenumbers::UnicodeText) -> &'l0 mut ::unicodetext::i18n::phonenumbers::UnicodeText {
    let first = self.0;
let last = self.1;
    let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_append_first_last(original_self as *mut ::unicodetext::i18n::phonenumbers::UnicodeText, first as *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, last as *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator) };
unsafe { &mut *ffi_result }
  }
}
      impl<'l0> UnicodeTextAppendArgs<'l0> for &'l0 ::unicodetext::i18n::phonenumbers::UnicodeText {
        fn exec(self,
                original_self: &'l0 mut ::unicodetext::i18n::phonenumbers::UnicodeText)
                -> &'l0 mut ::unicodetext::i18n::phonenumbers::UnicodeText {
          let source = self;
          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_append_source(original_self as *mut ::unicodetext::i18n::phonenumbers::UnicodeText, source as *const ::unicodetext::i18n::phonenumbers::UnicodeText) };
          unsafe { &mut *ffi_result }
        }
      }
      /// This trait represents a set of arguments accepted by [UnicodeText::begin](../struct.UnicodeText.html#method.begin) method.
      pub trait UnicodeTextBeginArgs<'l0> {
        type ReturnType;
        fn exec(self, original_self: &'l0 ::unicodetext::i18n::phonenumbers::UnicodeText) -> Self::ReturnType;
      }
      impl<'l0> UnicodeTextBeginArgs<'l0> for cpp_utils::AsBox {
        type ReturnType = cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator>;
        fn exec(self,
                original_self: &'l0 ::unicodetext::i18n::phonenumbers::UnicodeText)
                -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator> {

          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_begin_as_ptr(original_self as *const ::unicodetext::i18n::phonenumbers::UnicodeText) };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
      impl<'l0> UnicodeTextBeginArgs<'l0> for cpp_utils::AsStruct {
        type ReturnType = ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator;
        fn exec(self,
                original_self: &'l0 ::unicodetext::i18n::phonenumbers::UnicodeText)
                -> ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator {

          {
            let mut object: ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator =
              unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_begin_to_output(original_self as *const ::unicodetext::i18n::phonenumbers::UnicodeText, &mut object) }
            object
          }
        }
      }
      /// This trait represents a set of arguments accepted by [UnicodeText::debug_string](../struct.UnicodeText.html#method.debug_string) method.
      pub trait UnicodeTextDebugStringArgs<'l0> {
        type ReturnType;
        fn exec(self, original_self: &'l0 ::unicodetext::i18n::phonenumbers::UnicodeText) -> Self::ReturnType;
      }
      impl<'l0> UnicodeTextDebugStringArgs<'l0> for cpp_utils::AsBox {
        type ReturnType = cpp_utils::CppBox<::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef>;
        fn exec
          (self,
           original_self: &'l0 ::unicodetext::i18n::phonenumbers::UnicodeText)
           -> cpp_utils::CppBox<::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef> {

          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_DebugString_as_ptr(original_self as *const ::unicodetext::i18n::phonenumbers::UnicodeText) };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
      impl<'l0> UnicodeTextDebugStringArgs<'l0> for cpp_utils::AsStruct {
        type ReturnType = ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
        fn exec(self,
                original_self: &'l0 ::unicodetext::i18n::phonenumbers::UnicodeText)
                -> ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {

          {
            let mut object: ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef =
              unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_DebugString_to_output(original_self as *const ::unicodetext::i18n::phonenumbers::UnicodeText, &mut object) }
            object
          }
        }
      }
      /// This trait represents a set of arguments accepted by [UnicodeText::end](../struct.UnicodeText.html#method.end) method.
      pub trait UnicodeTextEndArgs<'l0> {
        type ReturnType;
        fn exec(self, original_self: &'l0 ::unicodetext::i18n::phonenumbers::UnicodeText) -> Self::ReturnType;
      }
      impl<'l0> UnicodeTextEndArgs<'l0> for cpp_utils::AsBox {
        type ReturnType = cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator>;
        fn exec(self,
                original_self: &'l0 ::unicodetext::i18n::phonenumbers::UnicodeText)
                -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator> {

          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_end_as_ptr(original_self as *const ::unicodetext::i18n::phonenumbers::UnicodeText) };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
      impl<'l0> UnicodeTextEndArgs<'l0> for cpp_utils::AsStruct {
        type ReturnType = ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator;
        fn exec(self,
                original_self: &'l0 ::unicodetext::i18n::phonenumbers::UnicodeText)
                -> ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator {

          {
            let mut object: ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator =
              unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_end_to_output(original_self as *const ::unicodetext::i18n::phonenumbers::UnicodeText, &mut object) }
            object
          }
        }
      }
      /// This trait represents a set of arguments accepted by [UnicodeText::find](../struct.UnicodeText.html#method.find) method.
      pub trait UnicodeTextFindArgs<'l0> {
        type ReturnType;
        fn exec(self, original_self: &'l0 ::unicodetext::i18n::phonenumbers::UnicodeText) -> Self::ReturnType;
      }
      impl<'l0> UnicodeTextFindArgs<'l0> for (&'l0 ::unicodetext::i18n::phonenumbers::UnicodeText, cpp_utils::AsBox) {
        type ReturnType = cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator>;
        fn exec(self,
                original_self: &'l0 ::unicodetext::i18n::phonenumbers::UnicodeText)
                -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator> {
          let look = self.0;
          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_find_as_ptr_look(original_self as *const ::unicodetext::i18n::phonenumbers::UnicodeText, look as *const ::unicodetext::i18n::phonenumbers::UnicodeText) };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
      impl<'l0> UnicodeTextFindArgs<'l0> for (&'l0 ::unicodetext::i18n::phonenumbers::UnicodeText,&'l0 ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator,cpp_utils::AsBox) {
  type ReturnType = cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator>;
  fn exec(self, original_self: &'l0 ::unicodetext::i18n::phonenumbers::UnicodeText) -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator> {
    let look = self.0;
let start_pos = self.1;
    let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_find_as_ptr_look_start_pos(original_self as *const ::unicodetext::i18n::phonenumbers::UnicodeText, look as *const ::unicodetext::i18n::phonenumbers::UnicodeText, start_pos as *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator) };
unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }
}
      impl<'l0> UnicodeTextFindArgs<'l0> for (&'l0 ::unicodetext::i18n::phonenumbers::UnicodeText, cpp_utils::AsStruct) {
        type ReturnType = ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator;
        fn exec(self,
                original_self: &'l0 ::unicodetext::i18n::phonenumbers::UnicodeText)
                -> ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator {
          let look = self.0;
          {
            let mut object: ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator =
              unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_find_to_output_look(original_self as *const ::unicodetext::i18n::phonenumbers::UnicodeText, look as *const ::unicodetext::i18n::phonenumbers::UnicodeText, &mut object) }
            object
          }
        }
      }
      impl<'l0> UnicodeTextFindArgs<'l0> for (&'l0 ::unicodetext::i18n::phonenumbers::UnicodeText,&'l0 ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator,cpp_utils::AsStruct) {
  type ReturnType = ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator;
  fn exec(self, original_self: &'l0 ::unicodetext::i18n::phonenumbers::UnicodeText) -> ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator {
    let look = self.0;
let start_pos = self.1;
    {
let mut object: ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator = unsafe { ::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_find_to_output_look_start_pos(original_self as *const ::unicodetext::i18n::phonenumbers::UnicodeText, look as *const ::unicodetext::i18n::phonenumbers::UnicodeText, start_pos as *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, &mut object) }object
}
  }
}
      /// This trait represents a set of arguments accepted by [UnicodeText::make_iterator](../struct.UnicodeText.html#method.make_iterator) method.
      pub trait UnicodeTextMakeIteratorArgs<'l0> {
        type ReturnType;
        fn exec(self, original_self: &'l0 ::unicodetext::i18n::phonenumbers::UnicodeText) -> Self::ReturnType;
      }
      impl<'l0> UnicodeTextMakeIteratorArgs<'l0> for (*const libc::c_char, cpp_utils::AsBox) {
        type ReturnType = cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator>;
        fn exec(self,
                original_self: &'l0 ::unicodetext::i18n::phonenumbers::UnicodeText)
                -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator> {
          let p = self.0;
          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_MakeIterator_as_ptr(original_self as *const ::unicodetext::i18n::phonenumbers::UnicodeText, p) };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
      impl<'l0> UnicodeTextMakeIteratorArgs<'l0> for (*const libc::c_char, cpp_utils::AsStruct) {
        type ReturnType = ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator;
        fn exec(self,
                original_self: &'l0 ::unicodetext::i18n::phonenumbers::UnicodeText)
                -> ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator {
          let p = self.0;
          {
            let mut object: ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator =
              unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_MakeIterator_to_output(original_self as *const ::unicodetext::i18n::phonenumbers::UnicodeText, p, &mut object) }
            object
          }
        }
      }
      /// This trait represents a set of arguments accepted by [UnicodeText::new](../struct.UnicodeText.html#method.new) method.
      pub trait UnicodeTextNewArgs {
        type ReturnType;
        fn exec(self) -> Self::ReturnType;
      }
      impl<'a> UnicodeTextNewArgs
        for (&'a ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator,
                                               &'a ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator,
                                               cpp_utils::AsStruct) {
        type ReturnType = ::unicodetext::i18n::phonenumbers::UnicodeText;
        fn exec(self) -> ::unicodetext::i18n::phonenumbers::UnicodeText {
          let first = self.0;
          let last = self.1;
          {
            let mut object: ::unicodetext::i18n::phonenumbers::UnicodeText =
              unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_constructor_first_last(first as *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, last as *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, &mut object) }
            object
          }
        }
      }
      impl UnicodeTextNewArgs for cpp_utils::AsStruct {
        type ReturnType = ::unicodetext::i18n::phonenumbers::UnicodeText;
        fn exec(self) -> ::unicodetext::i18n::phonenumbers::UnicodeText {

          {
            let mut object: ::unicodetext::i18n::phonenumbers::UnicodeText =
              unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_constructor_no_args(&mut object) }
            object
          }
        }
      }
      impl<'a> UnicodeTextNewArgs for (&'a ::unicodetext::i18n::phonenumbers::UnicodeText, cpp_utils::AsStruct) {
        type ReturnType = ::unicodetext::i18n::phonenumbers::UnicodeText;
        fn exec(self) -> ::unicodetext::i18n::phonenumbers::UnicodeText {
          let src = self.0;
          {
            let mut object: ::unicodetext::i18n::phonenumbers::UnicodeText =
              unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_constructor_src(src as *const ::unicodetext::i18n::phonenumbers::UnicodeText, &mut object) }
            object
          }
        }
      }
      impl<'a> UnicodeTextNewArgs
        for (&'a ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator,
                                               &'a ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator,
                                               cpp_utils::AsBox) {
        type ReturnType = cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::UnicodeText>;
        fn exec(self) -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::UnicodeText> {
          let first = self.0;
          let last = self.1;
          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_new_first_last(first as *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, last as *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator) };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
      impl UnicodeTextNewArgs for cpp_utils::AsBox {
        type ReturnType = cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::UnicodeText>;
        fn exec(self) -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::UnicodeText> {

          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_new_no_args() };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
      impl<'a> UnicodeTextNewArgs for (&'a ::unicodetext::i18n::phonenumbers::UnicodeText, cpp_utils::AsBox) {
        type ReturnType = cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::UnicodeText>;
        fn exec(self) -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::UnicodeText> {
          let src = self.0;
          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_new_src(src as *const ::unicodetext::i18n::phonenumbers::UnicodeText) };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
      /// This trait represents a set of arguments accepted by [UnicodeText::point_to](../struct.UnicodeText.html#method.point_to) method.
      pub trait UnicodeTextPointToArgs<'l0> {
        fn exec(self,
                original_self: &'l0 mut ::unicodetext::i18n::phonenumbers::UnicodeText)
                -> &'l0 mut ::unicodetext::i18n::phonenumbers::UnicodeText;
      }
      impl<'l0> UnicodeTextPointToArgs<'l0> for (&'l0 ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator,&'l0 ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator) {

  fn exec(self, original_self: &'l0 mut ::unicodetext::i18n::phonenumbers::UnicodeText) -> &'l0 mut ::unicodetext::i18n::phonenumbers::UnicodeText {
    let first = self.0;
let last = self.1;
    let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_PointTo_first_last(original_self as *mut ::unicodetext::i18n::phonenumbers::UnicodeText, first as *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, last as *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator) };
unsafe { &mut *ffi_result }
  }
}
      impl<'l0> UnicodeTextPointToArgs<'l0> for &'l0 ::unicodetext::i18n::phonenumbers::UnicodeText {
        fn exec(self,
                original_self: &'l0 mut ::unicodetext::i18n::phonenumbers::UnicodeText)
                -> &'l0 mut ::unicodetext::i18n::phonenumbers::UnicodeText {
          let src = self;
          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_PointTo_src(original_self as *mut ::unicodetext::i18n::phonenumbers::UnicodeText, src as *const ::unicodetext::i18n::phonenumbers::UnicodeText) };
          unsafe { &mut *ffi_result }
        }
      }
      /// This trait represents a set of arguments accepted by [UnicodeText::rbegin](../struct.UnicodeText.html#method.rbegin) method.
      pub trait UnicodeTextRbeginArgs<'l0> {
        type ReturnType;
        fn exec(self, original_self: &'l0 ::unicodetext::i18n::phonenumbers::UnicodeText) -> Self::ReturnType;
      }
      impl<'l0> UnicodeTextRbeginArgs<'l0> for cpp_utils::AsBox {
        type ReturnType = cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::unicode_text::ConstReverseIterator>;
        fn exec(self,
                original_self: &'l0 ::unicodetext::i18n::phonenumbers::UnicodeText)
                -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::unicode_text::ConstReverseIterator> {

          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_rbegin_as_ptr(original_self as *const ::unicodetext::i18n::phonenumbers::UnicodeText) };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
      impl<'l0> UnicodeTextRbeginArgs<'l0> for cpp_utils::AsStruct {
        type ReturnType = ::unicodetext::i18n::phonenumbers::unicode_text::ConstReverseIterator;
        fn exec(self,
                original_self: &'l0 ::unicodetext::i18n::phonenumbers::UnicodeText)
                -> ::unicodetext::i18n::phonenumbers::unicode_text::ConstReverseIterator {

          {
            let mut object: ::unicodetext::i18n::phonenumbers::unicode_text::ConstReverseIterator =
              unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_rbegin_to_output(original_self as *const ::unicodetext::i18n::phonenumbers::UnicodeText, &mut object) }
            object
          }
        }
      }
      /// This trait represents a set of arguments accepted by [UnicodeText::rend](../struct.UnicodeText.html#method.rend) method.
      pub trait UnicodeTextRendArgs<'l0> {
        type ReturnType;
        fn exec(self, original_self: &'l0 ::unicodetext::i18n::phonenumbers::UnicodeText) -> Self::ReturnType;
      }
      impl<'l0> UnicodeTextRendArgs<'l0> for cpp_utils::AsBox {
        type ReturnType = cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::unicode_text::ConstReverseIterator>;
        fn exec(self,
                original_self: &'l0 ::unicodetext::i18n::phonenumbers::UnicodeText)
                -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::unicode_text::ConstReverseIterator> {

          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_rend_as_ptr(original_self as *const ::unicodetext::i18n::phonenumbers::UnicodeText) };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
      impl<'l0> UnicodeTextRendArgs<'l0> for cpp_utils::AsStruct {
        type ReturnType = ::unicodetext::i18n::phonenumbers::unicode_text::ConstReverseIterator;
        fn exec(self,
                original_self: &'l0 ::unicodetext::i18n::phonenumbers::UnicodeText)
                -> ::unicodetext::i18n::phonenumbers::unicode_text::ConstReverseIterator {

          {
            let mut object: ::unicodetext::i18n::phonenumbers::unicode_text::ConstReverseIterator =
              unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_rend_to_output(original_self as *const ::unicodetext::i18n::phonenumbers::UnicodeText, &mut object) }
            object
          }
        }
      }
      /// This trait represents a set of arguments accepted by [unicode_text_to_u_t_f8](../fn.unicode_text_to_u_t_f8.html) method.
      pub trait UnicodeTextToUTF8Args {
        type ReturnType;
        fn exec(self) -> Self::ReturnType;
      }
      impl<'a> UnicodeTextToUTF8Args for (&'a ::unicodetext::i18n::phonenumbers::UnicodeText, cpp_utils::AsBox) {
        type ReturnType = cpp_utils::CppBox<::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef>;
        fn exec
          (self)
           -> cpp_utils::CppBox<::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef> {
          let t = self.0;
          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UnicodeTextToUTF8_as_ptr(t as *const ::unicodetext::i18n::phonenumbers::UnicodeText) };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
      impl<'a> UnicodeTextToUTF8Args for (&'a ::unicodetext::i18n::phonenumbers::UnicodeText, cpp_utils::AsStruct) {
        type ReturnType = ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
        fn exec(self) -> ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
          let t = self.0;
          {
            let mut object: ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef =
              unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe { ::ffi::libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UnicodeTextToUTF8_to_output(t as *const ::unicodetext::i18n::phonenumbers::UnicodeText, &mut object) }
            object
          }
        }
      }
      /// This trait represents a set of arguments accepted by [UnicodeText::utf_8_substring](../struct.UnicodeText.html#method.utf_8_substring) method.
      pub trait UnicodeTextUtf8SubstringArgs {
        type ReturnType;
        fn exec(self) -> Self::ReturnType;
      }
      impl<'a> UnicodeTextUtf8SubstringArgs for (&'a ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator,&'a ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator,cpp_utils::AsBox) {
  type ReturnType = cpp_utils::CppBox<::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef>;
  fn exec(self, ) -> cpp_utils::CppBox<::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef> {
    let first = self.0;
let last = self.1;
    let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_UTF8Substring_as_ptr(first as *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, last as *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator) };
unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }
}
      impl<'a> UnicodeTextUtf8SubstringArgs for (&'a ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator,&'a ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator,cpp_utils::AsStruct) {
  type ReturnType = ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  fn exec(self, ) -> ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
    let first = self.0;
let last = self.1;
    {
let mut object: ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef = unsafe { ::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_UTF8Substring_to_output(first as *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, last as *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, &mut object) }object
}
  }
}
      /// This trait represents a set of arguments accepted by [utf_8_to_unicode_text](../fn.utf_8_to_unicode_text.html) method.
      pub trait Utf8ToUnicodeTextArgs {
        type ReturnType;
        fn exec(self) -> Self::ReturnType;
      }
      impl Utf8ToUnicodeTextArgs for (*const libc::c_char, libc::c_int, cpp_utils::AsBox) {
        type ReturnType = cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::UnicodeText>;
        fn exec(self) -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::UnicodeText> {
          let utf8_buf = self.0;
          let len = self.1;
          let ffi_result =
            unsafe {
              ::ffi::libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UTF8ToUnicodeText_as_ptr_utf8_buf_len(utf8_buf, len)
            };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
      impl Utf8ToUnicodeTextArgs for (*const libc::c_char, libc::c_int, bool, cpp_utils::AsBox) {
        type ReturnType = cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::UnicodeText>;
        fn exec(self) -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::UnicodeText> {
          let utf8_buf = self.0;
          let len = self.1;
          let do_copy = self.2;
          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UTF8ToUnicodeText_as_ptr_utf8_buf_len_do_copy(utf8_buf, len, do_copy) };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
      impl<'a> Utf8ToUnicodeTextArgs for (&'a ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,cpp_utils::AsBox) {
  type ReturnType = cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::UnicodeText>;
  fn exec(self, ) -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::UnicodeText> {
    let utf8_string = self.0;
    let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UTF8ToUnicodeText_as_ptr_utf8_string(utf8_string as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) };
unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }
}
      impl<'a> Utf8ToUnicodeTextArgs for (&'a ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,bool,cpp_utils::AsBox) {
  type ReturnType = cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::UnicodeText>;
  fn exec(self, ) -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::UnicodeText> {
    let utf_string = self.0;
let do_copy = self.1;
    let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UTF8ToUnicodeText_as_ptr_utf_string_do_copy(utf_string as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, do_copy) };
unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }
}
      impl Utf8ToUnicodeTextArgs for (*const libc::c_char, libc::c_int, cpp_utils::AsStruct) {
        type ReturnType = ::unicodetext::i18n::phonenumbers::UnicodeText;
        fn exec(self) -> ::unicodetext::i18n::phonenumbers::UnicodeText {
          let utf8_buf = self.0;
          let len = self.1;
          {
            let mut object: ::unicodetext::i18n::phonenumbers::UnicodeText =
              unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe { ::ffi::libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UTF8ToUnicodeText_to_output_utf8_buf_len(utf8_buf, len, &mut object) }
            object
          }
        }
      }
      impl Utf8ToUnicodeTextArgs for (*const libc::c_char, libc::c_int, bool, cpp_utils::AsStruct) {
        type ReturnType = ::unicodetext::i18n::phonenumbers::UnicodeText;
        fn exec(self) -> ::unicodetext::i18n::phonenumbers::UnicodeText {
          let utf8_buf = self.0;
          let len = self.1;
          let do_copy = self.2;
          {
            let mut object: ::unicodetext::i18n::phonenumbers::UnicodeText =
              unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe { ::ffi::libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UTF8ToUnicodeText_to_output_utf8_buf_len_do_copy(utf8_buf, len, do_copy, &mut object) }
            object
          }
        }
      }
      impl<'a> Utf8ToUnicodeTextArgs for (&'a ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,cpp_utils::AsStruct) {
  type ReturnType = ::unicodetext::i18n::phonenumbers::UnicodeText;
  fn exec(self, ) -> ::unicodetext::i18n::phonenumbers::UnicodeText {
    let utf8_string = self.0;
    {
let mut object: ::unicodetext::i18n::phonenumbers::UnicodeText = unsafe { ::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UTF8ToUnicodeText_to_output_utf8_string(utf8_string as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, &mut object) }object
}
  }
}
      impl<'a> Utf8ToUnicodeTextArgs for (&'a ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,bool,cpp_utils::AsStruct) {
  type ReturnType = ::unicodetext::i18n::phonenumbers::UnicodeText;
  fn exec(self, ) -> ::unicodetext::i18n::phonenumbers::UnicodeText {
    let utf_string = self.0;
let do_copy = self.1;
    {
let mut object: ::unicodetext::i18n::phonenumbers::UnicodeText = unsafe { ::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UTF8ToUnicodeText_to_output_utf_string_do_copy(utf_string as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, do_copy, &mut object) }object
}
  }
}
    }

    pub mod unicode_text {
      #[allow(unused_imports)]
      use {libc, cpp_utils, std};

      /// C++ type: <span style='color: green;'>```i18n::phonenumbers::UnicodeText::const_iterator```</span>.
      #[repr(C)]
      pub struct ConstIterator {
        _buffer: [u8; 8],
      }

      impl ::NewUninitialized for ConstIterator {
        unsafe fn new_uninitialized() -> ConstIterator {
          ConstIterator { _buffer: std::mem::uninitialized() }
        }
      }
      //added
      impl cpp_utils::CppDeletable for ConstIterator {
	    fn deleter() -> cpp_utils::Deleter<Self> { 
			unsafe extern "C" fn deleter_func(this_ptr: *mut ConstIterator) {
				drop(this_ptr);
			};
			deleter_func
		}
	  }
      impl ConstIterator {
        /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText::const_iterator::DebugString```</span>
        ///
        /// This is an overloaded function. Available variants:
        ///
        /// Rust arguments: <br>1) ```fn debug_string(&self, cpp_utils::AsBox) -> cpp_utils::CppBox<::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef>```<br>2) ```fn debug_string(&self, cpp_utils::AsStruct) -> ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef```<br>
        /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>> i18n::phonenumbers::UnicodeText::const_iterator::DebugString() const```</span>
        ///
        ///
        pub fn debug_string<'l0, Args>(&'l0 self, args: Args) -> Args::ReturnType
          where Args: overloading::ConstIteratorDebugStringArgs<'l0>
        {
          args.exec(self)
        }
        /// C++ method: <span style='color: green;'>```int i18n::phonenumbers::UnicodeText::const_iterator::get_utf8(char* buf) const```</span>
        ///
        ///
        pub fn get_utf8(&self, buf: *mut libc::c_char) -> libc::c_int {
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_get_utf8(self as *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, buf) }
        }

        /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText::const_iterator::const_iterator```</span>
        ///
        /// This is an overloaded function. Available variants:
        ///
        ///
        ///
        /// ## Variant 1
        ///
        /// Rust arguments: <br>1) ```fn new(cpp_utils::AsStruct) -> ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator```<br>2) ```fn new(cpp_utils::AsBox) -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator>```<br>
        /// C++ method: <span style='color: green;'>```[constructor] void i18n::phonenumbers::UnicodeText::const_iterator::const_iterator()```</span>
        ///
        ///
        ///
        /// ## Variant 2
        ///
        /// Rust arguments: <br>1) ```fn new((&::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, cpp_utils::AsStruct)) -> ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator```<br>2) ```fn new((&::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, cpp_utils::AsBox)) -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator>```<br>
        /// C++ method: <span style='color: green;'>```[constructor] void i18n::phonenumbers::UnicodeText::const_iterator::const_iterator(const i18n::phonenumbers::UnicodeText::const_iterator& other)```</span>
        ///
        ///
        pub fn new<Args>(args: Args) -> Args::ReturnType
          where Args: overloading::ConstIteratorNewArgs
        {
          args.exec()
        }
        /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText::const_iterator& i18n::phonenumbers::UnicodeText::const_iterator::operator=(const i18n::phonenumbers::UnicodeText::const_iterator& other)```</span>
        ///
        ///
        pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                                   other: &'l1 ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator)
                                   -> &'l0 mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator {
          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_operator_assign(self as *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, other as *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator) };
          unsafe { &mut *ffi_result }
        }

        /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText::const_iterator& i18n::phonenumbers::UnicodeText::const_iterator::operator--()```</span>
        ///
        ///
        pub fn op_dec<'l0>(&'l0 mut self) -> &'l0 mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator {
          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_operator_dec(self as *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator) };
          unsafe { &mut *ffi_result }
        }

        /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText::const_iterator::operator--```</span>
        ///
        /// This is an overloaded function. Available variants:
        ///
        /// Rust arguments: <br>1) ```fn op_dec_postfix(&mut self, (libc::c_int, cpp_utils::AsBox)) -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator>```<br>2) ```fn op_dec_postfix(&mut self, (libc::c_int, cpp_utils::AsStruct)) -> ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator```<br>
        /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText::const_iterator i18n::phonenumbers::UnicodeText::const_iterator::operator--(int arg1)```</span>
        ///
        ///
        pub fn op_dec_postfix<'l0, Args>(&'l0 mut self, args: Args) -> Args::ReturnType
          where Args: overloading::ConstIteratorOpDecPostfixArgs<'l0>
        {
          args.exec(self)
        }
        /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText::const_iterator& i18n::phonenumbers::UnicodeText::const_iterator::operator++()```</span>
        ///
        ///
        pub fn op_inc<'l0>(&'l0 mut self) -> &'l0 mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator {
          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_operator_inc(self as *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator) };
          unsafe { &mut *ffi_result }
        }

        /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText::const_iterator::operator++```</span>
        ///
        /// This is an overloaded function. Available variants:
        ///
        /// Rust arguments: <br>1) ```fn op_inc_postfix(&mut self, (libc::c_int, cpp_utils::AsBox)) -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator>```<br>2) ```fn op_inc_postfix(&mut self, (libc::c_int, cpp_utils::AsStruct)) -> ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator```<br>
        /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText::const_iterator i18n::phonenumbers::UnicodeText::const_iterator::operator++(int arg1)```</span>
        ///
        ///
        pub fn op_inc_postfix<'l0, Args>(&'l0 mut self, args: Args) -> Args::ReturnType
          where Args: overloading::ConstIteratorOpIncPostfixArgs<'l0>
        {
          args.exec(self)
        }
        /// C++ method: <span style='color: green;'>```int i18n::phonenumbers::UnicodeText::const_iterator::operator*() const```</span>
        ///
        ///
        pub fn op_indirection(&self) -> libc::c_int {
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_operator_indirection(self as *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator) }
        }

        /// C++ method: <span style='color: green;'>```const char* i18n::phonenumbers::UnicodeText::const_iterator::utf8_data() const```</span>
        ///
        ///
        pub fn utf8_data(&self) -> *const libc::c_char {
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_utf8_data(self as *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator) }
        }
      }

      /// C++ type: <span style='color: green;'>```i18n::phonenumbers::UnicodeText::const_reverse_iterator```</span>.
      #[repr(C)]
      pub struct ConstReverseIterator {
        _buffer: [u8; 8],
      }

      impl ::NewUninitialized for ConstReverseIterator {
        unsafe fn new_uninitialized() -> ConstReverseIterator {
          ConstReverseIterator { _buffer: std::mem::uninitialized() }
        }
      }

      //added
      impl cpp_utils::CppDeletable for ConstReverseIterator {
	    fn deleter() -> cpp_utils::Deleter<Self> { 
			unsafe extern "C" fn deleter_func(this_ptr: *mut ConstReverseIterator) {
				drop(this_ptr);
			};
			return deleter_func;
		}
	  }

      impl ConstReverseIterator {
        /// C++ method: <span style='color: green;'>```int i18n::phonenumbers::UnicodeText::const_reverse_iterator::get_utf8(char* buf) const```</span>
        ///
        ///
        pub fn get_utf8(&self, buf: *mut libc::c_char) -> libc::c_int {
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_reverse_iterator_get_utf8(self as *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstReverseIterator, buf) }
        }

        /// C++ method: <span style='color: green;'>```i18n::phonenumbers::UnicodeText::const_reverse_iterator::const_reverse_iterator```</span>
        ///
        /// This is an overloaded function. Available variants:
        ///
        /// Rust arguments: <br>1) ```fn new((&::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, cpp_utils::AsStruct)) -> ::unicodetext::i18n::phonenumbers::unicode_text::ConstReverseIterator```<br>2) ```fn new((&::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, cpp_utils::AsBox)) -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::unicode_text::ConstReverseIterator>```<br>
        /// C++ method: <span style='color: green;'>```[constructor] void i18n::phonenumbers::UnicodeText::const_reverse_iterator::const_reverse_iterator(i18n::phonenumbers::UnicodeText::const_iterator it)```</span>
        ///
        ///
        pub fn new<Args>(args: Args) -> Args::ReturnType
          where Args: overloading::ConstReverseIteratorNewArgs
        {
          args.exec()
        }
        /// C++ method: <span style='color: green;'>```const char* i18n::phonenumbers::UnicodeText::const_reverse_iterator::utf8_data() const```</span>
        ///
        ///
        pub fn utf8_data(&self) -> *const libc::c_char {
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_reverse_iterator_utf8_data(self as *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstReverseIterator) }
        }
      }

      pub mod overloading {
        #[allow(unused_imports)]
        use {libc, cpp_utils, std};

        /// This trait represents a set of arguments accepted by [ConstIterator::debug_string](../struct.ConstIterator.html#method.debug_string) method.
        pub trait ConstIteratorDebugStringArgs<'l0> {
          type ReturnType;
          fn exec(self,
                  original_self: &'l0 ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator)
                  -> Self::ReturnType;
        }
        impl<'l0> ConstIteratorDebugStringArgs<'l0> for cpp_utils::AsBox {
          type ReturnType = cpp_utils::CppBox<::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef>;
          fn exec
            (self,
             original_self: &'l0 ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator)
             -> cpp_utils::CppBox<::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef> {

            let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_DebugString_as_ptr(original_self as *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator) };
            unsafe { ::cpp_utils::CppBox::new(ffi_result) }
          }
        }
        impl<'l0> ConstIteratorDebugStringArgs<'l0> for cpp_utils::AsStruct {
          type ReturnType = ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
          fn exec(self,
                  original_self: &'l0 ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator)
                  -> ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {

            {
              let mut object: ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef =
                unsafe { ::NewUninitialized::new_uninitialized() };
              unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_DebugString_to_output(original_self as *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, &mut object) }
              object
            }
          }
        }
        /// This trait represents a set of arguments accepted by [ConstIterator::new](../struct.ConstIterator.html#method.new) method.
        pub trait ConstIteratorNewArgs {
          type ReturnType;
          fn exec(self) -> Self::ReturnType;
        }
        impl ConstIteratorNewArgs for cpp_utils::AsStruct {
          type ReturnType = ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator;
          fn exec(self) -> ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator {

            {
              let mut object: ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator =
                unsafe { ::NewUninitialized::new_uninitialized() };
              unsafe {
                ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_constructor_no_args(&mut object)
              }
              object
            }
          }
        }
        impl<'a> ConstIteratorNewArgs for (&'a ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator,cpp_utils::AsStruct) {
  type ReturnType = ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator;
  fn exec(self, ) -> ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator {
    let other = self.0;
    {
let mut object: ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator = unsafe { ::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_constructor_other(other as *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, &mut object) }object
}
  }
}
        impl ConstIteratorNewArgs for cpp_utils::AsBox {
          type ReturnType = cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator>;
          fn exec(self) -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator> {

            let ffi_result =
              unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_new_no_args() };
            unsafe { ::cpp_utils::CppBox::new(ffi_result) }
          }
        }
        impl<'a> ConstIteratorNewArgs for (&'a ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator,cpp_utils::AsBox) {
  type ReturnType = cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator>;
  fn exec(self, ) -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator> {
    let other = self.0;
    let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_new_other(other as *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator) };
unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }
}
        /// This trait represents a set of arguments accepted by [ConstIterator::op_dec_postfix](../struct.ConstIterator.html#method.op_dec_postfix) method.
        pub trait ConstIteratorOpDecPostfixArgs<'l0> {
          type ReturnType;
          fn exec(self,
                  original_self: &'l0 mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator)
                  -> Self::ReturnType;
        }
        impl<'l0> ConstIteratorOpDecPostfixArgs<'l0> for (libc::c_int, cpp_utils::AsBox) {
          type ReturnType = cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator>;
          fn exec(self,
                  original_self: &'l0 mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator)
                  -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator> {
            let arg1 = self.0;
            let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_operator_dec_postfix_as_ptr(original_self as *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, arg1) };
            unsafe { ::cpp_utils::CppBox::new(ffi_result) }
          }
        }
        impl<'l0> ConstIteratorOpDecPostfixArgs<'l0> for (libc::c_int, cpp_utils::AsStruct) {
          type ReturnType = ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator;
          fn exec(self,
                  original_self: &'l0 mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator)
                  -> ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator {
            let arg1 = self.0;
            {
              let mut object: ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator =
                unsafe { ::NewUninitialized::new_uninitialized() };
              unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_operator_dec_postfix_to_output(original_self as *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, arg1, &mut object) }
              object
            }
          }
        }
        /// This trait represents a set of arguments accepted by [ConstIterator::op_inc_postfix](../struct.ConstIterator.html#method.op_inc_postfix) method.
        pub trait ConstIteratorOpIncPostfixArgs<'l0> {
          type ReturnType;
          fn exec(self,
                  original_self: &'l0 mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator)
                  -> Self::ReturnType;
        }
        impl<'l0> ConstIteratorOpIncPostfixArgs<'l0> for (libc::c_int, cpp_utils::AsBox) {
          type ReturnType = cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator>;
          fn exec(self,
                  original_self: &'l0 mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator)
                  -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator> {
            let arg1 = self.0;
            let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_operator_inc_postfix_as_ptr(original_self as *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, arg1) };
            unsafe { ::cpp_utils::CppBox::new(ffi_result) }
          }
        }
        impl<'l0> ConstIteratorOpIncPostfixArgs<'l0> for (libc::c_int, cpp_utils::AsStruct) {
          type ReturnType = ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator;
          fn exec(self,
                  original_self: &'l0 mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator)
                  -> ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator {
            let arg1 = self.0;
            {
              let mut object: ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator =
                unsafe { ::NewUninitialized::new_uninitialized() };
              unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_operator_inc_postfix_to_output(original_self as *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, arg1, &mut object) }
              object
            }
          }
        }
        /// This trait represents a set of arguments accepted by [ConstReverseIterator::new](../struct.ConstReverseIterator.html#method.new) method.
        pub trait ConstReverseIteratorNewArgs {
          type ReturnType;
          fn exec(self) -> Self::ReturnType;
        }
        impl<'a> ConstReverseIteratorNewArgs for (&'a ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator,cpp_utils::AsStruct) {
  type ReturnType = ::unicodetext::i18n::phonenumbers::unicode_text::ConstReverseIterator;
  fn exec(self, ) -> ::unicodetext::i18n::phonenumbers::unicode_text::ConstReverseIterator {
    let it = self.0;
    {
let mut object: ::unicodetext::i18n::phonenumbers::unicode_text::ConstReverseIterator = unsafe { ::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_reverse_iterator_constructor(it as *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, &mut object) }object
}
  }
}
        impl<'a> ConstReverseIteratorNewArgs for (&'a ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator,cpp_utils::AsBox) {
  type ReturnType = cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::unicode_text::ConstReverseIterator>;
  fn exec(self, ) -> cpp_utils::CppBox<::unicodetext::i18n::phonenumbers::unicode_text::ConstReverseIterator> {
    let it = self.0;
    let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_reverse_iterator_new(it as *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator) };
unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }
}
      }

    }

  }

}
