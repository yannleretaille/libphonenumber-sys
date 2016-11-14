#[allow(unused_imports)]
use {libc, cpp_utils, std};

/// C++ type: <span style='color: green;'>```sigaltstack```</span>.
#[repr(C)]
pub struct Sigaltstack {
  _buffer: [u8; 24],
}

impl ::NewUninitialized for Sigaltstack {
  unsafe fn new_uninitialized() -> Sigaltstack {
    Sigaltstack { _buffer: std::mem::uninitialized() }
  }
}

/// C++ type: <span style='color: green;'>```sigstack```</span>.
#[repr(C)]
pub struct Sigstack {
  _buffer: [u8; 16],
}

impl ::NewUninitialized for Sigstack {
  unsafe fn new_uninitialized() -> Sigstack {
    Sigstack { _buffer: std::mem::uninitialized() }
  }
}
