use std::env;

pub use sys_locale::{get_locale as fetch, get_locales as fetch_locales};
use tap::Pipe;

use crate::{LangID, consts::lang_id_en};

/// - env::var("LANG")
///   - "C.UTF-8" -> "C" -> parse_to_opt_langid || "en"
///   - "C" -> "C" -> parse_to_opt_langid || "en"
fn get_env_lang_or_en() -> LangID {
  try_get_env_locale(None) //
    .pipe(unwrap_or_en)
}

fn unwrap_or_en(id: Option<LangID>) -> LangID {
  id.unwrap_or_else(lang_id_en)
}

/// - env_name: None.unwrap_or("LANG")
///   - env::var("LANG")
///     - "C.UTF-8" -> "C" -> parse_to_opt_langid
///     - "C" -> "C" -> parse_to_opt_langid
pub fn try_get_env_locale(env_name: Option<&str>) -> Option<LangID> {
  env_name
    .unwrap_or("LANG")
    .pipe(env::var)
    .ok()?
    .pipe_deref(|x| x.split('.').next())
    .and_then(parse_to_opt_langid)
}

/// On some systems, getting environment variables is faster than fetching
/// sys_locale.
///
/// This function first tries to get `env::var("LANG")`, and if it
/// fails, it then fetches sys_locale.
///
/// - `env::var("LANG")` || system locale || "en"
pub fn fetch_env_lang_or_sys_locale() -> LangID {
  try_get_env_locale(None) //
    .unwrap_or_else(|| fetch_opt().pipe(unwrap_or_en))
}

/// - If `env::var("LANG")` is Some("en"), then -> "en-Latn-US".
/// - If `env::var("LANG")` is None, then it fetches the system locale
///   (maximised).
///   - If the system locale is "en-GB", then -> "en-Latn-GB".
pub fn fetch_max_env_lang_or_sys_locale() -> LangID {
  try_get_env_locale(None)
    .map(into_max_langid)
    .unwrap_or_else(fetch_max)
}

/// Returns the current language identification code based on the system's
/// locale settings, falling back to `env::var("LANG")` if necessary.
///
/// - system locale || `env::var("LANG")` || "en"
pub fn fetch_sys_or_env_lang() -> LangID {
  fetch_opt().unwrap_or_else(get_env_lang_or_en)
}

/// Maximised/Maximized LangID:
/// - zh => zh-Hans-CN
/// - en => en-Latn-US
/// - de => de-Latn-DE
pub fn fetch_max() -> LangID {
  fetch_opt()
    .map(into_max_langid)
    .pipe(unwrap_or_en)
}

/// Minimised/Minimized LangID:
/// - en-Latn-US => en
/// - en-US      => en
/// - en-Latn-GB => en-GB
/// - zh-Hans-CN => zh
/// - zh-Hant-TW => zh-TW
pub fn fetch_min() -> LangID {
  fetch_opt()
    .map(|mut x| {
      x.minimize();
      x
    })
    .pipe(unwrap_or_en)
}

/// Fetches system's locale and parse to opt LangID
pub fn fetch_opt() -> Option<LangID> {
  fetch()
    .as_deref()
    .and_then(parse_to_opt_langid)
}
// -----------
fn into_max_langid(mut id: LangID) -> LangID {
  id.maximize();
  id
}

fn parse_to_opt_langid(s: &str) -> Option<LangID> {
  match s.trim() {
    "" => None,
    x => x.parse().ok(),
  }
}

#[cfg(test)]
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
      let locale_id = fetch_sys_or_env_lang();
      dbg!(locale_id)
    })
  }

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
