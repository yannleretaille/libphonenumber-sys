#[allow(unused_imports)]
use {libc, cpp_utils, std};

/// C++ type: <span style='color: green;'>```drand48_data```</span>.
#[repr(C)]
pub struct Drand48Data {
  _buffer: [u8; 24],
}

impl ::NewUninitialized for Drand48Data {
  unsafe fn new_uninitialized() -> Drand48Data {
    Drand48Data { _buffer: std::mem::uninitialized() }
  }
}

/// C++ type: <span style='color: green;'>```random_data```</span>.
#[repr(C)]
pub struct RandomData {
  _buffer: [u8; 48],
}

impl ::NewUninitialized for RandomData {
  unsafe fn new_uninitialized() -> RandomData {
    RandomData { _buffer: std::mem::uninitialized() }
  }
}
