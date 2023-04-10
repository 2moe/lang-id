use std::env;

use crate::{consts::get_en, LangID};
pub use sys_locale::get_locale;

fn get_env_lang() -> LangID {
    env::var("LANG")
        .iter()
        .find_map(|x| x.split('.').next())
        .and_then(|x| x.parse::<LangID>().ok())
        .unwrap_or_else(|| unsafe { get_en() })
}

/// Returns the current language identification code based on the system's locale settings.
///
/// The function calls `sys_locale::get_locale()` to get the system's locale settings as a string,
/// then attempts to parse the string as a `LangID` enum variant using the `FromStr` trait.
/// If the parsing succeeds, the resulting `LangID` variant is returned. If the parsing fails,
/// the function falls back to returning the English (`en`) `LangID` variant.
///
/// # Example
///
/// ```
/// use lang_id::sys_lang;
///
/// let cur = sys_lang::current();
/// dbg!(cur);
/// ```
pub fn current() -> LangID {
    get_opt_id().unwrap_or_else(get_env_lang) // If parsing fails, return the English LangID variant
}

/// Gets the system locale string, if None, then returns "en".
pub fn get_locale_or_default() -> String {
    get_locale()
        .unwrap_or_else(|| env::var("LANG").unwrap_or_else(|_| "en".to_owned()))
}

fn get_opt_id() -> Option<LangID> {
    get_locale() // Get system's locale settings as a string
        .and_then(|x| x.parse::<LangID>().ok()) // Attempt to parse the string as a LangID variant
}

/// # Example
///
/// ```
/// use lang_id::sys_lang;
///
/// let cur = sys_lang::get_current_maximised();
/// dbg!(cur);
/// ```
pub fn get_current_maximised() -> LangID {
    get_opt_id()
        .map(|mut s| {
            s.maximize();
            s
        })
        .unwrap_or_else(|| unsafe { get_en() })
}

/// # Example
///
/// ```
/// use lang_id::sys_lang;
///
/// let cur = sys_lang::get_current_minimised();
/// dbg!(cur);
/// ```
pub fn get_current_minimised() -> LangID {
    get_opt_id()
        .map(|mut s| {
            s.minimize();
            s
        })
        .unwrap_or_else(|| unsafe { get_en() })
}
