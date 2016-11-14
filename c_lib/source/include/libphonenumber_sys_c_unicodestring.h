#ifndef LIBPHONENUMBER_SYS_C_UNICODESTRING_H
#define LIBPHONENUMBER_SYS_C_UNICODESTRING_H

#include "libphonenumber_sys_c_global.h"

extern "C" {

LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_append_codepoint(i18n::phonenumbers::UnicodeString* this_ptr, int codepoint);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_append_unicode_string(i18n::phonenumbers::UnicodeString* this_ptr, const i18n::phonenumbers::UnicodeString* unicode_string);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::UnicodeText::const_iterator* libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_begin_as_ptr(const i18n::phonenumbers::UnicodeString* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_begin_to_output(const i18n::phonenumbers::UnicodeString* this_ptr, i18n::phonenumbers::UnicodeText::const_iterator* output);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_constructor_codepoint(int codepoint, i18n::phonenumbers::UnicodeString* output);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_constructor_no_args(i18n::phonenumbers::UnicodeString* output);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_constructor_src(const i18n::phonenumbers::UnicodeString* src, i18n::phonenumbers::UnicodeString* output);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_constructor_utf8(const char* utf8, i18n::phonenumbers::UnicodeString* output);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::UnicodeText::const_iterator* libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_end_as_ptr(const i18n::phonenumbers::UnicodeString* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_end_to_output(const i18n::phonenumbers::UnicodeString* this_ptr, i18n::phonenumbers::UnicodeText::const_iterator* output);
LIBPHONENUMBER_SYS_C_EXPORT int libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_indexOf(const i18n::phonenumbers::UnicodeString* this_ptr, int codepoint);
LIBPHONENUMBER_SYS_C_EXPORT int libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_length(const i18n::phonenumbers::UnicodeString* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::UnicodeString* libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_new_codepoint(int codepoint);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::UnicodeString* libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_new_no_args();
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::UnicodeString* libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_new_src(const i18n::phonenumbers::UnicodeString* src);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::UnicodeString* libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_new_utf8(const char* utf8);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::UnicodeString* libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_operator_assign(i18n::phonenumbers::UnicodeString* this_ptr, const i18n::phonenumbers::UnicodeString* src);
LIBPHONENUMBER_SYS_C_EXPORT bool libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_operator_eq(const i18n::phonenumbers::UnicodeString* this_ptr, const i18n::phonenumbers::UnicodeString* rhs);
LIBPHONENUMBER_SYS_C_EXPORT int libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_operator_index(const i18n::phonenumbers::UnicodeString* this_ptr, int index);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_remove(i18n::phonenumbers::UnicodeString* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_replace(i18n::phonenumbers::UnicodeString* this_ptr, int start, int length, const i18n::phonenumbers::UnicodeString* src);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_setCharAt(i18n::phonenumbers::UnicodeString* this_ptr, int pos, int c);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_setTo(i18n::phonenumbers::UnicodeString* this_ptr, const char* s, unsigned long len);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::UnicodeString* libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_tempSubString_as_ptr_start(const i18n::phonenumbers::UnicodeString* this_ptr, int start);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::UnicodeString* libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_tempSubString_as_ptr_start_length(const i18n::phonenumbers::UnicodeString* this_ptr, int start, int length);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_tempSubString_to_output_start(const i18n::phonenumbers::UnicodeString* this_ptr, int start, i18n::phonenumbers::UnicodeString* output);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_tempSubString_to_output_start_length(const i18n::phonenumbers::UnicodeString* this_ptr, int start, int length, i18n::phonenumbers::UnicodeString* output);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_toUTF8String(const i18n::phonenumbers::UnicodeString* this_ptr, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* out);

} // extern "C"

#endif // LIBPHONENUMBER_SYS_C_UNICODESTRING_H
