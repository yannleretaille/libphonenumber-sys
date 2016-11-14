use libc;

#[link(name = "phonenumber")]
#[link(name = "protobuf")]
#[link(name = "stdc++")]
#[link(name = "libphonenumber_sys_c", kind = "static")]
extern "C" {
  // Header: asyoutypeformatter.h
  pub fn libphonenumber_sys_c_i18n_phonenumbers_AsYouTypeFormatter_Clear(this_ptr: *mut ::asyoutypeformatter::i18n::phonenumbers::AsYouTypeFormatter);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_AsYouTypeFormatter_GetRememberedPosition(this_ptr: *const ::asyoutypeformatter::i18n::phonenumbers::AsYouTypeFormatter) -> libc::c_int;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_AsYouTypeFormatter_InputDigit
    (this_ptr: *mut ::asyoutypeformatter::i18n::phonenumbers::AsYouTypeFormatter,
     next_char: libc::c_int,
     result: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef)
     -> *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_AsYouTypeFormatter_InputDigitAndRememberPosition
    (this_ptr: *mut ::asyoutypeformatter::i18n::phonenumbers::AsYouTypeFormatter,
     next_char: libc::c_int,
     result: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef)
     -> *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_AsYouTypeFormatter_delete(this_ptr: *mut ::asyoutypeformatter::i18n::phonenumbers::AsYouTypeFormatter);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_AsYouTypeFormatter_destructor(this_ptr: *mut ::asyoutypeformatter::i18n::phonenumbers::AsYouTypeFormatter);

  // Header: lock.h
  pub fn libphonenumber_sys_c_i18n_phonenumbers_AutoLock_constructor(lock: *mut ::lock_posix::i18n::phonenumbers::Lock, output: *mut ::lock::i18n::phonenumbers::AutoLock);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_AutoLock_delete(this_ptr: *mut ::lock::i18n::phonenumbers::AutoLock);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_AutoLock_destructor(this_ptr: *mut ::lock::i18n::phonenumbers::AutoLock);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_AutoLock_new(lock: *mut ::lock_posix::i18n::phonenumbers::Lock)
                                                             -> *mut ::lock::i18n::phonenumbers::AutoLock;

  // Header: lock_posix.h
  pub fn libphonenumber_sys_c_i18n_phonenumbers_Lock_Acquire(this_ptr: *const ::lock_posix::i18n::phonenumbers::Lock);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_Lock_Release(this_ptr: *const ::lock_posix::i18n::phonenumbers::Lock);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_Lock_constructor(output: *mut ::lock_posix::i18n::phonenumbers::Lock);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_Lock_delete(this_ptr: *mut ::lock_posix::i18n::phonenumbers::Lock);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_Lock_destructor(this_ptr: *mut ::lock_posix::i18n::phonenumbers::Lock);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_Lock_new() -> *mut ::lock_posix::i18n::phonenumbers::Lock;

  // Header: phonemetadata.pb.h
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_ByteSize(this_ptr: *const ::phonemetadata::i18n::phonenumbers::NumberFormat) -> libc::c_int;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_CheckTypeAndMergeFrom(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, from: *const ::message_lite::google::protobuf::MessageLite);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_Clear(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_CopyFrom(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, from: *const ::phonemetadata::i18n::phonenumbers::NumberFormat);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_DiscardUnknownFields(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_GetCachedSize(this_ptr: *const ::phonemetadata::i18n::phonenumbers::NumberFormat) -> libc::c_int;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_GetTypeName_as_ptr
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::NumberFormat)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_GetTypeName_to_output(this_ptr: *const ::phonemetadata::i18n::phonenumbers::NumberFormat, output: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_IsInitialized(this_ptr: *const ::phonemetadata::i18n::phonenumbers::NumberFormat) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_MergeFrom(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, from: *const ::phonemetadata::i18n::phonenumbers::NumberFormat);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_New_arena
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::NumberFormat,
     arena: *mut ::arena::google::protobuf::Arena)
     -> *mut ::phonemetadata::i18n::phonenumbers::NumberFormat;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_New_no_args
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::NumberFormat)
     -> *mut ::phonemetadata::i18n::phonenumbers::NumberFormat;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_Swap(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, other: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_add_leading_digits_pattern_char(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, value: *const libc::c_char);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_add_leading_digits_pattern_char_unsigned_long(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, value: *const libc::c_char, size: libc::c_ulong);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_add_leading_digits_pattern_no_args
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_add_leading_digits_pattern_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat,
     value: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_clear_domestic_carrier_code_formatting_rule(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_clear_format(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_clear_leading_digits_pattern(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_clear_national_prefix_formatting_rule(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_clear_national_prefix_optional_when_formatting(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_clear_pattern(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_constructor_from(from: *const ::phonemetadata::i18n::phonenumbers::NumberFormat, output: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_constructor_no_args(output: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_default_instance
    ()
      -> *const ::phonemetadata::i18n::phonenumbers::NumberFormat;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_delete(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_destructor(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_domestic_carrier_code_formatting_rule
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::NumberFormat)
     -> *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_format
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::NumberFormat)
     -> *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_has_domestic_carrier_code_formatting_rule(this_ptr: *const ::phonemetadata::i18n::phonenumbers::NumberFormat) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_has_format(this_ptr: *const ::phonemetadata::i18n::phonenumbers::NumberFormat) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_has_national_prefix_formatting_rule(this_ptr: *const ::phonemetadata::i18n::phonenumbers::NumberFormat) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_has_national_prefix_optional_when_formatting(this_ptr: *const ::phonemetadata::i18n::phonenumbers::NumberFormat) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_has_pattern(this_ptr: *const ::phonemetadata::i18n::phonenumbers::NumberFormat) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_leading_digits_pattern_index
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::NumberFormat,
     index: libc::c_int)
     -> *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_leading_digits_pattern_no_args(this_ptr: *const ::phonemetadata::i18n::phonenumbers::NumberFormat) -> *const ::repeated_field::google::protobuf::RepeatedPtrFieldBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_leading_digits_pattern_size(this_ptr: *const ::phonemetadata::i18n::phonenumbers::NumberFormat) -> libc::c_int;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_mutable_domestic_carrier_code_formatting_rule
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_mutable_format
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_mutable_leading_digits_pattern_index
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat,
     index: libc::c_int)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_mutable_leading_digits_pattern_no_args
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat)
     -> *mut ::repeated_field::google::protobuf::RepeatedPtrFieldBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_mutable_national_prefix_formatting_rule
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_mutable_pattern
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_mutable_unknown_fields
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_national_prefix_formatting_rule
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::NumberFormat)
     -> *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_national_prefix_optional_when_formatting(this_ptr: *const ::phonemetadata::i18n::phonenumbers::NumberFormat) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_new_from
    (from: *const ::phonemetadata::i18n::phonenumbers::NumberFormat)
     -> *mut ::phonemetadata::i18n::phonenumbers::NumberFormat;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_new_no_args
    ()
      -> *mut ::phonemetadata::i18n::phonenumbers::NumberFormat;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_operator_assign
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat,
     from: *const ::phonemetadata::i18n::phonenumbers::NumberFormat)
     -> *mut ::phonemetadata::i18n::phonenumbers::NumberFormat;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_pattern
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::NumberFormat)
     -> *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_release_domestic_carrier_code_formatting_rule
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_release_format
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_release_national_prefix_formatting_rule
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_release_pattern
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_allocated_domestic_carrier_code_formatting_rule(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, domestic_carrier_code_formatting_rule: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_allocated_format(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, format: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_allocated_national_prefix_formatting_rule(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, national_prefix_formatting_rule: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_allocated_pattern(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, pattern: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_domestic_carrier_code_formatting_rule_char(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, value: *const libc::c_char);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_domestic_carrier_code_formatting_rule_char_unsigned_long
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat,
     value: *const libc::c_char,
     size: libc::c_ulong);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_domestic_carrier_code_formatting_rule_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat,
     value: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_format_char(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, value: *const libc::c_char);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_format_char_unsigned_long(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, value: *const libc::c_char, size: libc::c_ulong);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_format_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat,
     value: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_leading_digits_pattern_int_char(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, index: libc::c_int, value: *const libc::c_char);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_leading_digits_pattern_int_char_unsigned_long(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, index: libc::c_int, value: *const libc::c_char, size: libc::c_ulong);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_leading_digits_pattern_int_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat,
     index: libc::c_int,
     value: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_national_prefix_formatting_rule_char(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, value: *const libc::c_char);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_national_prefix_formatting_rule_char_unsigned_long
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat,
     value: *const libc::c_char,
     size: libc::c_ulong);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_national_prefix_formatting_rule_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat,
     value: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_national_prefix_optional_when_formatting(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, value: bool);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_pattern_char(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, value: *const libc::c_char);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_pattern_char_unsigned_long(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat, value: *const libc::c_char, size: libc::c_ulong);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_set_pattern_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::NumberFormat,
     value: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_NumberFormat_unknown_fields
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::NumberFormat)
     -> *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_ByteSize(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection) -> libc::c_int;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_CheckTypeAndMergeFrom(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection, from: *const ::message_lite::google::protobuf::MessageLite);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_Clear(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_CopyFrom(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection, from: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_DiscardUnknownFields(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_GetCachedSize(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection) -> libc::c_int;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_GetTypeName_as_ptr
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_GetTypeName_to_output(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection, output: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_IsInitialized(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_MergeFrom(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection, from: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_New_arena
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection,
     arena: *mut ::arena::google::protobuf::Arena)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_New_no_args
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_Swap(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection, other: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_add_metadata
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_clear_metadata(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_constructor_from(from: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection, output: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_constructor_no_args(output: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_default_instance
    ()
      -> *const ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_delete(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_destructor(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_metadata_index
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection,
     index: libc::c_int)
     -> *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_metadata_no_args
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection)
     -> *const ::repeated_field::google::protobuf::RepeatedPtrFieldPhoneMetadataRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_metadata_size(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection) -> libc::c_int;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_mutable_metadata_index
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection,
     index: libc::c_int)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_mutable_metadata_no_args
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection)
     -> *mut ::repeated_field::google::protobuf::RepeatedPtrFieldPhoneMetadataRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_mutable_unknown_fields
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_new_from
    (from: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_new_no_args
    ()
      -> *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_operator_assign
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection,
     from: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadataCollection_unknown_fields
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadataCollection)
     -> *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_ByteSize(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> libc::c_int;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_CheckTypeAndMergeFrom(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, from: *const ::message_lite::google::protobuf::MessageLite);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_Clear(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_CopyFrom(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, from: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_DiscardUnknownFields(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_GetCachedSize(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> libc::c_int;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_GetTypeName_as_ptr
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_GetTypeName_to_output(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata, output: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_IsInitialized(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_MergeFrom(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, from: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_New_arena
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata,
     arena: *mut ::arena::google::protobuf::Arena)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_New_no_args
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_Swap(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, other: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_add_intl_number_format
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::phonemetadata::i18n::phonenumbers::NumberFormat;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_add_number_format
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::phonemetadata::i18n::phonenumbers::NumberFormat;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_carrier_specific
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_carrier_specific(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_country_code(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_emergency(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_fixed_line(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_general_desc(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_id(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_international_prefix(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_intl_number_format(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_leading_digits(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_leading_zero_possible(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_main_country_for_code(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_mobile(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_mobile_number_portable_region(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_national_prefix(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_national_prefix_for_parsing(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_national_prefix_transform_rule(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_no_international_dialling(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_number_format(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_pager(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_personal_number(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_preferred_extn_prefix(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_preferred_international_prefix(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_premium_rate(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_same_mobile_and_fixed_line_pattern(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_shared_cost(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_short_code(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_standard_rate(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_toll_free(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_uan(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_voicemail(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_clear_voip(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_constructor_from(from: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata, output: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_constructor_no_args(output: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_country_code(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> libc::c_int;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_default_instance
    ()
      -> *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_delete(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_destructor(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_emergency
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_fixed_line
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_general_desc
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_carrier_specific(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_country_code(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_emergency(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_fixed_line(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_general_desc(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_id(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_international_prefix(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_leading_digits(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_leading_zero_possible(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_main_country_for_code(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_mobile(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_mobile_number_portable_region(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_national_prefix(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_national_prefix_for_parsing(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_national_prefix_transform_rule(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_no_international_dialling(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_pager(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_personal_number(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_preferred_extn_prefix(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_preferred_international_prefix(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_premium_rate(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_same_mobile_and_fixed_line_pattern(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_shared_cost(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_short_code(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_standard_rate(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_toll_free(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_uan(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_voicemail(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_has_voip(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_id
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_international_prefix
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_intl_number_format_index
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata,
     index: libc::c_int)
     -> *const ::phonemetadata::i18n::phonenumbers::NumberFormat;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_intl_number_format_no_args
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *const ::repeated_field::google::protobuf::RepeatedPtrFieldNumberFormatRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_intl_number_format_size(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> libc::c_int;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_leading_digits
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_leading_zero_possible(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_main_country_for_code(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mobile
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mobile_number_portable_region(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_carrier_specific
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_emergency
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_fixed_line
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_general_desc
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_id
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_international_prefix
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_intl_number_format_index
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata,
     index: libc::c_int)
     -> *mut ::phonemetadata::i18n::phonenumbers::NumberFormat;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_intl_number_format_no_args
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::repeated_field::google::protobuf::RepeatedPtrFieldNumberFormatRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_leading_digits
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_mobile
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_national_prefix
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_national_prefix_for_parsing
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_national_prefix_transform_rule
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_no_international_dialling
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_number_format_index
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata,
     index: libc::c_int)
     -> *mut ::phonemetadata::i18n::phonenumbers::NumberFormat;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_number_format_no_args
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::repeated_field::google::protobuf::RepeatedPtrFieldNumberFormatRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_pager
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_personal_number
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_preferred_extn_prefix
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_preferred_international_prefix
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_premium_rate
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_shared_cost
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_short_code
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_standard_rate
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_toll_free
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_uan
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_unknown_fields
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_voicemail
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_mutable_voip
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_national_prefix
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_national_prefix_for_parsing
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_national_prefix_transform_rule
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_new_from
    (from: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_new_no_args
    ()
      -> *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_no_international_dialling
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_number_format_index
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata,
     index: libc::c_int)
     -> *const ::phonemetadata::i18n::phonenumbers::NumberFormat;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_number_format_no_args
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *const ::repeated_field::google::protobuf::RepeatedPtrFieldNumberFormatRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_number_format_size(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> libc::c_int;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_operator_assign
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata,
     from: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_pager
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_personal_number
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_preferred_extn_prefix
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_preferred_international_prefix
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_premium_rate
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_carrier_specific
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_emergency
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_fixed_line
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_general_desc
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_id
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_international_prefix
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_leading_digits
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_mobile
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_national_prefix
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_national_prefix_for_parsing
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_national_prefix_transform_rule
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_no_international_dialling
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_pager
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_personal_number
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_preferred_extn_prefix
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_preferred_international_prefix
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_premium_rate
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_shared_cost
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_short_code
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_standard_rate
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_toll_free
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_uan
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_voicemail
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_release_voip
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_same_mobile_and_fixed_line_pattern(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_carrier_specific(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, carrier_specific: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_emergency(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, emergency: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_fixed_line(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, fixed_line: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_general_desc(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, general_desc: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_id(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, id: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_international_prefix(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, international_prefix: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_leading_digits(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, leading_digits: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_mobile(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, mobile: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_national_prefix(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, national_prefix: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_national_prefix_for_parsing(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, national_prefix_for_parsing: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_national_prefix_transform_rule(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, national_prefix_transform_rule: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_no_international_dialling(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, no_international_dialling: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_pager(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, pager: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_personal_number(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, personal_number: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_preferred_extn_prefix(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, preferred_extn_prefix: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_preferred_international_prefix(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, preferred_international_prefix: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_premium_rate(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, premium_rate: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_shared_cost(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, shared_cost: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_short_code(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, short_code: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_standard_rate(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, standard_rate: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_toll_free(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, toll_free: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_uan(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, uan: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_voicemail(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, voicemail: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_allocated_voip(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, voip: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_country_code(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value: libc::c_int);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_id_char(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value: *const libc::c_char);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_id_char_unsigned_long(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value: *const libc::c_char, size: libc::c_ulong);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_id_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata,
     value: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_international_prefix_char(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value: *const libc::c_char);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_international_prefix_char_unsigned_long(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value: *const libc::c_char, size: libc::c_ulong);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_international_prefix_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata,
     value: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_leading_digits_char(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value: *const libc::c_char);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_leading_digits_char_unsigned_long(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value: *const libc::c_char, size: libc::c_ulong);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_leading_digits_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata,
     value: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_leading_zero_possible(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value: bool);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_main_country_for_code(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value: bool);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_mobile_number_portable_region(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value: bool);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_national_prefix_char(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value: *const libc::c_char);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_national_prefix_char_unsigned_long(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value: *const libc::c_char, size: libc::c_ulong);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_national_prefix_for_parsing_char(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value: *const libc::c_char);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_national_prefix_for_parsing_char_unsigned_long(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value: *const libc::c_char, size: libc::c_ulong);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_national_prefix_for_parsing_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata,
     value: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_national_prefix_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata,
     value: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_national_prefix_transform_rule_char(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value: *const libc::c_char);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_national_prefix_transform_rule_char_unsigned_long
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata,
     value: *const libc::c_char,
     size: libc::c_ulong);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_national_prefix_transform_rule_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata,
     value: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_preferred_extn_prefix_char(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value: *const libc::c_char);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_preferred_extn_prefix_char_unsigned_long(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value: *const libc::c_char, size: libc::c_ulong);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_preferred_extn_prefix_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata,
     value: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_preferred_international_prefix_char(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value: *const libc::c_char);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_preferred_international_prefix_char_unsigned_long
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata,
     value: *const libc::c_char,
     size: libc::c_ulong);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_preferred_international_prefix_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata,
     value: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_set_same_mobile_and_fixed_line_pattern(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneMetadata, value: bool);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_shared_cost
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_short_code
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_standard_rate
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_toll_free
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_uan
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_unknown_fields
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_voicemail
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneMetadata_voip
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneMetadata)
     -> *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_ByteSize(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) -> libc::c_int;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_CheckTypeAndMergeFrom(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, from: *const ::message_lite::google::protobuf::MessageLite);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_Clear(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_CopyFrom(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, from: *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_DiscardUnknownFields(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_GetCachedSize(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) -> libc::c_int;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_GetTypeName_as_ptr
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_GetTypeName_to_output(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, output: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_IsInitialized(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_MergeFrom(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, from: *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_New_arena
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc,
     arena: *mut ::arena::google::protobuf::Arena)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_New_no_args
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_Swap(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, other: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_add_possible_length(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, value: libc::c_int);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_add_possible_length_local_only(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, value: libc::c_int);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_clear_example_number(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_clear_national_number_pattern(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_clear_possible_length(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_clear_possible_length_local_only(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_clear_possible_number_pattern(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_constructor_from(from: *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, output: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_constructor_no_args(output: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_default_instance
    ()
      -> *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_delete(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_destructor(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_example_number
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc)
     -> *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_has_example_number(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_has_national_number_pattern(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_has_possible_number_pattern(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_mutable_example_number
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_mutable_national_number_pattern
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_mutable_possible_number_pattern
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_mutable_unknown_fields
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_national_number_pattern
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc)
     -> *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_new_from
    (from: *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_new_no_args
    ()
      -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_operator_assign
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc,
     from: *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc)
     -> *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_possible_length_local_only_size(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) -> libc::c_int;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_possible_length_size(this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc) -> libc::c_int;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_possible_number_pattern
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc)
     -> *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_release_example_number
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_release_national_number_pattern
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_release_possible_number_pattern
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_set_allocated_example_number(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, example_number: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_set_allocated_national_number_pattern(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, national_number_pattern: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_set_allocated_possible_number_pattern(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, possible_number_pattern: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_set_example_number_char(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, value: *const libc::c_char);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_set_example_number_char_unsigned_long(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, value: *const libc::c_char, size: libc::c_ulong);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_set_example_number_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc,
     value: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_set_national_number_pattern_char(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, value: *const libc::c_char);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_set_national_number_pattern_char_unsigned_long(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, value: *const libc::c_char, size: libc::c_ulong);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_set_national_number_pattern_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc,
     value: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_set_possible_length(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, index: libc::c_int, value: libc::c_int);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_set_possible_length_local_only(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, index: libc::c_int, value: libc::c_int);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_set_possible_number_pattern_char(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, value: *const libc::c_char);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_set_possible_number_pattern_char_unsigned_long(this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc, value: *const libc::c_char, size: libc::c_ulong);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_set_possible_number_pattern_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char
    (this_ptr: *mut ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc,
     value: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberDesc_unknown_fields
    (this_ptr: *const ::phonemetadata::i18n::phonenumbers::PhoneNumberDesc)
     -> *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_phonemetadata_G_i18n_phonenumbers_protobuf_AddDesc_phonemetadata_2eproto();
  pub fn libphonenumber_sys_c_phonemetadata_G_i18n_phonenumbers_protobuf_AssignDesc_phonemetadata_2eproto();
  pub fn libphonenumber_sys_c_phonemetadata_G_i18n_phonenumbers_protobuf_ShutdownFile_phonemetadata_2eproto();

  // Header: phonenumber.pb.h
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_ByteSize(this_ptr: *const ::phonenumber::i18n::phonenumbers::PhoneNumber) -> libc::c_int;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_CheckTypeAndMergeFrom(this_ptr: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber, from: *const ::message_lite::google::protobuf::MessageLite);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_Clear(this_ptr: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_CopyFrom(this_ptr: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber, from: *const ::phonenumber::i18n::phonenumbers::PhoneNumber);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_CountryCodeSource_IsValid(value: libc::c_int) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_GetCachedSize(this_ptr: *const ::phonenumber::i18n::phonenumbers::PhoneNumber) -> libc::c_int;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_GetTypeName_as_ptr
    (this_ptr: *const ::phonenumber::i18n::phonenumbers::PhoneNumber)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_GetTypeName_to_output(this_ptr: *const ::phonenumber::i18n::phonenumbers::PhoneNumber, output: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_IsInitialized(this_ptr: *const ::phonenumber::i18n::phonenumbers::PhoneNumber) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_MergeFrom(this_ptr: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber, from: *const ::phonenumber::i18n::phonenumbers::PhoneNumber);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_New_arena
    (this_ptr: *const ::phonenumber::i18n::phonenumbers::PhoneNumber,
     arena: *mut ::arena::google::protobuf::Arena)
     -> *mut ::phonenumber::i18n::phonenumbers::PhoneNumber;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_New_no_args
    (this_ptr: *const ::phonenumber::i18n::phonenumbers::PhoneNumber)
     -> *mut ::phonenumber::i18n::phonenumbers::PhoneNumber;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_Swap(this_ptr: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber, other: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_clear_country_code(this_ptr: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_clear_country_code_source(this_ptr: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_clear_extension(this_ptr: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_clear_italian_leading_zero(this_ptr: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_clear_national_number(this_ptr: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_clear_number_of_leading_zeros(this_ptr: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_clear_preferred_domestic_carrier_code(this_ptr: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_clear_raw_input(this_ptr: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_constructor_from(from: *const ::phonenumber::i18n::phonenumbers::PhoneNumber, output: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_constructor_no_args(output: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_country_code(this_ptr: *const ::phonenumber::i18n::phonenumbers::PhoneNumber) -> libc::c_int;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_country_code_source
    (this_ptr: *const ::phonenumber::i18n::phonenumbers::PhoneNumber)
     -> ::phonenumber::i18n::phonenumbers::PhoneNumberCountryCodeSource;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_default_instance
    ()
      -> *const ::phonenumber::i18n::phonenumbers::PhoneNumber;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_delete(this_ptr: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_destructor(this_ptr: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_extension
    (this_ptr: *const ::phonenumber::i18n::phonenumbers::PhoneNumber)
     -> *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_has_country_code(this_ptr: *const ::phonenumber::i18n::phonenumbers::PhoneNumber) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_has_country_code_source(this_ptr: *const ::phonenumber::i18n::phonenumbers::PhoneNumber) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_has_extension(this_ptr: *const ::phonenumber::i18n::phonenumbers::PhoneNumber) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_has_italian_leading_zero(this_ptr: *const ::phonenumber::i18n::phonenumbers::PhoneNumber) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_has_national_number(this_ptr: *const ::phonenumber::i18n::phonenumbers::PhoneNumber) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_has_number_of_leading_zeros(this_ptr: *const ::phonenumber::i18n::phonenumbers::PhoneNumber) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_has_preferred_domestic_carrier_code(this_ptr: *const ::phonenumber::i18n::phonenumbers::PhoneNumber) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_has_raw_input(this_ptr: *const ::phonenumber::i18n::phonenumbers::PhoneNumber) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_italian_leading_zero(this_ptr: *const ::phonenumber::i18n::phonenumbers::PhoneNumber) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_mutable_extension
    (this_ptr: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_mutable_preferred_domestic_carrier_code
    (this_ptr: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_mutable_raw_input
    (this_ptr: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_mutable_unknown_fields
    (this_ptr: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_national_number(this_ptr: *const ::phonenumber::i18n::phonenumbers::PhoneNumber) -> libc::c_ulong;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_new_from
    (from: *const ::phonenumber::i18n::phonenumbers::PhoneNumber)
     -> *mut ::phonenumber::i18n::phonenumbers::PhoneNumber;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_new_no_args
    ()
      -> *mut ::phonenumber::i18n::phonenumbers::PhoneNumber;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_number_of_leading_zeros(this_ptr: *const ::phonenumber::i18n::phonenumbers::PhoneNumber) -> libc::c_int;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_operator_assign
    (this_ptr: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber,
     from: *const ::phonenumber::i18n::phonenumbers::PhoneNumber)
     -> *mut ::phonenumber::i18n::phonenumbers::PhoneNumber;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_preferred_domestic_carrier_code
    (this_ptr: *const ::phonenumber::i18n::phonenumbers::PhoneNumber)
     -> *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_raw_input
    (this_ptr: *const ::phonenumber::i18n::phonenumbers::PhoneNumber)
     -> *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_release_extension
    (this_ptr: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_release_preferred_domestic_carrier_code
    (this_ptr: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_release_raw_input
    (this_ptr: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_allocated_extension(this_ptr: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber, extension: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_allocated_preferred_domestic_carrier_code(this_ptr: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber, preferred_domestic_carrier_code: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_allocated_raw_input(this_ptr: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber, raw_input: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_country_code(this_ptr: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber, value: libc::c_int);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_country_code_source(this_ptr: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber, value: ::phonenumber::i18n::phonenumbers::PhoneNumberCountryCodeSource);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_extension_char(this_ptr: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber, value: *const libc::c_char);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_extension_char_unsigned_long(this_ptr: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber, value: *const libc::c_char, size: libc::c_ulong);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_extension_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char
    (this_ptr: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber,
     value: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_italian_leading_zero(this_ptr: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber, value: bool);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_national_number(this_ptr: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber, value: libc::c_ulong);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_number_of_leading_zeros(this_ptr: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber, value: libc::c_int);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_preferred_domestic_carrier_code_char(this_ptr: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber, value: *const libc::c_char);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_preferred_domestic_carrier_code_char_unsigned_long(this_ptr: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber, value: *const libc::c_char, size: libc::c_ulong);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_preferred_domestic_carrier_code_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char
    (this_ptr: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber,
     value: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_raw_input_char(this_ptr: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber, value: *const libc::c_char);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_raw_input_char_unsigned_long(this_ptr: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber, value: *const libc::c_char, size: libc::c_ulong);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_set_raw_input_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char
    (this_ptr: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber,
     value: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumber_unknown_fields
    (this_ptr: *const ::phonenumber::i18n::phonenumbers::PhoneNumber)
     -> *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_phonenumber_G_i18n_phonenumbers_PhoneNumber_CountryCodeSource_IsValid(value: libc::c_int) -> bool;
  pub fn libphonenumber_sys_c_phonenumber_G_i18n_phonenumbers_protobuf_AddDesc_phonenumber_2eproto();
  pub fn libphonenumber_sys_c_phonenumber_G_i18n_phonenumbers_protobuf_ShutdownFile_phonenumber_2eproto();

  // Header: phonenumbermatch.h
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_CopyFrom(this_ptr: *mut ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch, number: *const ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_Equals(this_ptr: *const ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch, number: *const ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_ToString_as_ptr
    (this_ptr: *const ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_ToString_to_output(this_ptr: *const ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch, output: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_constructor_no_args(output: *mut ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_constructor_start_raw_string_number(start: libc::c_int, raw_string: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, number: *const ::phonenumber::i18n::phonenumbers::PhoneNumber, output: *mut ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_delete(this_ptr: *mut ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_destructor(this_ptr: *mut ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_end(this_ptr: *const ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch) -> libc::c_int;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_length(this_ptr: *const ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch) -> libc::c_int;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_new_no_args
    ()
      -> *mut ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_new_start_raw_string_number
    (start: libc::c_int,
     raw_string: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,
     number: *const ::phonenumber::i18n::phonenumbers::PhoneNumber)
     -> *mut ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_number
    (this_ptr: *const ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch)
     -> *const ::phonenumber::i18n::phonenumbers::PhoneNumber;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_raw_string
    (this_ptr: *const ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch)
     -> *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_set_number(this_ptr: *mut ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch, number: *const ::phonenumber::i18n::phonenumbers::PhoneNumber);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_set_raw_string(this_ptr: *mut ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch, raw_string: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_set_start(this_ptr: *mut ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch, start: libc::c_int);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatch_start(this_ptr: *const ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch) -> libc::c_int;

  // Header: phonenumbermatcher.h
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatcher_HasNext(this_ptr: *mut ::phonenumbermatcher::i18n::phonenumbers::PhoneNumberMatcher) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatcher_Next(this_ptr: *mut ::phonenumbermatcher::i18n::phonenumbers::PhoneNumberMatcher, match_: *mut ::phonenumbermatch::i18n::phonenumbers::PhoneNumberMatch) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatcher_constructor_text_region_code(text: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, region_code: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, output: *mut ::phonenumbermatcher::i18n::phonenumbers::PhoneNumberMatcher);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatcher_constructor_util_text_region_code_leniency_max_tries
    (util: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil,
     text: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,
     region_code: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,
     leniency: ::phonenumbermatcher::i18n::phonenumbers::phone_number_matcher::Leniency,
     max_tries: libc::c_int,
     output: *mut ::phonenumbermatcher::i18n::phonenumbers::PhoneNumberMatcher);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatcher_delete(this_ptr: *mut ::phonenumbermatcher::i18n::phonenumbers::PhoneNumberMatcher);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatcher_destructor(this_ptr: *mut ::phonenumbermatcher::i18n::phonenumbers::PhoneNumberMatcher);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatcher_new_text_region_code
    (text: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,
     region_code: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef)
     -> *mut ::phonenumbermatcher::i18n::phonenumbers::PhoneNumberMatcher;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberMatcher_new_util_text_region_code_leniency_max_tries
    (util: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil,
     text: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,
     region_code: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,
     leniency: ::phonenumbermatcher::i18n::phonenumbers::phone_number_matcher::Leniency,
     max_tries: libc::c_int)
     -> *mut ::phonenumbermatcher::i18n::phonenumbers::PhoneNumberMatcher;

  // Header: phonenumberutil.h
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_ConvertAlphaCharactersInNumber(this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_Format(this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number: *const ::phonenumber::i18n::phonenumbers::PhoneNumber, number_format: ::phonenumberutil::i18n::phonenumbers::phone_number_util::PhoneNumberFormat, formatted_number: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_Format_Str(this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number: *const ::phonenumber::i18n::phonenumbers::PhoneNumber, number_format: ::phonenumberutil::i18n::phonenumbers::phone_number_util::PhoneNumberFormat) -> *const i8;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_FormatByPattern(this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number: *const ::phonenumber::i18n::phonenumbers::PhoneNumber, number_format: ::phonenumberutil::i18n::phonenumbers::phone_number_util::PhoneNumberFormat, user_defined_formats: *const ::repeated_field::google::protobuf::RepeatedPtrFieldNumberFormatRef, formatted_number: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_FormatInOriginalFormat(this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number: *const ::phonenumber::i18n::phonenumbers::PhoneNumber, region_calling_from: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, formatted_number: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_FormatNationalNumberWithCarrierCode(this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number: *const ::phonenumber::i18n::phonenumbers::PhoneNumber, carrier_code: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, formatted_number: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_FormatNationalNumberWithPreferredCarrierCode(this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number: *const ::phonenumber::i18n::phonenumbers::PhoneNumber, fallback_carrier_code: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, formatted_number: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_FormatNumberForMobileDialing(this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number: *const ::phonenumber::i18n::phonenumbers::PhoneNumber, region_calling_from: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, with_formatting: bool, formatted_number: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_FormatOutOfCountryCallingNumber(this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number: *const ::phonenumber::i18n::phonenumbers::PhoneNumber, calling_from: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, formatted_number: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_FormatOutOfCountryKeepingAlphaChars(this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number: *const ::phonenumber::i18n::phonenumbers::PhoneNumber, calling_from: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, formatted_number: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetAsYouTypeFormatter
    (this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil,
     region_code: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef)
     -> *mut ::asyoutypeformatter::i18n::phonenumbers::AsYouTypeFormatter;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetCountryCodeForRegion(this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, region_code: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> libc::c_int;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetCountryMobileToken(this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, country_calling_code: libc::c_int, mobile_token: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetExampleNumber(this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, region_code: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, number: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetExampleNumberForNonGeoEntity(this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, country_calling_code: libc::c_int, number: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetExampleNumberForType_region_code_type_number
    (this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil,
     region_code: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,
     type_: ::phonenumberutil::i18n::phonenumbers::phone_number_util::PhoneNumberType,
     number: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber)
     -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetExampleNumberForType_type_number(this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, type_: ::phonenumberutil::i18n::phonenumbers::phone_number_util::PhoneNumberType, number: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetInstance
    ()
      -> *mut ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetInvalidExampleNumber(this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, region_code: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, number: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetLengthOfGeographicalAreaCode(this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number: *const ::phonenumber::i18n::phonenumbers::PhoneNumber) -> libc::c_int;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetLengthOfNationalDestinationCode(this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number: *const ::phonenumber::i18n::phonenumbers::PhoneNumber) -> libc::c_int;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetNationalSignificantNumber(this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number: *const ::phonenumber::i18n::phonenumbers::PhoneNumber, national_significant_num: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetNddPrefixForRegion(this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, region_code: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, strip_non_digits: bool, national_prefix: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetNumberType
    (this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil,
     number: *const ::phonenumber::i18n::phonenumbers::PhoneNumber)
     -> ::phonenumberutil::i18n::phonenumbers::phone_number_util::PhoneNumberType;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetRegionCodeForCountryCode(this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, country_code: libc::c_int, region_code: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetRegionCodeForNumber(this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number: *const ::phonenumber::i18n::phonenumbers::PhoneNumber, region_code: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetRegionCodesForCountryCallingCode(this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, country_calling_code: libc::c_int, region_codes: *mut ::stl_list::std::cxx11::ListBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRefAllocatorBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRefRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetSupportedGlobalNetworkCallingCodes(this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, calling_codes: *mut ::stl_set::std::SetCIntLessCIntRefAllocatorCIntRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_GetSupportedRegions(this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, regions: *mut ::stl_set::std::SetBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRefLessBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRefRefAllocatorBasicStringCCharCharTraitsCCharRefAllocatorCCharRefRefRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_IsAlphaNumber(this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_IsNANPACountry(this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, region_code: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_IsNumberGeographical_phone_number(this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, phone_number: *const ::phonenumber::i18n::phonenumbers::PhoneNumber) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_IsNumberGeographical_phone_number_type_country_calling_code
    (this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil,
     phone_number_type: ::phonenumberutil::i18n::phonenumbers::phone_number_util::PhoneNumberType,
     country_calling_code: libc::c_int)
     -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_IsNumberMatch
    (this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil,
     first_number: *const ::phonenumber::i18n::phonenumbers::PhoneNumber,
     second_number: *const ::phonenumber::i18n::phonenumbers::PhoneNumber)
     -> ::phonenumberutil::i18n::phonenumbers::phone_number_util::MatchType;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_IsNumberMatchWithOneString
    (this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil,
     first_number: *const ::phonenumber::i18n::phonenumbers::PhoneNumber,
     second_number: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef)
     -> ::phonenumberutil::i18n::phonenumbers::phone_number_util::MatchType;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_IsNumberMatchWithTwoStrings
    (this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil,
     first_number: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,
     second_number: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef)
     -> ::phonenumberutil::i18n::phonenumbers::phone_number_util::MatchType;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_IsPossibleNumber(this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number: *const ::phonenumber::i18n::phonenumbers::PhoneNumber) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_IsPossibleNumberForString(this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, region_dialing_from: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_IsPossibleNumberWithReason
    (this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil,
     number: *const ::phonenumber::i18n::phonenumbers::PhoneNumber)
     -> ::phonenumberutil::i18n::phonenumbers::phone_number_util::ValidationResult;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_IsValidNumber(this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number: *const ::phonenumber::i18n::phonenumbers::PhoneNumber) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_IsValidNumberForRegion(this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number: *const ::phonenumber::i18n::phonenumbers::PhoneNumber, region_code: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_NormalizeDiallableCharsOnly(this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_NormalizeDigitsOnly(this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_Parse
    (this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil,
     number_to_parse: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,
     default_region: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,
     number: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber)
     -> ::phonenumberutil::i18n::phonenumbers::phone_number_util::ErrorType;
pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_Parse_Str
       (this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil,
	    number_to_parse_str: *const u8,
		number_to_parse_len: libc::size_t,
		default_region_str: *const u8,
		default_region_len: libc::size_t,
		number: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber)
		-> ::phonenumberutil::i18n::phonenumbers::phone_number_util::ErrorType;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_ParseAndKeepRawInput
    (this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil,
     number_to_parse: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,
     default_region: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,
     number: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber)
     -> ::phonenumberutil::i18n::phonenumbers::phone_number_util::ErrorType;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_TruncateTooLongNumber(this_ptr: *const ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil, number: *mut ::phonenumber::i18n::phonenumbers::PhoneNumber) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_delete(this_ptr: *mut ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_PhoneNumberUtil_destructor(this_ptr: *mut ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil);

  // Header: regexp_adapter.h
  pub fn libphonenumber_sys_c_i18n_phonenumbers_AbstractRegExpFactory_CreateInput
    (this_ptr: *const ::regexp_adapter::i18n::phonenumbers::AbstractRegExpFactory,
     utf8_input: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef)
     -> *mut ::regexp_adapter::i18n::phonenumbers::RegExpInput;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_AbstractRegExpFactory_CreateRegExp
    (this_ptr: *const ::regexp_adapter::i18n::phonenumbers::AbstractRegExpFactory,
     utf8_regexp: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef)
     -> *mut ::regexp_adapter::i18n::phonenumbers::RegExp;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_AbstractRegExpFactory_delete(this_ptr: *mut ::regexp_adapter::i18n::phonenumbers::AbstractRegExpFactory);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_AbstractRegExpFactory_destructor(this_ptr: *mut ::regexp_adapter::i18n::phonenumbers::AbstractRegExpFactory);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_RegExpInput_ToString_as_ptr
    (this_ptr: *const ::regexp_adapter::i18n::phonenumbers::RegExpInput)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_RegExpInput_ToString_to_output(this_ptr: *const ::regexp_adapter::i18n::phonenumbers::RegExpInput, output: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_RegExpInput_delete(this_ptr: *mut ::regexp_adapter::i18n::phonenumbers::RegExpInput);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_RegExpInput_destructor(this_ptr: *mut ::regexp_adapter::i18n::phonenumbers::RegExpInput);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_RegExp_Consume_input_string(this_ptr: *const ::regexp_adapter::i18n::phonenumbers::RegExp, input_string: *mut ::regexp_adapter::i18n::phonenumbers::RegExpInput) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_RegExp_Consume_input_string_anchor_at_start_matched_string1_matched_string2_matched_string3
    (this_ptr: *const ::regexp_adapter::i18n::phonenumbers::RegExp,
     input_string: *mut ::regexp_adapter::i18n::phonenumbers::RegExpInput,
     anchor_at_start: bool,
     matched_string1: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,
     matched_string2: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,
     matched_string3: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef)
     -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_RegExp_Consume_input_string_matched_string(this_ptr: *const ::regexp_adapter::i18n::phonenumbers::RegExp, input_string: *mut ::regexp_adapter::i18n::phonenumbers::RegExpInput, matched_string: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_RegExp_Consume_input_string_matched_string1_matched_string2(this_ptr: *const ::regexp_adapter::i18n::phonenumbers::RegExp, input_string: *mut ::regexp_adapter::i18n::phonenumbers::RegExpInput, matched_string1: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, matched_string2: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_RegExp_Consume_input_string_matched_string1_matched_string2_matched_string3
    (this_ptr: *const ::regexp_adapter::i18n::phonenumbers::RegExp,
     input_string: *mut ::regexp_adapter::i18n::phonenumbers::RegExpInput,
     matched_string1: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,
     matched_string2: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,
     matched_string3: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef)
     -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_RegExp_FindAndConsume(this_ptr: *const ::regexp_adapter::i18n::phonenumbers::RegExp, input_string: *mut ::regexp_adapter::i18n::phonenumbers::RegExpInput, matched_string: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_RegExp_FullMatch_input_string(this_ptr: *const ::regexp_adapter::i18n::phonenumbers::RegExp, input_string: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_RegExp_FullMatch_input_string_matched_string(this_ptr: *const ::regexp_adapter::i18n::phonenumbers::RegExp, input_string: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, matched_string: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_RegExp_GlobalReplace(this_ptr: *const ::regexp_adapter::i18n::phonenumbers::RegExp, string_to_process: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, replacement_string: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_RegExp_Match(this_ptr: *const ::regexp_adapter::i18n::phonenumbers::RegExp, input_string: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, full_match: bool, matched_string: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_RegExp_PartialMatch_input_string(this_ptr: *const ::regexp_adapter::i18n::phonenumbers::RegExp, input_string: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_RegExp_PartialMatch_input_string_matched_string(this_ptr: *const ::regexp_adapter::i18n::phonenumbers::RegExp, input_string: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, matched_string: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_RegExp_Replace_string_to_process_global_replacement_string(this_ptr: *const ::regexp_adapter::i18n::phonenumbers::RegExp, string_to_process: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, global: bool, replacement_string: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_RegExp_Replace_string_to_process_replacement_string(this_ptr: *const ::regexp_adapter::i18n::phonenumbers::RegExp, string_to_process: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, replacement_string: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_RegExp_delete(this_ptr: *mut ::regexp_adapter::i18n::phonenumbers::RegExp);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_RegExp_destructor(this_ptr: *mut ::regexp_adapter::i18n::phonenumbers::RegExp);

  // Header: regexp_cache.h
  pub fn libphonenumber_sys_c_i18n_phonenumbers_RegExpCache_GetRegExp
    (this_ptr: *mut ::regexp_cache::i18n::phonenumbers::RegExpCache,
     pattern: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef)
     -> *const ::regexp_adapter::i18n::phonenumbers::RegExp;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_RegExpCache_constructor(regexp_factory: *const ::regexp_adapter::i18n::phonenumbers::AbstractRegExpFactory, min_items: libc::c_ulong, output: *mut ::regexp_cache::i18n::phonenumbers::RegExpCache);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_RegExpCache_delete(this_ptr: *mut ::regexp_cache::i18n::phonenumbers::RegExpCache);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_RegExpCache_destructor(this_ptr: *mut ::regexp_cache::i18n::phonenumbers::RegExpCache);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_RegExpCache_new
    (regexp_factory: *const ::regexp_adapter::i18n::phonenumbers::AbstractRegExpFactory,
     min_items: libc::c_ulong)
     -> *mut ::regexp_cache::i18n::phonenumbers::RegExpCache;

  // Header: scoped_ptr.h
  pub fn libphonenumber_sys_c_i18n_phonenumbers_FreeDeleter_operator_call(this_ptr: *const ::scoped_ptr::i18n::phonenumbers::FreeDeleter, ptr: *mut libc::c_void);

  // Header: shortnumberinfo.h
  pub fn libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_ConnectsToEmergencyNumber(this_ptr: *const ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo, number: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, region_code: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_GetExampleShortNumberForCost_as_ptr
    (this_ptr: *const ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo,
     region_code: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,
     cost: ::shortnumberinfo::i18n::phonenumbers::short_number_info::ShortNumberCost)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_GetExampleShortNumberForCost_to_output(this_ptr: *const ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo, region_code: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, cost: ::shortnumberinfo::i18n::phonenumbers::short_number_info::ShortNumberCost, output: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_GetExampleShortNumber_as_ptr
    (this_ptr: *const ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo,
     region_code: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_GetExampleShortNumber_to_output(this_ptr: *const ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo, region_code: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, output: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_GetExpectedCost
    (this_ptr: *const ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo,
     number: *const ::phonenumber::i18n::phonenumbers::PhoneNumber)
     -> ::shortnumberinfo::i18n::phonenumbers::short_number_info::ShortNumberCost;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_GetExpectedCostForRegion_i18n_phonenumbers_PhoneNumber_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char
    (this_ptr: *const ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo,
     short_number: *const ::phonenumber::i18n::phonenumbers::PhoneNumber,
     region_dialing_from: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef)
     -> ::shortnumberinfo::i18n::phonenumbers::short_number_info::ShortNumberCost;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_GetExpectedCostForRegion_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char
    (this_ptr: *const ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo,
     short_number: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,
     region_dialing_from: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef)
     -> ::shortnumberinfo::i18n::phonenumbers::short_number_info::ShortNumberCost;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_IsCarrierSpecific(this_ptr: *const ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo, number: *const ::phonenumber::i18n::phonenumbers::PhoneNumber) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_IsEmergencyNumber(this_ptr: *const ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo, number: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, region_code: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_IsPossibleShortNumber(this_ptr: *const ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo, number: *const ::phonenumber::i18n::phonenumbers::PhoneNumber) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_IsPossibleShortNumberForRegion_i18n_phonenumbers_PhoneNumber_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char
    (this_ptr: *const ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo,
     short_number: *const ::phonenumber::i18n::phonenumbers::PhoneNumber,
     region_dialing_from: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef)
     -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_IsPossibleShortNumberForRegion_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char
    (this_ptr: *const ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo,
     short_number: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,
     region_dialing_from: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef)
     -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_IsValidShortNumber(this_ptr: *const ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo, number: *const ::phonenumber::i18n::phonenumbers::PhoneNumber) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_IsValidShortNumberForRegion_i18n_phonenumbers_PhoneNumber_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char
    (this_ptr: *const ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo,
     short_number: *const ::phonenumber::i18n::phonenumbers::PhoneNumber,
     region_dialing_from: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef)
     -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_IsValidShortNumberForRegion_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char
    (this_ptr: *const ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo,
     short_number: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,
     region_dialing_from: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef)
     -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_constructor(output: *mut ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_delete(this_ptr: *mut ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_destructor(this_ptr: *mut ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_new
    ()
      -> *mut ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo;

  // Header: singleton_posix.h
  pub fn libphonenumber_sys_c_i18n_phonenumbers_Singleton_i18n_phonenumbers_PhoneNumberUtil_GetInstance
    ()
      -> *mut ::phonenumberutil::i18n::phonenumbers::PhoneNumberUtil;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_Singleton_i18n_phonenumbers_PhoneNumberUtil_delete(this_ptr: *mut ::singleton_posix::i18n::phonenumbers::SingletonPhoneNumberUtilRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_Singleton_i18n_phonenumbers_PhoneNumberUtil_destructor(this_ptr: *mut ::singleton_posix::i18n::phonenumbers::SingletonPhoneNumberUtilRef);

  // Header: thread_checker.h
  pub fn libphonenumber_sys_c_i18n_phonenumbers_ThreadChecker_CalledOnValidThread(this_ptr: *const ::thread_checker::i18n::phonenumbers::ThreadChecker) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_ThreadChecker_constructor(output: *mut ::thread_checker::i18n::phonenumbers::ThreadChecker);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_ThreadChecker_new
    ()
      -> *mut ::thread_checker::i18n::phonenumbers::ThreadChecker;

  // Header: unicodestring.h
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_append_codepoint(this_ptr: *mut ::unicodestring::i18n::phonenumbers::UnicodeString, codepoint: libc::c_int);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_append_unicode_string(this_ptr: *mut ::unicodestring::i18n::phonenumbers::UnicodeString, unicode_string: *const ::unicodestring::i18n::phonenumbers::UnicodeString);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_begin_as_ptr
    (this_ptr: *const ::unicodestring::i18n::phonenumbers::UnicodeString)
     -> *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_begin_to_output(this_ptr: *const ::unicodestring::i18n::phonenumbers::UnicodeString, output: *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_constructor_codepoint(codepoint: libc::c_int, output: *mut ::unicodestring::i18n::phonenumbers::UnicodeString);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_constructor_no_args(output: *mut ::unicodestring::i18n::phonenumbers::UnicodeString);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_constructor_src(src: *const ::unicodestring::i18n::phonenumbers::UnicodeString, output: *mut ::unicodestring::i18n::phonenumbers::UnicodeString);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_constructor_utf8(utf8: *const libc::c_char, output: *mut ::unicodestring::i18n::phonenumbers::UnicodeString);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_end_as_ptr
    (this_ptr: *const ::unicodestring::i18n::phonenumbers::UnicodeString)
     -> *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_end_to_output(this_ptr: *const ::unicodestring::i18n::phonenumbers::UnicodeString, output: *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_indexOf(this_ptr: *const ::unicodestring::i18n::phonenumbers::UnicodeString, codepoint: libc::c_int) -> libc::c_int;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_length(this_ptr: *const ::unicodestring::i18n::phonenumbers::UnicodeString) -> libc::c_int;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_new_codepoint
    (codepoint: libc::c_int)
     -> *mut ::unicodestring::i18n::phonenumbers::UnicodeString;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_new_no_args
    ()
      -> *mut ::unicodestring::i18n::phonenumbers::UnicodeString;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_new_src
    (src: *const ::unicodestring::i18n::phonenumbers::UnicodeString)
     -> *mut ::unicodestring::i18n::phonenumbers::UnicodeString;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_new_utf8
    (utf8: *const libc::c_char)
     -> *mut ::unicodestring::i18n::phonenumbers::UnicodeString;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_operator_assign
    (this_ptr: *mut ::unicodestring::i18n::phonenumbers::UnicodeString,
     src: *const ::unicodestring::i18n::phonenumbers::UnicodeString)
     -> *mut ::unicodestring::i18n::phonenumbers::UnicodeString;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_operator_eq(this_ptr: *const ::unicodestring::i18n::phonenumbers::UnicodeString, rhs: *const ::unicodestring::i18n::phonenumbers::UnicodeString) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_operator_index(this_ptr: *const ::unicodestring::i18n::phonenumbers::UnicodeString, index: libc::c_int) -> libc::c_int;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_remove(this_ptr: *mut ::unicodestring::i18n::phonenumbers::UnicodeString);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_replace(this_ptr: *mut ::unicodestring::i18n::phonenumbers::UnicodeString, start: libc::c_int, length: libc::c_int, src: *const ::unicodestring::i18n::phonenumbers::UnicodeString);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_setCharAt(this_ptr: *mut ::unicodestring::i18n::phonenumbers::UnicodeString, pos: libc::c_int, c: libc::c_int);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_setTo(this_ptr: *mut ::unicodestring::i18n::phonenumbers::UnicodeString, s: *const libc::c_char, len: libc::c_ulong);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_tempSubString_as_ptr_start
    (this_ptr: *const ::unicodestring::i18n::phonenumbers::UnicodeString,
     start: libc::c_int)
     -> *mut ::unicodestring::i18n::phonenumbers::UnicodeString;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_tempSubString_as_ptr_start_length
    (this_ptr: *const ::unicodestring::i18n::phonenumbers::UnicodeString,
     start: libc::c_int,
     length: libc::c_int)
     -> *mut ::unicodestring::i18n::phonenumbers::UnicodeString;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_tempSubString_to_output_start(this_ptr: *const ::unicodestring::i18n::phonenumbers::UnicodeString, start: libc::c_int, output: *mut ::unicodestring::i18n::phonenumbers::UnicodeString);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_tempSubString_to_output_start_length(this_ptr: *const ::unicodestring::i18n::phonenumbers::UnicodeString, start: libc::c_int, length: libc::c_int, output: *mut ::unicodestring::i18n::phonenumbers::UnicodeString);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeString_toUTF8String(this_ptr: *const ::unicodestring::i18n::phonenumbers::UnicodeString, out: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);

  // Header: unicodetext.h
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_Copy
    (this_ptr: *mut ::unicodetext::i18n::phonenumbers::UnicodeText,
     src: *const ::unicodetext::i18n::phonenumbers::UnicodeText)
     -> *mut ::unicodetext::i18n::phonenumbers::UnicodeText;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_CopyUTF8
    (this_ptr: *mut ::unicodetext::i18n::phonenumbers::UnicodeText,
     utf8_buffer: *const libc::c_char,
     byte_length: libc::c_int)
     -> *mut ::unicodetext::i18n::phonenumbers::UnicodeText;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_DebugString_as_ptr
    (this_ptr: *const ::unicodetext::i18n::phonenumbers::UnicodeText)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_DebugString_to_output(this_ptr: *const ::unicodetext::i18n::phonenumbers::UnicodeText, output: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_HasReplacementChar(this_ptr: *const ::unicodetext::i18n::phonenumbers::UnicodeText) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_MakeIterator_as_ptr
    (this_ptr: *const ::unicodetext::i18n::phonenumbers::UnicodeText,
     p: *const libc::c_char)
     -> *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_MakeIterator_to_output(this_ptr: *const ::unicodetext::i18n::phonenumbers::UnicodeText, p: *const libc::c_char, output: *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_PointToUTF8
    (this_ptr: *mut ::unicodetext::i18n::phonenumbers::UnicodeText,
     utf8_buffer: *const libc::c_char,
     byte_length: libc::c_int)
     -> *mut ::unicodetext::i18n::phonenumbers::UnicodeText;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_PointTo_first_last
    (this_ptr: *mut ::unicodetext::i18n::phonenumbers::UnicodeText,
     first: *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator,
     last: *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator)
     -> *mut ::unicodetext::i18n::phonenumbers::UnicodeText;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_PointTo_src
    (this_ptr: *mut ::unicodetext::i18n::phonenumbers::UnicodeText,
     src: *const ::unicodetext::i18n::phonenumbers::UnicodeText)
     -> *mut ::unicodetext::i18n::phonenumbers::UnicodeText;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_TakeOwnershipOfUTF8
    (this_ptr: *mut ::unicodetext::i18n::phonenumbers::UnicodeText,
     utf8_buffer: *mut libc::c_char,
     byte_length: libc::c_int,
     byte_capacity: libc::c_int)
     -> *mut ::unicodetext::i18n::phonenumbers::UnicodeText;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_UTF8Substring_as_ptr
    (first: *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator,
     last: *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_UTF8Substring_to_output(first: *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, last: *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, output: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_append_first_last
    (this_ptr: *mut ::unicodetext::i18n::phonenumbers::UnicodeText,
     first: *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator,
     last: *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator)
     -> *mut ::unicodetext::i18n::phonenumbers::UnicodeText;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_append_source
    (this_ptr: *mut ::unicodetext::i18n::phonenumbers::UnicodeText,
     source: *const ::unicodetext::i18n::phonenumbers::UnicodeText)
     -> *mut ::unicodetext::i18n::phonenumbers::UnicodeText;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_assign
    (this_ptr: *mut ::unicodetext::i18n::phonenumbers::UnicodeText,
     src: *const ::unicodetext::i18n::phonenumbers::UnicodeText)
     -> *mut ::unicodetext::i18n::phonenumbers::UnicodeText;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_begin_as_ptr
    (this_ptr: *const ::unicodetext::i18n::phonenumbers::UnicodeText)
     -> *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_begin_to_output(this_ptr: *const ::unicodetext::i18n::phonenumbers::UnicodeText, output: *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_clear(this_ptr: *mut ::unicodetext::i18n::phonenumbers::UnicodeText);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_DebugString_as_ptr
    (this_ptr: *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_DebugString_to_output(this_ptr: *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, output: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_constructor_no_args(output: *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_constructor_other(other: *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, output: *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_get_utf8(this_ptr: *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, buf: *mut libc::c_char) -> libc::c_int;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_new_no_args
    ()
      -> *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_new_other
    (other: *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator)
     -> *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_operator_assign
    (this_ptr: *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator,
     other: *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator)
     -> *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_operator_dec
    (this_ptr: *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator)
     -> *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_operator_dec_postfix_as_ptr
    (this_ptr: *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator,
     arg1: libc::c_int)
     -> *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_operator_dec_postfix_to_output(this_ptr: *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, arg1: libc::c_int, output: *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_operator_inc
    (this_ptr: *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator)
     -> *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_operator_inc_postfix_as_ptr
    (this_ptr: *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator,
     arg1: libc::c_int)
     -> *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_operator_inc_postfix_to_output(this_ptr: *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, arg1: libc::c_int, output: *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_operator_indirection(this_ptr: *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator) -> libc::c_int;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_iterator_utf8_data(this_ptr: *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator) -> *const libc::c_char;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_reverse_iterator_constructor(it: *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, output: *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstReverseIterator);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_reverse_iterator_get_utf8(this_ptr: *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstReverseIterator, buf: *mut libc::c_char) -> libc::c_int;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_reverse_iterator_new
    (it: *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator)
     -> *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstReverseIterator;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_const_reverse_iterator_utf8_data(this_ptr: *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstReverseIterator) -> *const libc::c_char;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_constructor_first_last(first: *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, last: *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, output: *mut ::unicodetext::i18n::phonenumbers::UnicodeText);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_constructor_no_args(output: *mut ::unicodetext::i18n::phonenumbers::UnicodeText);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_constructor_src(src: *const ::unicodetext::i18n::phonenumbers::UnicodeText, output: *mut ::unicodetext::i18n::phonenumbers::UnicodeText);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_delete(this_ptr: *mut ::unicodetext::i18n::phonenumbers::UnicodeText);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_destructor(this_ptr: *mut ::unicodetext::i18n::phonenumbers::UnicodeText);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_empty(this_ptr: *mut ::unicodetext::i18n::phonenumbers::UnicodeText) -> bool;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_end_as_ptr
    (this_ptr: *const ::unicodetext::i18n::phonenumbers::UnicodeText)
     -> *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_end_to_output(this_ptr: *const ::unicodetext::i18n::phonenumbers::UnicodeText, output: *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_find_as_ptr_look
    (this_ptr: *const ::unicodetext::i18n::phonenumbers::UnicodeText,
     look: *const ::unicodetext::i18n::phonenumbers::UnicodeText)
     -> *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_find_as_ptr_look_start_pos
    (this_ptr: *const ::unicodetext::i18n::phonenumbers::UnicodeText,
     look: *const ::unicodetext::i18n::phonenumbers::UnicodeText,
     start_pos: *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator)
     -> *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_find_to_output_look(this_ptr: *const ::unicodetext::i18n::phonenumbers::UnicodeText, look: *const ::unicodetext::i18n::phonenumbers::UnicodeText, output: *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_find_to_output_look_start_pos(this_ptr: *const ::unicodetext::i18n::phonenumbers::UnicodeText, look: *const ::unicodetext::i18n::phonenumbers::UnicodeText, start_pos: *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator, output: *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_new_first_last
    (first: *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator,
     last: *const ::unicodetext::i18n::phonenumbers::unicode_text::ConstIterator)
     -> *mut ::unicodetext::i18n::phonenumbers::UnicodeText;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_new_no_args
    ()
      -> *mut ::unicodetext::i18n::phonenumbers::UnicodeText;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_new_src
    (src: *const ::unicodetext::i18n::phonenumbers::UnicodeText)
     -> *mut ::unicodetext::i18n::phonenumbers::UnicodeText;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_operator_assign
    (this_ptr: *mut ::unicodetext::i18n::phonenumbers::UnicodeText,
     src: *const ::unicodetext::i18n::phonenumbers::UnicodeText)
     -> *mut ::unicodetext::i18n::phonenumbers::UnicodeText;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_push_back(this_ptr: *mut ::unicodetext::i18n::phonenumbers::UnicodeText, codepoint: libc::c_int);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_rbegin_as_ptr
    (this_ptr: *const ::unicodetext::i18n::phonenumbers::UnicodeText)
     -> *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstReverseIterator;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_rbegin_to_output(this_ptr: *const ::unicodetext::i18n::phonenumbers::UnicodeText, output: *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstReverseIterator);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_rend_as_ptr
    (this_ptr: *const ::unicodetext::i18n::phonenumbers::UnicodeText)
     -> *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstReverseIterator;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_rend_to_output(this_ptr: *const ::unicodetext::i18n::phonenumbers::UnicodeText, output: *mut ::unicodetext::i18n::phonenumbers::unicode_text::ConstReverseIterator);
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_size(this_ptr: *const ::unicodetext::i18n::phonenumbers::UnicodeText) -> libc::c_int;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_utf8_capacity(this_ptr: *const ::unicodetext::i18n::phonenumbers::UnicodeText) -> libc::c_int;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_utf8_data(this_ptr: *const ::unicodetext::i18n::phonenumbers::UnicodeText) -> *const libc::c_char;
  pub fn libphonenumber_sys_c_i18n_phonenumbers_UnicodeText_utf8_length(this_ptr: *const ::unicodetext::i18n::phonenumbers::UnicodeText) -> libc::c_int;
  pub fn libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_MakeUnicodeTextAcceptingOwnership_as_ptr
    (utf8_buffer: *mut libc::c_char,
     byte_length: libc::c_int,
     byte_capacity: libc::c_int)
     -> *mut ::unicodetext::i18n::phonenumbers::UnicodeText;
  pub fn libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_MakeUnicodeTextAcceptingOwnership_to_output(utf8_buffer: *mut libc::c_char, byte_length: libc::c_int, byte_capacity: libc::c_int, output: *mut ::unicodetext::i18n::phonenumbers::UnicodeText);
  pub fn libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_MakeUnicodeTextWithoutAcceptingOwnership_as_ptr
    (utf8_buffer: *const libc::c_char,
     byte_length: libc::c_int)
     -> *mut ::unicodetext::i18n::phonenumbers::UnicodeText;
  pub fn libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_MakeUnicodeTextWithoutAcceptingOwnership_to_output(utf8_buffer: *const libc::c_char, byte_length: libc::c_int, output: *mut ::unicodetext::i18n::phonenumbers::UnicodeText);
  pub fn libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UTF8ToUnicodeText_as_ptr_utf8_buf_len
    (utf8_buf: *const libc::c_char,
     len: libc::c_int)
     -> *mut ::unicodetext::i18n::phonenumbers::UnicodeText;
  pub fn libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UTF8ToUnicodeText_as_ptr_utf8_buf_len_do_copy
    (utf8_buf: *const libc::c_char,
     len: libc::c_int,
     do_copy: bool)
     -> *mut ::unicodetext::i18n::phonenumbers::UnicodeText;
  pub fn libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UTF8ToUnicodeText_as_ptr_utf8_string
    (utf8_string: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef)
     -> *mut ::unicodetext::i18n::phonenumbers::UnicodeText;
  pub fn libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UTF8ToUnicodeText_as_ptr_utf_string_do_copy
    (utf_string: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,
     do_copy: bool)
     -> *mut ::unicodetext::i18n::phonenumbers::UnicodeText;
  pub fn libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UTF8ToUnicodeText_to_output_utf8_buf_len(utf8_buf: *const libc::c_char, len: libc::c_int, output: *mut ::unicodetext::i18n::phonenumbers::UnicodeText);
  pub fn libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UTF8ToUnicodeText_to_output_utf8_buf_len_do_copy(utf8_buf: *const libc::c_char, len: libc::c_int, do_copy: bool, output: *mut ::unicodetext::i18n::phonenumbers::UnicodeText);
  pub fn libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UTF8ToUnicodeText_to_output_utf8_string(utf8_string: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, output: *mut ::unicodetext::i18n::phonenumbers::UnicodeText);
  pub fn libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UTF8ToUnicodeText_to_output_utf_string_do_copy(utf_string: *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, do_copy: bool, output: *mut ::unicodetext::i18n::phonenumbers::UnicodeText);
  pub fn libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UnicodeTextRangeIsEmpty(r: *const ::stl_pair::std::PairConstIteratorRefConstIteratorRef) -> bool;
  pub fn libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UnicodeTextToUTF8_as_ptr
    (t: *const ::unicodetext::i18n::phonenumbers::UnicodeText)
     -> *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  pub fn libphonenumber_sys_c_unicodetext_G_i18n_phonenumbers_UnicodeTextToUTF8_to_output(t: *const ::unicodetext::i18n::phonenumbers::UnicodeText, output: *mut ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef);

}
