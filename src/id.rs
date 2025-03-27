pub use tinystr::ParseError;
use unic_langid::subtags;

use crate::LangID;
/// RawID
///
/// # Example
///
/// ```
/// use lang_id::{LangID, RawID};
///
/// pub const fn id_und() -> LangID {
///   RawID::new(6581877, None, None).into_lang_id()
/// }
///
/// assert_eq!(id_und().language, "und");
/// ```
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ID {
  pub language: u64,
  pub script: Option<u32>,
  pub region: Option<u32>,
}

impl core::fmt::Display for ID {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let Self {
      language,
      script,
      region,
    } = self;

    write!(
      f,
      r##"RawID::new({language}, {script:?}, {region:?}).into_lang_id()"##,
    )
  }
}

impl ID {
  pub const fn new(language: u64, script: Option<u32>, region: Option<u32>) -> Self {
    Self {
      language,
      script,
      region,
    }
  }
  /// Attempts to construct a Raw language ID from individual BCP 47 components.
  ///
  /// # Example
  ///
  /// ```
  /// use lang_id::RawID;
  ///
  /// let lzh = RawID::try_from_str("lzh", "Hant", "")?;
  ///
  /// assert_eq!(
  ///   lzh.to_string(),
  ///   "RawID::new(6847084, Some(1953390920), None).into_lang_id()"
  /// );
  ///
  /// # Ok::<(), tinystr::ParseError>(())
  /// ```
  pub fn try_from_str(
    language: &str,
    script: &str,
    region: &str,
  ) -> Result<Self, ParseError> {
    use tinystr::TinyAsciiStr as TinyStr;
    type Language = TinyStr<8>;
    type Tiny4 = TinyStr<4>;

    let language = match language {
      "" => return Err(ParseError::ContainsNull),
      x => {
        let tmp = Language::try_from_str(x)?;
        u64::from_le_bytes(*tmp.all_bytes())
      }
    };

    let parse_as_u32 = |s| {
      let bytes = Tiny4::try_from_str(s)?;
      let num = u32::from_le_bytes(*bytes.all_bytes());
      Ok::<_, ParseError>(num)
    };

    let script = match script {
      "" => None,
      x => Some(parse_as_u32(x)?),
    };

    let region = match region {
      "" => None,
      x => Some(parse_as_u32(x)?),
    };

    let id = ID::new(language, script, region);

    Ok(id)
  }

  /// Warn: This function is unsafe.
  ///
  /// Since `Option<T>.map(|x| y)` cannot be used in **const fn** in rust 1.85,
  /// use match expressions instead.
  #[inline]
  pub const fn into_lang_id(self) -> LangID {
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

#[cfg(test)]
mod tests {
  use super::*;

  #[cfg(feature = "std")]
  #[test]
  fn test_build_lzh() -> Result<(), ParseError> {
    let lzh = ID::try_from_str("lzh", "Hant", "")?;
    assert_eq!(
      lzh.to_string(),
      "RawID::new(6847084, Some(1953390920), None).into_lang_id()"
    );
    Ok(())
  }
}
