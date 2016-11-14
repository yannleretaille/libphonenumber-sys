#[allow(unused_imports)]
use {libc, cpp_utils, std};

/// C++ type: <span style='color: green;'>```_fpreg```</span>.
#[repr(C)]
pub struct Fpreg {
  _buffer: [u8; 10],
}

impl ::NewUninitialized for Fpreg {
  unsafe fn new_uninitialized() -> Fpreg {
    Fpreg { _buffer: std::mem::uninitialized() }
  }
}

/// C++ type: <span style='color: green;'>```_fpstate```</span>.
#[repr(C)]
pub struct Fpstate {
  _buffer: [u8; 512],
}

impl ::NewUninitialized for Fpstate {
  unsafe fn new_uninitialized() -> Fpstate {
    Fpstate { _buffer: std::mem::uninitialized() }
  }
}

/// C++ type: <span style='color: green;'>```_fpx_sw_bytes```</span>.
#[repr(C)]
pub struct FpxSwBytes {
  _buffer: [u8; 48],
}

impl ::NewUninitialized for FpxSwBytes {
  unsafe fn new_uninitialized() -> FpxSwBytes {
    FpxSwBytes { _buffer: std::mem::uninitialized() }
  }
}

/// C++ type: <span style='color: green;'>```_fpxreg```</span>.
#[repr(C)]
pub struct Fpxreg {
  _buffer: [u8; 16],
}

impl ::NewUninitialized for Fpxreg {
  unsafe fn new_uninitialized() -> Fpxreg {
    Fpxreg { _buffer: std::mem::uninitialized() }
  }
}

/// C++ type: <span style='color: green;'>```sigcontext```</span>.
#[repr(C)]
pub struct Sigcontext {
  _buffer: [u8; 256],
}

impl ::NewUninitialized for Sigcontext {
  unsafe fn new_uninitialized() -> Sigcontext {
    Sigcontext { _buffer: std::mem::uninitialized() }
  }
}

/// C++ type: <span style='color: green;'>```_xmmreg```</span>.
#[repr(C)]
pub struct Xmmreg {
  _buffer: [u8; 16],
}

impl ::NewUninitialized for Xmmreg {
  unsafe fn new_uninitialized() -> Xmmreg {
    Xmmreg { _buffer: std::mem::uninitialized() }
  }
}

/// C++ type: <span style='color: green;'>```_xsave_hdr```</span>.
#[repr(C)]
pub struct XsaveHdr {
  _buffer: [u8; 64],
}

impl ::NewUninitialized for XsaveHdr {
  unsafe fn new_uninitialized() -> XsaveHdr {
    XsaveHdr { _buffer: std::mem::uninitialized() }
  }
}

/// C++ type: <span style='color: green;'>```_xstate```</span>.
#[repr(C)]
pub struct Xstate {
  _buffer: [u8; 832],
}

impl ::NewUninitialized for Xstate {
  unsafe fn new_uninitialized() -> Xstate {
    Xstate { _buffer: std::mem::uninitialized() }
  }
}

/// C++ type: <span style='color: green;'>```_ymmh_state```</span>.
#[repr(C)]
pub struct YmmhState {
  _buffer: [u8; 256],
}

impl ::NewUninitialized for YmmhState {
  unsafe fn new_uninitialized() -> YmmhState {
    YmmhState { _buffer: std::mem::uninitialized() }
  }
}
