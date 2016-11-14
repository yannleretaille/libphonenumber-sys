#[allow(unused_imports)]
use {libc, cpp_utils, std};

pub mod i18n {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  pub mod phonenumbers {
    #[allow(unused_imports)]
    use {libc, cpp_utils, std};

    /// C++ type: <span style='color: green;'>```i18n::phonenumbers::PhoneNumber```</span>.
    #[repr(C)]
    pub struct PhoneNumber {
      _buffer: [u8; 80],
    }

    impl ::NewUninitialized for PhoneNumber {
      unsafe fn new_uninitialized() -> PhoneNumber {
        PhoneNumber { _buffer: std::mem::uninitialized() }
      }
    }

    impl PhoneNumber {
      /// C++ method: <span style='color: green;'>```virtual int i18n::phonenumbers::PhoneNumber::ByteSize() const```</span>
      ///
      ///
      pub fn byte_size(&self) -> libc::c_int {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_ByteSize(self as *const ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```virtual void i18n::phonenumbers::PhoneNumber::CheckTypeAndMergeFrom(const google::protobuf::MessageLite& from)```</span>
      ///
      ///
      pub fn check_type_and_merge_from(&mut self, from: &::message_lite::google::protobuf::MessageLite) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_CheckTypeAndMergeFrom(self as *mut ::phonenumber::i18n::phonenumbers::PhoneNumber, from as *const ::message_lite::google::protobuf::MessageLite) }
      }

      /// C++ method: <span style='color: green;'>```virtual void i18n::phonenumbers::PhoneNumber::Clear()```</span>
      ///
      ///
      pub fn clear(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_Clear(self as *mut ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumber::clear_country_code()```</span>
      ///
      ///
      pub fn clear_country_code(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_clear_country_code(self as *mut ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumber::clear_country_code_source()```</span>
      ///
      ///
      pub fn clear_country_code_source(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_clear_country_code_source(self as *mut ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumber::clear_extension()```</span>
      ///
      ///
      pub fn clear_extension(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_clear_extension(self as *mut ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumber::clear_italian_leading_zero()```</span>
      ///
      ///
      pub fn clear_italian_leading_zero(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_clear_italian_leading_zero(self as *mut ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumber::clear_national_number()```</span>
      ///
      ///
      pub fn clear_national_number(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_clear_national_number(self as *mut ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumber::clear_number_of_leading_zeros()```</span>
      ///
      ///
      pub fn clear_number_of_leading_zeros(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_clear_number_of_leading_zeros(self as *mut ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumber::clear_preferred_domestic_carrier_code()```</span>
      ///
      ///
      pub fn clear_preferred_domestic_carrier_code(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_clear_preferred_domestic_carrier_code(self as *mut ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumber::clear_raw_input()```</span>
      ///
      ///
      pub fn clear_raw_input(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_clear_raw_input(self as *mut ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumber::CopyFrom(const i18n::phonenumbers::PhoneNumber& from)```</span>
      ///
      ///
      pub fn copy_from(&mut self, from: &::phonenumber::i18n::phonenumbers::PhoneNumber) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_CopyFrom(self as *mut ::phonenumber::i18n::phonenumbers::PhoneNumber, from as *const ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```int i18n::phonenumbers::PhoneNumber::country_code() const```</span>
      ///
      ///
      pub fn country_code(&self) -> libc::c_int {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_country_code(self as *const ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumber_CountryCodeSource i18n::phonenumbers::PhoneNumber::country_code_source() const```</span>
      ///
      ///
      pub fn country_code_source(&self) -> ::phonenumber::i18n::phonenumbers::PhoneNumberCountryCodeSource {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_country_code_source(self as *const ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```static bool i18n::phonenumbers::PhoneNumber::CountryCodeSource_IsValid(int value)```</span>
      ///
      ///
      pub fn country_code_source_is_valid(value: libc::c_int) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_CountryCodeSource_IsValid(value) }
      }

      /// C++ method: <span style='color: green;'>```static const i18n::phonenumbers::PhoneNumber& i18n::phonenumbers::PhoneNumber::default_instance()```</span>
      ///
      ///
      pub fn default_instance() -> &'static ::phonenumber::i18n::phonenumbers::PhoneNumber {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_default_instance() };
        unsafe { &*ffi_result }
      }

      /// C++ method: <span style='color: green;'>```const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& i18n::phonenumbers::PhoneNumber::extension() const```</span>
      ///
      ///
      pub fn extension<'l0>(&'l0 self)
                            -> &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_extension(self as *const ::phonenumber::i18n::phonenumbers::PhoneNumber) };
        unsafe { &*ffi_result }
      }

      /// C++ method: <span style='color: green;'>```virtual int i18n::phonenumbers::PhoneNumber::GetCachedSize() const```</span>
      ///
      ///
      pub fn get_cached_size(&self) -> libc::c_int {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_GetCachedSize(self as *const ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumber::GetTypeName```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      /// Rust arguments: <br>1) ```fn get_type_name(&self, cpp_utils::AsBox) -> cpp_utils::CppBox<::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef>```<br>2) ```fn get_type_name(&self, cpp_utils::AsStruct) -> ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef```<br>
      /// C++ method: <span style='color: green;'>```virtual std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>> i18n::phonenumbers::PhoneNumber::GetTypeName() const```</span>
      ///
      ///
      pub fn get_type_name<'l0, Args>(&'l0 self, args: Args) -> Args::ReturnType
        where Args: overloading::PhoneNumberGetTypeNameArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneNumber::has_country_code() const```</span>
      ///
      ///
      pub fn has_country_code(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_has_country_code(self as *const ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneNumber::has_country_code_source() const```</span>
      ///
      ///
      pub fn has_country_code_source(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_has_country_code_source(self as *const ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneNumber::has_extension() const```</span>
      ///
      ///
      pub fn has_extension(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_has_extension(self as *const ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneNumber::has_italian_leading_zero() const```</span>
      ///
      ///
      pub fn has_italian_leading_zero(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_has_italian_leading_zero(self as *const ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneNumber::has_national_number() const```</span>
      ///
      ///
      pub fn has_national_number(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_has_national_number(self as *const ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneNumber::has_number_of_leading_zeros() const```</span>
      ///
      ///
      pub fn has_number_of_leading_zeros(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_has_number_of_leading_zeros(self as *const ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneNumber::has_preferred_domestic_carrier_code() const```</span>
      ///
      ///
      pub fn has_preferred_domestic_carrier_code(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_has_preferred_domestic_carrier_code(self as *const ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneNumber::has_raw_input() const```</span>
      ///
      ///
      pub fn has_raw_input(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_has_raw_input(self as *const ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```virtual bool i18n::phonenumbers::PhoneNumber::IsInitialized() const```</span>
      ///
      ///
      pub fn is_initialized(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_IsInitialized(self as *const ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }
	//added
	  ///new from string
	  pub fn from_string(number: &str) -> ::phonenumber::i18n::phonenumbers::PhoneNumber {
	    let mut buffer: [u8; 80] = [0;80];
		let bytes = number.as_bytes();
		buffer[..bytes.len()].clone_from_slice(bytes);
		PhoneNumber { _buffer: buffer }
	  }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneNumber::italian_leading_zero() const```</span>
      ///
      ///
      pub fn italian_leading_zero(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_italian_leading_zero(self as *const ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumber::MergeFrom(const i18n::phonenumbers::PhoneNumber& from)```</span>
      ///
      ///
      pub fn merge_from(&mut self, from: &::phonenumber::i18n::phonenumbers::PhoneNumber) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_MergeFrom(self as *mut ::phonenumber::i18n::phonenumbers::PhoneNumber, from as *const ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* i18n::phonenumbers::PhoneNumber::mutable_extension()```</span>
      ///
      ///
      pub fn mutable_extension
        (&mut self)
         -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_mutable_extension(self as *mut ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* i18n::phonenumbers::PhoneNumber::mutable_preferred_domestic_carrier_code()```</span>
      ///
      ///
      pub fn mutable_preferred_domestic_carrier_code
        (&mut self)
         -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_mutable_preferred_domestic_carrier_code(self as *mut ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* i18n::phonenumbers::PhoneNumber::mutable_raw_input()```</span>
      ///
      ///
      pub fn mutable_raw_input
        (&mut self)
         -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_mutable_raw_input(self as *mut ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* i18n::phonenumbers::PhoneNumber::mutable_unknown_fields()```</span>
      ///
      ///
      pub fn mutable_unknown_fields
        (&mut self)
         -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_mutable_unknown_fields(self as *mut ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```unsigned long i18n::phonenumbers::PhoneNumber::national_number() const```</span>
      ///
      ///
      pub fn national_number(&self) -> libc::c_ulong {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_national_number(self as *const ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumber::New```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn new(&self, ()) -> *mut ::phonenumber::i18n::phonenumbers::PhoneNumber```<br>
      /// C++ method: <span style='color: green;'>```virtual i18n::phonenumbers::PhoneNumber* i18n::phonenumbers::PhoneNumber::New() const```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn new(&self, *mut ::arena::google::protobuf::Arena) -> *mut ::phonenumber::i18n::phonenumbers::PhoneNumber```<br>
      /// C++ method: <span style='color: green;'>```virtual i18n::phonenumbers::PhoneNumber* i18n::phonenumbers::PhoneNumber::New(google::protobuf::Arena* arena) const```</span>
      ///
      ///
      /*pub fn new<'l0, Args>(&'l0 self, args: Args) -> *mut ::phonenumber::i18n::phonenumbers::PhoneNumber
        where Args: overloading::PhoneNumberNewArgs<'l0>
      {
        args.exec(self)
      }*/
	  pub fn new<'a>() -> &'a mut PhoneNumber {
	    let mut ffi_result =  unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_new_no_args()};
		unsafe { &mut *ffi_result}
	  }
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumber::PhoneNumber```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: <br>1) ```fn new_static(cpp_utils::AsStruct) -> ::phonenumber::i18n::phonenumbers::PhoneNumber```<br>2) ```fn new_static(cpp_utils::AsBox) -> cpp_utils::CppBox<::phonenumber::i18n::phonenumbers::PhoneNumber>```<br>
      /// C++ method: <span style='color: green;'>```[constructor] void i18n::phonenumbers::PhoneNumber::PhoneNumber()```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: <br>1) ```fn new_static((&::phonenumber::i18n::phonenumbers::PhoneNumber, cpp_utils::AsStruct)) -> ::phonenumber::i18n::phonenumbers::PhoneNumber```<br>2) ```fn new_static((&::phonenumber::i18n::phonenumbers::PhoneNumber, cpp_utils::AsBox)) -> cpp_utils::CppBox<::phonenumber::i18n::phonenumbers::PhoneNumber>```<br>
      /// C++ method: <span style='color: green;'>```[constructor] void i18n::phonenumbers::PhoneNumber::PhoneNumber(const i18n::phonenumbers::PhoneNumber& from)```</span>
      ///
      ///
      pub fn new_static<Args>(args: Args) -> Args::ReturnType
        where Args: overloading::PhoneNumberNewStaticArgs
      {
        args.exec()
      }
      /// C++ method: <span style='color: green;'>```int i18n::phonenumbers::PhoneNumber::number_of_leading_zeros() const```</span>
      ///
      ///
      pub fn number_of_leading_zeros(&self) -> libc::c_int {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_number_of_leading_zeros(self as *const ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumber& i18n::phonenumbers::PhoneNumber::operator=(const i18n::phonenumbers::PhoneNumber& from)```</span>
      ///
      ///
      pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                                 from: &'l1 ::phonenumber::i18n::phonenumbers::PhoneNumber)
                                 -> &'l0 mut ::phonenumber::i18n::phonenumbers::PhoneNumber {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_operator_assign(self as *mut ::phonenumber::i18n::phonenumbers::PhoneNumber, from as *const ::phonenumber::i18n::phonenumbers::PhoneNumber) };
        unsafe { &mut *ffi_result }
      }

      /// C++ method: <span style='color: green;'>```const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& i18n::phonenumbers::PhoneNumber::preferred_domestic_carrier_code() const```</span>
      ///
      ///
      pub fn preferred_domestic_carrier_code<'l0>
        (&'l0 self)
         -> &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_preferred_domestic_carrier_code(self as *const ::phonenumber::i18n::phonenumbers::PhoneNumber) };
        unsafe { &*ffi_result }
      }

      /// C++ method: <span style='color: green;'>```const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& i18n::phonenumbers::PhoneNumber::raw_input() const```</span>
      ///
      ///
      pub fn raw_input<'l0>(&'l0 self)
                            -> &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_raw_input(self as *const ::phonenumber::i18n::phonenumbers::PhoneNumber) };
        unsafe { &*ffi_result }
      }

      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* i18n::phonenumbers::PhoneNumber::release_extension()```</span>
      ///
      ///
      pub fn release_extension
        (&mut self)
         -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_release_extension(self as *mut ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* i18n::phonenumbers::PhoneNumber::release_preferred_domestic_carrier_code()```</span>
      ///
      ///
      pub fn release_preferred_domestic_carrier_code
        (&mut self)
         -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_release_preferred_domestic_carrier_code(self as *mut ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* i18n::phonenumbers::PhoneNumber::release_raw_input()```</span>
      ///
      ///
      pub fn release_raw_input
        (&mut self)
         -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_release_raw_input(self as *mut ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumber::set_allocated_extension(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* extension)```</span>
      ///
      ///
pub fn set_allocated_extension(&mut self, extension: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_allocated_extension(self as *mut ::phonenumber::i18n::phonenumbers::PhoneNumber, extension) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumber::set_allocated_preferred_domestic_carrier_code(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* preferred_domestic_carrier_code)```</span>
      ///
      ///
pub fn set_allocated_preferred_domestic_carrier_code(&mut self, preferred_domestic_carrier_code: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_allocated_preferred_domestic_carrier_code(self as *mut ::phonenumber::i18n::phonenumbers::PhoneNumber, preferred_domestic_carrier_code) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumber::set_allocated_raw_input(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* raw_input)```</span>
      ///
      ///
pub fn set_allocated_raw_input(&mut self, raw_input: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_allocated_raw_input(self as *mut ::phonenumber::i18n::phonenumbers::PhoneNumber, raw_input) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumber::set_country_code(int value)```</span>
      ///
      ///
      pub fn set_country_code(&mut self, value: libc::c_int) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_country_code(self as *mut ::phonenumber::i18n::phonenumbers::PhoneNumber, value) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumber::set_country_code_source(i18n::phonenumbers::PhoneNumber_CountryCodeSource value)```</span>
      ///
      ///
      pub fn set_country_code_source(&mut self,
                                     value: ::phonenumber::i18n::phonenumbers::PhoneNumberCountryCodeSource) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_country_code_source(self as *mut ::phonenumber::i18n::phonenumbers::PhoneNumber, value) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumber::set_extension```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn set_extension(&mut self, *const libc::c_char) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumber::set_extension(const char* value)```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn set_extension(&mut self, (*const libc::c_char, libc::c_ulong)) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumber::set_extension(const char* value, unsigned long size)```</span>
      ///
      ///
      ///
      /// ## Variant 3
      ///
      /// Rust arguments: ```fn set_extension(&mut self, &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumber::set_extension(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& value)```</span>
      ///
      ///
      pub fn set_extension<'l0, Args>(&'l0 mut self, args: Args) -> ()
        where Args: overloading::PhoneNumberSetExtensionArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumber::set_italian_leading_zero(bool value)```</span>
      ///
      ///
      pub fn set_italian_leading_zero(&mut self, value: bool) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_italian_leading_zero(self as *mut ::phonenumber::i18n::phonenumbers::PhoneNumber, value) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumber::set_national_number(unsigned long value)```</span>
      ///
      ///
      pub fn set_national_number(&mut self, value: libc::c_ulong) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_national_number(self as *mut ::phonenumber::i18n::phonenumbers::PhoneNumber, value) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumber::set_number_of_leading_zeros(int value)```</span>
      ///
      ///
      pub fn set_number_of_leading_zeros(&mut self, value: libc::c_int) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_number_of_leading_zeros(self as *mut ::phonenumber::i18n::phonenumbers::PhoneNumber, value) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumber::set_preferred_domestic_carrier_code```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn set_preferred_domestic_carrier_code(&mut self, *const libc::c_char) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumber::set_preferred_domestic_carrier_code(const char* value)```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn set_preferred_domestic_carrier_code(&mut self, (*const libc::c_char, libc::c_ulong)) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumber::set_preferred_domestic_carrier_code(const char* value, unsigned long size)```</span>
      ///
      ///
      ///
      /// ## Variant 3
      ///
      /// Rust arguments: ```fn set_preferred_domestic_carrier_code(&mut self, &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumber::set_preferred_domestic_carrier_code(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& value)```</span>
      ///
      ///
      pub fn set_preferred_domestic_carrier_code<'l0, Args>(&'l0 mut self, args: Args) -> ()
        where Args: overloading::PhoneNumberSetPreferredDomesticCarrierCodeArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumber::set_raw_input```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn set_raw_input(&mut self, *const libc::c_char) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumber::set_raw_input(const char* value)```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn set_raw_input(&mut self, (*const libc::c_char, libc::c_ulong)) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumber::set_raw_input(const char* value, unsigned long size)```</span>
      ///
      ///
      ///
      /// ## Variant 3
      ///
      /// Rust arguments: ```fn set_raw_input(&mut self, &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> ()```<br>
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumber::set_raw_input(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& value)```</span>
      ///
      ///
      pub fn set_raw_input<'l0, Args>(&'l0 mut self, args: Args) -> ()
        where Args: overloading::PhoneNumberSetRawInputArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumber::Swap(i18n::phonenumbers::PhoneNumber* other)```</span>
      ///
      ///
      pub fn swap(&mut self, other: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_Swap(self as *mut ::phonenumber::i18n::phonenumbers::PhoneNumber, other) }
      }

      /// C++ method: <span style='color: green;'>```const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& i18n::phonenumbers::PhoneNumber::unknown_fields() const```</span>
      ///
      ///
      pub fn unknown_fields<'l0>
        (&'l0 self)
         -> &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_unknown_fields(self as *const ::phonenumber::i18n::phonenumbers::PhoneNumber) };
        unsafe { &*ffi_result }
      }
    }

    impl Drop for PhoneNumber {
      /// C++ method: <span style='color: green;'>```virtual [destructor] void i18n::phonenumbers::PhoneNumber::~PhoneNumber()```</span>
      ///
      ///
      fn drop(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_destructor(self as *mut ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }
    }

    impl cpp_utils::CppDeletable for PhoneNumber {
      fn deleter() -> cpp_utils::Deleter<Self> {
        ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_delete
      }
    }

    /// C++ type: <span style='color: green;'>```i18n::phonenumbers::PhoneNumber_CountryCodeSource```</span>
    #[derive(Debug, PartialEq, Eq, Clone)]
    #[repr(C)]
    pub enum PhoneNumberCountryCodeSource {
      /// C++ variant: PhoneNumber_CountryCodeSource_FROM_NUMBER_WITH_PLUS_SIGN
      NUMBERWITHPLUSSIGN = 1,
      /// C++ variant: PhoneNumber_CountryCodeSource_FROM_NUMBER_WITH_IDD
      NUMBERWITHIDD = 5,
      /// C++ variant: PhoneNumber_CountryCodeSource_FROM_NUMBER_WITHOUT_PLUS_SIGN
      NUMBERWITHOUTPLUSSIGN = 10,
      /// C++ variant: PhoneNumber_CountryCodeSource_FROM_DEFAULT_COUNTRY
      DEFAULTCOUNTRY = 20,
    }

    /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneNumber_CountryCodeSource_IsValid(int value)```</span>
    ///
    ///
    pub fn phone_number_country_code_source_is_valid(value: libc::c_int) -> bool {
      unsafe {
        ::ffi::libphonenumber_sys_c_phonenumber_G_i18n_phonenumbers_PhoneNumber_CountryCodeSource_IsValid(value)
      }
    }

    /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::protobuf_AddDesc_phonenumber_2eproto()```</span>
    ///
    ///
    pub fn protobuf_add_desc_phonenumber_2eproto() {
      unsafe { ::ffi::libphonenumber_sys_c_phonenumber_G_i18n_phonenumbers_protobuf_AddDesc_phonenumber_2eproto() }
    }

    /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::protobuf_ShutdownFile_phonenumber_2eproto()```</span>
    ///
    ///
    pub fn protobuf_shutdown_file_phonenumber_2eproto() {
      unsafe { ::ffi::libphonenumber_sys_c_phonenumber_G_i18n_phonenumbers_protobuf_ShutdownFile_phonenumber_2eproto() }
    }

    pub mod overloading {
      #[allow(unused_imports)]
      use {libc, cpp_utils, std};

      /// This trait represents a set of arguments accepted by [PhoneNumber::get_type_name](../struct.PhoneNumber.html#method.get_type_name) method.
      pub trait PhoneNumberGetTypeNameArgs<'l0> {
        type ReturnType;
        fn exec(self, original_self: &'l0 ::phonenumber::i18n::phonenumbers::PhoneNumber) -> Self::ReturnType;
      }
      impl<'l0> PhoneNumberGetTypeNameArgs<'l0> for cpp_utils::AsBox {
        type ReturnType = cpp_utils::CppBox<::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef>;
        fn exec
          (self,
           original_self: &'l0 ::phonenumber::i18n::phonenumbers::PhoneNumber)
           -> cpp_utils::CppBox<::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef> {

          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_GetTypeName_as_ptr(original_self as *const ::phonenumber::i18n::phonenumbers::PhoneNumber) };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
      impl<'l0> PhoneNumberGetTypeNameArgs<'l0> for cpp_utils::AsStruct {
        type ReturnType = ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
        fn exec(self,
                original_self: &'l0 ::phonenumber::i18n::phonenumbers::PhoneNumber)
                -> ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {

          {
            let mut object: ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef =
              unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_GetTypeName_to_output(original_self as *const ::phonenumber::i18n::phonenumbers::PhoneNumber, &mut object) }
            object
          }
        }
      }
      /// This trait represents a set of arguments accepted by [PhoneNumber::new](../struct.PhoneNumber.html#method.new) method.
      pub trait PhoneNumberNewArgs<'l0> {
        fn exec(self,
                original_self: &'l0 ::phonenumber::i18n::phonenumbers::PhoneNumber)
                -> *mut ::phonenumber::i18n::phonenumbers::PhoneNumber;
      }
      impl<'l0> PhoneNumberNewArgs<'l0> for *mut ::arena::google::protobuf::Arena {
        fn exec(self,
                original_self: &'l0 ::phonenumber::i18n::phonenumbers::PhoneNumber)
                -> *mut ::phonenumber::i18n::phonenumbers::PhoneNumber {
          let arena = self;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_New_arena(original_self as *const ::phonenumber::i18n::phonenumbers::PhoneNumber, arena) }
        }
      }
      impl<'l0> PhoneNumberNewArgs<'l0> for () {
        fn exec(self,
                original_self: &'l0 ::phonenumber::i18n::phonenumbers::PhoneNumber)
                -> *mut ::phonenumber::i18n::phonenumbers::PhoneNumber {

          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_New_no_args(original_self as *const ::phonenumber::i18n::phonenumbers::PhoneNumber) }
        }
      }
      /// This trait represents a set of arguments accepted by [PhoneNumber::new_static](../struct.PhoneNumber.html#method.new_static) method.
      pub trait PhoneNumberNewStaticArgs {
        type ReturnType;
        fn exec(self) -> Self::ReturnType;
      }
      impl<'a> PhoneNumberNewStaticArgs for (&'a ::phonenumber::i18n::phonenumbers::PhoneNumber, cpp_utils::AsStruct) {
        type ReturnType = ::phonenumber::i18n::phonenumbers::PhoneNumber;
        fn exec(self) -> ::phonenumber::i18n::phonenumbers::PhoneNumber {
          let from = self.0;
          {
            let mut object: ::phonenumber::i18n::phonenumbers::PhoneNumber =
              unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_constructor_from(from as *const ::phonenumber::i18n::phonenumbers::PhoneNumber, &mut object) }
            object
          }
        }
      }
      impl PhoneNumberNewStaticArgs for cpp_utils::AsStruct {
        type ReturnType = ::phonenumber::i18n::phonenumbers::PhoneNumber;
        fn exec(self) -> ::phonenumber::i18n::phonenumbers::PhoneNumber {

          {
            let mut object: ::phonenumber::i18n::phonenumbers::PhoneNumber =
              unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_constructor_no_args(&mut object) }
            object
          }
        }
      }
      impl<'a> PhoneNumberNewStaticArgs for (&'a ::phonenumber::i18n::phonenumbers::PhoneNumber, cpp_utils::AsBox) {
        type ReturnType = cpp_utils::CppBox<::phonenumber::i18n::phonenumbers::PhoneNumber>;
        fn exec(self) -> cpp_utils::CppBox<::phonenumber::i18n::phonenumbers::PhoneNumber> {
          let from = self.0;
          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_new_from(from as *const ::phonenumber::i18n::phonenumbers::PhoneNumber) };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
      impl PhoneNumberNewStaticArgs for cpp_utils::AsBox {
        type ReturnType = cpp_utils::CppBox<::phonenumber::i18n::phonenumbers::PhoneNumber>;
        fn exec(self) -> cpp_utils::CppBox<::phonenumber::i18n::phonenumbers::PhoneNumber> {

          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_new_no_args() };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
      /// This trait represents a set of arguments accepted by [PhoneNumber::set_extension](../struct.PhoneNumber.html#method.set_extension) method.
      pub trait PhoneNumberSetExtensionArgs<'l0> {
        fn exec(self, original_self: &'l0 mut ::phonenumber::i18n::phonenumbers::PhoneNumber) -> ();
      }
      impl<'l0> PhoneNumberSetExtensionArgs<'l0> for *const libc::c_char {
        fn exec(self, original_self: &'l0 mut ::phonenumber::i18n::phonenumbers::PhoneNumber) -> () {
          let value = self;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_extension_char(original_self as *mut ::phonenumber::i18n::phonenumbers::PhoneNumber, value) }
        }
      }
      impl<'l0> PhoneNumberSetExtensionArgs<'l0> for (*const libc::c_char, libc::c_ulong) {
        fn exec(self, original_self: &'l0 mut ::phonenumber::i18n::phonenumbers::PhoneNumber) -> () {
          let value = self.0;
          let size = self.1;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_extension_char_unsigned_long(original_self as *mut ::phonenumber::i18n::phonenumbers::PhoneNumber, value, size) }
        }
      }
      impl<'l0> PhoneNumberSetExtensionArgs<'l0> for &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {

  fn exec(self, original_self: &'l0 mut ::phonenumber::i18n::phonenumbers::PhoneNumber) -> () {
    let value = self;
    unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_extension_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(original_self as *mut ::phonenumber::i18n::phonenumbers::PhoneNumber, value as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
  }
}
      /// This trait represents a set of arguments accepted by [PhoneNumber::set_preferred_domestic_carrier_code](../struct.PhoneNumber.html#method.set_preferred_domestic_carrier_code) method.
      pub trait PhoneNumberSetPreferredDomesticCarrierCodeArgs<'l0> {
        fn exec(self, original_self: &'l0 mut ::phonenumber::i18n::phonenumbers::PhoneNumber) -> ();
      }
      impl<'l0> PhoneNumberSetPreferredDomesticCarrierCodeArgs<'l0> for *const libc::c_char {
        fn exec(self, original_self: &'l0 mut ::phonenumber::i18n::phonenumbers::PhoneNumber) -> () {
          let value = self;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_preferred_domestic_carrier_code_char(original_self as *mut ::phonenumber::i18n::phonenumbers::PhoneNumber, value) }
        }
      }
      impl<'l0> PhoneNumberSetPreferredDomesticCarrierCodeArgs<'l0> for (*const libc::c_char, libc::c_ulong) {
        fn exec(self, original_self: &'l0 mut ::phonenumber::i18n::phonenumbers::PhoneNumber) -> () {
          let value = self.0;
          let size = self.1;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_preferred_domestic_carrier_code_char_unsigned_long(original_self as *mut ::phonenumber::i18n::phonenumbers::PhoneNumber, value, size) }
        }
      }
      impl<'l0> PhoneNumberSetPreferredDomesticCarrierCodeArgs<'l0> for &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {

  fn exec(self, original_self: &'l0 mut ::phonenumber::i18n::phonenumbers::PhoneNumber) -> () {
    let value = self;
    unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_preferred_domestic_carrier_code_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(original_self as *mut ::phonenumber::i18n::phonenumbers::PhoneNumber, value as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
  }
}
      /// This trait represents a set of arguments accepted by [PhoneNumber::set_raw_input](../struct.PhoneNumber.html#method.set_raw_input) method.
      pub trait PhoneNumberSetRawInputArgs<'l0> {
        fn exec(self, original_self: &'l0 mut ::phonenumber::i18n::phonenumbers::PhoneNumber) -> ();
      }
      impl<'l0> PhoneNumberSetRawInputArgs<'l0> for *const libc::c_char {
        fn exec(self, original_self: &'l0 mut ::phonenumber::i18n::phonenumbers::PhoneNumber) -> () {
          let value = self;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_raw_input_char(original_self as *mut ::phonenumber::i18n::phonenumbers::PhoneNumber, value) }
        }
      }
      impl<'l0> PhoneNumberSetRawInputArgs<'l0> for (*const libc::c_char, libc::c_ulong) {
        fn exec(self, original_self: &'l0 mut ::phonenumber::i18n::phonenumbers::PhoneNumber) -> () {
          let value = self.0;
          let size = self.1;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_raw_input_char_unsigned_long(original_self as *mut ::phonenumber::i18n::phonenumbers::PhoneNumber, value, size) }
        }
      }
      impl<'l0> PhoneNumberSetRawInputArgs<'l0> for &'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {

  fn exec(self, original_self: &'l0 mut ::phonenumber::i18n::phonenumbers::PhoneNumber) -> () {
    let value = self;
    unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_raw_input_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(original_self as *mut ::phonenumber::i18n::phonenumbers::PhoneNumber, value as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
  }
}
    }

  }

}
