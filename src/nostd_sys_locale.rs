use core::ffi::CStr;

use compact_str::{CompactString, ToCompactString};

use crate::LangID;

/// Gets the value of an OS environment variable, e.g., `getenv(c"LANG")`.
///
/// # Safety
///
/// This function is unsafe because it calls the [libc::getenv()] function.
/// Ensure that the provided `key` is a valid C-style string.
pub fn getenv(key: &CStr) -> Option<CompactString> {
  unsafe {
    match libc::getenv(key.as_ptr()) {
      p if p.is_null() => None,
      ptr => CStr::from_ptr(ptr)
        .to_string_lossy()
        .to_compact_string()
        .into(),
    }
  }
}

#[cfg(target_os = "android")]
pub mod android;

#[cfg(windows)]
pub mod windows;

pub fn get_raw_sys_locale() -> Option<CompactString> {
  #[allow(unreachable_patterns)]
  match () {
    #[cfg(target_os = "android")]
    () => android::get_system_property(c"persist.sys.locale"),

    #[cfg(all(unix, not(target_os = "android")))]
    () => getenv(c"LANG"),

    #[cfg(windows)]
    () => windows::get_system_locale().ok(),

    () => None,
  }
}

pub fn get_system_locale_id() -> Option<LangID> {
  get_raw_sys_locale()
    .and_then(|s| {
      s.split(['.', '@'])
        .next()
        .map(|x| x.trim().to_compact_string())
    })
    .and_then(|s| match s.as_str() {
      "" => None,
      x => x.parse().ok(),
    })
}

#[cfg(test)]
mod tests {

  use super::*;

  #[ignore]
  #[test]
  #[cfg(feature = "std")]
  fn test_get_system_locale_id() {
    dbg!(get_raw_sys_locale());
    let id = get_system_locale_id();
    dbg!(id);
  }

  #[ignore]
  #[test]
  #[cfg(feature = "std")]
  fn get_env_lang() {
    let lang = getenv(c"LANG");
    println!("{lang:?}");
  }
}
