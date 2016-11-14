#ifndef LIBPHONENUMBER_SYS_C_LOCK_POSIX_H
#define LIBPHONENUMBER_SYS_C_LOCK_POSIX_H

#include "libphonenumber_sys_c_global.h"

extern "C" {

LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_Lock_Acquire(const i18n::phonenumbers::Lock* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_Lock_Release(const i18n::phonenumbers::Lock* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_Lock_constructor(i18n::phonenumbers::Lock* output);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_Lock_delete(i18n::phonenumbers::Lock* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_Lock_destructor(i18n::phonenumbers::Lock* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::Lock* libphonenumber_sys_c_i18n_phonenumbers_Lock_new();

} // extern "C"

#endif // LIBPHONENUMBER_SYS_C_LOCK_POSIX_H
