#ifndef LIBPHONENUMBER_SYS_C_REGEXP_CACHE_H
#define LIBPHONENUMBER_SYS_C_REGEXP_CACHE_H

#include "libphonenumber_sys_c_global.h"

extern "C" {

LIBPHONENUMBER_SYS_C_EXPORT const i18n::phonenumbers::RegExp* libphonenumber_sys_c_i18n_phonenumbers_RegExpCache_GetRegExp(i18n::phonenumbers::RegExpCache* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* pattern);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_RegExpCache_constructor(const i18n::phonenumbers::AbstractRegExpFactory* regexp_factory, unsigned long min_items, i18n::phonenumbers::RegExpCache* output);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_RegExpCache_delete(i18n::phonenumbers::RegExpCache* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_RegExpCache_destructor(i18n::phonenumbers::RegExpCache* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::RegExpCache* libphonenumber_sys_c_i18n_phonenumbers_RegExpCache_new(const i18n::phonenumbers::AbstractRegExpFactory* regexp_factory, unsigned long min_items);

} // extern "C"

#endif // LIBPHONENUMBER_SYS_C_REGEXP_CACHE_H
