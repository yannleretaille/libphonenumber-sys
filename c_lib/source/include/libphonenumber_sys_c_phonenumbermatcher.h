#ifndef LIBPHONENUMBER_SYS_C_PHONENUMBERMATCHER_H
#define LIBPHONENUMBER_SYS_C_PHONENUMBERMATCHER_H

#include "libphonenumber_sys_c_global.h"

extern "C" {

LIBPHONENUMBER_SYS_C_EXPORT bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatcher_HasNext(i18n::phonenumbers::PhoneNumberMatcher* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatcher_Next(i18n::phonenumbers::PhoneNumberMatcher* this_ptr, i18n::phonenumbers::PhoneNumberMatch* match);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatcher_constructor_text_region_code(const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* text, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* region_code, i18n::phonenumbers::PhoneNumberMatcher* output);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatcher_constructor_util_text_region_code_leniency_max_tries(const i18n::phonenumbers::PhoneNumberUtil* util, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* text, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* region_code, i18n::phonenumbers::PhoneNumberMatcher::Leniency leniency, int max_tries, i18n::phonenumbers::PhoneNumberMatcher* output);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatcher_delete(i18n::phonenumbers::PhoneNumberMatcher* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatcher_destructor(i18n::phonenumbers::PhoneNumberMatcher* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::PhoneNumberMatcher* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatcher_new_text_region_code(const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* text, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* region_code);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::PhoneNumberMatcher* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatcher_new_util_text_region_code_leniency_max_tries(const i18n::phonenumbers::PhoneNumberUtil* util, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* text, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* region_code, i18n::phonenumbers::PhoneNumberMatcher::Leniency leniency, int max_tries);

} // extern "C"

#endif // LIBPHONENUMBER_SYS_C_PHONENUMBERMATCHER_H
