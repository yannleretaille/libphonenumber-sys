#include "libphonenumber_sys_c_unicodestring.h"

void libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_append_codepoint(i18n::phonenumbers::UnicodeString* this_ptr, int codepoint) {
  this_ptr->append(codepoint);
}

void libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_append_unicode_string(i18n::phonenumbers::UnicodeString* this_ptr, const i18n::phonenumbers::UnicodeString* unicode_string) {
  this_ptr->append(*unicode_string);
}

i18n::phonenumbers::UnicodeText::const_iterator* libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_begin_as_ptr(const i18n::phonenumbers::UnicodeString* this_ptr) {
  return new i18n::phonenumbers::UnicodeText::const_iterator(this_ptr->begin());
}

void libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_begin_to_output(const i18n::phonenumbers::UnicodeString* this_ptr, i18n::phonenumbers::UnicodeText::const_iterator* output) {
  new(output) i18n::phonenumbers::UnicodeText::const_iterator(this_ptr->begin());
}

void libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_constructor_codepoint(int codepoint, i18n::phonenumbers::UnicodeString* output) {
  new(output) i18n::phonenumbers::UnicodeString(codepoint);
}

void libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_constructor_no_args(i18n::phonenumbers::UnicodeString* output) {
  new(output) i18n::phonenumbers::UnicodeString();
}

void libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_constructor_src(const i18n::phonenumbers::UnicodeString* src, i18n::phonenumbers::UnicodeString* output) {
  new(output) i18n::phonenumbers::UnicodeString(*src);
}

void libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_constructor_utf8(const char* utf8, i18n::phonenumbers::UnicodeString* output) {
  new(output) i18n::phonenumbers::UnicodeString(utf8);
}

i18n::phonenumbers::UnicodeText::const_iterator* libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_end_as_ptr(const i18n::phonenumbers::UnicodeString* this_ptr) {
  return new i18n::phonenumbers::UnicodeText::const_iterator(this_ptr->end());
}

void libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_end_to_output(const i18n::phonenumbers::UnicodeString* this_ptr, i18n::phonenumbers::UnicodeText::const_iterator* output) {
  new(output) i18n::phonenumbers::UnicodeText::const_iterator(this_ptr->end());
}

int libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_indexOf(const i18n::phonenumbers::UnicodeString* this_ptr, int codepoint) {
  return this_ptr->indexOf(codepoint);
}

int libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_length(const i18n::phonenumbers::UnicodeString* this_ptr) {
  return this_ptr->length();
}

i18n::phonenumbers::UnicodeString* libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_new_codepoint(int codepoint) {
  return new i18n::phonenumbers::UnicodeString(codepoint);
}

i18n::phonenumbers::UnicodeString* libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_new_no_args() {
  return new i18n::phonenumbers::UnicodeString();
}

i18n::phonenumbers::UnicodeString* libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_new_src(const i18n::phonenumbers::UnicodeString* src) {
  return new i18n::phonenumbers::UnicodeString(*src);
}

i18n::phonenumbers::UnicodeString* libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_new_utf8(const char* utf8) {
  return new i18n::phonenumbers::UnicodeString(utf8);
}

i18n::phonenumbers::UnicodeString* libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_operator_assign(i18n::phonenumbers::UnicodeString* this_ptr, const i18n::phonenumbers::UnicodeString* src) {
  return &this_ptr->operator=(*src);
}

bool libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_operator_eq(const i18n::phonenumbers::UnicodeString* this_ptr, const i18n::phonenumbers::UnicodeString* rhs) {
  return this_ptr->operator==(*rhs);
}

int libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_operator_index(const i18n::phonenumbers::UnicodeString* this_ptr, int index) {
  return this_ptr->operator[](index);
}

void libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_remove(i18n::phonenumbers::UnicodeString* this_ptr) {
  this_ptr->remove();
}

void libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_replace(i18n::phonenumbers::UnicodeString* this_ptr, int start, int length, const i18n::phonenumbers::UnicodeString* src) {
  this_ptr->replace(start, length, *src);
}

void libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_setCharAt(i18n::phonenumbers::UnicodeString* this_ptr, int pos, int c) {
  this_ptr->setCharAt(pos, c);
}

void libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_setTo(i18n::phonenumbers::UnicodeString* this_ptr, const char* s, unsigned long len) {
  this_ptr->setTo(s, len);
}

i18n::phonenumbers::UnicodeString* libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_tempSubString_as_ptr_start(const i18n::phonenumbers::UnicodeString* this_ptr, int start) {
  return new i18n::phonenumbers::UnicodeString(this_ptr->tempSubString(start));
}

i18n::phonenumbers::UnicodeString* libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_tempSubString_as_ptr_start_length(const i18n::phonenumbers::UnicodeString* this_ptr, int start, int length) {
  return new i18n::phonenumbers::UnicodeString(this_ptr->tempSubString(start, length));
}

void libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_tempSubString_to_output_start(const i18n::phonenumbers::UnicodeString* this_ptr, int start, i18n::phonenumbers::UnicodeString* output) {
  new(output) i18n::phonenumbers::UnicodeString(this_ptr->tempSubString(start));
}

void libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_tempSubString_to_output_start_length(const i18n::phonenumbers::UnicodeString* this_ptr, int start, int length, i18n::phonenumbers::UnicodeString* output) {
  new(output) i18n::phonenumbers::UnicodeString(this_ptr->tempSubString(start, length));
}

void libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_toUTF8String(const i18n::phonenumbers::UnicodeString* this_ptr, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* out) {
  this_ptr->toUTF8String(*out);
}

