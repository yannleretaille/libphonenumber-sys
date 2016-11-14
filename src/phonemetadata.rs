#[allow(unused_imports)]
use {libc, cpp_utils, std};

pub mod i18n {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  pub mod phonenumbers {
    #[allow(unused_imports)]
    use {libc, cpp_utils, std};

    /// C++ type: <span style='color: green;'>```i18n::phonenumbers::NumberFormat```</span>.
    #[repr(C)]
    pub struct NumberFormat {
      _buffer: [u8; 96],
    }

    impl ::NewUninitialized for NumberFormat {
      unsafe fn new_uninitialized() -> NumberFormat {
        NumberFormat { _buffer: std::mem::uninitialized() }
      }
    }

    impl NumberFormat {
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::NumberFormat::add_leading_digits_pattern```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn add_leading_digits_pattern(&mut self, ()) -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef```<br>
      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* i18n::phonenumbers::NumberFormat::add_leading_digits_pattern()```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn add_leading_digits_pattern(&mut self, *const libc::c_char) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::NumberFormat::add_leading_digits_pattern(const char* value)```</span>
      ///
      ///
      ///
      /// ## Variant 3
      ///
      /// Rust arguments: ```fn add_leading_digits_pattern(&mut self, (*const libc::c_char, libc::c_ulong)) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::NumberFormat::add_leading_digits_pattern(const char* value, unsigned long size)```</span>
      ///
      ///
      ///
      /// ## Variant 4
      ///
      /// Rust arguments: ```fn add_leading_digits_pattern(&mut self, &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::NumberFormat::add_leading_digits_pattern(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& value)```</span>
      ///
      ///
      pub fn add_leading_digits_pattern<'l0, Args>(&'l0 mut self, args: Args) -> Args::ReturnType
        where Args: overloading::NumberFormatAddLeadingDigitsPatternArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```virtual int i18n::phonenumbers::NumberFormat::ByteSize() const```</span>
      ///
      ///
      pub fn byte_size(&self) -> libc::c_int {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_ByteSize(self as *const ::phonemetadata::i18n::phonenumbers::NumberFormat) }
      }

      /// C++ method: <span style='color: green;'>```virtual void i18n::phonenumbers::NumberFormat::CheckTypeAndMergeFrom(const google::protobuf::MessageLite& from)```</span>
      ///
      ///
      pub fn check_type_and_merge_from(&mut self, from: &::message_lite::google::protobuf::MessageLite) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_CheckTypeAndMergeFrom(self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, from as *const ::message_lite::google::protobuf::MessageLite) }
      }

      /// C++ method: <span style='color: green;'>```virtual void i18n::phonenumbers::NumberFormat::Clear()```</span>
      ///
      ///
      pub fn clear(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_Clear(self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::NumberFormat::clear_domestic_carrier_code_formatting_rule()```</span>
      ///
      ///
      pub fn clear_domestic_carrier_code_formatting_rule(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_clear_domestic_carrier_code_formatting_rule(self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::NumberFormat::clear_format()```</span>
      ///
      ///
      pub fn clear_format(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_clear_format(self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::NumberFormat::clear_leading_digits_pattern()```</span>
      ///
      ///
      pub fn clear_leading_digits_pattern(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_clear_leading_digits_pattern(self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::NumberFormat::clear_national_prefix_formatting_rule()```</span>
      ///
      ///
      pub fn clear_national_prefix_formatting_rule(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_clear_national_prefix_formatting_rule(self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::NumberFormat::clear_national_prefix_optional_when_formatting()```</span>
      ///
      ///
      pub fn clear_national_prefix_optional_when_formatting(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_clear_national_prefix_optional_when_formatting(self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::NumberFormat::clear_pattern()```</span>
      ///
      ///
      pub fn clear_pattern(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_clear_pattern(self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::NumberFormat::CopyFrom(const i18n::phonenumbers::NumberFormat& from)```</span>
      ///
      ///
      pub fn copy_from(&mut self, from: &::phonemetadata::i18n::phonenumbers::NumberFormat) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_CopyFrom(self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, from as *const ::phonemetadata::i18n::phonenumbers::NumberFormat) }
      }

      /// C++ method: <span style='color: green;'>```static const i18n::phonenumbers::NumberFormat& i18n::phonenumbers::NumberFormat::default_instance()```</span>
      ///
      ///
      pub fn default_instance() -> &'static ::phonemetadata::i18n::phonenumbers::NumberFormat {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_default_instance() };
        unsafe { &*ffi_result }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::NumberFormat::DiscardUnknownFields()```</span>
      ///
      ///
      pub fn discard_unknown_fields(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_DiscardUnknownFields(self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat) }
      }

      /// C++ method: <span style='color: green;'>```const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& i18n::phonenumbers::NumberFormat::domestic_carrier_code_formatting_rule() const```</span>
      ///
      ///
      pub fn domestic_carrier_code_formatting_rule<'l0>
        (&'l0 self)
         -> &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_domestic_carrier_code_formatting_rule(self as *const ::phonemetadata::i18n::phonenumbers::NumberFormat) };
        unsafe { &*ffi_result }
      }

      /// C++ method: <span style='color: green;'>```const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& i18n::phonenumbers::NumberFormat::format() const```</span>
      ///
      ///
      pub fn format<'l0>(&'l0 self)
                         -> &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_format(self as *const ::phonemetadata::i18n::phonenumbers::NumberFormat) };
        unsafe { &*ffi_result }
      }

      /// C++ method: <span style='color: green;'>```virtual int i18n::phonenumbers::NumberFormat::GetCachedSize() const```</span>
      ///
      ///
      pub fn get_cached_size(&self) -> libc::c_int {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_GetCachedSize(self as *const ::phonemetadata::i18n::phonenumbers::NumberFormat) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::NumberFormat::GetTypeName```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      /// Rust arguments: <br>1) ```fn get_type_name(&self, cpp_utils::AsBox) -> cpp_utils::CppBox<::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef>```<br>2) ```fn get_type_name(&self, cpp_utils::AsStruct) -> ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef```<br>
      /// C++ method: <span style='color: green;'>```virtual std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>> i18n::phonenumbers::NumberFormat::GetTypeName() const```</span>
      ///
      ///
      pub fn get_type_name<'l0, Args>(&'l0 self, args: Args) -> Args::ReturnType
        where Args: overloading::NumberFormatGetTypeNameArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::NumberFormat::has_domestic_carrier_code_formatting_rule() const```</span>
      ///
      ///
      pub fn has_domestic_carrier_code_formatting_rule(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_has_domestic_carrier_code_formatting_rule(self as *const ::phonemetadata::i18n::phonenumbers::NumberFormat) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::NumberFormat::has_format() const```</span>
      ///
      ///
      pub fn has_format(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_has_format(self as *const ::phonemetadata::i18n::phonenumbers::NumberFormat) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::NumberFormat::has_national_prefix_formatting_rule() const```</span>
      ///
      ///
      pub fn has_national_prefix_formatting_rule(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_has_national_prefix_formatting_rule(self as *const ::phonemetadata::i18n::phonenumbers::NumberFormat) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::NumberFormat::has_national_prefix_optional_when_formatting() const```</span>
      ///
      ///
      pub fn has_national_prefix_optional_when_formatting(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_has_national_prefix_optional_when_formatting(self as *const ::phonemetadata::i18n::phonenumbers::NumberFormat) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::NumberFormat::has_pattern() const```</span>
      ///
      ///
      pub fn has_pattern(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_has_pattern(self as *const ::phonemetadata::i18n::phonenumbers::NumberFormat) }
      }

      /// C++ method: <span style='color: green;'>```virtual bool i18n::phonenumbers::NumberFormat::IsInitialized() const```</span>
      ///
      ///
      pub fn is_initialized(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_IsInitialized(self as *const ::phonemetadata::i18n::phonenumbers::NumberFormat) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::NumberFormat::leading_digits_pattern```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn leading_digits_pattern(&self, ()) -> &'l0 ::repeated_field::google::protobuf::RepeatedPtrFieldBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRef```<br>
      /// C++ method: <span style='color: green;'>```const google::protobuf::RepeatedPtrField<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>>& i18n::phonenumbers::NumberFormat::leading_digits_pattern() const```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn leading_digits_pattern(&self, libc::c_int) -> &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef```<br>
      /// C++ method: <span style='color: green;'>```const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& i18n::phonenumbers::NumberFormat::leading_digits_pattern(int index) const```</span>
      ///
      ///
      pub fn leading_digits_pattern<'l0, Args>(&'l0 self, args: Args) -> Args::ReturnType
        where Args: overloading::NumberFormatLeadingDigitsPatternArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```int i18n::phonenumbers::NumberFormat::leading_digits_pattern_size() const```</span>
      ///
      ///
      pub fn leading_digits_pattern_size(&self) -> libc::c_int {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_leading_digits_pattern_size(self as *const ::phonemetadata::i18n::phonenumbers::NumberFormat) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::NumberFormat::MergeFrom(const i18n::phonenumbers::NumberFormat& from)```</span>
      ///
      ///
      pub fn merge_from(&mut self, from: &::phonemetadata::i18n::phonenumbers::NumberFormat) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_MergeFrom(self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, from as *const ::phonemetadata::i18n::phonenumbers::NumberFormat) }
      }

      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* i18n::phonenumbers::NumberFormat::mutable_domestic_carrier_code_formatting_rule()```</span>
      ///
      ///
      pub fn mutable_domestic_carrier_code_formatting_rule
        (&mut self)
         -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_mutable_domestic_carrier_code_formatting_rule(self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat) }
      }

      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* i18n::phonenumbers::NumberFormat::mutable_format()```</span>
      ///
      ///
      pub fn mutable_format(&mut self)
                            -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_mutable_format(self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::NumberFormat::mutable_leading_digits_pattern```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn mutable_leading_digits_pattern(&mut self, ()) -> *mut ::repeated_field::google::protobuf::RepeatedPtrFieldBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRef```<br>
      /// C++ method: <span style='color: green;'>```google::protobuf::RepeatedPtrField<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>>* i18n::phonenumbers::NumberFormat::mutable_leading_digits_pattern()```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn mutable_leading_digits_pattern(&mut self, libc::c_int) -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef```<br>
      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* i18n::phonenumbers::NumberFormat::mutable_leading_digits_pattern(int index)```</span>
      ///
      ///
      pub fn mutable_leading_digits_pattern<'l0, Args>(&'l0 mut self, args: Args) -> Args::ReturnType
        where Args: overloading::NumberFormatMutableLeadingDigitsPatternArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* i18n::phonenumbers::NumberFormat::mutable_national_prefix_formatting_rule()```</span>
      ///
      ///
      pub fn mutable_national_prefix_formatting_rule
        (&mut self)
         -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_mutable_national_prefix_formatting_rule(self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat) }
      }

      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* i18n::phonenumbers::NumberFormat::mutable_pattern()```</span>
      ///
      ///
      pub fn mutable_pattern
        (&mut self)
         -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_mutable_pattern(self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat) }
      }

      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* i18n::phonenumbers::NumberFormat::mutable_unknown_fields()```</span>
      ///
      ///
      pub fn mutable_unknown_fields
        (&mut self)
         -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_mutable_unknown_fields(self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat) }
      }

      /// C++ method: <span style='color: green;'>```const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& i18n::phonenumbers::NumberFormat::national_prefix_formatting_rule() const```</span>
      ///
      ///
      pub fn national_prefix_formatting_rule<'l0>
        (&'l0 self)
         -> &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_national_prefix_formatting_rule(self as *const ::phonemetadata::i18n::phonenumbers::NumberFormat) };
        unsafe { &*ffi_result }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::NumberFormat::national_prefix_optional_when_formatting() const```</span>
      ///
      ///
      pub fn national_prefix_optional_when_formatting(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_national_prefix_optional_when_formatting(self as *const ::phonemetadata::i18n::phonenumbers::NumberFormat) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::NumberFormat::New```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn new(&self, ()) -> *mut ::phonemetadata::i18n::phonenumbers::NumberFormat```<br>
      /// C++ method: <span style='color: green;'>```virtual i18n::phonenumbers::NumberFormat* i18n::phonenumbers::NumberFormat::New() const```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn new(&self, *mut ::arena::google::protobuf::Arena) -> *mut ::phonemetadata::i18n::phonenumbers::NumberFormat```<br>
      /// C++ method: <span style='color: green;'>```virtual i18n::phonenumbers::NumberFormat* i18n::phonenumbers::NumberFormat::New(google::protobuf::Arena* arena) const```</span>
      ///
      ///
      pub fn new<'l0, Args>(&'l0 self, args: Args) -> *mut ::phonemetadata::i18n::phonenumbers::NumberFormat
        where Args: overloading::NumberFormatNewArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::NumberFormat::NumberFormat```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: <br>1) ```fn new_static(cpp_utils::AsStruct) -> ::phonemetadata::i18n::phonenumbers::NumberFormat```<br>2) ```fn new_static(cpp_utils::AsBox) -> cpp_utils::CppBox<::phonemetadata::i18n::phonenumbers::NumberFormat>```<br>
      /// C++ method: <span style='color: green;'>```[constructor] void i18n::phonenumbers::NumberFormat::NumberFormat()```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: <br>1) ```fn new_static((&::phonemetadata::i18n::phonenumbers::NumberFormat, cpp_utils::AsStruct)) -> ::phonemetadata::i18n::phonenumbers::NumberFormat```<br>2) ```fn new_static((&::phonemetadata::i18n::phonenumbers::NumberFormat, cpp_utils::AsBox)) -> cpp_utils::CppBox<::phonemetadata::i18n::phonenumbers::NumberFormat>```<br>
      /// C++ method: <span style='color: green;'>```[constructor] void i18n::phonenumbers::NumberFormat::NumberFormat(const i18n::phonenumbers::NumberFormat& from)```</span>
      ///
      ///
      pub fn new_static<Args>(args: Args) -> Args::ReturnType
        where Args: overloading::NumberFormatNewStaticArgs
      {
        args.exec()
      }
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::NumberFormat& i18n::phonenumbers::NumberFormat::operator=(const i18n::phonenumbers::NumberFormat& from)```</span>
      ///
      ///
      pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                                 from: &'l1 ::phonemetadata::i18n::phonenumbers::NumberFormat)
                                 -> &'l0 mut ::phonemetadata::i18n::phonenumbers::NumberFormat {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_operator_assign(self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, from as *const ::phonemetadata::i18n::phonenumbers::NumberFormat) };
        unsafe { &mut *ffi_result }
      }

      /// C++ method: <span style='color: green;'>```const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& i18n::phonenumbers::NumberFormat::pattern() const```</span>
      ///
      ///
      pub fn pattern<'l0>(&'l0 self)
                          -> &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_pattern(self as *const ::phonemetadata::i18n::phonenumbers::NumberFormat) };
        unsafe { &*ffi_result }
      }

      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* i18n::phonenumbers::NumberFormat::release_domestic_carrier_code_formatting_rule()```</span>
      ///
      ///
      pub fn release_domestic_carrier_code_formatting_rule
        (&mut self)
         -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_release_domestic_carrier_code_formatting_rule(self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat) }
      }

      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* i18n::phonenumbers::NumberFormat::release_format()```</span>
      ///
      ///
      pub fn release_format(&mut self)
                            -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_release_format(self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat) }
      }

      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* i18n::phonenumbers::NumberFormat::release_national_prefix_formatting_rule()```</span>
      ///
      ///
      pub fn release_national_prefix_formatting_rule
        (&mut self)
         -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_release_national_prefix_formatting_rule(self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat) }
      }

      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* i18n::phonenumbers::NumberFormat::release_pattern()```</span>
      ///
      ///
      pub fn release_pattern
        (&mut self)
         -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_release_pattern(self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::NumberFormat::set_allocated_domestic_carrier_code_formatting_rule(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* domestic_carrier_code_formatting_rule)```</span>
      ///
      ///
pub fn set_allocated_domestic_carrier_code_formatting_rule(&mut self, domestic_carrier_code_formatting_rule: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_allocated_domestic_carrier_code_formatting_rule(self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, domestic_carrier_code_formatting_rule) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::NumberFormat::set_allocated_format(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* format)```</span>
      ///
      ///
pub fn set_allocated_format(&mut self, format: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_allocated_format(self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, format) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::NumberFormat::set_allocated_national_prefix_formatting_rule(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* national_prefix_formatting_rule)```</span>
      ///
      ///
pub fn set_allocated_national_prefix_formatting_rule(&mut self, national_prefix_formatting_rule: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_allocated_national_prefix_formatting_rule(self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, national_prefix_formatting_rule) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::NumberFormat::set_allocated_pattern(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* pattern)```</span>
      ///
      ///
pub fn set_allocated_pattern(&mut self, pattern: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_allocated_pattern(self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, pattern) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::NumberFormat::set_domestic_carrier_code_formatting_rule```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn set_domestic_carrier_code_formatting_rule(&mut self, *const libc::c_char) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::NumberFormat::set_domestic_carrier_code_formatting_rule(const char* value)```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn set_domestic_carrier_code_formatting_rule(&mut self, (*const libc::c_char, libc::c_ulong)) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::NumberFormat::set_domestic_carrier_code_formatting_rule(const char* value, unsigned long size)```</span>
      ///
      ///
      ///
      /// ## Variant 3
      ///
      /// Rust arguments: ```fn set_domestic_carrier_code_formatting_rule(&mut self, &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::NumberFormat::set_domestic_carrier_code_formatting_rule(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& value)```</span>
      ///
      ///
      pub fn set_domestic_carrier_code_formatting_rule<'l0, Args>(&'l0 mut self, args: Args) -> ()
        where Args: overloading::NumberFormatSetDomesticCarrierCodeFormattingRuleArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::NumberFormat::set_format```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn set_format(&mut self, *const libc::c_char) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::NumberFormat::set_format(const char* value)```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn set_format(&mut self, (*const libc::c_char, libc::c_ulong)) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::NumberFormat::set_format(const char* value, unsigned long size)```</span>
      ///
      ///
      ///
      /// ## Variant 3
      ///
      /// Rust arguments: ```fn set_format(&mut self, &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::NumberFormat::set_format(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& value)```</span>
      ///
      ///
      pub fn set_format<'l0, Args>(&'l0 mut self, args: Args) -> ()
        where Args: overloading::NumberFormatSetFormatArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::NumberFormat::set_leading_digits_pattern```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn set_leading_digits_pattern(&mut self, (libc::c_int, *const libc::c_char)) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::NumberFormat::set_leading_digits_pattern(int index, const char* value)```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn set_leading_digits_pattern(&mut self, (libc::c_int, *const libc::c_char, libc::c_ulong)) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::NumberFormat::set_leading_digits_pattern(int index, const char* value, unsigned long size)```</span>
      ///
      ///
      ///
      /// ## Variant 3
      ///
      /// Rust arguments: ```fn set_leading_digits_pattern(&mut self, (libc::c_int, &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef)) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::NumberFormat::set_leading_digits_pattern(int index, const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& value)```</span>
      ///
      ///
      pub fn set_leading_digits_pattern<'l0, Args>(&'l0 mut self, args: Args) -> ()
        where Args: overloading::NumberFormatSetLeadingDigitsPatternArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::NumberFormat::set_national_prefix_formatting_rule```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn set_national_prefix_formatting_rule(&mut self, *const libc::c_char) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::NumberFormat::set_national_prefix_formatting_rule(const char* value)```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn set_national_prefix_formatting_rule(&mut self, (*const libc::c_char, libc::c_ulong)) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::NumberFormat::set_national_prefix_formatting_rule(const char* value, unsigned long size)```</span>
      ///
      ///
      ///
      /// ## Variant 3
      ///
      /// Rust arguments: ```fn set_national_prefix_formatting_rule(&mut self, &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::NumberFormat::set_national_prefix_formatting_rule(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& value)```</span>
      ///
      ///
      pub fn set_national_prefix_formatting_rule<'l0, Args>(&'l0 mut self, args: Args) -> ()
        where Args: overloading::NumberFormatSetNationalPrefixFormattingRuleArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::NumberFormat::set_national_prefix_optional_when_formatting(bool value)```</span>
      ///
      ///
      pub fn set_national_prefix_optional_when_formatting(&mut self, value: bool) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_national_prefix_optional_when_formatting(self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, value) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::NumberFormat::set_pattern```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn set_pattern(&mut self, *const libc::c_char) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::NumberFormat::set_pattern(const char* value)```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn set_pattern(&mut self, (*const libc::c_char, libc::c_ulong)) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::NumberFormat::set_pattern(const char* value, unsigned long size)```</span>
      ///
      ///
      ///
      /// ## Variant 3
      ///
      /// Rust arguments: ```fn set_pattern(&mut self, &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::NumberFormat::set_pattern(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& value)```</span>
      ///
      ///
      pub fn set_pattern<'l0, Args>(&'l0 mut self, args: Args) -> ()
        where Args: overloading::NumberFormatSetPatternArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::NumberFormat::Swap(i18n::phonenumbers::NumberFormat* other)```</span>
      ///
      ///
      pub fn swap(&mut self, other: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_Swap(self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, other) }
      }

      /// C++ method: <span style='color: green;'>```const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& i18n::phonenumbers::NumberFormat::unknown_fields() const```</span>
      ///
      ///
      pub fn unknown_fields<'l0>
        (&'l0 self)
         -> &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_unknown_fields(self as *const ::phonemetadata::i18n::phonenumbers::NumberFormat) };
        unsafe { &*ffi_result }
      }
    }

    impl Drop for NumberFormat {
      /// C++ method: <span style='color: green;'>```virtual [destructor] void i18n::phonenumbers::NumberFormat::~NumberFormat()```</span>
      ///
      ///
      fn drop(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_destructor(self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat) }
      }
    }

    impl cpp_utils::CppDeletable for NumberFormat {
      fn deleter() -> cpp_utils::Deleter<Self> {
        ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_delete
      }
    }

    /// C++ type: <span style='color: green;'>```i18n::phonenumbers::PhoneMetadata```</span>.
    #[repr(C)]
    pub struct PhoneMetadata {
      _buffer: [u8; 280],
    }

    impl ::NewUninitialized for PhoneMetadata {
      unsafe fn new_uninitialized() -> PhoneMetadata {
        PhoneMetadata { _buffer: std::mem::uninitialized() }
      }
    }

    impl PhoneMetadata {
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::NumberFormat* i18n::phonenumbers::PhoneMetadata::add_intl_number_format()```</span>
      ///
      ///
      pub fn add_intl_number_format(&mut self) -> *mut ::phonemetadata::i18n::phonenumbers::NumberFormat {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_add_intl_number_format(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::NumberFormat* i18n::phonenumbers::PhoneMetadata::add_number_format()```</span>
      ///
      ///
      pub fn add_number_format(&mut self) -> *mut ::phonemetadata::i18n::phonenumbers::NumberFormat {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_add_number_format(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```virtual int i18n::phonenumbers::PhoneMetadata::ByteSize() const```</span>
      ///
      ///
      pub fn byte_size(&self) -> libc::c_int {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_ByteSize(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```const i18n::phonenumbers::PhoneNumberDesc& i18n::phonenumbers::PhoneMetadata::carrier_specific() const```</span>
      ///
      ///
      pub fn carrier_specific<'l0>(&'l0 self) -> &'l0 ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_carrier_specific(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) };
        unsafe { &*ffi_result }
      }

      /// C++ method: <span style='color: green;'>```virtual void i18n::phonenumbers::PhoneMetadata::CheckTypeAndMergeFrom(const google::protobuf::MessageLite& from)```</span>
      ///
      ///
      pub fn check_type_and_merge_from(&mut self, from: &::message_lite::google::protobuf::MessageLite) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_CheckTypeAndMergeFrom(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, from as *const ::message_lite::google::protobuf::MessageLite) }
      }

      /// C++ method: <span style='color: green;'>```virtual void i18n::phonenumbers::PhoneMetadata::Clear()```</span>
      ///
      ///
      pub fn clear(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_Clear(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::clear_carrier_specific()```</span>
      ///
      ///
      pub fn clear_carrier_specific(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_carrier_specific(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::clear_country_code()```</span>
      ///
      ///
      pub fn clear_country_code(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_country_code(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::clear_emergency()```</span>
      ///
      ///
      pub fn clear_emergency(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_emergency(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::clear_fixed_line()```</span>
      ///
      ///
      pub fn clear_fixed_line(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_fixed_line(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::clear_general_desc()```</span>
      ///
      ///
      pub fn clear_general_desc(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_general_desc(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::clear_id()```</span>
      ///
      ///
      pub fn clear_id(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_id(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::clear_international_prefix()```</span>
      ///
      ///
      pub fn clear_international_prefix(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_international_prefix(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::clear_intl_number_format()```</span>
      ///
      ///
      pub fn clear_intl_number_format(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_intl_number_format(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::clear_leading_digits()```</span>
      ///
      ///
      pub fn clear_leading_digits(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_leading_digits(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::clear_leading_zero_possible()```</span>
      ///
      ///
      pub fn clear_leading_zero_possible(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_leading_zero_possible(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::clear_main_country_for_code()```</span>
      ///
      ///
      pub fn clear_main_country_for_code(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_main_country_for_code(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::clear_mobile()```</span>
      ///
      ///
      pub fn clear_mobile(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_mobile(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::clear_mobile_number_portable_region()```</span>
      ///
      ///
      pub fn clear_mobile_number_portable_region(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_mobile_number_portable_region(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::clear_national_prefix()```</span>
      ///
      ///
      pub fn clear_national_prefix(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_national_prefix(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::clear_national_prefix_for_parsing()```</span>
      ///
      ///
      pub fn clear_national_prefix_for_parsing(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_national_prefix_for_parsing(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::clear_national_prefix_transform_rule()```</span>
      ///
      ///
      pub fn clear_national_prefix_transform_rule(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_national_prefix_transform_rule(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::clear_no_international_dialling()```</span>
      ///
      ///
      pub fn clear_no_international_dialling(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_no_international_dialling(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::clear_number_format()```</span>
      ///
      ///
      pub fn clear_number_format(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_number_format(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::clear_pager()```</span>
      ///
      ///
      pub fn clear_pager(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_pager(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::clear_personal_number()```</span>
      ///
      ///
      pub fn clear_personal_number(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_personal_number(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::clear_preferred_extn_prefix()```</span>
      ///
      ///
      pub fn clear_preferred_extn_prefix(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_preferred_extn_prefix(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::clear_preferred_international_prefix()```</span>
      ///
      ///
      pub fn clear_preferred_international_prefix(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_preferred_international_prefix(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::clear_premium_rate()```</span>
      ///
      ///
      pub fn clear_premium_rate(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_premium_rate(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::clear_same_mobile_and_fixed_line_pattern()```</span>
      ///
      ///
      pub fn clear_same_mobile_and_fixed_line_pattern(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_same_mobile_and_fixed_line_pattern(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::clear_shared_cost()```</span>
      ///
      ///
      pub fn clear_shared_cost(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_shared_cost(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::clear_short_code()```</span>
      ///
      ///
      pub fn clear_short_code(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_short_code(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::clear_standard_rate()```</span>
      ///
      ///
      pub fn clear_standard_rate(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_standard_rate(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::clear_toll_free()```</span>
      ///
      ///
      pub fn clear_toll_free(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_toll_free(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::clear_uan()```</span>
      ///
      ///
      pub fn clear_uan(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_uan(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::clear_voicemail()```</span>
      ///
      ///
      pub fn clear_voicemail(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_voicemail(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::clear_voip()```</span>
      ///
      ///
      pub fn clear_voip(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_voip(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::CopyFrom(const i18n::phonenumbers::PhoneMetadata& from)```</span>
      ///
      ///
      pub fn copy_from(&mut self, from: &::phonemetadata::i18n::phonenumbers::PhoneMetadata) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_CopyFrom(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, from as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```int i18n::phonenumbers::PhoneMetadata::country_code() const```</span>
      ///
      ///
      pub fn country_code(&self) -> libc::c_int {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_country_code(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```static const i18n::phonenumbers::PhoneMetadata& i18n::phonenumbers::PhoneMetadata::default_instance()```</span>
      ///
      ///
      pub fn default_instance() -> &'static ::phonemetadata::i18n::phonenumbers::PhoneMetadata {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_default_instance() };
        unsafe { &*ffi_result }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::DiscardUnknownFields()```</span>
      ///
      ///
      pub fn discard_unknown_fields(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_DiscardUnknownFields(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```const i18n::phonenumbers::PhoneNumberDesc& i18n::phonenumbers::PhoneMetadata::emergency() const```</span>
      ///
      ///
      pub fn emergency<'l0>(&'l0 self) -> &'l0 ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_emergency(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) };
        unsafe { &*ffi_result }
      }

      /// C++ method: <span style='color: green;'>```const i18n::phonenumbers::PhoneNumberDesc& i18n::phonenumbers::PhoneMetadata::fixed_line() const```</span>
      ///
      ///
      pub fn fixed_line<'l0>(&'l0 self) -> &'l0 ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_fixed_line(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) };
        unsafe { &*ffi_result }
      }

      /// C++ method: <span style='color: green;'>```const i18n::phonenumbers::PhoneNumberDesc& i18n::phonenumbers::PhoneMetadata::general_desc() const```</span>
      ///
      ///
      pub fn general_desc<'l0>(&'l0 self) -> &'l0 ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_general_desc(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) };
        unsafe { &*ffi_result }
      }

      /// C++ method: <span style='color: green;'>```virtual int i18n::phonenumbers::PhoneMetadata::GetCachedSize() const```</span>
      ///
      ///
      pub fn get_cached_size(&self) -> libc::c_int {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_GetCachedSize(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneMetadata::GetTypeName```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      /// Rust arguments: <br>1) ```fn get_type_name(&self, cpp_utils::AsBox) -> cpp_utils::CppBox<::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef>```<br>2) ```fn get_type_name(&self, cpp_utils::AsStruct) -> ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef```<br>
      /// C++ method: <span style='color: green;'>```virtual std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>> i18n::phonenumbers::PhoneMetadata::GetTypeName() const```</span>
      ///
      ///
      pub fn get_type_name<'l0, Args>(&'l0 self, args: Args) -> Args::ReturnType
        where Args: overloading::PhoneMetadataGetTypeNameArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneMetadata::has_carrier_specific() const```</span>
      ///
      ///
      pub fn has_carrier_specific(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_carrier_specific(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneMetadata::has_country_code() const```</span>
      ///
      ///
      pub fn has_country_code(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_country_code(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneMetadata::has_emergency() const```</span>
      ///
      ///
      pub fn has_emergency(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_emergency(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneMetadata::has_fixed_line() const```</span>
      ///
      ///
      pub fn has_fixed_line(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_fixed_line(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneMetadata::has_general_desc() const```</span>
      ///
      ///
      pub fn has_general_desc(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_general_desc(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneMetadata::has_id() const```</span>
      ///
      ///
      pub fn has_id(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_id(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneMetadata::has_international_prefix() const```</span>
      ///
      ///
      pub fn has_international_prefix(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_international_prefix(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneMetadata::has_leading_digits() const```</span>
      ///
      ///
      pub fn has_leading_digits(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_leading_digits(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneMetadata::has_leading_zero_possible() const```</span>
      ///
      ///
      pub fn has_leading_zero_possible(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_leading_zero_possible(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneMetadata::has_main_country_for_code() const```</span>
      ///
      ///
      pub fn has_main_country_for_code(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_main_country_for_code(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneMetadata::has_mobile() const```</span>
      ///
      ///
      pub fn has_mobile(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_mobile(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneMetadata::has_mobile_number_portable_region() const```</span>
      ///
      ///
      pub fn has_mobile_number_portable_region(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_mobile_number_portable_region(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneMetadata::has_national_prefix() const```</span>
      ///
      ///
      pub fn has_national_prefix(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_national_prefix(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneMetadata::has_national_prefix_for_parsing() const```</span>
      ///
      ///
      pub fn has_national_prefix_for_parsing(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_national_prefix_for_parsing(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneMetadata::has_national_prefix_transform_rule() const```</span>
      ///
      ///
      pub fn has_national_prefix_transform_rule(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_national_prefix_transform_rule(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneMetadata::has_no_international_dialling() const```</span>
      ///
      ///
      pub fn has_no_international_dialling(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_no_international_dialling(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneMetadata::has_pager() const```</span>
      ///
      ///
      pub fn has_pager(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_pager(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneMetadata::has_personal_number() const```</span>
      ///
      ///
      pub fn has_personal_number(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_personal_number(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneMetadata::has_preferred_extn_prefix() const```</span>
      ///
      ///
      pub fn has_preferred_extn_prefix(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_preferred_extn_prefix(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneMetadata::has_preferred_international_prefix() const```</span>
      ///
      ///
      pub fn has_preferred_international_prefix(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_preferred_international_prefix(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneMetadata::has_premium_rate() const```</span>
      ///
      ///
      pub fn has_premium_rate(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_premium_rate(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneMetadata::has_same_mobile_and_fixed_line_pattern() const```</span>
      ///
      ///
      pub fn has_same_mobile_and_fixed_line_pattern(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_same_mobile_and_fixed_line_pattern(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneMetadata::has_shared_cost() const```</span>
      ///
      ///
      pub fn has_shared_cost(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_shared_cost(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneMetadata::has_short_code() const```</span>
      ///
      ///
      pub fn has_short_code(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_short_code(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneMetadata::has_standard_rate() const```</span>
      ///
      ///
      pub fn has_standard_rate(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_standard_rate(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneMetadata::has_toll_free() const```</span>
      ///
      ///
      pub fn has_toll_free(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_toll_free(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneMetadata::has_uan() const```</span>
      ///
      ///
      pub fn has_uan(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_uan(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneMetadata::has_voicemail() const```</span>
      ///
      ///
      pub fn has_voicemail(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_voicemail(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneMetadata::has_voip() const```</span>
      ///
      ///
      pub fn has_voip(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_voip(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& i18n::phonenumbers::PhoneMetadata::id() const```</span>
      ///
      ///
      pub fn id<'l0>(&'l0 self)
                     -> &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_id(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) };
        unsafe { &*ffi_result }
      }

      /// C++ method: <span style='color: green;'>```const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& i18n::phonenumbers::PhoneMetadata::international_prefix() const```</span>
      ///
      ///
      pub fn international_prefix<'l0>
        (&'l0 self)
         -> &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_international_prefix(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) };
        unsafe { &*ffi_result }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneMetadata::intl_number_format```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn intl_number_format(&self, ()) -> &'l0 ::repeated_field::google::protobuf::RepeatedPtrFieldNumberFormatRef```<br>
      /// C++ method: <span style='color: green;'>```const google::protobuf::RepeatedPtrField<i18n::phonenumbers::NumberFormat>& i18n::phonenumbers::PhoneMetadata::intl_number_format() const```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn intl_number_format(&self, libc::c_int) -> &'l0 ::phonemetadata::i18n::phonenumbers::NumberFormat```<br>
      /// C++ method: <span style='color: green;'>```const i18n::phonenumbers::NumberFormat& i18n::phonenumbers::PhoneMetadata::intl_number_format(int index) const```</span>
      ///
      ///
      pub fn intl_number_format<'l0, Args>(&'l0 self, args: Args) -> Args::ReturnType
        where Args: overloading::PhoneMetadataIntlNumberFormatArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```int i18n::phonenumbers::PhoneMetadata::intl_number_format_size() const```</span>
      ///
      ///
      pub fn intl_number_format_size(&self) -> libc::c_int {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_intl_number_format_size(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```virtual bool i18n::phonenumbers::PhoneMetadata::IsInitialized() const```</span>
      ///
      ///
      pub fn is_initialized(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_IsInitialized(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& i18n::phonenumbers::PhoneMetadata::leading_digits() const```</span>
      ///
      ///
      pub fn leading_digits<'l0>
        (&'l0 self)
         -> &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_leading_digits(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) };
        unsafe { &*ffi_result }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneMetadata::leading_zero_possible() const```</span>
      ///
      ///
      pub fn leading_zero_possible(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_leading_zero_possible(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneMetadata::main_country_for_code() const```</span>
      ///
      ///
      pub fn main_country_for_code(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_main_country_for_code(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::MergeFrom(const i18n::phonenumbers::PhoneMetadata& from)```</span>
      ///
      ///
      pub fn merge_from(&mut self, from: &::phonemetadata::i18n::phonenumbers::PhoneMetadata) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_MergeFrom(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, from as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```const i18n::phonenumbers::PhoneNumberDesc& i18n::phonenumbers::PhoneMetadata::mobile() const```</span>
      ///
      ///
      pub fn mobile<'l0>(&'l0 self) -> &'l0 ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mobile(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) };
        unsafe { &*ffi_result }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneMetadata::mobile_number_portable_region() const```</span>
      ///
      ///
      pub fn mobile_number_portable_region(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mobile_number_portable_region(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberDesc* i18n::phonenumbers::PhoneMetadata::mutable_carrier_specific()```</span>
      ///
      ///
      pub fn mutable_carrier_specific(&mut self) -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_carrier_specific(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberDesc* i18n::phonenumbers::PhoneMetadata::mutable_emergency()```</span>
      ///
      ///
      pub fn mutable_emergency(&mut self) -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_emergency(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberDesc* i18n::phonenumbers::PhoneMetadata::mutable_fixed_line()```</span>
      ///
      ///
      pub fn mutable_fixed_line(&mut self) -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_fixed_line(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberDesc* i18n::phonenumbers::PhoneMetadata::mutable_general_desc()```</span>
      ///
      ///
      pub fn mutable_general_desc(&mut self) -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_general_desc(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* i18n::phonenumbers::PhoneMetadata::mutable_id()```</span>
      ///
      ///
      pub fn mutable_id(&mut self)
                        -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_id(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* i18n::phonenumbers::PhoneMetadata::mutable_international_prefix()```</span>
      ///
      ///
      pub fn mutable_international_prefix
        (&mut self)
         -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_international_prefix(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneMetadata::mutable_intl_number_format```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn mutable_intl_number_format(&mut self, ()) -> *mut ::repeated_field::google::protobuf::RepeatedPtrFieldNumberFormatRef```<br>
      /// C++ method: <span style='color: green;'>```google::protobuf::RepeatedPtrField<i18n::phonenumbers::NumberFormat>* i18n::phonenumbers::PhoneMetadata::mutable_intl_number_format()```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn mutable_intl_number_format(&mut self, libc::c_int) -> *mut ::phonemetadata::i18n::phonenumbers::NumberFormat```<br>
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::NumberFormat* i18n::phonenumbers::PhoneMetadata::mutable_intl_number_format(int index)```</span>
      ///
      ///
      pub fn mutable_intl_number_format<'l0, Args>(&'l0 mut self, args: Args) -> Args::ReturnType
        where Args: overloading::PhoneMetadataMutableIntlNumberFormatArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* i18n::phonenumbers::PhoneMetadata::mutable_leading_digits()```</span>
      ///
      ///
      pub fn mutable_leading_digits
        (&mut self)
         -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_leading_digits(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberDesc* i18n::phonenumbers::PhoneMetadata::mutable_mobile()```</span>
      ///
      ///
      pub fn mutable_mobile(&mut self) -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_mobile(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* i18n::phonenumbers::PhoneMetadata::mutable_national_prefix()```</span>
      ///
      ///
      pub fn mutable_national_prefix
        (&mut self)
         -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_national_prefix(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* i18n::phonenumbers::PhoneMetadata::mutable_national_prefix_for_parsing()```</span>
      ///
      ///
      pub fn mutable_national_prefix_for_parsing
        (&mut self)
         -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_national_prefix_for_parsing(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* i18n::phonenumbers::PhoneMetadata::mutable_national_prefix_transform_rule()```</span>
      ///
      ///
      pub fn mutable_national_prefix_transform_rule
        (&mut self)
         -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_national_prefix_transform_rule(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberDesc* i18n::phonenumbers::PhoneMetadata::mutable_no_international_dialling()```</span>
      ///
      ///
      pub fn mutable_no_international_dialling(&mut self) -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_no_international_dialling(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneMetadata::mutable_number_format```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn mutable_number_format(&mut self, ()) -> *mut ::repeated_field::google::protobuf::RepeatedPtrFieldNumberFormatRef```<br>
      /// C++ method: <span style='color: green;'>```google::protobuf::RepeatedPtrField<i18n::phonenumbers::NumberFormat>* i18n::phonenumbers::PhoneMetadata::mutable_number_format()```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn mutable_number_format(&mut self, libc::c_int) -> *mut ::phonemetadata::i18n::phonenumbers::NumberFormat```<br>
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::NumberFormat* i18n::phonenumbers::PhoneMetadata::mutable_number_format(int index)```</span>
      ///
      ///
      pub fn mutable_number_format<'l0, Args>(&'l0 mut self, args: Args) -> Args::ReturnType
        where Args: overloading::PhoneMetadataMutableNumberFormatArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberDesc* i18n::phonenumbers::PhoneMetadata::mutable_pager()```</span>
      ///
      ///
      pub fn mutable_pager(&mut self) -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_pager(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberDesc* i18n::phonenumbers::PhoneMetadata::mutable_personal_number()```</span>
      ///
      ///
      pub fn mutable_personal_number(&mut self) -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_personal_number(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* i18n::phonenumbers::PhoneMetadata::mutable_preferred_extn_prefix()```</span>
      ///
      ///
      pub fn mutable_preferred_extn_prefix
        (&mut self)
         -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_preferred_extn_prefix(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* i18n::phonenumbers::PhoneMetadata::mutable_preferred_international_prefix()```</span>
      ///
      ///
      pub fn mutable_preferred_international_prefix
        (&mut self)
         -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_preferred_international_prefix(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberDesc* i18n::phonenumbers::PhoneMetadata::mutable_premium_rate()```</span>
      ///
      ///
      pub fn mutable_premium_rate(&mut self) -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_premium_rate(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberDesc* i18n::phonenumbers::PhoneMetadata::mutable_shared_cost()```</span>
      ///
      ///
      pub fn mutable_shared_cost(&mut self) -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_shared_cost(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberDesc* i18n::phonenumbers::PhoneMetadata::mutable_short_code()```</span>
      ///
      ///
      pub fn mutable_short_code(&mut self) -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_short_code(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberDesc* i18n::phonenumbers::PhoneMetadata::mutable_standard_rate()```</span>
      ///
      ///
      pub fn mutable_standard_rate(&mut self) -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_standard_rate(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberDesc* i18n::phonenumbers::PhoneMetadata::mutable_toll_free()```</span>
      ///
      ///
      pub fn mutable_toll_free(&mut self) -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_toll_free(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberDesc* i18n::phonenumbers::PhoneMetadata::mutable_uan()```</span>
      ///
      ///
      pub fn mutable_uan(&mut self) -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_uan(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* i18n::phonenumbers::PhoneMetadata::mutable_unknown_fields()```</span>
      ///
      ///
      pub fn mutable_unknown_fields
        (&mut self)
         -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_unknown_fields(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberDesc* i18n::phonenumbers::PhoneMetadata::mutable_voicemail()```</span>
      ///
      ///
      pub fn mutable_voicemail(&mut self) -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_voicemail(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberDesc* i18n::phonenumbers::PhoneMetadata::mutable_voip()```</span>
      ///
      ///
      pub fn mutable_voip(&mut self) -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_voip(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& i18n::phonenumbers::PhoneMetadata::national_prefix() const```</span>
      ///
      ///
      pub fn national_prefix<'l0>
        (&'l0 self)
         -> &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_national_prefix(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) };
        unsafe { &*ffi_result }
      }

      /// C++ method: <span style='color: green;'>```const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& i18n::phonenumbers::PhoneMetadata::national_prefix_for_parsing() const```</span>
      ///
      ///
      pub fn national_prefix_for_parsing<'l0>
        (&'l0 self)
         -> &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_national_prefix_for_parsing(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) };
        unsafe { &*ffi_result }
      }

      /// C++ method: <span style='color: green;'>```const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& i18n::phonenumbers::PhoneMetadata::national_prefix_transform_rule() const```</span>
      ///
      ///
      pub fn national_prefix_transform_rule<'l0>
        (&'l0 self)
         -> &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_national_prefix_transform_rule(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) };
        unsafe { &*ffi_result }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneMetadata::New```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn new(&self, ()) -> *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata```<br>
      /// C++ method: <span style='color: green;'>```virtual i18n::phonenumbers::PhoneMetadata* i18n::phonenumbers::PhoneMetadata::New() const```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn new(&self, *mut ::arena::google::protobuf::Arena) -> *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata```<br>
      /// C++ method: <span style='color: green;'>```virtual i18n::phonenumbers::PhoneMetadata* i18n::phonenumbers::PhoneMetadata::New(google::protobuf::Arena* arena) const```</span>
      ///
      ///
      pub fn new<'l0, Args>(&'l0 self, args: Args) -> *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata
        where Args: overloading::PhoneMetadataNewArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneMetadata::PhoneMetadata```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: <br>1) ```fn new_static(cpp_utils::AsStruct) -> ::phonemetadata::i18n::phonenumbers::PhoneMetadata```<br>2) ```fn new_static(cpp_utils::AsBox) -> cpp_utils::CppBox<::phonemetadata::i18n::phonenumbers::PhoneMetadata>```<br>
      /// C++ method: <span style='color: green;'>```[constructor] void i18n::phonenumbers::PhoneMetadata::PhoneMetadata()```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: <br>1) ```fn new_static((&::phonemetadata::i18n::phonenumbers::PhoneMetadata, cpp_utils::AsStruct)) -> ::phonemetadata::i18n::phonenumbers::PhoneMetadata```<br>2) ```fn new_static((&::phonemetadata::i18n::phonenumbers::PhoneMetadata, cpp_utils::AsBox)) -> cpp_utils::CppBox<::phonemetadata::i18n::phonenumbers::PhoneMetadata>```<br>
      /// C++ method: <span style='color: green;'>```[constructor] void i18n::phonenumbers::PhoneMetadata::PhoneMetadata(const i18n::phonenumbers::PhoneMetadata& from)```</span>
      ///
      ///
      pub fn new_static<Args>(args: Args) -> Args::ReturnType
        where Args: overloading::PhoneMetadataNewStaticArgs
      {
        args.exec()
      }
      /// C++ method: <span style='color: green;'>```const i18n::phonenumbers::PhoneNumberDesc& i18n::phonenumbers::PhoneMetadata::no_international_dialling() const```</span>
      ///
      ///
      pub fn no_international_dialling<'l0>(&'l0 self) -> &'l0 ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_no_international_dialling(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) };
        unsafe { &*ffi_result }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneMetadata::number_format```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn number_format(&self, ()) -> &'l0 ::repeated_field::google::protobuf::RepeatedPtrFieldNumberFormatRef```<br>
      /// C++ method: <span style='color: green;'>```const google::protobuf::RepeatedPtrField<i18n::phonenumbers::NumberFormat>& i18n::phonenumbers::PhoneMetadata::number_format() const```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn number_format(&self, libc::c_int) -> &'l0 ::phonemetadata::i18n::phonenumbers::NumberFormat```<br>
      /// C++ method: <span style='color: green;'>```const i18n::phonenumbers::NumberFormat& i18n::phonenumbers::PhoneMetadata::number_format(int index) const```</span>
      ///
      ///
      pub fn number_format<'l0, Args>(&'l0 self, args: Args) -> Args::ReturnType
        where Args: overloading::PhoneMetadataNumberFormatArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```int i18n::phonenumbers::PhoneMetadata::number_format_size() const```</span>
      ///
      ///
      pub fn number_format_size(&self) -> libc::c_int {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_number_format_size(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneMetadata& i18n::phonenumbers::PhoneMetadata::operator=(const i18n::phonenumbers::PhoneMetadata& from)```</span>
      ///
      ///
      pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                                 from: &'l1 ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
                                 -> &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_operator_assign(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, from as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) };
        unsafe { &mut *ffi_result }
      }

      /// C++ method: <span style='color: green;'>```const i18n::phonenumbers::PhoneNumberDesc& i18n::phonenumbers::PhoneMetadata::pager() const```</span>
      ///
      ///
      pub fn pager<'l0>(&'l0 self) -> &'l0 ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_pager(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) };
        unsafe { &*ffi_result }
      }

      /// C++ method: <span style='color: green;'>```const i18n::phonenumbers::PhoneNumberDesc& i18n::phonenumbers::PhoneMetadata::personal_number() const```</span>
      ///
      ///
      pub fn personal_number<'l0>(&'l0 self) -> &'l0 ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_personal_number(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) };
        unsafe { &*ffi_result }
      }

      /// C++ method: <span style='color: green;'>```const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& i18n::phonenumbers::PhoneMetadata::preferred_extn_prefix() const```</span>
      ///
      ///
      pub fn preferred_extn_prefix<'l0>
        (&'l0 self)
         -> &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_preferred_extn_prefix(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) };
        unsafe { &*ffi_result }
      }

      /// C++ method: <span style='color: green;'>```const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& i18n::phonenumbers::PhoneMetadata::preferred_international_prefix() const```</span>
      ///
      ///
      pub fn preferred_international_prefix<'l0>
        (&'l0 self)
         -> &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_preferred_international_prefix(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) };
        unsafe { &*ffi_result }
      }

      /// C++ method: <span style='color: green;'>```const i18n::phonenumbers::PhoneNumberDesc& i18n::phonenumbers::PhoneMetadata::premium_rate() const```</span>
      ///
      ///
      pub fn premium_rate<'l0>(&'l0 self) -> &'l0 ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_premium_rate(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) };
        unsafe { &*ffi_result }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberDesc* i18n::phonenumbers::PhoneMetadata::release_carrier_specific()```</span>
      ///
      ///
      pub fn release_carrier_specific(&mut self) -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_carrier_specific(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberDesc* i18n::phonenumbers::PhoneMetadata::release_emergency()```</span>
      ///
      ///
      pub fn release_emergency(&mut self) -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_emergency(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberDesc* i18n::phonenumbers::PhoneMetadata::release_fixed_line()```</span>
      ///
      ///
      pub fn release_fixed_line(&mut self) -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_fixed_line(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberDesc* i18n::phonenumbers::PhoneMetadata::release_general_desc()```</span>
      ///
      ///
      pub fn release_general_desc(&mut self) -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_general_desc(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* i18n::phonenumbers::PhoneMetadata::release_id()```</span>
      ///
      ///
      pub fn release_id(&mut self)
                        -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_id(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* i18n::phonenumbers::PhoneMetadata::release_international_prefix()```</span>
      ///
      ///
      pub fn release_international_prefix
        (&mut self)
         -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_international_prefix(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* i18n::phonenumbers::PhoneMetadata::release_leading_digits()```</span>
      ///
      ///
      pub fn release_leading_digits
        (&mut self)
         -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_leading_digits(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberDesc* i18n::phonenumbers::PhoneMetadata::release_mobile()```</span>
      ///
      ///
      pub fn release_mobile(&mut self) -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_mobile(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* i18n::phonenumbers::PhoneMetadata::release_national_prefix()```</span>
      ///
      ///
      pub fn release_national_prefix
        (&mut self)
         -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_national_prefix(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* i18n::phonenumbers::PhoneMetadata::release_national_prefix_for_parsing()```</span>
      ///
      ///
      pub fn release_national_prefix_for_parsing
        (&mut self)
         -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_national_prefix_for_parsing(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* i18n::phonenumbers::PhoneMetadata::release_national_prefix_transform_rule()```</span>
      ///
      ///
      pub fn release_national_prefix_transform_rule
        (&mut self)
         -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_national_prefix_transform_rule(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberDesc* i18n::phonenumbers::PhoneMetadata::release_no_international_dialling()```</span>
      ///
      ///
      pub fn release_no_international_dialling(&mut self) -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_no_international_dialling(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberDesc* i18n::phonenumbers::PhoneMetadata::release_pager()```</span>
      ///
      ///
      pub fn release_pager(&mut self) -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_pager(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberDesc* i18n::phonenumbers::PhoneMetadata::release_personal_number()```</span>
      ///
      ///
      pub fn release_personal_number(&mut self) -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_personal_number(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* i18n::phonenumbers::PhoneMetadata::release_preferred_extn_prefix()```</span>
      ///
      ///
      pub fn release_preferred_extn_prefix
        (&mut self)
         -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_preferred_extn_prefix(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* i18n::phonenumbers::PhoneMetadata::release_preferred_international_prefix()```</span>
      ///
      ///
      pub fn release_preferred_international_prefix
        (&mut self)
         -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_preferred_international_prefix(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberDesc* i18n::phonenumbers::PhoneMetadata::release_premium_rate()```</span>
      ///
      ///
      pub fn release_premium_rate(&mut self) -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_premium_rate(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberDesc* i18n::phonenumbers::PhoneMetadata::release_shared_cost()```</span>
      ///
      ///
      pub fn release_shared_cost(&mut self) -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_shared_cost(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberDesc* i18n::phonenumbers::PhoneMetadata::release_short_code()```</span>
      ///
      ///
      pub fn release_short_code(&mut self) -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_short_code(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberDesc* i18n::phonenumbers::PhoneMetadata::release_standard_rate()```</span>
      ///
      ///
      pub fn release_standard_rate(&mut self) -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_standard_rate(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberDesc* i18n::phonenumbers::PhoneMetadata::release_toll_free()```</span>
      ///
      ///
      pub fn release_toll_free(&mut self) -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_toll_free(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberDesc* i18n::phonenumbers::PhoneMetadata::release_uan()```</span>
      ///
      ///
      pub fn release_uan(&mut self) -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_uan(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberDesc* i18n::phonenumbers::PhoneMetadata::release_voicemail()```</span>
      ///
      ///
      pub fn release_voicemail(&mut self) -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_voicemail(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberDesc* i18n::phonenumbers::PhoneMetadata::release_voip()```</span>
      ///
      ///
      pub fn release_voip(&mut self) -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_voip(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneMetadata::same_mobile_and_fixed_line_pattern() const```</span>
      ///
      ///
      pub fn same_mobile_and_fixed_line_pattern(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_same_mobile_and_fixed_line_pattern(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_allocated_carrier_specific(i18n::phonenumbers::PhoneNumberDesc* carrier_specific)```</span>
      ///
      ///
pub fn set_allocated_carrier_specific(&mut self, carrier_specific: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_carrier_specific(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, carrier_specific) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_allocated_emergency(i18n::phonenumbers::PhoneNumberDesc* emergency)```</span>
      ///
      ///
      pub fn set_allocated_emergency(&mut self, emergency: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_emergency(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, emergency) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_allocated_fixed_line(i18n::phonenumbers::PhoneNumberDesc* fixed_line)```</span>
      ///
      ///
      pub fn set_allocated_fixed_line(&mut self,
                                      fixed_line: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_fixed_line(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, fixed_line) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_allocated_general_desc(i18n::phonenumbers::PhoneNumberDesc* general_desc)```</span>
      ///
      ///
      pub fn set_allocated_general_desc(&mut self,
                                        general_desc: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_general_desc(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, general_desc) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_allocated_id(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* id)```</span>
      ///
      ///
pub fn set_allocated_id(&mut self, id: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_id(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, id) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_allocated_international_prefix(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* international_prefix)```</span>
      ///
      ///
pub fn set_allocated_international_prefix(&mut self, international_prefix: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_international_prefix(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, international_prefix) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_allocated_leading_digits(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* leading_digits)```</span>
      ///
      ///
pub fn set_allocated_leading_digits(&mut self, leading_digits: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_leading_digits(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, leading_digits) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_allocated_mobile(i18n::phonenumbers::PhoneNumberDesc* mobile)```</span>
      ///
      ///
      pub fn set_allocated_mobile(&mut self, mobile: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_mobile(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, mobile) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_allocated_national_prefix(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* national_prefix)```</span>
      ///
      ///
pub fn set_allocated_national_prefix(&mut self, national_prefix: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_national_prefix(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, national_prefix) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_allocated_national_prefix_for_parsing(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* national_prefix_for_parsing)```</span>
      ///
      ///
pub fn set_allocated_national_prefix_for_parsing(&mut self, national_prefix_for_parsing: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_national_prefix_for_parsing(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, national_prefix_for_parsing) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_allocated_national_prefix_transform_rule(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* national_prefix_transform_rule)```</span>
      ///
      ///
pub fn set_allocated_national_prefix_transform_rule(&mut self, national_prefix_transform_rule: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_national_prefix_transform_rule(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, national_prefix_transform_rule) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_allocated_no_international_dialling(i18n::phonenumbers::PhoneNumberDesc* no_international_dialling)```</span>
      ///
      ///
pub fn set_allocated_no_international_dialling(&mut self, no_international_dialling: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_no_international_dialling(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, no_international_dialling) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_allocated_pager(i18n::phonenumbers::PhoneNumberDesc* pager)```</span>
      ///
      ///
      pub fn set_allocated_pager(&mut self, pager: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_pager(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, pager) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_allocated_personal_number(i18n::phonenumbers::PhoneNumberDesc* personal_number)```</span>
      ///
      ///
pub fn set_allocated_personal_number(&mut self, personal_number: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_personal_number(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, personal_number) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_allocated_preferred_extn_prefix(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* preferred_extn_prefix)```</span>
      ///
      ///
pub fn set_allocated_preferred_extn_prefix(&mut self, preferred_extn_prefix: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_preferred_extn_prefix(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, preferred_extn_prefix) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_allocated_preferred_international_prefix(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* preferred_international_prefix)```</span>
      ///
      ///
pub fn set_allocated_preferred_international_prefix(&mut self, preferred_international_prefix: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_preferred_international_prefix(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, preferred_international_prefix) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_allocated_premium_rate(i18n::phonenumbers::PhoneNumberDesc* premium_rate)```</span>
      ///
      ///
      pub fn set_allocated_premium_rate(&mut self,
                                        premium_rate: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_premium_rate(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, premium_rate) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_allocated_shared_cost(i18n::phonenumbers::PhoneNumberDesc* shared_cost)```</span>
      ///
      ///
      pub fn set_allocated_shared_cost(&mut self,
                                       shared_cost: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_shared_cost(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, shared_cost) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_allocated_short_code(i18n::phonenumbers::PhoneNumberDesc* short_code)```</span>
      ///
      ///
      pub fn set_allocated_short_code(&mut self,
                                      short_code: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_short_code(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, short_code) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_allocated_standard_rate(i18n::phonenumbers::PhoneNumberDesc* standard_rate)```</span>
      ///
      ///
      pub fn set_allocated_standard_rate(&mut self,
                                         standard_rate: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_standard_rate(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, standard_rate) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_allocated_toll_free(i18n::phonenumbers::PhoneNumberDesc* toll_free)```</span>
      ///
      ///
      pub fn set_allocated_toll_free(&mut self, toll_free: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_toll_free(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, toll_free) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_allocated_uan(i18n::phonenumbers::PhoneNumberDesc* uan)```</span>
      ///
      ///
      pub fn set_allocated_uan(&mut self, uan: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_uan(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, uan) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_allocated_voicemail(i18n::phonenumbers::PhoneNumberDesc* voicemail)```</span>
      ///
      ///
      pub fn set_allocated_voicemail(&mut self, voicemail: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_voicemail(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, voicemail) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_allocated_voip(i18n::phonenumbers::PhoneNumberDesc* voip)```</span>
      ///
      ///
      pub fn set_allocated_voip(&mut self, voip: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_voip(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, voip) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_country_code(int value)```</span>
      ///
      ///
      pub fn set_country_code(&mut self, value: libc::c_int) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_country_code(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneMetadata::set_id```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn set_id(&mut self, *const libc::c_char) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_id(const char* value)```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn set_id(&mut self, (*const libc::c_char, libc::c_ulong)) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_id(const char* value, unsigned long size)```</span>
      ///
      ///
      ///
      /// ## Variant 3
      ///
      /// Rust arguments: ```fn set_id(&mut self, &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_id(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& value)```</span>
      ///
      ///
      pub fn set_id<'l0, Args>(&'l0 mut self, args: Args) -> ()
        where Args: overloading::PhoneMetadataSetIdArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneMetadata::set_international_prefix```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn set_international_prefix(&mut self, *const libc::c_char) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_international_prefix(const char* value)```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn set_international_prefix(&mut self, (*const libc::c_char, libc::c_ulong)) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_international_prefix(const char* value, unsigned long size)```</span>
      ///
      ///
      ///
      /// ## Variant 3
      ///
      /// Rust arguments: ```fn set_international_prefix(&mut self, &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_international_prefix(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& value)```</span>
      ///
      ///
      pub fn set_international_prefix<'l0, Args>(&'l0 mut self, args: Args) -> ()
        where Args: overloading::PhoneMetadataSetInternationalPrefixArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneMetadata::set_leading_digits```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn set_leading_digits(&mut self, *const libc::c_char) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_leading_digits(const char* value)```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn set_leading_digits(&mut self, (*const libc::c_char, libc::c_ulong)) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_leading_digits(const char* value, unsigned long size)```</span>
      ///
      ///
      ///
      /// ## Variant 3
      ///
      /// Rust arguments: ```fn set_leading_digits(&mut self, &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_leading_digits(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& value)```</span>
      ///
      ///
      pub fn set_leading_digits<'l0, Args>(&'l0 mut self, args: Args) -> ()
        where Args: overloading::PhoneMetadataSetLeadingDigitsArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_leading_zero_possible(bool value)```</span>
      ///
      ///
      pub fn set_leading_zero_possible(&mut self, value: bool) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_leading_zero_possible(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_main_country_for_code(bool value)```</span>
      ///
      ///
      pub fn set_main_country_for_code(&mut self, value: bool) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_main_country_for_code(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_mobile_number_portable_region(bool value)```</span>
      ///
      ///
      pub fn set_mobile_number_portable_region(&mut self, value: bool) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_mobile_number_portable_region(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneMetadata::set_national_prefix```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn set_national_prefix(&mut self, *const libc::c_char) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_national_prefix(const char* value)```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn set_national_prefix(&mut self, (*const libc::c_char, libc::c_ulong)) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_national_prefix(const char* value, unsigned long size)```</span>
      ///
      ///
      ///
      /// ## Variant 3
      ///
      /// Rust arguments: ```fn set_national_prefix(&mut self, &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_national_prefix(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& value)```</span>
      ///
      ///
      pub fn set_national_prefix<'l0, Args>(&'l0 mut self, args: Args) -> ()
        where Args: overloading::PhoneMetadataSetNationalPrefixArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneMetadata::set_national_prefix_for_parsing```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn set_national_prefix_for_parsing(&mut self, *const libc::c_char) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_national_prefix_for_parsing(const char* value)```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn set_national_prefix_for_parsing(&mut self, (*const libc::c_char, libc::c_ulong)) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_national_prefix_for_parsing(const char* value, unsigned long size)```</span>
      ///
      ///
      ///
      /// ## Variant 3
      ///
      /// Rust arguments: ```fn set_national_prefix_for_parsing(&mut self, &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_national_prefix_for_parsing(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& value)```</span>
      ///
      ///
      pub fn set_national_prefix_for_parsing<'l0, Args>(&'l0 mut self, args: Args) -> ()
        where Args: overloading::PhoneMetadataSetNationalPrefixForParsingArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneMetadata::set_national_prefix_transform_rule```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn set_national_prefix_transform_rule(&mut self, *const libc::c_char) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_national_prefix_transform_rule(const char* value)```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn set_national_prefix_transform_rule(&mut self, (*const libc::c_char, libc::c_ulong)) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_national_prefix_transform_rule(const char* value, unsigned long size)```</span>
      ///
      ///
      ///
      /// ## Variant 3
      ///
      /// Rust arguments: ```fn set_national_prefix_transform_rule(&mut self, &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_national_prefix_transform_rule(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& value)```</span>
      ///
      ///
      pub fn set_national_prefix_transform_rule<'l0, Args>(&'l0 mut self, args: Args) -> ()
        where Args: overloading::PhoneMetadataSetNationalPrefixTransformRuleArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneMetadata::set_preferred_extn_prefix```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn set_preferred_extn_prefix(&mut self, *const libc::c_char) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_preferred_extn_prefix(const char* value)```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn set_preferred_extn_prefix(&mut self, (*const libc::c_char, libc::c_ulong)) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_preferred_extn_prefix(const char* value, unsigned long size)```</span>
      ///
      ///
      ///
      /// ## Variant 3
      ///
      /// Rust arguments: ```fn set_preferred_extn_prefix(&mut self, &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_preferred_extn_prefix(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& value)```</span>
      ///
      ///
      pub fn set_preferred_extn_prefix<'l0, Args>(&'l0 mut self, args: Args) -> ()
        where Args: overloading::PhoneMetadataSetPreferredExtnPrefixArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneMetadata::set_preferred_international_prefix```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn set_preferred_international_prefix(&mut self, *const libc::c_char) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_preferred_international_prefix(const char* value)```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn set_preferred_international_prefix(&mut self, (*const libc::c_char, libc::c_ulong)) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_preferred_international_prefix(const char* value, unsigned long size)```</span>
      ///
      ///
      ///
      /// ## Variant 3
      ///
      /// Rust arguments: ```fn set_preferred_international_prefix(&mut self, &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_preferred_international_prefix(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& value)```</span>
      ///
      ///
      pub fn set_preferred_international_prefix<'l0, Args>(&'l0 mut self, args: Args) -> ()
        where Args: overloading::PhoneMetadataSetPreferredInternationalPrefixArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::set_same_mobile_and_fixed_line_pattern(bool value)```</span>
      ///
      ///
      pub fn set_same_mobile_and_fixed_line_pattern(&mut self, value: bool) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_same_mobile_and_fixed_line_pattern(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value) }
      }

      /// C++ method: <span style='color: green;'>```const i18n::phonenumbers::PhoneNumberDesc& i18n::phonenumbers::PhoneMetadata::shared_cost() const```</span>
      ///
      ///
      pub fn shared_cost<'l0>(&'l0 self) -> &'l0 ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_shared_cost(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) };
        unsafe { &*ffi_result }
      }

      /// C++ method: <span style='color: green;'>```const i18n::phonenumbers::PhoneNumberDesc& i18n::phonenumbers::PhoneMetadata::short_code() const```</span>
      ///
      ///
      pub fn short_code<'l0>(&'l0 self) -> &'l0 ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_short_code(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) };
        unsafe { &*ffi_result }
      }

      /// C++ method: <span style='color: green;'>```const i18n::phonenumbers::PhoneNumberDesc& i18n::phonenumbers::PhoneMetadata::standard_rate() const```</span>
      ///
      ///
      pub fn standard_rate<'l0>(&'l0 self) -> &'l0 ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_standard_rate(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) };
        unsafe { &*ffi_result }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadata::Swap(i18n::phonenumbers::PhoneMetadata* other)```</span>
      ///
      ///
      pub fn swap(&mut self, other: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_Swap(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, other) }
      }

      /// C++ method: <span style='color: green;'>```const i18n::phonenumbers::PhoneNumberDesc& i18n::phonenumbers::PhoneMetadata::toll_free() const```</span>
      ///
      ///
      pub fn toll_free<'l0>(&'l0 self) -> &'l0 ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_toll_free(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) };
        unsafe { &*ffi_result }
      }

      /// C++ method: <span style='color: green;'>```const i18n::phonenumbers::PhoneNumberDesc& i18n::phonenumbers::PhoneMetadata::uan() const```</span>
      ///
      ///
      pub fn uan<'l0>(&'l0 self) -> &'l0 ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_uan(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) };
        unsafe { &*ffi_result }
      }

      /// C++ method: <span style='color: green;'>```const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& i18n::phonenumbers::PhoneMetadata::unknown_fields() const```</span>
      ///
      ///
      pub fn unknown_fields<'l0>
        (&'l0 self)
         -> &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_unknown_fields(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) };
        unsafe { &*ffi_result }
      }

      /// C++ method: <span style='color: green;'>```const i18n::phonenumbers::PhoneNumberDesc& i18n::phonenumbers::PhoneMetadata::voicemail() const```</span>
      ///
      ///
      pub fn voicemail<'l0>(&'l0 self) -> &'l0 ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_voicemail(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) };
        unsafe { &*ffi_result }
      }

      /// C++ method: <span style='color: green;'>```const i18n::phonenumbers::PhoneNumberDesc& i18n::phonenumbers::PhoneMetadata::voip() const```</span>
      ///
      ///
      pub fn voip<'l0>(&'l0 self) -> &'l0 ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_voip(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) };
        unsafe { &*ffi_result }
      }
    }

    impl Drop for PhoneMetadata {
      /// C++ method: <span style='color: green;'>```virtual [destructor] void i18n::phonenumbers::PhoneMetadata::~PhoneMetadata()```</span>
      ///
      ///
      fn drop(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_destructor(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
      }
    }

    impl cpp_utils::CppDeletable for PhoneMetadata {
      fn deleter() -> cpp_utils::Deleter<Self> {
        ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_delete
      }
    }

    /// C++ type: <span style='color: green;'>```i18n::phonenumbers::PhoneMetadataCollection```</span>.
    #[repr(C)]
    pub struct PhoneMetadataCollection {
      _buffer: [u8; 56],
    }

    impl ::NewUninitialized for PhoneMetadataCollection {
      unsafe fn new_uninitialized() -> PhoneMetadataCollection {
        PhoneMetadataCollection { _buffer: std::mem::uninitialized() }
      }
    }

    impl PhoneMetadataCollection {
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneMetadata* i18n::phonenumbers::PhoneMetadataCollection::add_metadata()```</span>
      ///
      ///
      pub fn add_metadata(&mut self) -> *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_add_metadata(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection) }
      }

      /// C++ method: <span style='color: green;'>```virtual int i18n::phonenumbers::PhoneMetadataCollection::ByteSize() const```</span>
      ///
      ///
      pub fn byte_size(&self) -> libc::c_int {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_ByteSize(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection) }
      }

      /// C++ method: <span style='color: green;'>```virtual void i18n::phonenumbers::PhoneMetadataCollection::CheckTypeAndMergeFrom(const google::protobuf::MessageLite& from)```</span>
      ///
      ///
      pub fn check_type_and_merge_from(&mut self, from: &::message_lite::google::protobuf::MessageLite) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_CheckTypeAndMergeFrom(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection, from as *const ::message_lite::google::protobuf::MessageLite) }
      }

      /// C++ method: <span style='color: green;'>```virtual void i18n::phonenumbers::PhoneMetadataCollection::Clear()```</span>
      ///
      ///
      pub fn clear(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_Clear(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadataCollection::clear_metadata()```</span>
      ///
      ///
      pub fn clear_metadata(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_clear_metadata(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadataCollection::CopyFrom(const i18n::phonenumbers::PhoneMetadataCollection& from)```</span>
      ///
      ///
      pub fn copy_from(&mut self, from: &::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_CopyFrom(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection, from as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection) }
      }

      /// C++ method: <span style='color: green;'>```static const i18n::phonenumbers::PhoneMetadataCollection& i18n::phonenumbers::PhoneMetadataCollection::default_instance()```</span>
      ///
      ///
      pub fn default_instance() -> &'static ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection {
        let ffi_result =
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_default_instance() };
        unsafe { &*ffi_result }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadataCollection::DiscardUnknownFields()```</span>
      ///
      ///
      pub fn discard_unknown_fields(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_DiscardUnknownFields(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection) }
      }

      /// C++ method: <span style='color: green;'>```virtual int i18n::phonenumbers::PhoneMetadataCollection::GetCachedSize() const```</span>
      ///
      ///
      pub fn get_cached_size(&self) -> libc::c_int {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_GetCachedSize(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneMetadataCollection::GetTypeName```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      /// Rust arguments: <br>1) ```fn get_type_name(&self, cpp_utils::AsBox) -> cpp_utils::CppBox<::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef>```<br>2) ```fn get_type_name(&self, cpp_utils::AsStruct) -> ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef```<br>
      /// C++ method: <span style='color: green;'>```virtual std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>> i18n::phonenumbers::PhoneMetadataCollection::GetTypeName() const```</span>
      ///
      ///
      pub fn get_type_name<'l0, Args>(&'l0 self, args: Args) -> Args::ReturnType
        where Args: overloading::PhoneMetadataCollectionGetTypeNameArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```virtual bool i18n::phonenumbers::PhoneMetadataCollection::IsInitialized() const```</span>
      ///
      ///
      pub fn is_initialized(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_IsInitialized(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadataCollection::MergeFrom(const i18n::phonenumbers::PhoneMetadataCollection& from)```</span>
      ///
      ///
      pub fn merge_from(&mut self, from: &::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_MergeFrom(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection, from as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneMetadataCollection::metadata```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn metadata(&self, ()) -> &'l0 ::repeated_field::google::protobuf::RepeatedPtrFieldPhoneMetadataRef```<br>
      /// C++ method: <span style='color: green;'>```const google::protobuf::RepeatedPtrField<i18n::phonenumbers::PhoneMetadata>& i18n::phonenumbers::PhoneMetadataCollection::metadata() const```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn metadata(&self, libc::c_int) -> &'l0 ::phonemetadata::i18n::phonenumbers::PhoneMetadata```<br>
      /// C++ method: <span style='color: green;'>```const i18n::phonenumbers::PhoneMetadata& i18n::phonenumbers::PhoneMetadataCollection::metadata(int index) const```</span>
      ///
      ///
      pub fn metadata<'l0, Args>(&'l0 self, args: Args) -> Args::ReturnType
        where Args: overloading::PhoneMetadataCollectionMetadataArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```int i18n::phonenumbers::PhoneMetadataCollection::metadata_size() const```</span>
      ///
      ///
      pub fn metadata_size(&self) -> libc::c_int {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_metadata_size(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneMetadataCollection::mutable_metadata```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn mutable_metadata(&mut self, ()) -> *mut ::repeated_field::google::protobuf::RepeatedPtrFieldPhoneMetadataRef```<br>
      /// C++ method: <span style='color: green;'>```google::protobuf::RepeatedPtrField<i18n::phonenumbers::PhoneMetadata>* i18n::phonenumbers::PhoneMetadataCollection::mutable_metadata()```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn mutable_metadata(&mut self, libc::c_int) -> *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata```<br>
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneMetadata* i18n::phonenumbers::PhoneMetadataCollection::mutable_metadata(int index)```</span>
      ///
      ///
      pub fn mutable_metadata<'l0, Args>(&'l0 mut self, args: Args) -> Args::ReturnType
        where Args: overloading::PhoneMetadataCollectionMutableMetadataArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* i18n::phonenumbers::PhoneMetadataCollection::mutable_unknown_fields()```</span>
      ///
      ///
      pub fn mutable_unknown_fields
        (&mut self)
         -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_mutable_unknown_fields(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneMetadataCollection::New```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn new(&self, ()) -> *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection```<br>
      /// C++ method: <span style='color: green;'>```virtual i18n::phonenumbers::PhoneMetadataCollection* i18n::phonenumbers::PhoneMetadataCollection::New() const```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn new(&self, *mut ::arena::google::protobuf::Arena) -> *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection```<br>
      /// C++ method: <span style='color: green;'>```virtual i18n::phonenumbers::PhoneMetadataCollection* i18n::phonenumbers::PhoneMetadataCollection::New(google::protobuf::Arena* arena) const```</span>
      ///
      ///
      pub fn new<'l0, Args>(&'l0 self, args: Args) -> *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection
        where Args: overloading::PhoneMetadataCollectionNewArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneMetadataCollection::PhoneMetadataCollection```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: <br>1) ```fn new_static(cpp_utils::AsStruct) -> ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection```<br>2) ```fn new_static(cpp_utils::AsBox) -> cpp_utils::CppBox<::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection>```<br>
      /// C++ method: <span style='color: green;'>```[constructor] void i18n::phonenumbers::PhoneMetadataCollection::PhoneMetadataCollection()```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: <br>1) ```fn new_static((&::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection, cpp_utils::AsStruct)) -> ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection```<br>2) ```fn new_static((&::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection, cpp_utils::AsBox)) -> cpp_utils::CppBox<::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection>```<br>
      /// C++ method: <span style='color: green;'>```[constructor] void i18n::phonenumbers::PhoneMetadataCollection::PhoneMetadataCollection(const i18n::phonenumbers::PhoneMetadataCollection& from)```</span>
      ///
      ///
      pub fn new_static<Args>(args: Args) -> Args::ReturnType
        where Args: overloading::PhoneMetadataCollectionNewStaticArgs
      {
        args.exec()
      }
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneMetadataCollection& i18n::phonenumbers::PhoneMetadataCollection::operator=(const i18n::phonenumbers::PhoneMetadataCollection& from)```</span>
      ///
      ///
      pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                                 from: &'l1 ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection)
                                 -> &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_operator_assign(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection, from as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection) };
        unsafe { &mut *ffi_result }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneMetadataCollection::Swap(i18n::phonenumbers::PhoneMetadataCollection* other)```</span>
      ///
      ///
      pub fn swap(&mut self, other: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_Swap(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection, other) }
      }

      /// C++ method: <span style='color: green;'>```const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& i18n::phonenumbers::PhoneMetadataCollection::unknown_fields() const```</span>
      ///
      ///
      pub fn unknown_fields<'l0>
        (&'l0 self)
         -> &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_unknown_fields(self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection) };
        unsafe { &*ffi_result }
      }
    }

    impl Drop for PhoneMetadataCollection {
      /// C++ method: <span style='color: green;'>```virtual [destructor] void i18n::phonenumbers::PhoneMetadataCollection::~PhoneMetadataCollection()```</span>
      ///
      ///
      fn drop(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_destructor(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection) }
      }
    }

    impl cpp_utils::CppDeletable for PhoneMetadataCollection {
      fn deleter() -> cpp_utils::Deleter<Self> {
        ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_delete
      }
    }

    /// C++ type: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberDesc```</span>.
    #[repr(C)]
    pub struct PhoneNumberDesc {
      _buffer: [u8; 88],
    }

    impl ::NewUninitialized for PhoneNumberDesc {
      unsafe fn new_uninitialized() -> PhoneNumberDesc {
        PhoneNumberDesc { _buffer: std::mem::uninitialized() }
      }
    }

    impl PhoneNumberDesc {
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberDesc::add_possible_length(int value)```</span>
      ///
      ///
      pub fn add_possible_length(&mut self, value: libc::c_int) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_add_possible_length(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, value) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberDesc::add_possible_length_local_only(int value)```</span>
      ///
      ///
      pub fn add_possible_length_local_only(&mut self, value: libc::c_int) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_add_possible_length_local_only(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, value) }
      }

      /// C++ method: <span style='color: green;'>```virtual int i18n::phonenumbers::PhoneNumberDesc::ByteSize() const```</span>
      ///
      ///
      pub fn byte_size(&self) -> libc::c_int {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_ByteSize(self as *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) }
      }

      /// C++ method: <span style='color: green;'>```virtual void i18n::phonenumbers::PhoneNumberDesc::CheckTypeAndMergeFrom(const google::protobuf::MessageLite& from)```</span>
      ///
      ///
      pub fn check_type_and_merge_from(&mut self, from: &::message_lite::google::protobuf::MessageLite) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_CheckTypeAndMergeFrom(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, from as *const ::message_lite::google::protobuf::MessageLite) }
      }

      /// C++ method: <span style='color: green;'>```virtual void i18n::phonenumbers::PhoneNumberDesc::Clear()```</span>
      ///
      ///
      pub fn clear(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_Clear(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberDesc::clear_example_number()```</span>
      ///
      ///
      pub fn clear_example_number(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_clear_example_number(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberDesc::clear_national_number_pattern()```</span>
      ///
      ///
      pub fn clear_national_number_pattern(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_clear_national_number_pattern(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberDesc::clear_possible_length()```</span>
      ///
      ///
      pub fn clear_possible_length(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_clear_possible_length(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberDesc::clear_possible_length_local_only()```</span>
      ///
      ///
      pub fn clear_possible_length_local_only(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_clear_possible_length_local_only(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberDesc::clear_possible_number_pattern()```</span>
      ///
      ///
      pub fn clear_possible_number_pattern(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_clear_possible_number_pattern(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberDesc::CopyFrom(const i18n::phonenumbers::PhoneNumberDesc& from)```</span>
      ///
      ///
      pub fn copy_from(&mut self, from: &::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_CopyFrom(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, from as *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) }
      }

      /// C++ method: <span style='color: green;'>```static const i18n::phonenumbers::PhoneNumberDesc& i18n::phonenumbers::PhoneNumberDesc::default_instance()```</span>
      ///
      ///
      pub fn default_instance() -> &'static ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_default_instance() };
        unsafe { &*ffi_result }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberDesc::DiscardUnknownFields()```</span>
      ///
      ///
      pub fn discard_unknown_fields(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_DiscardUnknownFields(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) }
      }

      /// C++ method: <span style='color: green;'>```const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& i18n::phonenumbers::PhoneNumberDesc::example_number() const```</span>
      ///
      ///
      pub fn example_number<'l0>
        (&'l0 self)
         -> &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_example_number(self as *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) };
        unsafe { &*ffi_result }
      }

      /// C++ method: <span style='color: green;'>```virtual int i18n::phonenumbers::PhoneNumberDesc::GetCachedSize() const```</span>
      ///
      ///
      pub fn get_cached_size(&self) -> libc::c_int {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_GetCachedSize(self as *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberDesc::GetTypeName```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      /// Rust arguments: <br>1) ```fn get_type_name(&self, cpp_utils::AsBox) -> cpp_utils::CppBox<::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef>```<br>2) ```fn get_type_name(&self, cpp_utils::AsStruct) -> ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef```<br>
      /// C++ method: <span style='color: green;'>```virtual std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>> i18n::phonenumbers::PhoneNumberDesc::GetTypeName() const```</span>
      ///
      ///
      pub fn get_type_name<'l0, Args>(&'l0 self, args: Args) -> Args::ReturnType
        where Args: overloading::PhoneNumberDescGetTypeNameArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneNumberDesc::has_example_number() const```</span>
      ///
      ///
      pub fn has_example_number(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_has_example_number(self as *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneNumberDesc::has_national_number_pattern() const```</span>
      ///
      ///
      pub fn has_national_number_pattern(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_has_national_number_pattern(self as *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneNumberDesc::has_possible_number_pattern() const```</span>
      ///
      ///
      pub fn has_possible_number_pattern(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_has_possible_number_pattern(self as *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) }
      }

      /// C++ method: <span style='color: green;'>```virtual bool i18n::phonenumbers::PhoneNumberDesc::IsInitialized() const```</span>
      ///
      ///
      pub fn is_initialized(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_IsInitialized(self as *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberDesc::MergeFrom(const i18n::phonenumbers::PhoneNumberDesc& from)```</span>
      ///
      ///
      pub fn merge_from(&mut self, from: &::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_MergeFrom(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, from as *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) }
      }

      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* i18n::phonenumbers::PhoneNumberDesc::mutable_example_number()```</span>
      ///
      ///
      pub fn mutable_example_number
        (&mut self)
         -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_mutable_example_number(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) }
      }

      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* i18n::phonenumbers::PhoneNumberDesc::mutable_national_number_pattern()```</span>
      ///
      ///
      pub fn mutable_national_number_pattern
        (&mut self)
         -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_mutable_national_number_pattern(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) }
      }

      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* i18n::phonenumbers::PhoneNumberDesc::mutable_possible_number_pattern()```</span>
      ///
      ///
      pub fn mutable_possible_number_pattern
        (&mut self)
         -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_mutable_possible_number_pattern(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) }
      }

      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* i18n::phonenumbers::PhoneNumberDesc::mutable_unknown_fields()```</span>
      ///
      ///
      pub fn mutable_unknown_fields
        (&mut self)
         -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_mutable_unknown_fields(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) }
      }

      /// C++ method: <span style='color: green;'>```const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& i18n::phonenumbers::PhoneNumberDesc::national_number_pattern() const```</span>
      ///
      ///
      pub fn national_number_pattern<'l0>
        (&'l0 self)
         -> &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_national_number_pattern(self as *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) };
        unsafe { &*ffi_result }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberDesc::New```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn new(&self, ()) -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc```<br>
      /// C++ method: <span style='color: green;'>```virtual i18n::phonenumbers::PhoneNumberDesc* i18n::phonenumbers::PhoneNumberDesc::New() const```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn new(&self, *mut ::arena::google::protobuf::Arena) -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc```<br>
      /// C++ method: <span style='color: green;'>```virtual i18n::phonenumbers::PhoneNumberDesc* i18n::phonenumbers::PhoneNumberDesc::New(google::protobuf::Arena* arena) const```</span>
      ///
      ///
      pub fn new<'l0, Args>(&'l0 self, args: Args) -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc
        where Args: overloading::PhoneNumberDescNewArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberDesc::PhoneNumberDesc```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: <br>1) ```fn new_static(cpp_utils::AsStruct) -> ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc```<br>2) ```fn new_static(cpp_utils::AsBox) -> cpp_utils::CppBox<::phonemetadata::i18n::phonenumbers::PhoneNumberDesc>```<br>
      /// C++ method: <span style='color: green;'>```[constructor] void i18n::phonenumbers::PhoneNumberDesc::PhoneNumberDesc()```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: <br>1) ```fn new_static((&::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, cpp_utils::AsStruct)) -> ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc```<br>2) ```fn new_static((&::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, cpp_utils::AsBox)) -> cpp_utils::CppBox<::phonemetadata::i18n::phonenumbers::PhoneNumberDesc>```<br>
      /// C++ method: <span style='color: green;'>```[constructor] void i18n::phonenumbers::PhoneNumberDesc::PhoneNumberDesc(const i18n::phonenumbers::PhoneNumberDesc& from)```</span>
      ///
      ///
      pub fn new_static<Args>(args: Args) -> Args::ReturnType
        where Args: overloading::PhoneNumberDescNewStaticArgs
      {
        args.exec()
      }
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberDesc& i18n::phonenumbers::PhoneNumberDesc::operator=(const i18n::phonenumbers::PhoneNumberDesc& from)```</span>
      ///
      ///
      pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                                 from: &'l1 ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc)
                                 -> &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_operator_assign(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, from as *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) };
        unsafe { &mut *ffi_result }
      }

      /// C++ method: <span style='color: green;'>```int i18n::phonenumbers::PhoneNumberDesc::possible_length_local_only_size() const```</span>
      ///
      ///
      pub fn possible_length_local_only_size(&self) -> libc::c_int {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_possible_length_local_only_size(self as *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) }
      }

      /// C++ method: <span style='color: green;'>```int i18n::phonenumbers::PhoneNumberDesc::possible_length_size() const```</span>
      ///
      ///
      pub fn possible_length_size(&self) -> libc::c_int {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_possible_length_size(self as *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) }
      }

      /// C++ method: <span style='color: green;'>```const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& i18n::phonenumbers::PhoneNumberDesc::possible_number_pattern() const```</span>
      ///
      ///
      pub fn possible_number_pattern<'l0>
        (&'l0 self)
         -> &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_possible_number_pattern(self as *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) };
        unsafe { &*ffi_result }
      }

      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* i18n::phonenumbers::PhoneNumberDesc::release_example_number()```</span>
      ///
      ///
      pub fn release_example_number
        (&mut self)
         -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_release_example_number(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) }
      }

      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* i18n::phonenumbers::PhoneNumberDesc::release_national_number_pattern()```</span>
      ///
      ///
      pub fn release_national_number_pattern
        (&mut self)
         -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_release_national_number_pattern(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) }
      }

      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* i18n::phonenumbers::PhoneNumberDesc::release_possible_number_pattern()```</span>
      ///
      ///
      pub fn release_possible_number_pattern
        (&mut self)
         -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_release_possible_number_pattern(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberDesc::set_allocated_example_number(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* example_number)```</span>
      ///
      ///
pub fn set_allocated_example_number(&mut self, example_number: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_set_allocated_example_number(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, example_number) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberDesc::set_allocated_national_number_pattern(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* national_number_pattern)```</span>
      ///
      ///
pub fn set_allocated_national_number_pattern(&mut self, national_number_pattern: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_set_allocated_national_number_pattern(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, national_number_pattern) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberDesc::set_allocated_possible_number_pattern(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* possible_number_pattern)```</span>
      ///
      ///
pub fn set_allocated_possible_number_pattern(&mut self, possible_number_pattern: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_set_allocated_possible_number_pattern(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, possible_number_pattern) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberDesc::set_example_number```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn set_example_number(&mut self, *const libc::c_char) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberDesc::set_example_number(const char* value)```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn set_example_number(&mut self, (*const libc::c_char, libc::c_ulong)) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberDesc::set_example_number(const char* value, unsigned long size)```</span>
      ///
      ///
      ///
      /// ## Variant 3
      ///
      /// Rust arguments: ```fn set_example_number(&mut self, &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberDesc::set_example_number(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& value)```</span>
      ///
      ///
      pub fn set_example_number<'l0, Args>(&'l0 mut self, args: Args) -> ()
        where Args: overloading::PhoneNumberDescSetExampleNumberArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberDesc::set_national_number_pattern```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn set_national_number_pattern(&mut self, *const libc::c_char) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberDesc::set_national_number_pattern(const char* value)```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn set_national_number_pattern(&mut self, (*const libc::c_char, libc::c_ulong)) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberDesc::set_national_number_pattern(const char* value, unsigned long size)```</span>
      ///
      ///
      ///
      /// ## Variant 3
      ///
      /// Rust arguments: ```fn set_national_number_pattern(&mut self, &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberDesc::set_national_number_pattern(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& value)```</span>
      ///
      ///
      pub fn set_national_number_pattern<'l0, Args>(&'l0 mut self, args: Args) -> ()
        where Args: overloading::PhoneNumberDescSetNationalNumberPatternArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberDesc::set_possible_length(int index, int value)```</span>
      ///
      ///
      pub fn set_possible_length(&mut self, index: libc::c_int, value: libc::c_int) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_set_possible_length(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, index, value) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberDesc::set_possible_length_local_only(int index, int value)```</span>
      ///
      ///
      pub fn set_possible_length_local_only(&mut self, index: libc::c_int, value: libc::c_int) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_set_possible_length_local_only(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, index, value) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberDesc::set_possible_number_pattern```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn set_possible_number_pattern(&mut self, *const libc::c_char) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberDesc::set_possible_number_pattern(const char* value)```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn set_possible_number_pattern(&mut self, (*const libc::c_char, libc::c_ulong)) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberDesc::set_possible_number_pattern(const char* value, unsigned long size)```</span>
      ///
      ///
      ///
      /// ## Variant 3
      ///
      /// Rust arguments: ```fn set_possible_number_pattern(&mut self, &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberDesc::set_possible_number_pattern(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& value)```</span>
      ///
      ///
      pub fn set_possible_number_pattern<'l0, Args>(&'l0 mut self, args: Args) -> ()
        where Args: overloading::PhoneNumberDescSetPossibleNumberPatternArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberDesc::Swap(i18n::phonenumbers::PhoneNumberDesc* other)```</span>
      ///
      ///
      pub fn swap(&mut self, other: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_Swap(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, other) }
      }

      /// C++ method: <span style='color: green;'>```const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& i18n::phonenumbers::PhoneNumberDesc::unknown_fields() const```</span>
      ///
      ///
      pub fn unknown_fields<'l0>
        (&'l0 self)
         -> &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_unknown_fields(self as *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) };
        unsafe { &*ffi_result }
      }
    }

    impl Drop for PhoneNumberDesc {
      /// C++ method: <span style='color: green;'>```virtual [destructor] void i18n::phonenumbers::PhoneNumberDesc::~PhoneNumberDesc()```</span>
      ///
      ///
      fn drop(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_destructor(self as *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) }
      }
    }

    impl cpp_utils::CppDeletable for PhoneNumberDesc {
      fn deleter() -> cpp_utils::Deleter<Self> {
        ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_delete
      }
    }

    /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::protobuf_AddDesc_phonemetadata_2eproto()```</span>
    ///
    ///
    pub fn protobuf_add_desc_phonemetadata_2eproto() {
      unsafe { ::ffi::libphonenumber_sys_c_phonemetadata_G_i18n_phonenumbers_protobuf_AddDesc_phonemetadata_2eproto() }
    }

    /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::protobuf_AssignDesc_phonemetadata_2eproto()```</span>
    ///
    ///
    pub fn protobuf_assign_desc_phonemetadata_2eproto() {
      unsafe {
        ::ffi::libphonenumber_sys_c_phonemetadata_G_i18n_phonenumbers_protobuf_AssignDesc_phonemetadata_2eproto()
      }
    }

    /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::protobuf_ShutdownFile_phonemetadata_2eproto()```</span>
    ///
    ///
    pub fn protobuf_shutdown_file_phonemetadata_2eproto() {
      unsafe {
        ::ffi::libphonenumber_sys_c_phonemetadata_G_i18n_phonenumbers_protobuf_ShutdownFile_phonemetadata_2eproto()
      }
    }

    pub mod overloading {
      #[allow(unused_imports)]
      use {libc, cpp_utils, std};

      /// This trait represents a set of arguments accepted by [NumberFormat::add_leading_digits_pattern](../struct.NumberFormat.html#method.add_leading_digits_pattern) method.
      pub trait NumberFormatAddLeadingDigitsPatternArgs<'l0> {
        type ReturnType;
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::NumberFormat) -> Self::ReturnType;
      }
      impl<'l0> NumberFormatAddLeadingDigitsPatternArgs<'l0> for *const libc::c_char {
        type ReturnType = ();
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::NumberFormat) -> () {
          let value = self;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_add_leading_digits_pattern_char(original_self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, value) }
        }
      }
      impl<'l0> NumberFormatAddLeadingDigitsPatternArgs<'l0> for (*const libc::c_char, libc::c_ulong) {
        type ReturnType = ();
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::NumberFormat) -> () {
          let value = self.0;
          let size = self.1;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_add_leading_digits_pattern_char_unsigned_long(original_self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, value, size) }
        }
      }
      impl<'l0> NumberFormatAddLeadingDigitsPatternArgs<'l0> for () {
        type ReturnType = *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
        fn exec(self,
                original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::NumberFormat)
                -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {

          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_add_leading_digits_pattern_no_args(original_self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat) }
        }
      }
      impl<'l0> NumberFormatAddLeadingDigitsPatternArgs<'l0> for &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
  type ReturnType = ();
  fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::NumberFormat) -> () {
    let value = self;
    unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_add_leading_digits_pattern_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(original_self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, value as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
  }
}
      /// This trait represents a set of arguments accepted by [NumberFormat::get_type_name](../struct.NumberFormat.html#method.get_type_name) method.
      pub trait NumberFormatGetTypeNameArgs<'l0> {
        type ReturnType;
        fn exec(self, original_self: &'l0 ::phonemetadata::i18n::phonenumbers::NumberFormat) -> Self::ReturnType;
      }
      impl<'l0> NumberFormatGetTypeNameArgs<'l0> for cpp_utils::AsBox {
        type ReturnType = cpp_utils::CppBox<::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef>;
        fn exec
          (self,
           original_self: &'l0 ::phonemetadata::i18n::phonenumbers::NumberFormat)
           -> cpp_utils::CppBox<::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef> {

          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_GetTypeName_as_ptr(original_self as *const ::phonemetadata::i18n::phonenumbers::NumberFormat) };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
      impl<'l0> NumberFormatGetTypeNameArgs<'l0> for cpp_utils::AsStruct {
        type ReturnType = ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
        fn exec(self,
                original_self: &'l0 ::phonemetadata::i18n::phonenumbers::NumberFormat)
                -> ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {

          {
            let mut object: ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef =
              unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_GetTypeName_to_output(original_self as *const ::phonemetadata::i18n::phonenumbers::NumberFormat, &mut object) }
            object
          }
        }
      }
      /// This trait represents a set of arguments accepted by [NumberFormat::leading_digits_pattern](../struct.NumberFormat.html#method.leading_digits_pattern) method.
      pub trait NumberFormatLeadingDigitsPatternArgs<'l0> {
        type ReturnType;
        fn exec(self, original_self: &'l0 ::phonemetadata::i18n::phonenumbers::NumberFormat) -> Self::ReturnType;
      }
      impl<'l0> NumberFormatLeadingDigitsPatternArgs<'l0> for libc::c_int {
        type ReturnType = &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
        fn exec(self,
                original_self: &'l0 ::phonemetadata::i18n::phonenumbers::NumberFormat)
                -> &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
          let index = self;
          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_leading_digits_pattern_index(original_self as *const ::phonemetadata::i18n::phonenumbers::NumberFormat, index) };
          unsafe { &*ffi_result }
        }
      }
      impl<'l0> NumberFormatLeadingDigitsPatternArgs<'l0> for () {
        type ReturnType = &'l0 ::repeated_field::google::protobuf::RepeatedPtrFieldBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRef;
  fn exec(self, original_self: &'l0 ::phonemetadata::i18n::phonenumbers::NumberFormat) -> &'l0 ::repeated_field::google::protobuf::RepeatedPtrFieldBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRef {

          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_leading_digits_pattern_no_args(original_self as *const ::phonemetadata::i18n::phonenumbers::NumberFormat) };
          unsafe { &*ffi_result }
        }
      }
      /// This trait represents a set of arguments accepted by [NumberFormat::mutable_leading_digits_pattern](../struct.NumberFormat.html#method.mutable_leading_digits_pattern) method.
      pub trait NumberFormatMutableLeadingDigitsPatternArgs<'l0> {
        type ReturnType;
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::NumberFormat) -> Self::ReturnType;
      }
      impl<'l0> NumberFormatMutableLeadingDigitsPatternArgs<'l0> for libc::c_int {
        type ReturnType = *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
        fn exec(self,
                original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::NumberFormat)
                -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
          let index = self;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_mutable_leading_digits_pattern_index(original_self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, index) }
        }
      }
      impl<'l0> NumberFormatMutableLeadingDigitsPatternArgs<'l0> for () {
        type ReturnType = *mut ::repeated_field::google::protobuf::RepeatedPtrFieldBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRef;
  fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::NumberFormat) -> *mut ::repeated_field::google::protobuf::RepeatedPtrFieldBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRef {

          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_mutable_leading_digits_pattern_no_args(original_self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat) }
        }
      }
      /// This trait represents a set of arguments accepted by [NumberFormat::new](../struct.NumberFormat.html#method.new) method.
      pub trait NumberFormatNewArgs<'l0> {
        fn exec(self,
                original_self: &'l0 ::phonemetadata::i18n::phonenumbers::NumberFormat)
                -> *mut ::phonemetadata::i18n::phonenumbers::NumberFormat;
      }
      impl<'l0> NumberFormatNewArgs<'l0> for *mut ::arena::google::protobuf::Arena {
        fn exec(self,
                original_self: &'l0 ::phonemetadata::i18n::phonenumbers::NumberFormat)
                -> *mut ::phonemetadata::i18n::phonenumbers::NumberFormat {
          let arena = self;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_New_arena(original_self as *const ::phonemetadata::i18n::phonenumbers::NumberFormat, arena) }
        }
      }
      impl<'l0> NumberFormatNewArgs<'l0> for () {
        fn exec(self,
                original_self: &'l0 ::phonemetadata::i18n::phonenumbers::NumberFormat)
                -> *mut ::phonemetadata::i18n::phonenumbers::NumberFormat {

          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_New_no_args(original_self as *const ::phonemetadata::i18n::phonenumbers::NumberFormat) }
        }
      }
      /// This trait represents a set of arguments accepted by [NumberFormat::new_static](../struct.NumberFormat.html#method.new_static) method.
      pub trait NumberFormatNewStaticArgs {
        type ReturnType;
        fn exec(self) -> Self::ReturnType;
      }
      impl<'a> NumberFormatNewStaticArgs for (&'a ::phonemetadata::i18n::phonenumbers::NumberFormat, cpp_utils::AsStruct) {
        type ReturnType = ::phonemetadata::i18n::phonenumbers::NumberFormat;
        fn exec(self) -> ::phonemetadata::i18n::phonenumbers::NumberFormat {
          let from = self.0;
          {
            let mut object: ::phonemetadata::i18n::phonenumbers::NumberFormat =
              unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_constructor_from(from as *const ::phonemetadata::i18n::phonenumbers::NumberFormat, &mut object) }
            object
          }
        }
      }
      impl NumberFormatNewStaticArgs for cpp_utils::AsStruct {
        type ReturnType = ::phonemetadata::i18n::phonenumbers::NumberFormat;
        fn exec(self) -> ::phonemetadata::i18n::phonenumbers::NumberFormat {

          {
            let mut object: ::phonemetadata::i18n::phonenumbers::NumberFormat =
              unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_constructor_no_args(&mut object) }
            object
          }
        }
      }
      impl<'a> NumberFormatNewStaticArgs for (&'a ::phonemetadata::i18n::phonenumbers::NumberFormat, cpp_utils::AsBox) {
        type ReturnType = cpp_utils::CppBox<::phonemetadata::i18n::phonenumbers::NumberFormat>;
        fn exec(self) -> cpp_utils::CppBox<::phonemetadata::i18n::phonenumbers::NumberFormat> {
          let from = self.0;
          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_new_from(from as *const ::phonemetadata::i18n::phonenumbers::NumberFormat) };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
      impl NumberFormatNewStaticArgs for cpp_utils::AsBox {
        type ReturnType = cpp_utils::CppBox<::phonemetadata::i18n::phonenumbers::NumberFormat>;
        fn exec(self) -> cpp_utils::CppBox<::phonemetadata::i18n::phonenumbers::NumberFormat> {

          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_new_no_args() };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
      /// This trait represents a set of arguments accepted by [NumberFormat::set_domestic_carrier_code_formatting_rule](../struct.NumberFormat.html#method.set_domestic_carrier_code_formatting_rule) method.
      pub trait NumberFormatSetDomesticCarrierCodeFormattingRuleArgs<'l0> {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::NumberFormat) -> ();
      }
      impl<'l0> NumberFormatSetDomesticCarrierCodeFormattingRuleArgs<'l0> for *const libc::c_char {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::NumberFormat) -> () {
          let value = self;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_domestic_carrier_code_formatting_rule_char(original_self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, value) }
        }
      }
      impl<'l0> NumberFormatSetDomesticCarrierCodeFormattingRuleArgs<'l0> for (*const libc::c_char, libc::c_ulong) {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::NumberFormat) -> () {
          let value = self.0;
          let size = self.1;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_domestic_carrier_code_formatting_rule_char_unsigned_long(original_self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, value, size) }
        }
      }
      impl<'l0> NumberFormatSetDomesticCarrierCodeFormattingRuleArgs<'l0> for &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {

  fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::NumberFormat) -> () {
    let value = self;
    unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_domestic_carrier_code_formatting_rule_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(original_self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, value as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
  }
}
      /// This trait represents a set of arguments accepted by [NumberFormat::set_format](../struct.NumberFormat.html#method.set_format) method.
      pub trait NumberFormatSetFormatArgs<'l0> {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::NumberFormat) -> ();
      }
      impl<'l0> NumberFormatSetFormatArgs<'l0> for *const libc::c_char {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::NumberFormat) -> () {
          let value = self;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_format_char(original_self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, value) }
        }
      }
      impl<'l0> NumberFormatSetFormatArgs<'l0> for (*const libc::c_char, libc::c_ulong) {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::NumberFormat) -> () {
          let value = self.0;
          let size = self.1;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_format_char_unsigned_long(original_self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, value, size) }
        }
      }
      impl<'l0> NumberFormatSetFormatArgs<'l0> for &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {

  fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::NumberFormat) -> () {
    let value = self;
    unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_format_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(original_self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, value as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
  }
}
      /// This trait represents a set of arguments accepted by [NumberFormat::set_leading_digits_pattern](../struct.NumberFormat.html#method.set_leading_digits_pattern) method.
      pub trait NumberFormatSetLeadingDigitsPatternArgs<'l0> {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::NumberFormat) -> ();
      }
      impl<'l0> NumberFormatSetLeadingDigitsPatternArgs<'l0> for (libc::c_int, *const libc::c_char) {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::NumberFormat) -> () {
          let index = self.0;
          let value = self.1;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_leading_digits_pattern_int_char(original_self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, index, value) }
        }
      }
      impl<'l0> NumberFormatSetLeadingDigitsPatternArgs<'l0> for (libc::c_int, *const libc::c_char, libc::c_ulong) {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::NumberFormat) -> () {
          let index = self.0;
          let value = self.1;
          let size = self.2;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_leading_digits_pattern_int_char_unsigned_long(original_self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, index, value, size) }
        }
      }
      impl<'l0> NumberFormatSetLeadingDigitsPatternArgs<'l0> for (libc::c_int,&'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {

  fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::NumberFormat) -> () {
    let index = self.0;
let value = self.1;
    unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_leading_digits_pattern_int_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(original_self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, index, value as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
  }
}
      /// This trait represents a set of arguments accepted by [NumberFormat::set_national_prefix_formatting_rule](../struct.NumberFormat.html#method.set_national_prefix_formatting_rule) method.
      pub trait NumberFormatSetNationalPrefixFormattingRuleArgs<'l0> {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::NumberFormat) -> ();
      }
      impl<'l0> NumberFormatSetNationalPrefixFormattingRuleArgs<'l0> for *const libc::c_char {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::NumberFormat) -> () {
          let value = self;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_national_prefix_formatting_rule_char(original_self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, value) }
        }
      }
      impl<'l0> NumberFormatSetNationalPrefixFormattingRuleArgs<'l0> for (*const libc::c_char, libc::c_ulong) {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::NumberFormat) -> () {
          let value = self.0;
          let size = self.1;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_national_prefix_formatting_rule_char_unsigned_long(original_self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, value, size) }
        }
      }
      impl<'l0> NumberFormatSetNationalPrefixFormattingRuleArgs<'l0> for &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {

  fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::NumberFormat) -> () {
    let value = self;
    unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_national_prefix_formatting_rule_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(original_self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, value as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
  }
}
      /// This trait represents a set of arguments accepted by [NumberFormat::set_pattern](../struct.NumberFormat.html#method.set_pattern) method.
      pub trait NumberFormatSetPatternArgs<'l0> {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::NumberFormat) -> ();
      }
      impl<'l0> NumberFormatSetPatternArgs<'l0> for *const libc::c_char {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::NumberFormat) -> () {
          let value = self;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_pattern_char(original_self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, value) }
        }
      }
      impl<'l0> NumberFormatSetPatternArgs<'l0> for (*const libc::c_char, libc::c_ulong) {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::NumberFormat) -> () {
          let value = self.0;
          let size = self.1;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_pattern_char_unsigned_long(original_self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, value, size) }
        }
      }
      impl<'l0> NumberFormatSetPatternArgs<'l0> for &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {

  fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::NumberFormat) -> () {
    let value = self;
    unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_pattern_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(original_self as *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, value as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
  }
}
      /// This trait represents a set of arguments accepted by [PhoneMetadataCollection::get_type_name](../struct.PhoneMetadataCollection.html#method.get_type_name) method.
      pub trait PhoneMetadataCollectionGetTypeNameArgs<'l0> {
        type ReturnType;
        fn exec(self,
                original_self: &'l0 ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection)
                -> Self::ReturnType;
      }
      impl<'l0> PhoneMetadataCollectionGetTypeNameArgs<'l0> for cpp_utils::AsBox {
        type ReturnType = cpp_utils::CppBox<::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef>;
        fn exec
          (self,
           original_self: &'l0 ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection)
           -> cpp_utils::CppBox<::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef> {

          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_GetTypeName_as_ptr(original_self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection) };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
      impl<'l0> PhoneMetadataCollectionGetTypeNameArgs<'l0> for cpp_utils::AsStruct {
        type ReturnType = ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
        fn exec(self,
                original_self: &'l0 ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection)
                -> ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {

          {
            let mut object: ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef =
              unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_GetTypeName_to_output(original_self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection, &mut object) }
            object
          }
        }
      }
      /// This trait represents a set of arguments accepted by [PhoneMetadataCollection::metadata](../struct.PhoneMetadataCollection.html#method.metadata) method.
      pub trait PhoneMetadataCollectionMetadataArgs<'l0> {
        type ReturnType;
        fn exec(self,
                original_self: &'l0 ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection)
                -> Self::ReturnType;
      }
      impl<'l0> PhoneMetadataCollectionMetadataArgs<'l0> for libc::c_int {
        type ReturnType = &'l0 ::phonemetadata::i18n::phonenumbers::PhoneMetadata;
        fn exec(self,
                original_self: &'l0 ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection)
                -> &'l0 ::phonemetadata::i18n::phonenumbers::PhoneMetadata {
          let index = self;
          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_metadata_index(original_self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection, index) };
          unsafe { &*ffi_result }
        }
      }
      impl<'l0> PhoneMetadataCollectionMetadataArgs<'l0> for () {
        type ReturnType = &'l0 ::repeated_field::google::protobuf::RepeatedPtrFieldPhoneMetadataRef;
        fn exec(self,
                original_self: &'l0 ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection)
                -> &'l0 ::repeated_field::google::protobuf::RepeatedPtrFieldPhoneMetadataRef {

          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_metadata_no_args(original_self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection) };
          unsafe { &*ffi_result }
        }
      }
      /// This trait represents a set of arguments accepted by [PhoneMetadataCollection::mutable_metadata](../struct.PhoneMetadataCollection.html#method.mutable_metadata) method.
      pub trait PhoneMetadataCollectionMutableMetadataArgs<'l0> {
        type ReturnType;
        fn exec(self,
                original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection)
                -> Self::ReturnType;
      }
      impl<'l0> PhoneMetadataCollectionMutableMetadataArgs<'l0> for libc::c_int {
        type ReturnType = *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata;
        fn exec(self,
                original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection)
                -> *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata {
          let index = self;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_mutable_metadata_index(original_self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection, index) }
        }
      }
      impl<'l0> PhoneMetadataCollectionMutableMetadataArgs<'l0> for () {
        type ReturnType = *mut ::repeated_field::google::protobuf::RepeatedPtrFieldPhoneMetadataRef;
        fn exec(self,
                original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection)
                -> *mut ::repeated_field::google::protobuf::RepeatedPtrFieldPhoneMetadataRef {

          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_mutable_metadata_no_args(original_self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection) }
        }
      }
      /// This trait represents a set of arguments accepted by [PhoneMetadataCollection::new](../struct.PhoneMetadataCollection.html#method.new) method.
      pub trait PhoneMetadataCollectionNewArgs<'l0> {
        fn exec(self,
                original_self: &'l0 ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection)
                -> *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection;
      }
      impl<'l0> PhoneMetadataCollectionNewArgs<'l0> for *mut ::arena::google::protobuf::Arena {
        fn exec(self,
                original_self: &'l0 ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection)
                -> *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection {
          let arena = self;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_New_arena(original_self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection, arena) }
        }
      }
      impl<'l0> PhoneMetadataCollectionNewArgs<'l0> for () {
        fn exec(self,
                original_self: &'l0 ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection)
                -> *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection {

          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_New_no_args(original_self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection) }
        }
      }
      /// This trait represents a set of arguments accepted by [PhoneMetadataCollection::new_static](../struct.PhoneMetadataCollection.html#method.new_static) method.
      pub trait PhoneMetadataCollectionNewStaticArgs {
        type ReturnType;
        fn exec(self) -> Self::ReturnType;
      }
      impl<'a> PhoneMetadataCollectionNewStaticArgs for (&'a ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection,cpp_utils::AsStruct) {
  type ReturnType = ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection;
  fn exec(self, ) -> ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection {
    let from = self.0;
    {
let mut object: ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection = unsafe { ::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_constructor_from(from as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection, &mut object) }object
}
  }
}
      impl PhoneMetadataCollectionNewStaticArgs for cpp_utils::AsStruct {
        type ReturnType = ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection;
        fn exec(self) -> ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection {

          {
            let mut object: ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection =
              unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe {
              ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_constructor_no_args(&mut object)
            }
            object
          }
        }
      }
      impl<'a> PhoneMetadataCollectionNewStaticArgs for (&'a ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection,cpp_utils::AsBox) {
  type ReturnType = cpp_utils::CppBox<::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection>;
  fn exec(self, ) -> cpp_utils::CppBox<::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection> {
    let from = self.0;
    let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_new_from(from as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection) };
unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }
}
      impl PhoneMetadataCollectionNewStaticArgs for cpp_utils::AsBox {
        type ReturnType = cpp_utils::CppBox<::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection>;
        fn exec(self) -> cpp_utils::CppBox<::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection> {

          let ffi_result =
            unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_new_no_args() };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
      /// This trait represents a set of arguments accepted by [PhoneMetadata::get_type_name](../struct.PhoneMetadata.html#method.get_type_name) method.
      pub trait PhoneMetadataGetTypeNameArgs<'l0> {
        type ReturnType;
        fn exec(self, original_self: &'l0 ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> Self::ReturnType;
      }
      impl<'l0> PhoneMetadataGetTypeNameArgs<'l0> for cpp_utils::AsBox {
        type ReturnType = cpp_utils::CppBox<::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef>;
        fn exec
          (self,
           original_self: &'l0 ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
           -> cpp_utils::CppBox<::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef> {

          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_GetTypeName_as_ptr(original_self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
      impl<'l0> PhoneMetadataGetTypeNameArgs<'l0> for cpp_utils::AsStruct {
        type ReturnType = ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
        fn exec(self,
                original_self: &'l0 ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
                -> ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {

          {
            let mut object: ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef =
              unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_GetTypeName_to_output(original_self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata, &mut object) }
            object
          }
        }
      }
      /// This trait represents a set of arguments accepted by [PhoneMetadata::intl_number_format](../struct.PhoneMetadata.html#method.intl_number_format) method.
      pub trait PhoneMetadataIntlNumberFormatArgs<'l0> {
        type ReturnType;
        fn exec(self, original_self: &'l0 ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> Self::ReturnType;
      }
      impl<'l0> PhoneMetadataIntlNumberFormatArgs<'l0> for libc::c_int {
        type ReturnType = &'l0 ::phonemetadata::i18n::phonenumbers::NumberFormat;
        fn exec(self,
                original_self: &'l0 ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
                -> &'l0 ::phonemetadata::i18n::phonenumbers::NumberFormat {
          let index = self;
          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_intl_number_format_index(original_self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata, index) };
          unsafe { &*ffi_result }
        }
      }
      impl<'l0> PhoneMetadataIntlNumberFormatArgs<'l0> for () {
        type ReturnType = &'l0 ::repeated_field::google::protobuf::RepeatedPtrFieldNumberFormatRef;
        fn exec(self,
                original_self: &'l0 ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
                -> &'l0 ::repeated_field::google::protobuf::RepeatedPtrFieldNumberFormatRef {

          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_intl_number_format_no_args(original_self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) };
          unsafe { &*ffi_result }
        }
      }
      /// This trait represents a set of arguments accepted by [PhoneMetadata::mutable_intl_number_format](../struct.PhoneMetadata.html#method.mutable_intl_number_format) method.
      pub trait PhoneMetadataMutableIntlNumberFormatArgs<'l0> {
        type ReturnType;
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> Self::ReturnType;
      }
      impl<'l0> PhoneMetadataMutableIntlNumberFormatArgs<'l0> for libc::c_int {
        type ReturnType = *mut ::phonemetadata::i18n::phonenumbers::NumberFormat;
        fn exec(self,
                original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
                -> *mut ::phonemetadata::i18n::phonenumbers::NumberFormat {
          let index = self;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_intl_number_format_index(original_self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, index) }
        }
      }
      impl<'l0> PhoneMetadataMutableIntlNumberFormatArgs<'l0> for () {
        type ReturnType = *mut ::repeated_field::google::protobuf::RepeatedPtrFieldNumberFormatRef;
        fn exec(self,
                original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
                -> *mut ::repeated_field::google::protobuf::RepeatedPtrFieldNumberFormatRef {

          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_intl_number_format_no_args(original_self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
        }
      }
      /// This trait represents a set of arguments accepted by [PhoneMetadata::mutable_number_format](../struct.PhoneMetadata.html#method.mutable_number_format) method.
      pub trait PhoneMetadataMutableNumberFormatArgs<'l0> {
        type ReturnType;
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> Self::ReturnType;
      }
      impl<'l0> PhoneMetadataMutableNumberFormatArgs<'l0> for libc::c_int {
        type ReturnType = *mut ::phonemetadata::i18n::phonenumbers::NumberFormat;
        fn exec(self,
                original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
                -> *mut ::phonemetadata::i18n::phonenumbers::NumberFormat {
          let index = self;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_number_format_index(original_self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, index) }
        }
      }
      impl<'l0> PhoneMetadataMutableNumberFormatArgs<'l0> for () {
        type ReturnType = *mut ::repeated_field::google::protobuf::RepeatedPtrFieldNumberFormatRef;
        fn exec(self,
                original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
                -> *mut ::repeated_field::google::protobuf::RepeatedPtrFieldNumberFormatRef {

          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_number_format_no_args(original_self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
        }
      }
      /// This trait represents a set of arguments accepted by [PhoneMetadata::new](../struct.PhoneMetadata.html#method.new) method.
      pub trait PhoneMetadataNewArgs<'l0> {
        fn exec(self,
                original_self: &'l0 ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
                -> *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata;
      }
      impl<'l0> PhoneMetadataNewArgs<'l0> for *mut ::arena::google::protobuf::Arena {
        fn exec(self,
                original_self: &'l0 ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
                -> *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata {
          let arena = self;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_New_arena(original_self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata, arena) }
        }
      }
      impl<'l0> PhoneMetadataNewArgs<'l0> for () {
        fn exec(self,
                original_self: &'l0 ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
                -> *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata {

          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_New_no_args(original_self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) }
        }
      }
      /// This trait represents a set of arguments accepted by [PhoneMetadata::new_static](../struct.PhoneMetadata.html#method.new_static) method.
      pub trait PhoneMetadataNewStaticArgs {
        type ReturnType;
        fn exec(self) -> Self::ReturnType;
      }
      impl<'a> PhoneMetadataNewStaticArgs for (&'a ::phonemetadata::i18n::phonenumbers::PhoneMetadata, cpp_utils::AsStruct) {
        type ReturnType = ::phonemetadata::i18n::phonenumbers::PhoneMetadata;
        fn exec(self) -> ::phonemetadata::i18n::phonenumbers::PhoneMetadata {
          let from = self.0;
          {
            let mut object: ::phonemetadata::i18n::phonenumbers::PhoneMetadata =
              unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_constructor_from(from as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata, &mut object) }
            object
          }
        }
      }
      impl PhoneMetadataNewStaticArgs for cpp_utils::AsStruct {
        type ReturnType = ::phonemetadata::i18n::phonenumbers::PhoneMetadata;
        fn exec(self) -> ::phonemetadata::i18n::phonenumbers::PhoneMetadata {

          {
            let mut object: ::phonemetadata::i18n::phonenumbers::PhoneMetadata =
              unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_constructor_no_args(&mut object) }
            object
          }
        }
      }
      impl<'a> PhoneMetadataNewStaticArgs for (&'a ::phonemetadata::i18n::phonenumbers::PhoneMetadata, cpp_utils::AsBox) {
        type ReturnType = cpp_utils::CppBox<::phonemetadata::i18n::phonenumbers::PhoneMetadata>;
        fn exec(self) -> cpp_utils::CppBox<::phonemetadata::i18n::phonenumbers::PhoneMetadata> {
          let from = self.0;
          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_new_from(from as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
      impl PhoneMetadataNewStaticArgs for cpp_utils::AsBox {
        type ReturnType = cpp_utils::CppBox<::phonemetadata::i18n::phonenumbers::PhoneMetadata>;
        fn exec(self) -> cpp_utils::CppBox<::phonemetadata::i18n::phonenumbers::PhoneMetadata> {

          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_new_no_args() };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
      /// This trait represents a set of arguments accepted by [PhoneMetadata::number_format](../struct.PhoneMetadata.html#method.number_format) method.
      pub trait PhoneMetadataNumberFormatArgs<'l0> {
        type ReturnType;
        fn exec(self, original_self: &'l0 ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> Self::ReturnType;
      }
      impl<'l0> PhoneMetadataNumberFormatArgs<'l0> for libc::c_int {
        type ReturnType = &'l0 ::phonemetadata::i18n::phonenumbers::NumberFormat;
        fn exec(self,
                original_self: &'l0 ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
                -> &'l0 ::phonemetadata::i18n::phonenumbers::NumberFormat {
          let index = self;
          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_number_format_index(original_self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata, index) };
          unsafe { &*ffi_result }
        }
      }
      impl<'l0> PhoneMetadataNumberFormatArgs<'l0> for () {
        type ReturnType = &'l0 ::repeated_field::google::protobuf::RepeatedPtrFieldNumberFormatRef;
        fn exec(self,
                original_self: &'l0 ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
                -> &'l0 ::repeated_field::google::protobuf::RepeatedPtrFieldNumberFormatRef {

          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_number_format_no_args(original_self as *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) };
          unsafe { &*ffi_result }
        }
      }
      /// This trait represents a set of arguments accepted by [PhoneMetadata::set_id](../struct.PhoneMetadata.html#method.set_id) method.
      pub trait PhoneMetadataSetIdArgs<'l0> {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> ();
      }
      impl<'l0> PhoneMetadataSetIdArgs<'l0> for *const libc::c_char {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> () {
          let value = self;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_id_char(original_self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value) }
        }
      }
      impl<'l0> PhoneMetadataSetIdArgs<'l0> for (*const libc::c_char, libc::c_ulong) {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> () {
          let value = self.0;
          let size = self.1;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_id_char_unsigned_long(original_self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value, size) }
        }
      }
      impl<'l0> PhoneMetadataSetIdArgs<'l0> for &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {

  fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> () {
    let value = self;
    unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_id_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(original_self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
  }
}
      /// This trait represents a set of arguments accepted by [PhoneMetadata::set_international_prefix](../struct.PhoneMetadata.html#method.set_international_prefix) method.
      pub trait PhoneMetadataSetInternationalPrefixArgs<'l0> {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> ();
      }
      impl<'l0> PhoneMetadataSetInternationalPrefixArgs<'l0> for *const libc::c_char {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> () {
          let value = self;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_international_prefix_char(original_self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value) }
        }
      }
      impl<'l0> PhoneMetadataSetInternationalPrefixArgs<'l0> for (*const libc::c_char, libc::c_ulong) {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> () {
          let value = self.0;
          let size = self.1;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_international_prefix_char_unsigned_long(original_self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value, size) }
        }
      }
      impl<'l0> PhoneMetadataSetInternationalPrefixArgs<'l0> for &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {

  fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> () {
    let value = self;
    unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_international_prefix_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(original_self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
  }
}
      /// This trait represents a set of arguments accepted by [PhoneMetadata::set_leading_digits](../struct.PhoneMetadata.html#method.set_leading_digits) method.
      pub trait PhoneMetadataSetLeadingDigitsArgs<'l0> {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> ();
      }
      impl<'l0> PhoneMetadataSetLeadingDigitsArgs<'l0> for *const libc::c_char {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> () {
          let value = self;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_leading_digits_char(original_self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value) }
        }
      }
      impl<'l0> PhoneMetadataSetLeadingDigitsArgs<'l0> for (*const libc::c_char, libc::c_ulong) {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> () {
          let value = self.0;
          let size = self.1;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_leading_digits_char_unsigned_long(original_self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value, size) }
        }
      }
      impl<'l0> PhoneMetadataSetLeadingDigitsArgs<'l0> for &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {

  fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> () {
    let value = self;
    unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_leading_digits_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(original_self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
  }
}
      /// This trait represents a set of arguments accepted by [PhoneMetadata::set_national_prefix](../struct.PhoneMetadata.html#method.set_national_prefix) method.
      pub trait PhoneMetadataSetNationalPrefixArgs<'l0> {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> ();
      }
      impl<'l0> PhoneMetadataSetNationalPrefixArgs<'l0> for *const libc::c_char {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> () {
          let value = self;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_national_prefix_char(original_self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value) }
        }
      }
      impl<'l0> PhoneMetadataSetNationalPrefixArgs<'l0> for (*const libc::c_char, libc::c_ulong) {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> () {
          let value = self.0;
          let size = self.1;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_national_prefix_char_unsigned_long(original_self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value, size) }
        }
      }
      impl<'l0> PhoneMetadataSetNationalPrefixArgs<'l0> for &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {

  fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> () {
    let value = self;
    unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_national_prefix_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(original_self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
  }
}
      /// This trait represents a set of arguments accepted by [PhoneMetadata::set_national_prefix_for_parsing](../struct.PhoneMetadata.html#method.set_national_prefix_for_parsing) method.
      pub trait PhoneMetadataSetNationalPrefixForParsingArgs<'l0> {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> ();
      }
      impl<'l0> PhoneMetadataSetNationalPrefixForParsingArgs<'l0> for *const libc::c_char {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> () {
          let value = self;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_national_prefix_for_parsing_char(original_self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value) }
        }
      }
      impl<'l0> PhoneMetadataSetNationalPrefixForParsingArgs<'l0> for (*const libc::c_char, libc::c_ulong) {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> () {
          let value = self.0;
          let size = self.1;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_national_prefix_for_parsing_char_unsigned_long(original_self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value, size) }
        }
      }
      impl<'l0> PhoneMetadataSetNationalPrefixForParsingArgs<'l0> for &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {

  fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> () {
    let value = self;
    unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_national_prefix_for_parsing_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(original_self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
  }
}
      /// This trait represents a set of arguments accepted by [PhoneMetadata::set_national_prefix_transform_rule](../struct.PhoneMetadata.html#method.set_national_prefix_transform_rule) method.
      pub trait PhoneMetadataSetNationalPrefixTransformRuleArgs<'l0> {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> ();
      }
      impl<'l0> PhoneMetadataSetNationalPrefixTransformRuleArgs<'l0> for *const libc::c_char {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> () {
          let value = self;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_national_prefix_transform_rule_char(original_self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value) }
        }
      }
      impl<'l0> PhoneMetadataSetNationalPrefixTransformRuleArgs<'l0> for (*const libc::c_char, libc::c_ulong) {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> () {
          let value = self.0;
          let size = self.1;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_national_prefix_transform_rule_char_unsigned_long(original_self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value, size) }
        }
      }
      impl<'l0> PhoneMetadataSetNationalPrefixTransformRuleArgs<'l0> for &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {

  fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> () {
    let value = self;
    unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_national_prefix_transform_rule_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(original_self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
  }
}
      /// This trait represents a set of arguments accepted by [PhoneMetadata::set_preferred_extn_prefix](../struct.PhoneMetadata.html#method.set_preferred_extn_prefix) method.
      pub trait PhoneMetadataSetPreferredExtnPrefixArgs<'l0> {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> ();
      }
      impl<'l0> PhoneMetadataSetPreferredExtnPrefixArgs<'l0> for *const libc::c_char {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> () {
          let value = self;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_preferred_extn_prefix_char(original_self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value) }
        }
      }
      impl<'l0> PhoneMetadataSetPreferredExtnPrefixArgs<'l0> for (*const libc::c_char, libc::c_ulong) {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> () {
          let value = self.0;
          let size = self.1;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_preferred_extn_prefix_char_unsigned_long(original_self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value, size) }
        }
      }
      impl<'l0> PhoneMetadataSetPreferredExtnPrefixArgs<'l0> for &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {

  fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> () {
    let value = self;
    unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_preferred_extn_prefix_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(original_self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
  }
}
      /// This trait represents a set of arguments accepted by [PhoneMetadata::set_preferred_international_prefix](../struct.PhoneMetadata.html#method.set_preferred_international_prefix) method.
      pub trait PhoneMetadataSetPreferredInternationalPrefixArgs<'l0> {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> ();
      }
      impl<'l0> PhoneMetadataSetPreferredInternationalPrefixArgs<'l0> for *const libc::c_char {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> () {
          let value = self;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_preferred_international_prefix_char(original_self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value) }
        }
      }
      impl<'l0> PhoneMetadataSetPreferredInternationalPrefixArgs<'l0> for (*const libc::c_char, libc::c_ulong) {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> () {
          let value = self.0;
          let size = self.1;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_preferred_international_prefix_char_unsigned_long(original_self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value, size) }
        }
      }
      impl<'l0> PhoneMetadataSetPreferredInternationalPrefixArgs<'l0> for &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {

  fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> () {
    let value = self;
    unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_preferred_international_prefix_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(original_self as *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
  }
}
      /// This trait represents a set of arguments accepted by [PhoneNumberDesc::get_type_name](../struct.PhoneNumberDesc.html#method.get_type_name) method.
      pub trait PhoneNumberDescGetTypeNameArgs<'l0> {
        type ReturnType;
        fn exec(self, original_self: &'l0 ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) -> Self::ReturnType;
      }
      impl<'l0> PhoneNumberDescGetTypeNameArgs<'l0> for cpp_utils::AsBox {
        type ReturnType = cpp_utils::CppBox<::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef>;
        fn exec
          (self,
           original_self: &'l0 ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc)
           -> cpp_utils::CppBox<::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef> {

          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_GetTypeName_as_ptr(original_self as *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
      impl<'l0> PhoneNumberDescGetTypeNameArgs<'l0> for cpp_utils::AsStruct {
        type ReturnType = ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
        fn exec(self,
                original_self: &'l0 ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc)
                -> ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {

          {
            let mut object: ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef =
              unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_GetTypeName_to_output(original_self as *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, &mut object) }
            object
          }
        }
      }
      /// This trait represents a set of arguments accepted by [PhoneNumberDesc::new](../struct.PhoneNumberDesc.html#method.new) method.
      pub trait PhoneNumberDescNewArgs<'l0> {
        fn exec(self,
                original_self: &'l0 ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc)
                -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
      }
      impl<'l0> PhoneNumberDescNewArgs<'l0> for *mut ::arena::google::protobuf::Arena {
        fn exec(self,
                original_self: &'l0 ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc)
                -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
          let arena = self;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_New_arena(original_self as *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, arena) }
        }
      }
      impl<'l0> PhoneNumberDescNewArgs<'l0> for () {
        fn exec(self,
                original_self: &'l0 ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc)
                -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {

          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_New_no_args(original_self as *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) }
        }
      }
      /// This trait represents a set of arguments accepted by [PhoneNumberDesc::new_static](../struct.PhoneNumberDesc.html#method.new_static) method.
      pub trait PhoneNumberDescNewStaticArgs {
        type ReturnType;
        fn exec(self) -> Self::ReturnType;
      }
      impl<'a> PhoneNumberDescNewStaticArgs
        for (&'a ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, cpp_utils::AsStruct) {
        type ReturnType = ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
        fn exec(self) -> ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {
          let from = self.0;
          {
            let mut object: ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc =
              unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_constructor_from(from as *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, &mut object) }
            object
          }
        }
      }
      impl PhoneNumberDescNewStaticArgs for cpp_utils::AsStruct {
        type ReturnType = ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
        fn exec(self) -> ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc {

          {
            let mut object: ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc =
              unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_constructor_no_args(&mut object) }
            object
          }
        }
      }
      impl<'a> PhoneNumberDescNewStaticArgs for (&'a ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, cpp_utils::AsBox) {
        type ReturnType = cpp_utils::CppBox<::phonemetadata::i18n::phonenumbers::PhoneNumberDesc>;
        fn exec(self) -> cpp_utils::CppBox<::phonemetadata::i18n::phonenumbers::PhoneNumberDesc> {
          let from = self.0;
          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_new_from(from as *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
      impl PhoneNumberDescNewStaticArgs for cpp_utils::AsBox {
        type ReturnType = cpp_utils::CppBox<::phonemetadata::i18n::phonenumbers::PhoneNumberDesc>;
        fn exec(self) -> cpp_utils::CppBox<::phonemetadata::i18n::phonenumbers::PhoneNumberDesc> {

          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_new_no_args() };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
      /// This trait represents a set of arguments accepted by [PhoneNumberDesc::set_example_number](../struct.PhoneNumberDesc.html#method.set_example_number) method.
      pub trait PhoneNumberDescSetExampleNumberArgs<'l0> {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) -> ();
      }
      impl<'l0> PhoneNumberDescSetExampleNumberArgs<'l0> for *const libc::c_char {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) -> () {
          let value = self;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_set_example_number_char(original_self as *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, value) }
        }
      }
      impl<'l0> PhoneNumberDescSetExampleNumberArgs<'l0> for (*const libc::c_char, libc::c_ulong) {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) -> () {
          let value = self.0;
          let size = self.1;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_set_example_number_char_unsigned_long(original_self as *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, value, size) }
        }
      }
      impl<'l0> PhoneNumberDescSetExampleNumberArgs<'l0> for &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {

  fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) -> () {
    let value = self;
    unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_set_example_number_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(original_self as *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, value as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
  }
}
      /// This trait represents a set of arguments accepted by [PhoneNumberDesc::set_national_number_pattern](../struct.PhoneNumberDesc.html#method.set_national_number_pattern) method.
      pub trait PhoneNumberDescSetNationalNumberPatternArgs<'l0> {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) -> ();
      }
      impl<'l0> PhoneNumberDescSetNationalNumberPatternArgs<'l0> for *const libc::c_char {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) -> () {
          let value = self;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_set_national_number_pattern_char(original_self as *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, value) }
        }
      }
      impl<'l0> PhoneNumberDescSetNationalNumberPatternArgs<'l0> for (*const libc::c_char, libc::c_ulong) {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) -> () {
          let value = self.0;
          let size = self.1;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_set_national_number_pattern_char_unsigned_long(original_self as *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, value, size) }
        }
      }
      impl<'l0> PhoneNumberDescSetNationalNumberPatternArgs<'l0> for &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {

  fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) -> () {
    let value = self;
    unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_set_national_number_pattern_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(original_self as *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, value as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
  }
}
      /// This trait represents a set of arguments accepted by [PhoneNumberDesc::set_possible_number_pattern](../struct.PhoneNumberDesc.html#method.set_possible_number_pattern) method.
      pub trait PhoneNumberDescSetPossibleNumberPatternArgs<'l0> {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) -> ();
      }
      impl<'l0> PhoneNumberDescSetPossibleNumberPatternArgs<'l0> for *const libc::c_char {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) -> () {
          let value = self;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_set_possible_number_pattern_char(original_self as *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, value) }
        }
      }
      impl<'l0> PhoneNumberDescSetPossibleNumberPatternArgs<'l0> for (*const libc::c_char, libc::c_ulong) {
        fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) -> () {
          let value = self.0;
          let size = self.1;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_set_possible_number_pattern_char_unsigned_long(original_self as *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, value, size) }
        }
      }
      impl<'l0> PhoneNumberDescSetPossibleNumberPatternArgs<'l0> for &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {

  fn exec(self, original_self: &'l0 mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) -> () {
    let value = self;
    unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_set_possible_number_pattern_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(original_self as *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, value as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
  }
}
    }

  }

}
