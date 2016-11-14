#include "libphonenumber_sys_c_thread_checker.h"

bool libphonenumber_sys_c_i18n_phonenumbers_ThreadChecker_CalledOnValidThread(const i18n::phonenumbers::ThreadChecker* this_ptr) {
  return this_ptr->CalledOnValidThread();
}

void libphonenumber_sys_c_i18n_phonenumbers_ThreadChecker_constructor(i18n::phonenumbers::ThreadChecker* output) {
  new(output) i18n::phonenumbers::ThreadChecker();
}

i18n::phonenumbers::ThreadChecker* libphonenumber_sys_c_i18n_phonenumbers_ThreadChecker_new() {
  return new i18n::phonenumbers::ThreadChecker();
}

