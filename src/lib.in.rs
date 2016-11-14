pub extern crate libc;
pub extern crate cpp_utils;

trait NewUninitialized {
  unsafe fn new_uninitialized() -> Self;
}
#[allow(dead_code)]
mod ffi {
  include!(concat!(env!("OUT_DIR"), "/src/ffi.rs"));
}
pub mod allocator {
  include!(concat!(env!("OUT_DIR"), "/src/allocator.rs"));
}
pub mod arena {
  include!(concat!(env!("OUT_DIR"), "/src/arena.rs"));
}
pub mod arenastring {
  include!(concat!(env!("OUT_DIR"), "/src/arenastring.rs"));
}
pub mod asyoutypeformatter {
  include!(concat!(env!("OUT_DIR"), "/src/asyoutypeformatter.rs"));
}
pub mod atomic_sequence_num {
  include!(concat!(env!("OUT_DIR"), "/src/atomic_sequence_num.rs"));
}
pub mod atomicops_internals_x86_gcc {
  include!(concat!(env!("OUT_DIR"), "/src/atomicops_internals_x86_gcc.rs"));
}
pub mod basic_string {
  include!(concat!(env!("OUT_DIR"), "/src/basic_string.rs"));
}
pub mod callback {
  include!(concat!(env!("OUT_DIR"), "/src/callback.rs"));
}
pub mod char_traits {
  include!(concat!(env!("OUT_DIR"), "/src/char_traits.rs"));
}
pub mod common {
  include!(concat!(env!("OUT_DIR"), "/src/common.rs"));
}
pub mod cpp_type_traits {
  include!(concat!(env!("OUT_DIR"), "/src/cpp_type_traits.rs"));
}
pub mod ctype_base {
  include!(concat!(env!("OUT_DIR"), "/src/ctype_base.rs"));
}
pub mod cxxabi_forced {
  include!(concat!(env!("OUT_DIR"), "/src/cxxabi_forced.rs"));
}
pub mod exception {
  include!(concat!(env!("OUT_DIR"), "/src/exception.rs"));
}
pub mod extension_set {
  include!(concat!(env!("OUT_DIR"), "/src/extension_set.rs"));
}
pub mod ios_base {
  include!(concat!(env!("OUT_DIR"), "/src/ios_base.rs"));
}
pub mod libio {
  include!(concat!(env!("OUT_DIR"), "/src/libio.rs"));
}
pub mod limits {
  include!(concat!(env!("OUT_DIR"), "/src/limits.rs"));
}
pub mod locale {
  include!(concat!(env!("OUT_DIR"), "/src/locale.rs"));
}
pub mod locale_classes {
  include!(concat!(env!("OUT_DIR"), "/src/locale_classes.rs"));
}
pub mod locale_facets {
  include!(concat!(env!("OUT_DIR"), "/src/locale_facets.rs"));
}
pub mod lock {
  include!(concat!(env!("OUT_DIR"), "/src/lock.rs"));
}
pub mod lock_posix {
  include!(concat!(env!("OUT_DIR"), "/src/lock_posix.rs"));
}
pub mod logging {
  include!(concat!(env!("OUT_DIR"), "/src/logging.rs"));
}
pub mod message_lite {
  include!(concat!(env!("OUT_DIR"), "/src/message_lite.rs"));
}
pub mod mutex {
  include!(concat!(env!("OUT_DIR"), "/src/mutex.rs"));
}
pub mod new {
  include!(concat!(env!("OUT_DIR"), "/src/new.rs"));
}
pub mod once {
  include!(concat!(env!("OUT_DIR"), "/src/once.rs"));
}
pub mod phonemetadata {
  include!(concat!(env!("OUT_DIR"), "/src/phonemetadata.rs"));
}
pub mod phonenumber {
  include!(concat!(env!("OUT_DIR"), "/src/phonenumber.rs"));
}
pub mod phonenumbermatch {
  include!(concat!(env!("OUT_DIR"), "/src/phonenumbermatch.rs"));
}
pub mod phonenumbermatcher {
  include!(concat!(env!("OUT_DIR"), "/src/phonenumbermatcher.rs"));
}
pub mod phonenumberutil {
  include!(concat!(env!("OUT_DIR"), "/src/phonenumberutil.rs"));
}
pub mod port {
  include!(concat!(env!("OUT_DIR"), "/src/port.rs"));
}
pub mod predefined_ops {
  include!(concat!(env!("OUT_DIR"), "/src/predefined_ops.rs"));
}
pub mod pthread {
  include!(concat!(env!("OUT_DIR"), "/src/pthread.rs"));
}
pub mod pthreadtypes {
  include!(concat!(env!("OUT_DIR"), "/src/pthreadtypes.rs"));
}
pub mod regexp_adapter {
  include!(concat!(env!("OUT_DIR"), "/src/regexp_adapter.rs"));
}
pub mod regexp_cache {
  include!(concat!(env!("OUT_DIR"), "/src/regexp_cache.rs"));
}
pub mod repeated_field {
  include!(concat!(env!("OUT_DIR"), "/src/repeated_field.rs"));
}
pub mod sched {
  include!(concat!(env!("OUT_DIR"), "/src/sched.rs"));
}
pub mod scoped_ptr {
  include!(concat!(env!("OUT_DIR"), "/src/scoped_ptr.rs"));
}
pub mod shortnumberinfo {
  include!(concat!(env!("OUT_DIR"), "/src/shortnumberinfo.rs"));
}
pub mod sigaction {
  include!(concat!(env!("OUT_DIR"), "/src/sigaction.rs"));
}
pub mod sigcontext {
  include!(concat!(env!("OUT_DIR"), "/src/sigcontext.rs"));
}
pub mod siginfo {
  include!(concat!(env!("OUT_DIR"), "/src/siginfo.rs"));
}
pub mod sigstack {
  include!(concat!(env!("OUT_DIR"), "/src/sigstack.rs"));
}
pub mod singleton_posix {
  include!(concat!(env!("OUT_DIR"), "/src/singleton_posix.rs"));
}
pub mod stdexcept {
  include!(concat!(env!("OUT_DIR"), "/src/stdexcept.rs"));
}
pub mod stdlib {
  include!(concat!(env!("OUT_DIR"), "/src/stdlib.rs"));
}
pub mod stl_bvector {
  include!(concat!(env!("OUT_DIR"), "/src/stl_bvector.rs"));
}
pub mod stl_function {
  include!(concat!(env!("OUT_DIR"), "/src/stl_function.rs"));
}
pub mod stl_iterator_base_types {
  include!(concat!(env!("OUT_DIR"), "/src/stl_iterator_base_types.rs"));
}
pub mod stl_list {
  include!(concat!(env!("OUT_DIR"), "/src/stl_list.rs"));
}
pub mod stl_pair {
  include!(concat!(env!("OUT_DIR"), "/src/stl_pair.rs"));
}
pub mod stl_set {
  include!(concat!(env!("OUT_DIR"), "/src/stl_set.rs"));
}
pub mod stl_tree {
  include!(concat!(env!("OUT_DIR"), "/src/stl_tree.rs"));
}
pub mod stl_vector {
  include!(concat!(env!("OUT_DIR"), "/src/stl_vector.rs"));
}
pub mod template_util {
  include!(concat!(env!("OUT_DIR"), "/src/template_util.rs"));
}
pub mod thread_checker {
  include!(concat!(env!("OUT_DIR"), "/src/thread_checker.rs"));
}
pub mod time {
  include!(concat!(env!("OUT_DIR"), "/src/time.rs"));
}
pub mod timex {
  include!(concat!(env!("OUT_DIR"), "/src/timex.rs"));
}
pub mod typeinfo {
  include!(concat!(env!("OUT_DIR"), "/src/typeinfo.rs"));
}
pub mod ucontext {
  include!(concat!(env!("OUT_DIR"), "/src/ucontext.rs"));
}
pub mod unicodestring {
  include!(concat!(env!("OUT_DIR"), "/src/unicodestring.rs"));
}
pub mod unicodetext {
  include!(concat!(env!("OUT_DIR"), "/src/unicodetext.rs"));
}
pub mod xlocale {
  include!(concat!(env!("OUT_DIR"), "/src/xlocale.rs"));
}
