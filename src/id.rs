#![allow(clippy::missing_safety_doc)]
use crate::LangID;
use unic_langid::subtags::{self, Language};

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

    pub(crate) const unsafe fn get_id(self) -> LangID {
        LangID::from_raw_parts_unchecked(
            Language::from_raw_unchecked(self.language),
            match self.script {
                Some(s) => Some(subtags::Script::from_raw_unchecked(s)),
                _ => None,
            },
            match self.region {
                Some(r) => Some(subtags::Region::from_raw_unchecked(r)),
                _ => None,
            },
            None,
        )
    }
}
