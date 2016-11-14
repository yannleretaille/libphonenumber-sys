#[allow(unused_imports)]
use {libc, cpp_utils, std};

/// C++ type: <span style='color: green;'>```__pthread_internal_list```</span>.
#[repr(C)]
pub struct PthreadInternalList {
  _buffer: [u8; 16],
}

impl ::NewUninitialized for PthreadInternalList {
  unsafe fn new_uninitialized() -> PthreadInternalList {
    PthreadInternalList { _buffer: std::mem::uninitialized() }
  }
}

/// C++ type: <span style='color: green;'>```__pthread_mutex_s```</span>.
#[repr(C)]
pub struct PthreadMutexS {
  _buffer: [u8; 40],
}

impl ::NewUninitialized for PthreadMutexS {
  unsafe fn new_uninitialized() -> PthreadMutexS {
    PthreadMutexS { _buffer: std::mem::uninitialized() }
  }
}
