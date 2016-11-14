#ifndef LIBPHONENUMBER_SYS_C_PHONENUMBERMATCH_H
#define LIBPHONENUMBER_SYS_C_PHONENUMBERMATCH_H

#include "libphonenumber_sys_c_global.h"

extern "C" {

LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_CopyFrom(i18n::phonenumbers::PhoneNumberMatch* this_ptr, const i18n::phonenumbers::PhoneNumberMatch* number);
LIBPHONENUMBER_SYS_C_EXPORT bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_Equals(const i18n::phonenumbers::PhoneNumberMatch* this_ptr, const i18n::phonenumbers::PhoneNumberMatch* number);
LIBPHONENUMBER_SYS_C_EXPORT std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_ToString_as_ptr(const i18n::phonenumbers::PhoneNumberMatch* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_ToString_to_output(const i18n::phonenumbers::PhoneNumberMatch* this_ptr, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* output);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_constructor_no_args(i18n::phonenumbers::PhoneNumberMatch* output);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_constructor_start_raw_string_number(int start, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* raw_string, const i18n::phonenumbers::PhoneNumber* number, i18n::phonenumbers::PhoneNumberMatch* output);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_delete(i18n::phonenumbers::PhoneNumberMatch* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_destructor(i18n::phonenumbers::PhoneNumberMatch* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT int libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_end(const i18n::phonenumbers::PhoneNumberMatch* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT int libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_length(const i18n::phonenumbers::PhoneNumberMatch* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::PhoneNumberMatch* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_new_no_args();
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::PhoneNumberMatch* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_new_start_raw_string_number(int start, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* raw_string, const i18n::phonenumbers::PhoneNumber* number);
LIBPHONENUMBER_SYS_C_EXPORT const i18n::phonenumbers::PhoneNumber* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_number(const i18n::phonenumbers::PhoneNumberMatch* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_raw_string(const i18n::phonenumbers::PhoneNumberMatch* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_set_number(i18n::phonenumbers::PhoneNumberMatch* this_ptr, const i18n::phonenumbers::PhoneNumber* number);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_set_raw_string(i18n::phonenumbers::PhoneNumberMatch* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* raw_string);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_set_start(i18n::phonenumbers::PhoneNumberMatch* this_ptr, int start);
LIBPHONENUMBER_SYS_C_EXPORT int libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_start(const i18n::phonenumbers::PhoneNumberMatch* this_ptr);

} // extern "C"

#endif // LIBPHONENUMBER_SYS_C_PHONENUMBERMATCH_H
