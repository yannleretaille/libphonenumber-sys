#[allow(unused_imports)]
use {libc, cpp_utils, std};

pub mod i18n {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  pub mod phonenumbers {
    #[allow(unused_imports)]
    use {libc, cpp_utils, std};

    /// C++ type: <span style='color: green;'>```i18n::phonenumbers::AutoLock```</span>.
    #[repr(C)]
    pub struct AutoLock {
      _buffer: [u8; 8],
    }

    impl ::NewUninitialized for AutoLock {
      unsafe fn new_uninitialized() -> AutoLock {
        AutoLock { _buffer: std::mem::uninitialized() }
      }
    }

    impl AutoLock {
      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::AutoLock::AutoLock```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      /// Rust arguments: <br>1) ```fn new((&mut ::lock_posix::i18n::phonenumbers::Lock, cpp_utils::AsStruct)) -> ::lock::i18n::phonenumbers::AutoLock```<br>2) ```fn new((&mut ::lock_posix::i18n::phonenumbers::Lock, cpp_utils::AsBox)) -> cpp_utils::CppBox<::lock::i18n::phonenumbers::AutoLock>```<br>
      /// C++ method: <span style='color: green;'>```[constructor] void i18n::phonenumbers::AutoLock::AutoLock(i18n::phonenumbers::Lock& lock)```</span>
      ///
      ///
      pub fn new<Args>(args: Args) -> Args::ReturnType
        where Args: overloading::AutoLockNewArgs
      {
        args.exec()
      }
    }

    impl Drop for AutoLock {
      /// C++ method: <span style='color: green;'>```[destructor] void i18n::phonenumbers::AutoLock::~AutoLock()```</span>
      ///
      ///
      fn drop(&mut self) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_AutoLock_destructor(self as *mut ::lock::i18n::phonenumbers::AutoLock) }
      }
    }

    impl cpp_utils::CppDeletable for AutoLock {
      fn deleter() -> cpp_utils::Deleter<Self> {
        ::ffi::libphonenumber_sys_c_i18n_phonenumbers_AutoLock_delete
      }
    }

    pub mod overloading {
      #[allow(unused_imports)]
      use {libc, cpp_utils, std};

      /// This trait represents a set of arguments accepted by [AutoLock::new](../struct.AutoLock.html#method.new) method.
      pub trait AutoLockNewArgs {
        type ReturnType;
        fn exec(self) -> Self::ReturnType;
      }
      impl<'a> AutoLockNewArgs for (&'a mut ::lock_posix::i18n::phonenumbers::Lock, cpp_utils::AsStruct) {
        type ReturnType = ::lock::i18n::phonenumbers::AutoLock;
        fn exec(self) -> ::lock::i18n::phonenumbers::AutoLock {
          let lock = self.0;
          {
            let mut object: ::lock::i18n::phonenumbers::AutoLock = unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_AutoLock_constructor(lock as *mut ::lock_posix::i18n::phonenumbers::Lock, &mut object) }
            object
          }
        }
      }
      impl<'a> AutoLockNewArgs for (&'a mut ::lock_posix::i18n::phonenumbers::Lock, cpp_utils::AsBox) {
        type ReturnType = cpp_utils::CppBox<::lock::i18n::phonenumbers::AutoLock>;
        fn exec(self) -> cpp_utils::CppBox<::lock::i18n::phonenumbers::AutoLock> {
          let lock = self.0;
          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_AutoLock_new(lock as *mut ::lock_posix::i18n::phonenumbers::Lock) };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
    }

  }

}
