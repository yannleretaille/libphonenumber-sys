#[allow(unused_imports)]
use {libc, cpp_utils, std};

pub mod google {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  pub mod protobuf {
    #[allow(unused_imports)]
    use {libc, cpp_utils, std};

    pub mod internal {
      #[allow(unused_imports)]
      use {libc, cpp_utils, std};

      /// C++ type: <span style='color: green;'>```google::protobuf::internal::Mutex```</span>.
      #[repr(C)]
      pub struct Mutex {
        _buffer: [u8; 8],
      }

      impl ::NewUninitialized for Mutex {
        unsafe fn new_uninitialized() -> Mutex {
          Mutex { _buffer: std::mem::uninitialized() }
        }
      }

      /// C++ type: <span style='color: green;'>```google::protobuf::internal::MutexLock```</span>.
      #[repr(C)]
      pub struct MutexLock {
        _buffer: [u8; 8],
      }

      impl ::NewUninitialized for MutexLock {
        unsafe fn new_uninitialized() -> MutexLock {
          MutexLock { _buffer: std::mem::uninitialized() }
        }
      }

      /// C++ type: <span style='color: green;'>```google::protobuf::internal::MutexLockMaybe```</span>.
      #[repr(C)]
      pub struct MutexLockMaybe {
        _buffer: [u8; 8],
      }

      impl ::NewUninitialized for MutexLockMaybe {
        unsafe fn new_uninitialized() -> MutexLockMaybe {
          MutexLockMaybe { _buffer: std::mem::uninitialized() }
        }
      }

    }

  }

}
