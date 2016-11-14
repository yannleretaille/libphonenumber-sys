#[allow(unused_imports)]
use {libc, cpp_utils, std};

pub mod i18n {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  pub mod phonenumbers {
    #[allow(unused_imports)]
    use {libc, cpp_utils, std};

    /// C++ type: <span style='color: green;'>```i18n::phonenumbers::Lock```</span>.
    #[repr(C)]
    pub struct Lock {
      _buffer: [u8; 40],
    }

    impl ::NewUninitialized for Lock {
      unsafe fn new_uninitialized() -> Lock {
        Lock { _buffer: std::mem::uninitialized() }
      }
    }

    impl Lock {
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::Lock::Acquire() const```</span>
      ///
      ///
      pub fn acquire(&self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_Lock_Acquire(self as *const ::lock_posix::i18n::phonenumbers::Lock) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::Lock::Lock```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      /// Rust arguments: <br>1) ```fn new(cpp_utils::AsStruct) -> ::lock_posix::i18n::phonenumbers::Lock```<br>2) ```fn new(cpp_utils::AsBox) -> cpp_utils::CppBox<::lock_posix::i18n::phonenumbers::Lock>```<br>
      /// C++ method: <span style='color: green;'>```[constructor] void i18n::phonenumbers::Lock::Lock()```</span>
      ///
      ///
      pub fn new<Args>(args: Args) -> Args::ReturnType
        where Args: overloading::LockNewArgs
      {
        args.exec()
      }
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::Lock::Release() const```</span>
      ///
      ///
      pub fn release(&self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_Lock_Release(self as *const ::lock_posix::i18n::phonenumbers::Lock) }
      }
    }

    impl Drop for Lock {
      /// C++ method: <span style='color: green;'>```[destructor] void i18n::phonenumbers::Lock::~Lock()```</span>
      ///
      ///
      fn drop(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_Lock_destructor(self as *mut ::lock_posix::i18n::phonenumbers::Lock) }
      }
    }

    impl cpp_utils::CppDeletable for Lock {
      fn deleter() -> cpp_utils::Deleter<Self> {
        ::ffi::libphonenumber_sys_c_i18n_phonenumbers_Lock_delete
      }
    }

    pub mod overloading {
      #[allow(unused_imports)]
      use {libc, cpp_utils, std};

      /// This trait represents a set of arguments accepted by [Lock::new](../struct.Lock.html#method.new) method.
      pub trait LockNewArgs {
        type ReturnType;
        fn exec(self) -> Self::ReturnType;
      }
      impl LockNewArgs for cpp_utils::AsStruct {
        type ReturnType = ::lock_posix::i18n::phonenumbers::Lock;
        fn exec(self) -> ::lock_posix::i18n::phonenumbers::Lock {

          {
            let mut object: ::lock_posix::i18n::phonenumbers::Lock = unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_Lock_constructor(&mut object) }
            object
          }
        }
      }
      impl LockNewArgs for cpp_utils::AsBox {
        type ReturnType = cpp_utils::CppBox<::lock_posix::i18n::phonenumbers::Lock>;
        fn exec(self) -> cpp_utils::CppBox<::lock_posix::i18n::phonenumbers::Lock> {

          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_Lock_new() };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
    }

  }

}
