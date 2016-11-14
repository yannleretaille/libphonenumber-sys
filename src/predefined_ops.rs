#[allow(unused_imports)]
use {libc, cpp_utils, std};

pub mod gnu_cxx {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  pub mod ops {
    #[allow(unused_imports)]
    use {libc, cpp_utils, std};

    /// C++ type: <span style='color: green;'>```__gnu_cxx::__ops::_Iter_equal_to_iter```</span>.
    #[repr(C)]
    pub struct IterEqualToIter {
      _buffer: [u8; 1],
    }

    impl ::NewUninitialized for IterEqualToIter {
      unsafe fn new_uninitialized() -> IterEqualToIter {
        IterEqualToIter { _buffer: std::mem::uninitialized() }
      }
    }

    /// C++ type: <span style='color: green;'>```__gnu_cxx::__ops::_Iter_equal_to_val```</span>.
    #[repr(C)]
    pub struct IterEqualToVal {
      _buffer: [u8; 1],
    }

    impl ::NewUninitialized for IterEqualToVal {
      unsafe fn new_uninitialized() -> IterEqualToVal {
        IterEqualToVal { _buffer: std::mem::uninitialized() }
      }
    }

    /// C++ type: <span style='color: green;'>```__gnu_cxx::__ops::_Iter_less_iter```</span>.
    #[repr(C)]
    pub struct IterLessIter {
      _buffer: [u8; 1],
    }

    impl ::NewUninitialized for IterLessIter {
      unsafe fn new_uninitialized() -> IterLessIter {
        IterLessIter { _buffer: std::mem::uninitialized() }
      }
    }

    /// C++ type: <span style='color: green;'>```__gnu_cxx::__ops::_Iter_less_val```</span>.
    #[repr(C)]
    pub struct IterLessVal {
      _buffer: [u8; 1],
    }

    impl ::NewUninitialized for IterLessVal {
      unsafe fn new_uninitialized() -> IterLessVal {
        IterLessVal { _buffer: std::mem::uninitialized() }
      }
    }

    /// C++ type: <span style='color: green;'>```__gnu_cxx::__ops::_Val_less_iter```</span>.
    #[repr(C)]
    pub struct ValLessIter {
      _buffer: [u8; 1],
    }

    impl ::NewUninitialized for ValLessIter {
      unsafe fn new_uninitialized() -> ValLessIter {
        ValLessIter { _buffer: std::mem::uninitialized() }
      }
    }

  }

}
