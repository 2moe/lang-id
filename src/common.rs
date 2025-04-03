use crate::{LangID, RawID};

/// `s.trim()` if `is_not_empty()` => `.parse::<LangID>().ok()`
pub fn try_parse_to_langid(s: &str) -> Option<LangID> {
  match s.trim() {
    "" => None,
    x => x.parse().ok(),
  }
}

/// en: (en-Latn-US) English, Latin, United States
pub const fn lang_id_en() -> LangID {
  RawID::new(28261, None, None).into_lang_id()
}
