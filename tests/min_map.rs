mod builder;
use std::io;

use crate::builder::map_type::MapType;

#[test]
#[ignore]
fn new_map() -> io::Result<()> {
  let mod_name = "min";
  let str_kv = &[
    ("af", "af"),
    ("af-NA", "af-NA"),
    ("agq", "agq"),
    ("ak", "ak"),
    ("am", "am"),
    ("ann", "ann"),
    ("ar", "ar"),
    ("ar-AE", "ar-AE"),
    ("ar-BH", "ar-BH"),
    ("ar-DJ", "ar-DJ"),
    ("ar-DZ", "ar-DZ"),
    ("ar-EG", "ar"),
    ("ar-EH", "ar-EH"),
    ("ar-ER", "ar-ER"),
    ("ar-IL", "ar-IL"),
    ("ar-IQ", "ar-IQ"),
    ("ar-JO", "ar-JO"),
    ("ar-KM", "ar-KM"),
    ("ar-KW", "ar-KW"),
    ("ar-LB", "ar-LB"),
    ("ar-LY", "ar-LY"),
    ("ar-MA", "ar-MA"),
    ("ar-MR", "ar-MR"),
    ("ar-OM", "ar-OM"),
    ("ar-PS", "ar-PS"),
    ("ar-QA", "ar-QA"),
    ("ar-SA", "ar-SA"),
    ("ar-SD", "ar-SD"),
    ("ar-SO", "ar-SO"),
    ("ar-SS", "ar-SS"),
    ("ar-SY", "ar-SY"),
    ("ar-TD", "ar-TD"),
    ("ar-TN", "ar-TN"),
    ("ar-YE", "ar-YE"),
    ("as", "as"),
    ("asa", "asa"),
    ("ast", "ast"),
    ("az", "az"),
    ("az-Cyrl", "az-Cyrl"),
    ("az-Latn", "az"),
    ("bas", "bas"),
    ("be", "be"),
    ("bem", "bem"),
    ("bez", "bez"),
    ("bg", "bg"),
    ("bgc", "bgc"),
    ("bho", "bho"),
    ("bm", "bm"),
    ("bn", "bn"),
    ("bn-IN", "bn-IN"),
    ("bo", "bo"),
    ("bo-IN", "bo-IN"),
    ("br", "br"),
    ("brx", "brx"),
    ("bs", "bs"),
    ("bs-Cyrl", "bs-Cyrl"),
    ("bs-Latn", "bs"),
    ("ca", "ca"),
    ("ca-AD", "ca-AD"),
    ("ca-FR", "ca-FR"),
    ("ca-IT", "ca-IT"),
    ("ccp", "ccp"),
    ("ccp-IN", "ccp-IN"),
    ("ce", "ce"),
    ("ceb", "ceb"),
    ("cgg", "cgg"),
    ("chr", "chr"),
    ("ckb", "ckb"),
    ("ckb-IR", "ckb-IR"),
    ("cs", "cs"),
    ("cv", "cv"),
    ("cy", "cy"),
    ("da", "da"),
    ("da-GL", "da-GL"),
    ("dav", "dav"),
    ("de", "de"),
    ("de-AT", "de-AT"),
    ("de-BE", "de-BE"),
    ("de-CH", "de-CH"),
    ("de-IT", "de-IT"),
    ("de-LI", "de-LI"),
    ("de-LU", "de-LU"),
    ("dje", "dje"),
    ("doi", "doi"),
    ("dsb", "dsb"),
    ("dua", "dua"),
    ("dyo", "dyo"),
    ("dz", "dz"),
    ("ebu", "ebu"),
    ("ee", "ee"),
    ("ee-TG", "ee-TG"),
    ("el", "el"),
    ("el-CY", "el-CY"),
    ("en", "en"),
    ("en-001", "en-001"),
    ("en-150", "en-150"),
    ("en-AE", "en-AE"),
    ("en-AG", "en-AG"),
    ("en-AI", "en-AI"),
    ("en-AS", "en-AS"),
    ("en-AT", "en-AT"),
    ("en-AU", "en-AU"),
    ("en-BB", "en-BB"),
    ("en-BE", "en-BE"),
    ("en-BI", "en-BI"),
    ("en-BM", "en-BM"),
    ("en-BS", "en-BS"),
    ("en-BW", "en-BW"),
    ("en-BZ", "en-BZ"),
    ("en-CA", "en-CA"),
    ("en-CC", "en-CC"),
    ("en-CH", "en-CH"),
    ("en-CK", "en-CK"),
    ("en-CM", "en-CM"),
    ("en-CX", "en-CX"),
    ("en-CY", "en-CY"),
    ("en-DE", "en-DE"),
    ("en-DG", "en-DG"),
    ("en-DK", "en-DK"),
    ("en-DM", "en-DM"),
    ("en-ER", "en-ER"),
    ("en-FI", "en-FI"),
    ("en-FJ", "en-FJ"),
    ("en-FK", "en-FK"),
    ("en-FM", "en-FM"),
    ("en-GB", "en-GB"),
    ("en-GD", "en-GD"),
    ("en-GG", "en-GG"),
    ("en-GH", "en-GH"),
    ("en-GI", "en-GI"),
    ("en-GM", "en-GM"),
    ("en-GU", "en-GU"),
    ("en-GY", "en-GY"),
    ("en-HK", "en-HK"),
    ("en-IE", "en-IE"),
    ("en-IL", "en-IL"),
    ("en-IM", "en-IM"),
    ("en-IN", "en-IN"),
    ("en-IO", "en-IO"),
    ("en-JE", "en-JE"),
    ("en-JM", "en-JM"),
    ("en-KE", "en-KE"),
    ("en-KI", "en-KI"),
    ("en-KN", "en-KN"),
    ("en-KY", "en-KY"),
    ("en-LC", "en-LC"),
    ("en-LR", "en-LR"),
    ("en-LS", "en-LS"),
    ("en-MG", "en-MG"),
    ("en-MH", "en-MH"),
    ("en-MO", "en-MO"),
    ("en-MP", "en-MP"),
    ("en-MS", "en-MS"),
    ("en-MT", "en-MT"),
    ("en-MU", "en-MU"),
    ("en-MV", "en-MV"),
    ("en-MW", "en-MW"),
    ("en-MY", "en-MY"),
    ("en-NA", "en-NA"),
    ("en-NF", "en-NF"),
    ("en-NG", "en-NG"),
    ("en-NL", "en-NL"),
    ("en-NR", "en-NR"),
    ("en-NU", "en-NU"),
    ("en-NZ", "en-NZ"),
    ("en-PG", "en-PG"),
    ("en-PH", "en-PH"),
    ("en-PK", "en-PK"),
    ("en-PN", "en-PN"),
    ("en-PR", "en-PR"),
    ("en-PW", "en-PW"),
    ("en-RW", "en-RW"),
    ("en-SB", "en-SB"),
    ("en-SC", "en-SC"),
    ("en-SD", "en-SD"),
    ("en-SE", "en-SE"),
    ("en-SG", "en-SG"),
    ("en-SH", "en-SH"),
    ("en-SI", "en-SI"),
    ("en-SL", "en-SL"),
    ("en-SS", "en-SS"),
    ("en-SX", "en-SX"),
    ("en-SZ", "en-SZ"),
    ("en-TC", "en-TC"),
    ("en-TK", "en-TK"),
    ("en-TO", "en-TO"),
    ("en-TT", "en-TT"),
    ("en-TV", "en-TV"),
    ("en-TZ", "en-TZ"),
    ("en-UG", "en-UG"),
    ("en-UM", "en-UM"),
    ("en-VC", "en-VC"),
    ("en-VG", "en-VG"),
    ("en-VI", "en-VI"),
    ("en-VU", "en-VU"),
    ("en-WS", "en-WS"),
    ("en-ZA", "en-ZA"),
    ("en-ZM", "en-ZM"),
    ("en-ZW", "en-ZW"),
    ("eo", "eo"),
    ("es", "es"),
    ("es-419", "es-419"),
    ("es-AR", "es-AR"),
    ("es-BO", "es-BO"),
    ("es-BR", "es-BR"),
    ("es-BZ", "es-BZ"),
    ("es-CL", "es-CL"),
    ("es-CO", "es-CO"),
    ("es-CR", "es-CR"),
    ("es-CU", "es-CU"),
    ("es-DO", "es-DO"),
    ("es-EA", "es-EA"),
    ("es-EC", "es-EC"),
    ("es-GQ", "es-GQ"),
    ("es-GT", "es-GT"),
    ("es-HN", "es-HN"),
    ("es-IC", "es-IC"),
    ("es-MX", "es-MX"),
    ("es-NI", "es-NI"),
    ("es-PA", "es-PA"),
    ("es-PE", "es-PE"),
    ("es-PH", "es-PH"),
    ("es-PR", "es-PR"),
    ("es-PY", "es-PY"),
    ("es-SV", "es-SV"),
    ("es-US", "es-US"),
    ("es-UY", "es-UY"),
    ("es-VE", "es-VE"),
    ("et", "et"),
    ("eu", "eu"),
    ("ewo", "ewo"),
    ("fa", "fa"),
    ("fa-AF", "fa-AF"),
    ("ff", "ff"),
    ("ff-Adlm", "ff-Adlm"),
    ("ff-Adlm-BF", "ff-Adlm-BF"),
    ("ff-Adlm-CM", "ff-Adlm-CM"),
    ("ff-Adlm-GH", "ff-Adlm-GH"),
    ("ff-Adlm-GM", "ff-Adlm-GM"),
    ("ff-Adlm-GW", "ff-Adlm-GW"),
    ("ff-Adlm-LR", "ff-Adlm-LR"),
    ("ff-Adlm-MR", "ff-Adlm-MR"),
    ("ff-Adlm-NE", "ff-Adlm-NE"),
    ("ff-Adlm-NG", "ff-Adlm-NG"),
    ("ff-Adlm-SL", "ff-Adlm-SL"),
    ("ff-Adlm-SN", "ff-Adlm-SN"),
    ("ff-Latn", "ff"),
    ("ff-Latn-BF", "ff-BF"),
    ("ff-Latn-CM", "ff-CM"),
    ("ff-Latn-GH", "ff-GH"),
    ("ff-Latn-GM", "ff-GM"),
    ("ff-Latn-GN", "ff-GN"),
    ("ff-Latn-GW", "ff-GW"),
    ("ff-Latn-LR", "ff-LR"),
    ("ff-Latn-MR", "ff-MR"),
    ("ff-Latn-NE", "ff-NE"),
    ("ff-Latn-NG", "ff-NG"),
    ("ff-Latn-SL", "ff-SL"),
    ("fi", "fi"),
    ("fil", "fil"),
    ("fo", "fo"),
    ("fo-DK", "fo-DK"),
    ("fr", "fr"),
    ("fr-BE", "fr-BE"),
    ("fr-BF", "fr-BF"),
    ("fr-BI", "fr-BI"),
    ("fr-BJ", "fr-BJ"),
    ("fr-BL", "fr-BL"),
    ("fr-CA", "fr-CA"),
    ("fr-CD", "fr-CD"),
    ("fr-CF", "fr-CF"),
    ("fr-CG", "fr-CG"),
    ("fr-CH", "fr-CH"),
    ("fr-CI", "fr-CI"),
    ("fr-CM", "fr-CM"),
    ("fr-DJ", "fr-DJ"),
    ("fr-DZ", "fr-DZ"),
    ("fr-GA", "fr-GA"),
    ("fr-GF", "fr-GF"),
    ("fr-GN", "fr-GN"),
    ("fr-GP", "fr-GP"),
    ("fr-GQ", "fr-GQ"),
    ("fr-HT", "fr-HT"),
    ("fr-KM", "fr-KM"),
    ("fr-LU", "fr-LU"),
    ("fr-MA", "fr-MA"),
    ("fr-MC", "fr-MC"),
    ("fr-MF", "fr-MF"),
    ("fr-MG", "fr-MG"),
    ("fr-ML", "fr-ML"),
    ("fr-MQ", "fr-MQ"),
    ("fr-MR", "fr-MR"),
    ("fr-MU", "fr-MU"),
    ("fr-NC", "fr-NC"),
    ("fr-NE", "fr-NE"),
    ("fr-PF", "fr-PF"),
    ("fr-PM", "fr-PM"),
    ("fr-RE", "fr-RE"),
    ("fr-RW", "fr-RW"),
    ("fr-SC", "fr-SC"),
    ("fr-SN", "fr-SN"),
    ("fr-SY", "fr-SY"),
    ("fr-TD", "fr-TD"),
    ("fr-TG", "fr-TG"),
    ("fr-TN", "fr-TN"),
    ("fr-VU", "fr-VU"),
    ("fr-WF", "fr-WF"),
    ("fr-YT", "fr-YT"),
    ("frr", "frr"),
    ("fur", "fur"),
    ("fy", "fy"),
    ("ga", "ga"),
    ("ga-GB", "ga-GB"),
    ("gd", "gd"),
    ("gl", "gl"),
    ("gsw", "gsw"),
    ("gsw-FR", "gsw-FR"),
    ("gsw-LI", "gsw-LI"),
    ("gu", "gu"),
    ("guz", "guz"),
    ("gv", "gv"),
    ("ha", "ha"),
    ("ha-GH", "ha-GH"),
    ("ha-NE", "ha-NE"),
    ("haw", "haw"),
    ("he", "he"),
    ("hi", "hi"),
    ("hi-Latn", "hi-Latn"),
    ("hr", "hr"),
    ("hr-BA", "hr-BA"),
    ("hsb", "hsb"),
    ("hu", "hu"),
    ("hy", "hy"),
    ("ia", "ia"),
    ("id", "id"),
    ("ig", "ig"),
    ("ii", "ii"),
    ("is", "is"),
    ("it", "it"),
    ("it-CH", "it-CH"),
    ("it-SM", "it-SM"),
    ("it-VA", "it-VA"),
    ("ja", "ja"),
    ("jgo", "jgo"),
    ("jmc", "jmc"),
    ("jv", "jv"),
    ("ka", "ka"),
    ("kab", "kab"),
    ("kam", "kam"),
    ("kde", "kde"),
    ("kea", "kea"),
    ("kgp", "kgp"),
    ("khq", "khq"),
    ("ki", "ki"),
    ("kk", "kk"),
    ("kkj", "kkj"),
    ("kl", "kl"),
    ("kln", "kln"),
    ("km", "km"),
    ("kn", "kn"),
    ("ko", "ko"),
    ("ko-KP", "ko-KP"),
    ("kok", "kok"),
    ("ks", "ks"),
    ("ks-Arab", "ks"),
    ("ks-Deva", "ks-Deva"),
    ("ksb", "ksb"),
    ("ksf", "ksf"),
    ("ksh", "ksh"),
    ("ku", "ku"),
    ("kw", "kw"),
    ("ky", "ky"),
    ("lag", "lag"),
    ("lb", "lb"),
    ("lg", "lg"),
    ("lkt", "lkt"),
    ("ln", "ln"),
    ("ln-AO", "ln-AO"),
    ("ln-CF", "ln-CF"),
    ("ln-CG", "ln-CG"),
    ("lo", "lo"),
    ("lrc", "lrc"),
    ("lrc-IQ", "lrc-IQ"),
    ("lt", "lt"),
    ("lu", "lu"),
    ("luo", "luo"),
    ("luy", "luy"),
    ("lv", "lv"),
    ("mai", "mai"),
    ("mas", "mas"),
    ("mas-TZ", "mas-TZ"),
    ("mdf", "mdf"),
    ("mer", "mer"),
    ("mfe", "mfe"),
    ("mg", "mg"),
    ("mgh", "mgh"),
    ("mgo", "mgo"),
    ("mi", "mi"),
    ("mk", "mk"),
    ("ml", "ml"),
    ("mn", "mn"),
    ("mni", "mni"),
    ("mni-Beng", "mni"),
    ("mr", "mr"),
    ("ms", "ms"),
    ("ms-BN", "ms-BN"),
    ("ms-ID", "ms-ID"),
    ("ms-SG", "ms-SG"),
    ("mt", "mt"),
    ("mua", "mua"),
    ("my", "my"),
    ("mzn", "mzn"),
    ("naq", "naq"),
    ("nb", "nb"),
    ("nb-SJ", "nb-SJ"),
    ("nd", "nd"),
    ("nds", "nds"),
    ("nds-NL", "nds-NL"),
    ("ne", "ne"),
    ("ne-IN", "ne-IN"),
    ("nl", "nl"),
    ("nl-AW", "nl-AW"),
    ("nl-BE", "nl-BE"),
    ("nl-BQ", "nl-BQ"),
    ("nl-CW", "nl-CW"),
    ("nl-SR", "nl-SR"),
    ("nl-SX", "nl-SX"),
    ("nmg", "nmg"),
    ("nn", "nn"),
    ("nnh", "nnh"),
    ("no", "no"),
    ("nus", "nus"),
    ("nyn", "nyn"),
    ("oc", "oc"),
    ("oc-ES", "oc-ES"),
    ("om", "om"),
    ("om-KE", "om-KE"),
    ("or", "or"),
    ("os", "os"),
    ("os-RU", "os-RU"),
    ("pa", "pa"),
    ("pa-Arab", "pa-PK"),
    ("pa-Guru", "pa"),
    ("pcm", "pcm"),
    ("pis", "pis"),
    ("pl", "pl"),
    ("ps", "ps"),
    ("ps-PK", "ps-PK"),
    ("pt", "pt"),
    ("pt-AO", "pt-AO"),
    ("pt-CH", "pt-CH"),
    ("pt-CV", "pt-CV"),
    ("pt-GQ", "pt-GQ"),
    ("pt-GW", "pt-GW"),
    ("pt-LU", "pt-LU"),
    ("pt-MO", "pt-MO"),
    ("pt-MZ", "pt-MZ"),
    ("pt-PT", "pt-PT"),
    ("pt-ST", "pt-ST"),
    ("pt-TL", "pt-TL"),
    ("qu", "qu"),
    ("qu-BO", "qu-BO"),
    ("qu-EC", "qu-EC"),
    ("raj", "raj"),
    ("rm", "rm"),
    ("rn", "rn"),
    ("ro", "ro"),
    ("ro-MD", "ro-MD"),
    ("rof", "rof"),
    ("ru", "ru"),
    ("ru-BY", "ru-BY"),
    ("ru-KG", "ru-KG"),
    ("ru-KZ", "ru-KZ"),
    ("ru-MD", "ru-MD"),
    ("ru-UA", "ru-UA"),
    ("rw", "rw"),
    ("rwk", "rwk"),
    ("sa", "sa"),
    ("sah", "sah"),
    ("saq", "saq"),
    ("sat", "sat"),
    ("sat-Olck", "sat"),
    ("sbp", "sbp"),
    ("sc", "sc"),
    ("sd", "sd"),
    ("sd-Arab", "sd"),
    ("sd-Deva", "sd-IN"),
    ("se", "se"),
    ("se-FI", "se-FI"),
    ("se-SE", "se-SE"),
    ("seh", "seh"),
    ("ses", "ses"),
    ("sg", "sg"),
    ("shi", "shi"),
    ("shi-Latn", "shi-Latn"),
    ("shi-Tfng", "shi"),
    ("si", "si"),
    ("sk", "sk"),
    ("sl", "sl"),
    ("smn", "smn"),
    ("sms", "sms"),
    ("sn", "sn"),
    ("so", "so"),
    ("so-DJ", "so-DJ"),
    ("so-ET", "so-ET"),
    ("so-KE", "so-KE"),
    ("sq", "sq"),
    ("sq-MK", "sq-MK"),
    ("sq-XK", "sq-XK"),
    ("sr", "sr"),
    ("sr-Cyrl", "sr"),
    ("sr-Cyrl-BA", "sr-BA"),
    ("sr-Cyrl-ME", "sr-Cyrl-ME"),
    ("sr-Cyrl-XK", "sr-XK"),
    ("sr-Latn", "sr-Latn"),
    ("sr-Latn-BA", "sr-Latn-BA"),
    ("sr-Latn-ME", "sr-ME"),
    ("sr-Latn-XK", "sr-Latn-XK"),
    ("su", "su"),
    ("su-Latn", "su"),
    ("sv", "sv"),
    ("sv-AX", "sv-AX"),
    ("sv-FI", "sv-FI"),
    ("sw", "sw"),
    ("sw-CD", "sw-CD"),
    ("sw-KE", "sw-KE"),
    ("sw-UG", "sw-UG"),
    ("ta", "ta"),
    ("ta-LK", "ta-LK"),
    ("ta-MY", "ta-MY"),
    ("ta-SG", "ta-SG"),
    ("te", "te"),
    ("teo", "teo"),
    ("teo-KE", "teo-KE"),
    ("tg", "tg"),
    ("th", "th"),
    ("ti", "ti"),
    ("ti-ER", "ti-ER"),
    ("tk", "tk"),
    ("to", "to"),
    ("tok", "tok"),
    ("tr", "tr"),
    ("tr-CY", "tr-CY"),
    ("tt", "tt"),
    ("twq", "twq"),
    ("tzm", "tzm"),
    ("ug", "ug"),
    ("uk", "uk"),
    ("und", "en"),
    ("ur", "ur"),
    ("ur-IN", "ur-IN"),
    ("uz", "uz"),
    ("uz-Arab", "uz-AF"),
    ("uz-Cyrl", "uz-Cyrl"),
    ("uz-Latn", "uz"),
    ("vai", "vai"),
    ("vai-Latn", "vai-Latn"),
    ("vai-Vaii", "vai"),
    ("vi", "vi"),
    ("vun", "vun"),
    ("wae", "wae"),
    ("wo", "wo"),
    ("xh", "xh"),
    ("xog", "xog"),
    ("yav", "yav"),
    ("yi", "yi"),
    ("yo", "yo"),
    ("yo-BJ", "yo-BJ"),
    ("yrl", "yrl"),
    ("yrl-CO", "yrl-CO"),
    ("yrl-VE", "yrl-VE"),
    ("yue", "yue"),
    ("yue-Hans", "yue-CN"),
    ("yue-Hant", "yue"),
    ("zgh", "zgh"),
    ("zh", "zh"),
    ("zh-CN", "zh"),
    ("zh-Hans", "zh"),
    ("zh-Hans-CN", "zh"),
    ("zh-Hans-HK", "zh-Hans-HK"),
    ("zh-Hans-MO", "zh-Hans-MO"),
    ("zh-Hans-SG", "zh-SG"),
    ("zh-Hant", "zh-TW"),
    ("zh-Hant-HK", "zh-HK"),
    ("zh-Hant-MO", "zh-MO"),
    ("zh-Hant-TW", "zh-TW"),
    ("zh-Latn-CN", "zh-pinyin"),
    ("zh-Latn-CN-pinyin", "zh-pinyin"),
    ("zu", "zu"),
  ];

  let raw_doc = r#"
Generates a const perfect hash map for minimized language tag canonicalization.

## Example

```
let map = lang_id::maps::min::map();

let sg = map.get("zh-Hans-SG");
assert_eq!(sg, Some(&"zh-SG"));
```
  "#;
  builder::MapBuilder::<()> {
    mod_name,
    str_kv,
    map_type: MapType::Ordered,
    raw_doc,
    ..Default::default()
  }
  .build()
}
