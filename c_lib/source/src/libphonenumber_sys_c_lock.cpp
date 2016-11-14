#include "libphonenumber_sys_c_lock.h"

void libphonenumber_sys_c_i18n_phonenumbers_AutoLock_constructor(i18n::phonenumbers::Lock* lock, i18n::phonenumbers::AutoLock* output) {
  new(output) i18n::phonenumbers::AutoLock(*lock);
}

void libphonenumber_sys_c_i18n_phonenumbers_AutoLock_delete(i18n::phonenumbers::AutoLock* this_ptr) {
  delete this_ptr;
}

void libphonenumber_sys_c_i18n_phonenumbers_AutoLock_destructor(i18n::phonenumbers::AutoLock* this_ptr) {
  libphonenumber_sys_c_call_destructor(this_ptr);
}

i18n::phonenumbers::AutoLock* libphonenumber_sys_c_i18n_phonenumbers_AutoLock_new(i18n::phonenumbers::Lock* lock) {
  return new i18n::phonenumbers::AutoLock(*lock);
}

