#include "libphonenumber_sys_c_regexp_cache.h"

const i18n::phonenumbers::RegExp* libphonenumber_sys_c_i18n_phonenumbers_RegExpCache_GetRegExp(i18n::phonenumbers::RegExpCache* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* pattern) {
  return &this_ptr->GetRegExp(*pattern);
}

void libphonenumber_sys_c_i18n_phonenumbers_RegExpCache_constructor(const i18n::phonenumbers::AbstractRegExpFactory* regexp_factory, unsigned long min_items, i18n::phonenumbers::RegExpCache* output) {
  new(output) i18n::phonenumbers::RegExpCache(*regexp_factory, min_items);
}

void libphonenumber_sys_c_i18n_phonenumbers_RegExpCache_delete(i18n::phonenumbers::RegExpCache* this_ptr) {
  delete this_ptr;
}

void libphonenumber_sys_c_i18n_phonenumbers_RegExpCache_destructor(i18n::phonenumbers::RegExpCache* this_ptr) {
  libphonenumber_sys_c_call_destructor(this_ptr);
}

i18n::phonenumbers::RegExpCache* libphonenumber_sys_c_i18n_phonenumbers_RegExpCache_new(const i18n::phonenumbers::AbstractRegExpFactory* regexp_factory, unsigned long min_items) {
  return new i18n::phonenumbers::RegExpCache(*regexp_factory, min_items);
}

