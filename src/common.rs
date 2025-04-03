use crate::LangID;

/// `s.trim()` if `is_not_empty()` => `.parse::<LangID>().ok()`
pub fn try_parse_to_langid(s: &str) -> Option<LangID> {
  match s.trim() {
    "" => None,
    x => x.parse().ok(),
  }
}
