#include "libphonenumber_sys_c_regexp_adapter.h"

i18n::phonenumbers::RegExpInput* libphonenumber_sys_c_i18n_phonenumbers_AbstractRegExpFactory_CreateInput(const i18n::phonenumbers::AbstractRegExpFactory* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* utf8_input) {
  return this_ptr->CreateInput(*utf8_input);
}

i18n::phonenumbers::RegExp* libphonenumber_sys_c_i18n_phonenumbers_AbstractRegExpFactory_CreateRegExp(const i18n::phonenumbers::AbstractRegExpFactory* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* utf8_regexp) {
  return this_ptr->CreateRegExp(*utf8_regexp);
}

void libphonenumber_sys_c_i18n_phonenumbers_AbstractRegExpFactory_delete(i18n::phonenumbers::AbstractRegExpFactory* this_ptr) {
  delete this_ptr;
}

void libphonenumber_sys_c_i18n_phonenumbers_AbstractRegExpFactory_destructor(i18n::phonenumbers::AbstractRegExpFactory* this_ptr) {
  libphonenumber_sys_c_call_destructor(this_ptr);
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_RegExpInput_ToString_as_ptr(const i18n::phonenumbers::RegExpInput* this_ptr) {
  return new std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >(this_ptr->ToString());
}

void libphonenumber_sys_c_i18n_phonenumbers_RegExpInput_ToString_to_output(const i18n::phonenumbers::RegExpInput* this_ptr, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* output) {
  new(output) std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >(this_ptr->ToString());
}

void libphonenumber_sys_c_i18n_phonenumbers_RegExpInput_delete(i18n::phonenumbers::RegExpInput* this_ptr) {
  delete this_ptr;
}

void libphonenumber_sys_c_i18n_phonenumbers_RegExpInput_destructor(i18n::phonenumbers::RegExpInput* this_ptr) {
  libphonenumber_sys_c_call_destructor(this_ptr);
}

bool libphonenumber_sys_c_i18n_phonenumbers_RegExp_Consume_input_string(const i18n::phonenumbers::RegExp* this_ptr, i18n::phonenumbers::RegExpInput* input_string) {
  return this_ptr->Consume(input_string);
}

bool libphonenumber_sys_c_i18n_phonenumbers_RegExp_Consume_input_string_anchor_at_start_matched_string1_matched_string2_matched_string3(const i18n::phonenumbers::RegExp* this_ptr, i18n::phonenumbers::RegExpInput* input_string, bool anchor_at_start, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* matched_string1, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* matched_string2, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* matched_string3) {
  return this_ptr->Consume(input_string, anchor_at_start, matched_string1, matched_string2, matched_string3);
}

bool libphonenumber_sys_c_i18n_phonenumbers_RegExp_Consume_input_string_matched_string(const i18n::phonenumbers::RegExp* this_ptr, i18n::phonenumbers::RegExpInput* input_string, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* matched_string) {
  return this_ptr->Consume(input_string, matched_string);
}

bool libphonenumber_sys_c_i18n_phonenumbers_RegExp_Consume_input_string_matched_string1_matched_string2(const i18n::phonenumbers::RegExp* this_ptr, i18n::phonenumbers::RegExpInput* input_string, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* matched_string1, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* matched_string2) {
  return this_ptr->Consume(input_string, matched_string1, matched_string2);
}

bool libphonenumber_sys_c_i18n_phonenumbers_RegExp_Consume_input_string_matched_string1_matched_string2_matched_string3(const i18n::phonenumbers::RegExp* this_ptr, i18n::phonenumbers::RegExpInput* input_string, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* matched_string1, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* matched_string2, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* matched_string3) {
  return this_ptr->Consume(input_string, matched_string1, matched_string2, matched_string3);
}

bool libphonenumber_sys_c_i18n_phonenumbers_RegExp_FindAndConsume(const i18n::phonenumbers::RegExp* this_ptr, i18n::phonenumbers::RegExpInput* input_string, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* matched_string) {
  return this_ptr->FindAndConsume(input_string, matched_string);
}

bool libphonenumber_sys_c_i18n_phonenumbers_RegExp_FullMatch_input_string(const i18n::phonenumbers::RegExp* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* input_string) {
  return this_ptr->FullMatch(*input_string);
}

bool libphonenumber_sys_c_i18n_phonenumbers_RegExp_FullMatch_input_string_matched_string(const i18n::phonenumbers::RegExp* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* input_string, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* matched_string) {
  return this_ptr->FullMatch(*input_string, matched_string);
}

bool libphonenumber_sys_c_i18n_phonenumbers_RegExp_GlobalReplace(const i18n::phonenumbers::RegExp* this_ptr, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* string_to_process, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* replacement_string) {
  return this_ptr->GlobalReplace(string_to_process, *replacement_string);
}

bool libphonenumber_sys_c_i18n_phonenumbers_RegExp_Match(const i18n::phonenumbers::RegExp* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* input_string, bool full_match, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* matched_string) {
  return this_ptr->Match(*input_string, full_match, matched_string);
}

bool libphonenumber_sys_c_i18n_phonenumbers_RegExp_PartialMatch_input_string(const i18n::phonenumbers::RegExp* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* input_string) {
  return this_ptr->PartialMatch(*input_string);
}

bool libphonenumber_sys_c_i18n_phonenumbers_RegExp_PartialMatch_input_string_matched_string(const i18n::phonenumbers::RegExp* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* input_string, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* matched_string) {
  return this_ptr->PartialMatch(*input_string, matched_string);
}

bool libphonenumber_sys_c_i18n_phonenumbers_RegExp_Replace_string_to_process_global_replacement_string(const i18n::phonenumbers::RegExp* this_ptr, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* string_to_process, bool global, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* replacement_string) {
  return this_ptr->Replace(string_to_process, global, *replacement_string);
}

bool libphonenumber_sys_c_i18n_phonenumbers_RegExp_Replace_string_to_process_replacement_string(const i18n::phonenumbers::RegExp* this_ptr, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* string_to_process, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* replacement_string) {
  return this_ptr->Replace(string_to_process, *replacement_string);
}

void libphonenumber_sys_c_i18n_phonenumbers_RegExp_delete(i18n::phonenumbers::RegExp* this_ptr) {
  delete this_ptr;
}

void libphonenumber_sys_c_i18n_phonenumbers_RegExp_destructor(i18n::phonenumbers::RegExp* this_ptr) {
  libphonenumber_sys_c_call_destructor(this_ptr);
}

