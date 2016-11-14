#[allow(unused_imports)]
use {libc, cpp_utils};

pub mod std {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  /// C++ type: <span style='color: green;'>```std::float_denorm_style```</span>
  #[derive(Debug, PartialEq, Eq, Clone)]
  #[repr(C)]
  pub enum FloatDenormStyle {
    /// C++ variant: denorm_indeterminate
    Indeterminate = -1,
    /// C++ variant: denorm_absent
    Absent = 0,
    /// C++ variant: denorm_present
    Present = 1,
  }

  /// C++ type: <span style='color: green;'>```std::float_round_style```</span>
  #[derive(Debug, PartialEq, Eq, Clone)]
  #[repr(C)]
  pub enum FloatRoundStyle {
    /// C++ variant: round_indeterminate
    Indeterminate = -1,
    /// C++ variant: round_toward_zero
    TowardZero = 0,
    /// C++ variant: round_to_nearest
    ToNearest = 1,
    /// C++ variant: round_toward_infinity
    TowardInfinity = 2,
    /// C++ variant: round_toward_neg_infinity
    TowardNegInfinity = 3,
  }

  /// C++ type: <span style='color: green;'>```std::__numeric_limits_base```</span>.
  #[repr(C)]
  pub struct NumericLimitsBase {
    _buffer: [u8; 1],
  }

  impl ::NewUninitialized for NumericLimitsBase {
    unsafe fn new_uninitialized() -> NumericLimitsBase {
      NumericLimitsBase { _buffer: std::mem::uninitialized() }
    }
  }

}
