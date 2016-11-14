#include "libphonenumber_sys_c_singleton_posix.h"

i18n::phonenumbers::PhoneNumberUtil* libphonenumber_sys_c_i18n_phonenumbers_Singleton_i18n_phonenumbers_PhoneNumberUtil_GetInstance() {
  return i18n::phonenumbers::Singleton< i18n::phonenumbers::PhoneNumberUtil >::GetInstance();
}

void libphonenumber_sys_c_i18n_phonenumbers_Singleton_i18n_phonenumbers_PhoneNumberUtil_delete(i18n::phonenumbers::Singleton< i18n::phonenumbers::PhoneNumberUtil >* this_ptr) {
  delete this_ptr;
}

void libphonenumber_sys_c_i18n_phonenumbers_Singleton_i18n_phonenumbers_PhoneNumberUtil_destructor(i18n::phonenumbers::Singleton< i18n::phonenumbers::PhoneNumberUtil >* this_ptr) {
  libphonenumber_sys_c_call_destructor(this_ptr);
}

