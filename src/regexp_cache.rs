#[allow(unused_imports)]
use {libc, cpp_utils, std};

pub mod i18n {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  pub mod phonenumbers {
    #[allow(unused_imports)]
    use {libc, cpp_utils, std};

    /// C++ type: <span style='color: green;'>```i18n::phonenumbers::RegExpCache```</span>.
    #[repr(C)]
    pub struct RegExpCache {
      _buffer: [u8; 56],
    }

    impl ::NewUninitialized for RegExpCache {
      unsafe fn new_uninitialized() -> RegExpCache {
        RegExpCache { _buffer: std::mem::uninitialized() }
      }
    }

    impl RegExpCache {
      /// C++ method: <span style='color: green;'>```const i18n::phonenumbers::RegExp& i18n::phonenumbers::RegExpCache::GetRegExp(const std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>& pattern)```</span>
      ///
      ///
pub fn get_reg_exp<'l0, 'l1>(&'l0 mut self, pattern: &'l1 ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) -> &'l0 ::regexp_adapter::i18n::phonenumbers::RegExp {
        let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_RegExpCache_GetRegExp(self as *mut ::regexp_cache::i18n::phonenumbers::RegExpCache, pattern as *const ::basic_string::std::cxx11::BasicStringCCharCharTraitsCCharRefAllocatorCCharRef) };
        unsafe { &*ffi_result }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::RegExpCache::RegExpCache```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      /// Rust arguments: <br>1) ```fn new((&::regexp_adapter::i18n::phonenumbers::AbstractRegExpFactory, libc::c_ulong, cpp_utils::AsStruct)) -> ::regexp_cache::i18n::phonenumbers::RegExpCache```<br>2) ```fn new((&::regexp_adapter::i18n::phonenumbers::AbstractRegExpFactory, libc::c_ulong, cpp_utils::AsBox)) -> cpp_utils::CppBox<::regexp_cache::i18n::phonenumbers::RegExpCache>```<br>
      /// C++ method: <span style='color: green;'>```[constructor] void i18n::phonenumbers::RegExpCache::RegExpCache(const i18n::phonenumbers::AbstractRegExpFactory& regexp_factory, unsigned long min_items)```</span>
      ///
      ///
      pub fn new<Args>(args: Args) -> Args::ReturnType
        where Args: overloading::RegExpCacheNewArgs
      {
        args.exec()
      }
    }

    impl Drop for RegExpCache {
      /// C++ method: <span style='color: green;'>```[destructor] void i18n::phonenumbers::RegExpCache::~RegExpCache()```</span>
      ///
      ///
      fn drop(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_RegExpCache_destructor(self as *mut ::regexp_cache::i18n::phonenumbers::RegExpCache) }
      }
    }

    impl cpp_utils::CppDeletable for RegExpCache {
      fn deleter() -> cpp_utils::Deleter<Self> {
        ::ffi::libphonenumber_sys_c_i18n_phonenumbers_RegExpCache_delete
      }
    }

    pub mod overloading {
      #[allow(unused_imports)]
      use {libc, cpp_utils, std};

      /// This trait represents a set of arguments accepted by [RegExpCache::new](../struct.RegExpCache.html#method.new) method.
      pub trait RegExpCacheNewArgs {
        type ReturnType;
        fn exec(self) -> Self::ReturnType;
      }
      impl<'a> RegExpCacheNewArgs
        for (&'a ::regexp_adapter::i18n::phonenumbers::AbstractRegExpFactory, libc::c_ulong, cpp_utils::AsStruct) {
        type ReturnType = ::regexp_cache::i18n::phonenumbers::RegExpCache;
        fn exec(self) -> ::regexp_cache::i18n::phonenumbers::RegExpCache {
          let regexp_factory = self.0;
          let min_items = self.1;
          {
            let mut object: ::regexp_cache::i18n::phonenumbers::RegExpCache =
              unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_RegExpCache_constructor(regexp_factory as *const ::regexp_adapter::i18n::phonenumbers::AbstractRegExpFactory, min_items, &mut object) }
            object
          }
        }
      }
      impl<'a> RegExpCacheNewArgs
        for (&'a ::regexp_adapter::i18n::phonenumbers::AbstractRegExpFactory, libc::c_ulong, cpp_utils::AsBox) {
        type ReturnType = cpp_utils::CppBox<::regexp_cache::i18n::phonenumbers::RegExpCache>;
        fn exec(self) -> cpp_utils::CppBox<::regexp_cache::i18n::phonenumbers::RegExpCache> {
          let regexp_factory = self.0;
          let min_items = self.1;
          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_RegExpCache_new(regexp_factory as *const ::regexp_adapter::i18n::phonenumbers::AbstractRegExpFactory, min_items) };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
    }

  }

}
