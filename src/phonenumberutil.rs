#[allow(unused_imports)]
use {libc, cpp_utils, std};

pub mod i18n {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  pub mod phonenumbers {
    #[allow(unused_imports)]
    use {libc, cpp_utils, std};

    /// C++ type: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberUtil```</span>.
    #[repr(C)]
    pub struct PhoneNumberUtil {
      _buffer: [u8; 56],
    }

    impl ::NewUninitialized for PhoneNumberUtil {
      unsafe fn new_uninitialized() -> PhoneNumberUtil {
        PhoneNumberUtil { _buffer: std::mem::uninitialized() }
      }
    }

    impl PhoneNumberUtil {
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberUtil::ConvertAlphaCharactersInNumber(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* number) const```</span>
      ///
      ///
pub fn convert_alpha_characters_in_number(&self, number: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_ConvertAlphaCharactersInNumber(self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberUtil::Format(const i18n::phonenumbers::PhoneNumber& number, i18n::phonenumbers::PhoneNumberUtil::PhoneNumberFormat number_format, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* formatted_number) const```</span>
      ///
      ///
/*OLD pub fn format(&self, number: &::phonenumber::i18n::phonenumbers::PhoneNumber, number_format: ::phonenumberutil::i18n::phonenumbers::phone_number_util::PhoneNumberFormat, formatted_number: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_Format(self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number as *const ::phonenumber::i18n::phonenumbers::PhoneNumber, number_format, formatted_number) }
		}*/
pub fn format(&self, number: &::phonenumber::i18n::phonenumbers::PhoneNumber, number_format: ::phonenumberutil::i18n::phonenumbers::phone_number_util::PhoneNumberFormat) -> String {
        unsafe {
		    std::ffi::CStr::from_ptr(::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_Format_Str(self as *const PhoneNumberUtil, number as *const ::phonenumber::i18n::phonenumbers::PhoneNumber, number_format)).to_string_lossy().to_string()
		}
	}
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberUtil::FormatByPattern(const i18n::phonenumbers::PhoneNumber& number, i18n::phonenumbers::PhoneNumberUtil::PhoneNumberFormat number_format, const google::protobuf::RepeatedPtrField<i18n::phonenumbers::NumberFormat>& user_defined_formats, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* formatted_number) const```</span>
      ///
      ///
pub fn format_by_pattern(&self, number: &::phonenumber::i18n::phonenumbers::PhoneNumber, number_format: ::phonenumberutil::i18n::phonenumbers::phone_number_util::PhoneNumberFormat, user_defined_formats: &::repeated_field::google::protobuf::RepeatedPtrFieldNumberFormatRef, formatted_number: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_FormatByPattern(self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number as *const ::phonenumber::i18n::phonenumbers::PhoneNumber, number_format, user_defined_formats as *const ::repeated_field::google::protobuf::RepeatedPtrFieldNumberFormatRef, formatted_number) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberUtil::FormatInOriginalFormat(const i18n::phonenumbers::PhoneNumber& number, const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& region_calling_from, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* formatted_number) const```</span>
      ///
      ///
pub fn format_in_original_format(&self, number: &::phonenumber::i18n::phonenumbers::PhoneNumber, region_calling_from: &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, formatted_number: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_FormatInOriginalFormat(self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number as *const ::phonenumber::i18n::phonenumbers::PhoneNumber, region_calling_from as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, formatted_number) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberUtil::FormatNationalNumberWithCarrierCode(const i18n::phonenumbers::PhoneNumber& number, const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& carrier_code, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* formatted_number) const```</span>
      ///
      ///
pub fn format_national_number_with_carrier_code(&self, number: &::phonenumber::i18n::phonenumbers::PhoneNumber, carrier_code: &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, formatted_number: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_FormatNationalNumberWithCarrierCode(self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number as *const ::phonenumber::i18n::phonenumbers::PhoneNumber, carrier_code as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, formatted_number) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberUtil::FormatNationalNumberWithPreferredCarrierCode(const i18n::phonenumbers::PhoneNumber& number, const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& fallback_carrier_code, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* formatted_number) const```</span>
      ///
      ///
pub fn format_national_number_with_preferred_carrier_code(&self, number: &::phonenumber::i18n::phonenumbers::PhoneNumber, fallback_carrier_code: &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, formatted_number: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_FormatNationalNumberWithPreferredCarrierCode(self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number as *const ::phonenumber::i18n::phonenumbers::PhoneNumber, fallback_carrier_code as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, formatted_number) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberUtil::FormatNumberForMobileDialing(const i18n::phonenumbers::PhoneNumber& number, const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& region_calling_from, bool with_formatting, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* formatted_number) const```</span>
      ///
      ///
pub fn format_number_for_mobile_dialing(&self, number: &::phonenumber::i18n::phonenumbers::PhoneNumber, region_calling_from: &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, with_formatting: bool, formatted_number: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_FormatNumberForMobileDialing(self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number as *const ::phonenumber::i18n::phonenumbers::PhoneNumber, region_calling_from as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, with_formatting, formatted_number) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberUtil::FormatOutOfCountryCallingNumber(const i18n::phonenumbers::PhoneNumber& number, const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& calling_from, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* formatted_number) const```</span>
      ///
      ///
pub fn format_out_of_country_calling_number(&self, number: &::phonenumber::i18n::phonenumbers::PhoneNumber, calling_from: &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, formatted_number: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_FormatOutOfCountryCallingNumber(self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number as *const ::phonenumber::i18n::phonenumbers::PhoneNumber, calling_from as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, formatted_number) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberUtil::FormatOutOfCountryKeepingAlphaChars(const i18n::phonenumbers::PhoneNumber& number, const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& calling_from, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* formatted_number) const```</span>
      ///
      ///
pub fn format_out_of_country_keeping_alpha_chars(&self, number: &::phonenumber::i18n::phonenumbers::PhoneNumber, calling_from: &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, formatted_number: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_FormatOutOfCountryKeepingAlphaChars(self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number as *const ::phonenumber::i18n::phonenumbers::PhoneNumber, calling_from as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, formatted_number) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::AsYouTypeFormatter* i18n::phonenumbers::PhoneNumberUtil::GetAsYouTypeFormatter(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& region_code) const```</span>
      ///
      ///
pub fn get_as_you_type_formatter(&self, region_code: &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> *mut ::asyoutypeformatter::i18n::phonenumbers::AsYouTypeFormatter {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetAsYouTypeFormatter(self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, region_code as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
      }

      /// C++ method: <span style='color: green;'>```int i18n::phonenumbers::PhoneNumberUtil::GetCountryCodeForRegion(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& region_code) const```</span>
      ///
      ///
pub fn get_country_code_for_region(&self, region_code: &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> libc::c_int {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetCountryCodeForRegion(self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, region_code as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberUtil::GetCountryMobileToken(int country_calling_code, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* mobile_token) const```</span>
      ///
      ///
pub fn get_country_mobile_token(&self, country_calling_code: libc::c_int, mobile_token: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetCountryMobileToken(self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, country_calling_code, mobile_token) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneNumberUtil::GetExampleNumber(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& region_code, i18n::phonenumbers::PhoneNumber* number) const```</span>
      ///
      ///
pub fn get_example_number(&self, region_code: &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, number: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetExampleNumber(self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, region_code as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, number) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneNumberUtil::GetExampleNumberForNonGeoEntity(int country_calling_code, i18n::phonenumbers::PhoneNumber* number) const```</span>
      ///
      ///
      pub fn get_example_number_for_non_geo_entity(&self,
                                                   country_calling_code: libc::c_int,
                                                   number: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber)
                                                   -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetExampleNumberForNonGeoEntity(self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, country_calling_code, number) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberUtil::GetExampleNumberForType```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn get_example_number_for_type(&self, (&::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, ::phonenumberutil::i18n::phonenumbers::phone_number_util::PhoneNumberType, *mut ::phonenumber::i18n::phonenumbers::PhoneNumber)) -> bool```<br>
      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneNumberUtil::GetExampleNumberForType(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& region_code, i18n::phonenumbers::PhoneNumberUtil::PhoneNumberType type, i18n::phonenumbers::PhoneNumber* number) const```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn get_example_number_for_type(&self, (::phonenumberutil::i18n::phonenumbers::phone_number_util::PhoneNumberType, *mut ::phonenumber::i18n::phonenumbers::PhoneNumber)) -> bool```<br>
      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneNumberUtil::GetExampleNumberForType(i18n::phonenumbers::PhoneNumberUtil::PhoneNumberType type, i18n::phonenumbers::PhoneNumber* number) const```</span>
      ///
      ///
      pub fn get_example_number_for_type<'l0, Args>(&'l0 self, args: Args) -> bool
        where Args: overloading::PhoneNumberUtilGetExampleNumberForTypeArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```static i18n::phonenumbers::PhoneNumberUtil* i18n::phonenumbers::PhoneNumberUtil::GetInstance()```</span>
      ///
      ///
      /*OLD pub fn get_instance() -> *mut ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetInstance() }
      }*/
      pub fn get_instance() ->  &'static ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil {
	    let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetInstance() };
		unsafe { &*ffi_result }
	   }
      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneNumberUtil::GetInvalidExampleNumber(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& region_code, i18n::phonenumbers::PhoneNumber* number) const```</span>
      ///
      ///
pub fn get_invalid_example_number(&self, region_code: &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, number: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetInvalidExampleNumber(self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, region_code as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, number) }
      }

      /// C++ method: <span style='color: green;'>```int i18n::phonenumbers::PhoneNumberUtil::GetLengthOfGeographicalAreaCode(const i18n::phonenumbers::PhoneNumber& number) const```</span>
      ///
      ///
      pub fn get_length_of_geographical_area_code(&self,
                                                  number: &::phonenumber::i18n::phonenumbers::PhoneNumber)
                                                  -> libc::c_int {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetLengthOfGeographicalAreaCode(self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number as *const ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```int i18n::phonenumbers::PhoneNumberUtil::GetLengthOfNationalDestinationCode(const i18n::phonenumbers::PhoneNumber& number) const```</span>
      ///
      ///
      pub fn get_length_of_national_destination_code(&self,
                                                     number: &::phonenumber::i18n::phonenumbers::PhoneNumber)
                                                     -> libc::c_int {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetLengthOfNationalDestinationCode(self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number as *const ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberUtil::GetNationalSignificantNumber(const i18n::phonenumbers::PhoneNumber& number, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* national_significant_num) const```</span>
      ///
      ///
pub fn get_national_significant_number(&self, number: &::phonenumber::i18n::phonenumbers::PhoneNumber, national_significant_num: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetNationalSignificantNumber(self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number as *const ::phonenumber::i18n::phonenumbers::PhoneNumber, national_significant_num) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberUtil::GetNddPrefixForRegion(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& region_code, bool strip_non_digits, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* national_prefix) const```</span>
      ///
      ///
pub fn get_ndd_prefix_for_region(&self, region_code: &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, strip_non_digits: bool, national_prefix: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetNddPrefixForRegion(self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, region_code as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, strip_non_digits, national_prefix) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberUtil::PhoneNumberType i18n::phonenumbers::PhoneNumberUtil::GetNumberType(const i18n::phonenumbers::PhoneNumber& number) const```</span>
      ///
      ///
      pub fn get_number_type(&self,
                             number: &::phonenumber::i18n::phonenumbers::PhoneNumber)
                             -> ::phonenumberutil::i18n::phonenumbers::phone_number_util::PhoneNumberType {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetNumberType(self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number as *const ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberUtil::GetRegionCodeForCountryCode(int country_code, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* region_code) const```</span>
      ///
      ///
pub fn get_region_code_for_country_code(&self, country_code: libc::c_int, region_code: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetRegionCodeForCountryCode(self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, country_code, region_code) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberUtil::GetRegionCodeForNumber(const i18n::phonenumbers::PhoneNumber& number, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* region_code) const```</span>
      ///
      ///
pub fn get_region_code_for_number(&self, number: &::phonenumber::i18n::phonenumbers::PhoneNumber, region_code: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetRegionCodeForNumber(self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number as *const ::phonenumber::i18n::phonenumbers::PhoneNumber, region_code) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberUtil::GetRegionCodesForCountryCallingCode(int country_calling_code, std::__cxx11::list<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>, std::allocator<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>>>* region_codes) const```</span>
      ///
      ///
pub fn get_region_codes_for_country_calling_code(&self, country_calling_code: libc::c_int, region_codes: *mut ::stl_list::std::cxx11::ListBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRefAllocatorBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRefRef) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetRegionCodesForCountryCallingCode(self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, country_calling_code, region_codes) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberUtil::GetSupportedGlobalNetworkCallingCodes(std::set<int, std::less<int>, std::allocator<int>>* calling_codes) const```</span>
      ///
      ///
pub fn get_supported_global_network_calling_codes(&self, calling_codes: *mut ::stl_set::std::SetCIntLessCIntRefAllocatorCIntRef) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetSupportedGlobalNetworkCallingCodes(self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, calling_codes) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberUtil::GetSupportedRegions(std::set<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>, std::less<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>>, std::allocator<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>>>* regions) const```</span>
      ///
      ///
pub fn get_supported_regions(&self, regions: *mut ::stl_set::std::SetBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRefLessBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRefRefAllocatorBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRefRef) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetSupportedRegions(self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, regions) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneNumberUtil::IsAlphaNumber(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& number) const```</span>
      ///
      ///
pub fn is_alpha_number(&self, number: &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_IsAlphaNumber(self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneNumberUtil::IsNANPACountry(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& region_code) const```</span>
      ///
      ///
pub fn is_n_a_n_p_a_country(&self, region_code: &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_IsNANPACountry(self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, region_code as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberUtil::IsNumberGeographical```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn is_number_geographical(&self, &::phonenumber::i18n::phonenumbers::PhoneNumber) -> bool```<br>
      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneNumberUtil::IsNumberGeographical(const i18n::phonenumbers::PhoneNumber& phone_number) const```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn is_number_geographical(&self, (::phonenumberutil::i18n::phonenumbers::phone_number_util::PhoneNumberType, libc::c_int)) -> bool```<br>
      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneNumberUtil::IsNumberGeographical(i18n::phonenumbers::PhoneNumberUtil::PhoneNumberType phone_number_type, int country_calling_code) const```</span>
      ///
      ///
      pub fn is_number_geographical<'l0, Args>(&'l0 self, args: Args) -> bool
        where Args: overloading::PhoneNumberUtilIsNumberGeographicalArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberUtil::MatchType i18n::phonenumbers::PhoneNumberUtil::IsNumberMatch(const i18n::phonenumbers::PhoneNumber& first_number, const i18n::phonenumbers::PhoneNumber& second_number) const```</span>
      ///
      ///
      pub fn is_number_match(&self,
                             first_number: &::phonenumber::i18n::phonenumbers::PhoneNumber,
                             second_number: &::phonenumber::i18n::phonenumbers::PhoneNumber)
                             -> ::phonenumberutil::i18n::phonenumbers::phone_number_util::MatchType {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_IsNumberMatch(self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, first_number as *const ::phonenumber::i18n::phonenumbers::PhoneNumber, second_number as *const ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberUtil::MatchType i18n::phonenumbers::PhoneNumberUtil::IsNumberMatchWithOneString(const i18n::phonenumbers::PhoneNumber& first_number, const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& second_number) const```</span>
      ///
      ///
pub fn is_number_match_with_one_string(&self, first_number: &::phonenumber::i18n::phonenumbers::PhoneNumber, second_number: &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> ::phonenumberutil::i18n::phonenumbers::phone_number_util::MatchType {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_IsNumberMatchWithOneString(self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, first_number as *const ::phonenumber::i18n::phonenumbers::PhoneNumber, second_number as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberUtil::MatchType i18n::phonenumbers::PhoneNumberUtil::IsNumberMatchWithTwoStrings(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& first_number, const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& second_number) const```</span>
      ///
      ///
      pub fn is_number_match_with_two_strings
        (&self,
         first_number: &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,
         second_number: &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef)
         -> ::phonenumberutil::i18n::phonenumbers::phone_number_util::MatchType {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_IsNumberMatchWithTwoStrings(self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, first_number as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, second_number as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneNumberUtil::IsPossibleNumber(const i18n::phonenumbers::PhoneNumber& number) const```</span>
      ///
      ///
      pub fn is_possible_number(&self, number: &::phonenumber::i18n::phonenumbers::PhoneNumber) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_IsPossibleNumber(self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number as *const ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneNumberUtil::IsPossibleNumberForString(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& number, const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& region_dialing_from) const```</span>
      ///
      ///
pub fn is_possible_number_for_string(&self, number: &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, region_dialing_from: &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_IsPossibleNumberForString(self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, region_dialing_from as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberUtil::ValidationResult i18n::phonenumbers::PhoneNumberUtil::IsPossibleNumberWithReason(const i18n::phonenumbers::PhoneNumber& number) const```</span>
      ///
      ///
      pub fn is_possible_number_with_reason
        (&self,
         number: &::phonenumber::i18n::phonenumbers::PhoneNumber)
         -> ::phonenumberutil::i18n::phonenumbers::phone_number_util::ValidationResult {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_IsPossibleNumberWithReason(self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number as *const ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneNumberUtil::IsValidNumber(const i18n::phonenumbers::PhoneNumber& number) const```</span>
      ///
      ///
      pub fn is_valid_number(&self, number: &::phonenumber::i18n::phonenumbers::PhoneNumber) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_IsValidNumber(self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number as *const ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneNumberUtil::IsValidNumberForRegion(const i18n::phonenumbers::PhoneNumber& number, const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& region_code) const```</span>
      ///
      ///
pub fn is_valid_number_for_region(&self, number: &::phonenumber::i18n::phonenumbers::PhoneNumber, region_code: &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_IsValidNumberForRegion(self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number as *const ::phonenumber::i18n::phonenumbers::PhoneNumber, region_code as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberUtil::NormalizeDiallableCharsOnly(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* number) const```</span>
      ///
      ///
pub fn normalize_diallable_chars_only(&self, number: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_NormalizeDiallableCharsOnly(self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number) }
      }

      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::PhoneNumberUtil::NormalizeDigitsOnly(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>* number) const```</span>
      ///
      ///
pub fn normalize_digits_only(&self, number: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_NormalizeDigitsOnly(self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberUtil::ErrorType i18n::phonenumbers::PhoneNumberUtil::Parse(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& number_to_parse, const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& default_region, i18n::phonenumbers::PhoneNumber* number) const```</span>
      ///
      ///
/* OLD pub fn parse(&self, number_to_parse: &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, default_region: &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, number: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber) -> ::phonenumberutil::i18n::phonenumbers::phone_number_util::ErrorType {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_Parse(self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number_to_parse as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, default_region as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, number) }
		}*/
pub fn parse_into(&self, number_to_parse: &str, default_region: &str, number: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber) -> ::phonenumberutil::i18n::phonenumbers::phone_number_util::ErrorType {
    unsafe {	::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_Parse_Str(self as *const PhoneNumberUtil, number_to_parse.as_ptr(), number_to_parse.len(), default_region.as_ptr(), default_region.len(), number) }
}
pub fn parse(&self, number_to_parse: &str, default_region: &str) -> Result<&mut ::phonenumber::i18n::phonenumbers::PhoneNumber, ::phonenumberutil::i18n::phonenumbers::phone_number_util::ErrorType> {
    let new_number = ::phonenumber::i18n::phonenumbers::PhoneNumber::new();
	let result = unsafe {	::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_Parse_Str(self as *const PhoneNumberUtil, number_to_parse.as_ptr(), number_to_parse.len(), default_region.as_ptr(), default_region.len(), new_number)  };
	match result {
	    ::phonenumberutil::i18n::phonenumbers::phone_number_util::ErrorType::NOPARSINGERROR => Ok(new_number),
		_ => Err(result)
	 }
}

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberUtil::ErrorType i18n::phonenumbers::PhoneNumberUtil::ParseAndKeepRawInput(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& number_to_parse, const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& default_region, i18n::phonenumbers::PhoneNumber* number) const```</span>
      ///
      ///
pub fn parse_and_keep_raw_input(&self, number_to_parse: &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, default_region: &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, number: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber) -> ::phonenumberutil::i18n::phonenumbers::phone_number_util::ErrorType {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_ParseAndKeepRawInput(self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number_to_parse as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, default_region as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, number) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::PhoneNumberUtil::TruncateTooLongNumber(i18n::phonenumbers::PhoneNumber* number) const```</span>
      ///
      ///
      pub fn truncate_too_long_number(&self, number: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_TruncateTooLongNumber(self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number) }
      }
    }

    impl Drop for PhoneNumberUtil {
      /// C++ method: <span style='color: green;'>```virtual [destructor] void i18n::phonenumbers::PhoneNumberUtil::~PhoneNumberUtil()```</span>
      ///
      ///
      fn drop(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_destructor(self as *mut ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil) }
      }
    }

    impl cpp_utils::CppDeletable for PhoneNumberUtil {
      fn deleter() -> cpp_utils::Deleter<Self> {
        ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_delete
      }
    }

    pub mod overloading {
      #[allow(unused_imports)]
      use {libc, cpp_utils, std};

      /// This trait represents a set of arguments accepted by [PhoneNumberUtil::get_example_number_for_type](../struct.PhoneNumberUtil.html#method.get_example_number_for_type) method.
      pub trait PhoneNumberUtilGetExampleNumberForTypeArgs<'l0> {
        fn exec(self, original_self: &'l0 ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil) -> bool;
      }
      impl<'l0> PhoneNumberUtilGetExampleNumberForTypeArgs<'l0> for (&'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,::phonenumberutil::i18n::phonenumbers::phone_number_util::PhoneNumberType,*mut ::phonenumber::i18n::phonenumbers::PhoneNumber) {

  fn exec(self, original_self: &'l0 ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil) -> bool {
    let region_code = self.0;
let type_ = self.1;
let number = self.2;
    unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetExampleNumberForType_region_code_type_number(original_self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, region_code as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, type_, number) }
  }
}
      impl<'l0> PhoneNumberUtilGetExampleNumberForTypeArgs<'l0> for (::phonenumberutil::i18n::phonenumbers::phone_number_util::PhoneNumberType,*mut ::phonenumber::i18n::phonenumbers::PhoneNumber) {

  fn exec(self, original_self: &'l0 ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil) -> bool {
    let type_ = self.0;
let number = self.1;
    unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetExampleNumberForType_type_number(original_self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, type_, number) }
  }
}
      /// This trait represents a set of arguments accepted by [PhoneNumberUtil::is_number_geographical](../struct.PhoneNumberUtil.html#method.is_number_geographical) method.
      pub trait PhoneNumberUtilIsNumberGeographicalArgs<'l0> {
        fn exec(self, original_self: &'l0 ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil) -> bool;
      }
      impl<'l0> PhoneNumberUtilIsNumberGeographicalArgs<'l0> for &'l0 ::phonenumber::i18n::phonenumbers::PhoneNumber {
        fn exec(self, original_self: &'l0 ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil) -> bool {
          let phone_number = self;
          unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_IsNumberGeographical_phone_number(original_self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, phone_number as *const ::phonenumber::i18n::phonenumbers::PhoneNumber) }
        }
      }
      impl<'l0> PhoneNumberUtilIsNumberGeographicalArgs<'l0> for (::phonenumberutil::i18n::phonenumbers::phone_number_util::PhoneNumberType,libc::c_int) {

  fn exec(self, original_self: &'l0 ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil) -> bool {
    let phone_number_type = self.0;
let country_calling_code = self.1;
    unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_IsNumberGeographical_phone_number_type_country_calling_code(original_self as *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, phone_number_type, country_calling_code) }
  }
}
    }

    pub mod phone_number_util {
      #[allow(unused_imports)]
      use {libc, cpp_utils, std};

      /// C++ type: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberUtil::ErrorType```</span>
      #[derive(Debug, PartialEq, Eq, Clone)]
      #[repr(C)]
      pub enum ErrorType {
        /// C++ variant: NO_PARSING_ERROR
        NOPARSINGERROR = 0,
        /// C++ variant: INVALID_COUNTRY_CODE_ERROR
        InvalidCountryCodeError = 1,
        /// C++ variant: NOT_A_NUMBER
        NotANUMBER = 2,
        /// C++ variant: TOO_SHORT_AFTER_IDD
        TooShortAfterIdd = 3,
        /// C++ variant: TOO_SHORT_NSN
        TooShortNsn = 4,
        /// C++ variant: TOO_LONG_NSN
        TooLongNsn = 5,
      }

      /// C++ type: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberUtil::MatchType```</span>
      #[derive(Debug, PartialEq, Eq, Clone)]
      #[repr(C)]
      pub enum MatchType {
        /// C++ variant: INVALID_NUMBER
        InvalidNumber = 0,
        /// C++ variant: NO_MATCH
        NOMATCH = 1,
        /// C++ variant: SHORT_NSN_MATCH
        ShortNsnMatch = 2,
        /// C++ variant: NSN_MATCH
        NsnMatch = 3,
        /// C++ variant: EXACT_MATCH
        ExactMatch = 4,
      }

      /// C++ type: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberUtil::PhoneNumberFormat```</span>
      #[derive(Debug, PartialEq, Eq, Clone)]
      #[repr(C)]
      pub enum PhoneNumberFormat {
        /// C++ variant: E164
        E164 = 0,
        /// C++ variant: INTERNATIONAL
        International = 1,
        /// C++ variant: NATIONAL
        National = 2,
        /// C++ variant: RFC3966
        Rfc3966 = 3,
      }

      /// C++ type: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberUtil::PhoneNumberType```</span>
      #[derive(Debug, PartialEq, Eq, Clone)]
      #[repr(C)]
      pub enum PhoneNumberType {
        /// C++ variant: FIXED_LINE
        FixedLine = 0,
        /// C++ variant: MOBILE
        Mobile = 1,
        /// C++ variant: FIXED_LINE_OR_MOBILE
        FixedLineORMOBILE = 2,
        /// C++ variant: TOLL_FREE
        TollFree = 3,
        /// C++ variant: PREMIUM_RATE
        PremiumRate = 4,
        /// C++ variant: SHARED_COST
        SharedCost = 5,
        /// C++ variant: VOIP
        Voip = 6,
        /// C++ variant: PERSONAL_NUMBER
        PersonalNumber = 7,
        /// C++ variant: PAGER
        Pager = 8,
        /// C++ variant: UAN
        Uan = 9,
        /// C++ variant: VOICEMAIL
        Voicemail = 10,
        /// C++ variant: UNKNOWN
        Unknown = 11,
      }

      /// C++ type: <span style='color: green;'>```i18n::phonenumbers::PhoneNumberUtil::ValidationResult```</span>
      #[derive(Debug, PartialEq, Eq, Clone)]
      #[repr(C)]
      pub enum ValidationResult {
        /// C++ variant: IS_POSSIBLE
        ISPOSSIBLE = 0,
        /// C++ variant: INVALID_COUNTRY_CODE
        InvalidCountryCode = 1,
        /// C++ variant: TOO_SHORT
        TooShort = 2,
        /// C++ variant: TOO_LONG
        TooLong = 3,
      }

    }

  }

}
