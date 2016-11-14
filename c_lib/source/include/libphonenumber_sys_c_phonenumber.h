#ifndef LIBPHONENUMBER_SYS_C_PHONENUMBER_H
#define LIBPHONENUMBER_SYS_C_PHONENUMBER_H

#include "libphonenumber_sys_c_global.h"

extern "C" {

LIBPHONENUMBER_SYS_C_EXPORT int libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_ByteSize(const i18n::phonenumbers::PhoneNumber* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_CheckTypeAndMergeFrom(i18n::phonenumbers::PhoneNumber* this_ptr, const google::protobuf::MessageLite* from);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_Clear(i18n::phonenumbers::PhoneNumber* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_CopyFrom(i18n::phonenumbers::PhoneNumber* this_ptr, const i18n::phonenumbers::PhoneNumber* from);
LIBPHONENUMBER_SYS_C_EXPORT bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_CountryCodeSource_IsValid(int value);
LIBPHONENUMBER_SYS_C_EXPORT int libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_GetCachedSize(const i18n::phonenumbers::PhoneNumber* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_GetTypeName_as_ptr(const i18n::phonenumbers::PhoneNumber* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_GetTypeName_to_output(const i18n::phonenumbers::PhoneNumber* this_ptr, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* output);
LIBPHONENUMBER_SYS_C_EXPORT bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_IsInitialized(const i18n::phonenumbers::PhoneNumber* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_MergeFrom(i18n::phonenumbers::PhoneNumber* this_ptr, const i18n::phonenumbers::PhoneNumber* from);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::PhoneNumber* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_New_arena(const i18n::phonenumbers::PhoneNumber* this_ptr, google::protobuf::Arena* arena);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::PhoneNumber* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_New_no_args(const i18n::phonenumbers::PhoneNumber* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_Swap(i18n::phonenumbers::PhoneNumber* this_ptr, i18n::phonenumbers::PhoneNumber* other);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_clear_country_code(i18n::phonenumbers::PhoneNumber* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_clear_country_code_source(i18n::phonenumbers::PhoneNumber* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_clear_extension(i18n::phonenumbers::PhoneNumber* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_clear_italian_leading_zero(i18n::phonenumbers::PhoneNumber* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_clear_national_number(i18n::phonenumbers::PhoneNumber* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_clear_number_of_leading_zeros(i18n::phonenumbers::PhoneNumber* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_clear_preferred_domestic_carrier_code(i18n::phonenumbers::PhoneNumber* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_clear_raw_input(i18n::phonenumbers::PhoneNumber* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_constructor_from(const i18n::phonenumbers::PhoneNumber* from, i18n::phonenumbers::PhoneNumber* output);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_constructor_no_args(i18n::phonenumbers::PhoneNumber* output);
LIBPHONENUMBER_SYS_C_EXPORT int libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_country_code(const i18n::phonenumbers::PhoneNumber* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::PhoneNumber_CountryCodeSource libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_country_code_source(const i18n::phonenumbers::PhoneNumber* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT const i18n::phonenumbers::PhoneNumber* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_default_instance();
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_delete(i18n::phonenumbers::PhoneNumber* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_destructor(i18n::phonenumbers::PhoneNumber* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_extension(const i18n::phonenumbers::PhoneNumber* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_has_country_code(const i18n::phonenumbers::PhoneNumber* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_has_country_code_source(const i18n::phonenumbers::PhoneNumber* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_has_extension(const i18n::phonenumbers::PhoneNumber* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_has_italian_leading_zero(const i18n::phonenumbers::PhoneNumber* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_has_national_number(const i18n::phonenumbers::PhoneNumber* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_has_number_of_leading_zeros(const i18n::phonenumbers::PhoneNumber* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_has_preferred_domestic_carrier_code(const i18n::phonenumbers::PhoneNumber* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_has_raw_input(const i18n::phonenumbers::PhoneNumber* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_italian_leading_zero(const i18n::phonenumbers::PhoneNumber* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_mutable_extension(i18n::phonenumbers::PhoneNumber* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_mutable_preferred_domestic_carrier_code(i18n::phonenumbers::PhoneNumber* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_mutable_raw_input(i18n::phonenumbers::PhoneNumber* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_mutable_unknown_fields(i18n::phonenumbers::PhoneNumber* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT unsigned long libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_national_number(const i18n::phonenumbers::PhoneNumber* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::PhoneNumber* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_new_from(const i18n::phonenumbers::PhoneNumber* from);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::PhoneNumber* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_new_no_args();
LIBPHONENUMBER_SYS_C_EXPORT int libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_number_of_leading_zeros(const i18n::phonenumbers::PhoneNumber* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT i18n::phonenumbers::PhoneNumber* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_operator_assign(i18n::phonenumbers::PhoneNumber* this_ptr, const i18n::phonenumbers::PhoneNumber* from);
LIBPHONENUMBER_SYS_C_EXPORT const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_preferred_domestic_carrier_code(const i18n::phonenumbers::PhoneNumber* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_raw_input(const i18n::phonenumbers::PhoneNumber* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_release_extension(i18n::phonenumbers::PhoneNumber* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_release_preferred_domestic_carrier_code(i18n::phonenumbers::PhoneNumber* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_release_raw_input(i18n::phonenumbers::PhoneNumber* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_allocated_extension(i18n::phonenumbers::PhoneNumber* this_ptr, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* extension);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_allocated_preferred_domestic_carrier_code(i18n::phonenumbers::PhoneNumber* this_ptr, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* preferred_domestic_carrier_code);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_allocated_raw_input(i18n::phonenumbers::PhoneNumber* this_ptr, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* raw_input);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_country_code(i18n::phonenumbers::PhoneNumber* this_ptr, int value);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_country_code_source(i18n::phonenumbers::PhoneNumber* this_ptr, i18n::phonenumbers::PhoneNumber_CountryCodeSource value);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_extension_char(i18n::phonenumbers::PhoneNumber* this_ptr, const char* value);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_extension_char_unsigned_long(i18n::phonenumbers::PhoneNumber* this_ptr, const char* value, unsigned long size);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_extension_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(i18n::phonenumbers::PhoneNumber* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* value);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_italian_leading_zero(i18n::phonenumbers::PhoneNumber* this_ptr, bool value);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_national_number(i18n::phonenumbers::PhoneNumber* this_ptr, unsigned long value);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_number_of_leading_zeros(i18n::phonenumbers::PhoneNumber* this_ptr, int value);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_preferred_domestic_carrier_code_char(i18n::phonenumbers::PhoneNumber* this_ptr, const char* value);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_preferred_domestic_carrier_code_char_unsigned_long(i18n::phonenumbers::PhoneNumber* this_ptr, const char* value, unsigned long size);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_preferred_domestic_carrier_code_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(i18n::phonenumbers::PhoneNumber* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* value);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_raw_input_char(i18n::phonenumbers::PhoneNumber* this_ptr, const char* value);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_raw_input_char_unsigned_long(i18n::phonenumbers::PhoneNumber* this_ptr, const char* value, unsigned long size);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_raw_input_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(i18n::phonenumbers::PhoneNumber* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* value);
LIBPHONENUMBER_SYS_C_EXPORT const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_unknown_fields(const i18n::phonenumbers::PhoneNumber* this_ptr);
LIBPHONENUMBER_SYS_C_EXPORT bool libphonenumber_sys_c_phonenumber_G_i18n_phonenumbers_PhoneNumber_CountryCodeSource_IsValid(int value);
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_phonenumber_G_i18n_phonenumbers_protobuf_AddDesc_phonenumber_2eproto();
LIBPHONENUMBER_SYS_C_EXPORT void libphonenumber_sys_c_phonenumber_G_i18n_phonenumbers_protobuf_ShutdownFile_phonenumber_2eproto();

} // extern "C"

#endif // LIBPHONENUMBER_SYS_C_PHONENUMBER_H
