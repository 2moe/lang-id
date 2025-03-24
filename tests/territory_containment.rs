//! The JSON data comes from **unicode-org/cldr-json** and follows the Unicode
//! license.
//! - <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-core/supplemental/territoryContainment.json>
//!
//! License: UNICODE LICENSE V3
//! Copyright © 2004-2025 Unicode, Inc.

#[ignore]
#[test]
fn build_id_map() -> io::Result<()> {
  let content = build_id_map_content();
  println!("{content}");
  fs::write("src/matches/territory_containment_id.rs", content)?;
  Ok(())
}

#[ignore]
#[test]
fn build_name_map() -> io::Result<()> {
  let content = build_name_map_content();
  println!("{content}");
  fs::write("src/matches/territory_containment_name.rs", content)?;
  Ok(())
}

use std::{
  collections::{BTreeMap, HashMap},
  fs, io,
  sync::LazyLock,
};

use compact_str::{CompactString as MiniStr, format_compact};
use itertools::Itertools;
use lang_id::matches;
use serde::Deserialize;
use tap::{Pipe, Tap};

type AnyResult<T> = anyhow::Result<T>;

const fn territories_json() -> &'static str {
  r##"
    {
      "001": {
        "_contains": [
          "019",
          "002",
          "150",
          "142",
          "009"
        ]
      },
      "001-status-deprecated": {
        "_contains": [
          "QU"
        ]
      },
      "001-status-grouping": {
        "_contains": [
          "EU",
          "EZ",
          "UN"
        ]
      },
      "002": {
        "_contains": [
          "015",
          "011",
          "017",
          "014",
          "018"
        ]
      },
      "002-status-grouping": {
        "_contains": [
          "202"
        ]
      },
      "003": {
        "_contains": [
          "021",
          "013",
          "029"
        ],
        "_grouping": "true"
      },
      "005": {
        "_contains": [
          "AR",
          "BO",
          "BR",
          "BV",
          "CL",
          "CO",
          "EC",
          "FK",
          "GF",
          "GS",
          "GY",
          "PE",
          "PY",
          "SR",
          "UY",
          "VE"
        ]
      },
      "009": {
        "_contains": [
          "053",
          "054",
          "057",
          "061",
          "QO"
        ]
      },
      "011": {
        "_contains": [
          "BF",
          "BJ",
          "CI",
          "CV",
          "GH",
          "GM",
          "GN",
          "GW",
          "LR",
          "ML",
          "MR",
          "NE",
          "NG",
          "SH",
          "SL",
          "SN",
          "TG"
        ]
      },
      "013": {
        "_contains": [
          "BZ",
          "CR",
          "GT",
          "HN",
          "MX",
          "NI",
          "PA",
          "SV"
        ]
      },
      "014": {
        "_contains": [
          "BI",
          "DJ",
          "ER",
          "ET",
          "IO",
          "KE",
          "KM",
          "MG",
          "MU",
          "MW",
          "MZ",
          "RE",
          "RW",
          "SC",
          "SO",
          "SS",
          "TF",
          "TZ",
          "UG",
          "YT",
          "ZM",
          "ZW"
        ]
      },
      "015": {
        "_contains": [
          "DZ",
          "EG",
          "EH",
          "LY",
          "MA",
          "SD",
          "TN",
          "EA",
          "IC"
        ]
      },
      "017": {
        "_contains": [
          "AO",
          "CD",
          "CF",
          "CG",
          "CM",
          "GA",
          "GQ",
          "ST",
          "TD"
        ]
      },
      "017-status-deprecated": {
        "_contains": [
          "ZR"
        ]
      },
      "018": {
        "_contains": [
          "BW",
          "LS",
          "NA",
          "SZ",
          "ZA"
        ]
      },
      "019": {
        "_contains": [
          "021",
          "013",
          "029",
          "005"
        ]
      },
      "019-status-grouping": {
        "_contains": [
          "003",
          "419"
        ]
      },
      "021": {
        "_contains": [
          "BM",
          "CA",
          "GL",
          "PM",
          "US"
        ]
      },
      "029": {
        "_contains": [
          "AG",
          "AI",
          "AW",
          "BB",
          "BL",
          "BQ",
          "BS",
          "CU",
          "CW",
          "DM",
          "DO",
          "GD",
          "GP",
          "HT",
          "JM",
          "KN",
          "KY",
          "LC",
          "MF",
          "MQ",
          "MS",
          "PR",
          "SX",
          "TC",
          "TT",
          "VC",
          "VG",
          "VI"
        ]
      },
      "029-status-deprecated": {
        "_contains": [
          "AN"
        ]
      },
      "030": {
        "_contains": [
          "CN",
          "HK",
          "JP",
          "KP",
          "KR",
          "MN",
          "MO",
          "TW"
        ]
      },
      "034": {
        "_contains": [
          "AF",
          "BD",
          "BT",
          "IN",
          "IR",
          "LK",
          "MV",
          "NP",
          "PK"
        ]
      },
      "035": {
        "_contains": [
          "BN",
          "ID",
          "KH",
          "LA",
          "MM",
          "MY",
          "PH",
          "SG",
          "TH",
          "TL",
          "VN"
        ]
      },
      "035-status-deprecated": {
        "_contains": [
          "BU",
          "TP"
        ]
      },
      "039": {
        "_contains": [
          "AD",
          "AL",
          "BA",
          "ES",
          "GI",
          "GR",
          "HR",
          "IT",
          "ME",
          "MK",
          "MT",
          "RS",
          "PT",
          "SI",
          "SM",
          "VA",
          "XK"
        ]
      },
      "039-status-deprecated": {
        "_contains": [
          "CS",
          "YU"
        ]
      },
      "053": {
        "_contains": [
          "AU",
          "CC",
          "CX",
          "HM",
          "NF",
          "NZ"
        ]
      },
      "054": {
        "_contains": [
          "FJ",
          "NC",
          "PG",
          "SB",
          "VU"
        ]
      },
      "057": {
        "_contains": [
          "FM",
          "GU",
          "KI",
          "MH",
          "MP",
          "NR",
          "PW",
          "UM"
        ]
      },
      "061": {
        "_contains": [
          "AS",
          "CK",
          "NU",
          "PF",
          "PN",
          "TK",
          "TO",
          "TV",
          "WF",
          "WS"
        ]
      },
      "142": {
        "_contains": [
          "145",
          "143",
          "030",
          "034",
          "035"
        ]
      },
      "143": {
        "_contains": [
          "TM",
          "TJ",
          "KG",
          "KZ",
          "UZ"
        ]
      },
      "145": {
        "_contains": [
          "AE",
          "AM",
          "AZ",
          "BH",
          "CY",
          "GE",
          "IL",
          "IQ",
          "JO",
          "KW",
          "LB",
          "OM",
          "PS",
          "QA",
          "SA",
          "SY",
          "TR",
          "YE"
        ]
      },
      "145-status-deprecated": {
        "_contains": [
          "NT",
          "YD"
        ]
      },
      "150": {
        "_contains": [
          "154",
          "155",
          "151",
          "039"
        ]
      },
      "151": {
        "_contains": [
          "BG",
          "BY",
          "CZ",
          "HU",
          "MD",
          "PL",
          "RO",
          "RU",
          "SK",
          "UA"
        ]
      },
      "151-status-deprecated": {
        "_contains": [
          "SU"
        ]
      },
      "154": {
        "_contains": [
          "GG",
          "IM",
          "JE",
          "AX",
          "DK",
          "EE",
          "FI",
          "FO",
          "GB",
          "IE",
          "IS",
          "LT",
          "LV",
          "NO",
          "SE",
          "SJ",
          "CQ"
        ]
      },
      "155": {
        "_contains": [
          "AT",
          "BE",
          "CH",
          "DE",
          "FR",
          "LI",
          "LU",
          "MC",
          "NL"
        ]
      },
      "155-status-deprecated": {
        "_contains": [
          "DD",
          "FX"
        ]
      },
      "202": {
        "_contains": [
          "011",
          "017",
          "014",
          "018"
        ],
        "_grouping": "true"
      },
      "419": {
        "_contains": [
          "013",
          "029",
          "005"
        ],
        "_grouping": "true"
      },
      "EU": {
        "_contains": [
          "AT",
          "BE",
          "CY",
          "CZ",
          "DE",
          "DK",
          "EE",
          "ES",
          "FI",
          "FR",
          "GR",
          "HR",
          "HU",
          "IE",
          "IT",
          "LT",
          "LU",
          "LV",
          "MT",
          "NL",
          "PL",
          "PT",
          "SE",
          "SI",
          "SK",
          "BG",
          "RO"
        ],
        "_grouping": "true"
      },
      "QO": {
        "_contains": [
          "AQ",
          "AC",
          "CP",
          "DG",
          "TA"
        ]
      }
  }
  "##
}

#[derive(Default, Deserialize, Debug)]
#[serde(default)]
struct Contains {
  #[serde(rename = "_contains")]
  contains: Vec<MiniStr>,
}

type DecodedData = BTreeMap<MiniStr, Vec<MiniStr>>;
fn decode_raw_json() -> AnyResult<DecodedData> {
  let data: HashMap<MiniStr, Contains> = serde_json::from_str(territories_json())?;

  data
    .into_iter()
    .filter(|(k, _)| !k.contains('-') && !["UN", "EZ", "EU"].contains(&k.as_str()))
    .map(|(k, v)| (k, v.contains))
    .collect::<BTreeMap<_, _>>()
    .pipe(Ok)
}

fn static_decoded_data() -> &'static DecodedData {
  static S: LazyLock<DecodedData> =
    LazyLock::new(|| decode_raw_json().expect("Failed to decode json"));
  &S
}

#[ignore]
#[test]
fn test_show_decoded_data() {
  dbg!(static_decoded_data());
}

fn normalize_doc(s: &str) -> String {
  s.trim_ascii()
    .lines()
    .flat_map(|s| ["/// ", s, "\n"])
    .collect()
}

fn normalize_id_map_doc() -> String {
  r##"
Provides compile-time lookup of territory containment relationships.

Returns static string slices representing child territories/regions for a
given parent region ID according to the Unicode CLDR territory containment
specification.

## Example

```
use lang_id::matches::territory_containment_id::id_mapping;

// Lookup Latin America containment
let latin_america = id_mapping(b"419");
assert_eq!(latin_america, &["013", "029", "005"]);

let south_america = id_mapping(b"005");
assert_eq!(south_america, &[
  "AR", "BO", "BR", "BV", "CL", "CO", "EC", "FK", "GF", "GS", "GY", "PE", "PY",
  "SR", "UY", "VE",
]);

// Unknown code returns empty slice
assert!(id_mapping(b"999").is_empty());
```
  "##
    .pipe(normalize_doc)
}

fn build_id_map_content() -> String {
  let header = || {
    let s = r##"pub const fn id_mapping(id: &[u8]) -> &[&str] {
  match id {
  "##;
    String::with_capacity(8192).tap_mut(|buf| {
      for content in [&normalize_id_map_doc(), s] {
        buf.push_str(content)
      }
    })
  };

  static_decoded_data()
    .iter()
    .map(|(k, v)| format_compact!("    b\"{k}\" => &{v:?},"))
    .fold(header(), |mut acc, x| {
      acc.push('\n');
      acc.push_str(&x);
      acc
    })
    .tap_mut(|buf| {
      buf.push_str(
        "
    _ => &[],
    }}",
      )
    })
}

#[ignore]
#[test]
fn test_id_mapping() {
  let id_419_values = matches::territory_containment_id::id_mapping(b"419");
  assert_eq!(id_419_values, &["013", "029", "005"]);
}

fn normalize_name_map_doc() -> String {
  r##"
Provides compile-time mapping between ISO 3166-1 alpha-2 codes and
UN M.49 region containment hierarchies. This constant function
enables zero-cost geographical hierarchy lookups according to CLDR
specifications.

## Example

```
use lang_id::matches::territory_containment_name::name_mapping;

// China (CN) belongs to region 030 (Eastern Asia)
let china_hierarchy = name_mapping(b"CN");
assert_eq!(china_hierarchy, &["030"]);

// France (FR) belongs to region 155 (Western Europe)
let france_hierarchy = name_mapping(b"FR");
assert_eq!(france_hierarchy, &["155"]);

// Region 155 (Western Europe) is part of region 150 (Europe)
let eu = name_mapping(b"155");
assert_eq!(eu, &["150"]);

// Unknown code returns empty slice
assert!(name_mapping(b"999").is_empty());
```
  "##
    .pipe(normalize_doc)
}

fn build_name_map_content() -> String {
  let header = || {
    let s = r##"pub const fn name_mapping(name: &[u8]) -> &[&str] {
  match name {
  "##;
    String::with_capacity(8192).tap_mut(|buf| {
      for content in [&normalize_name_map_doc(), s] {
        buf.push_str(content)
      }
    })
  };

  static_decoded_data()
    .iter()
    .flat_map(|(k, values)| {
      values
        .iter()
        .map(move |v| (v, k))
    })
    .into_group_map()
    .into_iter()
    .sorted_by_key(|&(v, _)| v)
    .map(|(v, ks)| {
      let keys_str = ks
        .iter()
        .sorted()
        .map(|k| format_compact!("\"{k}\""))
        .join(", ");
      format_compact!(r#"    b"{v}" => &[{keys_str}],"#)
    })
    .fold(header(), |mut acc, line| {
      acc.push('\n');
      acc.push_str(&line);
      acc
    })
    .tap_mut(|buf| {
      buf.push_str(
        r#"
    _ => &[],
    }}"#,
      )
    })
}
