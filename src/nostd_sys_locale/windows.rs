use compact_str::CompactString;
use tap::Pipe;
use widestring::U16CStr;
use windows_sys::Win32::Globalization::{GetUserDefaultLocaleName, MAX_LOCALE_NAME};

/// Safely retrieves system locale identifier in BCP-47 format
pub fn get_system_locale<'a>() -> Result<CompactString, &'a str> {
  // Create buffer to store locale name (UTF-16 encoded)
  let mut buffer = [0u16; MAX_LOCALE_NAME as _];

  // Invoke Windows API to get locale information
  let result =
    unsafe { GetUserDefaultLocaleName(buffer.as_mut_ptr(), buffer.len() as i32) }
      as usize;

  // Process API return value
  match result {
    0 => Err("API call failed"), // Use GetLastError() for detailed error
    len if len > buffer.len() => Err("Buffer overflow"),
    // Convert UTF-16 buffer to Rust String
    _ => unsafe { U16CStr::from_ptr_str(buffer.as_ptr()) }
      .to_string_lossy()
      .pipe(CompactString::from)
      .pipe(Ok),
  }
}
