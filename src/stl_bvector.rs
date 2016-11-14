#[allow(unused_imports)]
use {libc, cpp_utils};

pub mod std {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  /// C++ type: <span style='color: green;'>```std::_Bit_const_iterator```</span>.
  #[repr(C)]
  pub struct BitConstIterator {
    _buffer: [u8; 16],
  }

  impl ::NewUninitialized for BitConstIterator {
    unsafe fn new_uninitialized() -> BitConstIterator {
      BitConstIterator { _buffer: std::mem::uninitialized() }
    }
  }

  /// C++ type: <span style='color: green;'>```std::_Bit_iterator```</span>.
  #[repr(C)]
  pub struct BitIterator {
    _buffer: [u8; 16],
  }

  impl ::NewUninitialized for BitIterator {
    unsafe fn new_uninitialized() -> BitIterator {
      BitIterator { _buffer: std::mem::uninitialized() }
    }
  }

  /// C++ type: <span style='color: green;'>```std::_Bit_iterator_base```</span>.
  #[repr(C)]
  pub struct BitIteratorBase {
    _buffer: [u8; 16],
  }

  impl ::NewUninitialized for BitIteratorBase {
    unsafe fn new_uninitialized() -> BitIteratorBase {
      BitIteratorBase { _buffer: std::mem::uninitialized() }
    }
  }

  /// C++ type: <span style='color: green;'>```std::_Bit_reference```</span>.
  #[repr(C)]
  pub struct BitReference {
    _buffer: [u8; 16],
  }

  impl ::NewUninitialized for BitReference {
    unsafe fn new_uninitialized() -> BitReference {
      BitReference { _buffer: std::mem::uninitialized() }
    }
  }

}
