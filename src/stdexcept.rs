#[allow(unused_imports)]
use {libc, cpp_utils};

pub mod std {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  /// C++ type: <span style='color: green;'>```std::__cow_string```</span>.
  #[repr(C)]
  pub struct CowString {
    _buffer: [u8; 8],
  }

  impl ::NewUninitialized for CowString {
    unsafe fn new_uninitialized() -> CowString {
      CowString { _buffer: std::mem::uninitialized() }
    }
  }

  /// C++ type: <span style='color: green;'>```std::domain_error```</span>.
  #[repr(C)]
  pub struct DomainError {
    _buffer: [u8; 16],
  }

  impl ::NewUninitialized for DomainError {
    unsafe fn new_uninitialized() -> DomainError {
      DomainError { _buffer: std::mem::uninitialized() }
    }
  }

  /// C++ type: <span style='color: green;'>```std::invalid_argument```</span>.
  #[repr(C)]
  pub struct InvalidArgument {
    _buffer: [u8; 16],
  }

  impl ::NewUninitialized for InvalidArgument {
    unsafe fn new_uninitialized() -> InvalidArgument {
      InvalidArgument { _buffer: std::mem::uninitialized() }
    }
  }

  /// C++ type: <span style='color: green;'>```std::length_error```</span>.
  #[repr(C)]
  pub struct LengthError {
    _buffer: [u8; 16],
  }

  impl ::NewUninitialized for LengthError {
    unsafe fn new_uninitialized() -> LengthError {
      LengthError { _buffer: std::mem::uninitialized() }
    }
  }

  /// C++ type: <span style='color: green;'>```std::logic_error```</span>.
  #[repr(C)]
  pub struct LogicError {
    _buffer: [u8; 16],
  }

  impl ::NewUninitialized for LogicError {
    unsafe fn new_uninitialized() -> LogicError {
      LogicError { _buffer: std::mem::uninitialized() }
    }
  }

  /// C++ type: <span style='color: green;'>```std::out_of_range```</span>.
  #[repr(C)]
  pub struct OutOfRange {
    _buffer: [u8; 16],
  }

  impl ::NewUninitialized for OutOfRange {
    unsafe fn new_uninitialized() -> OutOfRange {
      OutOfRange { _buffer: std::mem::uninitialized() }
    }
  }

  /// C++ type: <span style='color: green;'>```std::overflow_error```</span>.
  #[repr(C)]
  pub struct OverflowError {
    _buffer: [u8; 16],
  }

  impl ::NewUninitialized for OverflowError {
    unsafe fn new_uninitialized() -> OverflowError {
      OverflowError { _buffer: std::mem::uninitialized() }
    }
  }

  /// C++ type: <span style='color: green;'>```std::range_error```</span>.
  #[repr(C)]
  pub struct RangeError {
    _buffer: [u8; 16],
  }

  impl ::NewUninitialized for RangeError {
    unsafe fn new_uninitialized() -> RangeError {
      RangeError { _buffer: std::mem::uninitialized() }
    }
  }

  /// C++ type: <span style='color: green;'>```std::runtime_error```</span>.
  #[repr(C)]
  pub struct RuntimeError {
    _buffer: [u8; 16],
  }

  impl ::NewUninitialized for RuntimeError {
    unsafe fn new_uninitialized() -> RuntimeError {
      RuntimeError { _buffer: std::mem::uninitialized() }
    }
  }

  /// C++ type: <span style='color: green;'>```std::underflow_error```</span>.
  #[repr(C)]
  pub struct UnderflowError {
    _buffer: [u8; 16],
  }

  impl ::NewUninitialized for UnderflowError {
    unsafe fn new_uninitialized() -> UnderflowError {
      UnderflowError { _buffer: std::mem::uninitialized() }
    }
  }

}
