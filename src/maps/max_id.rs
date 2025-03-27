use compact_str::ToCompactString;
use tap::{Pipe, Tap};

use crate::{LangID, maps::TinyID};

/// Represents a maximized language identifier with memory-efficient storage
/// variants
///
/// Combines standard Unicode locale identifiers with optimized tiny
/// representations for common cases, following Unicode CLDR's locale
/// maximization rules.
///
/// # Variants
/// - `Regular`: Regular Language identifier.
/// - `Tiny`: Precomputed optimized representation for frequent locales
#[derive(Debug, Clone)]
pub enum MaxLangID {
  Regular(LangID),
  Tiny(TinyID),
}

impl core::fmt::Display for MaxLangID {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(
      f,
      "{}-{}-{}",
      self.get_language(),
      self.get_script(),
      self.get_region()
    )
  }
}

impl MaxLangID {
  /// Constructs a maximized locale identifier using CLDR supplementation data
  ///
  /// # Behavior
  /// 1. Checks against precomputed frequent locale map
  /// 2. Returns optimized TinyID if exists in map
  /// 3. Otherwise maximizes the input ID and stores full representation
  ///
  /// # Examples
  /// ```
  /// # #[cfg(feature = "consts")] {
  /// use lang_id::maps::MaxLangID;
  ///
  /// let id = lang_id::consts::lang_id_en();
  /// assert_eq!(id.region, None);
  ///
  /// let max_id = MaxLangID::new(&id);
  ///
  /// assert_eq!(max_id.get_region(), "US");
  /// # }
  /// ```
  pub fn new(language: &LangID) -> Self {
    let map = crate::maps::max::map();

    match map.get(&language.to_compact_string()) {
      Some(id) => Self::Tiny(*id),
      _ => language
        .clone()
        .tap_mut(|x| {
          x.maximize();
        })
        .pipe(MaxLangID::Regular),
    }
  }

  /// Retrieves the base language code (ISO 639 alpha-2/3)
  ///
  /// ## Returns
  ///
  /// - Normalized lowercase language code
  /// - Empty string for invalid/unknown codes
  pub fn get_language(&self) -> &str {
    match self {
      Self::Regular(id) => id.language.as_str(),
      Self::Tiny(id) => id.language.as_str(),
    }
  }

  /// Retrieves the script code if available
  pub fn get_script(&self) -> &str {
    match self {
      Self::Regular(id) => match &id.script {
        Some(s) => s.as_str(),
        _ => "",
      },
      Self::Tiny(id) => id.script.as_str(),
    }
  }

  /// Retrieves the region code if available
  ///
  /// ## Returns
  ///
  /// - Uppercase region code when explicitly defined
  /// - Empty string for inferred regions
  pub fn get_region(&self) -> &str {
    match self {
      Self::Regular(id) => match &id.region {
        Some(s) => s.as_str(),
        _ => "",
      },
      Self::Tiny(id) => id.region.as_str(),
    }
  }
}
