#![cfg_attr(__lang_id_doc, feature(doc_auto_cfg, doc_notable_trait))]

//! This library provides a series of const lang-ids which can be found in the `consts` module.
//! Additionally, it provides some handy maps.
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
//! Note: Finding Maximized by MAX_MAP does not enumerate all cases.
//!
//! ```
//! let map = lang_id::map::max::MAX_MAP;
//! let zh = &map["zh"];
//! assert_eq!(zh.language, "zh");
//! assert_eq!(zh.script, "Hans");
//! assert_eq!(zh.region, "CN");
//! ```
//!
//! Minimize:
//!
//! ```
//! let map = lang_id::map::min::min_map();
//!
//! let sg = map.get("zh-Hans-SG");
//! assert_eq!(sg, Some(&"zh-SG"));
//! ```
//!
//! Get description of a language:
//!
//! ```
//! let map = lang_id::map::description::desc_map();
//! let ja = map.get("ja");
//! assert_eq!(ja, Some(&"日本語, 日本語の文字, 日本"));
//! ```
pub mod consts;
mod id;
pub type LangID = unic_langid::LanguageIdentifier;

#[cfg(feature = "map")]
pub mod map;

#[cfg(feature = "match")]
pub mod get;

/// The sys-lang module is used to get the current system's locale and convert it into LangID.
#[cfg(feature = "sys-lang")]
pub mod sys_lang;

#[cfg(test)]
mod tests {

    #[test]
    #[cfg(feature = "map")]
    fn minimize_zh_sg() {
        let map = crate::map::min::min_map();
        let sg = map["zh-Hans-SG"];
        assert_eq!(sg, "zh-SG");
    }

    #[test]
    fn test_get_en() {
        let default_lang = unsafe { crate::consts::get_en() };
        dbg!(default_lang.language);
    }
}
