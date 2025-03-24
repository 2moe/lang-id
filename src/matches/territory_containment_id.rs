/// Provides compile-time lookup of territory containment relationships.
///
/// Returns static string slices representing child territories/regions for a
/// given parent region ID according to the Unicode CLDR territory containment
/// specification.
///
/// ## Example
///
/// ```
/// use lang_id::matches::territory_containment_id::id_mapping;
///
/// // Lookup Latin America containment
/// let latin_america = id_mapping(b"419");
/// assert_eq!(latin_america, &["013", "029", "005"]);
///
/// let south_america = id_mapping(b"005");
/// assert_eq!(south_america, &[
///   "AR", "BO", "BR", "BV", "CL", "CO", "EC", "FK", "GF", "GS", "GY", "PE", "PY",
///   "SR", "UY", "VE",
/// ]);
///
/// // Unknown code returns empty slice
/// assert!(id_mapping(b"999").is_empty());
/// ```
pub const fn id_mapping(id: &[u8]) -> &[&str] {
  match id {
    b"001" => &["019", "002", "150", "142", "009"],
    b"002" => &["015", "011", "017", "014", "018"],
    b"003" => &["021", "013", "029"],
    b"005" => &[
      "AR", "BO", "BR", "BV", "CL", "CO", "EC", "FK", "GF", "GS", "GY", "PE", "PY",
      "SR", "UY", "VE",
    ],
    b"009" => &["053", "054", "057", "061", "QO"],
    b"011" => &[
      "BF", "BJ", "CI", "CV", "GH", "GM", "GN", "GW", "LR", "ML", "MR", "NE", "NG",
      "SH", "SL", "SN", "TG",
    ],
    b"013" => &["BZ", "CR", "GT", "HN", "MX", "NI", "PA", "SV"],
    b"014" => &[
      "BI", "DJ", "ER", "ET", "IO", "KE", "KM", "MG", "MU", "MW", "MZ", "RE", "RW",
      "SC", "SO", "SS", "TF", "TZ", "UG", "YT", "ZM", "ZW",
    ],
    b"015" => &["DZ", "EG", "EH", "LY", "MA", "SD", "TN", "EA", "IC"],
    b"017" => &["AO", "CD", "CF", "CG", "CM", "GA", "GQ", "ST", "TD"],
    b"018" => &["BW", "LS", "NA", "SZ", "ZA"],
    b"019" => &["021", "013", "029", "005"],
    b"021" => &["BM", "CA", "GL", "PM", "US"],
    b"029" => &[
      "AG", "AI", "AW", "BB", "BL", "BQ", "BS", "CU", "CW", "DM", "DO", "GD", "GP",
      "HT", "JM", "KN", "KY", "LC", "MF", "MQ", "MS", "PR", "SX", "TC", "TT", "VC",
      "VG", "VI",
    ],
    b"030" => &["CN", "HK", "JP", "KP", "KR", "MN", "MO", "TW"],
    b"034" => &["AF", "BD", "BT", "IN", "IR", "LK", "MV", "NP", "PK"],
    b"035" => &[
      "BN", "ID", "KH", "LA", "MM", "MY", "PH", "SG", "TH", "TL", "VN",
    ],
    b"039" => &[
      "AD", "AL", "BA", "ES", "GI", "GR", "HR", "IT", "ME", "MK", "MT", "RS", "PT",
      "SI", "SM", "VA", "XK",
    ],
    b"053" => &["AU", "CC", "CX", "HM", "NF", "NZ"],
    b"054" => &["FJ", "NC", "PG", "SB", "VU"],
    b"057" => &["FM", "GU", "KI", "MH", "MP", "NR", "PW", "UM"],
    b"061" => &["AS", "CK", "NU", "PF", "PN", "TK", "TO", "TV", "WF", "WS"],
    b"142" => &["145", "143", "030", "034", "035"],
    b"143" => &["TM", "TJ", "KG", "KZ", "UZ"],
    b"145" => &[
      "AE", "AM", "AZ", "BH", "CY", "GE", "IL", "IQ", "JO", "KW", "LB", "OM", "PS",
      "QA", "SA", "SY", "TR", "YE",
    ],
    b"150" => &["154", "155", "151", "039"],
    b"151" => &["BG", "BY", "CZ", "HU", "MD", "PL", "RO", "RU", "SK", "UA"],
    b"154" => &[
      "GG", "IM", "JE", "AX", "DK", "EE", "FI", "FO", "GB", "IE", "IS", "LT", "LV",
      "NO", "SE", "SJ", "CQ",
    ],
    b"155" => &["AT", "BE", "CH", "DE", "FR", "LI", "LU", "MC", "NL"],
    b"202" => &["011", "017", "014", "018"],
    b"419" => &["013", "029", "005"],
    b"QO" => &["AQ", "AC", "CP", "DG", "TA"],
    _ => &[],
  }
}
