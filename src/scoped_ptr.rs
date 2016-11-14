#[allow(unused_imports)]
use {libc, cpp_utils, std};

pub mod i18n {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  pub mod phonenumbers {
    #[allow(unused_imports)]
    use {libc, cpp_utils, std};

    /// C++ type: <span style='color: green;'>```i18n::phonenumbers::FreeDeleter```</span>.
    #[repr(C)]
    pub struct FreeDeleter {
      _buffer: [u8; 1],
    }

    impl ::NewUninitialized for FreeDeleter {
      unsafe fn new_uninitialized() -> FreeDeleter {
        FreeDeleter { _buffer: std::mem::uninitialized() }
      }
    }

    impl FreeDeleter {
      /// C++ method: <span style='color: green;'>```void i18n::phonenumbers::FreeDeleter::operator()(void* ptr) const```</span>
      ///
      ///
      pub fn op_call(&self, ptr: *mut libc::c_void) {
        unsafe { ::ffi::libphonenumber_sys_c_i18n_phonenumbers_FreeDeleter_operator_call(self as *const ::scoped_ptr::i18n::phonenumbers::FreeDeleter, ptr) }
      }
    }

  }

}
