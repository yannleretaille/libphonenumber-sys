#[allow(unused_imports)]
use {libc, cpp_utils, std};

pub mod i18n {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  pub mod phonenumbers {
    #[allow(unused_imports)]
    use {libc, cpp_utils, std};

    /// C++ type: <span style='color: green;'>```i18n::phonenumbers::ThreadChecker```</span>.
    #[repr(C)]
    pub struct ThreadChecker {
      _buffer: [u8; 8],
    }

    impl ::NewUninitialized for ThreadChecker {
      unsafe fn new_uninitialized() -> ThreadChecker {
        ThreadChecker { _buffer: std::mem::uninitialized() }
      }
    }

    //added
    impl cpp_utils::CppDeletable for ThreadChecker {
      fn deleter() -> cpp_utils::Deleter<Self> { 
			unsafe extern "C" fn deleter_func(this_ptr: *mut ThreadChecker) {
				drop(this_ptr);
			};
			deleter_func
		}
    }

    impl ThreadChecker {
      /// C++ method: <span style='color: green;'>```bool i18n::phonenumbers::ThreadChecker::CalledOnValidThread() const```</span>
      ///
      ///
      pub fn called_on_valid_thread(&self) -> bool {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_ThreadChecker_CalledOnValidThread(self as *const ::thread_checker::i18n::phonenumbers::ThreadChecker) }
      }

      /// C++ method: <span style='color: green;'>```i18n::phonenumbers::ThreadChecker::ThreadChecker```</span>
      ///
      /// This is an overloaded function. Available variants:
      ///
      /// Rust arguments: <br>1) ```fn new(cpp_utils::AsStruct) -> ::thread_checker::i18n::phonenumbers::ThreadChecker```<br>2) ```fn new(cpp_utils::AsBox) -> cpp_utils::CppBox<::thread_checker::i18n::phonenumbers::ThreadChecker>```<br>
      /// C++ method: <span style='color: green;'>```[constructor] void i18n::phonenumbers::ThreadChecker::ThreadChecker()```</span>
      ///
      ///
      pub fn new<Args>(args: Args) -> Args::ReturnType
        where Args: overloading::ThreadCheckerNewArgs
      {
        args.exec()
      }
    }

    pub mod overloading {
      #[allow(unused_imports)]
      use {libc, cpp_utils, std};

      /// This trait represents a set of arguments accepted by [ThreadChecker::new](../struct.ThreadChecker.html#method.new) method.
      pub trait ThreadCheckerNewArgs {
        type ReturnType;
        fn exec(self) -> Self::ReturnType;
      }
      impl ThreadCheckerNewArgs for cpp_utils::AsStruct {
        type ReturnType = ::thread_checker::i18n::phonenumbers::ThreadChecker;
        fn exec(self) -> ::thread_checker::i18n::phonenumbers::ThreadChecker {

          {
            let mut object: ::thread_checker::i18n::phonenumbers::ThreadChecker =
              unsafe { ::NewUninitialized::new_uninitialized() };
            unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_ThreadChecker_constructor(&mut object) }
            object
          }
        }
      }
      impl ThreadCheckerNewArgs for cpp_utils::AsBox {
        type ReturnType = cpp_utils::CppBox<::thread_checker::i18n::phonenumbers::ThreadChecker>;
        fn exec(self) -> cpp_utils::CppBox<::thread_checker::i18n::phonenumbers::ThreadChecker> {

          let ffi_result = unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_ThreadChecker_new() };
          unsafe { ::cpp_utils::CppBox::new(ffi_result) }
        }
      }
    }

  }

}
