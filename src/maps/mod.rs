use core::fmt;

use tinystr::TinyAsciiStr;
// mod const_ids;
pub mod description;
pub mod max;
pub mod min;

// territories
pub mod en_001_territory_ids;
pub mod en_001_territory_names;

pub use phf;
// type PhfMap<'a> = phf::Map<&'a str, &'a str>;
type PhfTinyidMap<'a> = phf::OrderedMap<&'a str, TinyID>;
type PhfOrderedMap<'a> = phf::OrderedMap<&'a str, &'a str>;
// type PhfLangidMap<'a> = phf::Map<&'static str, crate::LangID>;

/// Function to convert a string slice to a TinyAsciiStr with a given length N
///
/// ## Example
///
/// ```
/// use lang_id::maps::as_tiny;
///
/// let latin = as_tiny::<4>("Latn");
/// assert_eq!(latin, "Latn");
/// ```
pub const fn as_tiny<const N: usize>(s: &str) -> TinyAsciiStr<N> {
  match TinyAsciiStr::try_from_str(s) {
    Ok(x) => x,
    _ => panic!("Failed to convert as tinystr"),
  }
}

type TinyAsciiID = TinyAsciiStr<4>;
/// Struct representing a language identification code with tiny string
/// components
#[derive(Debug)]
pub struct TinyID {
  pub language: TinyAsciiID,
  pub script: TinyAsciiID,
  pub region: TinyAsciiID,
}

/// Implementing the Display trait to enable printing of a TinyID struct
impl fmt::Display for TinyID {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}-{}-{}", self.language, self.script, self.region)
  }
}

/// Implementation block for the TinyID struct
impl TinyID {
  /// Constructor function for the TinyID struct
  pub const fn new(language: &str, script: &str, region: &str) -> Self {
    Self {
      language: as_tiny::<4>(language),
      script: as_tiny::<4>(script),
      region: as_tiny::<4>(region),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  // #[ignore]
  fn test_description_map() {
    let map = description::map();
    let gsw_fr = map.get("gsw-FR");
    assert_eq!(gsw_fr, Some(&"Schwiizertüütsch, Latiinisch, Frankriich"));

    let ja = map.get("ja");
    assert_eq!(ja, Some(&"日本語, 日本語の文字, 日本"));
  }
}
