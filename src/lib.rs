// cargo +nightly rustdoc --all-features -- --cfg __unstable_doc
// --document-private-items; open $CARGO_TARGET_DIR/doc/lang_id/index.html
#![cfg_attr(__unstable_doc, feature(doc_auto_cfg, doc_notable_trait))]
#![no_std]

//! This library provides a series of const lang-ids which can be found in the
//! `consts` module. Additionally, it provides some handy maps.
//!
//! # Examples
//!
//! Using the result of a const fn as a value:
//!
//! ```
//! use lang_id::LangID;
//!
//! const DEFAULT_LANG: LangID = unsafe { lang_id::consts::get_en() };
//! ```
//!
//! Maximize:
//!
//! Note: Finding Maximized by `max::map()` does not enumerate all cases.
//!
//! ```
//! let map = lang_id::maps::max::map();
//! let zh = &map["zh"];
//! assert_eq!(zh.language, "zh");
//! assert_eq!(zh.script, "Hans");
//! assert_eq!(zh.region, "CN");
//! ```
//!
//! Minimize:
//!
//! ```
//! let map = lang_id::maps::min::map();
//!
//! let sg = map.get("zh-Hans-SG");
//! assert_eq!(sg, Some(&"zh-SG"));
//! ```
//!
//! Get description of a language:
//!
//! ```
//! let map = lang_id::maps::description::map();
//! let gsw_fr = map.get("gsw-FR");
//! assert_eq!(gsw_fr, Some(&"Schwiizertüütsch, Latiinisch, Frankriich"));
//!
//! let ja = map.get("ja");
//! assert_eq!(ja, Some(&"日本語, 日本語の文字, 日本"));
//! ```
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
extern crate std;

// #[cfg(feature = "alloc")]
// extern crate alloc;

#[cfg(feature = "std")]
#[cfg(test)]
mod tests {
  use std::{borrow::ToOwned, boxed::Box, dbg, println};

  use super::*;
  use crate::matches::match_id;

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
    let default_lang = unsafe { crate::consts::get_en() };
    dbg!(default_lang.language);
  }
}

// #[ignore]
#[cfg(test)]
#[cfg(feature = "std")]
/// ## Example
///
/// ```no_run
/// simple_benchmark(|| { /* do sth. */ })
/// ```
pub(crate) fn simple_benchmark<U, F: FnOnce() -> U>(f: F) {
  let start = std::time::Instant::now();
  f();
  std::eprintln!("Time taken: {:?}", start.elapsed());
}
