#include "libphonenumber_sys_c_phonenumberutil.h"

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_ConvertAlphaCharactersInNumber(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* number) {
  this_ptr->ConvertAlphaCharactersInNumber(number);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_Format(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, const i18n::phonenumbers::PhoneNumber* number, i18n::phonenumbers::PhoneNumberUtil::PhoneNumberFormat number_format, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* formatted_number) {
  this_ptr->Format(*number, number_format, formatted_number);
}

const char* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_Format_Str(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, const i18n::phonenumbers::PhoneNumber* number, i18n::phonenumbers::PhoneNumberUtil::PhoneNumberFormat number_format) {
  auto result_str = new std::string();
  this_ptr->Format(*number, number_format, result_str);
  return result_str->data();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_FormatByPattern(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, const i18n::phonenumbers::PhoneNumber* number, i18n::phonenumbers::PhoneNumberUtil::PhoneNumberFormat number_format, const google::protobuf::RepeatedPtrField< i18n::phonenumbers::NumberFormat >* user_defined_formats, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* formatted_number) {
  this_ptr->FormatByPattern(*number, number_format, *user_defined_formats, formatted_number);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_FormatInOriginalFormat(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, const i18n::phonenumbers::PhoneNumber* number, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* region_calling_from, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* formatted_number) {
  this_ptr->FormatInOriginalFormat(*number, *region_calling_from, formatted_number);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_FormatNationalNumberWithCarrierCode(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, const i18n::phonenumbers::PhoneNumber* number, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* carrier_code, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* formatted_number) {
  this_ptr->FormatNationalNumberWithCarrierCode(*number, *carrier_code, formatted_number);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_FormatNationalNumberWithPreferredCarrierCode(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, const i18n::phonenumbers::PhoneNumber* number, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* fallback_carrier_code, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* formatted_number) {
  this_ptr->FormatNationalNumberWithPreferredCarrierCode(*number, *fallback_carrier_code, formatted_number);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_FormatNumberForMobileDialing(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, const i18n::phonenumbers::PhoneNumber* number, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* region_calling_from, bool with_formatting, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* formatted_number) {
  this_ptr->FormatNumberForMobileDialing(*number, *region_calling_from, with_formatting, formatted_number);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_FormatOutOfCountryCallingNumber(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, const i18n::phonenumbers::PhoneNumber* number, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* calling_from, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* formatted_number) {
  this_ptr->FormatOutOfCountryCallingNumber(*number, *calling_from, formatted_number);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_FormatOutOfCountryKeepingAlphaChars(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, const i18n::phonenumbers::PhoneNumber* number, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* calling_from, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* formatted_number) {
  this_ptr->FormatOutOfCountryKeepingAlphaChars(*number, *calling_from, formatted_number);
}

i18n::phonenumbers::AsYouTypeFormatter* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetAsYouTypeFormatter(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* region_code) {
  return this_ptr->GetAsYouTypeFormatter(*region_code);
}

int libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetCountryCodeForRegion(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* region_code) {
  return this_ptr->GetCountryCodeForRegion(*region_code);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetCountryMobileToken(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, int country_calling_code, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* mobile_token) {
  this_ptr->GetCountryMobileToken(country_calling_code, mobile_token);
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetExampleNumber(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* region_code, i18n::phonenumbers::PhoneNumber* number) {
  return this_ptr->GetExampleNumber(*region_code, number);
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetExampleNumberForNonGeoEntity(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, int country_calling_code, i18n::phonenumbers::PhoneNumber* number) {
  return this_ptr->GetExampleNumberForNonGeoEntity(country_calling_code, number);
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetExampleNumberForType_region_code_type_number(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* region_code, i18n::phonenumbers::PhoneNumberUtil::PhoneNumberType type, i18n::phonenumbers::PhoneNumber* number) {
  return this_ptr->GetExampleNumberForType(*region_code, type, number);
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetExampleNumberForType_type_number(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, i18n::phonenumbers::PhoneNumberUtil::PhoneNumberType type, i18n::phonenumbers::PhoneNumber* number) {
  return this_ptr->GetExampleNumberForType(type, number);
}

i18n::phonenumbers::PhoneNumberUtil* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetInstance() {
  return i18n::phonenumbers::PhoneNumberUtil::GetInstance();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetInvalidExampleNumber(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* region_code, i18n::phonenumbers::PhoneNumber* number) {
  return this_ptr->GetInvalidExampleNumber(*region_code, number);
}

int libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetLengthOfGeographicalAreaCode(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, const i18n::phonenumbers::PhoneNumber* number) {
  return this_ptr->GetLengthOfGeographicalAreaCode(*number);
}

int libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetLengthOfNationalDestinationCode(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, const i18n::phonenumbers::PhoneNumber* number) {
  return this_ptr->GetLengthOfNationalDestinationCode(*number);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetNationalSignificantNumber(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, const i18n::phonenumbers::PhoneNumber* number, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* national_significant_num) {
  this_ptr->GetNationalSignificantNumber(*number, national_significant_num);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetNddPrefixForRegion(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* region_code, bool strip_non_digits, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* national_prefix) {
  this_ptr->GetNddPrefixForRegion(*region_code, strip_non_digits, national_prefix);
}

i18n::phonenumbers::PhoneNumberUtil::PhoneNumberType libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetNumberType(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, const i18n::phonenumbers::PhoneNumber* number) {
  return this_ptr->GetNumberType(*number);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetRegionCodeForCountryCode(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, int country_code, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* region_code) {
  this_ptr->GetRegionCodeForCountryCode(country_code, region_code);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetRegionCodeForNumber(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, const i18n::phonenumbers::PhoneNumber* number, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* region_code) {
  this_ptr->GetRegionCodeForNumber(*number, region_code);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetRegionCodesForCountryCallingCode(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, int country_calling_code, std::__cxx11::list< std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >, std::allocator< std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > > > >* region_codes) {
  this_ptr->GetRegionCodesForCountryCallingCode(country_calling_code, region_codes);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetSupportedGlobalNetworkCallingCodes(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, std::set< int, std::less< int >, std::allocator< int > >* calling_codes) {
  this_ptr->GetSupportedGlobalNetworkCallingCodes(calling_codes);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetSupportedRegions(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, std::set< std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >, std::less< std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > > >, std::allocator< std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > > > >* regions) {
  this_ptr->GetSupportedRegions(regions);
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_IsAlphaNumber(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* number) {
  return this_ptr->IsAlphaNumber(*number);
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_IsNANPACountry(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* region_code) {
  return this_ptr->IsNANPACountry(*region_code);
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_IsNumberGeographical_phone_number(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, const i18n::phonenumbers::PhoneNumber* phone_number) {
  return this_ptr->IsNumberGeographical(*phone_number);
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_IsNumberGeographical_phone_number_type_country_calling_code(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, i18n::phonenumbers::PhoneNumberUtil::PhoneNumberType phone_number_type, int country_calling_code) {
  return this_ptr->IsNumberGeographical(phone_number_type, country_calling_code);
}

i18n::phonenumbers::PhoneNumberUtil::MatchType libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_IsNumberMatch(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, const i18n::phonenumbers::PhoneNumber* first_number, const i18n::phonenumbers::PhoneNumber* second_number) {
  return this_ptr->IsNumberMatch(*first_number, *second_number);
}

i18n::phonenumbers::PhoneNumberUtil::MatchType libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_IsNumberMatchWithOneString(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, const i18n::phonenumbers::PhoneNumber* first_number, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* second_number) {
  return this_ptr->IsNumberMatchWithOneString(*first_number, *second_number);
}

i18n::phonenumbers::PhoneNumberUtil::MatchType libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_IsNumberMatchWithTwoStrings(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* first_number, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* second_number) {
  return this_ptr->IsNumberMatchWithTwoStrings(*first_number, *second_number);
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_IsPossibleNumber(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, const i18n::phonenumbers::PhoneNumber* number) {
  return this_ptr->IsPossibleNumber(*number);
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_IsPossibleNumberForString(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* number, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* region_dialing_from) {
  return this_ptr->IsPossibleNumberForString(*number, *region_dialing_from);
}

i18n::phonenumbers::PhoneNumberUtil::ValidationResult libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_IsPossibleNumberWithReason(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, const i18n::phonenumbers::PhoneNumber* number) {
  return this_ptr->IsPossibleNumberWithReason(*number);
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_IsValidNumber(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, const i18n::phonenumbers::PhoneNumber* number) {
  return this_ptr->IsValidNumber(*number);
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_IsValidNumberForRegion(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, const i18n::phonenumbers::PhoneNumber* number, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* region_code) {
  return this_ptr->IsValidNumberForRegion(*number, *region_code);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_NormalizeDiallableCharsOnly(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* number) {
  this_ptr->NormalizeDiallableCharsOnly(number);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_NormalizeDigitsOnly(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* number) {
  this_ptr->NormalizeDigitsOnly(number);
}

i18n::phonenumbers::PhoneNumberUtil::ErrorType libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_Parse(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* number_to_parse, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* default_region, i18n::phonenumbers::PhoneNumber* number) {
  return this_ptr->Parse(*number_to_parse, *default_region, number);
}

i18n::phonenumbers::PhoneNumberUtil::ErrorType libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_Parse_Str(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, char* number_to_parse_str, size_t number_to_parse_len, char* default_region_str, size_t default_region_len, i18n::phonenumbers::PhoneNumber* number) {
  auto number_to_parse = std::string(number_to_parse_str,number_to_parse_len);
  auto default_region = std::string(default_region_str,default_region_len);
  return this_ptr->Parse(number_to_parse, default_region, number);
}

i18n::phonenumbers::PhoneNumberUtil::ErrorType libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_ParseAndKeepRawInput(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* number_to_parse, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* default_region, i18n::phonenumbers::PhoneNumber* number) {
  return this_ptr->ParseAndKeepRawInput(*number_to_parse, *default_region, number);
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_TruncateTooLongNumber(const i18n::phonenumbers::PhoneNumberUtil* this_ptr, i18n::phonenumbers::PhoneNumber* number) {
  return this_ptr->TruncateTooLongNumber(number);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_delete(i18n::phonenumbers::PhoneNumberUtil* this_ptr) {
  delete this_ptr;
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_destructor(i18n::phonenumbers::PhoneNumberUtil* this_ptr) {
  libphonenumber_sys_c_call_destructor(this_ptr);
}

