use std::env;

pub use sys_locale::get_locales as retrieve_raw_sys_locales;
use tap::Pipe;

use crate::{LangID, common::try_parse_to_langid};

/// - env::var("LANG")
///   - "C.UTF-8" -> "C" -> parse_to_opt_langid || "en"
///   - "C" -> "C" -> parse_to_opt_langid || "en"
fn get_env_lang_or_en() -> LangID {
  try_get_env_locale(None) //
    .pipe(unwrap_or_en)
}

fn unwrap_or_en(id: Option<LangID>) -> LangID {
  id.unwrap_or_else(crate::common::lang_id_en)
}

/// - env_name: None.unwrap_or("LANG")
///   - env::var("LANG")
///     - "en_US.UTF-8" -> "en_US" -> try_parse_to_langid -> Some("en-US")
///     - zh_CN -> try_parse_to_langid -> Some("zh-CN")
pub fn try_get_env_locale(env_name: Option<&str>) -> Option<LangID> {
  env_name
    .unwrap_or("LANG")
    .pipe(env::var)
    .ok()?
    .pipe_deref(|x| x.split(['.', '@']).next())
    .and_then(try_parse_to_langid)
}

/// On some systems, getting environment variables is faster than fetching
/// sys_locale.
///
/// This function first tries to get `env::var("LANG")`, and if it
/// fails, it then fetches sys_locale.
///
/// - `env::var("LANG")` || system locale || "en"
pub fn retrieve_env_lang_or_sys_locale() -> LangID {
  try_get_env_locale(None) //
    .unwrap_or_else(|| {
      try_retrieve_sys()
        .ok()
        .pipe(unwrap_or_en)
    })
}

/// Returns the current language identification code based on the system's
/// locale settings, falling back to `env::var("LANG")` if necessary.
///
/// - system locale || `env::var("LANG")` || "en"
pub fn retrieve_sys_or_env_lang() -> LangID {
  try_retrieve_sys()
    .ok()
    .unwrap_or_else(get_env_lang_or_en)
}

/// Fetches system's locale and parse to LangID
pub fn try_retrieve_sys() -> Result<LangID, String> {
  match sys_locale::get_locale() {
    Some(raw) => try_parse_to_langid(&raw).ok_or(raw),
    _ => Err("POSIX".into()),
  }
}
// -----------

#[cfg(test)]
#[cfg(feature = "std")]
mod tests {
  use std::dbg;

  use testutils::simple_benchmark;

  use super::*;

  #[test]
  #[ignore]
  /// release: 6.292µs
  /// debug: 15.083µs
  fn bench_get_env_id() {
    simple_benchmark(|| {
      let env_id = try_get_env_locale(None);
      dbg!(env_id)
    })
  }

  #[test]
  #[ignore]
  /// release: 2.043083ms
  /// debug: 3.241833ms
  fn bench_get_locale_id() {
    simple_benchmark(|| {
      let locale_id = retrieve_sys_or_env_lang();
      dbg!(locale_id)
    })
  }

  #[cfg(feature = "consts")]
  #[test]
  fn test_min_langid() {
    let mut id = crate::consts::lang_id_en_gb();
    id.maximize();
    id.minimize();

    if let Some(r) = id.region {
      assert_eq!(r.as_str(), "GB")
    }
    assert_eq!(id.script, None);
  }

  #[test]
  fn test_max_langid() {
    let mut id = crate::consts::lang_id_de();
    id.maximize();
    assert_eq!(id.language, "de");
    if let Some(r) = id.region {
      assert_eq!(r.as_str(), "DE")
    }
    if let Some(s) = id.script {
      assert_eq!(s.as_str(), "Latn")
    }
  }
}
