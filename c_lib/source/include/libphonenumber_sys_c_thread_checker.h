#ifndef LIBPHONENUMBER_SYS_C_THREAD_CHECKER_H
#define LIBPHONENUMBER_SYS_C_THREAD_CHECKER_H

#include "libphonenumber_sys_c_global.h"

extern "C" {

LIBPHONENUMBER_SYS_C_EXPORT bool libphonenumber_sys_c_i18n_phonenumbers_ThreadChecker_CalledOnValidThread(const i18n::phonenumbers::ThreadChecker* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_ThreadChecker_constructor(i18n::phonenumbers::ThreadChecker* output);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::ThreadChecker* libphonenumber_sys_c_i18n_phonenumbers_ThreadChecker_new();

} // extern "C"

#endif // LIBPHONENUMBER_SYS_C_THREAD_CHECKER_H
