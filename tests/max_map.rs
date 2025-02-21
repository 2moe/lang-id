mod builder;
use std::io;

#[test]
#[ignore]
fn new_map() -> io::Result<()> {
  let mod_name = "max";
  let header = "use super::TinyID;\n";
  // let tris = &[];
  // [(&str, [&str; 3])]\
  let kv = &[
    ("af", ("af", "Latn", "ZA")),
    ("af-NA", ("af", "Latn", "NA")),
    ("agq", ("agq", "Latn", "CM")),
    ("ak", ("ak", "Latn", "GH")),
    ("am", ("am", "Ethi", "ET")),
    ("ann", ("ann", "Latn", "NG")),
    ("ar", ("ar", "Arab", "EG")),
    ("ar-AE", ("ar", "Arab", "AE")),
    ("ar-BH", ("ar", "Arab", "BH")),
    ("ar-DJ", ("ar", "Arab", "DJ")),
    ("ar-DZ", ("ar", "Arab", "DZ")),
    ("ar-EG", ("ar", "Arab", "EG")),
    ("ar-EH", ("ar", "Arab", "EH")),
    ("ar-ER", ("ar", "Arab", "ER")),
    ("ar-IL", ("ar", "Arab", "IL")),
    ("ar-IQ", ("ar", "Arab", "IQ")),
    ("ar-JO", ("ar", "Arab", "JO")),
    ("ar-KM", ("ar", "Arab", "KM")),
    ("ar-KW", ("ar", "Arab", "KW")),
    ("ar-LB", ("ar", "Arab", "LB")),
    ("ar-LY", ("ar", "Arab", "LY")),
    ("ar-MA", ("ar", "Arab", "MA")),
    ("ar-MR", ("ar", "Arab", "MR")),
    ("ar-OM", ("ar", "Arab", "OM")),
    ("ar-PS", ("ar", "Arab", "PS")),
    ("ar-QA", ("ar", "Arab", "QA")),
    ("ar-SA", ("ar", "Arab", "SA")),
    ("ar-SD", ("ar", "Arab", "SD")),
    ("ar-SO", ("ar", "Arab", "SO")),
    ("ar-SS", ("ar", "Arab", "SS")),
    ("ar-SY", ("ar", "Arab", "SY")),
    ("ar-TD", ("ar", "Arab", "TD")),
    ("ar-TN", ("ar", "Arab", "TN")),
    ("ar-YE", ("ar", "Arab", "YE")),
    ("as", ("as", "Beng", "IN")),
    ("asa", ("asa", "Latn", "TZ")),
    ("ast", ("ast", "Latn", "ES")),
    ("az", ("az", "Latn", "AZ")),
    ("az-Cyrl", ("az", "Cyrl", "AZ")),
    ("az-Latn", ("az", "Latn", "AZ")),
    ("bas", ("bas", "Latn", "CM")),
    ("be", ("be", "Cyrl", "BY")),
    ("bem", ("bem", "Latn", "ZM")),
    ("bez", ("bez", "Latn", "TZ")),
    ("bg", ("bg", "Cyrl", "BG")),
    ("bgc", ("bgc", "Deva", "IN")),
    ("bho", ("bho", "Deva", "IN")),
    ("bm", ("bm", "Latn", "ML")),
    ("bn", ("bn", "Beng", "BD")),
    ("bn-IN", ("bn", "Beng", "IN")),
    ("bo", ("bo", "Tibt", "CN")),
    ("bo-IN", ("bo", "Tibt", "IN")),
    ("br", ("br", "Latn", "FR")),
    ("brx", ("brx", "Deva", "IN")),
    ("bs", ("bs", "Latn", "BA")),
    ("bs-Cyrl", ("bs", "Cyrl", "BA")),
    ("bs-Latn", ("bs", "Latn", "BA")),
    ("ca", ("ca", "Latn", "ES")),
    ("ca-AD", ("ca", "Latn", "AD")),
    ("ca-FR", ("ca", "Latn", "FR")),
    ("ca-IT", ("ca", "Latn", "IT")),
    ("ccp", ("ccp", "Cakm", "BD")),
    ("ccp-IN", ("ccp", "Cakm", "IN")),
    ("ce", ("ce", "Cyrl", "RU")),
    ("ceb", ("ceb", "Latn", "PH")),
    ("cgg", ("cgg", "Latn", "UG")),
    ("chr", ("chr", "Cher", "US")),
    ("ckb", ("ckb", "Arab", "IQ")),
    ("ckb-IR", ("ckb", "Arab", "IR")),
    ("cs", ("cs", "Latn", "CZ")),
    ("cv", ("cv", "Cyrl", "RU")),
    ("cy", ("cy", "Latn", "GB")),
    ("da", ("da", "Latn", "DK")),
    ("da-GL", ("da", "Latn", "GL")),
    ("dav", ("dav", "Latn", "KE")),
    ("de", ("de", "Latn", "DE")),
    ("de-AT", ("de", "Latn", "AT")),
    ("de-BE", ("de", "Latn", "BE")),
    ("de-CH", ("de", "Latn", "CH")),
    ("de-IT", ("de", "Latn", "IT")),
    ("de-LI", ("de", "Latn", "LI")),
    ("de-LU", ("de", "Latn", "LU")),
    ("dje", ("dje", "Latn", "NE")),
    ("doi", ("doi", "Deva", "IN")),
    ("dsb", ("dsb", "Latn", "DE")),
    ("dua", ("dua", "Latn", "CM")),
    ("dyo", ("dyo", "Latn", "SN")),
    ("dz", ("dz", "Tibt", "BT")),
    ("ebu", ("ebu", "Latn", "KE")),
    ("ee", ("ee", "Latn", "GH")),
    ("ee-TG", ("ee", "Latn", "TG")),
    ("el", ("el", "Grek", "GR")),
    ("el-CY", ("el", "Grek", "CY")),
    ("en", ("en", "Latn", "US")),
    ("en-001", ("en", "Latn", "001")),
    ("en-150", ("en", "Latn", "150")),
    ("en-AE", ("en", "Latn", "AE")),
    ("en-AG", ("en", "Latn", "AG")),
    ("en-AI", ("en", "Latn", "AI")),
    ("en-AS", ("en", "Latn", "AS")),
    ("en-AT", ("en", "Latn", "AT")),
    ("en-AU", ("en", "Latn", "AU")),
    ("en-BB", ("en", "Latn", "BB")),
    ("en-BE", ("en", "Latn", "BE")),
    ("en-BI", ("en", "Latn", "BI")),
    ("en-BM", ("en", "Latn", "BM")),
    ("en-BS", ("en", "Latn", "BS")),
    ("en-BW", ("en", "Latn", "BW")),
    ("en-BZ", ("en", "Latn", "BZ")),
    ("en-CA", ("en", "Latn", "CA")),
    ("en-CC", ("en", "Latn", "CC")),
    ("en-CH", ("en", "Latn", "CH")),
    ("en-CK", ("en", "Latn", "CK")),
    ("en-CM", ("en", "Latn", "CM")),
    ("en-CX", ("en", "Latn", "CX")),
    ("en-CY", ("en", "Latn", "CY")),
    ("en-DE", ("en", "Latn", "DE")),
    ("en-DG", ("en", "Latn", "DG")),
    ("en-DK", ("en", "Latn", "DK")),
    ("en-DM", ("en", "Latn", "DM")),
    ("en-ER", ("en", "Latn", "ER")),
    ("en-FI", ("en", "Latn", "FI")),
    ("en-FJ", ("en", "Latn", "FJ")),
    ("en-FK", ("en", "Latn", "FK")),
    ("en-FM", ("en", "Latn", "FM")),
    ("en-GB", ("en", "Latn", "GB")),
    ("en-GD", ("en", "Latn", "GD")),
    ("en-GG", ("en", "Latn", "GG")),
    ("en-GH", ("en", "Latn", "GH")),
    ("en-GI", ("en", "Latn", "GI")),
    ("en-GM", ("en", "Latn", "GM")),
    ("en-GU", ("en", "Latn", "GU")),
    ("en-GY", ("en", "Latn", "GY")),
    ("en-HK", ("en", "Latn", "HK")),
    ("en-IE", ("en", "Latn", "IE")),
    ("en-IL", ("en", "Latn", "IL")),
    ("en-IM", ("en", "Latn", "IM")),
    ("en-IN", ("en", "Latn", "IN")),
    ("en-IO", ("en", "Latn", "IO")),
    ("en-JE", ("en", "Latn", "JE")),
    ("en-JM", ("en", "Latn", "JM")),
    ("en-KE", ("en", "Latn", "KE")),
    ("en-KI", ("en", "Latn", "KI")),
    ("en-KN", ("en", "Latn", "KN")),
    ("en-KY", ("en", "Latn", "KY")),
    ("en-LC", ("en", "Latn", "LC")),
    ("en-LR", ("en", "Latn", "LR")),
    ("en-LS", ("en", "Latn", "LS")),
    ("en-MG", ("en", "Latn", "MG")),
    ("en-MH", ("en", "Latn", "MH")),
    ("en-MO", ("en", "Latn", "MO")),
    ("en-MP", ("en", "Latn", "MP")),
    ("en-MS", ("en", "Latn", "MS")),
    ("en-MT", ("en", "Latn", "MT")),
    ("en-MU", ("en", "Latn", "MU")),
    ("en-MV", ("en", "Latn", "MV")),
    ("en-MW", ("en", "Latn", "MW")),
    ("en-MY", ("en", "Latn", "MY")),
    ("en-NA", ("en", "Latn", "NA")),
    ("en-NF", ("en", "Latn", "NF")),
    ("en-NG", ("en", "Latn", "NG")),
    ("en-NL", ("en", "Latn", "NL")),
    ("en-NR", ("en", "Latn", "NR")),
    ("en-NU", ("en", "Latn", "NU")),
    ("en-NZ", ("en", "Latn", "NZ")),
    ("en-PG", ("en", "Latn", "PG")),
    ("en-PH", ("en", "Latn", "PH")),
    ("en-PK", ("en", "Latn", "PK")),
    ("en-PN", ("en", "Latn", "PN")),
    ("en-PR", ("en", "Latn", "PR")),
    ("en-PW", ("en", "Latn", "PW")),
    ("en-RW", ("en", "Latn", "RW")),
    ("en-SB", ("en", "Latn", "SB")),
    ("en-SC", ("en", "Latn", "SC")),
    ("en-SD", ("en", "Latn", "SD")),
    ("en-SE", ("en", "Latn", "SE")),
    ("en-SG", ("en", "Latn", "SG")),
    ("en-SH", ("en", "Latn", "SH")),
    ("en-SI", ("en", "Latn", "SI")),
    ("en-SL", ("en", "Latn", "SL")),
    ("en-SS", ("en", "Latn", "SS")),
    ("en-SX", ("en", "Latn", "SX")),
    ("en-SZ", ("en", "Latn", "SZ")),
    ("en-TC", ("en", "Latn", "TC")),
    ("en-TK", ("en", "Latn", "TK")),
    ("en-TO", ("en", "Latn", "TO")),
    ("en-TT", ("en", "Latn", "TT")),
    ("en-TV", ("en", "Latn", "TV")),
    ("en-TZ", ("en", "Latn", "TZ")),
    ("en-UG", ("en", "Latn", "UG")),
    ("en-UM", ("en", "Latn", "UM")),
    ("en-VC", ("en", "Latn", "VC")),
    ("en-VG", ("en", "Latn", "VG")),
    ("en-VI", ("en", "Latn", "VI")),
    ("en-VU", ("en", "Latn", "VU")),
    ("en-WS", ("en", "Latn", "WS")),
    ("en-ZA", ("en", "Latn", "ZA")),
    ("en-ZM", ("en", "Latn", "ZM")),
    ("en-ZW", ("en", "Latn", "ZW")),
    ("eo", ("eo", "Latn", "001")),
    ("es", ("es", "Latn", "ES")),
    ("es-419", ("es", "Latn", "419")),
    ("es-AR", ("es", "Latn", "AR")),
    ("es-BO", ("es", "Latn", "BO")),
    ("es-BR", ("es", "Latn", "BR")),
    ("es-BZ", ("es", "Latn", "BZ")),
    ("es-CL", ("es", "Latn", "CL")),
    ("es-CO", ("es", "Latn", "CO")),
    ("es-CR", ("es", "Latn", "CR")),
    ("es-CU", ("es", "Latn", "CU")),
    ("es-DO", ("es", "Latn", "DO")),
    ("es-EA", ("es", "Latn", "EA")),
    ("es-EC", ("es", "Latn", "EC")),
    ("es-GQ", ("es", "Latn", "GQ")),
    ("es-GT", ("es", "Latn", "GT")),
    ("es-HN", ("es", "Latn", "HN")),
    ("es-IC", ("es", "Latn", "IC")),
    ("es-MX", ("es", "Latn", "MX")),
    ("es-NI", ("es", "Latn", "NI")),
    ("es-PA", ("es", "Latn", "PA")),
    ("es-PE", ("es", "Latn", "PE")),
    ("es-PH", ("es", "Latn", "PH")),
    ("es-PR", ("es", "Latn", "PR")),
    ("es-PY", ("es", "Latn", "PY")),
    ("es-SV", ("es", "Latn", "SV")),
    ("es-US", ("es", "Latn", "US")),
    ("es-UY", ("es", "Latn", "UY")),
    ("es-VE", ("es", "Latn", "VE")),
    ("et", ("et", "Latn", "EE")),
    ("eu", ("eu", "Latn", "ES")),
    ("ewo", ("ewo", "Latn", "CM")),
    ("fa", ("fa", "Arab", "IR")),
    ("fa-AF", ("fa", "Arab", "AF")),
    ("ff", ("ff", "Latn", "SN")),
    ("ff-Adlm", ("ff", "Adlm", "GN")),
    ("ff-Adlm-BF", ("ff", "Adlm", "BF")),
    ("ff-Adlm-CM", ("ff", "Adlm", "CM")),
    ("ff-Adlm-GH", ("ff", "Adlm", "GH")),
    ("ff-Adlm-GM", ("ff", "Adlm", "GM")),
    ("ff-Adlm-GW", ("ff", "Adlm", "GW")),
    ("ff-Adlm-LR", ("ff", "Adlm", "LR")),
    ("ff-Adlm-MR", ("ff", "Adlm", "MR")),
    ("ff-Adlm-NE", ("ff", "Adlm", "NE")),
    ("ff-Adlm-NG", ("ff", "Adlm", "NG")),
    ("ff-Adlm-SL", ("ff", "Adlm", "SL")),
    ("ff-Adlm-SN", ("ff", "Adlm", "SN")),
    ("ff-Latn", ("ff", "Latn", "SN")),
    ("ff-Latn-BF", ("ff", "Latn", "BF")),
    ("ff-Latn-CM", ("ff", "Latn", "CM")),
    ("ff-Latn-GH", ("ff", "Latn", "GH")),
    ("ff-Latn-GM", ("ff", "Latn", "GM")),
    ("ff-Latn-GN", ("ff", "Latn", "GN")),
    ("ff-Latn-GW", ("ff", "Latn", "GW")),
    ("ff-Latn-LR", ("ff", "Latn", "LR")),
    ("ff-Latn-MR", ("ff", "Latn", "MR")),
    ("ff-Latn-NE", ("ff", "Latn", "NE")),
    ("ff-Latn-NG", ("ff", "Latn", "NG")),
    ("ff-Latn-SL", ("ff", "Latn", "SL")),
    ("fi", ("fi", "Latn", "FI")),
    ("fil", ("fil", "Latn", "PH")),
    ("fo", ("fo", "Latn", "FO")),
    ("fo-DK", ("fo", "Latn", "DK")),
    ("fr", ("fr", "Latn", "FR")),
    ("fr-BE", ("fr", "Latn", "BE")),
    ("fr-BF", ("fr", "Latn", "BF")),
    ("fr-BI", ("fr", "Latn", "BI")),
    ("fr-BJ", ("fr", "Latn", "BJ")),
    ("fr-BL", ("fr", "Latn", "BL")),
    ("fr-CA", ("fr", "Latn", "CA")),
    ("fr-CD", ("fr", "Latn", "CD")),
    ("fr-CF", ("fr", "Latn", "CF")),
    ("fr-CG", ("fr", "Latn", "CG")),
    ("fr-CH", ("fr", "Latn", "CH")),
    ("fr-CI", ("fr", "Latn", "CI")),
    ("fr-CM", ("fr", "Latn", "CM")),
    ("fr-DJ", ("fr", "Latn", "DJ")),
    ("fr-DZ", ("fr", "Latn", "DZ")),
    ("fr-GA", ("fr", "Latn", "GA")),
    ("fr-GF", ("fr", "Latn", "GF")),
    ("fr-GN", ("fr", "Latn", "GN")),
    ("fr-GP", ("fr", "Latn", "GP")),
    ("fr-GQ", ("fr", "Latn", "GQ")),
    ("fr-HT", ("fr", "Latn", "HT")),
    ("fr-KM", ("fr", "Latn", "KM")),
    ("fr-LU", ("fr", "Latn", "LU")),
    ("fr-MA", ("fr", "Latn", "MA")),
    ("fr-MC", ("fr", "Latn", "MC")),
    ("fr-MF", ("fr", "Latn", "MF")),
    ("fr-MG", ("fr", "Latn", "MG")),
    ("fr-ML", ("fr", "Latn", "ML")),
    ("fr-MQ", ("fr", "Latn", "MQ")),
    ("fr-MR", ("fr", "Latn", "MR")),
    ("fr-MU", ("fr", "Latn", "MU")),
    ("fr-NC", ("fr", "Latn", "NC")),
    ("fr-NE", ("fr", "Latn", "NE")),
    ("fr-PF", ("fr", "Latn", "PF")),
    ("fr-PM", ("fr", "Latn", "PM")),
    ("fr-RE", ("fr", "Latn", "RE")),
    ("fr-RW", ("fr", "Latn", "RW")),
    ("fr-SC", ("fr", "Latn", "SC")),
    ("fr-SN", ("fr", "Latn", "SN")),
    ("fr-SY", ("fr", "Latn", "SY")),
    ("fr-TD", ("fr", "Latn", "TD")),
    ("fr-TG", ("fr", "Latn", "TG")),
    ("fr-TN", ("fr", "Latn", "TN")),
    ("fr-VU", ("fr", "Latn", "VU")),
    ("fr-WF", ("fr", "Latn", "WF")),
    ("fr-YT", ("fr", "Latn", "YT")),
    ("frr", ("frr", "Latn", "DE")),
    ("fur", ("fur", "Latn", "IT")),
    ("fy", ("fy", "Latn", "NL")),
    ("ga", ("ga", "Latn", "IE")),
    ("ga-GB", ("ga", "Latn", "GB")),
    ("gd", ("gd", "Latn", "GB")),
    ("gl", ("gl", "Latn", "ES")),
    ("gsw", ("gsw", "Latn", "CH")),
    ("gsw-FR", ("gsw", "Latn", "FR")),
    ("gsw-LI", ("gsw", "Latn", "LI")),
    ("gu", ("gu", "Gujr", "IN")),
    ("guz", ("guz", "Latn", "KE")),
    ("gv", ("gv", "Latn", "IM")),
    ("ha", ("ha", "Latn", "NG")),
    ("ha-GH", ("ha", "Latn", "GH")),
    ("ha-NE", ("ha", "Latn", "NE")),
    ("haw", ("haw", "Latn", "US")),
    ("he", ("he", "Hebr", "IL")),
    ("hi", ("hi", "Deva", "IN")),
    ("hi-Latn", ("hi", "Latn", "IN")),
    ("hr", ("hr", "Latn", "HR")),
    ("hr-BA", ("hr", "Latn", "BA")),
    ("hsb", ("hsb", "Latn", "DE")),
    ("hu", ("hu", "Latn", "HU")),
    ("hy", ("hy", "Armn", "AM")),
    ("ia", ("ia", "Latn", "001")),
    ("id", ("id", "Latn", "ID")),
    ("ig", ("ig", "Latn", "NG")),
    ("ii", ("ii", "Yiii", "CN")),
    ("is", ("is", "Latn", "IS")),
    ("it", ("it", "Latn", "IT")),
    ("it-CH", ("it", "Latn", "CH")),
    ("it-SM", ("it", "Latn", "SM")),
    ("it-VA", ("it", "Latn", "VA")),
    ("ja", ("ja", "Jpan", "JP")),
    ("jgo", ("jgo", "Latn", "CM")),
    ("jmc", ("jmc", "Latn", "TZ")),
    ("jv", ("jv", "Latn", "ID")),
    ("ka", ("ka", "Geor", "GE")),
    ("kab", ("kab", "Latn", "DZ")),
    ("kam", ("kam", "Latn", "KE")),
    ("kde", ("kde", "Latn", "TZ")),
    ("kea", ("kea", "Latn", "CV")),
    ("kgp", ("kgp", "Latn", "BR")),
    ("khq", ("khq", "Latn", "ML")),
    ("ki", ("ki", "Latn", "KE")),
    ("kk", ("kk", "Cyrl", "KZ")),
    ("kkj", ("kkj", "Latn", "CM")),
    ("kl", ("kl", "Latn", "GL")),
    ("kln", ("kln", "Latn", "KE")),
    ("km", ("km", "Khmr", "KH")),
    ("kn", ("kn", "Knda", "IN")),
    ("ko", ("ko", "Kore", "KR")),
    ("ko-KP", ("ko", "Kore", "KP")),
    ("kok", ("kok", "Deva", "IN")),
    ("ks", ("ks", "Arab", "IN")),
    ("ks-Arab", ("ks", "Arab", "IN")),
    ("ks-Deva", ("ks", "Deva", "IN")),
    ("ksb", ("ksb", "Latn", "TZ")),
    ("ksf", ("ksf", "Latn", "CM")),
    ("ksh", ("ksh", "Latn", "DE")),
    ("ku", ("ku", "Latn", "TR")),
    ("kw", ("kw", "Latn", "GB")),
    ("ky", ("ky", "Cyrl", "KG")),
    ("lag", ("lag", "Latn", "TZ")),
    ("lb", ("lb", "Latn", "LU")),
    ("lg", ("lg", "Latn", "UG")),
    ("lkt", ("lkt", "Latn", "US")),
    ("ln", ("ln", "Latn", "CD")),
    ("ln-AO", ("ln", "Latn", "AO")),
    ("ln-CF", ("ln", "Latn", "CF")),
    ("ln-CG", ("ln", "Latn", "CG")),
    ("lo", ("lo", "Laoo", "LA")),
    ("lrc", ("lrc", "Arab", "IR")),
    ("lrc-IQ", ("lrc", "Arab", "IQ")),
    ("lt", ("lt", "Latn", "LT")),
    ("lu", ("lu", "Latn", "CD")),
    ("luo", ("luo", "Latn", "KE")),
    ("luy", ("luy", "Latn", "KE")),
    ("lv", ("lv", "Latn", "LV")),
    ("mai", ("mai", "Deva", "IN")),
    ("mas", ("mas", "Latn", "KE")),
    ("mas-TZ", ("mas", "Latn", "TZ")),
    ("mdf", ("mdf", "Cyrl", "RU")),
    ("mer", ("mer", "Latn", "KE")),
    ("mfe", ("mfe", "Latn", "MU")),
    ("mg", ("mg", "Latn", "MG")),
    ("mgh", ("mgh", "Latn", "MZ")),
    ("mgo", ("mgo", "Latn", "CM")),
    ("mi", ("mi", "Latn", "NZ")),
    ("mk", ("mk", "Cyrl", "MK")),
    ("ml", ("ml", "Mlym", "IN")),
    ("mn", ("mn", "Cyrl", "MN")),
    ("mni", ("mni", "Beng", "IN")),
    ("mni-Beng", ("mni", "Beng", "IN")),
    ("mr", ("mr", "Deva", "IN")),
    ("ms", ("ms", "Latn", "MY")),
    ("ms-BN", ("ms", "Latn", "BN")),
    ("ms-ID", ("ms", "Latn", "ID")),
    ("ms-SG", ("ms", "Latn", "SG")),
    ("mt", ("mt", "Latn", "MT")),
    ("mua", ("mua", "Latn", "CM")),
    ("my", ("my", "Mymr", "MM")),
    ("mzn", ("mzn", "Arab", "IR")),
    ("naq", ("naq", "Latn", "NA")),
    ("nb", ("nb", "Latn", "NO")),
    ("nb-SJ", ("nb", "Latn", "SJ")),
    ("nd", ("nd", "Latn", "ZW")),
    ("nds", ("nds", "Latn", "DE")),
    ("nds-NL", ("nds", "Latn", "NL")),
    ("ne", ("ne", "Deva", "NP")),
    ("ne-IN", ("ne", "Deva", "IN")),
    ("nl", ("nl", "Latn", "NL")),
    ("nl-AW", ("nl", "Latn", "AW")),
    ("nl-BE", ("nl", "Latn", "BE")),
    ("nl-BQ", ("nl", "Latn", "BQ")),
    ("nl-CW", ("nl", "Latn", "CW")),
    ("nl-SR", ("nl", "Latn", "SR")),
    ("nl-SX", ("nl", "Latn", "SX")),
    ("nmg", ("nmg", "Latn", "CM")),
    ("nn", ("nn", "Latn", "NO")),
    ("nnh", ("nnh", "Latn", "CM")),
    ("no", ("no", "Latn", "NO")),
    ("nus", ("nus", "Latn", "SS")),
    ("nyn", ("nyn", "Latn", "UG")),
    ("oc", ("oc", "Latn", "FR")),
    ("oc-ES", ("oc", "Latn", "ES")),
    ("om", ("om", "Latn", "ET")),
    ("om-KE", ("om", "Latn", "KE")),
    ("or", ("or", "Orya", "IN")),
    ("os", ("os", "Cyrl", "GE")),
    ("os-RU", ("os", "Cyrl", "RU")),
    ("pa", ("pa", "Guru", "IN")),
    ("pa-Arab", ("pa", "Arab", "PK")),
    ("pa-Guru", ("pa", "Guru", "IN")),
    ("pcm", ("pcm", "Latn", "NG")),
    ("pis", ("pis", "Latn", "SB")),
    ("pl", ("pl", "Latn", "PL")),
    ("ps", ("ps", "Arab", "AF")),
    ("ps-PK", ("ps", "Arab", "PK")),
    ("pt", ("pt", "Latn", "BR")),
    ("pt-AO", ("pt", "Latn", "AO")),
    ("pt-CH", ("pt", "Latn", "CH")),
    ("pt-CV", ("pt", "Latn", "CV")),
    ("pt-GQ", ("pt", "Latn", "GQ")),
    ("pt-GW", ("pt", "Latn", "GW")),
    ("pt-LU", ("pt", "Latn", "LU")),
    ("pt-MO", ("pt", "Latn", "MO")),
    ("pt-MZ", ("pt", "Latn", "MZ")),
    ("pt-PT", ("pt", "Latn", "PT")),
    ("pt-ST", ("pt", "Latn", "ST")),
    ("pt-TL", ("pt", "Latn", "TL")),
    ("qu", ("qu", "Latn", "PE")),
    ("qu-BO", ("qu", "Latn", "BO")),
    ("qu-EC", ("qu", "Latn", "EC")),
    ("raj", ("raj", "Deva", "IN")),
    ("rm", ("rm", "Latn", "CH")),
    ("rn", ("rn", "Latn", "BI")),
    ("ro", ("ro", "Latn", "RO")),
    ("ro-MD", ("ro", "Latn", "MD")),
    ("rof", ("rof", "Latn", "TZ")),
    ("ru", ("ru", "Cyrl", "RU")),
    ("ru-BY", ("ru", "Cyrl", "BY")),
    ("ru-KG", ("ru", "Cyrl", "KG")),
    ("ru-KZ", ("ru", "Cyrl", "KZ")),
    ("ru-MD", ("ru", "Cyrl", "MD")),
    ("ru-UA", ("ru", "Cyrl", "UA")),
    ("rw", ("rw", "Latn", "RW")),
    ("rwk", ("rwk", "Latn", "TZ")),
    ("sa", ("sa", "Deva", "IN")),
    ("sah", ("sah", "Cyrl", "RU")),
    ("saq", ("saq", "Latn", "KE")),
    ("sat", ("sat", "Olck", "IN")),
    ("sat-Olck", ("sat", "Olck", "IN")),
    ("sbp", ("sbp", "Latn", "TZ")),
    ("sc", ("sc", "Latn", "IT")),
    ("sd", ("sd", "Arab", "PK")),
    ("sd-Arab", ("sd", "Arab", "PK")),
    ("sd-Deva", ("sd", "Deva", "IN")),
    ("se", ("se", "Latn", "NO")),
    ("se-FI", ("se", "Latn", "FI")),
    ("se-SE", ("se", "Latn", "SE")),
    ("seh", ("seh", "Latn", "MZ")),
    ("ses", ("ses", "Latn", "ML")),
    ("sg", ("sg", "Latn", "CF")),
    ("shi", ("shi", "Tfng", "MA")),
    ("shi-Latn", ("shi", "Latn", "MA")),
    ("shi-Tfng", ("shi", "Tfng", "MA")),
    ("si", ("si", "Sinh", "LK")),
    ("sk", ("sk", "Latn", "SK")),
    ("sl", ("sl", "Latn", "SI")),
    ("smn", ("smn", "Latn", "FI")),
    ("sms", ("sms", "Latn", "FI")),
    ("sn", ("sn", "Latn", "ZW")),
    ("so", ("so", "Latn", "SO")),
    ("so-DJ", ("so", "Latn", "DJ")),
    ("so-ET", ("so", "Latn", "ET")),
    ("so-KE", ("so", "Latn", "KE")),
    ("sq", ("sq", "Latn", "AL")),
    ("sq-MK", ("sq", "Latn", "MK")),
    ("sq-XK", ("sq", "Latn", "XK")),
    ("sr", ("sr", "Cyrl", "RS")),
    ("sr-Cyrl", ("sr", "Cyrl", "RS")),
    ("sr-Cyrl-BA", ("sr", "Cyrl", "BA")),
    ("sr-Cyrl-ME", ("sr", "Cyrl", "ME")),
    ("sr-Cyrl-XK", ("sr", "Cyrl", "XK")),
    ("sr-Latn", ("sr", "Latn", "RS")),
    ("sr-Latn-BA", ("sr", "Latn", "BA")),
    ("sr-Latn-ME", ("sr", "Latn", "ME")),
    ("sr-Latn-XK", ("sr", "Latn", "XK")),
    ("su", ("su", "Latn", "ID")),
    ("su-Latn", ("su", "Latn", "ID")),
    ("sv", ("sv", "Latn", "SE")),
    ("sv-AX", ("sv", "Latn", "AX")),
    ("sv-FI", ("sv", "Latn", "FI")),
    ("sw", ("sw", "Latn", "TZ")),
    ("sw-CD", ("sw", "Latn", "CD")),
    ("sw-KE", ("sw", "Latn", "KE")),
    ("sw-UG", ("sw", "Latn", "UG")),
    ("ta", ("ta", "Taml", "IN")),
    ("ta-LK", ("ta", "Taml", "LK")),
    ("ta-MY", ("ta", "Taml", "MY")),
    ("ta-SG", ("ta", "Taml", "SG")),
    ("te", ("te", "Telu", "IN")),
    ("teo", ("teo", "Latn", "UG")),
    ("teo-KE", ("teo", "Latn", "KE")),
    ("tg", ("tg", "Cyrl", "TJ")),
    ("th", ("th", "Thai", "TH")),
    ("ti", ("ti", "Ethi", "ET")),
    ("ti-ER", ("ti", "Ethi", "ER")),
    ("tk", ("tk", "Latn", "TM")),
    ("to", ("to", "Latn", "TO")),
    ("tok", ("tok", "Latn", "001")),
    ("tr", ("tr", "Latn", "TR")),
    ("tr-CY", ("tr", "Latn", "CY")),
    ("tt", ("tt", "Cyrl", "RU")),
    ("twq", ("twq", "Latn", "NE")),
    ("tzm", ("tzm", "Latn", "MA")),
    ("ug", ("ug", "Arab", "CN")),
    ("uk", ("uk", "Cyrl", "UA")),
    ("und", ("und", "Latn", "US")),
    ("ur", ("ur", "Arab", "PK")),
    ("ur-IN", ("ur", "Arab", "IN")),
    ("uz", ("uz", "Latn", "UZ")),
    ("uz-Arab", ("uz", "Arab", "AF")),
    ("uz-Cyrl", ("uz", "Cyrl", "UZ")),
    ("uz-Latn", ("uz", "Latn", "UZ")),
    ("vai", ("vai", "Vaii", "LR")),
    ("vai-Latn", ("vai", "Latn", "LR")),
    ("vai-Vaii", ("vai", "Vaii", "LR")),
    ("vi", ("vi", "Latn", "VN")),
    ("vun", ("vun", "Latn", "TZ")),
    ("wae", ("wae", "Latn", "CH")),
    ("wo", ("wo", "Latn", "SN")),
    ("xh", ("xh", "Latn", "ZA")),
    ("xog", ("xog", "Latn", "UG")),
    ("yav", ("yav", "Latn", "CM")),
    ("yi", ("yi", "Hebr", "001")),
    ("yo", ("yo", "Latn", "NG")),
    ("yo-BJ", ("yo", "Latn", "BJ")),
    ("yrl", ("yrl", "Latn", "BR")),
    ("yrl-CO", ("yrl", "Latn", "CO")),
    ("yrl-VE", ("yrl", "Latn", "VE")),
    ("yue", ("yue", "Hant", "HK")),
    ("yue-Hans", ("yue", "Hans", "CN")),
    ("yue-Hant", ("yue", "Hant", "HK")),
    ("zgh", ("zgh", "Tfng", "MA")),
    ("zh", ("zh", "Hans", "CN")),
    ("zh-Hans", ("zh", "Hans", "CN")),
    ("zh-Hans-HK", ("zh", "Hans", "HK")),
    ("zh-Hans-MO", ("zh", "Hans", "MO")),
    ("zh-Hans-SG", ("zh", "Hans", "SG")),
    ("zh-Hant", ("zh", "Hant", "TW")),
    ("zh-Hant-HK", ("zh", "Hant", "HK")),
    ("zh-Hant-MO", ("zh", "Hant", "MO")),
    ("zu", ("zu", "Latn", "ZA")),
    ("st", ("st", "Latn", "ZA")),
    ("la", ("la", "Latn", "VA")),
    ("ny", ("ny", "Latn", "MW")),
    ("sm", ("sm", "Latn", "WS")),
    ("jw", ("jw", "Latn", "ID")),
    ("ht", ("ht", "Latn", "HT")),
    ("co", ("co", "Latn", "FR")),
    ("tl", ("tl", "Latn", "PH")),
    ("iw", ("iw", "Hebr", "IL")),
  ];

  let raw_doc = r###"
This is a `phf::Map` constant that maps language codes to their
corresponding `TinyID` structs.

MAX means Maximize.
For example: en -> en-Latn-US

The `TinyID` struct is created using the `TinyID::new()` function, which takes three
arguments: `language`, `script`, and `region`. These arguments represent the
ISO 639-1 language code, ISO 15924 script code, and ISO 3166-1 alpha-2
region code for the language respectively.

## Example

```
let map = lang_id::maps::max::map();
let de = &map["de"];
assert_eq!(de.language, "de");
assert_eq!(de.script, "Latn");
assert_eq!(de.region, "DE");
```
  "###;

  // builder::tinyid::TinyIDMapBuilder

  builder::MapBuilder {
    header,
    raw_doc,
    mod_name,
    kv,
    map_type: Some("PhfTinyidMap"),
    ..Default::default()
  }
  .build()
  // Ok(())
}
