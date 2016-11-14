#include "libphonenumber_sys_c_shortnumberinfo.h"

bool libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_ConnectsToEmergencyNumber(const i18n::phonenumbers::ShortNumberInfo* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* number, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* region_code) {
  return this_ptr->ConnectsToEmergencyNumber(*number, *region_code);
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_GetExampleShortNumberForCost_as_ptr(const i18n::phonenumbers::ShortNumberInfo* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* region_code, i18n::phonenumbers::ShortNumberInfo::ShortNumberCost cost) {
  return new std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >(this_ptr->GetExampleShortNumberForCost(*region_code, cost));
}

void libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_GetExampleShortNumberForCost_to_output(const i18n::phonenumbers::ShortNumberInfo* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* region_code, i18n::phonenumbers::ShortNumberInfo::ShortNumberCost cost, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* output) {
  new(output) std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >(this_ptr->GetExampleShortNumberForCost(*region_code, cost));
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_GetExampleShortNumber_as_ptr(const i18n::phonenumbers::ShortNumberInfo* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* region_code) {
  return new std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >(this_ptr->GetExampleShortNumber(*region_code));
}

void libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_GetExampleShortNumber_to_output(const i18n::phonenumbers::ShortNumberInfo* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* region_code, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* output) {
  new(output) std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >(this_ptr->GetExampleShortNumber(*region_code));
}

i18n::phonenumbers::ShortNumberInfo::ShortNumberCost libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_GetExpectedCost(const i18n::phonenumbers::ShortNumberInfo* this_ptr, const i18n::phonenumbers::PhoneNumber* number) {
  return this_ptr->GetExpectedCost(*number);
}

i18n::phonenumbers::ShortNumberInfo::ShortNumberCost libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_GetExpectedCostForRegion_i18n_phonenumbers_PhoneNumber_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(const i18n::phonenumbers::ShortNumberInfo* this_ptr, const i18n::phonenumbers::PhoneNumber* short_number, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* region_dialing_from) {
  return this_ptr->GetExpectedCostForRegion(*short_number, *region_dialing_from);
}

i18n::phonenumbers::ShortNumberInfo::ShortNumberCost libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_GetExpectedCostForRegion_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(const i18n::phonenumbers::ShortNumberInfo* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* short_number, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* region_dialing_from) {
  return this_ptr->GetExpectedCostForRegion(*short_number, *region_dialing_from);
}

bool libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_IsCarrierSpecific(const i18n::phonenumbers::ShortNumberInfo* this_ptr, const i18n::phonenumbers::PhoneNumber* number) {
  return this_ptr->IsCarrierSpecific(*number);
}

bool libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_IsEmergencyNumber(const i18n::phonenumbers::ShortNumberInfo* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* number, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* region_code) {
  return this_ptr->IsEmergencyNumber(*number, *region_code);
}

bool libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_IsPossibleShortNumber(const i18n::phonenumbers::ShortNumberInfo* this_ptr, const i18n::phonenumbers::PhoneNumber* number) {
  return this_ptr->IsPossibleShortNumber(*number);
}

bool libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_IsPossibleShortNumberForRegion_i18n_phonenumbers_PhoneNumber_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(const i18n::phonenumbers::ShortNumberInfo* this_ptr, const i18n::phonenumbers::PhoneNumber* short_number, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* region_dialing_from) {
  return this_ptr->IsPossibleShortNumberForRegion(*short_number, *region_dialing_from);
}

bool libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_IsPossibleShortNumberForRegion_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(const i18n::phonenumbers::ShortNumberInfo* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* short_number, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* region_dialing_from) {
  return this_ptr->IsPossibleShortNumberForRegion(*short_number, *region_dialing_from);
}

bool libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_IsValidShortNumber(const i18n::phonenumbers::ShortNumberInfo* this_ptr, const i18n::phonenumbers::PhoneNumber* number) {
  return this_ptr->IsValidShortNumber(*number);
}

bool libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_IsValidShortNumberForRegion_i18n_phonenumbers_PhoneNumber_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(const i18n::phonenumbers::ShortNumberInfo* this_ptr, const i18n::phonenumbers::PhoneNumber* short_number, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* region_dialing_from) {
  return this_ptr->IsValidShortNumberForRegion(*short_number, *region_dialing_from);
}

bool libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_IsValidShortNumberForRegion_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(const i18n::phonenumbers::ShortNumberInfo* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* short_number, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* region_dialing_from) {
  return this_ptr->IsValidShortNumberForRegion(*short_number, *region_dialing_from);
}

void libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_constructor(i18n::phonenumbers::ShortNumberInfo* output) {
  new(output) i18n::phonenumbers::ShortNumberInfo();
}

void libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_delete(i18n::phonenumbers::ShortNumberInfo* this_ptr) {
  delete this_ptr;
}

void libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_destructor(i18n::phonenumbers::ShortNumberInfo* this_ptr) {
  libphonenumber_sys_c_call_destructor(this_ptr);
}

i18n::phonenumbers::ShortNumberInfo* libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_new() {
  return new i18n::phonenumbers::ShortNumberInfo();
}

