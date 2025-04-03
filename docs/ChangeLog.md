# ChangeLog

## 0.0.17

- feat(nostd_sys_locale::try_get_system_locale): Preserves original locale string

## 0.0.16

- update feature(match): `["consts"]` => `[]`

## 0.0.15

- update feature(map): `["dep:phf", "compact_str", "consts"]` => `["dep:phf", "compact_str"]`
- add id: bal-Arab-PK

## 0.0.13

- add `RawID::to_bcp47()`
- `RawID::try_from_str` => `const fn`
- update feature: `compact_str = ["dep:compact_str"]`

## 0.0.12

- pub use id::ID as RawID;
- add `RawID::try_from_str()`

## 0.0.11

- modify the description of lzh (Literary Chinese)

## 0.0.10

- add const id: ja-romaji (Nihongo, Hepburn-shiki Rōmaji, Nihon)

## 0.0.9

- add const id: lzh (文言)

## 0.0.8

- add nostd-sys-locale feature

## 0.0.7

- `MaxLangID`: add `Display` impl

## 0.0.6

- add `MaxLangID`
- `TinyID`: + Copy, Hash, PartialEq, Default

## 0.0.5

- add territory_containment_id.rs & territory_containment_name.rs

## 0.0.4

- id.rs: `.get_id()` => `.into_lang_id()`
- add `zh-pinyin` (`zh-Latn-CN`)
- add `lang_id_en_us()`

## 0.0.2

- src/consts: const unsafe fn -> const fn
- src/maps/`*`: PhfMap -> PhfOrderedMap
- add serde feature
