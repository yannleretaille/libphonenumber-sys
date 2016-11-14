#[allow(unused_imports)]
use {libc, cpp_utils, std};

pub mod i18n {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  pub mod phonenumbers {
    #[allow(unused_imports)]
    use {libc, cpp_utils, std};

    /// C++ type: <span style='color: green;'>```i18n::phonenumbers::ShortNumberInfo```</span>.
    #[repr(C)]
    pub struct ShortNumberInfo {
      _buffer: [u8; 32],
    }

    impl ::NewUninitialized for ShortNumberInfo {
      unsafe fn new_uninitialized() -> ShortNumberInfo {
        ShortNumberInfo { _buffer: std::mem::uninitialized() }
      }
    }

    impl ShortNumberInfo {
      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::ShortNumberInfo::ConnectsToEmergencyNumber(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& number, const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& region_code) const```</span>
      ///
      ///
pub fn connects_to_emergency_number(&self, number: &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, region_code: &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_ConnectsToEmergencyNumber(self as *const ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo, number as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, region_code as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::ShortNumberInfo::GetExampleShortNumber```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      /// Rust arguments: <br>1) ```fn get_example_short_number(&self, (&::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, cpp_utils::AsBox)) -> cpp_utils::CppBox<::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef>```<br>2) ```fn get_example_short_number(&self, (&::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, cpp_utils::AsStruct)) -> ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef```<br>
      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>> i18n::phonenumbers::ShortNumberInfo::GetExampleShortNumber(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& region_code) const```</span>
      ///
      ///
      pub fn get_example_short_number<'l0, Args>(&'l0 self, args: Args) -> Args::ReturnType
        where Args: overloading::ShortNumberInfoGetExampleShortNumberArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::ShortNumberInfo::GetExampleShortNumberForCost```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      /// Rust arguments: <br>1) ```fn get_example_short_number_for_cost(&self, (&::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, ::shortnumberinfo::i18n::phonenumbers::short_number_info::ShortNumberCost, cpp_utils::AsBox)) -> cpp_utils::CppBox<::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef>```<br>2) ```fn get_example_short_number_for_cost(&self, (&::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, ::shortnumberinfo::i18n::phonenumbers::short_number_info::ShortNumberCost, cpp_utils::AsStruct)) -> ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef```<br>
      /// C++ method: <span style='color: green;'>```std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>> i18n::phonenumbers::ShortNumberInfo::GetExampleShortNumberForCost(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& region_code, i18n::phonenumbers::ShortNumberInfo::ShortNumberCost cost) const```</span>
      ///
      ///
      pub fn get_example_short_number_for_cost<'l0, Args>(&'l0 self, args: Args) -> Args::ReturnType
        where Args: overloading::ShortNumberInfoGetExampleShortNumberForCostArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::ShortNumberInfo::ShortNumberCost i18n::phonenumbers::ShortNumberInfo::GetExpectedCost(const i18n::phonenumbers::PhoneNumber& number) const```</span>
      ///
      ///
      pub fn get_expected_cost(&self,
                               number: &::phonenumber::i18n::phonenumbers::PhoneNumber)
                               -> ::shortnumberinfo::i18n::phonenumbers::short_number_info::ShortNumberCost {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_GetExpectedCost(self as *const ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo, number as *const ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::ShortNumberInfo::GetExpectedCostForRegion```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn get_expected_cost_for_region(&self, (&::phonenumber::i18n::phonenumbers::PhoneNumber, &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef)) -> ::shortnumberinfo::i18n::phonenumbers::short_number_info::ShortNumberCost```<br>
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::ShortNumberInfo::ShortNumberCost i18n::phonenumbers::ShortNumberInfo::GetExpectedCostForRegion(const i18n::phonenumbers::PhoneNumber& short_number, const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& region_dialing_from) const```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn get_expected_cost_for_region(&self, (&::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef)) -> ::shortnumberinfo::i18n::phonenumbers::short_number_info::ShortNumberCost```<br>
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::ShortNumberInfo::ShortNumberCost i18n::phonenumbers::ShortNumberInfo::GetExpectedCostForRegion(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& short_number, const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& region_dialing_from) const```</span>
      ///
      ///
      pub fn get_expected_cost_for_region<'l0, Args>
        (&'l0 self,
         args: Args)
         -> ::shortnumberinfo::i18n::phonenumbers::short_number_info::ShortNumberCost
        where Args: overloading::ShortNumberInfoGetExpectedCostForRegionArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::ShortNumberInfo::IsCarrierSpecific(const i18n::phonenumbers::PhoneNumber& number) const```</span>
      ///
      ///
      pub fn is_carrier_specific(&self, number: &::phonenumber::i18n::phonenumbers::PhoneNumber) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_IsCarrierSpecific(self as *const ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo, number as *const ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::ShortNumberInfo::IsEmergencyNumber(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& number, const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& region_code) const```</span>
      ///
      ///
pub fn is_emergency_number(&self, number: &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, region_code: &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_IsEmergencyNumber(self as *const ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo, number as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, region_code as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
      }

      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::ShortNumberInfo::IsPossibleShortNumber(const i18n::phonenumbers::PhoneNumber& number) const```</span>
      ///
      ///
      pub fn is_possible_short_number(&self, number: &::phonenumber::i18n::phonenumbers::PhoneNumber) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_IsPossibleShortNumber(self as *const ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo, number as *const ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::ShortNumberInfo::IsPossibleShortNumberForRegion```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn is_possible_short_number_for_region(&self, (&::phonenumber::i18n::phonenumbers::PhoneNumber, &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef)) -> bool```<br>
      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::ShortNumberInfo::IsPossibleShortNumberForRegion(const i18n::phonenumbers::PhoneNumber& short_number, const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& region_dialing_from) const```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn is_possible_short_number_for_region(&self, (&::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef)) -> bool```<br>
      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::ShortNumberInfo::IsPossibleShortNumberForRegion(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& short_number, const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& region_dialing_from) const```</span>
      ///
      ///
      pub fn is_possible_short_number_for_region<'l0, Args>(&'l0 self, args: Args) -> bool
        where Args: overloading::ShortNumberInfoIsPossibleShortNumberForRegionArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::ShortNumberInfo::IsValidShortNumber(const i18n::phonenumbers::PhoneNumber& number) const```</span>
      ///
      ///
      pub fn is_valid_short_number(&self, number: &::phonenumber::i18n::phonenumbers::PhoneNumber) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_IsValidShortNumber(self as *const ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo, number as *const ::phonenumber::i18n::phonenumbers::PhoneNumber) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::ShortNumberInfo::IsValidShortNumberForRegion```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      ///
      ///
      /// ## Variant 1
      ///
      /// Rust arguments: ```fn is_valid_short_number_for_region(&self, (&::phonenumber::i18n::phonenumbers::PhoneNumber, &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef)) -> bool```<br>
      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::ShortNumberInfo::IsValidShortNumberForRegion(const i18n::phonenumbers::PhoneNumber& short_number, const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& region_dialing_from) const```</span>
      ///
      ///
      ///
      /// ## Variant 2
      ///
      /// Rust arguments: ```fn is_valid_short_number_for_region(&self, (&::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, &::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef)) -> bool```<br>
      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::ShortNumberInfo::IsValidShortNumberForRegion(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& short_number, const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& region_dialing_from) const```</span>
      ///
      ///
      pub fn is_valid_short_number_for_region<'l0, Args>(&'l0 self, args: Args) -> bool
        where Args: overloading::ShortNumberInfoIsValidShortNumberForRegionArgs<'l0>
      {
        args.exec(self)
      }
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::ShortNumberInfo::ShortNumberInfo```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      /// Rust arguments: <br>1) ```fn new(cpp_utils::AsStruct) -> ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo```<br>2) ```fn new(cpp_utils::AsBox) -> cpp_utils::CppBox<::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo>```<br>
      /// C++ method: <span style='color: green;'>```[constructor] void i18n::phonenumbers::ShortNumberInfo::ShortNumberInfo()```</span>
      ///
      ///
      pub fn new<Args>(args: Args) -> Args::ReturnType
        where Args: overloading::ShortNumberInfoNewArgs
      {
        args.exec()
      }
    }

    impl Drop for ShortNumberInfo {
      /// C++ method: <span style='color: green;'>```[destructor] void i18n::phonenumbers::ShortNumberInfo::~ShortNumberInfo()```</span>
      ///
      ///
      fn drop(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_destructor(self as *mut ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo) }
      }
    }

    impl cpp_utils::CppDeletable for ShortNumberInfo {
      fn deleter() -> cpp_utils::Deleter<Self> {
        ::ffi::libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_delete
      }
    }

    pub mod overloading {
      #[allow(unused_imports)]
      use {libc, cpp_utils, std};

      /// This trait represents a set of arguments accepted by [ShortNumberInfo::get_example_short_number](../struct.ShortNumberInfo.html#method.get_example_short_number) method.
      pub trait ShortNumberInfoGetExampleShortNumberArgs<'l0> {
        type ReturnType;
        fn exec(self, original_self: &'l0 ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo) -> Self::ReturnType;
      }
      impl<'l0> ShortNumberInfoGetExampleShortNumberArgs<'l0> for (&'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,cpp_utils::AsBox) {
  type ReturnType = cpp_utils::CppBox<::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef>;
  fn exec(self, original_self: &'l0 ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo) -> cpp_utils::CppBox<::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef> {
    let region_code = self.0;
    let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_GetExampleShortNumber_as_ptr(original_self as *const ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo, region_code as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) };
unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }
}
      impl<'l0> ShortNumberInfoGetExampleShortNumberArgs<'l0> for (&'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,cpp_utils::AsStruct) {
  type ReturnType = ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  fn exec(self, original_self: &'l0 ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo) -> ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
    let region_code = self.0;
    {
let mut object: ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef = unsafe { ::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_GetExampleShortNumber_to_output(original_self as *const ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo, region_code as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, &mut object) }object
}
  }
}
      /// This trait represents a set of arguments accepted by [ShortNumberInfo::get_example_short_number_for_cost](../struct.ShortNumberInfo.html#method.get_example_short_number_for_cost) method.
      pub trait ShortNumberInfoGetExampleShortNumberForCostArgs<'l0> {
        type ReturnType;
        fn exec(self, original_self: &'l0 ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo) -> Self::ReturnType;
      }
      impl<'l0> ShortNumberInfoGetExampleShortNumberForCostArgs<'l0> for (&'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,::shortnumberinfo::i18n::phonenumbers::short_number_info::ShortNumberCost,cpp_utils::AsBox) {
  type ReturnType = cpp_utils::CppBox<::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef>;
  fn exec(self, original_self: &'l0 ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo) -> cpp_utils::CppBox<::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef> {
    let region_code = self.0;
let cost = self.1;
    let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_GetExampleShortNumberForCost_as_ptr(original_self as *const ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo, region_code as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, cost) };
unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }
}
      impl<'l0> ShortNumberInfoGetExampleShortNumberForCostArgs<'l0> for (&'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,::shortnumberinfo::i18n::phonenumbers::short_number_info::ShortNumberCost,cpp_utils::AsStruct) {
  type ReturnType = ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef;
  fn exec(self, original_self: &'l0 ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo) -> ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef {
    let region_code = self.0;
let cost = self.1;
    {
let mut object: ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef = unsafe { ::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_GetExampleShortNumberForCost_to_output(original_self as *const ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo, region_code as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, cost, &mut object) }object
}
  }
}
      /// This trait represents a set of arguments accepted by [ShortNumberInfo::get_expected_cost_for_region](../struct.ShortNumberInfo.html#method.get_expected_cost_for_region) method.
      pub trait ShortNumberInfoGetExpectedCostForRegionArgs<'l0> {
        fn exec(self,
                original_self: &'l0 ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo)
                -> ::shortnumberinfo::i18n::phonenumbers::short_number_info::ShortNumberCost;
      }
      impl<'l0> ShortNumberInfoGetExpectedCostForRegionArgs<'l0> for (&'l0 ::phonenumber::i18n::phonenumbers::PhoneNumber,&'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {

  fn exec(self, original_self: &'l0 ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo) -> ::shortnumberinfo::i18n::phonenumbers::short_number_info::ShortNumberCost {
    let short_number = self.0;
let region_dialing_from = self.1;
    unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_GetExpectedCostForRegion_i18n_phonenumbers_PhoneNumber_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(original_self as *const ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo, short_number as *const ::phonenumber::i18n::phonenumbers::PhoneNumber, region_dialing_from as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
  }
}
      impl<'l0> ShortNumberInfoGetExpectedCostForRegionArgs<'l0> for (&'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,&'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {

  fn exec(self, original_self: &'l0 ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo) -> ::shortnumberinfo::i18n::phonenumbers::short_number_info::ShortNumberCost {
    let short_number = self.0;
let region_dialing_from = self.1;
    unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_GetExpectedCostForRegion_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(original_self as *const ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo, short_number as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, region_dialing_from as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
  }
}
      /// This trait represents a set of arguments accepted by [ShortNumberInfo::is_possible_short_number_for_region](../struct.ShortNumberInfo.html#method.is_possible_short_number_for_region) method.
      pub trait ShortNumberInfoIsPossibleShortNumberForRegionArgs<'l0> {
        fn exec(self, original_self: &'l0 ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo) -> bool;
      }
      impl<'l0> ShortNumberInfoIsPossibleShortNumberForRegionArgs<'l0> for (&'l0 ::phonenumber::i18n::phonenumbers::PhoneNumber,&'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {

  fn exec(self, original_self: &'l0 ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo) -> bool {
    let short_number = self.0;
let region_dialing_from = self.1;
    unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_IsPossibleShortNumberForRegion_i18n_phonenumbers_PhoneNumber_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(original_self as *const ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo, short_number as *const ::phonenumber::i18n::phonenumbers::PhoneNumber, region_dialing_from as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
  }
}
      impl<'l0> ShortNumberInfoIsPossibleShortNumberForRegionArgs<'l0> for (&'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,&'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {

  fn exec(self, original_self: &'l0 ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo) -> bool {
    let short_number = self.0;
let region_dialing_from = self.1;
    unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_IsPossibleShortNumberForRegion_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(original_self as *const ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo, short_number as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, region_dialing_from as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
  }
}
      /// This trait represents a set of arguments accepted by [ShortNumberInfo::is_valid_short_number_for_region](../struct.ShortNumberInfo.html#method.is_valid_short_number_for_region) method.
      pub trait ShortNumberInfoIsValidShortNumberForRegionArgs<'l0> {
        fn exec(self, original_self: &'l0 ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo) -> bool;
      }
      impl<'l0> ShortNumberInfoIsValidShortNumberForRegionArgs<'l0> for (&'l0 ::phonenumber::i18n::phonenumbers::PhoneNumber,&'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {

  fn exec(self, original_self: &'l0 ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo) -> bool {
    let short_number = self.0;
let region_dialing_from = self.1;
    unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_IsValidShortNumberForRegion_i18n_phonenumbers_PhoneNumber_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(original_self as *const ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo, short_number as *const ::phonenumber::i18n::phonenumbers::PhoneNumber, region_dialing_from as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
  }
}
      impl<'l0> ShortNumberInfoIsValidShortNumberForRegionArgs<'l0> for (&'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef,&'l0 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) {

  fn exec(self, original_self: &'l0 ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo) -> bool {
    let short_number = self.0;
let region_dialing_from = self.1;
    unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_IsValidShortNumberForRegion_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char_std___cxx11_basic_string_char_std_char_traits_char_std_allocator_char(original_self as *const ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo, short_number as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef, region_dialing_from as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) }
  }
}
      /// This trait represents a set of arguments accepted by [ShortNumberInfo::new](../struct.ShortNumberInfo.html#method.new) method.
      pub trait ShortNumberInfoNewArgs {
        type ReturnType;
        fn exec(self) -> Self::ReturnType;
      }
      impl ShortNumberInfoNewArgs for cpp_utils::AsStruct {
        type ReturnType = ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo;
        fn exec(self) -> ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo {

          {
            let mut object: ::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo =
              unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_constructor(&mut object) }
            object
          }
        }
      }
      impl ShortNumberInfoNewArgs for cpp_utils::AsBox {
        type ReturnType = cpp_utils::CppBox<::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo>;
        fn exec(self) -> cpp_utils::CppBox<::shortnumberinfo::i18n::phonenumbers::ShortNumberInfo> {

          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_ShortNumberInfo_new() };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
    }

    pub mod short_number_info {
      #[allow(unused_imports)]
      use {libc, cpp_utils, std};

      /// C++ type: <span style='color: green;'>```i18n::phonenumbers::ShortNumberInfo::ShortNumberCost```</span>
      #[derive(Debug, PartialEq, Eq, Clone)]
      #[repr(C)]
      pub enum ShortNumberCost {
        /// C++ variant: TOLL_FREE
        TollFree = 0,
        /// C++ variant: STANDARD_RATE
        StandardRate = 1,
        /// C++ variant: PREMIUM_RATE
        PremiumRate = 2,
        /// C++ variant: UNKNOWN_COST
        UnknownCost = 3,
      }

    }

  }

}
