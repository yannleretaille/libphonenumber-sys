#ifndef LIBPHONENUMBER_SYS_C_ASYOUTYPEFORMATTER_H
#define LIBPHONENUMBER_SYS_C_ASYOUTYPEFORMATTER_H

#include "libphonenumber_sys_c_global.h"

extern "C" {

LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_AsYouTypeFormatter_Clear(i18n::phonenumbers::AsYouTypeFormatter* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT int libphonenumber_sys_c_i18n_phonenumbers_AsYouTypeFormatter_GetRememberedPosition(const i18n::phonenumbers::AsYouTypeFormatter* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_AsYouTypeFormatter_InputDigit(i18n::phonenumbers::AsYouTypeFormatter* this_ptr, int next_char, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* result);
LIBPHONENUMBER_SYS_C_EXPORT const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_AsYouTypeFormatter_InputDigitAndRememberPosition(i18n::phonenumbers::AsYouTypeFormatter* this_ptr, int next_char, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* result);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_AsYouTypeFormatter_delete(i18n::phonenumbers::AsYouTypeFormatter* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_AsYouTypeFormatter_destructor(i18n::phonenumbers::AsYouTypeFormatter* this_ptr);

} // extern "C"

#endif // LIBPHONENUMBER_SYS_C_ASYOUTYPEFORMATTER_H
