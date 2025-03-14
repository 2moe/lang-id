use unic_langid::subtags;

use crate::LangID;

#[derive(Debug)]
pub(crate) struct ID {
  language: u64,
  script: Option<u32>,
  region: Option<u32>,
}

impl ID {
  pub(crate) const fn new(
    language: u64,
    script: Option<u32>,
    region: Option<u32>,
  ) -> Self {
    Self {
      language,
      script,
      region,
    }
  }

  /// Warn: This function is unsafe.
  ///
  /// Since `Option<T>.map(|x| y)` cannot be used in **const fn** in rust 1.85,
  /// use match expressions instead.
  #[inline]
  pub(crate) const fn into_lang_id(self) -> LangID {
    unsafe {
      let language = subtags::Language::from_raw_unchecked(self.language);

      // self.script.map(subtags::Script::from_raw_unchecked)
      let script = match self.script {
        Some(s) => Some(subtags::Script::from_raw_unchecked(s)),
        _ => None,
      };

      let region = match self.region {
        Some(r) => Some(subtags::Region::from_raw_unchecked(r)),
        _ => None,
      };

      LangID::from_raw_parts_unchecked(language, script, region, None)
    }
  }
}
