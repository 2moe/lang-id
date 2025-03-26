# ChangeLog

## 0.0.9

- add const id: lzh(文言文)

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
