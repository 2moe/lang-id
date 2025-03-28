/// This function returns a `PhfMap` containing language codes as keys and their
/// corresponding descriptions as values.
///
/// ## Example
///
/// ```
/// use lang_id::maps::description;
///
/// let map = description::map();
/// assert_eq!(map["en-001"], "English, Latin, world");
/// ```
pub const fn map<'m>() -> super::PhfOrderedMap<'m> {
  super::phf::OrderedMap {
    key: 12913932095322966823,
    disps: &[
      (3, 108),
      (0, 488),
      (0, 10),
      (0, 34),
      (0, 4),
      (2, 205),
      (0, 33),
      (0, 130),
      (0, 4),
      (0, 37),
      (0, 69),
      (0, 5),
      (0, 1),
      (2, 537),
      (0, 0),
      (1, 167),
      (1, 369),
      (0, 19),
      (0, 77),
      (0, 27),
      (0, 23),
      (0, 19),
      (0, 51),
      (0, 0),
      (0, 180),
      (0, 5),
      (0, 18),
      (0, 279),
      (0, 1),
      (0, 6),
      (0, 44),
      (0, 250),
      (0, 13),
      (0, 0),
      (0, 68),
      (0, 10),
      (0, 2),
      (0, 42),
      (0, 25),
      (0, 1),
      (0, 33),
      (0, 71),
      (0, 0),
      (0, 382),
      (0, 0),
      (0, 172),
      (1, 47),
      (0, 12),
      (0, 0),
      (0, 253),
      (0, 159),
      (0, 0),
      (10, 361),
      (0, 25),
      (0, 42),
      (0, 1),
      (0, 81),
      (4, 331),
      (5, 372),
      (0, 35),
      (0, 20),
      (0, 387),
      (0, 68),
      (0, 62),
      (0, 8),
      (0, 1),
      (0, 148),
      (1, 104),
      (0, 83),
      (0, 244),
      (10, 320),
      (0, 10),
      (1, 222),
      (0, 83),
      (1, 43),
      (0, 6),
      (0, 0),
      (0, 38),
      (0, 174),
      (0, 5),
      (6, 164),
      (0, 1),
      (0, 64),
      (0, 31),
      (0, 57),
      (5, 366),
      (0, 24),
      (11, 124),
      (0, 470),
      (17, 410),
      (2, 314),
      (0, 1),
      (0, 322),
      (3, 394),
      (0, 0),
      (0, 56),
      (0, 178),
      (1, 416),
      (3, 510),
      (0, 521),
      (0, 9),
      (0, 23),
      (69, 290),
      (1, 498),
      (36, 67),
      (0, 557),
      (0, 511),
      (0, 0),
      (0, 206),
      (0, 452),
      (0, 309),
      (1, 356),
      (0, 71),
      (0, 64),
      (4, 253),
      (1, 59),
      (2, 10),
      (0, 4),
    ],
    idxs: &[
      320, 373, 28, 57, 12, 248, 289, 389, 102, 359, 126, 381, 65, 165, 41, 94, 580,
      219, 203, 413, 235, 582, 319, 507, 371, 51, 330, 378, 13, 151, 75, 27, 569,
      150, 538, 31, 99, 71, 24, 382, 179, 148, 369, 449, 180, 160, 295, 462, 273,
      465, 127, 362, 347, 464, 395, 234, 520, 53, 302, 442, 548, 181, 567, 152, 109,
      267, 448, 490, 9, 122, 379, 200, 174, 376, 247, 74, 138, 128, 552, 18, 92,
      229, 161, 354, 48, 62, 346, 385, 351, 456, 495, 158, 281, 251, 162, 198, 294,
      479, 154, 129, 40, 60, 192, 434, 334, 101, 416, 438, 45, 232, 170, 116, 585,
      211, 445, 473, 212, 564, 331, 576, 418, 282, 414, 261, 6, 88, 440, 558, 290,
      286, 545, 206, 292, 172, 393, 207, 332, 80, 187, 274, 147, 497, 562, 108, 225,
      322, 529, 52, 83, 422, 278, 559, 266, 337, 93, 431, 303, 213, 491, 485, 310,
      106, 502, 358, 264, 307, 584, 553, 249, 195, 279, 535, 329, 21, 46, 276, 565,
      446, 63, 547, 367, 2, 257, 426, 59, 201, 280, 270, 183, 493, 579, 476, 540,
      586, 72, 318, 242, 95, 410, 61, 25, 196, 120, 245, 43, 113, 452, 315, 577,
      244, 140, 499, 572, 399, 349, 166, 494, 316, 163, 397, 132, 215, 54, 167, 164,
      197, 78, 429, 237, 210, 384, 309, 145, 156, 472, 380, 409, 481, 169, 510, 90,
      91, 458, 406, 117, 8, 350, 372, 468, 370, 223, 519, 262, 107, 432, 486, 299,
      47, 190, 425, 441, 5, 326, 551, 246, 522, 325, 100, 526, 532, 541, 182, 321,
      288, 98, 514, 275, 506, 417, 272, 35, 39, 386, 411, 220, 38, 581, 79, 284,
      561, 352, 142, 216, 355, 308, 360, 84, 466, 218, 508, 436, 176, 70, 407, 199,
      365, 141, 130, 228, 521, 87, 364, 250, 459, 450, 7, 224, 470, 175, 336, 14,
      297, 49, 527, 492, 259, 342, 19, 205, 396, 554, 230, 184, 33, 474, 368, 67,
      496, 453, 233, 423, 188, 430, 168, 252, 217, 202, 221, 209, 254, 483, 437,
      111, 143, 444, 455, 348, 285, 461, 557, 81, 287, 323, 313, 23, 296, 505, 549,
      374, 428, 375, 194, 15, 563, 338, 583, 544, 536, 528, 487, 112, 317, 178, 96,
      419, 447, 340, 467, 311, 256, 304, 402, 556, 339, 11, 85, 241, 550, 277, 269,
      478, 574, 268, 504, 265, 86, 10, 204, 377, 484, 26, 50, 400, 0, 134, 533, 398,
      139, 537, 56, 344, 357, 401, 560, 525, 543, 133, 333, 171, 240, 335, 189, 159,
      345, 4, 566, 135, 153, 451, 66, 469, 408, 17, 471, 73, 131, 480, 555, 328,
      587, 121, 421, 366, 394, 137, 149, 518, 30, 1, 243, 68, 512, 64, 420, 103,
      214, 571, 511, 173, 97, 125, 305, 185, 513, 509, 37, 341, 55, 271, 343, 186,
      542, 501, 433, 44, 124, 110, 475, 231, 191, 227, 531, 539, 327, 76, 383, 32,
      404, 457, 16, 136, 488, 443, 482, 298, 22, 238, 424, 114, 403, 435, 361, 392,
      104, 573, 300, 324, 412, 77, 58, 258, 477, 356, 570, 42, 306, 157, 415, 387,
      208, 34, 89, 263, 460, 523, 144, 177, 236, 498, 155, 3, 115, 146, 314, 20,
      118, 119, 524, 283, 388, 405, 253, 534, 517, 69, 239, 489, 293, 439, 353, 255,
      575, 515, 391, 193, 427, 578, 500, 36, 291, 226, 260, 312, 454, 363, 82, 123,
      105, 530, 463, 503, 301, 516, 222, 390, 568, 29, 546,
    ],
    entries: &[
      ("af", r###"Afrikaans, Latyn, Suid-Afrika"###),
      ("af-NA", r###"Afrikaans, Latyn, Namibië"###),
      ("agq", r###"Aghem, Latn, Kàmàlûŋ"###),
      ("ak", r###"Akan, Latn, Gaana"###),
      ("am", r###"አማርኛ, ኢትዮፒክ, ኢትዮጵያ"###),
      ("ann", r###"Obolo, Latn, NG"###),
      ("ar", r###"العربية, العربية, مصر"###),
      ("ar-AE", r###"العربية, العربية, الإمارات العربية المتحدة"###),
      ("ar-BH", r###"العربية, العربية, البحرين"###),
      ("ar-DJ", r###"العربية, العربية, جيبوتي"###),
      ("ar-DZ", r###"العربية, العربية, الجزائر"###),
      ("ar-EG", r###"العربية, العربية, مصر"###),
      ("ar-EH", r###"العربية, العربية, الصحراء الغربية"###),
      ("ar-ER", r###"العربية, العربية, إريتريا"###),
      ("ar-IL", r###"العربية, العربية, إسرائيل"###),
      ("ar-IQ", r###"العربية, العربية, العراق"###),
      ("ar-JO", r###"العربية, العربية, الأردن"###),
      ("ar-KM", r###"العربية, العربية, جزر القمر"###),
      ("ar-KW", r###"العربية, العربية, الكويت"###),
      ("ar-LB", r###"العربية, العربية, لبنان"###),
      ("ar-LY", r###"العربية, العربية, ليبيا"###),
      ("ar-MA", r###"العربية, العربية, المغرب"###),
      ("ar-MR", r###"العربية, العربية, موريتانيا"###),
      ("ar-OM", r###"العربية, العربية, عُمان"###),
      ("ar-PS", r###"العربية, العربية, الأراضي الفلسطينية"###),
      ("ar-QA", r###"العربية, العربية, قطر"###),
      ("ar-SA", r###"العربية, العربية, المملكة العربية السعودية"###),
      ("ar-SD", r###"العربية, العربية, السودان"###),
      ("ar-SO", r###"العربية, العربية, الصومال"###),
      ("ar-SS", r###"العربية, العربية, جنوب السودان"###),
      ("ar-SY", r###"العربية, العربية, سوريا"###),
      ("ar-TD", r###"العربية, العربية, تشاد"###),
      ("ar-TN", r###"العربية, العربية, تونس"###),
      ("ar-YE", r###"العربية, العربية, اليمن"###),
      ("as", r###"অসমীয়া, বাংলা, ভাৰত"###),
      ("asa", r###"Kipare, Latn, Tadhania"###),
      ("ast", r###"asturianu, llatín, España"###),
      ("az", r###"azərbaycan, latın, Azərbaycan"###),
      ("az-Cyrl", r###"азәрбајҹан, Кирил, Азәрбајҹан"###),
      ("az-Latn", r###"azərbaycan, latın, Azərbaycan"###),
      ("bas", r###"Ɓàsàa, Latn, Kàmɛ̀rûn"###),
      ("be", r###"беларуская, кірыліца, Беларусь"###),
      ("bem", r###"Ichibemba, Latn, Zambia"###),
      ("bez", r###"Hibena, Latn, Hutanzania"###),
      ("bg", r###"български, кирилица, България"###),
      ("bgc", r###"हरियाणवी, देवानागारी, भारत"###),
      ("bho", r###"भोजपुरी, देवानागारी, भारत"###),
      ("bm", r###"bamanakan, Latn, Mali"###),
      ("bn", r###"বাংলা, বাংলা, বাংলাদেশ"###),
      ("bn-IN", r###"বাংলা, বাংলা, ভারত"###),
      ("bo", r###"བོད་སྐད་, བོད་ཡིག་, རྒྱ་ནག"###),
      ("bo-IN", r###"བོད་སྐད་, བོད་ཡིག་, རྒྱ་གར་"###),
      ("br", r###"brezhoneg, latin, Frañs"###),
      ("brx", r###"बर’, देबनागिरि, भारत"###),
      ("bs", r###"bosanski, latinica, Bosna i Hercegovina"###),
      ("bs-Cyrl", r###"босански, ћирилица, Босна и Херцеговина"###),
      ("bs-Latn", r###"bosanski, latinica, Bosna i Hercegovina"###),
      ("ca", r###"català, llatí, Espanya"###),
      ("ca-AD", r###"català, llatí, Andorra"###),
      ("ca-FR", r###"català, llatí, França"###),
      ("ca-IT", r###"català, llatí, Itàlia"###),
      ("ccp", r###"𑄌𑄋𑄴𑄟𑄳𑄦, 𑄌𑄇𑄴𑄟, 𑄝𑄁𑄣𑄘𑄬𑄌𑄴"###),
      ("ccp-IN", r###"𑄌𑄋𑄴𑄟𑄳𑄦, 𑄌𑄇𑄴𑄟, 𑄞𑄢𑄧𑄖𑄴"###),
      ("ce", r###"нохчийн, кириллица, Росси"###),
      ("ceb", r###"Cebuano, Latin, Pilipinas"###),
      ("cgg", r###"Rukiga, Latn, Uganda"###),
      ("chr", r###"ᏣᎳᎩ, ᏣᎳᎩ, ᏌᏊ ᎢᏳᎾᎵᏍᏔᏅ ᏍᎦᏚᎩ"###),
      ("ckb", r###"کوردیی ناوەندی, عەرەبی, عێراق"###),
      ("ckb-IR", r###"کوردیی ناوەندی, عەرەبی, ئێران"###),
      ("cs", r###"čeština, latinka, Česko"###),
      ("cv", r###"чӑваш, кириллица, Раҫҫей"###),
      ("cy", r###"Cymraeg, Lladin, Y Deyrnas Unedig"###),
      ("da", r###"dansk, latinsk, Danmark"###),
      ("da-GL", r###"dansk, latinsk, Grønland"###),
      ("dav", r###"Kitaita, Latn, Kenya"###),
      ("de", r###"Deutsch, Lateinisch, Deutschland"###),
      ("de-AT", r###"Deutsch, Lateinisch, Österreich"###),
      ("de-BE", r###"Deutsch, Lateinisch, Belgien"###),
      ("de-CH", r###"Deutsch, Lateinisch, Schweiz"###),
      ("de-IT", r###"Deutsch, Lateinisch, Italien"###),
      ("de-LI", r###"Deutsch, Lateinisch, Liechtenstein"###),
      ("de-LU", r###"Deutsch, Lateinisch, Luxemburg"###),
      ("dje", r###"Zarmaciine, Latn, Nižer"###),
      ("doi", r###"डोगरी, देवनागरी, भारत"###),
      ("dsb", r###"dolnoserbšćina, łatyński, Nimska"###),
      ("dua", r###"duálá, Latn, Cameroun"###),
      ("dyo", r###"joola, Latn, Senegal"###),
      ("dz", r###"རྫོང་ཁ, ང་བཅས་ཀྱི་ཡིག་གུ, འབྲུག"###),
      ("ebu", r###"Kĩembu, Latn, Kenya"###),
      ("ee", r###"Eʋegbe, Latingbeŋɔŋlɔ, Ghana nutome"###),
      ("ee-TG", r###"Eʋegbe, Latingbeŋɔŋlɔ, Togo nutome"###),
      ("el", r###"Ελληνικά, Ελληνικό, Ελλάδα"###),
      ("el-CY", r###"Ελληνικά, Ελληνικό, Κύπρος"###),
      ("en", r###"English, Latin, United States"###),
      ("en-001", r###"English, Latin, world"###),
      ("en-150", r###"English, Latin, Europe"###),
      ("en-AE", r###"English, Latin, United Arab Emirates"###),
      ("en-AG", r###"English, Latin, Antigua & Barbuda"###),
      ("en-AI", r###"English, Latin, Anguilla"###),
      ("en-AS", r###"English, Latin, American Samoa"###),
      ("en-AT", r###"English, Latin, Austria"###),
      ("en-AU", r###"English, Latin, Australia"###),
      ("en-BB", r###"English, Latin, Barbados"###),
      ("en-BE", r###"English, Latin, Belgium"###),
      ("en-BI", r###"English, Latin, Burundi"###),
      ("en-BM", r###"English, Latin, Bermuda"###),
      ("en-BS", r###"English, Latin, Bahamas"###),
      ("en-BW", r###"English, Latin, Botswana"###),
      ("en-BZ", r###"English, Latin, Belize"###),
      ("en-CA", r###"English, Latin, Canada"###),
      ("en-CC", r###"English, Latin, Cocos (Keeling) Islands"###),
      ("en-CH", r###"English, Latin, Switzerland"###),
      ("en-CK", r###"English, Latin, Cook Islands"###),
      ("en-CM", r###"English, Latin, Cameroon"###),
      ("en-CX", r###"English, Latin, Christmas Island"###),
      ("en-CY", r###"English, Latin, Cyprus"###),
      ("en-DE", r###"English, Latin, Germany"###),
      ("en-DG", r###"English, Latin, Diego Garcia"###),
      ("en-DK", r###"English, Latin, Denmark"###),
      ("en-DM", r###"English, Latin, Dominica"###),
      ("en-ER", r###"English, Latin, Eritrea"###),
      ("en-FI", r###"English, Latin, Finland"###),
      ("en-FJ", r###"English, Latin, Fiji"###),
      ("en-FK", r###"English, Latin, Falkland Islands"###),
      ("en-FM", r###"English, Latin, Micronesia"###),
      ("en-GB", r###"English, Latin, United Kingdom"###),
      ("en-GD", r###"English, Latin, Grenada"###),
      ("en-GG", r###"English, Latin, Guernsey"###),
      ("en-GH", r###"English, Latin, Ghana"###),
      ("en-GI", r###"English, Latin, Gibraltar"###),
      ("en-GM", r###"English, Latin, Gambia"###),
      ("en-GU", r###"English, Latin, Guam"###),
      ("en-GY", r###"English, Latin, Guyana"###),
      ("en-HK", r###"English, Latin, Hong Kong SAR China"###),
      ("en-IE", r###"English, Latin, Ireland"###),
      ("en-IL", r###"English, Latin, Israel"###),
      ("en-IM", r###"English, Latin, Isle of Man"###),
      ("en-IN", r###"English, Latin, India"###),
      (
        "en-IO",
        r###"English, Latin, British Indian Ocean Territory"###,
      ),
      ("en-JE", r###"English, Latin, Jersey"###),
      ("en-JM", r###"English, Latin, Jamaica"###),
      ("en-KE", r###"English, Latin, Kenya"###),
      ("en-KI", r###"English, Latin, Kiribati"###),
      ("en-KN", r###"English, Latin, St Kitts & Nevis"###),
      ("en-KY", r###"English, Latin, Cayman Islands"###),
      ("en-LC", r###"English, Latin, St Lucia"###),
      ("en-LR", r###"English, Latin, Liberia"###),
      ("en-LS", r###"English, Latin, Lesotho"###),
      ("en-MG", r###"English, Latin, Madagascar"###),
      ("en-MH", r###"English, Latin, Marshall Islands"###),
      ("en-MO", r###"English, Latin, Macao SAR China"###),
      ("en-MP", r###"English, Latin, Northern Mariana Islands"###),
      ("en-MS", r###"English, Latin, Montserrat"###),
      ("en-MT", r###"English, Latin, Malta"###),
      ("en-MU", r###"English, Latin, Mauritius"###),
      ("en-MV", r###"English, Latin, Maldives"###),
      ("en-MW", r###"English, Latin, Malawi"###),
      ("en-MY", r###"English, Latin, Malaysia"###),
      ("en-NA", r###"English, Latin, Namibia"###),
      ("en-NF", r###"English, Latin, Norfolk Island"###),
      ("en-NG", r###"English, Latin, Nigeria"###),
      ("en-NL", r###"English, Latin, Netherlands"###),
      ("en-NR", r###"English, Latin, Nauru"###),
      ("en-NU", r###"English, Latin, Niue"###),
      ("en-NZ", r###"English, Latin, New Zealand"###),
      ("en-PG", r###"English, Latin, Papua New Guinea"###),
      ("en-PH", r###"English, Latin, Philippines"###),
      ("en-PK", r###"English, Latin, Pakistan"###),
      ("en-PN", r###"English, Latin, Pitcairn Islands"###),
      ("en-PR", r###"English, Latin, Puerto Rico"###),
      ("en-PW", r###"English, Latin, Palau"###),
      ("en-RW", r###"English, Latin, Rwanda"###),
      ("en-SB", r###"English, Latin, Solomon Islands"###),
      ("en-SC", r###"English, Latin, Seychelles"###),
      ("en-SD", r###"English, Latin, Sudan"###),
      ("en-SE", r###"English, Latin, Sweden"###),
      ("en-SG", r###"English, Latin, Singapore"###),
      ("en-SH", r###"English, Latin, St Helena"###),
      ("en-SI", r###"English, Latin, Slovenia"###),
      ("en-SL", r###"English, Latin, Sierra Leone"###),
      ("en-SS", r###"English, Latin, South Sudan"###),
      ("en-SX", r###"English, Latin, Sint Maarten"###),
      ("en-SZ", r###"English, Latin, Eswatini"###),
      ("en-TC", r###"English, Latin, Turks & Caicos Islands"###),
      ("en-TK", r###"English, Latin, Tokelau"###),
      ("en-TO", r###"English, Latin, Tonga"###),
      ("en-TT", r###"English, Latin, Trinidad & Tobago"###),
      ("en-TV", r###"English, Latin, Tuvalu"###),
      ("en-TZ", r###"English, Latin, Tanzania"###),
      ("en-UG", r###"English, Latin, Uganda"###),
      ("en-UM", r###"English, Latin, U.S. Outlying Islands"###),
      (
        "en-VC",
        r###"English, Latin, St Vincent & the Grenadines"###,
      ),
      ("en-VG", r###"English, Latin, British Virgin Islands"###),
      ("en-VI", r###"English, Latin, U.S. Virgin Islands"###),
      ("en-VU", r###"English, Latin, Vanuatu"###),
      ("en-WS", r###"English, Latin, Samoa"###),
      ("en-ZA", r###"English, Latin, South Africa"###),
      ("en-ZM", r###"English, Latin, Zambia"###),
      ("en-ZW", r###"English, Latin, Zimbabwe"###),
      ("eo", r###"esperanto, Latn, Mondo"###),
      ("es", r###"español, latino, España"###),
      ("es-419", r###"español, latín, Latinoamérica"###),
      ("es-AR", r###"español, latín, Argentina"###),
      ("es-BO", r###"español, latín, Bolivia"###),
      ("es-BR", r###"español, latín, Brasil"###),
      ("es-BZ", r###"español, latín, Belice"###),
      ("es-CL", r###"español, latín, Chile"###),
      ("es-CO", r###"español, latín, Colombia"###),
      ("es-CR", r###"español, latín, Costa Rica"###),
      ("es-CU", r###"español, latín, Cuba"###),
      ("es-DO", r###"español, latín, República Dominicana"###),
      ("es-EA", r###"español, latino, Ceuta y Melilla"###),
      ("es-EC", r###"español, latín, Ecuador"###),
      ("es-GQ", r###"español, latino, Guinea Ecuatorial"###),
      ("es-GT", r###"español, latín, Guatemala"###),
      ("es-HN", r###"español, latín, Honduras"###),
      ("es-IC", r###"español, latino, Canarias"###),
      ("es-MX", r###"español, latín, México"###),
      ("es-NI", r###"español, latín, Nicaragua"###),
      ("es-PA", r###"español, latín, Panamá"###),
      ("es-PE", r###"español, latín, Perú"###),
      ("es-PH", r###"español, latino, Filipinas"###),
      ("es-PR", r###"español, latín, Puerto Rico"###),
      ("es-PY", r###"español, latín, Paraguay"###),
      ("es-SV", r###"español, latín, El Salvador"###),
      ("es-US", r###"español, latín, Estados Unidos"###),
      ("es-UY", r###"español, latín, Uruguay"###),
      ("es-VE", r###"español, latín, Venezuela"###),
      ("et", r###"eesti, ladina, Eesti"###),
      ("eu", r###"euskara, latinoa, Espainia"###),
      ("ewo", r###"ewondo, Latn, Kamərún"###),
      ("fa", r###"فارسی, عربی, ایران"###),
      ("fa-AF", r###"فارسی, عربی, افغانستان"###),
      ("ff", r###"Pulaar, Latn, Senegaal"###),
      ("ff-Adlm", r###"𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤘𞤭𞤲𞤫"###),
      ("ff-Adlm-BF", r###"𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤄𞤵𞤪𞤳𞤭𞤲𞤢 𞤊𞤢𞤧𞤮𞥅"###),
      ("ff-Adlm-CM", r###"𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤑𞤢𞤥𞤢𞤪𞤵𞥅𞤲"###),
      ("ff-Adlm-GH", r###"𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤘𞤢𞤲𞤢"###),
      ("ff-Adlm-GM", r###"𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤘𞤢𞤥𞤦𞤭𞤴𞤢"###),
      ("ff-Adlm-GW", r###"𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤘𞤭𞤲𞤫-𞤄𞤭𞤧𞤢𞤱𞤮𞥅"###),
      ("ff-Adlm-LR", r###"𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤂𞤢𞤦𞤭𞤪𞤭𞤴𞤢𞥄"###),
      ("ff-Adlm-MR", r###"𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤃𞤮𞤪𞤼𞤢𞤲𞤭𞥅"###),
      ("ff-Adlm-NE", r###"𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤐𞤭𞥅𞤶𞤫𞤪"###),
      ("ff-Adlm-NG", r###"𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤐𞤢𞤶𞤫𞤪𞤭𞤴𞤢𞥄"###),
      ("ff-Adlm-SL", r###"𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤅𞤢𞤪𞤢𞤤𞤮𞤲"###),
      ("ff-Adlm-SN", r###"𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤅𞤫𞤲𞤫𞤺𞤢𞥄𞤤"###),
      ("ff-Latn", r###"Pulaar, Latn, Senegaal"###),
      ("ff-Latn-BF", r###"Pulaar, Latn, Burkibaa Faaso"###),
      ("ff-Latn-CM", r###"Pulaar, Latn, Kameruun"###),
      ("ff-Latn-GH", r###"Pulaar, Latn, Ganaa"###),
      ("ff-Latn-GM", r###"Pulaar, Latn, Gammbi"###),
      ("ff-Latn-GN", r###"Pulaar, Latn, Gine"###),
      ("ff-Latn-GW", r###"Pulaar, Latn, Gine-Bisaawo"###),
      ("ff-Latn-LR", r###"Pulaar, Latn, Liberiyaa"###),
      ("ff-Latn-MR", r###"Pulaar, Latn, Muritani"###),
      ("ff-Latn-NE", r###"Pulaar, Latn, Nijeer"###),
      ("ff-Latn-NG", r###"Pulaar, Latn, Nijeriyaa"###),
      ("ff-Latn-SL", r###"Pulaar, Latn, Seraa liyon"###),
      ("fi", r###"suomi, latinalainen, Suomi"###),
      ("fil", r###"Filipino, Latin, Pilipinas"###),
      ("fo", r###"føroyskt, latínskt, Føroyar"###),
      ("fo-DK", r###"føroyskt, latínskt, Danmark"###),
      ("fr", r###"français, latin, France"###),
      ("fr-BE", r###"français, latin, Belgique"###),
      ("fr-BF", r###"français, latin, Burkina Faso"###),
      ("fr-BI", r###"français, latin, Burundi"###),
      ("fr-BJ", r###"français, latin, Bénin"###),
      ("fr-BL", r###"français, latin, Saint-Barthélemy"###),
      ("fr-CA", r###"français, latin, Canada"###),
      ("fr-CD", r###"français, latin, Congo-Kinshasa"###),
      ("fr-CF", r###"français, latin, République centrafricaine"###),
      ("fr-CG", r###"français, latin, Congo-Brazzaville"###),
      ("fr-CH", r###"français, latin, Suisse"###),
      ("fr-CI", r###"français, latin, Côte d’Ivoire"###),
      ("fr-CM", r###"français, latin, Cameroun"###),
      ("fr-DJ", r###"français, latin, Djibouti"###),
      ("fr-DZ", r###"français, latin, Algérie"###),
      ("fr-GA", r###"français, latin, Gabon"###),
      ("fr-GF", r###"français, latin, Guyane française"###),
      ("fr-GN", r###"français, latin, Guinée"###),
      ("fr-GP", r###"français, latin, Guadeloupe"###),
      ("fr-GQ", r###"français, latin, Guinée équatoriale"###),
      ("fr-HT", r###"français, latin, Haïti"###),
      ("fr-KM", r###"français, latin, Comores"###),
      ("fr-LU", r###"français, latin, Luxembourg"###),
      ("fr-MA", r###"français, latin, Maroc"###),
      ("fr-MC", r###"français, latin, Monaco"###),
      ("fr-MF", r###"français, latin, Saint-Martin"###),
      ("fr-MG", r###"français, latin, Madagascar"###),
      ("fr-ML", r###"français, latin, Mali"###),
      ("fr-MQ", r###"français, latin, Martinique"###),
      ("fr-MR", r###"français, latin, Mauritanie"###),
      ("fr-MU", r###"français, latin, Maurice"###),
      ("fr-NC", r###"français, latin, Nouvelle-Calédonie"###),
      ("fr-NE", r###"français, latin, Niger"###),
      ("fr-PF", r###"français, latin, Polynésie française"###),
      ("fr-PM", r###"français, latin, Saint-Pierre-et-Miquelon"###),
      ("fr-RE", r###"français, latin, La Réunion"###),
      ("fr-RW", r###"français, latin, Rwanda"###),
      ("fr-SC", r###"français, latin, Seychelles"###),
      ("fr-SN", r###"français, latin, Sénégal"###),
      ("fr-SY", r###"français, latin, Syrie"###),
      ("fr-TD", r###"français, latin, Tchad"###),
      ("fr-TG", r###"français, latin, Togo"###),
      ("fr-TN", r###"français, latin, Tunisie"###),
      ("fr-VU", r###"français, latin, Vanuatu"###),
      ("fr-WF", r###"français, latin, Wallis-et-Futuna"###),
      ("fr-YT", r###"français, latin, Mayotte"###),
      ("frr", r###"Nordfriisk, Latn, DE"###),
      ("fur", r###"furlan, latin, Italie"###),
      ("fy", r###"Frysk, Latyn, Nederlân"###),
      ("ga", r###"Gaeilge, Laidineach, Éire"###),
      ("ga-GB", r###"Gaeilge, Laidineach, an Ríocht Aontaithe"###),
      ("gd", r###"Gàidhlig, Laideann, An Rìoghachd Aonaichte"###),
      ("gl", r###"galego, latino, España"###),
      ("gsw", r###"Schwiizertüütsch, Latiinisch, Schwiiz"###),
      ("gsw-FR", r###"Schwiizertüütsch, Latiinisch, Frankriich"###),
      (
        "gsw-LI",
        r###"Schwiizertüütsch, Latiinisch, Liächteschtäi"###,
      ),
      ("gu", r###"ગુજરાતી, ગુજરાતી, ભારત"###),
      ("guz", r###"Ekegusii, Latn, Kenya"###),
      ("gv", r###"Gaelg, Latn, Ellan Vannin"###),
      ("ha", r###"Hausa, Latin, Nijeriya"###),
      ("ha-GH", r###"Hausa, Latin, Gana"###),
      ("ha-NE", r###"Hausa, Latin, Nijar"###),
      ("haw", r###"ʻŌlelo Hawaiʻi, Latn, ʻAmelika Hui Pū ʻIa"###),
      ("he", r###"עברית, עברי, ישראל"###),
      ("hi", r###"हिन्दी, देवनागरी, भारत"###),
      ("hi-Latn", r###"Hindi (Latin), India"###),
      ("hr", r###"hrvatski, latinica, Hrvatska"###),
      ("hr-BA", r###"hrvatski, latinica, Bosna i Hercegovina"###),
      ("hsb", r###"hornjoserbšćina, łaćonsce, Němska"###),
      ("hu", r###"magyar, Latin, Magyarország"###),
      ("hy", r###"հայերեն, հայկական, Հայաստան"###),
      ("ia", r###"interlingua, latin, Mundo"###),
      ("id", r###"Indonesia, Latin, Indonesia"###),
      ("ig", r###"Igbo, Latin, Naịjịrịa"###),
      ("ii", r###"ꆈꌠꉙ, ꆈꌠꁱꂷ, ꍏꇩ"###),
      ("is", r###"íslenska, latneskt, Ísland"###),
      ("it", r###"italiano, latino, Italia"###),
      ("it-CH", r###"italiano, latino, Svizzera"###),
      ("it-SM", r###"italiano, latino, San Marino"###),
      ("it-VA", r###"italiano, latino, Città del Vaticano"###),
      ("ja", r###"日本語, 日本語の文字, 日本"###),
      ("ja-romaji", r###"Nihongo, Hepburn-shiki Rōmaji"###),
      (
        "jgo",
        r###"Ndaꞌa, mík -ŋwaꞌnɛ yi ɛ́ líŋɛ́nɛ Latɛ̂ŋ, Kamɛlûn"###,
      ),
      ("jmc", r###"Kimachame, Latn, Tanzania"###),
      ("jv", r###"Jawa, Latin, Indonésia"###),
      ("ka", r###"ქართული, ქართული, საქართველო"###),
      ("kab", r###"Taqbaylit, Latn, Lezzayer"###),
      ("kam", r###"Kikamba, Latn, Kenya"###),
      ("kde", r###"Chimakonde, Latn, Tanzania"###),
      ("kea", r###"kabuverdianu, latinu, Kabu Verdi"###),
      ("kgp", r###"kanhgág, ratĩnh, Mrasir"###),
      ("khq", r###"Koyra ciini, Latn, Maali"###),
      ("ki", r###"Gikuyu, Latn, Kenya"###),
      ("kk", r###"қазақ тілі, кирилл жазуы, Қазақстан"###),
      ("kkj", r###"kakɔ, Latn, Kamɛrun"###),
      ("kl", r###"kalaallisut, Latn, Kalaallit Nunaat"###),
      ("kln", r###"Kalenjin, Latn, Emetab Kenya"###),
      ("km", r###"ខ្មែរ, ខ្មែរ, កម្ពុជា"###),
      ("kn", r###"ಕನ್ನಡ, ಕನ್ನಡ, ಭಾರತ"###),
      ("ko", r###"한국어, 한국 문자, 대한민국"###),
      ("ko-KP", r###"한국어, 한국 문자, 조선민주주의인민공화국"###),
      ("kok", r###"कोंकणी, देवनागरी, भारत"###),
      ("ks", r###"کٲشُر, عربی, ہِندوستان"###),
      ("ks-Arab", r###"کٲشُر, عربی, ہِندوستان"###),
      ("ks-Deva", r###"कॉशुर, देवनागरी, हिंदोस्तान"###),
      ("ksb", r###"Kishambaa, Latn, Tanzania"###),
      ("ksf", r###"rikpa, Latn, kamɛrún"###),
      ("ksh", r###"Kölsch, lateinesche Schreff, Doütschland"###),
      ("ku", r###"kurdî, latînî, Tirkiye"###),
      ("kw", r###"kernewek, Latn, Rywvaneth Unys"###),
      ("ky", r###"кыргызча, Кирилл, Кыргызстан"###),
      ("lag", r###"Kɨlaangi, Latn, Taansanía"###),
      ("lb", r###"Lëtzebuergesch, Laténgesch, Lëtzebuerg"###),
      ("lg", r###"Luganda, Latn, Yuganda"###),
      ("lkt", r###"Lakȟólʼiyapi, Latn, Mílahaŋska Tȟamákȟočhe"###),
      ("ln", r###"lingála, Latn, Republíki ya Kongó Demokratíki"###),
      ("ln-AO", r###"lingála, Latn, Angóla"###),
      ("ln-CF", r###"lingála, Latn, Repibiki ya Afríka ya Káti"###),
      ("ln-CG", r###"lingála, Latn, Kongo"###),
      ("lo", r###"ລາວ, ລາວ, ລາວ"###),
      ("lrc", r###"لۊری شومالی, عأرأڤی, IR"###),
      ("lrc-IQ", r###"لۊری شومالی, عأرأڤی, IQ"###),
      ("lt", r###"lietuvių, lotynų, Lietuva"###),
      ("lu", r###"Tshiluba, Latn, Ditunga wa Kongu"###),
      ("luo", r###"Dholuo, Latn, Kenya"###),
      ("luy", r###"Luluhia, Latn, Kenya"###),
      ("lv", r###"latviešu, latīņu, Latvija"###),
      ("lzh", r###"文言, 古典漢字, 古之華夏"###),
      ("lzh-Hans", r###"文言, 简体, 中国"###),
      ("mai", r###"मैथिली, देवनागरी, भारत"###),
      ("mas", r###"Maa, Latn, Kenya"###),
      ("mas-TZ", r###"Maa, Latn, Tansania"###),
      ("mdf", r###"мокшень кяль, Cyrl, RU"###),
      ("mer", r###"Kĩmĩrũ, Latn, Kenya"###),
      ("mfe", r###"kreol morisien, Latn, Moris"###),
      ("mg", r###"Malagasy, Latn, Madagasikara"###),
      ("mgh", r###"Makua, Latn, Umozambiki"###),
      ("mgo", r###"metaʼ, ngam ŋwaʼri, Kamalun"###),
      ("mi", r###"Māori, Rātina, Aotearoa"###),
      (
        "mk",
        r###"македонски, кирилско писмо, Северна Македонија"###,
      ),
      ("ml", r###"മലയാളം, മലയാളം, ഇന്ത്യ"###),
      ("mn", r###"монгол, кирилл, Монгол"###),
      ("mni", r###"মৈতৈলোন্, বাংলা, ইন্দিয়া"###),
      ("mni-Beng", r###"মৈতৈলোন্, বাংলা, ইন্দিয়া"###),
      ("mr", r###"मराठी, देवनागरी, भारत"###),
      ("ms", r###"Melayu, Latin, Malaysia"###),
      ("ms-BN", r###"Melayu, Latin, Brunei"###),
      ("ms-ID", r###"Melayu, Latin, Indonesia"###),
      ("ms-SG", r###"Melayu, Latin, Singapura"###),
      ("mt", r###"Malti, Latin, Malta"###),
      ("mua", r###"MUNDAŊ, Latn, kameruŋ"###),
      ("my", r###"မြန်မာ, မြန်မာ, မြန်မာ"###),
      ("mzn", r###"مازرونی, عربی, ایران"###),
      ("naq", r###"Khoekhoegowab, Latn, Namibiab"###),
      ("nb", r###"norsk bokmål, latinsk, Norge"###),
      (
        "nb-SJ",
        r###"norsk bokmål, latinsk, Svalbard og Jan Mayen"###,
      ),
      ("nd", r###"isiNdebele, Latn, Zimbabwe"###),
      ("nds", r###"Plattdüütsch, Latiensch Schrift, Düütschland"###),
      (
        "nds-NL",
        r###"Nedersaksisch, Latijns Schrift, Nederlaand"###,
      ),
      ("ne", r###"नेपाली, देवानागरी, नेपाल"###),
      ("ne-IN", r###"नेपाली, देवानागरी, भारत"###),
      ("nl", r###"Nederlands, Latijns, Nederland"###),
      ("nl-AW", r###"Nederlands, Latijns, Aruba"###),
      ("nl-BE", r###"Nederlands, Latijns, België"###),
      ("nl-BQ", r###"Nederlands, Latijns, Caribisch Nederland"###),
      ("nl-CW", r###"Nederlands, Latijns, Curaçao"###),
      ("nl-SR", r###"Nederlands, Latijns, Suriname"###),
      ("nl-SX", r###"Nederlands, Latijns, Sint-Maarten"###),
      ("nmg", r###"Nmgwa, Látín, Kamɛrun"###),
      ("nn", r###"norsk nynorsk, latinsk, Noreg"###),
      ("nnh", r###"Shwóŋò ngiembɔɔn, Latn, Kàmalûm"###),
      ("no", r###"norsk, latinsk, Norge"###),
      ("nus", r###"Thok Nath, Latn, SS"###),
      ("nyn", r###"Runyankore, Latn, Uganda"###),
      ("oc", r###"Occitan, Alfabet latin, França"###),
      ("oc-ES", r###"Aranés, Alfabet latin, Espanha"###),
      ("om", r###"Oromoo, Latin, Itoophiyaa"###),
      ("om-KE", r###"Oromoo, Latin, Keeniyaa"###),
      ("or", r###"ଓଡ଼ିଆ, ଓଡ଼ିଆ, ଭାରତ"###),
      ("os", r###"ирон, Киррилицӕ, Гуырдзыстон"###),
      ("os-RU", r###"ирон, Киррилицӕ, Уӕрӕсе"###),
      ("pa", r###"ਪੰਜਾਬੀ, ਗੁਰਮੁਖੀ, ਭਾਰਤ"###),
      ("pa-Arab", r###"پنجابی, عربی, پاکستان"###),
      ("pa-Guru", r###"ਪੰਜਾਬੀ, ਗੁਰਮੁਖੀ, ਭਾਰਤ"###),
      ("pcm", r###"Naijíriá Píjin, Látin, Naijíria"###),
      ("pis", r###"Pijin, Latin, Solomon Aelan"###),
      ("pl", r###"polski, łacińskie, Polska"###),
      ("ps", r###"پښتو, عربي, افغانستان"###),
      ("ps-PK", r###"پښتو, عربي, پاکستان"###),
      ("pt", r###"português, latim, Brasil"###),
      ("pt-AO", r###"português, latim, Angola"###),
      ("pt-CH", r###"português, latim, Suíça"###),
      ("pt-CV", r###"português, latim, Cabo Verde"###),
      ("pt-GQ", r###"português, latim, Guiné Equatorial"###),
      ("pt-GW", r###"português, latim, Guiné-Bissau"###),
      ("pt-LU", r###"português, latim, Luxemburgo"###),
      ("pt-MO", r###"português, latim, Macau, RAE da China"###),
      ("pt-MZ", r###"português, latim, Moçambique"###),
      ("pt-PT", r###"português, latim, Portugal"###),
      ("pt-ST", r###"português, latim, São Tomé e Príncipe"###),
      ("pt-TL", r###"português, latim, Timor-Leste"###),
      ("qu", r###"Runasimi, Latin Simi, Perú"###),
      ("qu-BO", r###"Runasimi, Latin Simi, Bolivia"###),
      ("qu-EC", r###"Runasimi, Latin Simi, Ecuador"###),
      ("raj", r###"राजस्थानी, देवनागरी, भारत"###),
      ("rm", r###"rumantsch, latin, Svizra"###),
      ("rn", r###"Ikirundi, Latn, Uburundi"###),
      ("ro", r###"română, latină, România"###),
      ("ro-MD", r###"română, latină, Republica Moldova"###),
      ("rof", r###"Kihorombo, Latn, Tanzania"###),
      ("ru", r###"русский, кириллица, Россия"###),
      ("ru-BY", r###"русский, кириллица, Беларусь"###),
      ("ru-KG", r###"русский, кириллица, Киргизия"###),
      ("ru-KZ", r###"русский, кириллица, Казахстан"###),
      ("ru-MD", r###"русский, кириллица, Молдова"###),
      ("ru-UA", r###"русский, кириллица, Украина"###),
      ("rw", r###"Kinyarwanda, Latn, U Rwanda"###),
      ("rwk", r###"Kiruwa, Latn, Tanzania"###),
      ("sa", r###"संस्कृतम्, देवनागरी, भारत"###),
      ("sah", r###"саха тыла, Нууччалыы, Арассыыйа"###),
      ("saq", r###"Kisampur, Latn, Kenya"###),
      ("sat", r###"ᱥᱟᱱᱛᱟᱲᱤ, ᱚᱞ ᱪᱤᱠᱤ, ᱤᱱᱰᱤᱭᱟ"###),
      ("sat-Olck", r###"ᱥᱟᱱᱛᱟᱲᱤ, ᱚᱞ ᱪᱤᱠᱤ, ᱤᱱᱰᱤᱭᱟ"###),
      ("sbp", r###"Ishisangu, Latn, Tansaniya"###),
      ("sc", r###"sardu, latinu, Itàlia"###),
      ("sd", r###"سنڌي, عربي, پاڪستان"###),
      ("sd-Arab", r###"سنڌي, عربي, پاڪستان"###),
      ("sd-Deva", r###"सिन्धी, देवनागिरी, भारत"###),
      ("se", r###"davvisámegiella, láhtenaš, Norga"###),
      ("se-FI", r###"davvisámegiella, láhtenaš, Suopma"###),
      ("se-SE", r###"davvisámegiella, láhtenaš, Ruoŧŧa"###),
      ("seh", r###"sena, Latn, Moçambique"###),
      ("ses", r###"Koyraboro senni, Latn, Maali"###),
      ("sg", r###"Sängö, Latn, Ködörösêse tî Bêafrîka"###),
      ("shi", r###"ⵜⴰⵛⵍⵃⵉⵜ, Tfng, ⵍⵎⵖⵔⵉⴱ"###),
      ("shi-Latn", r###"Tashelḥiyt, Latn, lmɣrib"###),
      ("shi-Tfng", r###"ⵜⴰⵛⵍⵃⵉⵜ, Tfng, ⵍⵎⵖⵔⵉⴱ"###),
      ("si", r###"සිංහල, සිංහල, ශ්‍රී ලංකාව"###),
      ("sk", r###"slovenčina, latinka, Slovensko"###),
      ("sl", r###"slovenščina, latinica, Slovenija"###),
      ("smn", r###"anarâškielâ, Latn, Suomâ"###),
      ("sms", r###"sms, Latn, FI"###),
      ("sn", r###"chiShona, Latn, Zimbabwe"###),
      ("so", r###"Soomaali, Laatiin, Soomaaliya"###),
      ("so-DJ", r###"Soomaali, Laatiin, Jabuuti"###),
      ("so-ET", r###"Soomaali, Laatiin, Itoobiya"###),
      ("so-KE", r###"Soomaali, Laatiin, Kenya"###),
      ("sq", r###"shqip, latin, Shqipëri"###),
      ("sq-MK", r###"shqip, latin, Maqedonia e Veriut"###),
      ("sq-XK", r###"shqip, latin, Kosovë"###),
      ("sr", r###"српски, ћирилица, Србија"###),
      ("sr-Cyrl", r###"српски, ћирилица, Србија"###),
      ("sr-Cyrl-BA", r###"српски, ћирилица, Босна и Херцеговина"###),
      ("sr-Cyrl-ME", r###"српски, ћирилица, Црна Гора"###),
      ("sr-Cyrl-XK", r###"српски, ћирилица, Косово"###),
      ("sr-Latn", r###"srpski, latinica, Srbija"###),
      ("sr-Latn-BA", r###"srpski, latinica, Bosna i Hercegovina"###),
      ("sr-Latn-ME", r###"srpski, latinica, Crna Gora"###),
      ("sr-Latn-XK", r###"srpski, latinica, Kosovo"###),
      ("su", r###"Basa Sunda, Latin, Indonesia"###),
      ("su-Latn", r###"Basa Sunda, Latin, Indonesia"###),
      ("sv", r###"svenska, latinska, Sverige"###),
      ("sv-AX", r###"svenska, latinska, Åland"###),
      ("sv-FI", r###"svenska, latinska, Finland"###),
      ("sw", r###"Kiswahili, Kilatini, Tanzania"###),
      (
        "sw-CD",
        r###"Kiswahili, Kilatini, Jamhuri ya Kidemokrasia ya Kongo"###,
      ),
      ("sw-KE", r###"Kiswahili, Kilatini, Kenya"###),
      ("sw-UG", r###"Kiswahili, Kilatini, Uganda"###),
      ("ta", r###"தமிழ், தமிழ், இந்தியா"###),
      ("ta-LK", r###"தமிழ், தமிழ், இலங்கை"###),
      ("ta-MY", r###"தமிழ், தமிழ், மலேசியா"###),
      ("ta-SG", r###"தமிழ், தமிழ், சிங்கப்பூர்"###),
      ("te", r###"తెలుగు, తెలుగు, భారతదేశం"###),
      ("teo", r###"Kiteso, Latn, Uganda"###),
      ("teo-KE", r###"Kiteso, Latn, Kenia"###),
      ("tg", r###"тоҷикӣ, Кириллӣ, Тоҷикистон"###),
      ("th", r###"ไทย, ไทย, ไทย"###),
      ("ti", r###"ትግርኛ, ፊደል, ኢትዮጵያ"###),
      ("ti-ER", r###"ትግርኛ, ፊደል, ኤርትራ"###),
      ("tk", r###"türkmen dili, Latyn elipbiýi, Türkmenistan"###),
      ("to", r###"lea fakatonga, tohinima fakalatina, Tonga"###),
      ("tok", r###"Toki Pona, Latn, 001"###),
      ("tr", r###"Türkçe, Latin, Türkiye"###),
      ("tr-CY", r###"Türkçe, Latin, Kıbrıs"###),
      ("tt", r###"татар, кирилл, Россия"###),
      ("twq", r###"Tasawaq senni, Latn, Nižer"###),
      ("tzm", r###"Tamaziɣt n laṭlaṣ, Latn, Meṛṛuk"###),
      ("ug", r###"ئۇيغۇرچە, ئەرەب, جۇڭگو"###),
      ("uk", r###"українська, кирилиця, Україна"###),
      ("und", r###"und, Latn, US"###),
      ("ur", r###"اردو, عربی, پاکستان"###),
      ("ur-IN", r###"اردو, عربی, بھارت"###),
      ("uz", r###"o‘zbek, lotin, Oʻzbekiston"###),
      ("uz-Arab", r###"اوزبیک, عربی, افغانستان"###),
      ("uz-Cyrl", r###"ўзбекча, Кирил, Ўзбекистон"###),
      ("uz-Latn", r###"o‘zbek, lotin, Oʻzbekiston"###),
      ("vai", r###"ꕙꔤ, Vaii, ꕞꔤꔫꕩ"###),
      ("vai-Latn", r###"Vai, Latn, Laibhiya"###),
      ("vai-Vaii", r###"ꕙꔤ, Vaii, ꕞꔤꔫꕩ"###),
      ("vi", r###"Tiếng Việt, Chữ La tinh, Việt Nam"###),
      ("vun", r###"Kyivunjo, Latn, Tanzania"###),
      ("wae", r###"Walser, Latiniš, Schwiz"###),
      ("wo", r###"Wolof, Latin, Senegaal"###),
      ("xh", r###"IsiXhosa, IsiLatin, EMzantsi Afrika"###),
      ("xog", r###"Olusoga, Latn, Yuganda"###),
      ("yav", r###"nuasue, Latn, Kemelún"###),
      ("yi", r###"ייִדיש, העברעיש, וועלט"###),
      ("yo", r###"Èdè Yorùbá, Èdè Látìn, Nàìjíríà"###),
      ("yo-BJ", r###"Èdè Yorùbá, Èdè Látìn, Bɛ̀nɛ̀"###),
      ("yrl", r###"nheẽgatu, ratĩ, Brasiu"###),
      ("yrl-CO", r###"ñengatú, ratĩ, Kurũbiya"###),
      ("yrl-VE", r###"ñengatú, ratĩ, Wenesuera"###),
      ("yue", r###"粵語, 繁體, 中華人民共和國香港特別行政區"###),
      ("yue-Hans", r###"粤语, 简体, 中华人民共和国"###),
      (
        "yue-Hant",
        r###"粵語, 繁體, 中華人民共和國香港特別行政區"###,
      ),
      ("zgh", r###"ⵜⴰⵎⴰⵣⵉⵖⵜ, Tfng, ⵍⵎⵖⵔⵉⴱ"###),
      ("zh", r###"简体中文, 中国"###),
      ("zh-Hans", r###"简体中文, 中国"###),
      ("zh-Hans-HK", r###"简体中文, 中国香港特别行政区"###),
      ("zh-Hans-MO", r###"简体中文, 中国澳门特别行政区"###),
      ("zh-Hans-SG", r###"华文, 新加坡"###),
      ("zh-Hant", r###"正體中文, 中國台灣"###),
      ("zh-Hant-HK", r###"繁體中文, 中國香港"###),
      ("zh-Hant-MO", r###"繁體中文, 中國澳門"###),
      ("zh-pinyin", r###"HànYǔ PīnYīn, ZhōngGuó"###),
      ("zu", r###"isiZulu, isi-Latin, iNingizimu Afrika"###),
    ],
  }
}
