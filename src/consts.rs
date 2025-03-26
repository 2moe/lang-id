use crate::{LangID, id::ID};

/// af: (af-Latn-ZA) Afrikaans, Latyn, Suid-Afrika
pub const fn lang_id_af() -> LangID {
  ID::new(26209, None, None).into_lang_id()
}

/// af-NA: (af-Latn-NA) Afrikaans, Latyn, Namibië
pub const fn lang_id_af_na() -> LangID {
  ID::new(26209, None, Some(16718)).into_lang_id()
}

/// agq: (agq-Latn-CM) Aghem, Latn, Kàmàlûŋ
pub const fn lang_id_agq() -> LangID {
  ID::new(7432033, None, None).into_lang_id()
}

/// ak: (ak-Latn-GH) Akan, Latn, Gaana
pub const fn lang_id_ak() -> LangID {
  ID::new(27489, None, None).into_lang_id()
}

/// am: (am-Ethi-ET) አማርኛ, ኢትዮፒክ, ኢትዮጵያ
pub const fn lang_id_am() -> LangID {
  ID::new(28001, None, None).into_lang_id()
}

/// ann: (ann-Latn-NG) Obolo, Latn, NG
pub const fn lang_id_ann() -> LangID {
  ID::new(7237217, None, None).into_lang_id()
}

/// ar: (ar-Arab-EG) العربية, العربية, مصر
pub const fn lang_id_ar() -> LangID {
  ID::new(29281, None, None).into_lang_id()
}

/// ar-AE: (ar-Arab-AE) العربية, العربية, الإمارات العربية المتحدة
pub const fn lang_id_ar_ae() -> LangID {
  ID::new(29281, None, Some(17729)).into_lang_id()
}

/// ar-BH: (ar-Arab-BH) العربية, العربية, البحرين
pub const fn lang_id_ar_bh() -> LangID {
  ID::new(29281, None, Some(18498)).into_lang_id()
}

/// ar-DJ: (ar-Arab-DJ) العربية, العربية, جيبوتي
pub const fn lang_id_ar_dj() -> LangID {
  ID::new(29281, None, Some(19012)).into_lang_id()
}

/// ar-DZ: (ar-Arab-DZ) العربية, العربية, الجزائر
pub const fn lang_id_ar_dz() -> LangID {
  ID::new(29281, None, Some(23108)).into_lang_id()
}

/// ar-EG: (ar-Arab-EG) العربية, العربية, مصر
pub const fn lang_id_ar_eg() -> LangID {
  ID::new(29281, None, Some(18245)).into_lang_id()
}

/// ar-EH: (ar-Arab-EH) العربية, العربية, الصحراء الغربية
pub const fn lang_id_ar_eh() -> LangID {
  ID::new(29281, None, Some(18501)).into_lang_id()
}

/// ar-ER: (ar-Arab-ER) العربية, العربية, إريتريا
pub const fn lang_id_ar_er() -> LangID {
  ID::new(29281, None, Some(21061)).into_lang_id()
}

/// ar-IL: (ar-Arab-IL) العربية, العربية, إسرائيل
pub const fn lang_id_ar_il() -> LangID {
  ID::new(29281, None, Some(19529)).into_lang_id()
}

/// ar-IQ: (ar-Arab-IQ) العربية, العربية, العراق
pub const fn lang_id_ar_iq() -> LangID {
  ID::new(29281, None, Some(20809)).into_lang_id()
}

/// ar-JO: (ar-Arab-JO) العربية, العربية, الأردن
pub const fn lang_id_ar_jo() -> LangID {
  ID::new(29281, None, Some(20298)).into_lang_id()
}

/// ar-KM: (ar-Arab-KM) العربية, العربية, جزر القمر
pub const fn lang_id_ar_km() -> LangID {
  ID::new(29281, None, Some(19787)).into_lang_id()
}

/// ar-KW: (ar-Arab-KW) العربية, العربية, الكويت
pub const fn lang_id_ar_kw() -> LangID {
  ID::new(29281, None, Some(22347)).into_lang_id()
}

/// ar-LB: (ar-Arab-LB) العربية, العربية, لبنان
pub const fn lang_id_ar_lb() -> LangID {
  ID::new(29281, None, Some(16972)).into_lang_id()
}

/// ar-LY: (ar-Arab-LY) العربية, العربية, ليبيا
pub const fn lang_id_ar_ly() -> LangID {
  ID::new(29281, None, Some(22860)).into_lang_id()
}

/// ar-MA: (ar-Arab-MA) العربية, العربية, المغرب
pub const fn lang_id_ar_ma() -> LangID {
  ID::new(29281, None, Some(16717)).into_lang_id()
}

/// ar-MR: (ar-Arab-MR) العربية, العربية, موريتانيا
pub const fn lang_id_ar_mr() -> LangID {
  ID::new(29281, None, Some(21069)).into_lang_id()
}

/// ar-OM: (ar-Arab-OM) العربية, العربية, عُمان
pub const fn lang_id_ar_om() -> LangID {
  ID::new(29281, None, Some(19791)).into_lang_id()
}

/// ar-PS: (ar-Arab-PS) العربية, العربية, الأراضي الفلسطينية
pub const fn lang_id_ar_ps() -> LangID {
  ID::new(29281, None, Some(21328)).into_lang_id()
}

/// ar-QA: (ar-Arab-QA) العربية, العربية, قطر
pub const fn lang_id_ar_qa() -> LangID {
  ID::new(29281, None, Some(16721)).into_lang_id()
}

/// ar-SA: (ar-Arab-SA) العربية, العربية, المملكة العربية السعودية
pub const fn lang_id_ar_sa() -> LangID {
  ID::new(29281, None, Some(16723)).into_lang_id()
}

/// ar-SD: (ar-Arab-SD) العربية, العربية, السودان
pub const fn lang_id_ar_sd() -> LangID {
  ID::new(29281, None, Some(17491)).into_lang_id()
}

/// ar-SO: (ar-Arab-SO) العربية, العربية, الصومال
pub const fn lang_id_ar_so() -> LangID {
  ID::new(29281, None, Some(20307)).into_lang_id()
}

/// ar-SS: (ar-Arab-SS) العربية, العربية, جنوب السودان
pub const fn lang_id_ar_ss() -> LangID {
  ID::new(29281, None, Some(21331)).into_lang_id()
}

/// ar-SY: (ar-Arab-SY) العربية, العربية, سوريا
pub const fn lang_id_ar_sy() -> LangID {
  ID::new(29281, None, Some(22867)).into_lang_id()
}

/// ar-TD: (ar-Arab-TD) العربية, العربية, تشاد
pub const fn lang_id_ar_td() -> LangID {
  ID::new(29281, None, Some(17492)).into_lang_id()
}

/// ar-TN: (ar-Arab-TN) العربية, العربية, تونس
pub const fn lang_id_ar_tn() -> LangID {
  ID::new(29281, None, Some(20052)).into_lang_id()
}

/// ar-YE: (ar-Arab-YE) العربية, العربية, اليمن
pub const fn lang_id_ar_ye() -> LangID {
  ID::new(29281, None, Some(17753)).into_lang_id()
}

/// as: (as-Beng-IN) অসমীয়া, বাংলা, ভাৰত
pub const fn lang_id_as() -> LangID {
  ID::new(29537, None, None).into_lang_id()
}

/// asa: (asa-Latn-TZ) Kipare, Latn, Tadhania
pub const fn lang_id_asa() -> LangID {
  ID::new(6386529, None, None).into_lang_id()
}

/// ast: (ast-Latn-ES) asturianu, llatín, España
pub const fn lang_id_ast() -> LangID {
  ID::new(7631713, None, None).into_lang_id()
}

/// az: (az-Latn-AZ) azərbaycan, latın, Azərbaycan
pub const fn lang_id_az() -> LangID {
  ID::new(31329, None, None).into_lang_id()
}

/// az-Cyrl: (az-Cyrl-AZ) азәрбајҹан, Кирил, Азәрбајҹан
pub const fn lang_id_az_cyrl() -> LangID {
  ID::new(31329, Some(1819441475), None).into_lang_id()
}

/// az-Latn: (az-Latn-AZ) azərbaycan, latın, Azərbaycan
pub const fn lang_id_az_latn() -> LangID {
  ID::new(31329, Some(1853120844), None).into_lang_id()
}

/// bas: (bas-Latn-CM) Ɓàsàa, Latn, Kàmɛ̀rûn
pub const fn lang_id_bas() -> LangID {
  ID::new(7561570, None, None).into_lang_id()
}

/// be: (be-Cyrl-BY) беларуская, кірыліца, Беларусь
pub const fn lang_id_be() -> LangID {
  ID::new(25954, None, None).into_lang_id()
}

/// bem: (bem-Latn-ZM) Ichibemba, Latn, Zambia
pub const fn lang_id_bem() -> LangID {
  ID::new(7169378, None, None).into_lang_id()
}

/// bez: (bez-Latn-TZ) Hibena, Latn, Hutanzania
pub const fn lang_id_bez() -> LangID {
  ID::new(8021346, None, None).into_lang_id()
}

/// bg: (bg-Cyrl-BG) български, кирилица, България
pub const fn lang_id_bg() -> LangID {
  ID::new(26466, None, None).into_lang_id()
}

/// bgc: (bgc-Deva-IN) हरियाणवी, देवानागारी, भारत
pub const fn lang_id_bgc() -> LangID {
  ID::new(6514530, None, None).into_lang_id()
}

/// bho: (bho-Deva-IN) भोजपुरी, देवानागारी, भारत
pub const fn lang_id_bho() -> LangID {
  ID::new(7301218, None, None).into_lang_id()
}

/// bm: (bm-Latn-ML) bamanakan, Latn, Mali
pub const fn lang_id_bm() -> LangID {
  ID::new(28002, None, None).into_lang_id()
}

/// bn: (bn-Beng-BD) বাংলা, বাংলা, বাংলাদেশ
pub const fn lang_id_bn() -> LangID {
  ID::new(28258, None, None).into_lang_id()
}

/// bn-IN: (bn-Beng-IN) বাংলা, বাংলা, ভারত
pub const fn lang_id_bn_in() -> LangID {
  ID::new(28258, None, Some(20041)).into_lang_id()
}

/// bo: (bo-Tibt-CN) བོད་སྐད་, བོད་ཡིག་, རྒྱ་ནག
pub const fn lang_id_bo() -> LangID {
  ID::new(28514, None, None).into_lang_id()
}

/// bo-IN: (bo-Tibt-IN) བོད་སྐད་, བོད་ཡིག་, རྒྱ་གར་
pub const fn lang_id_bo_in() -> LangID {
  ID::new(28514, None, Some(20041)).into_lang_id()
}

/// br: (br-Latn-FR) brezhoneg, latin, Frañs
pub const fn lang_id_br() -> LangID {
  ID::new(29282, None, None).into_lang_id()
}

/// brx: (brx-Deva-IN) बर’, देबनागिरि, भारत
pub const fn lang_id_brx() -> LangID {
  ID::new(7893602, None, None).into_lang_id()
}

/// bs: (bs-Latn-BA) bosanski, latinica, Bosna i Hercegovina
pub const fn lang_id_bs() -> LangID {
  ID::new(29538, None, None).into_lang_id()
}

/// bs-Cyrl: (bs-Cyrl-BA) босански, ћирилица, Босна и Херцеговина
pub const fn lang_id_bs_cyrl() -> LangID {
  ID::new(29538, Some(1819441475), None).into_lang_id()
}

/// bs-Latn: (bs-Latn-BA) bosanski, latinica, Bosna i Hercegovina
pub const fn lang_id_bs_latn() -> LangID {
  ID::new(29538, Some(1853120844), None).into_lang_id()
}

/// ca: (ca-Latn-ES) català, llatí, Espanya
pub const fn lang_id_ca() -> LangID {
  ID::new(24931, None, None).into_lang_id()
}

/// ca-AD: (ca-Latn-AD) català, llatí, Andorra
pub const fn lang_id_ca_ad() -> LangID {
  ID::new(24931, None, Some(17473)).into_lang_id()
}

/// ca-FR: (ca-Latn-FR) català, llatí, França
pub const fn lang_id_ca_fr() -> LangID {
  ID::new(24931, None, Some(21062)).into_lang_id()
}

/// ca-IT: (ca-Latn-IT) català, llatí, Itàlia
pub const fn lang_id_ca_it() -> LangID {
  ID::new(24931, None, Some(21577)).into_lang_id()
}

/// ccp: (ccp-Cakm-BD) 𑄌𑄋𑄴𑄟𑄳𑄦, 𑄌𑄇𑄴𑄟, 𑄝𑄁𑄣𑄘𑄬𑄌𑄴
pub const fn lang_id_ccp() -> LangID {
  ID::new(7365475, None, None).into_lang_id()
}

/// ccp-IN: (ccp-Cakm-IN) 𑄌𑄋𑄴𑄟𑄳𑄦, 𑄌𑄇𑄴𑄟, 𑄞𑄢𑄧𑄖𑄴
pub const fn lang_id_ccp_in() -> LangID {
  ID::new(7365475, None, Some(20041)).into_lang_id()
}

/// ce: (ce-Cyrl-RU) нохчийн, кириллица, Росси
pub const fn lang_id_ce() -> LangID {
  ID::new(25955, None, None).into_lang_id()
}

/// ceb: (ceb-Latn-PH) Cebuano, Latin, Pilipinas
pub const fn lang_id_ceb() -> LangID {
  ID::new(6448483, None, None).into_lang_id()
}

/// cgg: (cgg-Latn-UG) Rukiga, Latn, Uganda
pub const fn lang_id_cgg() -> LangID {
  ID::new(6776675, None, None).into_lang_id()
}

/// chr: (chr-Cher-US) ᏣᎳᎩ, ᏣᎳᎩ, ᏌᏊ ᎢᏳᎾᎵᏍᏔᏅ ᏍᎦᏚᎩ
pub const fn lang_id_chr() -> LangID {
  ID::new(7497827, None, None).into_lang_id()
}

/// ckb: (ckb-Arab-IQ) کوردیی ناوەندی, عەرەبی, عێراق
pub const fn lang_id_ckb() -> LangID {
  ID::new(6450019, None, None).into_lang_id()
}

/// ckb-IR: (ckb-Arab-IR) کوردیی ناوەندی, عەرەبی, ئێران
pub const fn lang_id_ckb_ir() -> LangID {
  ID::new(6450019, None, Some(21065)).into_lang_id()
}

/// cs: (cs-Latn-CZ) čeština, latinka, Česko
pub const fn lang_id_cs() -> LangID {
  ID::new(29539, None, None).into_lang_id()
}

/// cv: (cv-Cyrl-RU) чӑваш, кириллица, Раҫҫей
pub const fn lang_id_cv() -> LangID {
  ID::new(30307, None, None).into_lang_id()
}

/// cy: (cy-Latn-GB) Cymraeg, Lladin, Y Deyrnas Unedig
pub const fn lang_id_cy() -> LangID {
  ID::new(31075, None, None).into_lang_id()
}

/// da: (da-Latn-DK) dansk, latinsk, Danmark
pub const fn lang_id_da() -> LangID {
  ID::new(24932, None, None).into_lang_id()
}

/// da-GL: (da-Latn-GL) dansk, latinsk, Grønland
pub const fn lang_id_da_gl() -> LangID {
  ID::new(24932, None, Some(19527)).into_lang_id()
}

/// dav: (dav-Latn-KE) Kitaita, Latn, Kenya
pub const fn lang_id_dav() -> LangID {
  ID::new(7758180, None, None).into_lang_id()
}

/// de: (de-Latn-DE) Deutsch, Lateinisch, Deutschland
pub const fn lang_id_de() -> LangID {
  ID::new(25956, None, None).into_lang_id()
}

/// de-AT: (de-Latn-AT) Deutsch, Lateinisch, Österreich
pub const fn lang_id_de_at() -> LangID {
  ID::new(25956, None, Some(21569)).into_lang_id()
}

/// de-BE: (de-Latn-BE) Deutsch, Lateinisch, Belgien
pub const fn lang_id_de_be() -> LangID {
  ID::new(25956, None, Some(17730)).into_lang_id()
}

/// de-CH: (de-Latn-CH) Deutsch, Lateinisch, Schweiz
pub const fn lang_id_de_ch() -> LangID {
  ID::new(25956, None, Some(18499)).into_lang_id()
}

/// de-IT: (de-Latn-IT) Deutsch, Lateinisch, Italien
pub const fn lang_id_de_it() -> LangID {
  ID::new(25956, None, Some(21577)).into_lang_id()
}

/// de-LI: (de-Latn-LI) Deutsch, Lateinisch, Liechtenstein
pub const fn lang_id_de_li() -> LangID {
  ID::new(25956, None, Some(18764)).into_lang_id()
}

/// de-LU: (de-Latn-LU) Deutsch, Lateinisch, Luxemburg
pub const fn lang_id_de_lu() -> LangID {
  ID::new(25956, None, Some(21836)).into_lang_id()
}

/// dje: (dje-Latn-NE) Zarmaciine, Latn, Nižer
pub const fn lang_id_dje() -> LangID {
  ID::new(6646372, None, None).into_lang_id()
}

/// doi: (doi-Deva-IN) डोगरी, देवनागरी, भारत
pub const fn lang_id_doi() -> LangID {
  ID::new(6909796, None, None).into_lang_id()
}

/// dsb: (dsb-Latn-DE) dolnoserbšćina, łatyński, Nimska
pub const fn lang_id_dsb() -> LangID {
  ID::new(6452068, None, None).into_lang_id()
}

/// dua: (dua-Latn-CM) duálá, Latn, Cameroun
pub const fn lang_id_dua() -> LangID {
  ID::new(6387044, None, None).into_lang_id()
}

/// dyo: (dyo-Latn-SN) joola, Latn, Senegal
pub const fn lang_id_dyo() -> LangID {
  ID::new(7305572, None, None).into_lang_id()
}

/// dz: (dz-Tibt-BT) རྫོང་ཁ, ང་བཅས་ཀྱི་ཡིག་གུ, འབྲུག
pub const fn lang_id_dz() -> LangID {
  ID::new(31332, None, None).into_lang_id()
}

/// ebu: (ebu-Latn-KE) Kĩembu, Latn, Kenya
pub const fn lang_id_ebu() -> LangID {
  ID::new(7692901, None, None).into_lang_id()
}

/// ee: (ee-Latn-GH) Eʋegbe, Latingbeŋɔŋlɔ, Ghana nutome
pub const fn lang_id_ee() -> LangID {
  ID::new(25957, None, None).into_lang_id()
}

/// ee-TG: (ee-Latn-TG) Eʋegbe, Latingbeŋɔŋlɔ, Togo nutome
pub const fn lang_id_ee_tg() -> LangID {
  ID::new(25957, None, Some(18260)).into_lang_id()
}

/// el: (el-Grek-GR) Ελληνικά, Ελληνικό, Ελλάδα
pub const fn lang_id_el() -> LangID {
  ID::new(27749, None, None).into_lang_id()
}

/// el-CY: (el-Grek-CY) Ελληνικά, Ελληνικό, Κύπρος
pub const fn lang_id_el_cy() -> LangID {
  ID::new(27749, None, Some(22851)).into_lang_id()
}

/// en: (en-Latn-US) English, Latin, United States
pub const fn lang_id_en() -> LangID {
  ID::new(28261, None, None).into_lang_id()
}

/// en-US: (en-Latn-US) English, Latin, United States
pub const fn lang_id_en_us() -> LangID {
  ID::new(28261, None, Some(21333)).into_lang_id()
}

/// en-001: (en-Latn-001) English, Latin, world
pub const fn lang_id_en_001() -> LangID {
  ID::new(28261, None, Some(3223600)).into_lang_id()
}

/// en-150: (en-Latn-150) English, Latin, Europe
pub const fn lang_id_en_150() -> LangID {
  ID::new(28261, None, Some(3159345)).into_lang_id()
}

/// en-AE: (en-Latn-AE) English, Latin, United Arab Emirates
pub const fn lang_id_en_ae() -> LangID {
  ID::new(28261, None, Some(17729)).into_lang_id()
}

/// en-AG: (en-Latn-AG) English, Latin, Antigua & Barbuda
pub const fn lang_id_en_ag() -> LangID {
  ID::new(28261, None, Some(18241)).into_lang_id()
}

/// en-AI: (en-Latn-AI) English, Latin, Anguilla
pub const fn lang_id_en_ai() -> LangID {
  ID::new(28261, None, Some(18753)).into_lang_id()
}

/// en-AS: (en-Latn-AS) English, Latin, American Samoa
pub const fn lang_id_en_as() -> LangID {
  ID::new(28261, None, Some(21313)).into_lang_id()
}

/// en-AT: (en-Latn-AT) English, Latin, Austria
pub const fn lang_id_en_at() -> LangID {
  ID::new(28261, None, Some(21569)).into_lang_id()
}

/// en-AU: (en-Latn-AU) English, Latin, Australia
pub const fn lang_id_en_au() -> LangID {
  ID::new(28261, None, Some(21825)).into_lang_id()
}

/// en-BB: (en-Latn-BB) English, Latin, Barbados
pub const fn lang_id_en_bb() -> LangID {
  ID::new(28261, None, Some(16962)).into_lang_id()
}

/// en-BE: (en-Latn-BE) English, Latin, Belgium
pub const fn lang_id_en_be() -> LangID {
  ID::new(28261, None, Some(17730)).into_lang_id()
}

/// en-BI: (en-Latn-BI) English, Latin, Burundi
pub const fn lang_id_en_bi() -> LangID {
  ID::new(28261, None, Some(18754)).into_lang_id()
}

/// en-BM: (en-Latn-BM) English, Latin, Bermuda
pub const fn lang_id_en_bm() -> LangID {
  ID::new(28261, None, Some(19778)).into_lang_id()
}

/// en-BS: (en-Latn-BS) English, Latin, Bahamas
pub const fn lang_id_en_bs() -> LangID {
  ID::new(28261, None, Some(21314)).into_lang_id()
}

/// en-BW: (en-Latn-BW) English, Latin, Botswana
pub const fn lang_id_en_bw() -> LangID {
  ID::new(28261, None, Some(22338)).into_lang_id()
}

/// en-BZ: (en-Latn-BZ) English, Latin, Belize
pub const fn lang_id_en_bz() -> LangID {
  ID::new(28261, None, Some(23106)).into_lang_id()
}

/// en-CA: (en-Latn-CA) English, Latin, Canada
pub const fn lang_id_en_ca() -> LangID {
  ID::new(28261, None, Some(16707)).into_lang_id()
}

/// en-CC: (en-Latn-CC) English, Latin, Cocos (Keeling) Islands
pub const fn lang_id_en_cc() -> LangID {
  ID::new(28261, None, Some(17219)).into_lang_id()
}

/// en-CH: (en-Latn-CH) English, Latin, Switzerland
pub const fn lang_id_en_ch() -> LangID {
  ID::new(28261, None, Some(18499)).into_lang_id()
}

/// en-CK: (en-Latn-CK) English, Latin, Cook Islands
pub const fn lang_id_en_ck() -> LangID {
  ID::new(28261, None, Some(19267)).into_lang_id()
}

/// en-CM: (en-Latn-CM) English, Latin, Cameroon
pub const fn lang_id_en_cm() -> LangID {
  ID::new(28261, None, Some(19779)).into_lang_id()
}

/// en-CX: (en-Latn-CX) English, Latin, Christmas Island
pub const fn lang_id_en_cx() -> LangID {
  ID::new(28261, None, Some(22595)).into_lang_id()
}

/// en-CY: (en-Latn-CY) English, Latin, Cyprus
pub const fn lang_id_en_cy() -> LangID {
  ID::new(28261, None, Some(22851)).into_lang_id()
}

/// en-DE: (en-Latn-DE) English, Latin, Germany
pub const fn lang_id_en_de() -> LangID {
  ID::new(28261, None, Some(17732)).into_lang_id()
}

/// en-DG: (en-Latn-DG) English, Latin, Diego Garcia
pub const fn lang_id_en_dg() -> LangID {
  ID::new(28261, None, Some(18244)).into_lang_id()
}

/// en-DK: (en-Latn-DK) English, Latin, Denmark
pub const fn lang_id_en_dk() -> LangID {
  ID::new(28261, None, Some(19268)).into_lang_id()
}

/// en-DM: (en-Latn-DM) English, Latin, Dominica
pub const fn lang_id_en_dm() -> LangID {
  ID::new(28261, None, Some(19780)).into_lang_id()
}

/// en-ER: (en-Latn-ER) English, Latin, Eritrea
pub const fn lang_id_en_er() -> LangID {
  ID::new(28261, None, Some(21061)).into_lang_id()
}

/// en-FI: (en-Latn-FI) English, Latin, Finland
pub const fn lang_id_en_fi() -> LangID {
  ID::new(28261, None, Some(18758)).into_lang_id()
}

/// en-FJ: (en-Latn-FJ) English, Latin, Fiji
pub const fn lang_id_en_fj() -> LangID {
  ID::new(28261, None, Some(19014)).into_lang_id()
}

/// en-FK: (en-Latn-FK) English, Latin, Falkland Islands
pub const fn lang_id_en_fk() -> LangID {
  ID::new(28261, None, Some(19270)).into_lang_id()
}

/// en-FM: (en-Latn-FM) English, Latin, Micronesia
pub const fn lang_id_en_fm() -> LangID {
  ID::new(28261, None, Some(19782)).into_lang_id()
}

/// en-GB: (en-Latn-GB) English, Latin, United Kingdom
pub const fn lang_id_en_gb() -> LangID {
  ID::new(28261, None, Some(16967)).into_lang_id()
}

/// en-GD: (en-Latn-GD) English, Latin, Grenada
pub const fn lang_id_en_gd() -> LangID {
  ID::new(28261, None, Some(17479)).into_lang_id()
}

/// en-GG: (en-Latn-GG) English, Latin, Guernsey
pub const fn lang_id_en_gg() -> LangID {
  ID::new(28261, None, Some(18247)).into_lang_id()
}

/// en-GH: (en-Latn-GH) English, Latin, Ghana
pub const fn lang_id_en_gh() -> LangID {
  ID::new(28261, None, Some(18503)).into_lang_id()
}

/// en-GI: (en-Latn-GI) English, Latin, Gibraltar
pub const fn lang_id_en_gi() -> LangID {
  ID::new(28261, None, Some(18759)).into_lang_id()
}

/// en-GM: (en-Latn-GM) English, Latin, Gambia
pub const fn lang_id_en_gm() -> LangID {
  ID::new(28261, None, Some(19783)).into_lang_id()
}

/// en-GU: (en-Latn-GU) English, Latin, Guam
pub const fn lang_id_en_gu() -> LangID {
  ID::new(28261, None, Some(21831)).into_lang_id()
}

/// en-GY: (en-Latn-GY) English, Latin, Guyana
pub const fn lang_id_en_gy() -> LangID {
  ID::new(28261, None, Some(22855)).into_lang_id()
}

/// en-HK: (en-Latn-HK) English, Latin, Hong Kong SAR China
pub const fn lang_id_en_hk() -> LangID {
  ID::new(28261, None, Some(19272)).into_lang_id()
}

/// en-IE: (en-Latn-IE) English, Latin, Ireland
pub const fn lang_id_en_ie() -> LangID {
  ID::new(28261, None, Some(17737)).into_lang_id()
}

/// en-IL: (en-Latn-IL) English, Latin, Israel
pub const fn lang_id_en_il() -> LangID {
  ID::new(28261, None, Some(19529)).into_lang_id()
}

/// en-IM: (en-Latn-IM) English, Latin, Isle of Man
pub const fn lang_id_en_im() -> LangID {
  ID::new(28261, None, Some(19785)).into_lang_id()
}

/// en-IN: (en-Latn-IN) English, Latin, India
pub const fn lang_id_en_in() -> LangID {
  ID::new(28261, None, Some(20041)).into_lang_id()
}

/// en-IO: (en-Latn-IO) English, Latin, British Indian Ocean Territory
pub const fn lang_id_en_io() -> LangID {
  ID::new(28261, None, Some(20297)).into_lang_id()
}

/// en-JE: (en-Latn-JE) English, Latin, Jersey
pub const fn lang_id_en_je() -> LangID {
  ID::new(28261, None, Some(17738)).into_lang_id()
}

/// en-JM: (en-Latn-JM) English, Latin, Jamaica
pub const fn lang_id_en_jm() -> LangID {
  ID::new(28261, None, Some(19786)).into_lang_id()
}

/// en-KE: (en-Latn-KE) English, Latin, Kenya
pub const fn lang_id_en_ke() -> LangID {
  ID::new(28261, None, Some(17739)).into_lang_id()
}

/// en-KI: (en-Latn-KI) English, Latin, Kiribati
pub const fn lang_id_en_ki() -> LangID {
  ID::new(28261, None, Some(18763)).into_lang_id()
}

/// en-KN: (en-Latn-KN) English, Latin, St Kitts & Nevis
pub const fn lang_id_en_kn() -> LangID {
  ID::new(28261, None, Some(20043)).into_lang_id()
}

/// en-KY: (en-Latn-KY) English, Latin, Cayman Islands
pub const fn lang_id_en_ky() -> LangID {
  ID::new(28261, None, Some(22859)).into_lang_id()
}

/// en-LC: (en-Latn-LC) English, Latin, St Lucia
pub const fn lang_id_en_lc() -> LangID {
  ID::new(28261, None, Some(17228)).into_lang_id()
}

/// en-LR: (en-Latn-LR) English, Latin, Liberia
pub const fn lang_id_en_lr() -> LangID {
  ID::new(28261, None, Some(21068)).into_lang_id()
}

/// en-LS: (en-Latn-LS) English, Latin, Lesotho
pub const fn lang_id_en_ls() -> LangID {
  ID::new(28261, None, Some(21324)).into_lang_id()
}

/// en-MG: (en-Latn-MG) English, Latin, Madagascar
pub const fn lang_id_en_mg() -> LangID {
  ID::new(28261, None, Some(18253)).into_lang_id()
}

/// en-MH: (en-Latn-MH) English, Latin, Marshall Islands
pub const fn lang_id_en_mh() -> LangID {
  ID::new(28261, None, Some(18509)).into_lang_id()
}

/// en-MO: (en-Latn-MO) English, Latin, Macao SAR China
pub const fn lang_id_en_mo() -> LangID {
  ID::new(28261, None, Some(20301)).into_lang_id()
}

/// en-MP: (en-Latn-MP) English, Latin, Northern Mariana Islands
pub const fn lang_id_en_mp() -> LangID {
  ID::new(28261, None, Some(20557)).into_lang_id()
}

/// en-MS: (en-Latn-MS) English, Latin, Montserrat
pub const fn lang_id_en_ms() -> LangID {
  ID::new(28261, None, Some(21325)).into_lang_id()
}

/// en-MT: (en-Latn-MT) English, Latin, Malta
pub const fn lang_id_en_mt() -> LangID {
  ID::new(28261, None, Some(21581)).into_lang_id()
}

/// en-MU: (en-Latn-MU) English, Latin, Mauritius
pub const fn lang_id_en_mu() -> LangID {
  ID::new(28261, None, Some(21837)).into_lang_id()
}

/// en-MV: (en-Latn-MV) English, Latin, Maldives
pub const fn lang_id_en_mv() -> LangID {
  ID::new(28261, None, Some(22093)).into_lang_id()
}

/// en-MW: (en-Latn-MW) English, Latin, Malawi
pub const fn lang_id_en_mw() -> LangID {
  ID::new(28261, None, Some(22349)).into_lang_id()
}

/// en-MY: (en-Latn-MY) English, Latin, Malaysia
pub const fn lang_id_en_my() -> LangID {
  ID::new(28261, None, Some(22861)).into_lang_id()
}

/// en-NA: (en-Latn-NA) English, Latin, Namibia
pub const fn lang_id_en_na() -> LangID {
  ID::new(28261, None, Some(16718)).into_lang_id()
}

/// en-NF: (en-Latn-NF) English, Latin, Norfolk Island
pub const fn lang_id_en_nf() -> LangID {
  ID::new(28261, None, Some(17998)).into_lang_id()
}

/// en-NG: (en-Latn-NG) English, Latin, Nigeria
pub const fn lang_id_en_ng() -> LangID {
  ID::new(28261, None, Some(18254)).into_lang_id()
}

/// en-NL: (en-Latn-NL) English, Latin, Netherlands
pub const fn lang_id_en_nl() -> LangID {
  ID::new(28261, None, Some(19534)).into_lang_id()
}

/// en-NR: (en-Latn-NR) English, Latin, Nauru
pub const fn lang_id_en_nr() -> LangID {
  ID::new(28261, None, Some(21070)).into_lang_id()
}

/// en-NU: (en-Latn-NU) English, Latin, Niue
pub const fn lang_id_en_nu() -> LangID {
  ID::new(28261, None, Some(21838)).into_lang_id()
}

/// en-NZ: (en-Latn-NZ) English, Latin, New Zealand
pub const fn lang_id_en_nz() -> LangID {
  ID::new(28261, None, Some(23118)).into_lang_id()
}

/// en-PG: (en-Latn-PG) English, Latin, Papua New Guinea
pub const fn lang_id_en_pg() -> LangID {
  ID::new(28261, None, Some(18256)).into_lang_id()
}

/// en-PH: (en-Latn-PH) English, Latin, Philippines
pub const fn lang_id_en_ph() -> LangID {
  ID::new(28261, None, Some(18512)).into_lang_id()
}

/// en-PK: (en-Latn-PK) English, Latin, Pakistan
pub const fn lang_id_en_pk() -> LangID {
  ID::new(28261, None, Some(19280)).into_lang_id()
}

/// en-PN: (en-Latn-PN) English, Latin, Pitcairn Islands
pub const fn lang_id_en_pn() -> LangID {
  ID::new(28261, None, Some(20048)).into_lang_id()
}

/// en-PR: (en-Latn-PR) English, Latin, Puerto Rico
pub const fn lang_id_en_pr() -> LangID {
  ID::new(28261, None, Some(21072)).into_lang_id()
}

/// en-PW: (en-Latn-PW) English, Latin, Palau
pub const fn lang_id_en_pw() -> LangID {
  ID::new(28261, None, Some(22352)).into_lang_id()
}

/// en-RW: (en-Latn-RW) English, Latin, Rwanda
pub const fn lang_id_en_rw() -> LangID {
  ID::new(28261, None, Some(22354)).into_lang_id()
}

/// en-SB: (en-Latn-SB) English, Latin, Solomon Islands
pub const fn lang_id_en_sb() -> LangID {
  ID::new(28261, None, Some(16979)).into_lang_id()
}

/// en-SC: (en-Latn-SC) English, Latin, Seychelles
pub const fn lang_id_en_sc() -> LangID {
  ID::new(28261, None, Some(17235)).into_lang_id()
}

/// en-SD: (en-Latn-SD) English, Latin, Sudan
pub const fn lang_id_en_sd() -> LangID {
  ID::new(28261, None, Some(17491)).into_lang_id()
}

/// en-SE: (en-Latn-SE) English, Latin, Sweden
pub const fn lang_id_en_se() -> LangID {
  ID::new(28261, None, Some(17747)).into_lang_id()
}

/// en-SG: (en-Latn-SG) English, Latin, Singapore
pub const fn lang_id_en_sg() -> LangID {
  ID::new(28261, None, Some(18259)).into_lang_id()
}

/// en-SH: (en-Latn-SH) English, Latin, St Helena
pub const fn lang_id_en_sh() -> LangID {
  ID::new(28261, None, Some(18515)).into_lang_id()
}

/// en-SI: (en-Latn-SI) English, Latin, Slovenia
pub const fn lang_id_en_si() -> LangID {
  ID::new(28261, None, Some(18771)).into_lang_id()
}

/// en-SL: (en-Latn-SL) English, Latin, Sierra Leone
pub const fn lang_id_en_sl() -> LangID {
  ID::new(28261, None, Some(19539)).into_lang_id()
}

/// en-SS: (en-Latn-SS) English, Latin, South Sudan
pub const fn lang_id_en_ss() -> LangID {
  ID::new(28261, None, Some(21331)).into_lang_id()
}

/// en-SX: (en-Latn-SX) English, Latin, Sint Maarten
pub const fn lang_id_en_sx() -> LangID {
  ID::new(28261, None, Some(22611)).into_lang_id()
}

/// en-SZ: (en-Latn-SZ) English, Latin, Eswatini
pub const fn lang_id_en_sz() -> LangID {
  ID::new(28261, None, Some(23123)).into_lang_id()
}

/// en-TC: (en-Latn-TC) English, Latin, Turks & Caicos Islands
pub const fn lang_id_en_tc() -> LangID {
  ID::new(28261, None, Some(17236)).into_lang_id()
}

/// en-TK: (en-Latn-TK) English, Latin, Tokelau
pub const fn lang_id_en_tk() -> LangID {
  ID::new(28261, None, Some(19284)).into_lang_id()
}

/// en-TO: (en-Latn-TO) English, Latin, Tonga
pub const fn lang_id_en_to() -> LangID {
  ID::new(28261, None, Some(20308)).into_lang_id()
}

/// en-TT: (en-Latn-TT) English, Latin, Trinidad & Tobago
pub const fn lang_id_en_tt() -> LangID {
  ID::new(28261, None, Some(21588)).into_lang_id()
}

/// en-TV: (en-Latn-TV) English, Latin, Tuvalu
pub const fn lang_id_en_tv() -> LangID {
  ID::new(28261, None, Some(22100)).into_lang_id()
}

/// en-TZ: (en-Latn-TZ) English, Latin, Tanzania
pub const fn lang_id_en_tz() -> LangID {
  ID::new(28261, None, Some(23124)).into_lang_id()
}

/// en-UG: (en-Latn-UG) English, Latin, Uganda
pub const fn lang_id_en_ug() -> LangID {
  ID::new(28261, None, Some(18261)).into_lang_id()
}

/// en-UM: (en-Latn-UM) English, Latin, U.S. Outlying Islands
pub const fn lang_id_en_um() -> LangID {
  ID::new(28261, None, Some(19797)).into_lang_id()
}

/// en-VC: (en-Latn-VC) English, Latin, St Vincent & the Grenadines
pub const fn lang_id_en_vc() -> LangID {
  ID::new(28261, None, Some(17238)).into_lang_id()
}

/// en-VG: (en-Latn-VG) English, Latin, British Virgin Islands
pub const fn lang_id_en_vg() -> LangID {
  ID::new(28261, None, Some(18262)).into_lang_id()
}

/// en-VI: (en-Latn-VI) English, Latin, U.S. Virgin Islands
pub const fn lang_id_en_vi() -> LangID {
  ID::new(28261, None, Some(18774)).into_lang_id()
}

/// en-VU: (en-Latn-VU) English, Latin, Vanuatu
pub const fn lang_id_en_vu() -> LangID {
  ID::new(28261, None, Some(21846)).into_lang_id()
}

/// en-WS: (en-Latn-WS) English, Latin, Samoa
pub const fn lang_id_en_ws() -> LangID {
  ID::new(28261, None, Some(21335)).into_lang_id()
}

/// en-ZA: (en-Latn-ZA) English, Latin, South Africa
pub const fn lang_id_en_za() -> LangID {
  ID::new(28261, None, Some(16730)).into_lang_id()
}

/// en-ZM: (en-Latn-ZM) English, Latin, Zambia
pub const fn lang_id_en_zm() -> LangID {
  ID::new(28261, None, Some(19802)).into_lang_id()
}

/// en-ZW: (en-Latn-ZW) English, Latin, Zimbabwe
pub const fn lang_id_en_zw() -> LangID {
  ID::new(28261, None, Some(22362)).into_lang_id()
}

/// eo: (eo-Latn-001) esperanto, Latn, Mondo
pub const fn lang_id_eo() -> LangID {
  ID::new(28517, None, None).into_lang_id()
}

/// es: (es-Latn-ES) español, latino, España
pub const fn lang_id_es() -> LangID {
  ID::new(29541, None, None).into_lang_id()
}

/// es-419: (es-Latn-419) español, latín, Latinoamérica
pub const fn lang_id_es_419() -> LangID {
  ID::new(29541, None, Some(3748148)).into_lang_id()
}

/// es-AR: (es-Latn-AR) español, latín, Argentina
pub const fn lang_id_es_ar() -> LangID {
  ID::new(29541, None, Some(21057)).into_lang_id()
}

/// es-BO: (es-Latn-BO) español, latín, Bolivia
pub const fn lang_id_es_bo() -> LangID {
  ID::new(29541, None, Some(20290)).into_lang_id()
}

/// es-BR: (es-Latn-BR) español, latín, Brasil
pub const fn lang_id_es_br() -> LangID {
  ID::new(29541, None, Some(21058)).into_lang_id()
}

/// es-BZ: (es-Latn-BZ) español, latín, Belice
pub const fn lang_id_es_bz() -> LangID {
  ID::new(29541, None, Some(23106)).into_lang_id()
}

/// es-CL: (es-Latn-CL) español, latín, Chile
pub const fn lang_id_es_cl() -> LangID {
  ID::new(29541, None, Some(19523)).into_lang_id()
}

/// es-CO: (es-Latn-CO) español, latín, Colombia
pub const fn lang_id_es_co() -> LangID {
  ID::new(29541, None, Some(20291)).into_lang_id()
}

/// es-CR: (es-Latn-CR) español, latín, Costa Rica
pub const fn lang_id_es_cr() -> LangID {
  ID::new(29541, None, Some(21059)).into_lang_id()
}

/// es-CU: (es-Latn-CU) español, latín, Cuba
pub const fn lang_id_es_cu() -> LangID {
  ID::new(29541, None, Some(21827)).into_lang_id()
}

/// es-DO: (es-Latn-DO) español, latín, República Dominicana
pub const fn lang_id_es_do() -> LangID {
  ID::new(29541, None, Some(20292)).into_lang_id()
}

/// es-EA: (es-Latn-EA) español, latino, Ceuta y Melilla
pub const fn lang_id_es_ea() -> LangID {
  ID::new(29541, None, Some(16709)).into_lang_id()
}

/// es-EC: (es-Latn-EC) español, latín, Ecuador
pub const fn lang_id_es_ec() -> LangID {
  ID::new(29541, None, Some(17221)).into_lang_id()
}

/// es-GQ: (es-Latn-GQ) español, latino, Guinea Ecuatorial
pub const fn lang_id_es_gq() -> LangID {
  ID::new(29541, None, Some(20807)).into_lang_id()
}

/// es-GT: (es-Latn-GT) español, latín, Guatemala
pub const fn lang_id_es_gt() -> LangID {
  ID::new(29541, None, Some(21575)).into_lang_id()
}

/// es-HN: (es-Latn-HN) español, latín, Honduras
pub const fn lang_id_es_hn() -> LangID {
  ID::new(29541, None, Some(20040)).into_lang_id()
}

/// es-IC: (es-Latn-IC) español, latino, Canarias
pub const fn lang_id_es_ic() -> LangID {
  ID::new(29541, None, Some(17225)).into_lang_id()
}

/// es-MX: (es-Latn-MX) español, latín, México
pub const fn lang_id_es_mx() -> LangID {
  ID::new(29541, None, Some(22605)).into_lang_id()
}

/// es-NI: (es-Latn-NI) español, latín, Nicaragua
pub const fn lang_id_es_ni() -> LangID {
  ID::new(29541, None, Some(18766)).into_lang_id()
}

/// es-PA: (es-Latn-PA) español, latín, Panamá
pub const fn lang_id_es_pa() -> LangID {
  ID::new(29541, None, Some(16720)).into_lang_id()
}

/// es-PE: (es-Latn-PE) español, latín, Perú
pub const fn lang_id_es_pe() -> LangID {
  ID::new(29541, None, Some(17744)).into_lang_id()
}

/// es-PH: (es-Latn-PH) español, latino, Filipinas
pub const fn lang_id_es_ph() -> LangID {
  ID::new(29541, None, Some(18512)).into_lang_id()
}

/// es-PR: (es-Latn-PR) español, latín, Puerto Rico
pub const fn lang_id_es_pr() -> LangID {
  ID::new(29541, None, Some(21072)).into_lang_id()
}

/// es-PY: (es-Latn-PY) español, latín, Paraguay
pub const fn lang_id_es_py() -> LangID {
  ID::new(29541, None, Some(22864)).into_lang_id()
}

/// es-SV: (es-Latn-SV) español, latín, El Salvador
pub const fn lang_id_es_sv() -> LangID {
  ID::new(29541, None, Some(22099)).into_lang_id()
}

/// es-US: (es-Latn-US) español, latín, Estados Unidos
pub const fn lang_id_es_us() -> LangID {
  ID::new(29541, None, Some(21333)).into_lang_id()
}

/// es-UY: (es-Latn-UY) español, latín, Uruguay
pub const fn lang_id_es_uy() -> LangID {
  ID::new(29541, None, Some(22869)).into_lang_id()
}

/// es-VE: (es-Latn-VE) español, latín, Venezuela
pub const fn lang_id_es_ve() -> LangID {
  ID::new(29541, None, Some(17750)).into_lang_id()
}

/// et: (et-Latn-EE) eesti, ladina, Eesti
pub const fn lang_id_et() -> LangID {
  ID::new(29797, None, None).into_lang_id()
}

/// eu: (eu-Latn-ES) euskara, latinoa, Espainia
pub const fn lang_id_eu() -> LangID {
  ID::new(30053, None, None).into_lang_id()
}

/// ewo: (ewo-Latn-CM) ewondo, Latn, Kamərún
pub const fn lang_id_ewo() -> LangID {
  ID::new(7305061, None, None).into_lang_id()
}

/// fa: (fa-Arab-IR) فارسی, عربی, ایران
pub const fn lang_id_fa() -> LangID {
  ID::new(24934, None, None).into_lang_id()
}

/// fa-AF: (fa-Arab-AF) فارسی, عربی, افغانستان
pub const fn lang_id_fa_af() -> LangID {
  ID::new(24934, None, Some(17985)).into_lang_id()
}

/// ff: (ff-Latn-SN) Pulaar, Latn, Senegaal
pub const fn lang_id_ff() -> LangID {
  ID::new(26214, None, None).into_lang_id()
}

/// ff-Adlm: (ff-Adlm-GN) 𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤘𞤭𞤲𞤫
pub const fn lang_id_ff_adlm() -> LangID {
  ID::new(26214, Some(1835820097), None).into_lang_id()
}

/// ff-Adlm-BF: 𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤄𞤵𞤪𞤳𞤭𞤲𞤢 𞤊𞤢𞤧𞤮𞥅
pub const fn lang_id_ff_adlm_bf() -> LangID {
  ID::new(26214, Some(1835820097), Some(17986)).into_lang_id()
}

/// ff-Adlm-CM: 𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤑𞤢𞤥𞤢𞤪𞤵𞥅𞤲
pub const fn lang_id_ff_adlm_cm() -> LangID {
  ID::new(26214, Some(1835820097), Some(19779)).into_lang_id()
}

/// ff-Adlm-GH: 𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤘𞤢𞤲𞤢
pub const fn lang_id_ff_adlm_gh() -> LangID {
  ID::new(26214, Some(1835820097), Some(18503)).into_lang_id()
}

/// ff-Adlm-GM: 𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤘𞤢𞤥𞤦𞤭𞤴𞤢
pub const fn lang_id_ff_adlm_gm() -> LangID {
  ID::new(26214, Some(1835820097), Some(19783)).into_lang_id()
}

/// ff-Adlm-GW: 𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤘𞤭𞤲𞤫-𞤄𞤭𞤧𞤢𞤱𞤮𞥅
pub const fn lang_id_ff_adlm_gw() -> LangID {
  ID::new(26214, Some(1835820097), Some(22343)).into_lang_id()
}

/// ff-Adlm-LR: 𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤂𞤢𞤦𞤭𞤪𞤭𞤴𞤢𞥄
pub const fn lang_id_ff_adlm_lr() -> LangID {
  ID::new(26214, Some(1835820097), Some(21068)).into_lang_id()
}

/// ff-Adlm-MR: 𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤃𞤮𞤪𞤼𞤢𞤲𞤭𞥅
pub const fn lang_id_ff_adlm_mr() -> LangID {
  ID::new(26214, Some(1835820097), Some(21069)).into_lang_id()
}

/// ff-Adlm-NE: 𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤐𞤭𞥅𞤶𞤫𞤪
pub const fn lang_id_ff_adlm_ne() -> LangID {
  ID::new(26214, Some(1835820097), Some(17742)).into_lang_id()
}

/// ff-Adlm-NG: 𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤐𞤢𞤶𞤫𞤪𞤭𞤴𞤢𞥄
pub const fn lang_id_ff_adlm_ng() -> LangID {
  ID::new(26214, Some(1835820097), Some(18254)).into_lang_id()
}

/// ff-Adlm-SL: 𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤅𞤢𞤪𞤢𞤤𞤮𞤲
pub const fn lang_id_ff_adlm_sl() -> LangID {
  ID::new(26214, Some(1835820097), Some(19539)).into_lang_id()
}

/// ff-Adlm-SN: 𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤅𞤫𞤲𞤫𞤺𞤢𞥄𞤤
pub const fn lang_id_ff_adlm_sn() -> LangID {
  ID::new(26214, Some(1835820097), Some(20051)).into_lang_id()
}

/// ff-Latn: (ff-Latn-SN) Pulaar, Latn, Senegaal
pub const fn lang_id_ff_latn() -> LangID {
  ID::new(26214, Some(1853120844), None).into_lang_id()
}

/// ff-Latn-BF: Pulaar, Latn, Burkibaa Faaso
pub const fn lang_id_ff_latn_bf() -> LangID {
  ID::new(26214, Some(1853120844), Some(17986)).into_lang_id()
}

/// ff-Latn-CM: Pulaar, Latn, Kameruun
pub const fn lang_id_ff_latn_cm() -> LangID {
  ID::new(26214, Some(1853120844), Some(19779)).into_lang_id()
}

/// ff-Latn-GH: Pulaar, Latn, Ganaa
pub const fn lang_id_ff_latn_gh() -> LangID {
  ID::new(26214, Some(1853120844), Some(18503)).into_lang_id()
}

/// ff-Latn-GM: Pulaar, Latn, Gammbi
pub const fn lang_id_ff_latn_gm() -> LangID {
  ID::new(26214, Some(1853120844), Some(19783)).into_lang_id()
}

/// ff-Latn-GN: Pulaar, Latn, Gine
pub const fn lang_id_ff_latn_gn() -> LangID {
  ID::new(26214, Some(1853120844), Some(20039)).into_lang_id()
}

/// ff-Latn-GW: Pulaar, Latn, Gine-Bisaawo
pub const fn lang_id_ff_latn_gw() -> LangID {
  ID::new(26214, Some(1853120844), Some(22343)).into_lang_id()
}

/// ff-Latn-LR: Pulaar, Latn, Liberiyaa
pub const fn lang_id_ff_latn_lr() -> LangID {
  ID::new(26214, Some(1853120844), Some(21068)).into_lang_id()
}

/// ff-Latn-MR: Pulaar, Latn, Muritani
pub const fn lang_id_ff_latn_mr() -> LangID {
  ID::new(26214, Some(1853120844), Some(21069)).into_lang_id()
}

/// ff-Latn-NE: Pulaar, Latn, Nijeer
pub const fn lang_id_ff_latn_ne() -> LangID {
  ID::new(26214, Some(1853120844), Some(17742)).into_lang_id()
}

/// ff-Latn-NG: Pulaar, Latn, Nijeriyaa
pub const fn lang_id_ff_latn_ng() -> LangID {
  ID::new(26214, Some(1853120844), Some(18254)).into_lang_id()
}

/// ff-Latn-SL: Pulaar, Latn, Seraa liyon
pub const fn lang_id_ff_latn_sl() -> LangID {
  ID::new(26214, Some(1853120844), Some(19539)).into_lang_id()
}

/// fi: (fi-Latn-FI) suomi, latinalainen, Suomi
pub const fn lang_id_fi() -> LangID {
  ID::new(26982, None, None).into_lang_id()
}

/// fil: (fil-Latn-PH) Filipino, Latin, Pilipinas
pub const fn lang_id_fil() -> LangID {
  ID::new(7104870, None, None).into_lang_id()
}

/// fo: (fo-Latn-FO) føroyskt, latínskt, Føroyar
pub const fn lang_id_fo() -> LangID {
  ID::new(28518, None, None).into_lang_id()
}

/// fo-DK: (fo-Latn-DK) føroyskt, latínskt, Danmark
pub const fn lang_id_fo_dk() -> LangID {
  ID::new(28518, None, Some(19268)).into_lang_id()
}

/// fr: (fr-Latn-FR) français, latin, France
pub const fn lang_id_fr() -> LangID {
  ID::new(29286, None, None).into_lang_id()
}

/// fr-BE: (fr-Latn-BE) français, latin, Belgique
pub const fn lang_id_fr_be() -> LangID {
  ID::new(29286, None, Some(17730)).into_lang_id()
}

/// fr-BF: (fr-Latn-BF) français, latin, Burkina Faso
pub const fn lang_id_fr_bf() -> LangID {
  ID::new(29286, None, Some(17986)).into_lang_id()
}

/// fr-BI: (fr-Latn-BI) français, latin, Burundi
pub const fn lang_id_fr_bi() -> LangID {
  ID::new(29286, None, Some(18754)).into_lang_id()
}

/// fr-BJ: (fr-Latn-BJ) français, latin, Bénin
pub const fn lang_id_fr_bj() -> LangID {
  ID::new(29286, None, Some(19010)).into_lang_id()
}

/// fr-BL: (fr-Latn-BL) français, latin, Saint-Barthélemy
pub const fn lang_id_fr_bl() -> LangID {
  ID::new(29286, None, Some(19522)).into_lang_id()
}

/// fr-CA: (fr-Latn-CA) français, latin, Canada
pub const fn lang_id_fr_ca() -> LangID {
  ID::new(29286, None, Some(16707)).into_lang_id()
}

/// fr-CD: (fr-Latn-CD) français, latin, Congo-Kinshasa
pub const fn lang_id_fr_cd() -> LangID {
  ID::new(29286, None, Some(17475)).into_lang_id()
}

/// fr-CF: (fr-Latn-CF) français, latin, République centrafricaine
pub const fn lang_id_fr_cf() -> LangID {
  ID::new(29286, None, Some(17987)).into_lang_id()
}

/// fr-CG: (fr-Latn-CG) français, latin, Congo-Brazzaville
pub const fn lang_id_fr_cg() -> LangID {
  ID::new(29286, None, Some(18243)).into_lang_id()
}

/// fr-CH: (fr-Latn-CH) français, latin, Suisse
pub const fn lang_id_fr_ch() -> LangID {
  ID::new(29286, None, Some(18499)).into_lang_id()
}

/// fr-CI: (fr-Latn-CI) français, latin, Côte d’Ivoire
pub const fn lang_id_fr_ci() -> LangID {
  ID::new(29286, None, Some(18755)).into_lang_id()
}

/// fr-CM: (fr-Latn-CM) français, latin, Cameroun
pub const fn lang_id_fr_cm() -> LangID {
  ID::new(29286, None, Some(19779)).into_lang_id()
}

/// fr-DJ: (fr-Latn-DJ) français, latin, Djibouti
pub const fn lang_id_fr_dj() -> LangID {
  ID::new(29286, None, Some(19012)).into_lang_id()
}

/// fr-DZ: (fr-Latn-DZ) français, latin, Algérie
pub const fn lang_id_fr_dz() -> LangID {
  ID::new(29286, None, Some(23108)).into_lang_id()
}

/// fr-GA: (fr-Latn-GA) français, latin, Gabon
pub const fn lang_id_fr_ga() -> LangID {
  ID::new(29286, None, Some(16711)).into_lang_id()
}

/// fr-GF: (fr-Latn-GF) français, latin, Guyane française
pub const fn lang_id_fr_gf() -> LangID {
  ID::new(29286, None, Some(17991)).into_lang_id()
}

/// fr-GN: (fr-Latn-GN) français, latin, Guinée
pub const fn lang_id_fr_gn() -> LangID {
  ID::new(29286, None, Some(20039)).into_lang_id()
}

/// fr-GP: (fr-Latn-GP) français, latin, Guadeloupe
pub const fn lang_id_fr_gp() -> LangID {
  ID::new(29286, None, Some(20551)).into_lang_id()
}

/// fr-GQ: (fr-Latn-GQ) français, latin, Guinée équatoriale
pub const fn lang_id_fr_gq() -> LangID {
  ID::new(29286, None, Some(20807)).into_lang_id()
}

/// fr-HT: (fr-Latn-HT) français, latin, Haïti
pub const fn lang_id_fr_ht() -> LangID {
  ID::new(29286, None, Some(21576)).into_lang_id()
}

/// fr-KM: (fr-Latn-KM) français, latin, Comores
pub const fn lang_id_fr_km() -> LangID {
  ID::new(29286, None, Some(19787)).into_lang_id()
}

/// fr-LU: (fr-Latn-LU) français, latin, Luxembourg
pub const fn lang_id_fr_lu() -> LangID {
  ID::new(29286, None, Some(21836)).into_lang_id()
}

/// fr-MA: (fr-Latn-MA) français, latin, Maroc
pub const fn lang_id_fr_ma() -> LangID {
  ID::new(29286, None, Some(16717)).into_lang_id()
}

/// fr-MC: (fr-Latn-MC) français, latin, Monaco
pub const fn lang_id_fr_mc() -> LangID {
  ID::new(29286, None, Some(17229)).into_lang_id()
}

/// fr-MF: (fr-Latn-MF) français, latin, Saint-Martin
pub const fn lang_id_fr_mf() -> LangID {
  ID::new(29286, None, Some(17997)).into_lang_id()
}

/// fr-MG: (fr-Latn-MG) français, latin, Madagascar
pub const fn lang_id_fr_mg() -> LangID {
  ID::new(29286, None, Some(18253)).into_lang_id()
}

/// fr-ML: (fr-Latn-ML) français, latin, Mali
pub const fn lang_id_fr_ml() -> LangID {
  ID::new(29286, None, Some(19533)).into_lang_id()
}

/// fr-MQ: (fr-Latn-MQ) français, latin, Martinique
pub const fn lang_id_fr_mq() -> LangID {
  ID::new(29286, None, Some(20813)).into_lang_id()
}

/// fr-MR: (fr-Latn-MR) français, latin, Mauritanie
pub const fn lang_id_fr_mr() -> LangID {
  ID::new(29286, None, Some(21069)).into_lang_id()
}

/// fr-MU: (fr-Latn-MU) français, latin, Maurice
pub const fn lang_id_fr_mu() -> LangID {
  ID::new(29286, None, Some(21837)).into_lang_id()
}

/// fr-NC: (fr-Latn-NC) français, latin, Nouvelle-Calédonie
pub const fn lang_id_fr_nc() -> LangID {
  ID::new(29286, None, Some(17230)).into_lang_id()
}

/// fr-NE: (fr-Latn-NE) français, latin, Niger
pub const fn lang_id_fr_ne() -> LangID {
  ID::new(29286, None, Some(17742)).into_lang_id()
}

/// fr-PF: (fr-Latn-PF) français, latin, Polynésie française
pub const fn lang_id_fr_pf() -> LangID {
  ID::new(29286, None, Some(18000)).into_lang_id()
}

/// fr-PM: (fr-Latn-PM) français, latin, Saint-Pierre-et-Miquelon
pub const fn lang_id_fr_pm() -> LangID {
  ID::new(29286, None, Some(19792)).into_lang_id()
}

/// fr-RE: (fr-Latn-RE) français, latin, La Réunion
pub const fn lang_id_fr_re() -> LangID {
  ID::new(29286, None, Some(17746)).into_lang_id()
}

/// fr-RW: (fr-Latn-RW) français, latin, Rwanda
pub const fn lang_id_fr_rw() -> LangID {
  ID::new(29286, None, Some(22354)).into_lang_id()
}

/// fr-SC: (fr-Latn-SC) français, latin, Seychelles
pub const fn lang_id_fr_sc() -> LangID {
  ID::new(29286, None, Some(17235)).into_lang_id()
}

/// fr-SN: (fr-Latn-SN) français, latin, Sénégal
pub const fn lang_id_fr_sn() -> LangID {
  ID::new(29286, None, Some(20051)).into_lang_id()
}

/// fr-SY: (fr-Latn-SY) français, latin, Syrie
pub const fn lang_id_fr_sy() -> LangID {
  ID::new(29286, None, Some(22867)).into_lang_id()
}

/// fr-TD: (fr-Latn-TD) français, latin, Tchad
pub const fn lang_id_fr_td() -> LangID {
  ID::new(29286, None, Some(17492)).into_lang_id()
}

/// fr-TG: (fr-Latn-TG) français, latin, Togo
pub const fn lang_id_fr_tg() -> LangID {
  ID::new(29286, None, Some(18260)).into_lang_id()
}

/// fr-TN: (fr-Latn-TN) français, latin, Tunisie
pub const fn lang_id_fr_tn() -> LangID {
  ID::new(29286, None, Some(20052)).into_lang_id()
}

/// fr-VU: (fr-Latn-VU) français, latin, Vanuatu
pub const fn lang_id_fr_vu() -> LangID {
  ID::new(29286, None, Some(21846)).into_lang_id()
}

/// fr-WF: (fr-Latn-WF) français, latin, Wallis-et-Futuna
pub const fn lang_id_fr_wf() -> LangID {
  ID::new(29286, None, Some(18007)).into_lang_id()
}

/// fr-YT: (fr-Latn-YT) français, latin, Mayotte
pub const fn lang_id_fr_yt() -> LangID {
  ID::new(29286, None, Some(21593)).into_lang_id()
}

/// frr: (frr-Latn-DE) Nordfriisk, Latn, DE
pub const fn lang_id_frr() -> LangID {
  ID::new(7500390, None, None).into_lang_id()
}

/// fur: (fur-Latn-IT) furlan, latin, Italie
pub const fn lang_id_fur() -> LangID {
  ID::new(7501158, None, None).into_lang_id()
}

/// fy: (fy-Latn-NL) Frysk, Latyn, Nederlân
pub const fn lang_id_fy() -> LangID {
  ID::new(31078, None, None).into_lang_id()
}

/// ga: (ga-Latn-IE) Gaeilge, Laidineach, Éire
pub const fn lang_id_ga() -> LangID {
  ID::new(24935, None, None).into_lang_id()
}

/// ga-GB: (ga-Latn-GB) Gaeilge, Laidineach, an Ríocht Aontaithe
pub const fn lang_id_ga_gb() -> LangID {
  ID::new(24935, None, Some(16967)).into_lang_id()
}

/// gd: (gd-Latn-GB) Gàidhlig, Laideann, An Rìoghachd Aonaichte
pub const fn lang_id_gd() -> LangID {
  ID::new(25703, None, None).into_lang_id()
}

/// gl: (gl-Latn-ES) galego, latino, España
pub const fn lang_id_gl() -> LangID {
  ID::new(27751, None, None).into_lang_id()
}

/// gsw: (gsw-Latn-CH) Schwiizertüütsch, Latiinisch, Schwiiz
pub const fn lang_id_gsw() -> LangID {
  ID::new(7828327, None, None).into_lang_id()
}

/// gsw-FR: (gsw-Latn-FR) Schwiizertüütsch, Latiinisch, Frankriich
pub const fn lang_id_gsw_fr() -> LangID {
  ID::new(7828327, None, Some(21062)).into_lang_id()
}

/// gsw-LI: (gsw-Latn-LI) Schwiizertüütsch, Latiinisch, Liächteschtäi
pub const fn lang_id_gsw_li() -> LangID {
  ID::new(7828327, None, Some(18764)).into_lang_id()
}

/// gu: (gu-Gujr-IN) ગુજરાતી, ગુજરાતી, ભારત
pub const fn lang_id_gu() -> LangID {
  ID::new(30055, None, None).into_lang_id()
}

/// guz: (guz-Latn-KE) Ekegusii, Latn, Kenya
pub const fn lang_id_guz() -> LangID {
  ID::new(8025447, None, None).into_lang_id()
}

/// gv: (gv-Latn-IM) Gaelg, Latn, Ellan Vannin
pub const fn lang_id_gv() -> LangID {
  ID::new(30311, None, None).into_lang_id()
}

/// ha: (ha-Latn-NG) Hausa, Latin, Nijeriya
pub const fn lang_id_ha() -> LangID {
  ID::new(24936, None, None).into_lang_id()
}

/// ha-GH: (ha-Latn-GH) Hausa, Latin, Gana
pub const fn lang_id_ha_gh() -> LangID {
  ID::new(24936, None, Some(18503)).into_lang_id()
}

/// ha-NE: (ha-Latn-NE) Hausa, Latin, Nijar
pub const fn lang_id_ha_ne() -> LangID {
  ID::new(24936, None, Some(17742)).into_lang_id()
}

/// haw: (haw-Latn-US) ʻŌlelo Hawaiʻi, Latn, ʻAmelika Hui Pū ʻIa
pub const fn lang_id_haw() -> LangID {
  ID::new(7823720, None, None).into_lang_id()
}

/// he: (he-Hebr-IL) עברית, עברי, ישראל
pub const fn lang_id_he() -> LangID {
  ID::new(25960, None, None).into_lang_id()
}

/// hi: (hi-Deva-IN) हिन्दी, देवनागरी, भारत
pub const fn lang_id_hi() -> LangID {
  ID::new(26984, None, None).into_lang_id()
}

/// hi-Latn: (hi-Latn-IN) Hindi (Latin), India
pub const fn lang_id_hi_latn() -> LangID {
  ID::new(26984, Some(1853120844), None).into_lang_id()
}

/// hr: (hr-Latn-HR) hrvatski, latinica, Hrvatska
pub const fn lang_id_hr() -> LangID {
  ID::new(29288, None, None).into_lang_id()
}

/// hr-BA: (hr-Latn-BA) hrvatski, latinica, Bosna i Hercegovina
pub const fn lang_id_hr_ba() -> LangID {
  ID::new(29288, None, Some(16706)).into_lang_id()
}

/// hsb: (hsb-Latn-DE) hornjoserbšćina, łaćonsce, Němska
pub const fn lang_id_hsb() -> LangID {
  ID::new(6452072, None, None).into_lang_id()
}

/// hu: (hu-Latn-HU) magyar, Latin, Magyarország
pub const fn lang_id_hu() -> LangID {
  ID::new(30056, None, None).into_lang_id()
}

/// hy: (hy-Armn-AM) հայերեն, հայկական, Հայաստան
pub const fn lang_id_hy() -> LangID {
  ID::new(31080, None, None).into_lang_id()
}

/// ia: (ia-Latn-001) interlingua, latin, Mundo
pub const fn lang_id_ia() -> LangID {
  ID::new(24937, None, None).into_lang_id()
}

/// id: (id-Latn-ID) Indonesia, Latin, Indonesia
pub const fn lang_id_id() -> LangID {
  ID::new(25705, None, None).into_lang_id()
}

/// ig: (ig-Latn-NG) Igbo, Latin, Naịjịrịa
pub const fn lang_id_ig() -> LangID {
  ID::new(26473, None, None).into_lang_id()
}

/// ii: (ii-Yiii-CN) ꆈꌠꉙ, ꆈꌠꁱꂷ, ꍏꇩ
pub const fn lang_id_ii() -> LangID {
  ID::new(26985, None, None).into_lang_id()
}

/// is: (is-Latn-IS) íslenska, latneskt, Ísland
pub const fn lang_id_is() -> LangID {
  ID::new(29545, None, None).into_lang_id()
}

/// it: (it-Latn-IT) italiano, latino, Italia
pub const fn lang_id_it() -> LangID {
  ID::new(29801, None, None).into_lang_id()
}

/// it-CH: (it-Latn-CH) italiano, latino, Svizzera
pub const fn lang_id_it_ch() -> LangID {
  ID::new(29801, None, Some(18499)).into_lang_id()
}

/// it-SM: (it-Latn-SM) italiano, latino, San Marino
pub const fn lang_id_it_sm() -> LangID {
  ID::new(29801, None, Some(19795)).into_lang_id()
}

/// it-VA: (it-Latn-VA) italiano, latino, Città del Vaticano
pub const fn lang_id_it_va() -> LangID {
  ID::new(29801, None, Some(16726)).into_lang_id()
}

/// ja: (ja-Jpan-JP) 日本語, 日本語の文字, 日本
pub const fn lang_id_ja() -> LangID {
  ID::new(24938, None, None).into_lang_id()
}

/// jgo: (jgo-Latn-CM) Ndaꞌa, mík -ŋwaꞌnɛ yi ɛ́ líŋɛ́nɛ Latɛ̂ŋ, Kamɛlûn
pub const fn lang_id_jgo() -> LangID {
  ID::new(7300970, None, None).into_lang_id()
}

/// jmc: (jmc-Latn-TZ) Kimachame, Latn, Tanzania
pub const fn lang_id_jmc() -> LangID {
  ID::new(6516074, None, None).into_lang_id()
}

/// jv: (jv-Latn-ID) Jawa, Latin, Indonésia
pub const fn lang_id_jv() -> LangID {
  ID::new(30314, None, None).into_lang_id()
}

/// ka: (ka-Geor-GE) ქართული, ქართული, საქართველო
pub const fn lang_id_ka() -> LangID {
  ID::new(24939, None, None).into_lang_id()
}

/// kab: (kab-Latn-DZ) Taqbaylit, Latn, Lezzayer
pub const fn lang_id_kab() -> LangID {
  ID::new(6447467, None, None).into_lang_id()
}

/// kam: (kam-Latn-KE) Kikamba, Latn, Kenya
pub const fn lang_id_kam() -> LangID {
  ID::new(7168363, None, None).into_lang_id()
}

/// kde: (kde-Latn-TZ) Chimakonde, Latn, Tanzania
pub const fn lang_id_kde() -> LangID {
  ID::new(6644843, None, None).into_lang_id()
}

/// kea: (kea-Latn-CV) kabuverdianu, latinu, Kabu Verdi
pub const fn lang_id_kea() -> LangID {
  ID::new(6382955, None, None).into_lang_id()
}

/// kgp: (kgp-Latn-BR) kanhgág, ratĩnh, Mrasir
pub const fn lang_id_kgp() -> LangID {
  ID::new(7366507, None, None).into_lang_id()
}

/// khq: (khq-Latn-ML) Koyra ciini, Latn, Maali
pub const fn lang_id_khq() -> LangID {
  ID::new(7432299, None, None).into_lang_id()
}

/// ki: (ki-Latn-KE) Gikuyu, Latn, Kenya
pub const fn lang_id_ki() -> LangID {
  ID::new(26987, None, None).into_lang_id()
}

/// kk: (kk-Cyrl-KZ) қазақ тілі, кирилл жазуы, Қазақстан
pub const fn lang_id_kk() -> LangID {
  ID::new(27499, None, None).into_lang_id()
}

/// kkj: (kkj-Latn-CM) kakɔ, Latn, Kamɛrun
pub const fn lang_id_kkj() -> LangID {
  ID::new(6974315, None, None).into_lang_id()
}

/// kl: (kl-Latn-GL) kalaallisut, Latn, Kalaallit Nunaat
pub const fn lang_id_kl() -> LangID {
  ID::new(27755, None, None).into_lang_id()
}

/// kln: (kln-Latn-KE) Kalenjin, Latn, Emetab Kenya
pub const fn lang_id_kln() -> LangID {
  ID::new(7236715, None, None).into_lang_id()
}

/// km: (km-Khmr-KH) ខ្មែរ, ខ្មែរ, កម្ពុជា
pub const fn lang_id_km() -> LangID {
  ID::new(28011, None, None).into_lang_id()
}

/// kn: (kn-Knda-IN) ಕನ್ನಡ, ಕನ್ನಡ, ಭಾರತ
pub const fn lang_id_kn() -> LangID {
  ID::new(28267, None, None).into_lang_id()
}

/// ko: (ko-Kore-KR) 한국어, 한국 문자, 대한민국
pub const fn lang_id_ko() -> LangID {
  ID::new(28523, None, None).into_lang_id()
}

/// ko-KP: (ko-Kore-KP) 한국어, 한국 문자, 조선민주주의인민공화국
pub const fn lang_id_ko_kp() -> LangID {
  ID::new(28523, None, Some(20555)).into_lang_id()
}

/// kok: (kok-Deva-IN) कोंकणी, देवनागरी, भारत
pub const fn lang_id_kok() -> LangID {
  ID::new(7040875, None, None).into_lang_id()
}

/// ks: (ks-Arab-IN) کٲشُر, عربی, ہِندوستان
pub const fn lang_id_ks() -> LangID {
  ID::new(29547, None, None).into_lang_id()
}

/// ks-Arab: (ks-Arab-IN) کٲشُر, عربی, ہِندوستان
pub const fn lang_id_ks_arab() -> LangID {
  ID::new(29547, Some(1650553409), None).into_lang_id()
}

/// ks-Deva: (ks-Deva-IN) कॉशुर, देवनागरी, हिंदोस्तान
pub const fn lang_id_ks_deva() -> LangID {
  ID::new(29547, Some(1635149124), None).into_lang_id()
}

/// ksb: (ksb-Latn-TZ) Kishambaa, Latn, Tanzania
pub const fn lang_id_ksb() -> LangID {
  ID::new(6452075, None, None).into_lang_id()
}

/// ksf: (ksf-Latn-CM) rikpa, Latn, kamɛrún
pub const fn lang_id_ksf() -> LangID {
  ID::new(6714219, None, None).into_lang_id()
}

/// ksh: (ksh-Latn-DE) Kölsch, lateinesche Schreff, Doütschland
pub const fn lang_id_ksh() -> LangID {
  ID::new(6845291, None, None).into_lang_id()
}

/// ku: (ku-Latn-TR) kurdî, latînî, Tirkiye
pub const fn lang_id_ku() -> LangID {
  ID::new(30059, None, None).into_lang_id()
}

/// kw: (kw-Latn-GB) kernewek, Latn, Rywvaneth Unys
pub const fn lang_id_kw() -> LangID {
  ID::new(30571, None, None).into_lang_id()
}

/// ky: (ky-Cyrl-KG) кыргызча, Кирилл, Кыргызстан
pub const fn lang_id_ky() -> LangID {
  ID::new(31083, None, None).into_lang_id()
}

/// lag: (lag-Latn-TZ) Kɨlaangi, Latn, Taansanía
pub const fn lang_id_lag() -> LangID {
  ID::new(6775148, None, None).into_lang_id()
}

/// lb: (lb-Latn-LU) Lëtzebuergesch, Laténgesch, Lëtzebuerg
pub const fn lang_id_lb() -> LangID {
  ID::new(25196, None, None).into_lang_id()
}

/// lg: (lg-Latn-UG) Luganda, Latn, Yuganda
pub const fn lang_id_lg() -> LangID {
  ID::new(26476, None, None).into_lang_id()
}

/// lkt: (lkt-Latn-US) Lakȟólʼiyapi, Latn, Mílahaŋska Tȟamákȟočhe
pub const fn lang_id_lkt() -> LangID {
  ID::new(7629676, None, None).into_lang_id()
}

/// ln: (ln-Latn-CD) lingála, Latn, Republíki ya Kongó Demokratíki
pub const fn lang_id_ln() -> LangID {
  ID::new(28268, None, None).into_lang_id()
}

/// ln-AO: (ln-Latn-AO) lingála, Latn, Angóla
pub const fn lang_id_ln_ao() -> LangID {
  ID::new(28268, None, Some(20289)).into_lang_id()
}

/// ln-CF: (ln-Latn-CF) lingála, Latn, Repibiki ya Afríka ya Káti
pub const fn lang_id_ln_cf() -> LangID {
  ID::new(28268, None, Some(17987)).into_lang_id()
}

/// ln-CG: (ln-Latn-CG) lingála, Latn, Kongo
pub const fn lang_id_ln_cg() -> LangID {
  ID::new(28268, None, Some(18243)).into_lang_id()
}

/// lo: (lo-Laoo-LA) ລາວ, ລາວ, ລາວ
pub const fn lang_id_lo() -> LangID {
  ID::new(28524, None, None).into_lang_id()
}

/// lrc: (lrc-Arab-IR) لۊری شومالی, عأرأڤی, IR
pub const fn lang_id_lrc() -> LangID {
  ID::new(6517356, None, None).into_lang_id()
}

/// lrc-IQ: (lrc-Arab-IQ) لۊری شومالی, عأرأڤی, IQ
pub const fn lang_id_lrc_iq() -> LangID {
  ID::new(6517356, None, Some(20809)).into_lang_id()
}

/// lt: (lt-Latn-LT) lietuvių, lotynų, Lietuva
pub const fn lang_id_lt() -> LangID {
  ID::new(29804, None, None).into_lang_id()
}

/// lu: (lu-Latn-CD) Tshiluba, Latn, Ditunga wa Kongu
pub const fn lang_id_lu() -> LangID {
  ID::new(30060, None, None).into_lang_id()
}

/// luo: (luo-Latn-KE) Dholuo, Latn, Kenya
pub const fn lang_id_luo() -> LangID {
  ID::new(7304556, None, None).into_lang_id()
}

/// luy: (luy-Latn-KE) Luluhia, Latn, Kenya
pub const fn lang_id_luy() -> LangID {
  ID::new(7959916, None, None).into_lang_id()
}

/// lv: (lv-Latn-LV) latviešu, latīņu, Latvija
pub const fn lang_id_lv() -> LangID {
  ID::new(30316, None, None).into_lang_id()
}

/// mai: (mai-Deva-IN) मैथिली, देवनागरी, भारत
pub const fn lang_id_mai() -> LangID {
  ID::new(6906221, None, None).into_lang_id()
}

/// mas: (mas-Latn-KE) Maa, Latn, Kenya
pub const fn lang_id_mas() -> LangID {
  ID::new(7561581, None, None).into_lang_id()
}

/// mas-TZ: (mas-Latn-TZ) Maa, Latn, Tansania
pub const fn lang_id_mas_tz() -> LangID {
  ID::new(7561581, None, Some(23124)).into_lang_id()
}

/// mdf: (mdf-Cyrl-RU) мокшень кяль, Cyrl, RU
pub const fn lang_id_mdf() -> LangID {
  ID::new(6710381, None, None).into_lang_id()
}

/// mer: (mer-Latn-KE) Kĩmĩrũ, Latn, Kenya
pub const fn lang_id_mer() -> LangID {
  ID::new(7497069, None, None).into_lang_id()
}

/// mfe: (mfe-Latn-MU) kreol morisien, Latn, Moris
pub const fn lang_id_mfe() -> LangID {
  ID::new(6645357, None, None).into_lang_id()
}

/// mg: (mg-Latn-MG) Malagasy, Latn, Madagasikara
pub const fn lang_id_mg() -> LangID {
  ID::new(26477, None, None).into_lang_id()
}

/// mgh: (mgh-Latn-MZ) Makua, Latn, Umozambiki
pub const fn lang_id_mgh() -> LangID {
  ID::new(6842221, None, None).into_lang_id()
}

/// mgo: (mgo-Latn-CM) metaʼ, ngam ŋwaʼri, Kamalun
pub const fn lang_id_mgo() -> LangID {
  ID::new(7300973, None, None).into_lang_id()
}

/// mi: (mi-Latn-NZ) Māori, Rātina, Aotearoa
pub const fn lang_id_mi() -> LangID {
  ID::new(26989, None, None).into_lang_id()
}

/// mk: (mk-Cyrl-MK) македонски, кирилско писмо, Северна Македонија
pub const fn lang_id_mk() -> LangID {
  ID::new(27501, None, None).into_lang_id()
}

/// ml: (ml-Mlym-IN) മലയാളം, മലയാളം, ഇന്ത്യ
pub const fn lang_id_ml() -> LangID {
  ID::new(27757, None, None).into_lang_id()
}

/// mn: (mn-Cyrl-MN) монгол, кирилл, Монгол
pub const fn lang_id_mn() -> LangID {
  ID::new(28269, None, None).into_lang_id()
}

/// mni: (mni-Beng-IN) মৈতৈলোন্, বাংলা, ইন্দিয়া
pub const fn lang_id_mni() -> LangID {
  ID::new(6909549, None, None).into_lang_id()
}

/// mni-Beng: (mni-Beng-IN) মৈতৈলোন্, বাংলা, ইন্দিয়া
pub const fn lang_id_mni_beng() -> LangID {
  ID::new(6909549, Some(1735288130), None).into_lang_id()
}

/// mr: (mr-Deva-IN) मराठी, देवनागरी, भारत
pub const fn lang_id_mr() -> LangID {
  ID::new(29293, None, None).into_lang_id()
}

/// ms: (ms-Latn-MY) Melayu, Latin, Malaysia
pub const fn lang_id_ms() -> LangID {
  ID::new(29549, None, None).into_lang_id()
}

/// ms-BN: (ms-Latn-BN) Melayu, Latin, Brunei
pub const fn lang_id_ms_bn() -> LangID {
  ID::new(29549, None, Some(20034)).into_lang_id()
}

/// ms-ID: (ms-Latn-ID) Melayu, Latin, Indonesia
pub const fn lang_id_ms_id() -> LangID {
  ID::new(29549, None, Some(17481)).into_lang_id()
}

/// ms-SG: (ms-Latn-SG) Melayu, Latin, Singapura
pub const fn lang_id_ms_sg() -> LangID {
  ID::new(29549, None, Some(18259)).into_lang_id()
}

/// mt: (mt-Latn-MT) Malti, Latin, Malta
pub const fn lang_id_mt() -> LangID {
  ID::new(29805, None, None).into_lang_id()
}

/// mua: (mua-Latn-CM) MUNDAŊ, Latn, kameruŋ
pub const fn lang_id_mua() -> LangID {
  ID::new(6387053, None, None).into_lang_id()
}

/// my: (my-Mymr-MM) မြန်မာ, မြန်မာ, မြန်မာ
pub const fn lang_id_my() -> LangID {
  ID::new(31085, None, None).into_lang_id()
}

/// mzn: (mzn-Arab-IR) مازرونی, عربی, ایران
pub const fn lang_id_mzn() -> LangID {
  ID::new(7240301, None, None).into_lang_id()
}

/// naq: (naq-Latn-NA) Khoekhoegowab, Latn, Namibiab
pub const fn lang_id_naq() -> LangID {
  ID::new(7430510, None, None).into_lang_id()
}

/// nb: (nb-Latn-NO) norsk bokmål, latinsk, Norge
pub const fn lang_id_nb() -> LangID {
  ID::new(25198, None, None).into_lang_id()
}

/// nb-SJ: (nb-Latn-SJ) norsk bokmål, latinsk, Svalbard og Jan Mayen
pub const fn lang_id_nb_sj() -> LangID {
  ID::new(25198, None, Some(19027)).into_lang_id()
}

/// nd: (nd-Latn-ZW) isiNdebele, Latn, Zimbabwe
pub const fn lang_id_nd() -> LangID {
  ID::new(25710, None, None).into_lang_id()
}

/// nds: (nds-Latn-DE) nds, Latn, DE
pub const fn lang_id_nds() -> LangID {
  ID::new(7562350, None, None).into_lang_id()
}

/// nds-NL: (nds-Latn-NL) nds, Latn, NL
pub const fn lang_id_nds_nl() -> LangID {
  ID::new(7562350, None, Some(19534)).into_lang_id()
}

/// ne: (ne-Deva-NP) नेपाली, देवानागरी, नेपाल
pub const fn lang_id_ne() -> LangID {
  ID::new(25966, None, None).into_lang_id()
}

/// ne-IN: (ne-Deva-IN) नेपाली, देवानागरी, भारत
pub const fn lang_id_ne_in() -> LangID {
  ID::new(25966, None, Some(20041)).into_lang_id()
}

/// nl: (nl-Latn-NL) Nederlands, Latijns, Nederland
pub const fn lang_id_nl() -> LangID {
  ID::new(27758, None, None).into_lang_id()
}

/// nl-AW: (nl-Latn-AW) Nederlands, Latijns, Aruba
pub const fn lang_id_nl_aw() -> LangID {
  ID::new(27758, None, Some(22337)).into_lang_id()
}

/// nl-BE: (nl-Latn-BE) Nederlands, Latijns, België
pub const fn lang_id_nl_be() -> LangID {
  ID::new(27758, None, Some(17730)).into_lang_id()
}

/// nl-BQ: (nl-Latn-BQ) Nederlands, Latijns, Caribisch Nederland
pub const fn lang_id_nl_bq() -> LangID {
  ID::new(27758, None, Some(20802)).into_lang_id()
}

/// nl-CW: (nl-Latn-CW) Nederlands, Latijns, Curaçao
pub const fn lang_id_nl_cw() -> LangID {
  ID::new(27758, None, Some(22339)).into_lang_id()
}

/// nl-SR: (nl-Latn-SR) Nederlands, Latijns, Suriname
pub const fn lang_id_nl_sr() -> LangID {
  ID::new(27758, None, Some(21075)).into_lang_id()
}

/// nl-SX: (nl-Latn-SX) Nederlands, Latijns, Sint-Maarten
pub const fn lang_id_nl_sx() -> LangID {
  ID::new(27758, None, Some(22611)).into_lang_id()
}

/// nmg: (nmg-Latn-CM) nmg, Latn, Kamerun
pub const fn lang_id_nmg() -> LangID {
  ID::new(6778222, None, None).into_lang_id()
}

/// nn: (nn-Latn-NO) norsk nynorsk, latinsk, Noreg
pub const fn lang_id_nn() -> LangID {
  ID::new(28270, None, None).into_lang_id()
}

/// nnh: (nnh-Latn-CM) Shwóŋò ngiembɔɔn, Latn, Kàmalûm
pub const fn lang_id_nnh() -> LangID {
  ID::new(6844014, None, None).into_lang_id()
}

/// no: (no-Latn-NO) norsk, latinsk, Norge
pub const fn lang_id_no() -> LangID {
  ID::new(28526, None, None).into_lang_id()
}

/// nus: (nus-Latn-SS) Thok Nath, Latn, SS
pub const fn lang_id_nus() -> LangID {
  ID::new(7566702, None, None).into_lang_id()
}

/// nyn: (nyn-Latn-UG) Runyankore, Latn, Uganda
pub const fn lang_id_nyn() -> LangID {
  ID::new(7240046, None, None).into_lang_id()
}

/// oc: (oc-Latn-FR) oc, Latn, FR
pub const fn lang_id_oc() -> LangID {
  ID::new(25455, None, None).into_lang_id()
}

/// oc-ES: (oc-Latn-ES) oc, Latn, ES
pub const fn lang_id_oc_es() -> LangID {
  ID::new(25455, None, Some(21317)).into_lang_id()
}

/// om: (om-Latn-ET) Oromoo, Latin, Itoophiyaa
pub const fn lang_id_om() -> LangID {
  ID::new(28015, None, None).into_lang_id()
}

/// om-KE: (om-Latn-KE) Oromoo, Latin, Keeniyaa
pub const fn lang_id_om_ke() -> LangID {
  ID::new(28015, None, Some(17739)).into_lang_id()
}

/// or: (or-Orya-IN) ଓଡ଼ିଆ, ଓଡ଼ିଆ, ଭାରତ
pub const fn lang_id_or() -> LangID {
  ID::new(29295, None, None).into_lang_id()
}

/// os: (os-Cyrl-GE) ирон, Киррилицӕ, Гуырдзыстон
pub const fn lang_id_os() -> LangID {
  ID::new(29551, None, None).into_lang_id()
}

/// os-RU: (os-Cyrl-RU) ирон, Киррилицӕ, Уӕрӕсе
pub const fn lang_id_os_ru() -> LangID {
  ID::new(29551, None, Some(21842)).into_lang_id()
}

/// pa: (pa-Guru-IN) ਪੰਜਾਬੀ, ਗੁਰਮੁਖੀ, ਭਾਰਤ
pub const fn lang_id_pa() -> LangID {
  ID::new(24944, None, None).into_lang_id()
}

/// pa-Arab: (pa-Arab-PK) پنجابی, عربی, پاکستان
pub const fn lang_id_pa_arab() -> LangID {
  ID::new(24944, Some(1650553409), None).into_lang_id()
}

/// pa-Guru: (pa-Guru-IN) ਪੰਜਾਬੀ, ਗੁਰਮੁਖੀ, ਭਾਰਤ
pub const fn lang_id_pa_guru() -> LangID {
  ID::new(24944, Some(1970435399), None).into_lang_id()
}

/// pcm: (pcm-Latn-NG) Naijíriá Píjin, Látin, Naijíria
pub const fn lang_id_pcm() -> LangID {
  ID::new(7168880, None, None).into_lang_id()
}

/// pis: (pis-Latn-SB) Pijin, Latin, Solomon Aelan
pub const fn lang_id_pis() -> LangID {
  ID::new(7563632, None, None).into_lang_id()
}

/// pl: (pl-Latn-PL) polski, łacińskie, Polska
pub const fn lang_id_pl() -> LangID {
  ID::new(27760, None, None).into_lang_id()
}

/// ps: (ps-Arab-AF) پښتو, عربي, افغانستان
pub const fn lang_id_ps() -> LangID {
  ID::new(29552, None, None).into_lang_id()
}

/// ps-PK: (ps-Arab-PK) پښتو, عربي, پاکستان
pub const fn lang_id_ps_pk() -> LangID {
  ID::new(29552, None, Some(19280)).into_lang_id()
}

/// pt: (pt-Latn-BR) português, latim, Brasil
pub const fn lang_id_pt() -> LangID {
  ID::new(29808, None, None).into_lang_id()
}

/// pt-AO: (pt-Latn-AO) português, latim, Angola
pub const fn lang_id_pt_ao() -> LangID {
  ID::new(29808, None, Some(20289)).into_lang_id()
}

/// pt-CH: (pt-Latn-CH) português, latim, Suíça
pub const fn lang_id_pt_ch() -> LangID {
  ID::new(29808, None, Some(18499)).into_lang_id()
}

/// pt-CV: (pt-Latn-CV) português, latim, Cabo Verde
pub const fn lang_id_pt_cv() -> LangID {
  ID::new(29808, None, Some(22083)).into_lang_id()
}

/// pt-GQ: (pt-Latn-GQ) português, latim, Guiné Equatorial
pub const fn lang_id_pt_gq() -> LangID {
  ID::new(29808, None, Some(20807)).into_lang_id()
}

/// pt-GW: (pt-Latn-GW) português, latim, Guiné-Bissau
pub const fn lang_id_pt_gw() -> LangID {
  ID::new(29808, None, Some(22343)).into_lang_id()
}

/// pt-LU: (pt-Latn-LU) português, latim, Luxemburgo
pub const fn lang_id_pt_lu() -> LangID {
  ID::new(29808, None, Some(21836)).into_lang_id()
}

/// pt-MO: (pt-Latn-MO) português, latim, Macau, RAE da China
pub const fn lang_id_pt_mo() -> LangID {
  ID::new(29808, None, Some(20301)).into_lang_id()
}

/// pt-MZ: (pt-Latn-MZ) português, latim, Moçambique
pub const fn lang_id_pt_mz() -> LangID {
  ID::new(29808, None, Some(23117)).into_lang_id()
}

/// pt-PT: (pt-Latn-PT) português, latim, Portugal
pub const fn lang_id_pt_pt() -> LangID {
  ID::new(29808, None, Some(21584)).into_lang_id()
}

/// pt-ST: (pt-Latn-ST) português, latim, São Tomé e Príncipe
pub const fn lang_id_pt_st() -> LangID {
  ID::new(29808, None, Some(21587)).into_lang_id()
}

/// pt-TL: (pt-Latn-TL) português, latim, Timor-Leste
pub const fn lang_id_pt_tl() -> LangID {
  ID::new(29808, None, Some(19540)).into_lang_id()
}

/// qu: (qu-Latn-PE) Runasimi, Latin Simi, Perú
pub const fn lang_id_qu() -> LangID {
  ID::new(30065, None, None).into_lang_id()
}

/// qu-BO: (qu-Latn-BO) Runasimi, Latin Simi, Bolivia
pub const fn lang_id_qu_bo() -> LangID {
  ID::new(30065, None, Some(20290)).into_lang_id()
}

/// qu-EC: (qu-Latn-EC) Runasimi, Latin Simi, Ecuador
pub const fn lang_id_qu_ec() -> LangID {
  ID::new(30065, None, Some(17221)).into_lang_id()
}

/// raj: (raj-Deva-IN) राजस्थानी, देवनागरी, भारत
pub const fn lang_id_raj() -> LangID {
  ID::new(6971762, None, None).into_lang_id()
}

/// rm: (rm-Latn-CH) rumantsch, latin, Svizra
pub const fn lang_id_rm() -> LangID {
  ID::new(28018, None, None).into_lang_id()
}

/// rn: (rn-Latn-BI) Ikirundi, Latn, Uburundi
pub const fn lang_id_rn() -> LangID {
  ID::new(28274, None, None).into_lang_id()
}

/// ro: (ro-Latn-RO) română, latină, România
pub const fn lang_id_ro() -> LangID {
  ID::new(28530, None, None).into_lang_id()
}

/// ro-MD: (ro-Latn-MD) română, latină, Republica Moldova
pub const fn lang_id_ro_md() -> LangID {
  ID::new(28530, None, Some(17485)).into_lang_id()
}

/// rof: (rof-Latn-TZ) Kihorombo, Latn, Tanzania
pub const fn lang_id_rof() -> LangID {
  ID::new(6713202, None, None).into_lang_id()
}

/// ru: (ru-Cyrl-RU) русский, кириллица, Россия
pub const fn lang_id_ru() -> LangID {
  ID::new(30066, None, None).into_lang_id()
}

/// ru-BY: (ru-Cyrl-BY) русский, кириллица, Беларусь
pub const fn lang_id_ru_by() -> LangID {
  ID::new(30066, None, Some(22850)).into_lang_id()
}

/// ru-KG: (ru-Cyrl-KG) русский, кириллица, Киргизия
pub const fn lang_id_ru_kg() -> LangID {
  ID::new(30066, None, Some(18251)).into_lang_id()
}

/// ru-KZ: (ru-Cyrl-KZ) русский, кириллица, Казахстан
pub const fn lang_id_ru_kz() -> LangID {
  ID::new(30066, None, Some(23115)).into_lang_id()
}

/// ru-MD: (ru-Cyrl-MD) русский, кириллица, Молдова
pub const fn lang_id_ru_md() -> LangID {
  ID::new(30066, None, Some(17485)).into_lang_id()
}

/// ru-UA: (ru-Cyrl-UA) русский, кириллица, Украина
pub const fn lang_id_ru_ua() -> LangID {
  ID::new(30066, None, Some(16725)).into_lang_id()
}

/// rw: (rw-Latn-RW) Kinyarwanda, Latn, U Rwanda
pub const fn lang_id_rw() -> LangID {
  ID::new(30578, None, None).into_lang_id()
}

/// rwk: (rwk-Latn-TZ) Kiruwa, Latn, Tanzania
pub const fn lang_id_rwk() -> LangID {
  ID::new(7042930, None, None).into_lang_id()
}

/// sa: (sa-Deva-IN) संस्कृत भाषा, Deva, भारतः
pub const fn lang_id_sa() -> LangID {
  ID::new(24947, None, None).into_lang_id()
}

/// sah: (sah-Cyrl-RU) саха тыла, Нууччалыы, Арассыыйа
pub const fn lang_id_sah() -> LangID {
  ID::new(6840691, None, None).into_lang_id()
}

/// saq: (saq-Latn-KE) Kisampur, Latn, Kenya
pub const fn lang_id_saq() -> LangID {
  ID::new(7430515, None, None).into_lang_id()
}

/// sat: (sat-Olck-IN) ᱥᱟᱱᱛᱟᱲᱤ, ᱚᱞ ᱪᱤᱠᱤ, ᱤᱱᱰᱤᱭᱟ
pub const fn lang_id_sat() -> LangID {
  ID::new(7627123, None, None).into_lang_id()
}

/// sat-Olck: (sat-Olck-IN) ᱥᱟᱱᱛᱟᱲᱤ, ᱚᱞ ᱪᱤᱠᱤ, ᱤᱱᱰᱤᱭᱟ
pub const fn lang_id_sat_olck() -> LangID {
  ID::new(7627123, Some(1801677903), None).into_lang_id()
}

/// sbp: (sbp-Latn-TZ) Ishisangu, Latn, Tansaniya
pub const fn lang_id_sbp() -> LangID {
  ID::new(7365235, None, None).into_lang_id()
}

/// sc: (sc-Latn-IT) sardu, latinu, Itàlia
pub const fn lang_id_sc() -> LangID {
  ID::new(25459, None, None).into_lang_id()
}

/// sd: (sd-Arab-PK) سنڌي, عربي, پاڪستان
pub const fn lang_id_sd() -> LangID {
  ID::new(25715, None, None).into_lang_id()
}

/// sd-Arab: (sd-Arab-PK) سنڌي, عربي, پاڪستان
pub const fn lang_id_sd_arab() -> LangID {
  ID::new(25715, Some(1650553409), None).into_lang_id()
}

/// sd-Deva: (sd-Deva-IN) सिन्धी, देवनागिरी, भारत
pub const fn lang_id_sd_deva() -> LangID {
  ID::new(25715, Some(1635149124), None).into_lang_id()
}

/// se: (se-Latn-NO) davvisámegiella, láhtenaš, Norga
pub const fn lang_id_se() -> LangID {
  ID::new(25971, None, None).into_lang_id()
}

/// se-FI: (se-Latn-FI) davvisámegiella, láhtenaš, Suopma
pub const fn lang_id_se_fi() -> LangID {
  ID::new(25971, None, Some(18758)).into_lang_id()
}

/// se-SE: (se-Latn-SE) davvisámegiella, láhtenaš, Ruoŧŧa
pub const fn lang_id_se_se() -> LangID {
  ID::new(25971, None, Some(17747)).into_lang_id()
}

/// seh: (seh-Latn-MZ) sena, Latn, Moçambique
pub const fn lang_id_seh() -> LangID {
  ID::new(6841715, None, None).into_lang_id()
}

/// ses: (ses-Latn-ML) Koyraboro senni, Latn, Maali
pub const fn lang_id_ses() -> LangID {
  ID::new(7562611, None, None).into_lang_id()
}

/// sg: (sg-Latn-CF) Sängö, Latn, Ködörösêse tî Bêafrîka
pub const fn lang_id_sg() -> LangID {
  ID::new(26483, None, None).into_lang_id()
}

/// shi: (shi-Tfng-MA) ⵜⴰⵛⵍⵃⵉⵜ, Tfng, ⵍⵎⵖⵔⵉⴱ
pub const fn lang_id_shi() -> LangID {
  ID::new(6908019, None, None).into_lang_id()
}

/// shi-Latn: (shi-Latn-MA) Tashelḥiyt, Latn, lmɣrib
pub const fn lang_id_shi_latn() -> LangID {
  ID::new(6908019, Some(1853120844), None).into_lang_id()
}

/// shi-Tfng: (shi-Tfng-MA) ⵜⴰⵛⵍⵃⵉⵜ, Tfng, ⵍⵎⵖⵔⵉⴱ
pub const fn lang_id_shi_tfng() -> LangID {
  ID::new(6908019, Some(1735288404), None).into_lang_id()
}

/// si: (si-Sinh-LK) සිංහල, සිංහල, ශ්‍රී ලංකාව
pub const fn lang_id_si() -> LangID {
  ID::new(26995, None, None).into_lang_id()
}

/// sk: (sk-Latn-SK) slovenčina, latinka, Slovensko
pub const fn lang_id_sk() -> LangID {
  ID::new(27507, None, None).into_lang_id()
}

/// sl: (sl-Latn-SI) slovenščina, latinica, Slovenija
pub const fn lang_id_sl() -> LangID {
  ID::new(27763, None, None).into_lang_id()
}

/// smn: (smn-Latn-FI) anarâškielâ, Latn, Suomâ
pub const fn lang_id_smn() -> LangID {
  ID::new(7236979, None, None).into_lang_id()
}

/// sms: (sms-Latn-FI) sms, Latn, FI
pub const fn lang_id_sms() -> LangID {
  ID::new(7564659, None, None).into_lang_id()
}

/// sn: (sn-Latn-ZW) chiShona, Latn, Zimbabwe
pub const fn lang_id_sn() -> LangID {
  ID::new(28275, None, None).into_lang_id()
}

/// so: (so-Latn-SO) Soomaali, Laatiin, Soomaaliya
pub const fn lang_id_so() -> LangID {
  ID::new(28531, None, None).into_lang_id()
}

/// so-DJ: (so-Latn-DJ) Soomaali, Laatiin, Jabuuti
pub const fn lang_id_so_dj() -> LangID {
  ID::new(28531, None, Some(19012)).into_lang_id()
}

/// so-ET: (so-Latn-ET) Soomaali, Laatiin, Itoobiya
pub const fn lang_id_so_et() -> LangID {
  ID::new(28531, None, Some(21573)).into_lang_id()
}

/// so-KE: (so-Latn-KE) Soomaali, Laatiin, Kenya
pub const fn lang_id_so_ke() -> LangID {
  ID::new(28531, None, Some(17739)).into_lang_id()
}

/// sq: (sq-Latn-AL) shqip, latin, Shqipëri
pub const fn lang_id_sq() -> LangID {
  ID::new(29043, None, None).into_lang_id()
}

/// sq-MK: (sq-Latn-MK) shqip, latin, Maqedonia e Veriut
pub const fn lang_id_sq_mk() -> LangID {
  ID::new(29043, None, Some(19277)).into_lang_id()
}

/// sq-XK: (sq-Latn-XK) shqip, latin, Kosovë
pub const fn lang_id_sq_xk() -> LangID {
  ID::new(29043, None, Some(19288)).into_lang_id()
}

/// sr: (sr-Cyrl-RS) српски, ћирилица, Србија
pub const fn lang_id_sr() -> LangID {
  ID::new(29299, None, None).into_lang_id()
}

/// sr-Cyrl: (sr-Cyrl-RS) српски, ћирилица, Србија
pub const fn lang_id_sr_cyrl() -> LangID {
  ID::new(29299, Some(1819441475), None).into_lang_id()
}

/// sr-Cyrl-BA: српски, ћирилица, Босна и Херцеговина
pub const fn lang_id_sr_cyrl_ba() -> LangID {
  ID::new(29299, Some(1819441475), Some(16706)).into_lang_id()
}

/// sr-Cyrl-ME: српски, ћирилица, Црна Гора
pub const fn lang_id_sr_cyrl_me() -> LangID {
  ID::new(29299, Some(1819441475), Some(17741)).into_lang_id()
}

/// sr-Cyrl-XK: српски, ћирилица, Косово
pub const fn lang_id_sr_cyrl_xk() -> LangID {
  ID::new(29299, Some(1819441475), Some(19288)).into_lang_id()
}

/// sr-Latn: (sr-Latn-RS) srpski, latinica, Srbija
pub const fn lang_id_sr_latn() -> LangID {
  ID::new(29299, Some(1853120844), None).into_lang_id()
}

/// sr-Latn-BA: srpski, latinica, Bosna i Hercegovina
pub const fn lang_id_sr_latn_ba() -> LangID {
  ID::new(29299, Some(1853120844), Some(16706)).into_lang_id()
}

/// sr-Latn-ME: srpski, latinica, Crna Gora
pub const fn lang_id_sr_latn_me() -> LangID {
  ID::new(29299, Some(1853120844), Some(17741)).into_lang_id()
}

/// sr-Latn-XK: srpski, latinica, Kosovo
pub const fn lang_id_sr_latn_xk() -> LangID {
  ID::new(29299, Some(1853120844), Some(19288)).into_lang_id()
}

/// su: (su-Latn-ID) Basa Sunda, Latin, Indonesia
pub const fn lang_id_su() -> LangID {
  ID::new(30067, None, None).into_lang_id()
}

/// su-Latn: (su-Latn-ID) Basa Sunda, Latin, Indonesia
pub const fn lang_id_su_latn() -> LangID {
  ID::new(30067, Some(1853120844), None).into_lang_id()
}

/// sv: (sv-Latn-SE) svenska, latinska, Sverige
pub const fn lang_id_sv() -> LangID {
  ID::new(30323, None, None).into_lang_id()
}

/// sv-AX: (sv-Latn-AX) svenska, latinska, Åland
pub const fn lang_id_sv_ax() -> LangID {
  ID::new(30323, None, Some(22593)).into_lang_id()
}

/// sv-FI: (sv-Latn-FI) svenska, latinska, Finland
pub const fn lang_id_sv_fi() -> LangID {
  ID::new(30323, None, Some(18758)).into_lang_id()
}

/// sw: (sw-Latn-TZ) Kiswahili, Kilatini, Tanzania
pub const fn lang_id_sw() -> LangID {
  ID::new(30579, None, None).into_lang_id()
}

/// sw-CD: (sw-Latn-CD) Kiswahili, Kilatini, Jamhuri ya Kidemokrasia ya Kongo
pub const fn lang_id_sw_cd() -> LangID {
  ID::new(30579, None, Some(17475)).into_lang_id()
}

/// sw-KE: (sw-Latn-KE) Kiswahili, Kilatini, Kenya
pub const fn lang_id_sw_ke() -> LangID {
  ID::new(30579, None, Some(17739)).into_lang_id()
}

/// sw-UG: (sw-Latn-UG) Kiswahili, Kilatini, Uganda
pub const fn lang_id_sw_ug() -> LangID {
  ID::new(30579, None, Some(18261)).into_lang_id()
}

/// ta: (ta-Taml-IN) தமிழ், தமிழ், இந்தியா
pub const fn lang_id_ta() -> LangID {
  ID::new(24948, None, None).into_lang_id()
}

/// ta-LK: (ta-Taml-LK) தமிழ், தமிழ், இலங்கை
pub const fn lang_id_ta_lk() -> LangID {
  ID::new(24948, None, Some(19276)).into_lang_id()
}

/// ta-MY: (ta-Taml-MY) தமிழ், தமிழ், மலேசியா
pub const fn lang_id_ta_my() -> LangID {
  ID::new(24948, None, Some(22861)).into_lang_id()
}

/// ta-SG: (ta-Taml-SG) தமிழ், தமிழ், சிங்கப்பூர்
pub const fn lang_id_ta_sg() -> LangID {
  ID::new(24948, None, Some(18259)).into_lang_id()
}

/// te: (te-Telu-IN) తెలుగు, తెలుగు, భారతదేశం
pub const fn lang_id_te() -> LangID {
  ID::new(25972, None, None).into_lang_id()
}

/// teo: (teo-Latn-UG) Kiteso, Latn, Uganda
pub const fn lang_id_teo() -> LangID {
  ID::new(7300468, None, None).into_lang_id()
}

/// teo-KE: (teo-Latn-KE) Kiteso, Latn, Kenia
pub const fn lang_id_teo_ke() -> LangID {
  ID::new(7300468, None, Some(17739)).into_lang_id()
}

/// tg: (tg-Cyrl-TJ) тоҷикӣ, Кириллӣ, Тоҷикистон
pub const fn lang_id_tg() -> LangID {
  ID::new(26484, None, None).into_lang_id()
}

/// th: (th-Thai-TH) ไทย, ไทย, ไทย
pub const fn lang_id_th() -> LangID {
  ID::new(26740, None, None).into_lang_id()
}

/// ti: (ti-Ethi-ET) ትግርኛ, ፊደል, ኢትዮጵያ
pub const fn lang_id_ti() -> LangID {
  ID::new(26996, None, None).into_lang_id()
}

/// ti-ER: (ti-Ethi-ER) ትግርኛ, ፊደል, ኤርትራ
pub const fn lang_id_ti_er() -> LangID {
  ID::new(26996, None, Some(21061)).into_lang_id()
}

/// tk: (tk-Latn-TM) türkmen dili, Latyn elipbiýi, Türkmenistan
pub const fn lang_id_tk() -> LangID {
  ID::new(27508, None, None).into_lang_id()
}

/// to: (to-Latn-TO) lea fakatonga, tohinima fakalatina, Tonga
pub const fn lang_id_to() -> LangID {
  ID::new(28532, None, None).into_lang_id()
}

/// tok: (tok-Latn-001) Toki Pona, Latn, 001
pub const fn lang_id_tok() -> LangID {
  ID::new(7040884, None, None).into_lang_id()
}

/// tr: (tr-Latn-TR) Türkçe, Latin, Türkiye
pub const fn lang_id_tr() -> LangID {
  ID::new(29300, None, None).into_lang_id()
}

/// tr-CY: (tr-Latn-CY) Türkçe, Latin, Kıbrıs
pub const fn lang_id_tr_cy() -> LangID {
  ID::new(29300, None, Some(22851)).into_lang_id()
}

/// tt: (tt-Cyrl-RU) татар, кирилл, Россия
pub const fn lang_id_tt() -> LangID {
  ID::new(29812, None, None).into_lang_id()
}

/// twq: (twq-Latn-NE) Tasawaq senni, Latn, Nižer
pub const fn lang_id_twq() -> LangID {
  ID::new(7436148, None, None).into_lang_id()
}

/// tzm: (tzm-Latn-MA) Tamaziɣt n laṭlaṣ, Latn, Meṛṛuk
pub const fn lang_id_tzm() -> LangID {
  ID::new(7174772, None, None).into_lang_id()
}

/// ug: (ug-Arab-CN) ئۇيغۇرچە, ئەرەب, جۇڭگو
pub const fn lang_id_ug() -> LangID {
  ID::new(26485, None, None).into_lang_id()
}

/// uk: (uk-Cyrl-UA) українська, кирилиця, Україна
pub const fn lang_id_uk() -> LangID {
  ID::new(27509, None, None).into_lang_id()
}

/// Undefined.
pub const fn lang_id_und() -> LangID {
  ID::new(6581877, None, None).into_lang_id()
}

/// ur: (ur-Arab-PK) اردو, عربی, پاکستان
pub const fn lang_id_ur() -> LangID {
  ID::new(29301, None, None).into_lang_id()
}

/// ur-IN: (ur-Arab-IN) اردو, عربی, بھارت
pub const fn lang_id_ur_in() -> LangID {
  ID::new(29301, None, Some(20041)).into_lang_id()
}

/// uz: (uz-Latn-UZ) o‘zbek, lotin, Oʻzbekiston
pub const fn lang_id_uz() -> LangID {
  ID::new(31349, None, None).into_lang_id()
}

/// uz-Arab: (uz-Arab-AF) اوزبیک, عربی, افغانستان
pub const fn lang_id_uz_arab() -> LangID {
  ID::new(31349, Some(1650553409), None).into_lang_id()
}

/// uz-Cyrl: (uz-Cyrl-UZ) ўзбекча, Кирил, Ўзбекистон
pub const fn lang_id_uz_cyrl() -> LangID {
  ID::new(31349, Some(1819441475), None).into_lang_id()
}

/// uz-Latn: (uz-Latn-UZ) o‘zbek, lotin, Oʻzbekiston
pub const fn lang_id_uz_latn() -> LangID {
  ID::new(31349, Some(1853120844), None).into_lang_id()
}

/// vai: (vai-Vaii-LR) ꕙꔤ, Vaii, ꕞꔤꔫꕩ
pub const fn lang_id_vai() -> LangID {
  ID::new(6906230, None, None).into_lang_id()
}

/// vai-Latn: (vai-Latn-LR) Vai, Latn, Laibhiya
pub const fn lang_id_vai_latn() -> LangID {
  ID::new(6906230, Some(1853120844), None).into_lang_id()
}

/// vai-Vaii: (vai-Vaii-LR) ꕙꔤ, Vaii, ꕞꔤꔫꕩ
pub const fn lang_id_vai_vaii() -> LangID {
  ID::new(6906230, Some(1768513878), None).into_lang_id()
}

/// vi: (vi-Latn-VN) Tiếng Việt, Chữ La tinh, Việt Nam
pub const fn lang_id_vi() -> LangID {
  ID::new(26998, None, None).into_lang_id()
}

/// vun: (vun-Latn-TZ) Kyivunjo, Latn, Tanzania
pub const fn lang_id_vun() -> LangID {
  ID::new(7239030, None, None).into_lang_id()
}

/// wae: (wae-Latn-CH) Walser, Latiniš, Schwiz
pub const fn lang_id_wae() -> LangID {
  ID::new(6644087, None, None).into_lang_id()
}

/// wo: (wo-Latn-SN) Wolof, Latin, Senegaal
pub const fn lang_id_wo() -> LangID {
  ID::new(28535, None, None).into_lang_id()
}

/// xh: (xh-Latn-ZA) IsiXhosa, IsiLatin, EMzantsi Afrika
pub const fn lang_id_xh() -> LangID {
  ID::new(26744, None, None).into_lang_id()
}

/// xog: (xog-Latn-UG) Olusoga, Latn, Yuganda
pub const fn lang_id_xog() -> LangID {
  ID::new(6778744, None, None).into_lang_id()
}

/// yav: (yav-Latn-CM) nuasue, Latn, Kemelún
pub const fn lang_id_yav() -> LangID {
  ID::new(7758201, None, None).into_lang_id()
}

/// yi: (yi-Hebr-001) ייִדיש, העברעיש, וועלט
pub const fn lang_id_yi() -> LangID {
  ID::new(27001, None, None).into_lang_id()
}

/// yo: (yo-Latn-NG) Èdè Yorùbá, Èdè Látìn, Nàìjíríà
pub const fn lang_id_yo() -> LangID {
  ID::new(28537, None, None).into_lang_id()
}

/// yo-BJ: (yo-Latn-BJ) Èdè Yorùbá, Èdè Látìn, Bɛ̀nɛ̀
pub const fn lang_id_yo_bj() -> LangID {
  ID::new(28537, None, Some(19010)).into_lang_id()
}

/// yrl: (yrl-Latn-BR) nheẽgatu, ratĩ, Brasiu
pub const fn lang_id_yrl() -> LangID {
  ID::new(7107193, None, None).into_lang_id()
}

/// yrl-CO: (yrl-Latn-CO) ñengatú, ratĩ, Kurũbiya
pub const fn lang_id_yrl_co() -> LangID {
  ID::new(7107193, None, Some(20291)).into_lang_id()
}

/// yrl-VE: (yrl-Latn-VE) ñengatú, ratĩ, Wenesuera
pub const fn lang_id_yrl_ve() -> LangID {
  ID::new(7107193, None, Some(17750)).into_lang_id()
}

/// yue: (yue-Hant-HK) 粵語, 繁體, 中華人民共和國香港特別行政區
pub const fn lang_id_yue() -> LangID {
  ID::new(6649209, None, None).into_lang_id()
}

/// yue-Hans: (yue-Hans-CN) 粤语, 简体, 中华人民共和国
pub const fn lang_id_yue_hans() -> LangID {
  ID::new(6649209, Some(1936613704), None).into_lang_id()
}

/// yue-Hant: (yue-Hant-HK) 粵語, 繁體, 中華人民共和國香港特別行政區
pub const fn lang_id_yue_hant() -> LangID {
  ID::new(6649209, Some(1953390920), None).into_lang_id()
}

/// zgh: (zgh-Tfng-MA) ⵜⴰⵎⴰⵣⵉⵖⵜ, Tfng, ⵍⵎⵖⵔⵉⴱ
pub const fn lang_id_zgh() -> LangID {
  ID::new(6842234, None, None).into_lang_id()
}

/// zh: (zh-Hans-CN) 简体中文, 中国
pub const fn lang_id_zh() -> LangID {
  ID::new(26746, None, None).into_lang_id()
}

/// zh-Hans: (zh-Hans-CN) 简体中文, 中国
pub const fn lang_id_zh_hans() -> LangID {
  ID::new(26746, Some(1936613704), None).into_lang_id()
}

/// zh-Hans-HK: 简体中文, 中国香港特别行政区
pub const fn lang_id_zh_hans_hk() -> LangID {
  ID::new(26746, Some(1936613704), Some(19272)).into_lang_id()
}

/// zh-Hans-MO: 简体中文, 中国澳门特别行政区
pub const fn lang_id_zh_hans_mo() -> LangID {
  ID::new(26746, Some(1936613704), Some(20301)).into_lang_id()
}

/// zh-Hans-SG: 华文, 新加坡
pub const fn lang_id_zh_hans_sg() -> LangID {
  ID::new(26746, Some(1936613704), Some(18259)).into_lang_id()
}

/// zh-Hant: (zh-Hant-TW) 正體中文, 中國台灣
pub const fn lang_id_zh_hant() -> LangID {
  ID::new(26746, Some(1953390920), None).into_lang_id()
}

/// zh-Hant-HK: 繁體中文, 中國香港
pub const fn lang_id_zh_hant_hk() -> LangID {
  ID::new(26746, Some(1953390920), Some(19272)).into_lang_id()
}

/// zh-Hant-MO: 繁體中文, 中國澳門
pub const fn lang_id_zh_hant_mo() -> LangID {
  ID::new(26746, Some(1953390920), Some(20301)).into_lang_id()
}

/// zh-pinyin => zh-Latn-CN: HanYu PinYin
pub const fn lang_id_zh_pinyin() -> LangID {
  ID::new(26746, Some(1853120844), Some(20035)).into_lang_id()
}

/// lzh => lzh-Hant-CN: 文言文, 古典漢字, 華夏
pub const fn lang_id_lzh() -> LangID {
  ID::new(6847084, None, None).into_lang_id()
}

/// lzh-Hans => lzh-Hans-CN: 文言文, 简体汉字, 现代中国
pub const fn lang_id_lzh_hans() -> LangID {
  ID::new(6847084, Some(1936613704), None).into_lang_id()
}

/// zu: (zu-Latn-ZA) isiZulu, isi-Latin, iNingizimu Afrika
pub const fn lang_id_zu() -> LangID {
  ID::new(30074, None, None).into_lang_id()
}

/// st: (st-Latn-ZA)
pub const fn lang_id_st() -> LangID {
  ID::new(29811, None, None).into_lang_id()
}

/// la: (la-Latn-VA)
pub const fn lang_id_la() -> LangID {
  ID::new(24940, None, None).into_lang_id()
}

/// ny: (ny-Latn-MW)
pub const fn lang_id_ny() -> LangID {
  ID::new(31086, None, None).into_lang_id()
}

/// sm: (sm-Latn-WS)
pub const fn lang_id_sm() -> LangID {
  ID::new(28019, None, None).into_lang_id()
}

/// jw: (jw-Latn-ID)
pub const fn lang_id_jw() -> LangID {
  ID::new(30570, None, None).into_lang_id()
}

/// ht: (ht-Latn-HT)
pub const fn lang_id_ht() -> LangID {
  ID::new(29800, None, None).into_lang_id()
}

/// co: (co-Latn-FR)
pub const fn lang_id_co() -> LangID {
  ID::new(28515, None, None).into_lang_id()
}

/// tl: (tl-Latn-PH)
pub const fn lang_id_tl() -> LangID {
  ID::new(27764, None, None).into_lang_id()
}

/// iw: (iw-Hebr-IL)
pub const fn lang_id_iw() -> LangID {
  ID::new(30569, None, None).into_lang_id()
}
