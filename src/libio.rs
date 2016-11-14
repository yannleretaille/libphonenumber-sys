#[allow(unused_imports)]
use {libc, cpp_utils, std};

/// C++ type: <span style='color: green;'>```__codecvt_result```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum CodecvtResult {
  /// C++ variant: __codecvt_ok
  Ok = 0,
  /// C++ variant: __codecvt_partial
  Partial = 1,
  /// C++ variant: __codecvt_error
  Error = 2,
  /// C++ variant: __codecvt_noconv
  Noconv = 3,
}

/// C++ type: <span style='color: green;'>```_IO_FILE```</span>.
#[repr(C)]
pub struct IOFILE {
  _buffer: [u8; 216],
}

impl ::NewUninitialized for IOFILE {
  unsafe fn new_uninitialized() -> IOFILE {
    IOFILE { _buffer: std::mem::uninitialized() }
  }
}

/// C++ type: <span style='color: green;'>```_IO_marker```</span>.
#[repr(C)]
pub struct IOMarker {
  _buffer: [u8; 24],
}

impl ::NewUninitialized for IOMarker {
  unsafe fn new_uninitialized() -> IOMarker {
    IOMarker { _buffer: std::mem::uninitialized() }
  }
}
