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
/// const fn id_und() -> LangID {
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

    match () {
      #[cfg(not(feature = "compact_str"))]
      () => write!(
        f,
        r##"RawID::new({language}, {script:?}, {region:?}).into_lang_id()"##,
      ),
      #[cfg(feature = "compact_str")]
      () => write!(
        f,
        r##"/*{}*/ RawID::new({language}, {script:?}, {region:?}).into_lang_id()"##,
        self.to_bcp47(),
      ),
    }
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
  pub const fn try_from_str(
    language: &str,
    script: &str,
    region: &str,
  ) -> Result<Self, ParseError> {
    use tinystr::TinyAsciiStr as TinyStr;
    type Language = TinyStr<8>;
    type Tiny4 = TinyStr<4>;

    let language = match language.is_empty() {
      true => return Err(ParseError::ContainsNull),
      _ => {
        let tmp = match Language::try_from_str(language) {
          Ok(s) => s,
          Err(e) => return Err(e),
        };
        u64::from_le_bytes(*tmp.all_bytes())
      }
    };

    let script = match script.is_empty() {
      true => None,
      _ => Some({
        let str = match Tiny4::try_from_str(script) {
          Ok(s) => s,
          Err(e) => return Err(e),
        };
        u32::from_le_bytes(*str.all_bytes())
      }),
    };

    let region = match region.is_empty() {
      true => None,
      _ => Some({
        let str = match Tiny4::try_from_str(region) {
          Ok(s) => s,
          Err(e) => return Err(e),
        };
        u32::from_le_bytes(*str.all_bytes())
      }),
    };

    let id = ID::new(language, script, region);

    Ok(id)
  }

  #[cfg(feature = "compact_str")]
  /// Converts the language identifier to a BCP47-compliant string
  /// representation.
  ///
  /// Constructs a string in `language[-script][-region]` format.
  ///
  /// ```
  /// use lang_id::RawID;
  ///
  /// let id = RawID::try_from_str("es", "Latn", "419")?;
  /// let bcp47 = id.to_bcp47();
  /// assert_eq!(bcp47, "es-Latn-419");
  /// # Ok::<(), tinystr::ParseError>(())
  /// ```
  pub fn to_bcp47(&self) -> compact_str::CompactString {
    use compact_str::{CompactString, format_compact};
    let id = self.into_lang_id();

    let LangID {
      language,
      script,
      region,
      ..
    } = id;

    let empty_str = || CompactString::const_new("");
    format_compact!(
      "{language}{}{}",
      match script {
        Some(s) => format_compact!("-{s}"),
        _ => empty_str(),
      },
      match region {
        Some(s) => format_compact!("-{s}"),
        _ => empty_str(),
      }
    )
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

  #[test]
  fn test_build_lzh() -> Result<(), ParseError> {
    let lzh = ID::try_from_str("lzh", "Hant", "")?;
    assert_eq!(
      lzh.to_string(),
      "RawID::new(6847084, Some(1953390920), None).into_lang_id()"
    );
    Ok(())
  }

  #[test]
  #[cfg(feature = "compact_str")]
  fn test_to_bcp47() -> Result<(), ParseError> {
    let id = ID::try_from_str("es", "Latn", "419")?;
    let bcp47 = id.to_bcp47();
    assert_eq!(bcp47, "es-Latn-419");
    // dbg!(bcp47);
    Ok(())
  }
}
