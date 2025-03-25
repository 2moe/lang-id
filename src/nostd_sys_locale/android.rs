use core::ffi::CStr;

use compact_str::{CompactString, ToCompactString};
use libc::PROP_VALUE_MAX as PROP_MAX;

pub fn get_system_property(key: &CStr) -> Option<CompactString> {
  let mut buffer = [0; PROP_MAX as _]; // [c_char; 92]

  let result =
    unsafe { libc::__system_property_get(key.as_ptr(), buffer.as_mut_ptr()) };

  match result {
    1.. => unsafe { CStr::from_ptr(buffer.as_ptr()) }
      .to_string_lossy()
      .to_compact_string()
      .into(),
    _ => None,
  }
}
