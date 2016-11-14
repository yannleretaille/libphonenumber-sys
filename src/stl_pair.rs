#[allow(unused_imports)]
use {libc, cpp_utils};

pub mod std {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  /// C++ type: <span style='color: green;'>```std::pair<unsigned long, unsigned long>```</span>.
  #[repr(C)]
  pub struct PairCUlongCUlong {
    _buffer: [u8; 16],
  }

  impl ::NewUninitialized for PairCUlongCUlong {
    unsafe fn new_uninitialized() -> PairCUlongCUlong {
      PairCUlongCUlong { _buffer: std::mem::uninitialized() }
    }
  }

  /// C++ type: <span style='color: green;'>```std::pair<i18n::phonenumbers::UnicodeText::const_iterator, i18n::phonenumbers::UnicodeText::const_iterator>```</span>.
  #[repr(C)]
  pub struct PairConstIteratorRefConstIteratorRef {
    _buffer: [u8; 16],
  }

  impl ::NewUninitialized for PairConstIteratorRefConstIteratorRef {
    unsafe fn new_uninitialized() -> PairConstIteratorRefConstIteratorRef {
      PairConstIteratorRefConstIteratorRef { _buffer: std::mem::uninitialized() }
    }
  }

  /// C++ type: <span style='color: green;'>```std::pair<std::_Rb_tree_node_base*, std::_Rb_tree_node_base*>```</span>.
  #[repr(C)]
  pub struct PairRbTreeNodeBaseMutPtrRbTreeNodeBaseMutPtr {
    _buffer: [u8; 16],
  }

  impl ::NewUninitialized for PairRbTreeNodeBaseMutPtrRbTreeNodeBaseMutPtr {
    unsafe fn new_uninitialized() -> PairRbTreeNodeBaseMutPtrRbTreeNodeBaseMutPtr {
      PairRbTreeNodeBaseMutPtrRbTreeNodeBaseMutPtr { _buffer: std::mem::uninitialized() }
    }
  }

}
