#include "libphonenumber_sys_c_phonenumbermatch.h"

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_CopyFrom(i18n::phonenumbers::PhoneNumberMatch* this_ptr, const i18n::phonenumbers::PhoneNumberMatch* number) {
  this_ptr->CopyFrom(*number);
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_Equals(const i18n::phonenumbers::PhoneNumberMatch* this_ptr, const i18n::phonenumbers::PhoneNumberMatch* number) {
  return this_ptr->Equals(*number);
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_ToString_as_ptr(const i18n::phonenumbers::PhoneNumberMatch* this_ptr) {
  return new std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >(this_ptr->ToString());
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_ToString_to_output(const i18n::phonenumbers::PhoneNumberMatch* this_ptr, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* output) {
  new(output) std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >(this_ptr->ToString());
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_constructor_no_args(i18n::phonenumbers::PhoneNumberMatch* output) {
  new(output) i18n::phonenumbers::PhoneNumberMatch();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_constructor_start_raw_string_number(int start, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* raw_string, const i18n::phonenumbers::PhoneNumber* number, i18n::phonenumbers::PhoneNumberMatch* output) {
  new(output) i18n::phonenumbers::PhoneNumberMatch(start, *raw_string, *number);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_delete(i18n::phonenumbers::PhoneNumberMatch* this_ptr) {
  delete this_ptr;
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_destructor(i18n::phonenumbers::PhoneNumberMatch* this_ptr) {
  libphonenumber_sys_c_call_destructor(this_ptr);
}

int libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_end(const i18n::phonenumbers::PhoneNumberMatch* this_ptr) {
  return this_ptr->end();
}

int libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_length(const i18n::phonenumbers::PhoneNumberMatch* this_ptr) {
  return this_ptr->length();
}

i18n::phonenumbers::PhoneNumberMatch* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_new_no_args() {
  return new i18n::phonenumbers::PhoneNumberMatch();
}

i18n::phonenumbers::PhoneNumberMatch* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_new_start_raw_string_number(int start, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* raw_string, const i18n::phonenumbers::PhoneNumber* number) {
  return new i18n::phonenumbers::PhoneNumberMatch(start, *raw_string, *number);
}

const i18n::phonenumbers::PhoneNumber* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_number(const i18n::phonenumbers::PhoneNumberMatch* this_ptr) {
  return &this_ptr->number();
}

const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_raw_string(const i18n::phonenumbers::PhoneNumberMatch* this_ptr) {
  return &this_ptr->raw_string();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_set_number(i18n::phonenumbers::PhoneNumberMatch* this_ptr, const i18n::phonenumbers::PhoneNumber* number) {
  this_ptr->set_number(*number);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_set_raw_string(i18n::phonenumbers::PhoneNumberMatch* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* raw_string) {
  this_ptr->set_raw_string(*raw_string);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_set_start(i18n::phonenumbers::PhoneNumberMatch* this_ptr, int start) {
  this_ptr->set_start(start);
}

int libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_start(const i18n::phonenumbers::PhoneNumberMatch* this_ptr) {
  return this_ptr->start();
}

