#include "libphonenumber_sys_c_scoped_ptr.h"

void libphonenumber_sys_c_i18n_phonenumbers_FreeDeleter_operator_call(const i18n::phonenumbers::FreeDeleter* this_ptr, void* ptr) {
  this_ptr->operator()(ptr);
}

