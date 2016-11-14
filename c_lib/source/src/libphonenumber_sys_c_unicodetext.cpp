#include "libphonenumber_sys_c_unicodetext.h"

i18n::phonenumbers::UnicodeText* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_Copy(i18n::phonenumbers::UnicodeText* this_ptr, const i18n::phonenumbers::UnicodeText* src) {
  return &this_ptr->Copy(*src);
}

i18n::phonenumbers::UnicodeText* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_CopyUTF8(i18n::phonenumbers::UnicodeText* this_ptr, const char* utf8_buffer, int byte_length) {
  return &this_ptr->CopyUTF8(utf8_buffer, byte_length);
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_DebugString_as_ptr(const i18n::phonenumbers::UnicodeText* this_ptr) {
  return new std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >(this_ptr->DebugString());
}

void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_DebugString_to_output(const i18n::phonenumbers::UnicodeText* this_ptr, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* output) {
  new(output) std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >(this_ptr->DebugString());
}

bool libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_HasReplacementChar(const i18n::phonenumbers::UnicodeText* this_ptr) {
  return this_ptr->HasReplacementChar();
}

i18n::phonenumbers::UnicodeText::const_iterator* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_MakeIterator_as_ptr(const i18n::phonenumbers::UnicodeText* this_ptr, const char* p) {
  return new i18n::phonenumbers::UnicodeText::const_iterator(this_ptr->MakeIterator(p));
}

void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_MakeIterator_to_output(const i18n::phonenumbers::UnicodeText* this_ptr, const char* p, i18n::phonenumbers::UnicodeText::const_iterator* output) {
  new(output) i18n::phonenumbers::UnicodeText::const_iterator(this_ptr->MakeIterator(p));
}

i18n::phonenumbers::UnicodeText* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_PointToUTF8(i18n::phonenumbers::UnicodeText* this_ptr, const char* utf8_buffer, int byte_length) {
  return &this_ptr->PointToUTF8(utf8_buffer, byte_length);
}

i18n::phonenumbers::UnicodeText* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_PointTo_first_last(i18n::phonenumbers::UnicodeText* this_ptr, const i18n::phonenumbers::UnicodeText::const_iterator* first, const i18n::phonenumbers::UnicodeText::const_iterator* last) {
  return &this_ptr->PointTo(*first, *last);
}

i18n::phonenumbers::UnicodeText* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_PointTo_src(i18n::phonenumbers::UnicodeText* this_ptr, const i18n::phonenumbers::UnicodeText* src) {
  return &this_ptr->PointTo(*src);
}

i18n::phonenumbers::UnicodeText* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_TakeOwnershipOfUTF8(i18n::phonenumbers::UnicodeText* this_ptr, char* utf8_buffer, int byte_length, int byte_capacity) {
  return &this_ptr->TakeOwnershipOfUTF8(utf8_buffer, byte_length, byte_capacity);
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_UTF8Substring_as_ptr(const i18n::phonenumbers::UnicodeText::const_iterator* first, const i18n::phonenumbers::UnicodeText::const_iterator* last) {
  return new std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >(i18n::phonenumbers::UnicodeText::UTF8Substring(*first, *last));
}

void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_UTF8Substring_to_output(const i18n::phonenumbers::UnicodeText::const_iterator* first, const i18n::phonenumbers::UnicodeText::const_iterator* last, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* output) {
  new(output) std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >(i18n::phonenumbers::UnicodeText::UTF8Substring(*first, *last));
}

i18n::phonenumbers::UnicodeText* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_append_first_last(i18n::phonenumbers::UnicodeText* this_ptr, const i18n::phonenumbers::UnicodeText::const_iterator* first, const i18n::phonenumbers::UnicodeText::const_iterator* last) {
  return &this_ptr->append(*first, *last);
}

i18n::phonenumbers::UnicodeText* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_append_source(i18n::phonenumbers::UnicodeText* this_ptr, const i18n::phonenumbers::UnicodeText* source) {
  return &this_ptr->append(*source);
}

i18n::phonenumbers::UnicodeText* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_assign(i18n::phonenumbers::UnicodeText* this_ptr, const i18n::phonenumbers::UnicodeText* src) {
  return &this_ptr->assign(*src);
}

i18n::phonenumbers::UnicodeText::const_iterator* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_begin_as_ptr(const i18n::phonenumbers::UnicodeText* this_ptr) {
  return new i18n::phonenumbers::UnicodeText::const_iterator(this_ptr->begin());
}

void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_begin_to_output(const i18n::phonenumbers::UnicodeText* this_ptr, i18n::phonenumbers::UnicodeText::const_iterator* output) {
  new(output) i18n::phonenumbers::UnicodeText::const_iterator(this_ptr->begin());
}

void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_clear(i18n::phonenumbers::UnicodeText* this_ptr) {
  this_ptr->clear();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_DebugString_as_ptr(const i18n::phonenumbers::UnicodeText::const_iterator* this_ptr) {
  return new std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >(this_ptr->DebugString());
}

void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_DebugString_to_output(const i18n::phonenumbers::UnicodeText::const_iterator* this_ptr, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* output) {
  new(output) std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >(this_ptr->DebugString());
}

void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_constructor_no_args(i18n::phonenumbers::UnicodeText::const_iterator* output) {
  new(output) i18n::phonenumbers::UnicodeText::const_iterator();
}

void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_constructor_other(const i18n::phonenumbers::UnicodeText::const_iterator* other, i18n::phonenumbers::UnicodeText::const_iterator* output) {
  new(output) i18n::phonenumbers::UnicodeText::const_iterator(*other);
}

int libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_get_utf8(const i18n::phonenumbers::UnicodeText::const_iterator* this_ptr, char* buf) {
  return this_ptr->get_utf8(buf);
}

i18n::phonenumbers::UnicodeText::const_iterator* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_new_no_args() {
  return new i18n::phonenumbers::UnicodeText::const_iterator();
}

i18n::phonenumbers::UnicodeText::const_iterator* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_new_other(const i18n::phonenumbers::UnicodeText::const_iterator* other) {
  return new i18n::phonenumbers::UnicodeText::const_iterator(*other);
}

i18n::phonenumbers::UnicodeText::const_iterator* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_operator_assign(i18n::phonenumbers::UnicodeText::const_iterator* this_ptr, const i18n::phonenumbers::UnicodeText::const_iterator* other) {
  return &this_ptr->operator=(*other);
}

i18n::phonenumbers::UnicodeText::const_iterator* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_operator_dec(i18n::phonenumbers::UnicodeText::const_iterator* this_ptr) {
  return &this_ptr->operator--();
}

i18n::phonenumbers::UnicodeText::const_iterator* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_operator_dec_postfix_as_ptr(i18n::phonenumbers::UnicodeText::const_iterator* this_ptr, int arg1) {
  return new i18n::phonenumbers::UnicodeText::const_iterator(this_ptr->operator--(arg1));
}

void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_operator_dec_postfix_to_output(i18n::phonenumbers::UnicodeText::const_iterator* this_ptr, int arg1, i18n::phonenumbers::UnicodeText::const_iterator* output) {
  new(output) i18n::phonenumbers::UnicodeText::const_iterator(this_ptr->operator--(arg1));
}

i18n::phonenumbers::UnicodeText::const_iterator* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_operator_inc(i18n::phonenumbers::UnicodeText::const_iterator* this_ptr) {
  return &this_ptr->operator++();
}

i18n::phonenumbers::UnicodeText::const_iterator* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_operator_inc_postfix_as_ptr(i18n::phonenumbers::UnicodeText::const_iterator* this_ptr, int arg1) {
  return new i18n::phonenumbers::UnicodeText::const_iterator(this_ptr->operator++(arg1));
}

void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_operator_inc_postfix_to_output(i18n::phonenumbers::UnicodeText::const_iterator* this_ptr, int arg1, i18n::phonenumbers::UnicodeText::const_iterator* output) {
  new(output) i18n::phonenumbers::UnicodeText::const_iterator(this_ptr->operator++(arg1));
}

int libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_operator_indirection(const i18n::phonenumbers::UnicodeText::const_iterator* this_ptr) {
  return this_ptr->operator*();
}

const char* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_utf8_data(const i18n::phonenumbers::UnicodeText::const_iterator* this_ptr) {
  return this_ptr->utf8_data();
}

void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_reverse_iterator_constructor(const i18n::phonenumbers::UnicodeText::const_iterator* it, i18n::phonenumbers::UnicodeText::const_reverse_iterator* output) {
  new(output) i18n::phonenumbers::UnicodeText::const_reverse_iterator(*it);
}

int libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_reverse_iterator_get_utf8(const i18n::phonenumbers::UnicodeText::const_reverse_iterator* this_ptr, char* buf) {
  return this_ptr->get_utf8(buf);
}

i18n::phonenumbers::UnicodeText::const_reverse_iterator* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_reverse_iterator_new(const i18n::phonenumbers::UnicodeText::const_iterator* it) {
  return new i18n::phonenumbers::UnicodeText::const_reverse_iterator(*it);
}

const char* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_reverse_iterator_utf8_data(const i18n::phonenumbers::UnicodeText::const_reverse_iterator* this_ptr) {
  return this_ptr->utf8_data();
}

void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_constructor_first_last(const i18n::phonenumbers::UnicodeText::const_iterator* first, const i18n::phonenumbers::UnicodeText::const_iterator* last, i18n::phonenumbers::UnicodeText* output) {
  new(output) i18n::phonenumbers::UnicodeText(*first, *last);
}

void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_constructor_no_args(i18n::phonenumbers::UnicodeText* output) {
  new(output) i18n::phonenumbers::UnicodeText();
}

void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_constructor_src(const i18n::phonenumbers::UnicodeText* src, i18n::phonenumbers::UnicodeText* output) {
  new(output) i18n::phonenumbers::UnicodeText(*src);
}

void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_delete(i18n::phonenumbers::UnicodeText* this_ptr) {
  delete this_ptr;
}

void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_destructor(i18n::phonenumbers::UnicodeText* this_ptr) {
  libphonenumber_sys_c_call_destructor(this_ptr);
}

bool libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_empty(i18n::phonenumbers::UnicodeText* this_ptr) {
  return this_ptr->empty();
}

i18n::phonenumbers::UnicodeText::const_iterator* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_end_as_ptr(const i18n::phonenumbers::UnicodeText* this_ptr) {
  return new i18n::phonenumbers::UnicodeText::const_iterator(this_ptr->end());
}

void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_end_to_output(const i18n::phonenumbers::UnicodeText* this_ptr, i18n::phonenumbers::UnicodeText::const_iterator* output) {
  new(output) i18n::phonenumbers::UnicodeText::const_iterator(this_ptr->end());
}

i18n::phonenumbers::UnicodeText::const_iterator* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_find_as_ptr_look(const i18n::phonenumbers::UnicodeText* this_ptr, const i18n::phonenumbers::UnicodeText* look) {
  return new i18n::phonenumbers::UnicodeText::const_iterator(this_ptr->find(*look));
}

i18n::phonenumbers::UnicodeText::const_iterator* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_find_as_ptr_look_start_pos(const i18n::phonenumbers::UnicodeText* this_ptr, const i18n::phonenumbers::UnicodeText* look, const i18n::phonenumbers::UnicodeText::const_iterator* start_pos) {
  return new i18n::phonenumbers::UnicodeText::const_iterator(this_ptr->find(*look, *start_pos));
}

void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_find_to_output_look(const i18n::phonenumbers::UnicodeText* this_ptr, const i18n::phonenumbers::UnicodeText* look, i18n::phonenumbers::UnicodeText::const_iterator* output) {
  new(output) i18n::phonenumbers::UnicodeText::const_iterator(this_ptr->find(*look));
}

void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_find_to_output_look_start_pos(const i18n::phonenumbers::UnicodeText* this_ptr, const i18n::phonenumbers::UnicodeText* look, const i18n::phonenumbers::UnicodeText::const_iterator* start_pos, i18n::phonenumbers::UnicodeText::const_iterator* output) {
  new(output) i18n::phonenumbers::UnicodeText::const_iterator(this_ptr->find(*look, *start_pos));
}

i18n::phonenumbers::UnicodeText* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_new_first_last(const i18n::phonenumbers::UnicodeText::const_iterator* first, const i18n::phonenumbers::UnicodeText::const_iterator* last) {
  return new i18n::phonenumbers::UnicodeText(*first, *last);
}

i18n::phonenumbers::UnicodeText* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_new_no_args() {
  return new i18n::phonenumbers::UnicodeText();
}

i18n::phonenumbers::UnicodeText* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_new_src(const i18n::phonenumbers::UnicodeText* src) {
  return new i18n::phonenumbers::UnicodeText(*src);
}

i18n::phonenumbers::UnicodeText* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_operator_assign(i18n::phonenumbers::UnicodeText* this_ptr, const i18n::phonenumbers::UnicodeText* src) {
  return &this_ptr->operator=(*src);
}

void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_push_back(i18n::phonenumbers::UnicodeText* this_ptr, int codepoint) {
  this_ptr->push_back(codepoint);
}

i18n::phonenumbers::UnicodeText::const_reverse_iterator* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_rbegin_as_ptr(const i18n::phonenumbers::UnicodeText* this_ptr) {
  return new i18n::phonenumbers::UnicodeText::const_reverse_iterator(this_ptr->rbegin());
}

void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_rbegin_to_output(const i18n::phonenumbers::UnicodeText* this_ptr, i18n::phonenumbers::UnicodeText::const_reverse_iterator* output) {
  new(output) i18n::phonenumbers::UnicodeText::const_reverse_iterator(this_ptr->rbegin());
}

i18n::phonenumbers::UnicodeText::const_reverse_iterator* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_rend_as_ptr(const i18n::phonenumbers::UnicodeText* this_ptr) {
  return new i18n::phonenumbers::UnicodeText::const_reverse_iterator(this_ptr->rend());
}

void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_rend_to_output(const i18n::phonenumbers::UnicodeText* this_ptr, i18n::phonenumbers::UnicodeText::const_reverse_iterator* output) {
  new(output) i18n::phonenumbers::UnicodeText::const_reverse_iterator(this_ptr->rend());
}

int libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_size(const i18n::phonenumbers::UnicodeText* this_ptr) {
  return this_ptr->size();
}

int libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_utf8_capacity(const i18n::phonenumbers::UnicodeText* this_ptr) {
  return this_ptr->utf8_capacity();
}

const char* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_utf8_data(const i18n::phonenumbers::UnicodeText* this_ptr) {
  return this_ptr->utf8_data();
}

int libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_utf8_length(const i18n::phonenumbers::UnicodeText* this_ptr) {
  return this_ptr->utf8_length();
}

i18n::phonenumbers::UnicodeText* libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_MakeUnicodeTextAcceptingOwnership_as_ptr(char* utf8_buffer, int byte_length, int byte_capacity) {
  return new i18n::phonenumbers::UnicodeText(i18n::phonenumbers::MakeUnicodeTextAcceptingOwnership(utf8_buffer, byte_length, byte_capacity));
}

void libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_MakeUnicodeTextAcceptingOwnership_to_output(char* utf8_buffer, int byte_length, int byte_capacity, i18n::phonenumbers::UnicodeText* output) {
  new(output) i18n::phonenumbers::UnicodeText(i18n::phonenumbers::MakeUnicodeTextAcceptingOwnership(utf8_buffer, byte_length, byte_capacity));
}

i18n::phonenumbers::UnicodeText* libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_MakeUnicodeTextWithoutAcceptingOwnership_as_ptr(const char* utf8_buffer, int byte_length) {
  return new i18n::phonenumbers::UnicodeText(i18n::phonenumbers::MakeUnicodeTextWithoutAcceptingOwnership(utf8_buffer, byte_length));
}

void libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_MakeUnicodeTextWithoutAcceptingOwnership_to_output(const char* utf8_buffer, int byte_length, i18n::phonenumbers::UnicodeText* output) {
  new(output) i18n::phonenumbers::UnicodeText(i18n::phonenumbers::MakeUnicodeTextWithoutAcceptingOwnership(utf8_buffer, byte_length));
}

i18n::phonenumbers::UnicodeText* libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UTF8ToUnicodeText_as_ptr_utf8_buf_len(const char* utf8_buf, int len) {
  return new i18n::phonenumbers::UnicodeText(i18n::phonenumbers::UTF8ToUnicodeText(utf8_buf, len));
}

i18n::phonenumbers::UnicodeText* libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UTF8ToUnicodeText_as_ptr_utf8_buf_len_do_copy(const char* utf8_buf, int len, bool do_copy) {
  return new i18n::phonenumbers::UnicodeText(i18n::phonenumbers::UTF8ToUnicodeText(utf8_buf, len, do_copy));
}

i18n::phonenumbers::UnicodeText* libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UTF8ToUnicodeText_as_ptr_utf8_string(const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* utf8_string) {
  return new i18n::phonenumbers::UnicodeText(i18n::phonenumbers::UTF8ToUnicodeText(*utf8_string));
}

i18n::phonenumbers::UnicodeText* libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UTF8ToUnicodeText_as_ptr_utf_string_do_copy(const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* utf_string, bool do_copy) {
  return new i18n::phonenumbers::UnicodeText(i18n::phonenumbers::UTF8ToUnicodeText(*utf_string, do_copy));
}

void libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UTF8ToUnicodeText_to_output_utf8_buf_len(const char* utf8_buf, int len, i18n::phonenumbers::UnicodeText* output) {
  new(output) i18n::phonenumbers::UnicodeText(i18n::phonenumbers::UTF8ToUnicodeText(utf8_buf, len));
}

void libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UTF8ToUnicodeText_to_output_utf8_buf_len_do_copy(const char* utf8_buf, int len, bool do_copy, i18n::phonenumbers::UnicodeText* output) {
  new(output) i18n::phonenumbers::UnicodeText(i18n::phonenumbers::UTF8ToUnicodeText(utf8_buf, len, do_copy));
}

void libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UTF8ToUnicodeText_to_output_utf8_string(const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* utf8_string, i18n::phonenumbers::UnicodeText* output) {
  new(output) i18n::phonenumbers::UnicodeText(i18n::phonenumbers::UTF8ToUnicodeText(*utf8_string));
}

void libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UTF8ToUnicodeText_to_output_utf_string_do_copy(const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* utf_string, bool do_copy, i18n::phonenumbers::UnicodeText* output) {
  new(output) i18n::phonenumbers::UnicodeText(i18n::phonenumbers::UTF8ToUnicodeText(*utf_string, do_copy));
}

bool libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UnicodeTextRangeIsEmpty(const std::pair< i18n::phonenumbers::UnicodeText::const_iterator, i18n::phonenumbers::UnicodeText::const_iterator >* r) {
  return i18n::phonenumbers::UnicodeTextRangeIsEmpty(*r);
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UnicodeTextToUTF8_as_ptr(const i18n::phonenumbers::UnicodeText* t) {
  return new std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >(i18n::phonenumbers::UnicodeTextToUTF8(*t));
}

void libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UnicodeTextToUTF8_to_output(const i18n::phonenumbers::UnicodeText* t, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* output) {
  new(output) std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >(i18n::phonenumbers::UnicodeTextToUTF8(*t));
}

