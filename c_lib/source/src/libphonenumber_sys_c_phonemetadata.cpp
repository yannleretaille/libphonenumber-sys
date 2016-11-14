#include "libphonenumber_sys_c_phonemetadata.h"

int libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_ByteSize(const i18n::phonenumbers::NumberFormat* this_ptr) {
  return this_ptr->ByteSize();
}

void libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_CheckTypeAndMergeFrom(i18n::phonenumbers::NumberFormat* this_ptr, const google::protobuf::MessageLite* from) {
  this_ptr->CheckTypeAndMergeFrom(*from);
}

void libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_Clear(i18n::phonenumbers::NumberFormat* this_ptr) {
  this_ptr->Clear();
}

void libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_CopyFrom(i18n::phonenumbers::NumberFormat* this_ptr, const i18n::phonenumbers::NumberFormat* from) {
  this_ptr->CopyFrom(*from);
}

void libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_DiscardUnknownFields(i18n::phonenumbers::NumberFormat* this_ptr) {
  this_ptr->DiscardUnknownFields();
}

int libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_GetCachedSize(const i18n::phonenumbers::NumberFormat* this_ptr) {
  return this_ptr->GetCachedSize();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_GetTypeName_as_ptr(const i18n::phonenumbers::NumberFormat* this_ptr) {
  return new std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >(this_ptr->GetTypeName());
}

void libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_GetTypeName_to_output(const i18n::phonenumbers::NumberFormat* this_ptr, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* output) {
  new(output) std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >(this_ptr->GetTypeName());
}

bool libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_IsInitialized(const i18n::phonenumbers::NumberFormat* this_ptr) {
  return this_ptr->IsInitialized();
}

void libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_MergeFrom(i18n::phonenumbers::NumberFormat* this_ptr, const i18n::phonenumbers::NumberFormat* from) {
  this_ptr->MergeFrom(*from);
}

i18n::phonenumbers::NumberFormat* libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_New_arena(const i18n::phonenumbers::NumberFormat* this_ptr, google::protobuf::Arena* arena) {
  return this_ptr->New(arena);
}

i18n::phonenumbers::NumberFormat* libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_New_no_args(const i18n::phonenumbers::NumberFormat* this_ptr) {
  return this_ptr->New();
}

void libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_Swap(i18n::phonenumbers::NumberFormat* this_ptr, i18n::phonenumbers::NumberFormat* other) {
  this_ptr->Swap(other);
}

void libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_add_leading_digits_pattern_char(i18n::phonenumbers::NumberFormat* this_ptr, const char* value) {
  this_ptr->add_leading_digits_pattern(value);
}

void libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_add_leading_digits_pattern_char_unsigned_long(i18n::phonenumbers::NumberFormat* this_ptr, const char* value, unsigned long size) {
  this_ptr->add_leading_digits_pattern(value, size);
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_add_leading_digits_pattern_no_args(i18n::phonenumbers::NumberFormat* this_ptr) {
  return this_ptr->add_leading_digits_pattern();
}

void libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_add_leading_digits_pattern_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(i18n::phonenumbers::NumberFormat* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* value) {
  this_ptr->add_leading_digits_pattern(*value);
}

void libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_clear_domestic_carrier_code_formatting_rule(i18n::phonenumbers::NumberFormat* this_ptr) {
  this_ptr->clear_domestic_carrier_code_formatting_rule();
}

void libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_clear_format(i18n::phonenumbers::NumberFormat* this_ptr) {
  this_ptr->clear_format();
}

void libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_clear_leading_digits_pattern(i18n::phonenumbers::NumberFormat* this_ptr) {
  this_ptr->clear_leading_digits_pattern();
}

void libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_clear_national_prefix_formatting_rule(i18n::phonenumbers::NumberFormat* this_ptr) {
  this_ptr->clear_national_prefix_formatting_rule();
}

void libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_clear_national_prefix_optional_when_formatting(i18n::phonenumbers::NumberFormat* this_ptr) {
  this_ptr->clear_national_prefix_optional_when_formatting();
}

void libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_clear_pattern(i18n::phonenumbers::NumberFormat* this_ptr) {
  this_ptr->clear_pattern();
}

void libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_constructor_from(const i18n::phonenumbers::NumberFormat* from, i18n::phonenumbers::NumberFormat* output) {
  new(output) i18n::phonenumbers::NumberFormat(*from);
}

void libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_constructor_no_args(i18n::phonenumbers::NumberFormat* output) {
  new(output) i18n::phonenumbers::NumberFormat();
}

const i18n::phonenumbers::NumberFormat* libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_default_instance() {
  return &i18n::phonenumbers::NumberFormat::default_instance();
}

void libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_delete(i18n::phonenumbers::NumberFormat* this_ptr) {
  delete this_ptr;
}

void libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_destructor(i18n::phonenumbers::NumberFormat* this_ptr) {
  libphonenumber_sys_c_call_destructor(this_ptr);
}

const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_domestic_carrier_code_formatting_rule(const i18n::phonenumbers::NumberFormat* this_ptr) {
  return &this_ptr->domestic_carrier_code_formatting_rule();
}

const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_format(const i18n::phonenumbers::NumberFormat* this_ptr) {
  return &this_ptr->format();
}

bool libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_has_domestic_carrier_code_formatting_rule(const i18n::phonenumbers::NumberFormat* this_ptr) {
  return this_ptr->has_domestic_carrier_code_formatting_rule();
}

bool libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_has_format(const i18n::phonenumbers::NumberFormat* this_ptr) {
  return this_ptr->has_format();
}

bool libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_has_national_prefix_formatting_rule(const i18n::phonenumbers::NumberFormat* this_ptr) {
  return this_ptr->has_national_prefix_formatting_rule();
}

bool libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_has_national_prefix_optional_when_formatting(const i18n::phonenumbers::NumberFormat* this_ptr) {
  return this_ptr->has_national_prefix_optional_when_formatting();
}

bool libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_has_pattern(const i18n::phonenumbers::NumberFormat* this_ptr) {
  return this_ptr->has_pattern();
}

const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_leading_digits_pattern_index(const i18n::phonenumbers::NumberFormat* this_ptr, int index) {
  return &this_ptr->leading_digits_pattern(index);
}

const google::protobuf::RepeatedPtrField< std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > > >* libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_leading_digits_pattern_no_args(const i18n::phonenumbers::NumberFormat* this_ptr) {
  return &this_ptr->leading_digits_pattern();
}

int libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_leading_digits_pattern_size(const i18n::phonenumbers::NumberFormat* this_ptr) {
  return this_ptr->leading_digits_pattern_size();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_mutable_domestic_carrier_code_formatting_rule(i18n::phonenumbers::NumberFormat* this_ptr) {
  return this_ptr->mutable_domestic_carrier_code_formatting_rule();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_mutable_format(i18n::phonenumbers::NumberFormat* this_ptr) {
  return this_ptr->mutable_format();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_mutable_leading_digits_pattern_index(i18n::phonenumbers::NumberFormat* this_ptr, int index) {
  return this_ptr->mutable_leading_digits_pattern(index);
}

google::protobuf::RepeatedPtrField< std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > > >* libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_mutable_leading_digits_pattern_no_args(i18n::phonenumbers::NumberFormat* this_ptr) {
  return this_ptr->mutable_leading_digits_pattern();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_mutable_national_prefix_formatting_rule(i18n::phonenumbers::NumberFormat* this_ptr) {
  return this_ptr->mutable_national_prefix_formatting_rule();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_mutable_pattern(i18n::phonenumbers::NumberFormat* this_ptr) {
  return this_ptr->mutable_pattern();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_mutable_unknown_fields(i18n::phonenumbers::NumberFormat* this_ptr) {
  return this_ptr->mutable_unknown_fields();
}

const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_national_prefix_formatting_rule(const i18n::phonenumbers::NumberFormat* this_ptr) {
  return &this_ptr->national_prefix_formatting_rule();
}

bool libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_national_prefix_optional_when_formatting(const i18n::phonenumbers::NumberFormat* this_ptr) {
  return this_ptr->national_prefix_optional_when_formatting();
}

i18n::phonenumbers::NumberFormat* libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_new_from(const i18n::phonenumbers::NumberFormat* from) {
  return new i18n::phonenumbers::NumberFormat(*from);
}

i18n::phonenumbers::NumberFormat* libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_new_no_args() {
  return new i18n::phonenumbers::NumberFormat();
}

i18n::phonenumbers::NumberFormat* libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_operator_assign(i18n::phonenumbers::NumberFormat* this_ptr, const i18n::phonenumbers::NumberFormat* from) {
  return &this_ptr->operator=(*from);
}

const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_pattern(const i18n::phonenumbers::NumberFormat* this_ptr) {
  return &this_ptr->pattern();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_release_domestic_carrier_code_formatting_rule(i18n::phonenumbers::NumberFormat* this_ptr) {
  return this_ptr->release_domestic_carrier_code_formatting_rule();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_release_format(i18n::phonenumbers::NumberFormat* this_ptr) {
  return this_ptr->release_format();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_release_national_prefix_formatting_rule(i18n::phonenumbers::NumberFormat* this_ptr) {
  return this_ptr->release_national_prefix_formatting_rule();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_release_pattern(i18n::phonenumbers::NumberFormat* this_ptr) {
  return this_ptr->release_pattern();
}

void libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_allocated_domestic_carrier_code_formatting_rule(i18n::phonenumbers::NumberFormat* this_ptr, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* domestic_carrier_code_formatting_rule) {
  this_ptr->set_allocated_domestic_carrier_code_formatting_rule(domestic_carrier_code_formatting_rule);
}

void libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_allocated_format(i18n::phonenumbers::NumberFormat* this_ptr, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* format) {
  this_ptr->set_allocated_format(format);
}

void libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_allocated_national_prefix_formatting_rule(i18n::phonenumbers::NumberFormat* this_ptr, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* national_prefix_formatting_rule) {
  this_ptr->set_allocated_national_prefix_formatting_rule(national_prefix_formatting_rule);
}

void libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_allocated_pattern(i18n::phonenumbers::NumberFormat* this_ptr, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* pattern) {
  this_ptr->set_allocated_pattern(pattern);
}

void libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_domestic_carrier_code_formatting_rule_char(i18n::phonenumbers::NumberFormat* this_ptr, const char* value) {
  this_ptr->set_domestic_carrier_code_formatting_rule(value);
}

void libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_domestic_carrier_code_formatting_rule_char_unsigned_long(i18n::phonenumbers::NumberFormat* this_ptr, const char* value, unsigned long size) {
  this_ptr->set_domestic_carrier_code_formatting_rule(value, size);
}

void libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_domestic_carrier_code_formatting_rule_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(i18n::phonenumbers::NumberFormat* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* value) {
  this_ptr->set_domestic_carrier_code_formatting_rule(*value);
}

void libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_format_char(i18n::phonenumbers::NumberFormat* this_ptr, const char* value) {
  this_ptr->set_format(value);
}

void libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_format_char_unsigned_long(i18n::phonenumbers::NumberFormat* this_ptr, const char* value, unsigned long size) {
  this_ptr->set_format(value, size);
}

void libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_format_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(i18n::phonenumbers::NumberFormat* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* value) {
  this_ptr->set_format(*value);
}

void libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_leading_digits_pattern_int_char(i18n::phonenumbers::NumberFormat* this_ptr, int index, const char* value) {
  this_ptr->set_leading_digits_pattern(index, value);
}

void libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_leading_digits_pattern_int_char_unsigned_long(i18n::phonenumbers::NumberFormat* this_ptr, int index, const char* value, unsigned long size) {
  this_ptr->set_leading_digits_pattern(index, value, size);
}

void libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_leading_digits_pattern_int_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(i18n::phonenumbers::NumberFormat* this_ptr, int index, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* value) {
  this_ptr->set_leading_digits_pattern(index, *value);
}

void libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_national_prefix_formatting_rule_char(i18n::phonenumbers::NumberFormat* this_ptr, const char* value) {
  this_ptr->set_national_prefix_formatting_rule(value);
}

void libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_national_prefix_formatting_rule_char_unsigned_long(i18n::phonenumbers::NumberFormat* this_ptr, const char* value, unsigned long size) {
  this_ptr->set_national_prefix_formatting_rule(value, size);
}

void libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_national_prefix_formatting_rule_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(i18n::phonenumbers::NumberFormat* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* value) {
  this_ptr->set_national_prefix_formatting_rule(*value);
}

void libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_national_prefix_optional_when_formatting(i18n::phonenumbers::NumberFormat* this_ptr, bool value) {
  this_ptr->set_national_prefix_optional_when_formatting(value);
}

void libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_pattern_char(i18n::phonenumbers::NumberFormat* this_ptr, const char* value) {
  this_ptr->set_pattern(value);
}

void libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_pattern_char_unsigned_long(i18n::phonenumbers::NumberFormat* this_ptr, const char* value, unsigned long size) {
  this_ptr->set_pattern(value, size);
}

void libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_pattern_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(i18n::phonenumbers::NumberFormat* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* value) {
  this_ptr->set_pattern(*value);
}

const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_unknown_fields(const i18n::phonenumbers::NumberFormat* this_ptr) {
  return &this_ptr->unknown_fields();
}

int libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_ByteSize(const i18n::phonenumbers::PhoneMetadataCollection* this_ptr) {
  return this_ptr->ByteSize();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_CheckTypeAndMergeFrom(i18n::phonenumbers::PhoneMetadataCollection* this_ptr, const google::protobuf::MessageLite* from) {
  this_ptr->CheckTypeAndMergeFrom(*from);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_Clear(i18n::phonenumbers::PhoneMetadataCollection* this_ptr) {
  this_ptr->Clear();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_CopyFrom(i18n::phonenumbers::PhoneMetadataCollection* this_ptr, const i18n::phonenumbers::PhoneMetadataCollection* from) {
  this_ptr->CopyFrom(*from);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_DiscardUnknownFields(i18n::phonenumbers::PhoneMetadataCollection* this_ptr) {
  this_ptr->DiscardUnknownFields();
}

int libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_GetCachedSize(const i18n::phonenumbers::PhoneMetadataCollection* this_ptr) {
  return this_ptr->GetCachedSize();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_GetTypeName_as_ptr(const i18n::phonenumbers::PhoneMetadataCollection* this_ptr) {
  return new std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >(this_ptr->GetTypeName());
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_GetTypeName_to_output(const i18n::phonenumbers::PhoneMetadataCollection* this_ptr, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* output) {
  new(output) std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >(this_ptr->GetTypeName());
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_IsInitialized(const i18n::phonenumbers::PhoneMetadataCollection* this_ptr) {
  return this_ptr->IsInitialized();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_MergeFrom(i18n::phonenumbers::PhoneMetadataCollection* this_ptr, const i18n::phonenumbers::PhoneMetadataCollection* from) {
  this_ptr->MergeFrom(*from);
}

i18n::phonenumbers::PhoneMetadataCollection* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_New_arena(const i18n::phonenumbers::PhoneMetadataCollection* this_ptr, google::protobuf::Arena* arena) {
  return this_ptr->New(arena);
}

i18n::phonenumbers::PhoneMetadataCollection* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_New_no_args(const i18n::phonenumbers::PhoneMetadataCollection* this_ptr) {
  return this_ptr->New();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_Swap(i18n::phonenumbers::PhoneMetadataCollection* this_ptr, i18n::phonenumbers::PhoneMetadataCollection* other) {
  this_ptr->Swap(other);
}

i18n::phonenumbers::PhoneMetadata* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_add_metadata(i18n::phonenumbers::PhoneMetadataCollection* this_ptr) {
  return this_ptr->add_metadata();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_clear_metadata(i18n::phonenumbers::PhoneMetadataCollection* this_ptr) {
  this_ptr->clear_metadata();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_constructor_from(const i18n::phonenumbers::PhoneMetadataCollection* from, i18n::phonenumbers::PhoneMetadataCollection* output) {
  new(output) i18n::phonenumbers::PhoneMetadataCollection(*from);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_constructor_no_args(i18n::phonenumbers::PhoneMetadataCollection* output) {
  new(output) i18n::phonenumbers::PhoneMetadataCollection();
}

const i18n::phonenumbers::PhoneMetadataCollection* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_default_instance() {
  return &i18n::phonenumbers::PhoneMetadataCollection::default_instance();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_delete(i18n::phonenumbers::PhoneMetadataCollection* this_ptr) {
  delete this_ptr;
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_destructor(i18n::phonenumbers::PhoneMetadataCollection* this_ptr) {
  libphonenumber_sys_c_call_destructor(this_ptr);
}

const i18n::phonenumbers::PhoneMetadata* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_metadata_index(const i18n::phonenumbers::PhoneMetadataCollection* this_ptr, int index) {
  return &this_ptr->metadata(index);
}

const google::protobuf::RepeatedPtrField< i18n::phonenumbers::PhoneMetadata >* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_metadata_no_args(const i18n::phonenumbers::PhoneMetadataCollection* this_ptr) {
  return &this_ptr->metadata();
}

int libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_metadata_size(const i18n::phonenumbers::PhoneMetadataCollection* this_ptr) {
  return this_ptr->metadata_size();
}

i18n::phonenumbers::PhoneMetadata* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_mutable_metadata_index(i18n::phonenumbers::PhoneMetadataCollection* this_ptr, int index) {
  return this_ptr->mutable_metadata(index);
}

google::protobuf::RepeatedPtrField< i18n::phonenumbers::PhoneMetadata >* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_mutable_metadata_no_args(i18n::phonenumbers::PhoneMetadataCollection* this_ptr) {
  return this_ptr->mutable_metadata();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_mutable_unknown_fields(i18n::phonenumbers::PhoneMetadataCollection* this_ptr) {
  return this_ptr->mutable_unknown_fields();
}

i18n::phonenumbers::PhoneMetadataCollection* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_new_from(const i18n::phonenumbers::PhoneMetadataCollection* from) {
  return new i18n::phonenumbers::PhoneMetadataCollection(*from);
}

i18n::phonenumbers::PhoneMetadataCollection* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_new_no_args() {
  return new i18n::phonenumbers::PhoneMetadataCollection();
}

i18n::phonenumbers::PhoneMetadataCollection* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_operator_assign(i18n::phonenumbers::PhoneMetadataCollection* this_ptr, const i18n::phonenumbers::PhoneMetadataCollection* from) {
  return &this_ptr->operator=(*from);
}

const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_unknown_fields(const i18n::phonenumbers::PhoneMetadataCollection* this_ptr) {
  return &this_ptr->unknown_fields();
}

int libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_ByteSize(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->ByteSize();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_CheckTypeAndMergeFrom(i18n::phonenumbers::PhoneMetadata* this_ptr, const google::protobuf::MessageLite* from) {
  this_ptr->CheckTypeAndMergeFrom(*from);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_Clear(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  this_ptr->Clear();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_CopyFrom(i18n::phonenumbers::PhoneMetadata* this_ptr, const i18n::phonenumbers::PhoneMetadata* from) {
  this_ptr->CopyFrom(*from);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_DiscardUnknownFields(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  this_ptr->DiscardUnknownFields();
}

int libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_GetCachedSize(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->GetCachedSize();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_GetTypeName_as_ptr(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return new std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >(this_ptr->GetTypeName());
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_GetTypeName_to_output(const i18n::phonenumbers::PhoneMetadata* this_ptr, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* output) {
  new(output) std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >(this_ptr->GetTypeName());
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_IsInitialized(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->IsInitialized();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_MergeFrom(i18n::phonenumbers::PhoneMetadata* this_ptr, const i18n::phonenumbers::PhoneMetadata* from) {
  this_ptr->MergeFrom(*from);
}

i18n::phonenumbers::PhoneMetadata* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_New_arena(const i18n::phonenumbers::PhoneMetadata* this_ptr, google::protobuf::Arena* arena) {
  return this_ptr->New(arena);
}

i18n::phonenumbers::PhoneMetadata* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_New_no_args(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->New();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_Swap(i18n::phonenumbers::PhoneMetadata* this_ptr, i18n::phonenumbers::PhoneMetadata* other) {
  this_ptr->Swap(other);
}

i18n::phonenumbers::NumberFormat* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_add_intl_number_format(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->add_intl_number_format();
}

i18n::phonenumbers::NumberFormat* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_add_number_format(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->add_number_format();
}

const i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_carrier_specific(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return &this_ptr->carrier_specific();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_carrier_specific(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  this_ptr->clear_carrier_specific();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_country_code(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  this_ptr->clear_country_code();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_emergency(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  this_ptr->clear_emergency();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_fixed_line(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  this_ptr->clear_fixed_line();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_general_desc(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  this_ptr->clear_general_desc();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_id(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  this_ptr->clear_id();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_international_prefix(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  this_ptr->clear_international_prefix();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_intl_number_format(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  this_ptr->clear_intl_number_format();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_leading_digits(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  this_ptr->clear_leading_digits();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_leading_zero_possible(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  this_ptr->clear_leading_zero_possible();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_main_country_for_code(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  this_ptr->clear_main_country_for_code();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_mobile(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  this_ptr->clear_mobile();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_mobile_number_portable_region(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  this_ptr->clear_mobile_number_portable_region();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_national_prefix(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  this_ptr->clear_national_prefix();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_national_prefix_for_parsing(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  this_ptr->clear_national_prefix_for_parsing();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_national_prefix_transform_rule(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  this_ptr->clear_national_prefix_transform_rule();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_no_international_dialling(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  this_ptr->clear_no_international_dialling();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_number_format(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  this_ptr->clear_number_format();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_pager(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  this_ptr->clear_pager();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_personal_number(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  this_ptr->clear_personal_number();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_preferred_extn_prefix(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  this_ptr->clear_preferred_extn_prefix();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_preferred_international_prefix(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  this_ptr->clear_preferred_international_prefix();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_premium_rate(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  this_ptr->clear_premium_rate();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_same_mobile_and_fixed_line_pattern(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  this_ptr->clear_same_mobile_and_fixed_line_pattern();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_shared_cost(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  this_ptr->clear_shared_cost();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_short_code(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  this_ptr->clear_short_code();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_standard_rate(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  this_ptr->clear_standard_rate();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_toll_free(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  this_ptr->clear_toll_free();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_uan(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  this_ptr->clear_uan();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_voicemail(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  this_ptr->clear_voicemail();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_voip(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  this_ptr->clear_voip();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_constructor_from(const i18n::phonenumbers::PhoneMetadata* from, i18n::phonenumbers::PhoneMetadata* output) {
  new(output) i18n::phonenumbers::PhoneMetadata(*from);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_constructor_no_args(i18n::phonenumbers::PhoneMetadata* output) {
  new(output) i18n::phonenumbers::PhoneMetadata();
}

int libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_country_code(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->country_code();
}

const i18n::phonenumbers::PhoneMetadata* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_default_instance() {
  return &i18n::phonenumbers::PhoneMetadata::default_instance();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_delete(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  delete this_ptr;
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_destructor(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  libphonenumber_sys_c_call_destructor(this_ptr);
}

const i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_emergency(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return &this_ptr->emergency();
}

const i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_fixed_line(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return &this_ptr->fixed_line();
}

const i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_general_desc(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return &this_ptr->general_desc();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_carrier_specific(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->has_carrier_specific();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_country_code(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->has_country_code();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_emergency(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->has_emergency();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_fixed_line(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->has_fixed_line();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_general_desc(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->has_general_desc();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_id(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->has_id();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_international_prefix(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->has_international_prefix();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_leading_digits(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->has_leading_digits();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_leading_zero_possible(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->has_leading_zero_possible();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_main_country_for_code(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->has_main_country_for_code();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_mobile(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->has_mobile();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_mobile_number_portable_region(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->has_mobile_number_portable_region();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_national_prefix(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->has_national_prefix();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_national_prefix_for_parsing(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->has_national_prefix_for_parsing();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_national_prefix_transform_rule(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->has_national_prefix_transform_rule();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_no_international_dialling(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->has_no_international_dialling();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_pager(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->has_pager();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_personal_number(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->has_personal_number();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_preferred_extn_prefix(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->has_preferred_extn_prefix();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_preferred_international_prefix(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->has_preferred_international_prefix();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_premium_rate(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->has_premium_rate();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_same_mobile_and_fixed_line_pattern(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->has_same_mobile_and_fixed_line_pattern();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_shared_cost(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->has_shared_cost();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_short_code(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->has_short_code();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_standard_rate(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->has_standard_rate();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_toll_free(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->has_toll_free();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_uan(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->has_uan();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_voicemail(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->has_voicemail();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_voip(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->has_voip();
}

const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_id(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return &this_ptr->id();
}

const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_international_prefix(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return &this_ptr->international_prefix();
}

const i18n::phonenumbers::NumberFormat* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_intl_number_format_index(const i18n::phonenumbers::PhoneMetadata* this_ptr, int index) {
  return &this_ptr->intl_number_format(index);
}

const google::protobuf::RepeatedPtrField< i18n::phonenumbers::NumberFormat >* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_intl_number_format_no_args(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return &this_ptr->intl_number_format();
}

int libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_intl_number_format_size(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->intl_number_format_size();
}

const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_leading_digits(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return &this_ptr->leading_digits();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_leading_zero_possible(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->leading_zero_possible();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_main_country_for_code(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->main_country_for_code();
}

const i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mobile(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return &this_ptr->mobile();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mobile_number_portable_region(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->mobile_number_portable_region();
}

i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_carrier_specific(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->mutable_carrier_specific();
}

i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_emergency(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->mutable_emergency();
}

i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_fixed_line(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->mutable_fixed_line();
}

i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_general_desc(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->mutable_general_desc();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_id(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->mutable_id();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_international_prefix(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->mutable_international_prefix();
}

i18n::phonenumbers::NumberFormat* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_intl_number_format_index(i18n::phonenumbers::PhoneMetadata* this_ptr, int index) {
  return this_ptr->mutable_intl_number_format(index);
}

google::protobuf::RepeatedPtrField< i18n::phonenumbers::NumberFormat >* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_intl_number_format_no_args(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->mutable_intl_number_format();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_leading_digits(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->mutable_leading_digits();
}

i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_mobile(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->mutable_mobile();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_national_prefix(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->mutable_national_prefix();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_national_prefix_for_parsing(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->mutable_national_prefix_for_parsing();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_national_prefix_transform_rule(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->mutable_national_prefix_transform_rule();
}

i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_no_international_dialling(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->mutable_no_international_dialling();
}

i18n::phonenumbers::NumberFormat* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_number_format_index(i18n::phonenumbers::PhoneMetadata* this_ptr, int index) {
  return this_ptr->mutable_number_format(index);
}

google::protobuf::RepeatedPtrField< i18n::phonenumbers::NumberFormat >* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_number_format_no_args(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->mutable_number_format();
}

i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_pager(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->mutable_pager();
}

i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_personal_number(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->mutable_personal_number();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_preferred_extn_prefix(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->mutable_preferred_extn_prefix();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_preferred_international_prefix(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->mutable_preferred_international_prefix();
}

i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_premium_rate(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->mutable_premium_rate();
}

i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_shared_cost(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->mutable_shared_cost();
}

i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_short_code(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->mutable_short_code();
}

i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_standard_rate(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->mutable_standard_rate();
}

i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_toll_free(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->mutable_toll_free();
}

i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_uan(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->mutable_uan();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_unknown_fields(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->mutable_unknown_fields();
}

i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_voicemail(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->mutable_voicemail();
}

i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_voip(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->mutable_voip();
}

const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_national_prefix(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return &this_ptr->national_prefix();
}

const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_national_prefix_for_parsing(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return &this_ptr->national_prefix_for_parsing();
}

const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_national_prefix_transform_rule(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return &this_ptr->national_prefix_transform_rule();
}

i18n::phonenumbers::PhoneMetadata* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_new_from(const i18n::phonenumbers::PhoneMetadata* from) {
  return new i18n::phonenumbers::PhoneMetadata(*from);
}

i18n::phonenumbers::PhoneMetadata* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_new_no_args() {
  return new i18n::phonenumbers::PhoneMetadata();
}

const i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_no_international_dialling(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return &this_ptr->no_international_dialling();
}

const i18n::phonenumbers::NumberFormat* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_number_format_index(const i18n::phonenumbers::PhoneMetadata* this_ptr, int index) {
  return &this_ptr->number_format(index);
}

const google::protobuf::RepeatedPtrField< i18n::phonenumbers::NumberFormat >* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_number_format_no_args(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return &this_ptr->number_format();
}

int libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_number_format_size(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->number_format_size();
}

i18n::phonenumbers::PhoneMetadata* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_operator_assign(i18n::phonenumbers::PhoneMetadata* this_ptr, const i18n::phonenumbers::PhoneMetadata* from) {
  return &this_ptr->operator=(*from);
}

const i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_pager(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return &this_ptr->pager();
}

const i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_personal_number(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return &this_ptr->personal_number();
}

const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_preferred_extn_prefix(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return &this_ptr->preferred_extn_prefix();
}

const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_preferred_international_prefix(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return &this_ptr->preferred_international_prefix();
}

const i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_premium_rate(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return &this_ptr->premium_rate();
}

i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_carrier_specific(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->release_carrier_specific();
}

i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_emergency(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->release_emergency();
}

i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_fixed_line(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->release_fixed_line();
}

i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_general_desc(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->release_general_desc();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_id(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->release_id();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_international_prefix(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->release_international_prefix();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_leading_digits(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->release_leading_digits();
}

i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_mobile(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->release_mobile();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_national_prefix(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->release_national_prefix();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_national_prefix_for_parsing(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->release_national_prefix_for_parsing();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_national_prefix_transform_rule(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->release_national_prefix_transform_rule();
}

i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_no_international_dialling(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->release_no_international_dialling();
}

i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_pager(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->release_pager();
}

i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_personal_number(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->release_personal_number();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_preferred_extn_prefix(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->release_preferred_extn_prefix();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_preferred_international_prefix(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->release_preferred_international_prefix();
}

i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_premium_rate(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->release_premium_rate();
}

i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_shared_cost(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->release_shared_cost();
}

i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_short_code(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->release_short_code();
}

i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_standard_rate(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->release_standard_rate();
}

i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_toll_free(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->release_toll_free();
}

i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_uan(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->release_uan();
}

i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_voicemail(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->release_voicemail();
}

i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_voip(i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->release_voip();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_same_mobile_and_fixed_line_pattern(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return this_ptr->same_mobile_and_fixed_line_pattern();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_carrier_specific(i18n::phonenumbers::PhoneMetadata* this_ptr, i18n::phonenumbers::PhoneNumberDesc* carrier_specific) {
  this_ptr->set_allocated_carrier_specific(carrier_specific);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_emergency(i18n::phonenumbers::PhoneMetadata* this_ptr, i18n::phonenumbers::PhoneNumberDesc* emergency) {
  this_ptr->set_allocated_emergency(emergency);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_fixed_line(i18n::phonenumbers::PhoneMetadata* this_ptr, i18n::phonenumbers::PhoneNumberDesc* fixed_line) {
  this_ptr->set_allocated_fixed_line(fixed_line);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_general_desc(i18n::phonenumbers::PhoneMetadata* this_ptr, i18n::phonenumbers::PhoneNumberDesc* general_desc) {
  this_ptr->set_allocated_general_desc(general_desc);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_id(i18n::phonenumbers::PhoneMetadata* this_ptr, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* id) {
  this_ptr->set_allocated_id(id);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_international_prefix(i18n::phonenumbers::PhoneMetadata* this_ptr, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* international_prefix) {
  this_ptr->set_allocated_international_prefix(international_prefix);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_leading_digits(i18n::phonenumbers::PhoneMetadata* this_ptr, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* leading_digits) {
  this_ptr->set_allocated_leading_digits(leading_digits);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_mobile(i18n::phonenumbers::PhoneMetadata* this_ptr, i18n::phonenumbers::PhoneNumberDesc* mobile) {
  this_ptr->set_allocated_mobile(mobile);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_national_prefix(i18n::phonenumbers::PhoneMetadata* this_ptr, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* national_prefix) {
  this_ptr->set_allocated_national_prefix(national_prefix);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_national_prefix_for_parsing(i18n::phonenumbers::PhoneMetadata* this_ptr, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* national_prefix_for_parsing) {
  this_ptr->set_allocated_national_prefix_for_parsing(national_prefix_for_parsing);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_national_prefix_transform_rule(i18n::phonenumbers::PhoneMetadata* this_ptr, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* national_prefix_transform_rule) {
  this_ptr->set_allocated_national_prefix_transform_rule(national_prefix_transform_rule);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_no_international_dialling(i18n::phonenumbers::PhoneMetadata* this_ptr, i18n::phonenumbers::PhoneNumberDesc* no_international_dialling) {
  this_ptr->set_allocated_no_international_dialling(no_international_dialling);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_pager(i18n::phonenumbers::PhoneMetadata* this_ptr, i18n::phonenumbers::PhoneNumberDesc* pager) {
  this_ptr->set_allocated_pager(pager);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_personal_number(i18n::phonenumbers::PhoneMetadata* this_ptr, i18n::phonenumbers::PhoneNumberDesc* personal_number) {
  this_ptr->set_allocated_personal_number(personal_number);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_preferred_extn_prefix(i18n::phonenumbers::PhoneMetadata* this_ptr, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* preferred_extn_prefix) {
  this_ptr->set_allocated_preferred_extn_prefix(preferred_extn_prefix);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_preferred_international_prefix(i18n::phonenumbers::PhoneMetadata* this_ptr, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* preferred_international_prefix) {
  this_ptr->set_allocated_preferred_international_prefix(preferred_international_prefix);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_premium_rate(i18n::phonenumbers::PhoneMetadata* this_ptr, i18n::phonenumbers::PhoneNumberDesc* premium_rate) {
  this_ptr->set_allocated_premium_rate(premium_rate);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_shared_cost(i18n::phonenumbers::PhoneMetadata* this_ptr, i18n::phonenumbers::PhoneNumberDesc* shared_cost) {
  this_ptr->set_allocated_shared_cost(shared_cost);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_short_code(i18n::phonenumbers::PhoneMetadata* this_ptr, i18n::phonenumbers::PhoneNumberDesc* short_code) {
  this_ptr->set_allocated_short_code(short_code);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_standard_rate(i18n::phonenumbers::PhoneMetadata* this_ptr, i18n::phonenumbers::PhoneNumberDesc* standard_rate) {
  this_ptr->set_allocated_standard_rate(standard_rate);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_toll_free(i18n::phonenumbers::PhoneMetadata* this_ptr, i18n::phonenumbers::PhoneNumberDesc* toll_free) {
  this_ptr->set_allocated_toll_free(toll_free);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_uan(i18n::phonenumbers::PhoneMetadata* this_ptr, i18n::phonenumbers::PhoneNumberDesc* uan) {
  this_ptr->set_allocated_uan(uan);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_voicemail(i18n::phonenumbers::PhoneMetadata* this_ptr, i18n::phonenumbers::PhoneNumberDesc* voicemail) {
  this_ptr->set_allocated_voicemail(voicemail);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_voip(i18n::phonenumbers::PhoneMetadata* this_ptr, i18n::phonenumbers::PhoneNumberDesc* voip) {
  this_ptr->set_allocated_voip(voip);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_country_code(i18n::phonenumbers::PhoneMetadata* this_ptr, int value) {
  this_ptr->set_country_code(value);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_id_char(i18n::phonenumbers::PhoneMetadata* this_ptr, const char* value) {
  this_ptr->set_id(value);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_id_char_unsigned_long(i18n::phonenumbers::PhoneMetadata* this_ptr, const char* value, unsigned long size) {
  this_ptr->set_id(value, size);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_id_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(i18n::phonenumbers::PhoneMetadata* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* value) {
  this_ptr->set_id(*value);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_international_prefix_char(i18n::phonenumbers::PhoneMetadata* this_ptr, const char* value) {
  this_ptr->set_international_prefix(value);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_international_prefix_char_unsigned_long(i18n::phonenumbers::PhoneMetadata* this_ptr, const char* value, unsigned long size) {
  this_ptr->set_international_prefix(value, size);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_international_prefix_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(i18n::phonenumbers::PhoneMetadata* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* value) {
  this_ptr->set_international_prefix(*value);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_leading_digits_char(i18n::phonenumbers::PhoneMetadata* this_ptr, const char* value) {
  this_ptr->set_leading_digits(value);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_leading_digits_char_unsigned_long(i18n::phonenumbers::PhoneMetadata* this_ptr, const char* value, unsigned long size) {
  this_ptr->set_leading_digits(value, size);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_leading_digits_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(i18n::phonenumbers::PhoneMetadata* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* value) {
  this_ptr->set_leading_digits(*value);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_leading_zero_possible(i18n::phonenumbers::PhoneMetadata* this_ptr, bool value) {
  this_ptr->set_leading_zero_possible(value);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_main_country_for_code(i18n::phonenumbers::PhoneMetadata* this_ptr, bool value) {
  this_ptr->set_main_country_for_code(value);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_mobile_number_portable_region(i18n::phonenumbers::PhoneMetadata* this_ptr, bool value) {
  this_ptr->set_mobile_number_portable_region(value);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_national_prefix_char(i18n::phonenumbers::PhoneMetadata* this_ptr, const char* value) {
  this_ptr->set_national_prefix(value);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_national_prefix_char_unsigned_long(i18n::phonenumbers::PhoneMetadata* this_ptr, const char* value, unsigned long size) {
  this_ptr->set_national_prefix(value, size);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_national_prefix_for_parsing_char(i18n::phonenumbers::PhoneMetadata* this_ptr, const char* value) {
  this_ptr->set_national_prefix_for_parsing(value);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_national_prefix_for_parsing_char_unsigned_long(i18n::phonenumbers::PhoneMetadata* this_ptr, const char* value, unsigned long size) {
  this_ptr->set_national_prefix_for_parsing(value, size);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_national_prefix_for_parsing_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(i18n::phonenumbers::PhoneMetadata* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* value) {
  this_ptr->set_national_prefix_for_parsing(*value);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_national_prefix_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(i18n::phonenumbers::PhoneMetadata* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* value) {
  this_ptr->set_national_prefix(*value);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_national_prefix_transform_rule_char(i18n::phonenumbers::PhoneMetadata* this_ptr, const char* value) {
  this_ptr->set_national_prefix_transform_rule(value);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_national_prefix_transform_rule_char_unsigned_long(i18n::phonenumbers::PhoneMetadata* this_ptr, const char* value, unsigned long size) {
  this_ptr->set_national_prefix_transform_rule(value, size);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_national_prefix_transform_rule_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(i18n::phonenumbers::PhoneMetadata* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* value) {
  this_ptr->set_national_prefix_transform_rule(*value);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_preferred_extn_prefix_char(i18n::phonenumbers::PhoneMetadata* this_ptr, const char* value) {
  this_ptr->set_preferred_extn_prefix(value);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_preferred_extn_prefix_char_unsigned_long(i18n::phonenumbers::PhoneMetadata* this_ptr, const char* value, unsigned long size) {
  this_ptr->set_preferred_extn_prefix(value, size);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_preferred_extn_prefix_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(i18n::phonenumbers::PhoneMetadata* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* value) {
  this_ptr->set_preferred_extn_prefix(*value);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_preferred_international_prefix_char(i18n::phonenumbers::PhoneMetadata* this_ptr, const char* value) {
  this_ptr->set_preferred_international_prefix(value);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_preferred_international_prefix_char_unsigned_long(i18n::phonenumbers::PhoneMetadata* this_ptr, const char* value, unsigned long size) {
  this_ptr->set_preferred_international_prefix(value, size);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_preferred_international_prefix_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(i18n::phonenumbers::PhoneMetadata* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* value) {
  this_ptr->set_preferred_international_prefix(*value);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_same_mobile_and_fixed_line_pattern(i18n::phonenumbers::PhoneMetadata* this_ptr, bool value) {
  this_ptr->set_same_mobile_and_fixed_line_pattern(value);
}

const i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_shared_cost(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return &this_ptr->shared_cost();
}

const i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_short_code(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return &this_ptr->short_code();
}

const i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_standard_rate(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return &this_ptr->standard_rate();
}

const i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_toll_free(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return &this_ptr->toll_free();
}

const i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_uan(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return &this_ptr->uan();
}

const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_unknown_fields(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return &this_ptr->unknown_fields();
}

const i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_voicemail(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return &this_ptr->voicemail();
}

const i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_voip(const i18n::phonenumbers::PhoneMetadata* this_ptr) {
  return &this_ptr->voip();
}

int libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_ByteSize(const i18n::phonenumbers::PhoneNumberDesc* this_ptr) {
  return this_ptr->ByteSize();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_CheckTypeAndMergeFrom(i18n::phonenumbers::PhoneNumberDesc* this_ptr, const google::protobuf::MessageLite* from) {
  this_ptr->CheckTypeAndMergeFrom(*from);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_Clear(i18n::phonenumbers::PhoneNumberDesc* this_ptr) {
  this_ptr->Clear();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_CopyFrom(i18n::phonenumbers::PhoneNumberDesc* this_ptr, const i18n::phonenumbers::PhoneNumberDesc* from) {
  this_ptr->CopyFrom(*from);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_DiscardUnknownFields(i18n::phonenumbers::PhoneNumberDesc* this_ptr) {
  this_ptr->DiscardUnknownFields();
}

int libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_GetCachedSize(const i18n::phonenumbers::PhoneNumberDesc* this_ptr) {
  return this_ptr->GetCachedSize();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_GetTypeName_as_ptr(const i18n::phonenumbers::PhoneNumberDesc* this_ptr) {
  return new std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >(this_ptr->GetTypeName());
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_GetTypeName_to_output(const i18n::phonenumbers::PhoneNumberDesc* this_ptr, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* output) {
  new(output) std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >(this_ptr->GetTypeName());
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_IsInitialized(const i18n::phonenumbers::PhoneNumberDesc* this_ptr) {
  return this_ptr->IsInitialized();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_MergeFrom(i18n::phonenumbers::PhoneNumberDesc* this_ptr, const i18n::phonenumbers::PhoneNumberDesc* from) {
  this_ptr->MergeFrom(*from);
}

i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_New_arena(const i18n::phonenumbers::PhoneNumberDesc* this_ptr, google::protobuf::Arena* arena) {
  return this_ptr->New(arena);
}

i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_New_no_args(const i18n::phonenumbers::PhoneNumberDesc* this_ptr) {
  return this_ptr->New();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_Swap(i18n::phonenumbers::PhoneNumberDesc* this_ptr, i18n::phonenumbers::PhoneNumberDesc* other) {
  this_ptr->Swap(other);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_add_possible_length(i18n::phonenumbers::PhoneNumberDesc* this_ptr, int value) {
  this_ptr->add_possible_length(value);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_add_possible_length_local_only(i18n::phonenumbers::PhoneNumberDesc* this_ptr, int value) {
  this_ptr->add_possible_length_local_only(value);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_clear_example_number(i18n::phonenumbers::PhoneNumberDesc* this_ptr) {
  this_ptr->clear_example_number();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_clear_national_number_pattern(i18n::phonenumbers::PhoneNumberDesc* this_ptr) {
  this_ptr->clear_national_number_pattern();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_clear_possible_length(i18n::phonenumbers::PhoneNumberDesc* this_ptr) {
  this_ptr->clear_possible_length();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_clear_possible_length_local_only(i18n::phonenumbers::PhoneNumberDesc* this_ptr) {
  this_ptr->clear_possible_length_local_only();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_clear_possible_number_pattern(i18n::phonenumbers::PhoneNumberDesc* this_ptr) {
  this_ptr->clear_possible_number_pattern();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_constructor_from(const i18n::phonenumbers::PhoneNumberDesc* from, i18n::phonenumbers::PhoneNumberDesc* output) {
  new(output) i18n::phonenumbers::PhoneNumberDesc(*from);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_constructor_no_args(i18n::phonenumbers::PhoneNumberDesc* output) {
  new(output) i18n::phonenumbers::PhoneNumberDesc();
}

const i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_default_instance() {
  return &i18n::phonenumbers::PhoneNumberDesc::default_instance();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_delete(i18n::phonenumbers::PhoneNumberDesc* this_ptr) {
  delete this_ptr;
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_destructor(i18n::phonenumbers::PhoneNumberDesc* this_ptr) {
  libphonenumber_sys_c_call_destructor(this_ptr);
}

const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_example_number(const i18n::phonenumbers::PhoneNumberDesc* this_ptr) {
  return &this_ptr->example_number();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_has_example_number(const i18n::phonenumbers::PhoneNumberDesc* this_ptr) {
  return this_ptr->has_example_number();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_has_national_number_pattern(const i18n::phonenumbers::PhoneNumberDesc* this_ptr) {
  return this_ptr->has_national_number_pattern();
}

bool libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_has_possible_number_pattern(const i18n::phonenumbers::PhoneNumberDesc* this_ptr) {
  return this_ptr->has_possible_number_pattern();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_mutable_example_number(i18n::phonenumbers::PhoneNumberDesc* this_ptr) {
  return this_ptr->mutable_example_number();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_mutable_national_number_pattern(i18n::phonenumbers::PhoneNumberDesc* this_ptr) {
  return this_ptr->mutable_national_number_pattern();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_mutable_possible_number_pattern(i18n::phonenumbers::PhoneNumberDesc* this_ptr) {
  return this_ptr->mutable_possible_number_pattern();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_mutable_unknown_fields(i18n::phonenumbers::PhoneNumberDesc* this_ptr) {
  return this_ptr->mutable_unknown_fields();
}

const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_national_number_pattern(const i18n::phonenumbers::PhoneNumberDesc* this_ptr) {
  return &this_ptr->national_number_pattern();
}

i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_new_from(const i18n::phonenumbers::PhoneNumberDesc* from) {
  return new i18n::phonenumbers::PhoneNumberDesc(*from);
}

i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_new_no_args() {
  return new i18n::phonenumbers::PhoneNumberDesc();
}

i18n::phonenumbers::PhoneNumberDesc* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_operator_assign(i18n::phonenumbers::PhoneNumberDesc* this_ptr, const i18n::phonenumbers::PhoneNumberDesc* from) {
  return &this_ptr->operator=(*from);
}

int libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_possible_length_local_only_size(const i18n::phonenumbers::PhoneNumberDesc* this_ptr) {
  return this_ptr->possible_length_local_only_size();
}

int libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_possible_length_size(const i18n::phonenumbers::PhoneNumberDesc* this_ptr) {
  return this_ptr->possible_length_size();
}

const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_possible_number_pattern(const i18n::phonenumbers::PhoneNumberDesc* this_ptr) {
  return &this_ptr->possible_number_pattern();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_release_example_number(i18n::phonenumbers::PhoneNumberDesc* this_ptr) {
  return this_ptr->release_example_number();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_release_national_number_pattern(i18n::phonenumbers::PhoneNumberDesc* this_ptr) {
  return this_ptr->release_national_number_pattern();
}

std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_release_possible_number_pattern(i18n::phonenumbers::PhoneNumberDesc* this_ptr) {
  return this_ptr->release_possible_number_pattern();
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_set_allocated_example_number(i18n::phonenumbers::PhoneNumberDesc* this_ptr, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* example_number) {
  this_ptr->set_allocated_example_number(example_number);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_set_allocated_national_number_pattern(i18n::phonenumbers::PhoneNumberDesc* this_ptr, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* national_number_pattern) {
  this_ptr->set_allocated_national_number_pattern(national_number_pattern);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_set_allocated_possible_number_pattern(i18n::phonenumbers::PhoneNumberDesc* this_ptr, std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* possible_number_pattern) {
  this_ptr->set_allocated_possible_number_pattern(possible_number_pattern);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_set_example_number_char(i18n::phonenumbers::PhoneNumberDesc* this_ptr, const char* value) {
  this_ptr->set_example_number(value);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_set_example_number_char_unsigned_long(i18n::phonenumbers::PhoneNumberDesc* this_ptr, const char* value, unsigned long size) {
  this_ptr->set_example_number(value, size);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_set_example_number_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(i18n::phonenumbers::PhoneNumberDesc* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* value) {
  this_ptr->set_example_number(*value);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_set_national_number_pattern_char(i18n::phonenumbers::PhoneNumberDesc* this_ptr, const char* value) {
  this_ptr->set_national_number_pattern(value);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_set_national_number_pattern_char_unsigned_long(i18n::phonenumbers::PhoneNumberDesc* this_ptr, const char* value, unsigned long size) {
  this_ptr->set_national_number_pattern(value, size);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_set_national_number_pattern_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(i18n::phonenumbers::PhoneNumberDesc* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* value) {
  this_ptr->set_national_number_pattern(*value);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_set_possible_length(i18n::phonenumbers::PhoneNumberDesc* this_ptr, int index, int value) {
  this_ptr->set_possible_length(index, value);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_set_possible_length_local_only(i18n::phonenumbers::PhoneNumberDesc* this_ptr, int index, int value) {
  this_ptr->set_possible_length_local_only(index, value);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_set_possible_number_pattern_char(i18n::phonenumbers::PhoneNumberDesc* this_ptr, const char* value) {
  this_ptr->set_possible_number_pattern(value);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_set_possible_number_pattern_char_unsigned_long(i18n::phonenumbers::PhoneNumberDesc* this_ptr, const char* value, unsigned long size) {
  this_ptr->set_possible_number_pattern(value, size);
}

void libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_set_possible_number_pattern_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(i18n::phonenumbers::PhoneNumberDesc* this_ptr, const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* value) {
  this_ptr->set_possible_number_pattern(*value);
}

const std::__cxx11::basic_string< char, std::char_traits< char >, std::allocator< char > >* libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_unknown_fields(const i18n::phonenumbers::PhoneNumberDesc* this_ptr) {
  return &this_ptr->unknown_fields();
}

void libphonenumber_sys_c_phonemetadata_G_i18n_phonenumbers_protobuf_AddDesc_phonemetadata_2eproto() {
  i18n::phonenumbers::protobuf_AddDesc_phonemetadata_2eproto();
}

void libphonenumber_sys_c_phonemetadata_G_i18n_phonenumbers_protobuf_AssignDesc_phonemetadata_2eproto() {
  i18n::phonenumbers::protobuf_AssignDesc_phonemetadata_2eproto();
}

void libphonenumber_sys_c_phonemetadata_G_i18n_phonenumbers_protobuf_ShutdownFile_phonemetadata_2eproto() {
  i18n::phonenumbers::protobuf_ShutdownFile_phonemetadata_2eproto();
}

