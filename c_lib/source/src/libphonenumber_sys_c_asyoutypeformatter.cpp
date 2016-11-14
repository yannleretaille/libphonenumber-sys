#include "libphonenumber_sys_c_asyoutypeformatter.h"

void libphonenumber_sys_c_i18n_phonenumbers_AsYouTypeFormatter_Clear(i18n::phonenumbers::AsYouTypeFormatter* this_ptr) {
  this_ptr->Clear();
}

int libphonenumber_sys_c_i18n_phonenumbers_AsYouTypeFormatter_GetRememberedPosition(const i18n::phonenumbers::AsYouTypeFormatter* this_ptr) {
  return this_ptr->GetRememberedPosition();
}

const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_AsYouTypeFormatter_InputDigit(i18n::phonenumbers::AsYouTypeFormatter* this_ptr, int next_char, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* result) {
  return &this_ptr->InputDigit(next_char, result);
}

const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_AsYouTypeFormatter_InputDigitAndRememberPosition(i18n::phonenumbers::AsYouTypeFormatter* this_ptr, int next_char, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* result) {
  return &this_ptr->InputDigitAndRememberPosition(next_char, result);
}

void libphonenumber_sys_c_i18n_phonenumbers_AsYouTypeFormatter_delete(i18n::phonenumbers::AsYouTypeFormatter* this_ptr) {
  delete this_ptr;
}

void libphonenumber_sys_c_i18n_phonenumbers_AsYouTypeFormatter_destructor(i18n::phonenumbers::AsYouTypeFormatter* this_ptr) {
  libphonenumber_sys_c_call_destructor(this_ptr);
}

