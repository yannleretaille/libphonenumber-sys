#[allow(unused_imports)]
use {libc, cpp_utils, std};

/// C++ type: <span style='color: green;'>```_pthread_cleanup_buffer```</span>.
#[repr(C)]
pub struct PthreadCleanupBuffer {
  _buffer: [u8; 32],
}

impl ::NewUninitialized for PthreadCleanupBuffer {
  unsafe fn new_uninitialized() -> PthreadCleanupBuffer {
    PthreadCleanupBuffer { _buffer: std::mem::uninitialized() }
  }
}

/// C++ type: <span style='color: green;'>```__pthread_cleanup_class```</span>.
#[repr(C)]
pub struct PthreadCleanupClass {
  _buffer: [u8; 24],
}

impl ::NewUninitialized for PthreadCleanupClass {
  unsafe fn new_uninitialized() -> PthreadCleanupClass {
    PthreadCleanupClass { _buffer: std::mem::uninitialized() }
  }
}

/// C++ type: <span style='color: green;'>```__pthread_cleanup_frame```</span>.
#[repr(C)]
pub struct PthreadCleanupFrame {
  _buffer: [u8; 24],
}

impl ::NewUninitialized for PthreadCleanupFrame {
  unsafe fn new_uninitialized() -> PthreadCleanupFrame {
    PthreadCleanupFrame { _buffer: std::mem::uninitialized() }
  }
}
