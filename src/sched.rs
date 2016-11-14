#[allow(unused_imports)]
use {libc, cpp_utils, std};

/// C++ type: <span style='color: green;'>```sched_param```</span>.
#[repr(C)]
pub struct SchedParam {
  _buffer: [u8; 4],
}

impl ::NewUninitialized for SchedParam {
  unsafe fn new_uninitialized() -> SchedParam {
    SchedParam { _buffer: std::mem::uninitialized() }
  }
}
