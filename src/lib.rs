#![cfg_attr(__unstable_doc, feature(doc_auto_cfg, doc_notable_trait))]
#![cfg_attr(not(feature = "std"), no_std)]

/*!
This library provides a series of const lang-ids (language identifiers) which can be found in the
`consts` module. Additionally, it provides some handy maps.

## Examples

Using the result of a const fn as a value:

```
use lang_id::LangID;

// Compile-time verified language ID
const DEFAULT_LANG: LangID = lang_id::consts::lang_id_en();
```

Description-data Lookup (requires map feature)

```
# #[cfg(feature = "map")]
# {
let map = lang_id::maps::description::map();
let gsw_fr = map.get("gsw-FR");
assert_eq!(gsw_fr, Some(&"Schwiizertüütsch, Latiinisch, Frankriich"));

let zh = map.get("zh");
assert_eq!(zh, Some(&"简体中文, 中国"));

let ja = map.get("ja");
assert_eq!(ja, Some(&"日本語, 日本語の文字, 日本"));
# }
```

## Features

| Feature        | Dependencies          | Description                                                                                                          |
| -------------- | --------------------- | -------------------------------------------------------------------------------------------------------------------- |
| **std**        | None                  | Enables stdlib integrations                                                                                          |
| **map**        | `phf`, `tinystr`      | Adds precomputed static maps:<br>- Perfect hash maps for O(1) lookups<br>- Compact string storage with TinyStr |
| **sys-locale** | `sys-locale`, **std** | System locale detection:<br>- Cross-platform locale querying<br>- Integration with OS settings                       |
| **match**      | None                  | Match the value or function name in consts using bytes, for example, b"en-001" => lang_id_en_001().                      |
| **serde**      | `unic-langid/serde`   | Serialization/deserialization support                                                                                |

*/

pub mod consts;
mod id;
pub use unic_langid::LanguageIdentifier as LangID;

pub mod error {
  pub use unic_langid::LanguageIdentifierError as LangidError;
}

#[cfg(feature = "map")]
pub mod maps;

#[cfg(feature = "match")]
pub mod matches;

/// The sys-locale module is used to get the current system's locale and convert
/// it into LangID.
#[cfg(feature = "sys-locale")]
pub mod sys_locale;

#[cfg(feature = "std")]
#[cfg(test)]
mod tests {
  use super::*;

  // #[test]
  // #[cfg(feature = "map")]
  // fn minimize_zh_sg() {
  //   let map = crate::maps::min::map();
  //   let sg = map["zh-Hans-SG"];
  //   assert_eq!(sg, "zh-SG");
  // }
  #[test]
  #[cfg(feature = "map")]
  fn get_const_ids() {
    let now = std::time::Instant::now();
    let map = maps::description::map();
    for (k, v) in &map {
      println!("{k} {v}")
    }
    // let id = match_id(Box::new("en-GB").as_bytes());
    // let ja = map.get("ja");
    // println!("{id:?}");
    // dbg!(id);
    println!("{:?}", now.elapsed());
    // dbg!(id);
  }

  #[test]
  fn test_get_en() {
    let default_lang = crate::consts::lang_id_en();
    dbg!(default_lang.language);
  }
}
