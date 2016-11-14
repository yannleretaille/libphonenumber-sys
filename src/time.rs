#[allow(unused_imports)]
use {libc, cpp_utils, std};

/// C++ type: <span style='color: green;'>```itimerspec```</span>.
#[repr(C)]
pub struct Itimerspec {
  _buffer: [u8; 32],
}

impl ::NewUninitialized for Itimerspec {
  unsafe fn new_uninitialized() -> Itimerspec {
    Itimerspec { _buffer: std::mem::uninitialized() }
  }
}

/// C++ type: <span style='color: green;'>```timespec```</span>.
#[repr(C)]
pub struct Timespec {
  _buffer: [u8; 16],
}

impl ::NewUninitialized for Timespec {
  unsafe fn new_uninitialized() -> Timespec {
    Timespec { _buffer: std::mem::uninitialized() }
  }
}

/// C++ type: <span style='color: green;'>```timeval```</span>.
#[repr(C)]
pub struct Timeval {
  _buffer: [u8; 16],
}

impl ::NewUninitialized for Timeval {
  unsafe fn new_uninitialized() -> Timeval {
    Timeval { _buffer: std::mem::uninitialized() }
  }
}

/// C++ type: <span style='color: green;'>```tm```</span>.
#[repr(C)]
pub struct Tm {
  _buffer: [u8; 56],
}

impl ::NewUninitialized for Tm {
  unsafe fn new_uninitialized() -> Tm {
    Tm { _buffer: std::mem::uninitialized() }
  }
}
