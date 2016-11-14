#ifndef LIBPHONENUMBER_SYS_C_UNICODETEXT_H
#define LIBPHONENUMBER_SYS_C_UNICODETEXT_H

#include "libphonenumber_sys_c_global.h"

extern "C" {

LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::UnicodeText* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_Copy(i18n::phonenumbers::UnicodeText* this_ptr, const i18n::phonenumbers::UnicodeText* src);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::UnicodeText* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_CopyUTF8(i18n::phonenumbers::UnicodeText* this_ptr, const char* utf8_buffer, int byte_length);
LIBPHONENUMBER_SYS_C_EXPORT std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_DebugString_as_ptr(const i18n::phonenumbers::UnicodeText* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_DebugString_to_output(const i18n::phonenumbers::UnicodeText* this_ptr, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* output);
LIBPHONENUMBER_SYS_C_EXPORT bool libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_HasReplacementChar(const i18n::phonenumbers::UnicodeText* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::UnicodeText::const_iterator* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_MakeIterator_as_ptr(const i18n::phonenumbers::UnicodeText* this_ptr, const char* p);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_MakeIterator_to_output(const i18n::phonenumbers::UnicodeText* this_ptr, const char* p, i18n::phonenumbers::UnicodeText::const_iterator* output);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::UnicodeText* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_PointToUTF8(i18n::phonenumbers::UnicodeText* this_ptr, const char* utf8_buffer, int byte_length);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::UnicodeText* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_PointTo_first_last(i18n::phonenumbers::UnicodeText* this_ptr, const i18n::phonenumbers::UnicodeText::const_iterator* first, const i18n::phonenumbers::UnicodeText::const_iterator* last);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::UnicodeText* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_PointTo_src(i18n::phonenumbers::UnicodeText* this_ptr, const i18n::phonenumbers::UnicodeText* src);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::UnicodeText* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_TakeOwnershipOfUTF8(i18n::phonenumbers::UnicodeText* this_ptr, char* utf8_buffer, int byte_length, int byte_capacity);
LIBPHONENUMBER_SYS_C_EXPORT std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_UTF8Substring_as_ptr(const i18n::phonenumbers::UnicodeText::const_iterator* first, const i18n::phonenumbers::UnicodeText::const_iterator* last);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_UTF8Substring_to_output(const i18n::phonenumbers::UnicodeText::const_iterator* first, const i18n::phonenumbers::UnicodeText::const_iterator* last, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* output);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::UnicodeText* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_append_first_last(i18n::phonenumbers::UnicodeText* this_ptr, const i18n::phonenumbers::UnicodeText::const_iterator* first, const i18n::phonenumbers::UnicodeText::const_iterator* last);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::UnicodeText* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_append_source(i18n::phonenumbers::UnicodeText* this_ptr, const i18n::phonenumbers::UnicodeText* source);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::UnicodeText* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_assign(i18n::phonenumbers::UnicodeText* this_ptr, const i18n::phonenumbers::UnicodeText* src);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::UnicodeText::const_iterator* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_begin_as_ptr(const i18n::phonenumbers::UnicodeText* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_begin_to_output(const i18n::phonenumbers::UnicodeText* this_ptr, i18n::phonenumbers::UnicodeText::const_iterator* output);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_clear(i18n::phonenumbers::UnicodeText* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_DebugString_as_ptr(const i18n::phonenumbers::UnicodeText::const_iterator* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_DebugString_to_output(const i18n::phonenumbers::UnicodeText::const_iterator* this_ptr, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* output);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_constructor_no_args(i18n::phonenumbers::UnicodeText::const_iterator* output);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_constructor_other(const i18n::phonenumbers::UnicodeText::const_iterator* other, i18n::phonenumbers::UnicodeText::const_iterator* output);
LIBPHONENUMBER_SYS_C_EXPORT int libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_get_utf8(const i18n::phonenumbers::UnicodeText::const_iterator* this_ptr, char* buf);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::UnicodeText::const_iterator* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_new_no_args();
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::UnicodeText::const_iterator* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_new_other(const i18n::phonenumbers::UnicodeText::const_iterator* other);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::UnicodeText::const_iterator* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_operator_assign(i18n::phonenumbers::UnicodeText::const_iterator* this_ptr, const i18n::phonenumbers::UnicodeText::const_iterator* other);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::UnicodeText::const_iterator* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_operator_dec(i18n::phonenumbers::UnicodeText::const_iterator* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::UnicodeText::const_iterator* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_operator_dec_postfix_as_ptr(i18n::phonenumbers::UnicodeText::const_iterator* this_ptr, int arg1);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_operator_dec_postfix_to_output(i18n::phonenumbers::UnicodeText::const_iterator* this_ptr, int arg1, i18n::phonenumbers::UnicodeText::const_iterator* output);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::UnicodeText::const_iterator* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_operator_inc(i18n::phonenumbers::UnicodeText::const_iterator* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::UnicodeText::const_iterator* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_operator_inc_postfix_as_ptr(i18n::phonenumbers::UnicodeText::const_iterator* this_ptr, int arg1);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_operator_inc_postfix_to_output(i18n::phonenumbers::UnicodeText::const_iterator* this_ptr, int arg1, i18n::phonenumbers::UnicodeText::const_iterator* output);
LIBPHONENUMBER_SYS_C_EXPORT int libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_operator_indirection(const i18n::phonenumbers::UnicodeText::const_iterator* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT const char* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_utf8_data(const i18n::phonenumbers::UnicodeText::const_iterator* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_reverse_iterator_constructor(const i18n::phonenumbers::UnicodeText::const_iterator* it, i18n::phonenumbers::UnicodeText::const_reverse_iterator* output);
LIBPHONENUMBER_SYS_C_EXPORT int libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_reverse_iterator_get_utf8(const i18n::phonenumbers::UnicodeText::const_reverse_iterator* this_ptr, char* buf);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::UnicodeText::const_reverse_iterator* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_reverse_iterator_new(const i18n::phonenumbers::UnicodeText::const_iterator* it);
LIBPHONENUMBER_SYS_C_EXPORT const char* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_reverse_iterator_utf8_data(const i18n::phonenumbers::UnicodeText::const_reverse_iterator* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_constructor_first_last(const i18n::phonenumbers::UnicodeText::const_iterator* first, const i18n::phonenumbers::UnicodeText::const_iterator* last, i18n::phonenumbers::UnicodeText* output);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_constructor_no_args(i18n::phonenumbers::UnicodeText* output);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_constructor_src(const i18n::phonenumbers::UnicodeText* src, i18n::phonenumbers::UnicodeText* output);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_delete(i18n::phonenumbers::UnicodeText* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_destructor(i18n::phonenumbers::UnicodeText* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT bool libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_empty(i18n::phonenumbers::UnicodeText* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::UnicodeText::const_iterator* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_end_as_ptr(const i18n::phonenumbers::UnicodeText* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_end_to_output(const i18n::phonenumbers::UnicodeText* this_ptr, i18n::phonenumbers::UnicodeText::const_iterator* output);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::UnicodeText::const_iterator* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_find_as_ptr_look(const i18n::phonenumbers::UnicodeText* this_ptr, const i18n::phonenumbers::UnicodeText* look);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::UnicodeText::const_iterator* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_find_as_ptr_look_start_pos(const i18n::phonenumbers::UnicodeText* this_ptr, const i18n::phonenumbers::UnicodeText* look, const i18n::phonenumbers::UnicodeText::const_iterator* start_pos);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_find_to_output_look(const i18n::phonenumbers::UnicodeText* this_ptr, const i18n::phonenumbers::UnicodeText* look, i18n::phonenumbers::UnicodeText::const_iterator* output);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_find_to_output_look_start_pos(const i18n::phonenumbers::UnicodeText* this_ptr, const i18n::phonenumbers::UnicodeText* look, const i18n::phonenumbers::UnicodeText::const_iterator* start_pos, i18n::phonenumbers::UnicodeText::const_iterator* output);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::UnicodeText* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_new_first_last(const i18n::phonenumbers::UnicodeText::const_iterator* first, const i18n::phonenumbers::UnicodeText::const_iterator* last);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::UnicodeText* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_new_no_args();
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::UnicodeText* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_new_src(const i18n::phonenumbers::UnicodeText* src);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::UnicodeText* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_operator_assign(i18n::phonenumbers::UnicodeText* this_ptr, const i18n::phonenumbers::UnicodeText* src);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_push_back(i18n::phonenumbers::UnicodeText* this_ptr, int codepoint);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::UnicodeText::const_reverse_iterator* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_rbegin_as_ptr(const i18n::phonenumbers::UnicodeText* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_rbegin_to_output(const i18n::phonenumbers::UnicodeText* this_ptr, i18n::phonenumbers::UnicodeText::const_reverse_iterator* output);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::UnicodeText::const_reverse_iterator* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_rend_as_ptr(const i18n::phonenumbers::UnicodeText* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_rend_to_output(const i18n::phonenumbers::UnicodeText* this_ptr, i18n::phonenumbers::UnicodeText::const_reverse_iterator* output);
LIBPHONENUMBER_SYS_C_EXPORT int libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_size(const i18n::phonenumbers::UnicodeText* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT int libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_utf8_capacity(const i18n::phonenumbers::UnicodeText* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT const char* libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_utf8_data(const i18n::phonenumbers::UnicodeText* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT int libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_utf8_length(const i18n::phonenumbers::UnicodeText* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::UnicodeText* libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_MakeUnicodeTextAcceptingOwnership_as_ptr(char* utf8_buffer, int byte_length, int byte_capacity);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_MakeUnicodeTextAcceptingOwnership_to_output(char* utf8_buffer, int byte_length, int byte_capacity, i18n::phonenumbers::UnicodeText* output);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::UnicodeText* libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_MakeUnicodeTextWithoutAcceptingOwnership_as_ptr(const char* utf8_buffer, int byte_length);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_MakeUnicodeTextWithoutAcceptingOwnership_to_output(const char* utf8_buffer, int byte_length, i18n::phonenumbers::UnicodeText* output);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::UnicodeText* libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UTF8ToUnicodeText_as_ptr_utf8_buf_len(const char* utf8_buf, int len);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::UnicodeText* libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UTF8ToUnicodeText_as_ptr_utf8_buf_len_do_copy(const char* utf8_buf, int len, bool do_copy);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::UnicodeText* libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UTF8ToUnicodeText_as_ptr_utf8_string(const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* utf8_string);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::UnicodeText* libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UTF8ToUnicodeText_as_ptr_utf_string_do_copy(const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* utf_string, bool do_copy);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UTF8ToUnicodeText_to_output_utf8_buf_len(const char* utf8_buf, int len, i18n::phonenumbers::UnicodeText* output);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UTF8ToUnicodeText_to_output_utf8_buf_len_do_copy(const char* utf8_buf, int len, bool do_copy, i18n::phonenumbers::UnicodeText* output);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UTF8ToUnicodeText_to_output_utf8_string(const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* utf8_string, i18n::phonenumbers::UnicodeText* output);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UTF8ToUnicodeText_to_output_utf_string_do_copy(const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* utf_string, bool do_copy, i18n::phonenumbers::UnicodeText* output);
LIBPHONENUMBER_SYS_C_EXPORT bool libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UnicodeTextRangeIsEmpty(const std::pair< i18n::phonenumbers::UnicodeText::const_iterator, i18n::phonenumbers::UnicodeText::const_iterator >* r);
LIBPHONENUMBER_SYS_C_EXPORT std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UnicodeTextToUTF8_as_ptr(const i18n::phonenumbers::UnicodeText* t);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UnicodeTextToUTF8_to_output(const i18n::phonenumbers::UnicodeText* t, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* output);

} // extern "C"

#endif // LIBPHONENUMBER_SYS_C_UNICODETEXT_H
