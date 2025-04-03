use core::ffi::CStr;

use compact_str::{CompactString, ToCompactString};
use tap::Pipe;

use crate::{LangID, common::try_parse_to_langid};

/// Gets the value of an OS environment variable, e.g., `getenv(c"LANG")`.
///
/// # Safety
///
/// This function is unsafe because it calls the [libc::getenv()] function.
/// Ensure that the provided `key` is a valid C-style string.
///
///
/// # Example
///
/// ```
/// // #[cfg(no_std)]
/// let _sys_lang = lang_id::nostd_sys_locale::getenv(c"LANG");
/// ```
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

pub(crate) fn get_raw_sys_locale() -> Option<CompactString> {
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

/// Attempts to retrieve and parse the system locale identifier
///
/// # Returns
/// - `Ok(LangID)`: Successfully parsed language identifier
/// - `Err(CompactString)`: Original locale string if parsing fails, or "POSIX"
///   if `get_raw_sys_locale()` fails
///
/// # Error Handling
/// - Returns "POSIX" when no system locale detected (common in minimal Unix
///   environments)
/// - Preserves original locale string in errors for diagnostics
///
/// # Example
///
/// ```
/// use lang_id::nostd_sys_locale::try_get_system_locale;
///
/// let _sys_locale_id = try_get_system_locale();
/// ```
pub fn try_get_system_locale() -> Result<LangID, CompactString> {
  match get_raw_sys_locale().and_then(|s| {
    s.split(['.', '@'])
      .next()
      .map(|x| x.trim().to_compact_string())
  }) {
    Some(raw) => try_parse_to_langid(&raw).ok_or(raw),
    _ => "POSIX"
      .pipe(CompactString::const_new)
      .pipe(Err),
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[ignore]
  #[test]
  #[cfg(feature = "std")]
  fn test_get_system_locale_id() {
    dbg!(get_raw_sys_locale());
    let id = try_get_system_locale();
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
