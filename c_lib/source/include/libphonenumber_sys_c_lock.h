#ifndef LIBPHONENUMBER_SYS_C_LOCK_H
#define LIBPHONENUMBER_SYS_C_LOCK_H

#include "libphonenumber_sys_c_global.h"

extern "C" {

LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_AutoLock_constructor(i18n::phonenumbers::Lock* lock, i18n::phonenumbers::AutoLock* output);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_AutoLock_delete(i18n::phonenumbers::AutoLock* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_AutoLock_destructor(i18n::phonenumbers::AutoLock* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::AutoLock* libphonenumber_sys_c_i18n_phonenumbers_AutoLock_new(i18n::phonenumbers::Lock* lock);

} // extern "C"

#endif // LIBPHONENUMBER_SYS_C_LOCK_H
