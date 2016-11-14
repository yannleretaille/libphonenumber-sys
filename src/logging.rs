#[allow(unused_imports)]
use {libc, cpp_utils, std};

pub mod google {
  #[allow(unused_imports)]
  use {libc, cpp_utils, std};

  pub mod protobuf {
    #[allow(unused_imports)]
    use {libc, cpp_utils, std};

    /// C++ type: <span style='color: green;'>```google::protobuf::LogLevel```</span>
    #[derive(Debug, PartialEq, Eq, Clone)]
    #[repr(C)]
    pub enum LogLevel {
      /// C++ variant: LOGLEVEL_INFO
      Info = 0,
      /// C++ variant: LOGLEVEL_WARNING
      Warning = 1,
      /// C++ variant: LOGLEVEL_ERROR
      Error = 2,
      /// C++ variant: LOGLEVEL_FATAL
      Fatal = 3,
    }

    /// C++ type: <span style='color: green;'>```google::protobuf::LogSilencer```</span>.
    #[repr(C)]
    pub struct LogSilencer {
      _buffer: [u8; 1],
    }

    impl ::NewUninitialized for LogSilencer {
      unsafe fn new_uninitialized() -> LogSilencer {
        LogSilencer { _buffer: std::mem::uninitialized() }
      }
    }

    pub mod internal {
      #[allow(unused_imports)]
      use {libc, cpp_utils, std};

      /// C++ type: <span style='color: green;'>```google::protobuf::internal::LogFinisher```</span>.
      #[repr(C)]
      pub struct LogFinisher {
        _buffer: [u8; 1],
      }

      impl ::NewUninitialized for LogFinisher {
        unsafe fn new_uninitialized() -> LogFinisher {
          LogFinisher { _buffer: std::mem::uninitialized() }
        }
      }

      /// C++ type: <span style='color: green;'>```google::protobuf::internal::LogMessage```</span>.
      #[repr(C)]
      pub struct LogMessage {
        _buffer: [u8; 56],
      }

      impl ::NewUninitialized for LogMessage {
        unsafe fn new_uninitialized() -> LogMessage {
          LogMessage { _buffer: std::mem::uninitialized() }
        }
      }

    }

  }

}
