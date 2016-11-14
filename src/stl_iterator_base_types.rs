#[allow(unused_imports)]
use {libc, cpp_utils};

pub mod std {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  /// C++ type: <span style='color: green;'>```std::bidirectional_iterator_tag```</span>.
  #[repr(C)]
  pub struct BidirectionalIteratorTag {
    _buffer: [u8; 1],
  }

  impl ::NewUninitialized for BidirectionalIteratorTag {
    unsafe fn new_uninitialized() -> BidirectionalIteratorTag {
      BidirectionalIteratorTag { _buffer: std::mem::uninitialized() }
    }
  }

  /// C++ type: <span style='color: green;'>```std::forward_iterator_tag```</span>.
  #[repr(C)]
  pub struct ForwardIteratorTag {
    _buffer: [u8; 1],
  }

  impl ::NewUninitialized for ForwardIteratorTag {
    unsafe fn new_uninitialized() -> ForwardIteratorTag {
      ForwardIteratorTag { _buffer: std::mem::uninitialized() }
    }
  }

  /// C++ type: <span style='color: green;'>```std::input_iterator_tag```</span>.
  #[repr(C)]
  pub struct InputIteratorTag {
    _buffer: [u8; 1],
  }

  impl ::NewUninitialized for InputIteratorTag {
    unsafe fn new_uninitialized() -> InputIteratorTag {
      InputIteratorTag { _buffer: std::mem::uninitialized() }
    }
  }

  /// C++ type: <span style='color: green;'>```std::iterator<std::output_iterator_tag, void, void, void, void>```</span>.
  #[repr(C)]
  pub struct IteratorOutputIteratorTagRefVoidVoidVoidVoid {
    _buffer: [u8; 1],
  }

  impl ::NewUninitialized for IteratorOutputIteratorTagRefVoidVoidVoidVoid {
    unsafe fn new_uninitialized() -> IteratorOutputIteratorTagRefVoidVoidVoidVoid {
      IteratorOutputIteratorTagRefVoidVoidVoidVoid { _buffer: std::mem::uninitialized() }
    }
  }

  /// C++ type: <span style='color: green;'>```std::iterator<std::random_access_iterator_tag, bool, long, bool*, bool&>```</span>.
  #[repr(C)]
  pub struct IteratorRandomAccessIteratorTagRefBoolCLongBoolMutPtrBoolMutRef {
    _buffer: [u8; 1],
  }

  impl ::NewUninitialized for IteratorRandomAccessIteratorTagRefBoolCLongBoolMutPtrBoolMutRef {
    unsafe fn new_uninitialized() -> IteratorRandomAccessIteratorTagRefBoolCLongBoolMutPtrBoolMutRef {
      IteratorRandomAccessIteratorTagRefBoolCLongBoolMutPtrBoolMutRef { _buffer: std::mem::uninitialized() }
    }
  }

  /// C++ type: <span style='color: green;'>```std::output_iterator_tag```</span>.
  #[repr(C)]
  pub struct OutputIteratorTag {
    _buffer: [u8; 1],
  }

  impl ::NewUninitialized for OutputIteratorTag {
    unsafe fn new_uninitialized() -> OutputIteratorTag {
      OutputIteratorTag { _buffer: std::mem::uninitialized() }
    }
  }

  /// C++ type: <span style='color: green;'>```std::random_access_iterator_tag```</span>.
  #[repr(C)]
  pub struct RandomAccessIteratorTag {
    _buffer: [u8; 1],
  }

  impl ::NewUninitialized for RandomAccessIteratorTag {
    unsafe fn new_uninitialized() -> RandomAccessIteratorTag {
      RandomAccessIteratorTag { _buffer: std::mem::uninitialized() }
    }
  }

}
