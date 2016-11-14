#ifndef LIBPHONENUMBER_SYS_C_GLOBAL_H
#define LIBPHONENUMBER_SYS_C_GLOBAL_H

#include <stdint.h>

// placement new statements require this
#include <new>

#include "phonenumbers/phonenumberutil.h"
#include "phonenumbers/asyoutypeformatter.h"
#include "phonenumbers/shortnumberinfo.h"
#include "phonenumbers/phonenumber.pb.h"
#include "phonenumbers/phonenumbermatcher.h"
#include "phonenumbers/phonenumbermatch.h"
#include "phonenumbers/phonemetadata.pb.h"

#include "libphonenumber_sys_c_exports.h"

#ifdef __cplusplus // if C++
template<typename T>
void libphonenumber_sys_c_call_destructor(T* x) {
    x->~T();
}
#endif


#endif // LIBPHONENUMBER_SYS_C_GLOBAL_H
