mod builder;
use std::io;

use crate::builder::map_type::MapType;

#[test]
#[ignore]
fn new_map() -> io::Result<()> {
  let str_kv = &[
    ("af", "Afrikaans, Latyn, Suid-Afrika"),
    ("af-NA", "Afrikaans, Latyn, Namibië"),
    ("agq", "Aghem, Latn, Kàmàlûŋ"),
    ("ak", "Akan, Latn, Gaana"),
    ("am", "አማርኛ, ኢትዮፒክ, ኢትዮጵያ"),
    ("ann", "Obolo, Latn, NG"),
    ("ar", "العربية, العربية, مصر"),
    ("ar-AE", "العربية, العربية, الإمارات العربية المتحدة"),
    ("ar-BH", "العربية, العربية, البحرين"),
    ("ar-DJ", "العربية, العربية, جيبوتي"),
    ("ar-DZ", "العربية, العربية, الجزائر"),
    ("ar-EG", "العربية, العربية, مصر"),
    ("ar-EH", "العربية, العربية, الصحراء الغربية"),
    ("ar-ER", "العربية, العربية, إريتريا"),
    ("ar-IL", "العربية, العربية, إسرائيل"),
    ("ar-IQ", "العربية, العربية, العراق"),
    ("ar-JO", "العربية, العربية, الأردن"),
    ("ar-KM", "العربية, العربية, جزر القمر"),
    ("ar-KW", "العربية, العربية, الكويت"),
    ("ar-LB", "العربية, العربية, لبنان"),
    ("ar-LY", "العربية, العربية, ليبيا"),
    ("ar-MA", "العربية, العربية, المغرب"),
    ("ar-MR", "العربية, العربية, موريتانيا"),
    ("ar-OM", "العربية, العربية, عُمان"),
    ("ar-PS", "العربية, العربية, الأراضي الفلسطينية"),
    ("ar-QA", "العربية, العربية, قطر"),
    ("ar-SA", "العربية, العربية, المملكة العربية السعودية"),
    ("ar-SD", "العربية, العربية, السودان"),
    ("ar-SO", "العربية, العربية, الصومال"),
    ("ar-SS", "العربية, العربية, جنوب السودان"),
    ("ar-SY", "العربية, العربية, سوريا"),
    ("ar-TD", "العربية, العربية, تشاد"),
    ("ar-TN", "العربية, العربية, تونس"),
    ("ar-YE", "العربية, العربية, اليمن"),
    ("as", "অসমীয়া, বাংলা, ভাৰত"),
    ("asa", "Kipare, Latn, Tadhania"),
    ("ast", "asturianu, llatín, España"),
    ("az", "azərbaycan, latın, Azərbaycan"),
    ("az-Cyrl", "азәрбајҹан, Кирил, Азәрбајҹан"),
    ("az-Latn", "azərbaycan, latın, Azərbaycan"),
    ("bas", "Ɓàsàa, Latn, Kàmɛ̀rûn"),
    ("be", "беларуская, кірыліца, Беларусь"),
    ("bem", "Ichibemba, Latn, Zambia"),
    ("bez", "Hibena, Latn, Hutanzania"),
    ("bg", "български, кирилица, България"),
    ("bgc", "हरियाणवी, देवानागारी, भारत"),
    ("bho", "भोजपुरी, देवानागारी, भारत"),
    ("bm", "bamanakan, Latn, Mali"),
    ("bn", "বাংলা, বাংলা, বাংলাদেশ"),
    ("bn-IN", "বাংলা, বাংলা, ভারত"),
    ("bo", "བོད་སྐད་, བོད་ཡིག་, རྒྱ་ནག"),
    ("bo-IN", "བོད་སྐད་, བོད་ཡིག་, རྒྱ་གར་"),
    ("br", "brezhoneg, latin, Frañs"),
    ("brx", "बर’, देबनागिरि, भारत"),
    ("bs", "bosanski, latinica, Bosna i Hercegovina"),
    ("bs-Cyrl", "босански, ћирилица, Босна и Херцеговина"),
    ("bs-Latn", "bosanski, latinica, Bosna i Hercegovina"),
    ("ca", "català, llatí, Espanya"),
    ("ca-AD", "català, llatí, Andorra"),
    ("ca-FR", "català, llatí, França"),
    ("ca-IT", "català, llatí, Itàlia"),
    ("ccp", "𑄌𑄋𑄴𑄟𑄳𑄦, 𑄌𑄇𑄴𑄟, 𑄝𑄁𑄣𑄘𑄬𑄌𑄴"),
    ("ccp-IN", "𑄌𑄋𑄴𑄟𑄳𑄦, 𑄌𑄇𑄴𑄟, 𑄞𑄢𑄧𑄖𑄴"),
    ("ce", "нохчийн, кириллица, Росси"),
    ("ceb", "Cebuano, Latin, Pilipinas"),
    ("cgg", "Rukiga, Latn, Uganda"),
    ("chr", "ᏣᎳᎩ, ᏣᎳᎩ, ᏌᏊ ᎢᏳᎾᎵᏍᏔᏅ ᏍᎦᏚᎩ"),
    ("ckb", "کوردیی ناوەندی, عەرەبی, عێراق"),
    ("ckb-IR", "کوردیی ناوەندی, عەرەبی, ئێران"),
    ("cs", "čeština, latinka, Česko"),
    ("cv", "чӑваш, кириллица, Раҫҫей"),
    ("cy", "Cymraeg, Lladin, Y Deyrnas Unedig"),
    ("da", "dansk, latinsk, Danmark"),
    ("da-GL", "dansk, latinsk, Grønland"),
    ("dav", "Kitaita, Latn, Kenya"),
    ("de", "Deutsch, Lateinisch, Deutschland"),
    ("de-AT", "Deutsch, Lateinisch, Österreich"),
    ("de-BE", "Deutsch, Lateinisch, Belgien"),
    ("de-CH", "Deutsch, Lateinisch, Schweiz"),
    ("de-IT", "Deutsch, Lateinisch, Italien"),
    ("de-LI", "Deutsch, Lateinisch, Liechtenstein"),
    ("de-LU", "Deutsch, Lateinisch, Luxemburg"),
    ("dje", "Zarmaciine, Latn, Nižer"),
    ("doi", "डोगरी, देवनागरी, भारत"),
    ("dsb", "dolnoserbšćina, łatyński, Nimska"),
    ("dua", "duálá, Latn, Cameroun"),
    ("dyo", "joola, Latn, Senegal"),
    ("dz", "རྫོང་ཁ, ང་བཅས་ཀྱི་ཡིག་གུ, འབྲུག"),
    ("ebu", "Kĩembu, Latn, Kenya"),
    ("ee", "Eʋegbe, Latingbeŋɔŋlɔ, Ghana nutome"),
    ("ee-TG", "Eʋegbe, Latingbeŋɔŋlɔ, Togo nutome"),
    ("el", "Ελληνικά, Ελληνικό, Ελλάδα"),
    ("el-CY", "Ελληνικά, Ελληνικό, Κύπρος"),
    ("en", "English, Latin, United States"),
    ("en-001", "English, Latin, world"),
    ("en-150", "English, Latin, Europe"),
    ("en-AE", "English, Latin, United Arab Emirates"),
    ("en-AG", "English, Latin, Antigua & Barbuda"),
    ("en-AI", "English, Latin, Anguilla"),
    ("en-AS", "English, Latin, American Samoa"),
    ("en-AT", "English, Latin, Austria"),
    ("en-AU", "English, Latin, Australia"),
    ("en-BB", "English, Latin, Barbados"),
    ("en-BE", "English, Latin, Belgium"),
    ("en-BI", "English, Latin, Burundi"),
    ("en-BM", "English, Latin, Bermuda"),
    ("en-BS", "English, Latin, Bahamas"),
    ("en-BW", "English, Latin, Botswana"),
    ("en-BZ", "English, Latin, Belize"),
    ("en-CA", "English, Latin, Canada"),
    ("en-CC", "English, Latin, Cocos (Keeling) Islands"),
    ("en-CH", "English, Latin, Switzerland"),
    ("en-CK", "English, Latin, Cook Islands"),
    ("en-CM", "English, Latin, Cameroon"),
    ("en-CX", "English, Latin, Christmas Island"),
    ("en-CY", "English, Latin, Cyprus"),
    ("en-DE", "English, Latin, Germany"),
    ("en-DG", "English, Latin, Diego Garcia"),
    ("en-DK", "English, Latin, Denmark"),
    ("en-DM", "English, Latin, Dominica"),
    ("en-ER", "English, Latin, Eritrea"),
    ("en-FI", "English, Latin, Finland"),
    ("en-FJ", "English, Latin, Fiji"),
    ("en-FK", "English, Latin, Falkland Islands"),
    ("en-FM", "English, Latin, Micronesia"),
    ("en-GB", "English, Latin, United Kingdom"),
    ("en-GD", "English, Latin, Grenada"),
    ("en-GG", "English, Latin, Guernsey"),
    ("en-GH", "English, Latin, Ghana"),
    ("en-GI", "English, Latin, Gibraltar"),
    ("en-GM", "English, Latin, Gambia"),
    ("en-GU", "English, Latin, Guam"),
    ("en-GY", "English, Latin, Guyana"),
    ("en-HK", "English, Latin, Hong Kong SAR China"),
    ("en-IE", "English, Latin, Ireland"),
    ("en-IL", "English, Latin, Israel"),
    ("en-IM", "English, Latin, Isle of Man"),
    ("en-IN", "English, Latin, India"),
    ("en-IO", "English, Latin, British Indian Ocean Territory"),
    ("en-JE", "English, Latin, Jersey"),
    ("en-JM", "English, Latin, Jamaica"),
    ("en-KE", "English, Latin, Kenya"),
    ("en-KI", "English, Latin, Kiribati"),
    ("en-KN", "English, Latin, St Kitts & Nevis"),
    ("en-KY", "English, Latin, Cayman Islands"),
    ("en-LC", "English, Latin, St Lucia"),
    ("en-LR", "English, Latin, Liberia"),
    ("en-LS", "English, Latin, Lesotho"),
    ("en-MG", "English, Latin, Madagascar"),
    ("en-MH", "English, Latin, Marshall Islands"),
    ("en-MO", "English, Latin, Macao SAR China"),
    ("en-MP", "English, Latin, Northern Mariana Islands"),
    ("en-MS", "English, Latin, Montserrat"),
    ("en-MT", "English, Latin, Malta"),
    ("en-MU", "English, Latin, Mauritius"),
    ("en-MV", "English, Latin, Maldives"),
    ("en-MW", "English, Latin, Malawi"),
    ("en-MY", "English, Latin, Malaysia"),
    ("en-NA", "English, Latin, Namibia"),
    ("en-NF", "English, Latin, Norfolk Island"),
    ("en-NG", "English, Latin, Nigeria"),
    ("en-NL", "English, Latin, Netherlands"),
    ("en-NR", "English, Latin, Nauru"),
    ("en-NU", "English, Latin, Niue"),
    ("en-NZ", "English, Latin, New Zealand"),
    ("en-PG", "English, Latin, Papua New Guinea"),
    ("en-PH", "English, Latin, Philippines"),
    ("en-PK", "English, Latin, Pakistan"),
    ("en-PN", "English, Latin, Pitcairn Islands"),
    ("en-PR", "English, Latin, Puerto Rico"),
    ("en-PW", "English, Latin, Palau"),
    ("en-RW", "English, Latin, Rwanda"),
    ("en-SB", "English, Latin, Solomon Islands"),
    ("en-SC", "English, Latin, Seychelles"),
    ("en-SD", "English, Latin, Sudan"),
    ("en-SE", "English, Latin, Sweden"),
    ("en-SG", "English, Latin, Singapore"),
    ("en-SH", "English, Latin, St Helena"),
    ("en-SI", "English, Latin, Slovenia"),
    ("en-SL", "English, Latin, Sierra Leone"),
    ("en-SS", "English, Latin, South Sudan"),
    ("en-SX", "English, Latin, Sint Maarten"),
    ("en-SZ", "English, Latin, Eswatini"),
    ("en-TC", "English, Latin, Turks & Caicos Islands"),
    ("en-TK", "English, Latin, Tokelau"),
    ("en-TO", "English, Latin, Tonga"),
    ("en-TT", "English, Latin, Trinidad & Tobago"),
    ("en-TV", "English, Latin, Tuvalu"),
    ("en-TZ", "English, Latin, Tanzania"),
    ("en-UG", "English, Latin, Uganda"),
    ("en-UM", "English, Latin, U.S. Outlying Islands"),
    ("en-VC", "English, Latin, St Vincent & the Grenadines"),
    ("en-VG", "English, Latin, British Virgin Islands"),
    ("en-VI", "English, Latin, U.S. Virgin Islands"),
    ("en-VU", "English, Latin, Vanuatu"),
    ("en-WS", "English, Latin, Samoa"),
    ("en-ZA", "English, Latin, South Africa"),
    ("en-ZM", "English, Latin, Zambia"),
    ("en-ZW", "English, Latin, Zimbabwe"),
    ("eo", "esperanto, Latn, Mondo"),
    ("es", "español, latino, España"),
    ("es-419", "español, latín, Latinoamérica"),
    ("es-AR", "español, latín, Argentina"),
    ("es-BO", "español, latín, Bolivia"),
    ("es-BR", "español, latín, Brasil"),
    ("es-BZ", "español, latín, Belice"),
    ("es-CL", "español, latín, Chile"),
    ("es-CO", "español, latín, Colombia"),
    ("es-CR", "español, latín, Costa Rica"),
    ("es-CU", "español, latín, Cuba"),
    ("es-DO", "español, latín, República Dominicana"),
    ("es-EA", "español, latino, Ceuta y Melilla"),
    ("es-EC", "español, latín, Ecuador"),
    ("es-GQ", "español, latino, Guinea Ecuatorial"),
    ("es-GT", "español, latín, Guatemala"),
    ("es-HN", "español, latín, Honduras"),
    ("es-IC", "español, latino, Canarias"),
    ("es-MX", "español, latín, México"),
    ("es-NI", "español, latín, Nicaragua"),
    ("es-PA", "español, latín, Panamá"),
    ("es-PE", "español, latín, Perú"),
    ("es-PH", "español, latino, Filipinas"),
    ("es-PR", "español, latín, Puerto Rico"),
    ("es-PY", "español, latín, Paraguay"),
    ("es-SV", "español, latín, El Salvador"),
    ("es-US", "español, latín, Estados Unidos"),
    ("es-UY", "español, latín, Uruguay"),
    ("es-VE", "español, latín, Venezuela"),
    ("et", "eesti, ladina, Eesti"),
    ("eu", "euskara, latinoa, Espainia"),
    ("ewo", "ewondo, Latn, Kamərún"),
    ("fa", "فارسی, عربی, ایران"),
    ("fa-AF", "فارسی, عربی, افغانستان"),
    ("ff", "Pulaar, Latn, Senegaal"),
    ("ff-Adlm", "𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤘𞤭𞤲𞤫"),
    ("ff-Adlm-BF", "𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤄𞤵𞤪𞤳𞤭𞤲𞤢 𞤊𞤢𞤧𞤮𞥅"),
    ("ff-Adlm-CM", "𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤑𞤢𞤥𞤢𞤪𞤵𞥅𞤲"),
    ("ff-Adlm-GH", "𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤘𞤢𞤲𞤢"),
    ("ff-Adlm-GM", "𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤘𞤢𞤥𞤦𞤭𞤴𞤢"),
    ("ff-Adlm-GW", "𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤘𞤭𞤲𞤫-𞤄𞤭𞤧𞤢𞤱𞤮𞥅"),
    ("ff-Adlm-LR", "𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤂𞤢𞤦𞤭𞤪𞤭𞤴𞤢𞥄"),
    ("ff-Adlm-MR", "𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤃𞤮𞤪𞤼𞤢𞤲𞤭𞥅"),
    ("ff-Adlm-NE", "𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤐𞤭𞥅𞤶𞤫𞤪"),
    ("ff-Adlm-NG", "𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤐𞤢𞤶𞤫𞤪𞤭𞤴𞤢𞥄"),
    ("ff-Adlm-SL", "𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤅𞤢𞤪𞤢𞤤𞤮𞤲"),
    ("ff-Adlm-SN", "𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤅𞤫𞤲𞤫𞤺𞤢𞥄𞤤"),
    ("ff-Latn", "Pulaar, Latn, Senegaal"),
    ("ff-Latn-BF", "Pulaar, Latn, Burkibaa Faaso"),
    ("ff-Latn-CM", "Pulaar, Latn, Kameruun"),
    ("ff-Latn-GH", "Pulaar, Latn, Ganaa"),
    ("ff-Latn-GM", "Pulaar, Latn, Gammbi"),
    ("ff-Latn-GN", "Pulaar, Latn, Gine"),
    ("ff-Latn-GW", "Pulaar, Latn, Gine-Bisaawo"),
    ("ff-Latn-LR", "Pulaar, Latn, Liberiyaa"),
    ("ff-Latn-MR", "Pulaar, Latn, Muritani"),
    ("ff-Latn-NE", "Pulaar, Latn, Nijeer"),
    ("ff-Latn-NG", "Pulaar, Latn, Nijeriyaa"),
    ("ff-Latn-SL", "Pulaar, Latn, Seraa liyon"),
    ("fi", "suomi, latinalainen, Suomi"),
    ("fil", "Filipino, Latin, Pilipinas"),
    ("fo", "føroyskt, latínskt, Føroyar"),
    ("fo-DK", "føroyskt, latínskt, Danmark"),
    ("fr", "français, latin, France"),
    ("fr-BE", "français, latin, Belgique"),
    ("fr-BF", "français, latin, Burkina Faso"),
    ("fr-BI", "français, latin, Burundi"),
    ("fr-BJ", "français, latin, Bénin"),
    ("fr-BL", "français, latin, Saint-Barthélemy"),
    ("fr-CA", "français, latin, Canada"),
    ("fr-CD", "français, latin, Congo-Kinshasa"),
    ("fr-CF", "français, latin, République centrafricaine"),
    ("fr-CG", "français, latin, Congo-Brazzaville"),
    ("fr-CH", "français, latin, Suisse"),
    ("fr-CI", "français, latin, Côte d’Ivoire"),
    ("fr-CM", "français, latin, Cameroun"),
    ("fr-DJ", "français, latin, Djibouti"),
    ("fr-DZ", "français, latin, Algérie"),
    ("fr-GA", "français, latin, Gabon"),
    ("fr-GF", "français, latin, Guyane française"),
    ("fr-GN", "français, latin, Guinée"),
    ("fr-GP", "français, latin, Guadeloupe"),
    ("fr-GQ", "français, latin, Guinée équatoriale"),
    ("fr-HT", "français, latin, Haïti"),
    ("fr-KM", "français, latin, Comores"),
    ("fr-LU", "français, latin, Luxembourg"),
    ("fr-MA", "français, latin, Maroc"),
    ("fr-MC", "français, latin, Monaco"),
    ("fr-MF", "français, latin, Saint-Martin"),
    ("fr-MG", "français, latin, Madagascar"),
    ("fr-ML", "français, latin, Mali"),
    ("fr-MQ", "français, latin, Martinique"),
    ("fr-MR", "français, latin, Mauritanie"),
    ("fr-MU", "français, latin, Maurice"),
    ("fr-NC", "français, latin, Nouvelle-Calédonie"),
    ("fr-NE", "français, latin, Niger"),
    ("fr-PF", "français, latin, Polynésie française"),
    ("fr-PM", "français, latin, Saint-Pierre-et-Miquelon"),
    ("fr-RE", "français, latin, La Réunion"),
    ("fr-RW", "français, latin, Rwanda"),
    ("fr-SC", "français, latin, Seychelles"),
    ("fr-SN", "français, latin, Sénégal"),
    ("fr-SY", "français, latin, Syrie"),
    ("fr-TD", "français, latin, Tchad"),
    ("fr-TG", "français, latin, Togo"),
    ("fr-TN", "français, latin, Tunisie"),
    ("fr-VU", "français, latin, Vanuatu"),
    ("fr-WF", "français, latin, Wallis-et-Futuna"),
    ("fr-YT", "français, latin, Mayotte"),
    ("frr", "Nordfriisk, Latn, DE"),
    ("fur", "furlan, latin, Italie"),
    ("fy", "Frysk, Latyn, Nederlân"),
    ("ga", "Gaeilge, Laidineach, Éire"),
    ("ga-GB", "Gaeilge, Laidineach, an Ríocht Aontaithe"),
    ("gd", "Gàidhlig, Laideann, An Rìoghachd Aonaichte"),
    ("gl", "galego, latino, España"),
    ("gsw", "Schwiizertüütsch, Latiinisch, Schwiiz"),
    ("gsw-FR", "Schwiizertüütsch, Latiinisch, Frankriich"),
    ("gsw-LI", "Schwiizertüütsch, Latiinisch, Liächteschtäi"),
    ("gu", "ગુજરાતી, ગુજરાતી, ભારત"),
    ("guz", "Ekegusii, Latn, Kenya"),
    ("gv", "Gaelg, Latn, Ellan Vannin"),
    ("ha", "Hausa, Latin, Nijeriya"),
    ("ha-GH", "Hausa, Latin, Gana"),
    ("ha-NE", "Hausa, Latin, Nijar"),
    ("haw", "ʻŌlelo Hawaiʻi, Latn, ʻAmelika Hui Pū ʻIa"),
    ("he", "עברית, עברי, ישראל"),
    ("hi", "हिन्दी, देवनागरी, भारत"),
    ("hi-Latn", "Hindi (Latin), India"),
    ("hr", "hrvatski, latinica, Hrvatska"),
    ("hr-BA", "hrvatski, latinica, Bosna i Hercegovina"),
    ("hsb", "hornjoserbšćina, łaćonsce, Němska"),
    ("hu", "magyar, Latin, Magyarország"),
    ("hy", "հայերեն, հայկական, Հայաստան"),
    ("ia", "interlingua, latin, Mundo"),
    ("id", "Indonesia, Latin, Indonesia"),
    ("ig", "Igbo, Latin, Naịjịrịa"),
    ("ii", "ꆈꌠꉙ, ꆈꌠꁱꂷ, ꍏꇩ"),
    ("is", "íslenska, latneskt, Ísland"),
    ("it", "italiano, latino, Italia"),
    ("it-CH", "italiano, latino, Svizzera"),
    ("it-SM", "italiano, latino, San Marino"),
    ("it-VA", "italiano, latino, Città del Vaticano"),
    ("ja", "日本語, 日本語の文字, 日本"),
    ("jgo", "Ndaꞌa, mík -ŋwaꞌnɛ yi ɛ́ líŋɛ́nɛ Latɛ̂ŋ, Kamɛlûn"),
    ("jmc", "Kimachame, Latn, Tanzania"),
    ("jv", "Jawa, Latin, Indonésia"),
    ("ka", "ქართული, ქართული, საქართველო"),
    ("kab", "Taqbaylit, Latn, Lezzayer"),
    ("kam", "Kikamba, Latn, Kenya"),
    ("kde", "Chimakonde, Latn, Tanzania"),
    ("kea", "kabuverdianu, latinu, Kabu Verdi"),
    ("kgp", "kanhgág, ratĩnh, Mrasir"),
    ("khq", "Koyra ciini, Latn, Maali"),
    ("ki", "Gikuyu, Latn, Kenya"),
    ("kk", "қазақ тілі, кирилл жазуы, Қазақстан"),
    ("kkj", "kakɔ, Latn, Kamɛrun"),
    ("kl", "kalaallisut, Latn, Kalaallit Nunaat"),
    ("kln", "Kalenjin, Latn, Emetab Kenya"),
    ("km", "ខ្មែរ, ខ្មែរ, កម្ពុជា"),
    ("kn", "ಕನ್ನಡ, ಕನ್ನಡ, ಭಾರತ"),
    ("ko", "한국어, 한국 문자, 대한민국"),
    ("ko-KP", "한국어, 한국 문자, 조선민주주의인민공화국"),
    ("kok", "कोंकणी, देवनागरी, भारत"),
    ("ks", "کٲشُر, عربی, ہِندوستان"),
    ("ks-Arab", "کٲشُر, عربی, ہِندوستان"),
    ("ks-Deva", "कॉशुर, देवनागरी, हिंदोस्तान"),
    ("ksb", "Kishambaa, Latn, Tanzania"),
    ("ksf", "rikpa, Latn, kamɛrún"),
    ("ksh", "Kölsch, lateinesche Schreff, Doütschland"),
    ("ku", "kurdî, latînî, Tirkiye"),
    ("kw", "kernewek, Latn, Rywvaneth Unys"),
    ("ky", "кыргызча, Кирилл, Кыргызстан"),
    ("lag", "Kɨlaangi, Latn, Taansanía"),
    ("lb", "Lëtzebuergesch, Laténgesch, Lëtzebuerg"),
    ("lg", "Luganda, Latn, Yuganda"),
    ("lkt", "Lakȟólʼiyapi, Latn, Mílahaŋska Tȟamákȟočhe"),
    ("ln", "lingála, Latn, Republíki ya Kongó Demokratíki"),
    ("ln-AO", "lingála, Latn, Angóla"),
    ("ln-CF", "lingála, Latn, Repibiki ya Afríka ya Káti"),
    ("ln-CG", "lingála, Latn, Kongo"),
    ("lo", "ລາວ, ລາວ, ລາວ"),
    ("lrc", "لۊری شومالی, عأرأڤی, IR"),
    ("lrc-IQ", "لۊری شومالی, عأرأڤی, IQ"),
    ("lt", "lietuvių, lotynų, Lietuva"),
    ("lu", "Tshiluba, Latn, Ditunga wa Kongu"),
    ("luo", "Dholuo, Latn, Kenya"),
    ("luy", "Luluhia, Latn, Kenya"),
    ("lv", "latviešu, latīņu, Latvija"),
    ("lzh", "文言文, 古典漢字, 華夏"),
    ("lzh-Hans", "文言文, 简体汉字, 现代中国"),
    ("mai", "मैथिली, देवनागरी, भारत"),
    ("mas", "Maa, Latn, Kenya"),
    ("mas-TZ", "Maa, Latn, Tansania"),
    ("mdf", "мокшень кяль, Cyrl, RU"),
    ("mer", "Kĩmĩrũ, Latn, Kenya"),
    ("mfe", "kreol morisien, Latn, Moris"),
    ("mg", "Malagasy, Latn, Madagasikara"),
    ("mgh", "Makua, Latn, Umozambiki"),
    ("mgo", "metaʼ, ngam ŋwaʼri, Kamalun"),
    ("mi", "Māori, Rātina, Aotearoa"),
    ("mk", "македонски, кирилско писмо, Северна Македонија"),
    ("ml", "മലയാളം, മലയാളം, ഇന്ത്യ"),
    ("mn", "монгол, кирилл, Монгол"),
    ("mni", "মৈতৈলোন্, বাংলা, ইন্দিয়া"),
    ("mni-Beng", "মৈতৈলোন্, বাংলা, ইন্দিয়া"),
    ("mr", "मराठी, देवनागरी, भारत"),
    ("ms", "Melayu, Latin, Malaysia"),
    ("ms-BN", "Melayu, Latin, Brunei"),
    ("ms-ID", "Melayu, Latin, Indonesia"),
    ("ms-SG", "Melayu, Latin, Singapura"),
    ("mt", "Malti, Latin, Malta"),
    ("mua", "MUNDAŊ, Latn, kameruŋ"),
    ("my", "မြန်မာ, မြန်မာ, မြန်မာ"),
    ("mzn", "مازرونی, عربی, ایران"),
    ("naq", "Khoekhoegowab, Latn, Namibiab"),
    ("nb", "norsk bokmål, latinsk, Norge"),
    ("nb-SJ", "norsk bokmål, latinsk, Svalbard og Jan Mayen"),
    ("nd", "isiNdebele, Latn, Zimbabwe"),
    ("nds", "Plattdüütsch, Latiensch Schrift, Düütschland"),
    ("nds-NL", "Nedersaksisch, Latijns Schrift, Nederlaand"),
    ("ne", "नेपाली, देवानागरी, नेपाल"),
    ("ne-IN", "नेपाली, देवानागरी, भारत"),
    ("nl", "Nederlands, Latijns, Nederland"),
    ("nl-AW", "Nederlands, Latijns, Aruba"),
    ("nl-BE", "Nederlands, Latijns, België"),
    ("nl-BQ", "Nederlands, Latijns, Caribisch Nederland"),
    ("nl-CW", "Nederlands, Latijns, Curaçao"),
    ("nl-SR", "Nederlands, Latijns, Suriname"),
    ("nl-SX", "Nederlands, Latijns, Sint-Maarten"),
    ("nmg", "Nmgwa, Látín, Kamɛrun"),
    ("nn", "norsk nynorsk, latinsk, Noreg"),
    ("nnh", "Shwóŋò ngiembɔɔn, Latn, Kàmalûm"),
    ("no", "norsk, latinsk, Norge"),
    ("nus", "Thok Nath, Latn, SS"),
    ("nyn", "Runyankore, Latn, Uganda"),
    ("oc", "Occitan, Alfabet latin, França"), // ​Lenga d'òc​
    ("oc-ES", "Aranés, Alfabet latin, Espanha"),
    ("om", "Oromoo, Latin, Itoophiyaa"),
    ("om-KE", "Oromoo, Latin, Keeniyaa"),
    ("or", "ଓଡ଼ିଆ, ଓଡ଼ିଆ, ଭାରତ"),
    ("os", "ирон, Киррилицӕ, Гуырдзыстон"),
    ("os-RU", "ирон, Киррилицӕ, Уӕрӕсе"),
    ("pa", "ਪੰਜਾਬੀ, ਗੁਰਮੁਖੀ, ਭਾਰਤ"),
    ("pa-Arab", "پنجابی, عربی, پاکستان"),
    ("pa-Guru", "ਪੰਜਾਬੀ, ਗੁਰਮੁਖੀ, ਭਾਰਤ"),
    ("pcm", "Naijíriá Píjin, Látin, Naijíria"),
    ("pis", "Pijin, Latin, Solomon Aelan"),
    ("pl", "polski, łacińskie, Polska"),
    ("ps", "پښتو, عربي, افغانستان"),
    ("ps-PK", "پښتو, عربي, پاکستان"),
    ("pt", "português, latim, Brasil"),
    ("pt-AO", "português, latim, Angola"),
    ("pt-CH", "português, latim, Suíça"),
    ("pt-CV", "português, latim, Cabo Verde"),
    ("pt-GQ", "português, latim, Guiné Equatorial"),
    ("pt-GW", "português, latim, Guiné-Bissau"),
    ("pt-LU", "português, latim, Luxemburgo"),
    ("pt-MO", "português, latim, Macau, RAE da China"),
    ("pt-MZ", "português, latim, Moçambique"),
    ("pt-PT", "português, latim, Portugal"),
    ("pt-ST", "português, latim, São Tomé e Príncipe"),
    ("pt-TL", "português, latim, Timor-Leste"),
    ("qu", "Runasimi, Latin Simi, Perú"),
    ("qu-BO", "Runasimi, Latin Simi, Bolivia"),
    ("qu-EC", "Runasimi, Latin Simi, Ecuador"),
    ("raj", "राजस्थानी, देवनागरी, भारत"),
    ("rm", "rumantsch, latin, Svizra"),
    ("rn", "Ikirundi, Latn, Uburundi"),
    ("ro", "română, latină, România"),
    ("ro-MD", "română, latină, Republica Moldova"),
    ("rof", "Kihorombo, Latn, Tanzania"),
    ("ru", "русский, кириллица, Россия"),
    ("ru-BY", "русский, кириллица, Беларусь"),
    ("ru-KG", "русский, кириллица, Киргизия"),
    ("ru-KZ", "русский, кириллица, Казахстан"),
    ("ru-MD", "русский, кириллица, Молдова"),
    ("ru-UA", "русский, кириллица, Украина"),
    ("rw", "Kinyarwanda, Latn, U Rwanda"),
    ("rwk", "Kiruwa, Latn, Tanzania"),
    ("sa", "संस्कृतम्, देवनागरी, भारत"),
    ("sah", "саха тыла, Нууччалыы, Арассыыйа"),
    ("saq", "Kisampur, Latn, Kenya"),
    ("sat", "ᱥᱟᱱᱛᱟᱲᱤ, ᱚᱞ ᱪᱤᱠᱤ, ᱤᱱᱰᱤᱭᱟ"),
    ("sat-Olck", "ᱥᱟᱱᱛᱟᱲᱤ, ᱚᱞ ᱪᱤᱠᱤ, ᱤᱱᱰᱤᱭᱟ"),
    ("sbp", "Ishisangu, Latn, Tansaniya"),
    ("sc", "sardu, latinu, Itàlia"),
    ("sd", "سنڌي, عربي, پاڪستان"),
    ("sd-Arab", "سنڌي, عربي, پاڪستان"),
    ("sd-Deva", "सिन्धी, देवनागिरी, भारत"),
    ("se", "davvisámegiella, láhtenaš, Norga"),
    ("se-FI", "davvisámegiella, láhtenaš, Suopma"),
    ("se-SE", "davvisámegiella, láhtenaš, Ruoŧŧa"),
    ("seh", "sena, Latn, Moçambique"),
    ("ses", "Koyraboro senni, Latn, Maali"),
    ("sg", "Sängö, Latn, Ködörösêse tî Bêafrîka"),
    ("shi", "ⵜⴰⵛⵍⵃⵉⵜ, Tfng, ⵍⵎⵖⵔⵉⴱ"),
    ("shi-Latn", "Tashelḥiyt, Latn, lmɣrib"),
    ("shi-Tfng", "ⵜⴰⵛⵍⵃⵉⵜ, Tfng, ⵍⵎⵖⵔⵉⴱ"),
    ("si", "සිංහල, සිංහල, ශ්‍රී ලංකාව"),
    ("sk", "slovenčina, latinka, Slovensko"),
    ("sl", "slovenščina, latinica, Slovenija"),
    ("smn", "anarâškielâ, Latn, Suomâ"),
    ("sms", "sms, Latn, FI"),
    ("sn", "chiShona, Latn, Zimbabwe"),
    ("so", "Soomaali, Laatiin, Soomaaliya"),
    ("so-DJ", "Soomaali, Laatiin, Jabuuti"),
    ("so-ET", "Soomaali, Laatiin, Itoobiya"),
    ("so-KE", "Soomaali, Laatiin, Kenya"),
    ("sq", "shqip, latin, Shqipëri"),
    ("sq-MK", "shqip, latin, Maqedonia e Veriut"),
    ("sq-XK", "shqip, latin, Kosovë"),
    ("sr", "српски, ћирилица, Србија"),
    ("sr-Cyrl", "српски, ћирилица, Србија"),
    ("sr-Cyrl-BA", "српски, ћирилица, Босна и Херцеговина"),
    ("sr-Cyrl-ME", "српски, ћирилица, Црна Гора"),
    ("sr-Cyrl-XK", "српски, ћирилица, Косово"),
    ("sr-Latn", "srpski, latinica, Srbija"),
    ("sr-Latn-BA", "srpski, latinica, Bosna i Hercegovina"),
    ("sr-Latn-ME", "srpski, latinica, Crna Gora"),
    ("sr-Latn-XK", "srpski, latinica, Kosovo"),
    ("su", "Basa Sunda, Latin, Indonesia"),
    ("su-Latn", "Basa Sunda, Latin, Indonesia"),
    ("sv", "svenska, latinska, Sverige"),
    ("sv-AX", "svenska, latinska, Åland"),
    ("sv-FI", "svenska, latinska, Finland"),
    ("sw", "Kiswahili, Kilatini, Tanzania"),
    (
      "sw-CD",
      "Kiswahili, Kilatini, Jamhuri ya Kidemokrasia ya Kongo",
    ),
    ("sw-KE", "Kiswahili, Kilatini, Kenya"),
    ("sw-UG", "Kiswahili, Kilatini, Uganda"),
    ("ta", "தமிழ், தமிழ், இந்தியா"),
    ("ta-LK", "தமிழ், தமிழ், இலங்கை"),
    ("ta-MY", "தமிழ், தமிழ், மலேசியா"),
    ("ta-SG", "தமிழ், தமிழ், சிங்கப்பூர்"),
    ("te", "తెలుగు, తెలుగు, భారతదేశం"),
    ("teo", "Kiteso, Latn, Uganda"),
    ("teo-KE", "Kiteso, Latn, Kenia"),
    ("tg", "тоҷикӣ, Кириллӣ, Тоҷикистон"),
    ("th", "ไทย, ไทย, ไทย"),
    ("ti", "ትግርኛ, ፊደል, ኢትዮጵያ"),
    ("ti-ER", "ትግርኛ, ፊደል, ኤርትራ"),
    ("tk", "türkmen dili, Latyn elipbiýi, Türkmenistan"),
    ("to", "lea fakatonga, tohinima fakalatina, Tonga"),
    ("tok", "Toki Pona, Latn, 001"),
    ("tr", "Türkçe, Latin, Türkiye"),
    ("tr-CY", "Türkçe, Latin, Kıbrıs"),
    ("tt", "татар, кирилл, Россия"),
    ("twq", "Tasawaq senni, Latn, Nižer"),
    ("tzm", "Tamaziɣt n laṭlaṣ, Latn, Meṛṛuk"),
    ("ug", "ئۇيغۇرچە, ئەرەب, جۇڭگو"),
    ("uk", "українська, кирилиця, Україна"),
    ("und", "und, Latn, US"),
    ("ur", "اردو, عربی, پاکستان"),
    ("ur-IN", "اردو, عربی, بھارت"),
    ("uz", "o‘zbek, lotin, Oʻzbekiston"),
    ("uz-Arab", "اوزبیک, عربی, افغانستان"),
    ("uz-Cyrl", "ўзбекча, Кирил, Ўзбекистон"),
    ("uz-Latn", "o‘zbek, lotin, Oʻzbekiston"),
    ("vai", "ꕙꔤ, Vaii, ꕞꔤꔫꕩ"),
    ("vai-Latn", "Vai, Latn, Laibhiya"),
    ("vai-Vaii", "ꕙꔤ, Vaii, ꕞꔤꔫꕩ"),
    ("vi", "Tiếng Việt, Chữ La tinh, Việt Nam"),
    ("vun", "Kyivunjo, Latn, Tanzania"),
    ("wae", "Walser, Latiniš, Schwiz"),
    ("wo", "Wolof, Latin, Senegaal"),
    ("xh", "IsiXhosa, IsiLatin, EMzantsi Afrika"),
    ("xog", "Olusoga, Latn, Yuganda"),
    ("yav", "nuasue, Latn, Kemelún"),
    ("yi", "ייִדיש, העברעיש, וועלט"),
    ("yo", "Èdè Yorùbá, Èdè Látìn, Nàìjíríà"),
    ("yo-BJ", "Èdè Yorùbá, Èdè Látìn, Bɛ̀nɛ̀"),
    ("yrl", "nheẽgatu, ratĩ, Brasiu"),
    ("yrl-CO", "ñengatú, ratĩ, Kurũbiya"),
    ("yrl-VE", "ñengatú, ratĩ, Wenesuera"),
    ("yue", "粵語, 繁體, 中華人民共和國香港特別行政區"),
    ("yue-Hans", "粤语, 简体, 中华人民共和国"),
    ("yue-Hant", "粵語, 繁體, 中華人民共和國香港特別行政區"),
    ("zgh", "ⵜⴰⵎⴰⵣⵉⵖⵜ, Tfng, ⵍⵎⵖⵔⵉⴱ"),
    ("zh", "简体中文, 中国"),
    ("zh-Hans", "简体中文, 中国"),
    ("zh-Hans-HK", "简体中文, 中国香港特别行政区"),
    ("zh-Hans-MO", "简体中文, 中国澳门特别行政区"),
    ("zh-Hans-SG", "华文, 新加坡"),
    ("zh-Hant", "正體中文, 中國台灣"),
    ("zh-Hant-HK", "繁體中文, 中國香港"),
    ("zh-Hant-MO", "繁體中文, 中國澳門"),
    ("zh-pinyin", "HànYǔ PīnYīn, ZhōngGuó"),
    ("zu", "isiZulu, isi-Latin, iNingizimu Afrika"),
    // ("st", "st-Latn-ZA"),
    // ("la", "la-Latn-VA"),
    // ("ny", "ny-Latn-MW"),
    // ("sm", "sm-Latn-WS"),
    // ("jw", "jw-Latn-ID"),
    // ("ht", "ht-Latn-HT"),
    // ("co", "co-Latn-FR"),
    // ("tl", "tl-Latn-PH"),
    // ("iw", "iw-Hebr-IL"),
  ];

  let mod_name = "description";

  let raw_doc = r###"
  This function returns a `PhfMap` containing language codes as keys and their
corresponding descriptions as values.

## Example

```
use lang_id::maps::description;

let map = description::map();
assert_eq!(map["en-001"], "English, Latin, world");
```
  "###;

  builder::MapBuilder::<()> {
    mod_name,
    raw_doc,
    map_type: MapType::Ordered,
    str_kv,
    ..Default::default()
  }
  .build()
}

#[cfg(feature = "map")]
#[test]
fn test_español_desc() {
  let desc = lang_id::maps::description::map();
  let value = desc.get("es");

  if let Some(&v) = value {
    assert_eq!(v, "español, latino, España")
  }
}
