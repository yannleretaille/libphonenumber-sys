#[allow(unused_imports)]
use {libc, cpp_utils};

pub mod std {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  /// C++ type: <span style='color: green;'>```std::_Rb_tree_color```</span>
  #[derive(Debug, PartialEq, Eq, Clone)]
  #[repr(C)]
  pub enum RbTreeColor {
    /// C++ variant: _S_red
    Red = 0,
    /// C++ variant: _S_black
    Black = 1,
  }

  /// C++ type: <span style='color: green;'>```std::_Rb_tree_node_base```</span>.
  #[repr(C)]
  pub struct RbTreeNodeBase {
    _buffer: [u8; 32],
  }

  impl ::NewUninitialized for RbTreeNodeBase {
    unsafe fn new_uninitialized() -> RbTreeNodeBase {
      RbTreeNodeBase { _buffer: std::mem::uninitialized() }
    }
  }

}
