#[allow(unused_imports)]
use {libc, cpp_utils};

pub mod std {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  /// C++ type: <span style='color: green;'>```std::locale```</span>.
  #[repr(C)]
  pub struct Locale {
    _buffer: [u8; 8],
  }

  impl ::NewUninitialized for Locale {
    unsafe fn new_uninitialized() -> Locale {
      Locale { _buffer: std::mem::uninitialized() }
    }
  }

  pub mod locale {
    #[allow(unused_imports)]
    use {libc, cpp_utils, std};

    /// C++ type: <span style='color: green;'>```std::locale::facet```</span>.
    #[repr(C)]
    pub struct Facet {
      _buffer: [u8; 16],
    }

    impl ::NewUninitialized for Facet {
      unsafe fn new_uninitialized() -> Facet {
        Facet { _buffer: std::mem::uninitialized() }
      }
    }

    /// C++ type: <span style='color: green;'>```std::locale::id```</span>.
    #[repr(C)]
    pub struct Id {
      _buffer: [u8; 8],
    }

    impl ::NewUninitialized for Id {
      unsafe fn new_uninitialized() -> Id {
        Id { _buffer: std::mem::uninitialized() }
      }
    }

    /// C++ type: <span style='color: green;'>```std::locale::_Impl```</span>.
    #[repr(C)]
    pub struct Impl {
      _buffer: [u8; 40],
    }

    impl ::NewUninitialized for Impl {
      unsafe fn new_uninitialized() -> Impl {
        Impl { _buffer: std::mem::uninitialized() }
      }
    }

  }

}
