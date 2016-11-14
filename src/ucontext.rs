#[allow(unused_imports)]
use {libc, cpp_utils, std};

/// C++ type: <span style='color: green;'>```_libc_fpstate```</span>.
#[repr(C)]
pub struct LibcFpstate {
  _buffer: [u8; 512],
}

impl ::NewUninitialized for LibcFpstate {
  unsafe fn new_uninitialized() -> LibcFpstate {
    LibcFpstate { _buffer: std::mem::uninitialized() }
  }
}

/// C++ type: <span style='color: green;'>```_libc_fpxreg```</span>.
#[repr(C)]
pub struct LibcFpxreg {
  _buffer: [u8; 16],
}

impl ::NewUninitialized for LibcFpxreg {
  unsafe fn new_uninitialized() -> LibcFpxreg {
    LibcFpxreg { _buffer: std::mem::uninitialized() }
  }
}

/// C++ type: <span style='color: green;'>```_libc_xmmreg```</span>.
#[repr(C)]
pub struct LibcXmmreg {
  _buffer: [u8; 16],
}

impl ::NewUninitialized for LibcXmmreg {
  unsafe fn new_uninitialized() -> LibcXmmreg {
    LibcXmmreg { _buffer: std::mem::uninitialized() }
  }
}

/// C++ type: <span style='color: green;'>```ucontext```</span>.
#[repr(C)]
pub struct Ucontext {
  _buffer: [u8; 936],
}

impl ::NewUninitialized for Ucontext {
  unsafe fn new_uninitialized() -> Ucontext {
    Ucontext { _buffer: std::mem::uninitialized() }
  }
}
