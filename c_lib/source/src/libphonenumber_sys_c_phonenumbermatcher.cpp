#include "libphonenumber_sys_c_phonenumbermatcher.h"

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatcher_HasNext(i18n::phonenumbers::PhoneNumberMatcher* this_ptr) {
  return this_ptr->HasNext();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatcher_Next(i18n::phonenumbers::PhoneNumberMatcher* this_ptr, i18n::phonenumbers::PhoneNumberMatch* match) {
  return this_ptr->Next(match);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatcher_constructor_text_region_code(const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* text, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* region_code, i18n::phonenumbers::PhoneNumberMatcher* output) {
  new(output) i18n::phonenumbers::PhoneNumberMatcher(*text, *region_code);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatcher_constructor_util_text_region_code_leniency_max_tries(const i18n::phonenumbers::PhoneNumberUtil* util, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* text, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* region_code, i18n::phonenumbers::PhoneNumberMatcher::Leniency leniency, int max_tries, i18n::phonenumbers::PhoneNumberMatcher* output) {
  new(output) i18n::phonenumbers::PhoneNumberMatcher(*util, *text, *region_code, leniency, max_tries);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatcher_delete(i18n::phonenumbers::PhoneNumberMatcher* this_ptr) {
  delete this_ptr;
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatcher_destructor(i18n::phonenumbers::PhoneNumberMatcher* this_ptr) {
  libphonenumber_sys_c_call_destructor(this_ptr);
}

i18n::phonenumbers::PhoneNumberMatcher* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatcher_new_text_region_code(const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* text, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* region_code) {
  return new i18n::phonenumbers::PhoneNumberMatcher(*text, *region_code);
}

i18n::phonenumbers::PhoneNumberMatcher* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatcher_new_util_text_region_code_leniency_max_tries(const i18n::phonenumbers::PhoneNumberUtil* util, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* text, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* region_code, i18n::phonenumbers::PhoneNumberMatcher::Leniency leniency, int max_tries) {
  return new i18n::phonenumbers::PhoneNumberMatcher(*util, *text, *region_code, leniency, max_tries);
}

