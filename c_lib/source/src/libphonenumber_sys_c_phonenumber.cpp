#include "libphonenumber_sys_c_phonenumber.h"

int libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_ByteSize(const i18n::phonenumbers::PhoneNumber* this_ptr) {
  return this_ptr->ByteSize();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_CheckTypeAndMergeFrom(i18n::phonenumbers::PhoneNumber* this_ptr, const google::protobuf::MessageLite* from) {
  this_ptr->CheckTypeAndMergeFrom(*from);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_Clear(i18n::phonenumbers::PhoneNumber* this_ptr) {
  this_ptr->Clear();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_CopyFrom(i18n::phonenumbers::PhoneNumber* this_ptr, const i18n::phonenumbers::PhoneNumber* from) {
  this_ptr->CopyFrom(*from);
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_CountryCodeSource_IsValid(int value) {
  return i18n::phonenumbers::PhoneNumber::CountryCodeSource_IsValid(value);
}

int libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_GetCachedSize(const i18n::phonenumbers::PhoneNumber* this_ptr) {
  return this_ptr->GetCachedSize();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_GetTypeName_as_ptr(const i18n::phonenumbers::PhoneNumber* this_ptr) {
  return new std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >(this_ptr->GetTypeName());
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_GetTypeName_to_output(const i18n::phonenumbers::PhoneNumber* this_ptr, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* output) {
  new(output) std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >(this_ptr->GetTypeName());
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_IsInitialized(const i18n::phonenumbers::PhoneNumber* this_ptr) {
  return this_ptr->IsInitialized();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_MergeFrom(i18n::phonenumbers::PhoneNumber* this_ptr, const i18n::phonenumbers::PhoneNumber* from) {
  this_ptr->MergeFrom(*from);
}

i18n::phonenumbers::PhoneNumber* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_New_arena(const i18n::phonenumbers::PhoneNumber* this_ptr, google::protobuf::Arena* arena) {
  return this_ptr->New(arena);
}

i18n::phonenumbers::PhoneNumber* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_New_no_args(const i18n::phonenumbers::PhoneNumber* this_ptr) {
  return this_ptr->New();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_Swap(i18n::phonenumbers::PhoneNumber* this_ptr, i18n::phonenumbers::PhoneNumber* other) {
  this_ptr->Swap(other);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_clear_country_code(i18n::phonenumbers::PhoneNumber* this_ptr) {
  this_ptr->clear_country_code();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_clear_country_code_source(i18n::phonenumbers::PhoneNumber* this_ptr) {
  this_ptr->clear_country_code_source();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_clear_extension(i18n::phonenumbers::PhoneNumber* this_ptr) {
  this_ptr->clear_extension();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_clear_italian_leading_zero(i18n::phonenumbers::PhoneNumber* this_ptr) {
  this_ptr->clear_italian_leading_zero();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_clear_national_number(i18n::phonenumbers::PhoneNumber* this_ptr) {
  this_ptr->clear_national_number();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_clear_number_of_leading_zeros(i18n::phonenumbers::PhoneNumber* this_ptr) {
  this_ptr->clear_number_of_leading_zeros();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_clear_preferred_domestic_carrier_code(i18n::phonenumbers::PhoneNumber* this_ptr) {
  this_ptr->clear_preferred_domestic_carrier_code();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_clear_raw_input(i18n::phonenumbers::PhoneNumber* this_ptr) {
  this_ptr->clear_raw_input();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_constructor_from(const i18n::phonenumbers::PhoneNumber* from, i18n::phonenumbers::PhoneNumber* output) {
  new(output) i18n::phonenumbers::PhoneNumber(*from);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_constructor_no_args(i18n::phonenumbers::PhoneNumber* output) {
  new(output) i18n::phonenumbers::PhoneNumber();
}

int libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_country_code(const i18n::phonenumbers::PhoneNumber* this_ptr) {
  return this_ptr->country_code();
}

i18n::phonenumbers::PhoneNumber_CountryCodeSource libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_country_code_source(const i18n::phonenumbers::PhoneNumber* this_ptr) {
  return this_ptr->country_code_source();
}

const i18n::phonenumbers::PhoneNumber* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_default_instance() {
  return &i18n::phonenumbers::PhoneNumber::default_instance();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_delete(i18n::phonenumbers::PhoneNumber* this_ptr) {
  delete this_ptr;
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_destructor(i18n::phonenumbers::PhoneNumber* this_ptr) {
  libphonenumber_sys_c_call_destructor(this_ptr);
}

const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_extension(const i18n::phonenumbers::PhoneNumber* this_ptr) {
  return &this_ptr->extension();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_has_country_code(const i18n::phonenumbers::PhoneNumber* this_ptr) {
  return this_ptr->has_country_code();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_has_country_code_source(const i18n::phonenumbers::PhoneNumber* this_ptr) {
  return this_ptr->has_country_code_source();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_has_extension(const i18n::phonenumbers::PhoneNumber* this_ptr) {
  return this_ptr->has_extension();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_has_italian_leading_zero(const i18n::phonenumbers::PhoneNumber* this_ptr) {
  return this_ptr->has_italian_leading_zero();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_has_national_number(const i18n::phonenumbers::PhoneNumber* this_ptr) {
  return this_ptr->has_national_number();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_has_number_of_leading_zeros(const i18n::phonenumbers::PhoneNumber* this_ptr) {
  return this_ptr->has_number_of_leading_zeros();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_has_preferred_domestic_carrier_code(const i18n::phonenumbers::PhoneNumber* this_ptr) {
  return this_ptr->has_preferred_domestic_carrier_code();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_has_raw_input(const i18n::phonenumbers::PhoneNumber* this_ptr) {
  return this_ptr->has_raw_input();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_italian_leading_zero(const i18n::phonenumbers::PhoneNumber* this_ptr) {
  return this_ptr->italian_leading_zero();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_mutable_extension(i18n::phonenumbers::PhoneNumber* this_ptr) {
  return this_ptr->mutable_extension();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_mutable_preferred_domestic_carrier_code(i18n::phonenumbers::PhoneNumber* this_ptr) {
  return this_ptr->mutable_preferred_domestic_carrier_code();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_mutable_raw_input(i18n::phonenumbers::PhoneNumber* this_ptr) {
  return this_ptr->mutable_raw_input();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_mutable_unknown_fields(i18n::phonenumbers::PhoneNumber* this_ptr) {
  return this_ptr->mutable_unknown_fields();
}

unsigned long libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_national_number(const i18n::phonenumbers::PhoneNumber* this_ptr) {
  return this_ptr->national_number();
}

i18n::phonenumbers::PhoneNumber* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_new_from(const i18n::phonenumbers::PhoneNumber* from) {
  return new i18n::phonenumbers::PhoneNumber(*from);
}

i18n::phonenumbers::PhoneNumber* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_new_no_args() {
  return new i18n::phonenumbers::PhoneNumber();
}

int libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_number_of_leading_zeros(const i18n::phonenumbers::PhoneNumber* this_ptr) {
  return this_ptr->number_of_leading_zeros();
}

i18n::phonenumbers::PhoneNumber* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_operator_assign(i18n::phonenumbers::PhoneNumber* this_ptr, const i18n::phonenumbers::PhoneNumber* from) {
  return &this_ptr->operator=(*from);
}

const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_preferred_domestic_carrier_code(const i18n::phonenumbers::PhoneNumber* this_ptr) {
  return &this_ptr->preferred_domestic_carrier_code();
}

const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_raw_input(const i18n::phonenumbers::PhoneNumber* this_ptr) {
  return &this_ptr->raw_input();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_release_extension(i18n::phonenumbers::PhoneNumber* this_ptr) {
  return this_ptr->release_extension();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_release_preferred_domestic_carrier_code(i18n::phonenumbers::PhoneNumber* this_ptr) {
  return this_ptr->release_preferred_domestic_carrier_code();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_release_raw_input(i18n::phonenumbers::PhoneNumber* this_ptr) {
  return this_ptr->release_raw_input();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_allocated_extension(i18n::phonenumbers::PhoneNumber* this_ptr, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* extension) {
  this_ptr->set_allocated_extension(extension);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_allocated_preferred_domestic_carrier_code(i18n::phonenumbers::PhoneNumber* this_ptr, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* preferred_domestic_carrier_code) {
  this_ptr->set_allocated_preferred_domestic_carrier_code(preferred_domestic_carrier_code);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_allocated_raw_input(i18n::phonenumbers::PhoneNumber* this_ptr, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* raw_input) {
  this_ptr->set_allocated_raw_input(raw_input);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_country_code(i18n::phonenumbers::PhoneNumber* this_ptr, int value) {
  this_ptr->set_country_code(value);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_country_code_source(i18n::phonenumbers::PhoneNumber* this_ptr, i18n::phonenumbers::PhoneNumber_CountryCodeSource value) {
  this_ptr->set_country_code_source(value);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_extension_char(i18n::phonenumbers::PhoneNumber* this_ptr, const char* value) {
  this_ptr->set_extension(value);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_extension_char_unsigned_long(i18n::phonenumbers::PhoneNumber* this_ptr, const char* value, unsigned long size) {
  this_ptr->set_extension(value, size);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_extension_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(i18n::phonenumbers::PhoneNumber* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* value) {
  this_ptr->set_extension(*value);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_italian_leading_zero(i18n::phonenumbers::PhoneNumber* this_ptr, bool value) {
  this_ptr->set_italian_leading_zero(value);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_national_number(i18n::phonenumbers::PhoneNumber* this_ptr, unsigned long value) {
  this_ptr->set_national_number(value);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_number_of_leading_zeros(i18n::phonenumbers::PhoneNumber* this_ptr, int value) {
  this_ptr->set_number_of_leading_zeros(value);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_preferred_domestic_carrier_code_char(i18n::phonenumbers::PhoneNumber* this_ptr, const char* value) {
  this_ptr->set_preferred_domestic_carrier_code(value);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_preferred_domestic_carrier_code_char_unsigned_long(i18n::phonenumbers::PhoneNumber* this_ptr, const char* value, unsigned long size) {
  this_ptr->set_preferred_domestic_carrier_code(value, size);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_preferred_domestic_carrier_code_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(i18n::phonenumbers::PhoneNumber* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* value) {
  this_ptr->set_preferred_domestic_carrier_code(*value);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_raw_input_char(i18n::phonenumbers::PhoneNumber* this_ptr, const char* value) {
  this_ptr->set_raw_input(value);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_raw_input_char_unsigned_long(i18n::phonenumbers::PhoneNumber* this_ptr, const char* value, unsigned long size) {
  this_ptr->set_raw_input(value, size);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_raw_input_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(i18n::phonenumbers::PhoneNumber* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* value) {
  this_ptr->set_raw_input(*value);
}

const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_unknown_fields(const i18n::phonenumbers::PhoneNumber* this_ptr) {
  return &this_ptr->unknown_fields();
}

bool libphonenumber_sys_c_phonenumber_G_i18n_phonenumbers_PhoneNumber_CountryCodeSource_IsValid(int value) {
  return i18n::phonenumbers::PhoneNumber_CountryCodeSource_IsValid(value);
}

void libphonenumber_sys_c_phonenumber_G_i18n_phonenumbers_protobuf_AddDesc_phonenumber_2eproto() {
  i18n::phonenumbers::protobuf_AddDesc_phonenumber_2eproto();
}

void libphonenumber_sys_c_phonenumber_G_i18n_phonenumbers_protobuf_ShutdownFile_phonenumber_2eproto() {
  i18n::phonenumbers::protobuf_ShutdownFile_phonenumber_2eproto();
}

