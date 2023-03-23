use crate::{consts::get_en, LangID};
use std::str::FromStr;
use sys_locale::get_locale;

/// Returns the current language identification code based on the system's locale settings.
///
/// The function calls `sys_locale::get_locale()` to get the system's locale settings as a string,
/// then attempts to parse the string as a `LangID` enum variant using the `FromStr` trait.
/// If the parsing succeeds, the resulting `LangID` variant is returned. If the parsing fails,
/// the function falls back to returning the English (`en`) `LangID` variant.
/// 
/// Examples
/// 
/// ```
/// use lang_id::sys_lang;
/// 
/// let cur = sys_lang::current();
/// dbg!(cur);
/// ```
pub fn current() -> LangID {
    get_locale() // Get system's locale settings as a string
        .and_then(|x| LangID::from_str(&x).ok()) // Attempt to parse the string as a LangID variant
        .unwrap_or(unsafe { get_en() }) // If parsing fails, return the English LangID variant
}