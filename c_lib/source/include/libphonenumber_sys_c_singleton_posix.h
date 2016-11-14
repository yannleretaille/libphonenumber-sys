#ifndef LIBPHONENUMBER_SYS_C_SINGLETON_POSIX_H
#define LIBPHONENUMBER_SYS_C_SINGLETON_POSIX_H

#include "libphonenumber_sys_c_global.h"

extern "C" {

LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::PhoneNumberUtil* libphonenumber_sys_c_i18n_phonenumbers_Singleton_i18n_phonenumbers_PhoneNumberUtil_GetInstance();
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_Singleton_i18n_phonenumbers_PhoneNumberUtil_delete(i18n::phonenumbers::Singleton< i18n::phonenumbers::PhoneNumberUtil >* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_Singleton_i18n_phonenumbers_PhoneNumberUtil_destructor(i18n::phonenumbers::Singleton< i18n::phonenumbers::PhoneNumberUtil >* this_ptr);

} // extern "C"

#endif // LIBPHONENUMBER_SYS_C_SINGLETON_POSIX_H
