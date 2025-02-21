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

  #[allow(clippy::missing_safety_doc)]
  #[allow(unsafe_op_in_unsafe_fn)]
  pub(crate) const unsafe fn get_id(self) -> LangID {
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
