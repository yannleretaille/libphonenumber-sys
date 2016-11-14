#include "libphonenumber_sys_c_lock_posix.h"

void libphonenumber_sys_c_i18n_phonenumbers_Lock_Acquire(const i18n::phonenumbers::Lock* this_ptr) {
  this_ptr->Acquire();
}

void libphonenumber_sys_c_i18n_phonenumbers_Lock_Release(const i18n::phonenumbers::Lock* this_ptr) {
  this_ptr->Release();
}

void libphonenumber_sys_c_i18n_phonenumbers_Lock_constructor(i18n::phonenumbers::Lock* output) {
  new(output) i18n::phonenumbers::Lock();
}

void libphonenumber_sys_c_i18n_phonenumbers_Lock_delete(i18n::phonenumbers::Lock* this_ptr) {
  delete this_ptr;
}

void libphonenumber_sys_c_i18n_phonenumbers_Lock_destructor(i18n::phonenumbers::Lock* this_ptr) {
  libphonenumber_sys_c_call_destructor(this_ptr);
}

i18n::phonenumbers::Lock* libphonenumber_sys_c_i18n_phonenumbers_Lock_new() {
  return new i18n::phonenumbers::Lock();
}

