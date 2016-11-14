#[allow(unused_imports)]
use {libc, cpp_utils};

pub mod std {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  /// C++ type: <span style='color: green;'>```std::ios_base```</span>.
  #[repr(C)]
  pub struct IosBase {
    _buffer: [u8; 216],
  }

  impl ::NewUninitialized for IosBase {
    unsafe fn new_uninitialized() -> IosBase {
      IosBase { _buffer: std::mem::uninitialized() }
    }
  }

  /// C++ type: <span style='color: green;'>```std::_Ios_Fmtflags```</span>
  #[derive(Debug, PartialEq, Eq, Clone)]
  #[repr(C)]
  pub enum IosFmtflags {
    /// C++ variant: _S_ios_fmtflags_min
    IosFmtflagsMin = -2147483648,
    /// C++ variant: _S_boolalpha
    Boolalpha = 1,
    /// C++ variant: _S_dec
    Dec = 2,
    /// C++ variant: _S_fixed
    Fixed = 4,
    /// C++ variant: _S_hex
    Hex = 8,
    /// C++ variant: _S_internal
    Internal = 16,
    /// C++ variant: _S_left
    Left = 32,
    /// C++ variant: _S_oct
    Oct = 64,
    /// C++ variant: _S_basefield
    Basefield = 74,
    /// C++ variant: _S_right
    Right = 128,
    /// C++ variant: _S_adjustfield
    Adjustfield = 176,
    /// C++ variant: _S_scientific
    Scientific = 256,
    /// C++ variant: _S_floatfield
    Floatfield = 260,
    /// C++ variant: _S_showbase
    Showbase = 512,
    /// C++ variant: _S_showpoint
    Showpoint = 1024,
    /// C++ variant: _S_showpos
    Showpos = 2048,
    /// C++ variant: _S_skipws
    Skipws = 4096,
    /// C++ variant: _S_unitbuf
    Unitbuf = 8192,
    /// C++ variant: _S_uppercase
    Uppercase = 16384,
    /// C++ variant: _S_ios_fmtflags_end
    IosFmtflagsEnd = 65536,
    /// C++ variant: _S_ios_fmtflags_max
    IosFmtflagsMax = 2147483647,
  }

  /// C++ type: <span style='color: green;'>```std::_Ios_Iostate```</span>
  #[derive(Debug, PartialEq, Eq, Clone)]
  #[repr(C)]
  pub enum IosIostate {
    /// C++ variant: _S_ios_iostate_min
    IosIostateMin = -2147483648,
    /// C++ variant: _S_goodbit
    Goodbit = 0,
    /// C++ variant: _S_badbit
    Badbit = 1,
    /// C++ variant: _S_eofbit
    Eofbit = 2,
    /// C++ variant: _S_failbit
    Failbit = 4,
    /// C++ variant: _S_ios_iostate_end
    IosIostateEnd = 65536,
    /// C++ variant: _S_ios_iostate_max
    IosIostateMax = 2147483647,
  }

  /// C++ type: <span style='color: green;'>```std::_Ios_Openmode```</span>
  #[derive(Debug, PartialEq, Eq, Clone)]
  #[repr(C)]
  pub enum IosOpenmode {
    /// C++ variant: _S_ios_openmode_min
    IosOpenmodeMin = -2147483648,
    /// C++ variant: _S_app
    App = 1,
    /// C++ variant: _S_ate
    Ate = 2,
    /// C++ variant: _S_bin
    Bin = 4,
    /// C++ variant: _S_in
    In = 8,
    /// C++ variant: _S_out
    Out = 16,
    /// C++ variant: _S_trunc
    Trunc = 32,
    /// C++ variant: _S_ios_openmode_end
    IosOpenmodeEnd = 65536,
    /// C++ variant: _S_ios_openmode_max
    IosOpenmodeMax = 2147483647,
  }

  /// C++ type: <span style='color: green;'>```std::_Ios_Seekdir```</span>
  #[derive(Debug, PartialEq, Eq, Clone)]
  #[repr(C)]
  pub enum IosSeekdir {
    /// C++ variant: _S_beg
    Beg = 0,
    /// C++ variant: _S_cur
    Cur = 1,
    /// C++ variant: _S_end
    End = 2,
    /// C++ variant: _S_ios_seekdir_end
    IosSeekdirEnd = 65536,
  }

  pub mod ios_base {
    #[allow(unused_imports)]
    use {libc, cpp_utils, std};

    /// C++ type: <span style='color: green;'>```std::ios_base::_Callback_list```</span>.
    #[repr(C)]
    pub struct CallbackList {
      _buffer: [u8; 24],
    }

    impl ::NewUninitialized for CallbackList {
      unsafe fn new_uninitialized() -> CallbackList {
        CallbackList { _buffer: std::mem::uninitialized() }
      }
    }

    /// C++ type: <span style='color: green;'>```std::ios_base::event```</span>
    #[derive(Debug, PartialEq, Eq, Clone)]
    #[repr(C)]
    pub enum Event {
      /// C++ variant: erase_event
      Erase = 0,
      /// C++ variant: imbue_event
      Imbue = 1,
      /// C++ variant: copyfmt_event
      Copyfmt = 2,
    }

    /// C++ type: <span style='color: green;'>```std::ios_base::Init```</span>.
    #[repr(C)]
    pub struct Init {
      _buffer: [u8; 1],
    }

    impl ::NewUninitialized for Init {
      unsafe fn new_uninitialized() -> Init {
        Init { _buffer: std::mem::uninitialized() }
      }
    }

    /// C++ type: <span style='color: green;'>```std::ios_base::_Words```</span>.
    #[repr(C)]
    pub struct Words {
      _buffer: [u8; 16],
    }

    impl ::NewUninitialized for Words {
      unsafe fn new_uninitialized() -> Words {
        Words { _buffer: std::mem::uninitialized() }
      }
    }

  }

}
