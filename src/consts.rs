#![allow(clippy::missing_safety_doc)]
#![allow(unsafe_op_in_unsafe_fn)]

use crate::{LangID, id::ID};

/// af: (af-Latn-ZA) Afrikaans, Latyn, Suid-Afrika
pub const fn get_af() -> LangID {
  ID::new(26209, None, None).get_id()
}

/// af-NA: (af-Latn-NA) Afrikaans, Latyn, Namibië
pub const fn get_af_na() -> LangID {
  ID::new(26209, None, Some(16718)).get_id()
}

/// agq: (agq-Latn-CM) Aghem, Latn, Kàmàlûŋ
pub const fn get_agq() -> LangID {
  ID::new(7432033, None, None).get_id()
}

/// ak: (ak-Latn-GH) Akan, Latn, Gaana
pub const fn get_ak() -> LangID {
  ID::new(27489, None, None).get_id()
}

/// am: (am-Ethi-ET) አማርኛ, ኢትዮፒክ, ኢትዮጵያ
pub const fn get_am() -> LangID {
  ID::new(28001, None, None).get_id()
}

/// ann: (ann-Latn-NG) Obolo, Latn, NG
pub const fn get_ann() -> LangID {
  ID::new(7237217, None, None).get_id()
}

/// ar: (ar-Arab-EG) العربية, العربية, مصر
pub const fn get_ar() -> LangID {
  ID::new(29281, None, None).get_id()
}

/// ar-AE: (ar-Arab-AE) العربية, العربية, الإمارات العربية المتحدة
pub const fn get_ar_ae() -> LangID {
  ID::new(29281, None, Some(17729)).get_id()
}

/// ar-BH: (ar-Arab-BH) العربية, العربية, البحرين
pub const fn get_ar_bh() -> LangID {
  ID::new(29281, None, Some(18498)).get_id()
}

/// ar-DJ: (ar-Arab-DJ) العربية, العربية, جيبوتي
pub const fn get_ar_dj() -> LangID {
  ID::new(29281, None, Some(19012)).get_id()
}

/// ar-DZ: (ar-Arab-DZ) العربية, العربية, الجزائر
pub const fn get_ar_dz() -> LangID {
  ID::new(29281, None, Some(23108)).get_id()
}

/// ar-EG: (ar-Arab-EG) العربية, العربية, مصر
pub const fn get_ar_eg() -> LangID {
  ID::new(29281, None, Some(18245)).get_id()
}

/// ar-EH: (ar-Arab-EH) العربية, العربية, الصحراء الغربية
pub const fn get_ar_eh() -> LangID {
  ID::new(29281, None, Some(18501)).get_id()
}

/// ar-ER: (ar-Arab-ER) العربية, العربية, إريتريا
pub const fn get_ar_er() -> LangID {
  ID::new(29281, None, Some(21061)).get_id()
}

/// ar-IL: (ar-Arab-IL) العربية, العربية, إسرائيل
pub const fn get_ar_il() -> LangID {
  ID::new(29281, None, Some(19529)).get_id()
}

/// ar-IQ: (ar-Arab-IQ) العربية, العربية, العراق
pub const fn get_ar_iq() -> LangID {
  ID::new(29281, None, Some(20809)).get_id()
}

/// ar-JO: (ar-Arab-JO) العربية, العربية, الأردن
pub const fn get_ar_jo() -> LangID {
  ID::new(29281, None, Some(20298)).get_id()
}

/// ar-KM: (ar-Arab-KM) العربية, العربية, جزر القمر
pub const fn get_ar_km() -> LangID {
  ID::new(29281, None, Some(19787)).get_id()
}

/// ar-KW: (ar-Arab-KW) العربية, العربية, الكويت
pub const fn get_ar_kw() -> LangID {
  ID::new(29281, None, Some(22347)).get_id()
}

/// ar-LB: (ar-Arab-LB) العربية, العربية, لبنان
pub const fn get_ar_lb() -> LangID {
  ID::new(29281, None, Some(16972)).get_id()
}

/// ar-LY: (ar-Arab-LY) العربية, العربية, ليبيا
pub const fn get_ar_ly() -> LangID {
  ID::new(29281, None, Some(22860)).get_id()
}

/// ar-MA: (ar-Arab-MA) العربية, العربية, المغرب
pub const fn get_ar_ma() -> LangID {
  ID::new(29281, None, Some(16717)).get_id()
}

/// ar-MR: (ar-Arab-MR) العربية, العربية, موريتانيا
pub const fn get_ar_mr() -> LangID {
  ID::new(29281, None, Some(21069)).get_id()
}

/// ar-OM: (ar-Arab-OM) العربية, العربية, عُمان
pub const fn get_ar_om() -> LangID {
  ID::new(29281, None, Some(19791)).get_id()
}

/// ar-PS: (ar-Arab-PS) العربية, العربية, الأراضي الفلسطينية
pub const fn get_ar_ps() -> LangID {
  ID::new(29281, None, Some(21328)).get_id()
}

/// ar-QA: (ar-Arab-QA) العربية, العربية, قطر
pub const fn get_ar_qa() -> LangID {
  ID::new(29281, None, Some(16721)).get_id()
}

/// ar-SA: (ar-Arab-SA) العربية, العربية, المملكة العربية السعودية
pub const fn get_ar_sa() -> LangID {
  ID::new(29281, None, Some(16723)).get_id()
}

/// ar-SD: (ar-Arab-SD) العربية, العربية, السودان
pub const fn get_ar_sd() -> LangID {
  ID::new(29281, None, Some(17491)).get_id()
}

/// ar-SO: (ar-Arab-SO) العربية, العربية, الصومال
pub const fn get_ar_so() -> LangID {
  ID::new(29281, None, Some(20307)).get_id()
}

/// ar-SS: (ar-Arab-SS) العربية, العربية, جنوب السودان
pub const fn get_ar_ss() -> LangID {
  ID::new(29281, None, Some(21331)).get_id()
}

/// ar-SY: (ar-Arab-SY) العربية, العربية, سوريا
pub const fn get_ar_sy() -> LangID {
  ID::new(29281, None, Some(22867)).get_id()
}

/// ar-TD: (ar-Arab-TD) العربية, العربية, تشاد
pub const fn get_ar_td() -> LangID {
  ID::new(29281, None, Some(17492)).get_id()
}

/// ar-TN: (ar-Arab-TN) العربية, العربية, تونس
pub const fn get_ar_tn() -> LangID {
  ID::new(29281, None, Some(20052)).get_id()
}

/// ar-YE: (ar-Arab-YE) العربية, العربية, اليمن
pub const fn get_ar_ye() -> LangID {
  ID::new(29281, None, Some(17753)).get_id()
}

/// as: (as-Beng-IN) অসমীয়া, বাংলা, ভাৰত
pub const fn get_as() -> LangID {
  ID::new(29537, None, None).get_id()
}

/// asa: (asa-Latn-TZ) Kipare, Latn, Tadhania
pub const fn get_asa() -> LangID {
  ID::new(6386529, None, None).get_id()
}

/// ast: (ast-Latn-ES) asturianu, llatín, España
pub const fn get_ast() -> LangID {
  ID::new(7631713, None, None).get_id()
}

/// az: (az-Latn-AZ) azərbaycan, latın, Azərbaycan
pub const fn get_az() -> LangID {
  ID::new(31329, None, None).get_id()
}

/// az-Cyrl: (az-Cyrl-AZ) азәрбајҹан, Кирил, Азәрбајҹан
pub const fn get_az_cyrl() -> LangID {
  ID::new(31329, Some(1819441475), None).get_id()
}

/// az-Latn: (az-Latn-AZ) azərbaycan, latın, Azərbaycan
pub const fn get_az_latn() -> LangID {
  ID::new(31329, Some(1853120844), None).get_id()
}

/// bas: (bas-Latn-CM) Ɓàsàa, Latn, Kàmɛ̀rûn
pub const fn get_bas() -> LangID {
  ID::new(7561570, None, None).get_id()
}

/// be: (be-Cyrl-BY) беларуская, кірыліца, Беларусь
pub const fn get_be() -> LangID {
  ID::new(25954, None, None).get_id()
}

/// bem: (bem-Latn-ZM) Ichibemba, Latn, Zambia
pub const fn get_bem() -> LangID {
  ID::new(7169378, None, None).get_id()
}

/// bez: (bez-Latn-TZ) Hibena, Latn, Hutanzania
pub const fn get_bez() -> LangID {
  ID::new(8021346, None, None).get_id()
}

/// bg: (bg-Cyrl-BG) български, кирилица, България
pub const fn get_bg() -> LangID {
  ID::new(26466, None, None).get_id()
}

/// bgc: (bgc-Deva-IN) हरियाणवी, देवानागारी, भारत
pub const fn get_bgc() -> LangID {
  ID::new(6514530, None, None).get_id()
}

/// bho: (bho-Deva-IN) भोजपुरी, देवानागारी, भारत
pub const fn get_bho() -> LangID {
  ID::new(7301218, None, None).get_id()
}

/// bm: (bm-Latn-ML) bamanakan, Latn, Mali
pub const fn get_bm() -> LangID {
  ID::new(28002, None, None).get_id()
}

/// bn: (bn-Beng-BD) বাংলা, বাংলা, বাংলাদেশ
pub const fn get_bn() -> LangID {
  ID::new(28258, None, None).get_id()
}

/// bn-IN: (bn-Beng-IN) বাংলা, বাংলা, ভারত
pub const fn get_bn_in() -> LangID {
  ID::new(28258, None, Some(20041)).get_id()
}

/// bo: (bo-Tibt-CN) བོད་སྐད་, བོད་ཡིག་, རྒྱ་ནག
pub const fn get_bo() -> LangID {
  ID::new(28514, None, None).get_id()
}

/// bo-IN: (bo-Tibt-IN) བོད་སྐད་, བོད་ཡིག་, རྒྱ་གར་
pub const fn get_bo_in() -> LangID {
  ID::new(28514, None, Some(20041)).get_id()
}

/// br: (br-Latn-FR) brezhoneg, latin, Frañs
pub const fn get_br() -> LangID {
  ID::new(29282, None, None).get_id()
}

/// brx: (brx-Deva-IN) बर’, देबनागिरि, भारत
pub const fn get_brx() -> LangID {
  ID::new(7893602, None, None).get_id()
}

/// bs: (bs-Latn-BA) bosanski, latinica, Bosna i Hercegovina
pub const fn get_bs() -> LangID {
  ID::new(29538, None, None).get_id()
}

/// bs-Cyrl: (bs-Cyrl-BA) босански, ћирилица, Босна и Херцеговина
pub const fn get_bs_cyrl() -> LangID {
  ID::new(29538, Some(1819441475), None).get_id()
}

/// bs-Latn: (bs-Latn-BA) bosanski, latinica, Bosna i Hercegovina
pub const fn get_bs_latn() -> LangID {
  ID::new(29538, Some(1853120844), None).get_id()
}

/// ca: (ca-Latn-ES) català, llatí, Espanya
pub const fn get_ca() -> LangID {
  ID::new(24931, None, None).get_id()
}

/// ca-AD: (ca-Latn-AD) català, llatí, Andorra
pub const fn get_ca_ad() -> LangID {
  ID::new(24931, None, Some(17473)).get_id()
}

/// ca-FR: (ca-Latn-FR) català, llatí, França
pub const fn get_ca_fr() -> LangID {
  ID::new(24931, None, Some(21062)).get_id()
}

/// ca-IT: (ca-Latn-IT) català, llatí, Itàlia
pub const fn get_ca_it() -> LangID {
  ID::new(24931, None, Some(21577)).get_id()
}

/// ccp: (ccp-Cakm-BD) 𑄌𑄋𑄴𑄟𑄳𑄦, 𑄌𑄇𑄴𑄟, 𑄝𑄁𑄣𑄘𑄬𑄌𑄴
pub const fn get_ccp() -> LangID {
  ID::new(7365475, None, None).get_id()
}

/// ccp-IN: (ccp-Cakm-IN) 𑄌𑄋𑄴𑄟𑄳𑄦, 𑄌𑄇𑄴𑄟, 𑄞𑄢𑄧𑄖𑄴
pub const fn get_ccp_in() -> LangID {
  ID::new(7365475, None, Some(20041)).get_id()
}

/// ce: (ce-Cyrl-RU) нохчийн, кириллица, Росси
pub const fn get_ce() -> LangID {
  ID::new(25955, None, None).get_id()
}

/// ceb: (ceb-Latn-PH) Cebuano, Latin, Pilipinas
pub const fn get_ceb() -> LangID {
  ID::new(6448483, None, None).get_id()
}

/// cgg: (cgg-Latn-UG) Rukiga, Latn, Uganda
pub const fn get_cgg() -> LangID {
  ID::new(6776675, None, None).get_id()
}

/// chr: (chr-Cher-US) ᏣᎳᎩ, ᏣᎳᎩ, ᏌᏊ ᎢᏳᎾᎵᏍᏔᏅ ᏍᎦᏚᎩ
pub const fn get_chr() -> LangID {
  ID::new(7497827, None, None).get_id()
}

/// ckb: (ckb-Arab-IQ) کوردیی ناوەندی, عەرەبی, عێراق
pub const fn get_ckb() -> LangID {
  ID::new(6450019, None, None).get_id()
}

/// ckb-IR: (ckb-Arab-IR) کوردیی ناوەندی, عەرەبی, ئێران
pub const fn get_ckb_ir() -> LangID {
  ID::new(6450019, None, Some(21065)).get_id()
}

/// cs: (cs-Latn-CZ) čeština, latinka, Česko
pub const fn get_cs() -> LangID {
  ID::new(29539, None, None).get_id()
}

/// cv: (cv-Cyrl-RU) чӑваш, кириллица, Раҫҫей
pub const fn get_cv() -> LangID {
  ID::new(30307, None, None).get_id()
}

/// cy: (cy-Latn-GB) Cymraeg, Lladin, Y Deyrnas Unedig
pub const fn get_cy() -> LangID {
  ID::new(31075, None, None).get_id()
}

/// da: (da-Latn-DK) dansk, latinsk, Danmark
pub const fn get_da() -> LangID {
  ID::new(24932, None, None).get_id()
}

/// da-GL: (da-Latn-GL) dansk, latinsk, Grønland
pub const fn get_da_gl() -> LangID {
  ID::new(24932, None, Some(19527)).get_id()
}

/// dav: (dav-Latn-KE) Kitaita, Latn, Kenya
pub const fn get_dav() -> LangID {
  ID::new(7758180, None, None).get_id()
}

/// de: (de-Latn-DE) Deutsch, Lateinisch, Deutschland
pub const fn get_de() -> LangID {
  ID::new(25956, None, None).get_id()
}

/// de-AT: (de-Latn-AT) Deutsch, Lateinisch, Österreich
pub const fn get_de_at() -> LangID {
  ID::new(25956, None, Some(21569)).get_id()
}

/// de-BE: (de-Latn-BE) Deutsch, Lateinisch, Belgien
pub const fn get_de_be() -> LangID {
  ID::new(25956, None, Some(17730)).get_id()
}

/// de-CH: (de-Latn-CH) Deutsch, Lateinisch, Schweiz
pub const fn get_de_ch() -> LangID {
  ID::new(25956, None, Some(18499)).get_id()
}

/// de-IT: (de-Latn-IT) Deutsch, Lateinisch, Italien
pub const fn get_de_it() -> LangID {
  ID::new(25956, None, Some(21577)).get_id()
}

/// de-LI: (de-Latn-LI) Deutsch, Lateinisch, Liechtenstein
pub const fn get_de_li() -> LangID {
  ID::new(25956, None, Some(18764)).get_id()
}

/// de-LU: (de-Latn-LU) Deutsch, Lateinisch, Luxemburg
pub const fn get_de_lu() -> LangID {
  ID::new(25956, None, Some(21836)).get_id()
}

/// dje: (dje-Latn-NE) Zarmaciine, Latn, Nižer
pub const fn get_dje() -> LangID {
  ID::new(6646372, None, None).get_id()
}

/// doi: (doi-Deva-IN) डोगरी, देवनागरी, भारत
pub const fn get_doi() -> LangID {
  ID::new(6909796, None, None).get_id()
}

/// dsb: (dsb-Latn-DE) dolnoserbšćina, łatyński, Nimska
pub const fn get_dsb() -> LangID {
  ID::new(6452068, None, None).get_id()
}

/// dua: (dua-Latn-CM) duálá, Latn, Cameroun
pub const fn get_dua() -> LangID {
  ID::new(6387044, None, None).get_id()
}

/// dyo: (dyo-Latn-SN) joola, Latn, Senegal
pub const fn get_dyo() -> LangID {
  ID::new(7305572, None, None).get_id()
}

/// dz: (dz-Tibt-BT) རྫོང་ཁ, ང་བཅས་ཀྱི་ཡིག་གུ, འབྲུག
pub const fn get_dz() -> LangID {
  ID::new(31332, None, None).get_id()
}

/// ebu: (ebu-Latn-KE) Kĩembu, Latn, Kenya
pub const fn get_ebu() -> LangID {
  ID::new(7692901, None, None).get_id()
}

/// ee: (ee-Latn-GH) Eʋegbe, Latingbeŋɔŋlɔ, Ghana nutome
pub const fn get_ee() -> LangID {
  ID::new(25957, None, None).get_id()
}

/// ee-TG: (ee-Latn-TG) Eʋegbe, Latingbeŋɔŋlɔ, Togo nutome
pub const fn get_ee_tg() -> LangID {
  ID::new(25957, None, Some(18260)).get_id()
}

/// el: (el-Grek-GR) Ελληνικά, Ελληνικό, Ελλάδα
pub const fn get_el() -> LangID {
  ID::new(27749, None, None).get_id()
}

/// el-CY: (el-Grek-CY) Ελληνικά, Ελληνικό, Κύπρος
pub const fn get_el_cy() -> LangID {
  ID::new(27749, None, Some(22851)).get_id()
}

/// en: (en-Latn-US) English, Latin, United States
pub const fn get_en() -> LangID {
  ID::new(28261, None, None).get_id()
}

/// en-001: (en-Latn-001) English, Latin, world
pub const fn get_en_001() -> LangID {
  ID::new(28261, None, Some(3223600)).get_id()
}

/// en-150: (en-Latn-150) English, Latin, Europe
pub const fn get_en_150() -> LangID {
  ID::new(28261, None, Some(3159345)).get_id()
}

/// en-AE: (en-Latn-AE) English, Latin, United Arab Emirates
pub const fn get_en_ae() -> LangID {
  ID::new(28261, None, Some(17729)).get_id()
}

/// en-AG: (en-Latn-AG) English, Latin, Antigua & Barbuda
pub const fn get_en_ag() -> LangID {
  ID::new(28261, None, Some(18241)).get_id()
}

/// en-AI: (en-Latn-AI) English, Latin, Anguilla
pub const fn get_en_ai() -> LangID {
  ID::new(28261, None, Some(18753)).get_id()
}

/// en-AS: (en-Latn-AS) English, Latin, American Samoa
pub const fn get_en_as() -> LangID {
  ID::new(28261, None, Some(21313)).get_id()
}

/// en-AT: (en-Latn-AT) English, Latin, Austria
pub const fn get_en_at() -> LangID {
  ID::new(28261, None, Some(21569)).get_id()
}

/// en-AU: (en-Latn-AU) English, Latin, Australia
pub const fn get_en_au() -> LangID {
  ID::new(28261, None, Some(21825)).get_id()
}

/// en-BB: (en-Latn-BB) English, Latin, Barbados
pub const fn get_en_bb() -> LangID {
  ID::new(28261, None, Some(16962)).get_id()
}

/// en-BE: (en-Latn-BE) English, Latin, Belgium
pub const fn get_en_be() -> LangID {
  ID::new(28261, None, Some(17730)).get_id()
}

/// en-BI: (en-Latn-BI) English, Latin, Burundi
pub const fn get_en_bi() -> LangID {
  ID::new(28261, None, Some(18754)).get_id()
}

/// en-BM: (en-Latn-BM) English, Latin, Bermuda
pub const fn get_en_bm() -> LangID {
  ID::new(28261, None, Some(19778)).get_id()
}

/// en-BS: (en-Latn-BS) English, Latin, Bahamas
pub const fn get_en_bs() -> LangID {
  ID::new(28261, None, Some(21314)).get_id()
}

/// en-BW: (en-Latn-BW) English, Latin, Botswana
pub const fn get_en_bw() -> LangID {
  ID::new(28261, None, Some(22338)).get_id()
}

/// en-BZ: (en-Latn-BZ) English, Latin, Belize
pub const fn get_en_bz() -> LangID {
  ID::new(28261, None, Some(23106)).get_id()
}

/// en-CA: (en-Latn-CA) English, Latin, Canada
pub const fn get_en_ca() -> LangID {
  ID::new(28261, None, Some(16707)).get_id()
}

/// en-CC: (en-Latn-CC) English, Latin, Cocos (Keeling) Islands
pub const fn get_en_cc() -> LangID {
  ID::new(28261, None, Some(17219)).get_id()
}

/// en-CH: (en-Latn-CH) English, Latin, Switzerland
pub const fn get_en_ch() -> LangID {
  ID::new(28261, None, Some(18499)).get_id()
}

/// en-CK: (en-Latn-CK) English, Latin, Cook Islands
pub const fn get_en_ck() -> LangID {
  ID::new(28261, None, Some(19267)).get_id()
}

/// en-CM: (en-Latn-CM) English, Latin, Cameroon
pub const fn get_en_cm() -> LangID {
  ID::new(28261, None, Some(19779)).get_id()
}

/// en-CX: (en-Latn-CX) English, Latin, Christmas Island
pub const fn get_en_cx() -> LangID {
  ID::new(28261, None, Some(22595)).get_id()
}

/// en-CY: (en-Latn-CY) English, Latin, Cyprus
pub const fn get_en_cy() -> LangID {
  ID::new(28261, None, Some(22851)).get_id()
}

/// en-DE: (en-Latn-DE) English, Latin, Germany
pub const fn get_en_de() -> LangID {
  ID::new(28261, None, Some(17732)).get_id()
}

/// en-DG: (en-Latn-DG) English, Latin, Diego Garcia
pub const fn get_en_dg() -> LangID {
  ID::new(28261, None, Some(18244)).get_id()
}

/// en-DK: (en-Latn-DK) English, Latin, Denmark
pub const fn get_en_dk() -> LangID {
  ID::new(28261, None, Some(19268)).get_id()
}

/// en-DM: (en-Latn-DM) English, Latin, Dominica
pub const fn get_en_dm() -> LangID {
  ID::new(28261, None, Some(19780)).get_id()
}

/// en-ER: (en-Latn-ER) English, Latin, Eritrea
pub const fn get_en_er() -> LangID {
  ID::new(28261, None, Some(21061)).get_id()
}

/// en-FI: (en-Latn-FI) English, Latin, Finland
pub const fn get_en_fi() -> LangID {
  ID::new(28261, None, Some(18758)).get_id()
}

/// en-FJ: (en-Latn-FJ) English, Latin, Fiji
pub const fn get_en_fj() -> LangID {
  ID::new(28261, None, Some(19014)).get_id()
}

/// en-FK: (en-Latn-FK) English, Latin, Falkland Islands
pub const fn get_en_fk() -> LangID {
  ID::new(28261, None, Some(19270)).get_id()
}

/// en-FM: (en-Latn-FM) English, Latin, Micronesia
pub const fn get_en_fm() -> LangID {
  ID::new(28261, None, Some(19782)).get_id()
}

/// en-GB: (en-Latn-GB) English, Latin, United Kingdom
pub const fn get_en_gb() -> LangID {
  ID::new(28261, None, Some(16967)).get_id()
}

/// en-GD: (en-Latn-GD) English, Latin, Grenada
pub const fn get_en_gd() -> LangID {
  ID::new(28261, None, Some(17479)).get_id()
}

/// en-GG: (en-Latn-GG) English, Latin, Guernsey
pub const fn get_en_gg() -> LangID {
  ID::new(28261, None, Some(18247)).get_id()
}

/// en-GH: (en-Latn-GH) English, Latin, Ghana
pub const fn get_en_gh() -> LangID {
  ID::new(28261, None, Some(18503)).get_id()
}

/// en-GI: (en-Latn-GI) English, Latin, Gibraltar
pub const fn get_en_gi() -> LangID {
  ID::new(28261, None, Some(18759)).get_id()
}

/// en-GM: (en-Latn-GM) English, Latin, Gambia
pub const fn get_en_gm() -> LangID {
  ID::new(28261, None, Some(19783)).get_id()
}

/// en-GU: (en-Latn-GU) English, Latin, Guam
pub const fn get_en_gu() -> LangID {
  ID::new(28261, None, Some(21831)).get_id()
}

/// en-GY: (en-Latn-GY) English, Latin, Guyana
pub const fn get_en_gy() -> LangID {
  ID::new(28261, None, Some(22855)).get_id()
}

/// en-HK: (en-Latn-HK) English, Latin, Hong Kong SAR China
pub const fn get_en_hk() -> LangID {
  ID::new(28261, None, Some(19272)).get_id()
}

/// en-IE: (en-Latn-IE) English, Latin, Ireland
pub const fn get_en_ie() -> LangID {
  ID::new(28261, None, Some(17737)).get_id()
}

/// en-IL: (en-Latn-IL) English, Latin, Israel
pub const fn get_en_il() -> LangID {
  ID::new(28261, None, Some(19529)).get_id()
}

/// en-IM: (en-Latn-IM) English, Latin, Isle of Man
pub const fn get_en_im() -> LangID {
  ID::new(28261, None, Some(19785)).get_id()
}

/// en-IN: (en-Latn-IN) English, Latin, India
pub const fn get_en_in() -> LangID {
  ID::new(28261, None, Some(20041)).get_id()
}

/// en-IO: (en-Latn-IO) English, Latin, British Indian Ocean Territory
pub const fn get_en_io() -> LangID {
  ID::new(28261, None, Some(20297)).get_id()
}

/// en-JE: (en-Latn-JE) English, Latin, Jersey
pub const fn get_en_je() -> LangID {
  ID::new(28261, None, Some(17738)).get_id()
}

/// en-JM: (en-Latn-JM) English, Latin, Jamaica
pub const fn get_en_jm() -> LangID {
  ID::new(28261, None, Some(19786)).get_id()
}

/// en-KE: (en-Latn-KE) English, Latin, Kenya
pub const fn get_en_ke() -> LangID {
  ID::new(28261, None, Some(17739)).get_id()
}

/// en-KI: (en-Latn-KI) English, Latin, Kiribati
pub const fn get_en_ki() -> LangID {
  ID::new(28261, None, Some(18763)).get_id()
}

/// en-KN: (en-Latn-KN) English, Latin, St Kitts & Nevis
pub const fn get_en_kn() -> LangID {
  ID::new(28261, None, Some(20043)).get_id()
}

/// en-KY: (en-Latn-KY) English, Latin, Cayman Islands
pub const fn get_en_ky() -> LangID {
  ID::new(28261, None, Some(22859)).get_id()
}

/// en-LC: (en-Latn-LC) English, Latin, St Lucia
pub const fn get_en_lc() -> LangID {
  ID::new(28261, None, Some(17228)).get_id()
}

/// en-LR: (en-Latn-LR) English, Latin, Liberia
pub const fn get_en_lr() -> LangID {
  ID::new(28261, None, Some(21068)).get_id()
}

/// en-LS: (en-Latn-LS) English, Latin, Lesotho
pub const fn get_en_ls() -> LangID {
  ID::new(28261, None, Some(21324)).get_id()
}

/// en-MG: (en-Latn-MG) English, Latin, Madagascar
pub const fn get_en_mg() -> LangID {
  ID::new(28261, None, Some(18253)).get_id()
}

/// en-MH: (en-Latn-MH) English, Latin, Marshall Islands
pub const fn get_en_mh() -> LangID {
  ID::new(28261, None, Some(18509)).get_id()
}

/// en-MO: (en-Latn-MO) English, Latin, Macao SAR China
pub const fn get_en_mo() -> LangID {
  ID::new(28261, None, Some(20301)).get_id()
}

/// en-MP: (en-Latn-MP) English, Latin, Northern Mariana Islands
pub const fn get_en_mp() -> LangID {
  ID::new(28261, None, Some(20557)).get_id()
}

/// en-MS: (en-Latn-MS) English, Latin, Montserrat
pub const fn get_en_ms() -> LangID {
  ID::new(28261, None, Some(21325)).get_id()
}

/// en-MT: (en-Latn-MT) English, Latin, Malta
pub const fn get_en_mt() -> LangID {
  ID::new(28261, None, Some(21581)).get_id()
}

/// en-MU: (en-Latn-MU) English, Latin, Mauritius
pub const fn get_en_mu() -> LangID {
  ID::new(28261, None, Some(21837)).get_id()
}

/// en-MV: (en-Latn-MV) English, Latin, Maldives
pub const fn get_en_mv() -> LangID {
  ID::new(28261, None, Some(22093)).get_id()
}

/// en-MW: (en-Latn-MW) English, Latin, Malawi
pub const fn get_en_mw() -> LangID {
  ID::new(28261, None, Some(22349)).get_id()
}

/// en-MY: (en-Latn-MY) English, Latin, Malaysia
pub const fn get_en_my() -> LangID {
  ID::new(28261, None, Some(22861)).get_id()
}

/// en-NA: (en-Latn-NA) English, Latin, Namibia
pub const fn get_en_na() -> LangID {
  ID::new(28261, None, Some(16718)).get_id()
}

/// en-NF: (en-Latn-NF) English, Latin, Norfolk Island
pub const fn get_en_nf() -> LangID {
  ID::new(28261, None, Some(17998)).get_id()
}

/// en-NG: (en-Latn-NG) English, Latin, Nigeria
pub const fn get_en_ng() -> LangID {
  ID::new(28261, None, Some(18254)).get_id()
}

/// en-NL: (en-Latn-NL) English, Latin, Netherlands
pub const fn get_en_nl() -> LangID {
  ID::new(28261, None, Some(19534)).get_id()
}

/// en-NR: (en-Latn-NR) English, Latin, Nauru
pub const fn get_en_nr() -> LangID {
  ID::new(28261, None, Some(21070)).get_id()
}

/// en-NU: (en-Latn-NU) English, Latin, Niue
pub const fn get_en_nu() -> LangID {
  ID::new(28261, None, Some(21838)).get_id()
}

/// en-NZ: (en-Latn-NZ) English, Latin, New Zealand
pub const fn get_en_nz() -> LangID {
  ID::new(28261, None, Some(23118)).get_id()
}

/// en-PG: (en-Latn-PG) English, Latin, Papua New Guinea
pub const fn get_en_pg() -> LangID {
  ID::new(28261, None, Some(18256)).get_id()
}

/// en-PH: (en-Latn-PH) English, Latin, Philippines
pub const fn get_en_ph() -> LangID {
  ID::new(28261, None, Some(18512)).get_id()
}

/// en-PK: (en-Latn-PK) English, Latin, Pakistan
pub const fn get_en_pk() -> LangID {
  ID::new(28261, None, Some(19280)).get_id()
}

/// en-PN: (en-Latn-PN) English, Latin, Pitcairn Islands
pub const fn get_en_pn() -> LangID {
  ID::new(28261, None, Some(20048)).get_id()
}

/// en-PR: (en-Latn-PR) English, Latin, Puerto Rico
pub const fn get_en_pr() -> LangID {
  ID::new(28261, None, Some(21072)).get_id()
}

/// en-PW: (en-Latn-PW) English, Latin, Palau
pub const fn get_en_pw() -> LangID {
  ID::new(28261, None, Some(22352)).get_id()
}

/// en-RW: (en-Latn-RW) English, Latin, Rwanda
pub const fn get_en_rw() -> LangID {
  ID::new(28261, None, Some(22354)).get_id()
}

/// en-SB: (en-Latn-SB) English, Latin, Solomon Islands
pub const fn get_en_sb() -> LangID {
  ID::new(28261, None, Some(16979)).get_id()
}

/// en-SC: (en-Latn-SC) English, Latin, Seychelles
pub const fn get_en_sc() -> LangID {
  ID::new(28261, None, Some(17235)).get_id()
}

/// en-SD: (en-Latn-SD) English, Latin, Sudan
pub const fn get_en_sd() -> LangID {
  ID::new(28261, None, Some(17491)).get_id()
}

/// en-SE: (en-Latn-SE) English, Latin, Sweden
pub const fn get_en_se() -> LangID {
  ID::new(28261, None, Some(17747)).get_id()
}

/// en-SG: (en-Latn-SG) English, Latin, Singapore
pub const fn get_en_sg() -> LangID {
  ID::new(28261, None, Some(18259)).get_id()
}

/// en-SH: (en-Latn-SH) English, Latin, St Helena
pub const fn get_en_sh() -> LangID {
  ID::new(28261, None, Some(18515)).get_id()
}

/// en-SI: (en-Latn-SI) English, Latin, Slovenia
pub const fn get_en_si() -> LangID {
  ID::new(28261, None, Some(18771)).get_id()
}

/// en-SL: (en-Latn-SL) English, Latin, Sierra Leone
pub const fn get_en_sl() -> LangID {
  ID::new(28261, None, Some(19539)).get_id()
}

/// en-SS: (en-Latn-SS) English, Latin, South Sudan
pub const fn get_en_ss() -> LangID {
  ID::new(28261, None, Some(21331)).get_id()
}

/// en-SX: (en-Latn-SX) English, Latin, Sint Maarten
pub const fn get_en_sx() -> LangID {
  ID::new(28261, None, Some(22611)).get_id()
}

/// en-SZ: (en-Latn-SZ) English, Latin, Eswatini
pub const fn get_en_sz() -> LangID {
  ID::new(28261, None, Some(23123)).get_id()
}

/// en-TC: (en-Latn-TC) English, Latin, Turks & Caicos Islands
pub const fn get_en_tc() -> LangID {
  ID::new(28261, None, Some(17236)).get_id()
}

/// en-TK: (en-Latn-TK) English, Latin, Tokelau
pub const fn get_en_tk() -> LangID {
  ID::new(28261, None, Some(19284)).get_id()
}

/// en-TO: (en-Latn-TO) English, Latin, Tonga
pub const fn get_en_to() -> LangID {
  ID::new(28261, None, Some(20308)).get_id()
}

/// en-TT: (en-Latn-TT) English, Latin, Trinidad & Tobago
pub const fn get_en_tt() -> LangID {
  ID::new(28261, None, Some(21588)).get_id()
}

/// en-TV: (en-Latn-TV) English, Latin, Tuvalu
pub const fn get_en_tv() -> LangID {
  ID::new(28261, None, Some(22100)).get_id()
}

/// en-TZ: (en-Latn-TZ) English, Latin, Tanzania
pub const fn get_en_tz() -> LangID {
  ID::new(28261, None, Some(23124)).get_id()
}

/// en-UG: (en-Latn-UG) English, Latin, Uganda
pub const fn get_en_ug() -> LangID {
  ID::new(28261, None, Some(18261)).get_id()
}

/// en-UM: (en-Latn-UM) English, Latin, U.S. Outlying Islands
pub const fn get_en_um() -> LangID {
  ID::new(28261, None, Some(19797)).get_id()
}

/// en-VC: (en-Latn-VC) English, Latin, St Vincent & the Grenadines
pub const fn get_en_vc() -> LangID {
  ID::new(28261, None, Some(17238)).get_id()
}

/// en-VG: (en-Latn-VG) English, Latin, British Virgin Islands
pub const fn get_en_vg() -> LangID {
  ID::new(28261, None, Some(18262)).get_id()
}

/// en-VI: (en-Latn-VI) English, Latin, U.S. Virgin Islands
pub const fn get_en_vi() -> LangID {
  ID::new(28261, None, Some(18774)).get_id()
}

/// en-VU: (en-Latn-VU) English, Latin, Vanuatu
pub const fn get_en_vu() -> LangID {
  ID::new(28261, None, Some(21846)).get_id()
}

/// en-WS: (en-Latn-WS) English, Latin, Samoa
pub const fn get_en_ws() -> LangID {
  ID::new(28261, None, Some(21335)).get_id()
}

/// en-ZA: (en-Latn-ZA) English, Latin, South Africa
pub const fn get_en_za() -> LangID {
  ID::new(28261, None, Some(16730)).get_id()
}

/// en-ZM: (en-Latn-ZM) English, Latin, Zambia
pub const fn get_en_zm() -> LangID {
  ID::new(28261, None, Some(19802)).get_id()
}

/// en-ZW: (en-Latn-ZW) English, Latin, Zimbabwe
pub const fn get_en_zw() -> LangID {
  ID::new(28261, None, Some(22362)).get_id()
}

/// eo: (eo-Latn-001) esperanto, Latn, Mondo
pub const fn get_eo() -> LangID {
  ID::new(28517, None, None).get_id()
}

/// es: (es-Latn-ES) español, latino, España
pub const fn get_es() -> LangID {
  ID::new(29541, None, None).get_id()
}

/// es-419: (es-Latn-419) español, latín, Latinoamérica
pub const fn get_es_419() -> LangID {
  ID::new(29541, None, Some(3748148)).get_id()
}

/// es-AR: (es-Latn-AR) español, latín, Argentina
pub const fn get_es_ar() -> LangID {
  ID::new(29541, None, Some(21057)).get_id()
}

/// es-BO: (es-Latn-BO) español, latín, Bolivia
pub const fn get_es_bo() -> LangID {
  ID::new(29541, None, Some(20290)).get_id()
}

/// es-BR: (es-Latn-BR) español, latín, Brasil
pub const fn get_es_br() -> LangID {
  ID::new(29541, None, Some(21058)).get_id()
}

/// es-BZ: (es-Latn-BZ) español, latín, Belice
pub const fn get_es_bz() -> LangID {
  ID::new(29541, None, Some(23106)).get_id()
}

/// es-CL: (es-Latn-CL) español, latín, Chile
pub const fn get_es_cl() -> LangID {
  ID::new(29541, None, Some(19523)).get_id()
}

/// es-CO: (es-Latn-CO) español, latín, Colombia
pub const fn get_es_co() -> LangID {
  ID::new(29541, None, Some(20291)).get_id()
}

/// es-CR: (es-Latn-CR) español, latín, Costa Rica
pub const fn get_es_cr() -> LangID {
  ID::new(29541, None, Some(21059)).get_id()
}

/// es-CU: (es-Latn-CU) español, latín, Cuba
pub const fn get_es_cu() -> LangID {
  ID::new(29541, None, Some(21827)).get_id()
}

/// es-DO: (es-Latn-DO) español, latín, República Dominicana
pub const fn get_es_do() -> LangID {
  ID::new(29541, None, Some(20292)).get_id()
}

/// es-EA: (es-Latn-EA) español, latino, Ceuta y Melilla
pub const fn get_es_ea() -> LangID {
  ID::new(29541, None, Some(16709)).get_id()
}

/// es-EC: (es-Latn-EC) español, latín, Ecuador
pub const fn get_es_ec() -> LangID {
  ID::new(29541, None, Some(17221)).get_id()
}

/// es-GQ: (es-Latn-GQ) español, latino, Guinea Ecuatorial
pub const fn get_es_gq() -> LangID {
  ID::new(29541, None, Some(20807)).get_id()
}

/// es-GT: (es-Latn-GT) español, latín, Guatemala
pub const fn get_es_gt() -> LangID {
  ID::new(29541, None, Some(21575)).get_id()
}

/// es-HN: (es-Latn-HN) español, latín, Honduras
pub const fn get_es_hn() -> LangID {
  ID::new(29541, None, Some(20040)).get_id()
}

/// es-IC: (es-Latn-IC) español, latino, Canarias
pub const fn get_es_ic() -> LangID {
  ID::new(29541, None, Some(17225)).get_id()
}

/// es-MX: (es-Latn-MX) español, latín, México
pub const fn get_es_mx() -> LangID {
  ID::new(29541, None, Some(22605)).get_id()
}

/// es-NI: (es-Latn-NI) español, latín, Nicaragua
pub const fn get_es_ni() -> LangID {
  ID::new(29541, None, Some(18766)).get_id()
}

/// es-PA: (es-Latn-PA) español, latín, Panamá
pub const fn get_es_pa() -> LangID {
  ID::new(29541, None, Some(16720)).get_id()
}

/// es-PE: (es-Latn-PE) español, latín, Perú
pub const fn get_es_pe() -> LangID {
  ID::new(29541, None, Some(17744)).get_id()
}

/// es-PH: (es-Latn-PH) español, latino, Filipinas
pub const fn get_es_ph() -> LangID {
  ID::new(29541, None, Some(18512)).get_id()
}

/// es-PR: (es-Latn-PR) español, latín, Puerto Rico
pub const fn get_es_pr() -> LangID {
  ID::new(29541, None, Some(21072)).get_id()
}

/// es-PY: (es-Latn-PY) español, latín, Paraguay
pub const fn get_es_py() -> LangID {
  ID::new(29541, None, Some(22864)).get_id()
}

/// es-SV: (es-Latn-SV) español, latín, El Salvador
pub const fn get_es_sv() -> LangID {
  ID::new(29541, None, Some(22099)).get_id()
}

/// es-US: (es-Latn-US) español, latín, Estados Unidos
pub const fn get_es_us() -> LangID {
  ID::new(29541, None, Some(21333)).get_id()
}

/// es-UY: (es-Latn-UY) español, latín, Uruguay
pub const fn get_es_uy() -> LangID {
  ID::new(29541, None, Some(22869)).get_id()
}

/// es-VE: (es-Latn-VE) español, latín, Venezuela
pub const fn get_es_ve() -> LangID {
  ID::new(29541, None, Some(17750)).get_id()
}

/// et: (et-Latn-EE) eesti, ladina, Eesti
pub const fn get_et() -> LangID {
  ID::new(29797, None, None).get_id()
}

/// eu: (eu-Latn-ES) euskara, latinoa, Espainia
pub const fn get_eu() -> LangID {
  ID::new(30053, None, None).get_id()
}

/// ewo: (ewo-Latn-CM) ewondo, Latn, Kamərún
pub const fn get_ewo() -> LangID {
  ID::new(7305061, None, None).get_id()
}

/// fa: (fa-Arab-IR) فارسی, عربی, ایران
pub const fn get_fa() -> LangID {
  ID::new(24934, None, None).get_id()
}

/// fa-AF: (fa-Arab-AF) فارسی, عربی, افغانستان
pub const fn get_fa_af() -> LangID {
  ID::new(24934, None, Some(17985)).get_id()
}

/// ff: (ff-Latn-SN) Pulaar, Latn, Senegaal
pub const fn get_ff() -> LangID {
  ID::new(26214, None, None).get_id()
}

/// ff-Adlm: (ff-Adlm-GN) 𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤘𞤭𞤲𞤫
pub const fn get_ff_adlm() -> LangID {
  ID::new(26214, Some(1835820097), None).get_id()
}

/// ff-Adlm-BF: 𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤄𞤵𞤪𞤳𞤭𞤲𞤢 𞤊𞤢𞤧𞤮𞥅
pub const fn get_ff_adlm_bf() -> LangID {
  ID::new(26214, Some(1835820097), Some(17986)).get_id()
}

/// ff-Adlm-CM: 𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤑𞤢𞤥𞤢𞤪𞤵𞥅𞤲
pub const fn get_ff_adlm_cm() -> LangID {
  ID::new(26214, Some(1835820097), Some(19779)).get_id()
}

/// ff-Adlm-GH: 𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤘𞤢𞤲𞤢
pub const fn get_ff_adlm_gh() -> LangID {
  ID::new(26214, Some(1835820097), Some(18503)).get_id()
}

/// ff-Adlm-GM: 𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤘𞤢𞤥𞤦𞤭𞤴𞤢
pub const fn get_ff_adlm_gm() -> LangID {
  ID::new(26214, Some(1835820097), Some(19783)).get_id()
}

/// ff-Adlm-GW: 𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤘𞤭𞤲𞤫-𞤄𞤭𞤧𞤢𞤱𞤮𞥅
pub const fn get_ff_adlm_gw() -> LangID {
  ID::new(26214, Some(1835820097), Some(22343)).get_id()
}

/// ff-Adlm-LR: 𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤂𞤢𞤦𞤭𞤪𞤭𞤴𞤢𞥄
pub const fn get_ff_adlm_lr() -> LangID {
  ID::new(26214, Some(1835820097), Some(21068)).get_id()
}

/// ff-Adlm-MR: 𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤃𞤮𞤪𞤼𞤢𞤲𞤭𞥅
pub const fn get_ff_adlm_mr() -> LangID {
  ID::new(26214, Some(1835820097), Some(21069)).get_id()
}

/// ff-Adlm-NE: 𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤐𞤭𞥅𞤶𞤫𞤪
pub const fn get_ff_adlm_ne() -> LangID {
  ID::new(26214, Some(1835820097), Some(17742)).get_id()
}

/// ff-Adlm-NG: 𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤐𞤢𞤶𞤫𞤪𞤭𞤴𞤢𞥄
pub const fn get_ff_adlm_ng() -> LangID {
  ID::new(26214, Some(1835820097), Some(18254)).get_id()
}

/// ff-Adlm-SL: 𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤅𞤢𞤪𞤢𞤤𞤮𞤲
pub const fn get_ff_adlm_sl() -> LangID {
  ID::new(26214, Some(1835820097), Some(19539)).get_id()
}

/// ff-Adlm-SN: 𞤆𞤵𞤤𞤢𞤪, 𞤀𞤁𞤂𞤢𞤃, 𞤅𞤫𞤲𞤫𞤺𞤢𞥄𞤤
pub const fn get_ff_adlm_sn() -> LangID {
  ID::new(26214, Some(1835820097), Some(20051)).get_id()
}

/// ff-Latn: (ff-Latn-SN) Pulaar, Latn, Senegaal
pub const fn get_ff_latn() -> LangID {
  ID::new(26214, Some(1853120844), None).get_id()
}

/// ff-Latn-BF: Pulaar, Latn, Burkibaa Faaso
pub const fn get_ff_latn_bf() -> LangID {
  ID::new(26214, Some(1853120844), Some(17986)).get_id()
}

/// ff-Latn-CM: Pulaar, Latn, Kameruun
pub const fn get_ff_latn_cm() -> LangID {
  ID::new(26214, Some(1853120844), Some(19779)).get_id()
}

/// ff-Latn-GH: Pulaar, Latn, Ganaa
pub const fn get_ff_latn_gh() -> LangID {
  ID::new(26214, Some(1853120844), Some(18503)).get_id()
}

/// ff-Latn-GM: Pulaar, Latn, Gammbi
pub const fn get_ff_latn_gm() -> LangID {
  ID::new(26214, Some(1853120844), Some(19783)).get_id()
}

/// ff-Latn-GN: Pulaar, Latn, Gine
pub const fn get_ff_latn_gn() -> LangID {
  ID::new(26214, Some(1853120844), Some(20039)).get_id()
}

/// ff-Latn-GW: Pulaar, Latn, Gine-Bisaawo
pub const fn get_ff_latn_gw() -> LangID {
  ID::new(26214, Some(1853120844), Some(22343)).get_id()
}

/// ff-Latn-LR: Pulaar, Latn, Liberiyaa
pub const fn get_ff_latn_lr() -> LangID {
  ID::new(26214, Some(1853120844), Some(21068)).get_id()
}

/// ff-Latn-MR: Pulaar, Latn, Muritani
pub const fn get_ff_latn_mr() -> LangID {
  ID::new(26214, Some(1853120844), Some(21069)).get_id()
}

/// ff-Latn-NE: Pulaar, Latn, Nijeer
pub const fn get_ff_latn_ne() -> LangID {
  ID::new(26214, Some(1853120844), Some(17742)).get_id()
}

/// ff-Latn-NG: Pulaar, Latn, Nijeriyaa
pub const fn get_ff_latn_ng() -> LangID {
  ID::new(26214, Some(1853120844), Some(18254)).get_id()
}

/// ff-Latn-SL: Pulaar, Latn, Seraa liyon
pub const fn get_ff_latn_sl() -> LangID {
  ID::new(26214, Some(1853120844), Some(19539)).get_id()
}

/// fi: (fi-Latn-FI) suomi, latinalainen, Suomi
pub const fn get_fi() -> LangID {
  ID::new(26982, None, None).get_id()
}

/// fil: (fil-Latn-PH) Filipino, Latin, Pilipinas
pub const fn get_fil() -> LangID {
  ID::new(7104870, None, None).get_id()
}

/// fo: (fo-Latn-FO) føroyskt, latínskt, Føroyar
pub const fn get_fo() -> LangID {
  ID::new(28518, None, None).get_id()
}

/// fo-DK: (fo-Latn-DK) føroyskt, latínskt, Danmark
pub const fn get_fo_dk() -> LangID {
  ID::new(28518, None, Some(19268)).get_id()
}

/// fr: (fr-Latn-FR) français, latin, France
pub const fn get_fr() -> LangID {
  ID::new(29286, None, None).get_id()
}

/// fr-BE: (fr-Latn-BE) français, latin, Belgique
pub const fn get_fr_be() -> LangID {
  ID::new(29286, None, Some(17730)).get_id()
}

/// fr-BF: (fr-Latn-BF) français, latin, Burkina Faso
pub const fn get_fr_bf() -> LangID {
  ID::new(29286, None, Some(17986)).get_id()
}

/// fr-BI: (fr-Latn-BI) français, latin, Burundi
pub const fn get_fr_bi() -> LangID {
  ID::new(29286, None, Some(18754)).get_id()
}

/// fr-BJ: (fr-Latn-BJ) français, latin, Bénin
pub const fn get_fr_bj() -> LangID {
  ID::new(29286, None, Some(19010)).get_id()
}

/// fr-BL: (fr-Latn-BL) français, latin, Saint-Barthélemy
pub const fn get_fr_bl() -> LangID {
  ID::new(29286, None, Some(19522)).get_id()
}

/// fr-CA: (fr-Latn-CA) français, latin, Canada
pub const fn get_fr_ca() -> LangID {
  ID::new(29286, None, Some(16707)).get_id()
}

/// fr-CD: (fr-Latn-CD) français, latin, Congo-Kinshasa
pub const fn get_fr_cd() -> LangID {
  ID::new(29286, None, Some(17475)).get_id()
}

/// fr-CF: (fr-Latn-CF) français, latin, République centrafricaine
pub const fn get_fr_cf() -> LangID {
  ID::new(29286, None, Some(17987)).get_id()
}

/// fr-CG: (fr-Latn-CG) français, latin, Congo-Brazzaville
pub const fn get_fr_cg() -> LangID {
  ID::new(29286, None, Some(18243)).get_id()
}

/// fr-CH: (fr-Latn-CH) français, latin, Suisse
pub const fn get_fr_ch() -> LangID {
  ID::new(29286, None, Some(18499)).get_id()
}

/// fr-CI: (fr-Latn-CI) français, latin, Côte d’Ivoire
pub const fn get_fr_ci() -> LangID {
  ID::new(29286, None, Some(18755)).get_id()
}

/// fr-CM: (fr-Latn-CM) français, latin, Cameroun
pub const fn get_fr_cm() -> LangID {
  ID::new(29286, None, Some(19779)).get_id()
}

/// fr-DJ: (fr-Latn-DJ) français, latin, Djibouti
pub const fn get_fr_dj() -> LangID {
  ID::new(29286, None, Some(19012)).get_id()
}

/// fr-DZ: (fr-Latn-DZ) français, latin, Algérie
pub const fn get_fr_dz() -> LangID {
  ID::new(29286, None, Some(23108)).get_id()
}

/// fr-GA: (fr-Latn-GA) français, latin, Gabon
pub const fn get_fr_ga() -> LangID {
  ID::new(29286, None, Some(16711)).get_id()
}

/// fr-GF: (fr-Latn-GF) français, latin, Guyane française
pub const fn get_fr_gf() -> LangID {
  ID::new(29286, None, Some(17991)).get_id()
}

/// fr-GN: (fr-Latn-GN) français, latin, Guinée
pub const fn get_fr_gn() -> LangID {
  ID::new(29286, None, Some(20039)).get_id()
}

/// fr-GP: (fr-Latn-GP) français, latin, Guadeloupe
pub const fn get_fr_gp() -> LangID {
  ID::new(29286, None, Some(20551)).get_id()
}

/// fr-GQ: (fr-Latn-GQ) français, latin, Guinée équatoriale
pub const fn get_fr_gq() -> LangID {
  ID::new(29286, None, Some(20807)).get_id()
}

/// fr-HT: (fr-Latn-HT) français, latin, Haïti
pub const fn get_fr_ht() -> LangID {
  ID::new(29286, None, Some(21576)).get_id()
}

/// fr-KM: (fr-Latn-KM) français, latin, Comores
pub const fn get_fr_km() -> LangID {
  ID::new(29286, None, Some(19787)).get_id()
}

/// fr-LU: (fr-Latn-LU) français, latin, Luxembourg
pub const fn get_fr_lu() -> LangID {
  ID::new(29286, None, Some(21836)).get_id()
}

/// fr-MA: (fr-Latn-MA) français, latin, Maroc
pub const fn get_fr_ma() -> LangID {
  ID::new(29286, None, Some(16717)).get_id()
}

/// fr-MC: (fr-Latn-MC) français, latin, Monaco
pub const fn get_fr_mc() -> LangID {
  ID::new(29286, None, Some(17229)).get_id()
}

/// fr-MF: (fr-Latn-MF) français, latin, Saint-Martin
pub const fn get_fr_mf() -> LangID {
  ID::new(29286, None, Some(17997)).get_id()
}

/// fr-MG: (fr-Latn-MG) français, latin, Madagascar
pub const fn get_fr_mg() -> LangID {
  ID::new(29286, None, Some(18253)).get_id()
}

/// fr-ML: (fr-Latn-ML) français, latin, Mali
pub const fn get_fr_ml() -> LangID {
  ID::new(29286, None, Some(19533)).get_id()
}

/// fr-MQ: (fr-Latn-MQ) français, latin, Martinique
pub const fn get_fr_mq() -> LangID {
  ID::new(29286, None, Some(20813)).get_id()
}

/// fr-MR: (fr-Latn-MR) français, latin, Mauritanie
pub const fn get_fr_mr() -> LangID {
  ID::new(29286, None, Some(21069)).get_id()
}

/// fr-MU: (fr-Latn-MU) français, latin, Maurice
pub const fn get_fr_mu() -> LangID {
  ID::new(29286, None, Some(21837)).get_id()
}

/// fr-NC: (fr-Latn-NC) français, latin, Nouvelle-Calédonie
pub const fn get_fr_nc() -> LangID {
  ID::new(29286, None, Some(17230)).get_id()
}

/// fr-NE: (fr-Latn-NE) français, latin, Niger
pub const fn get_fr_ne() -> LangID {
  ID::new(29286, None, Some(17742)).get_id()
}

/// fr-PF: (fr-Latn-PF) français, latin, Polynésie française
pub const fn get_fr_pf() -> LangID {
  ID::new(29286, None, Some(18000)).get_id()
}

/// fr-PM: (fr-Latn-PM) français, latin, Saint-Pierre-et-Miquelon
pub const fn get_fr_pm() -> LangID {
  ID::new(29286, None, Some(19792)).get_id()
}

/// fr-RE: (fr-Latn-RE) français, latin, La Réunion
pub const fn get_fr_re() -> LangID {
  ID::new(29286, None, Some(17746)).get_id()
}

/// fr-RW: (fr-Latn-RW) français, latin, Rwanda
pub const fn get_fr_rw() -> LangID {
  ID::new(29286, None, Some(22354)).get_id()
}

/// fr-SC: (fr-Latn-SC) français, latin, Seychelles
pub const fn get_fr_sc() -> LangID {
  ID::new(29286, None, Some(17235)).get_id()
}

/// fr-SN: (fr-Latn-SN) français, latin, Sénégal
pub const fn get_fr_sn() -> LangID {
  ID::new(29286, None, Some(20051)).get_id()
}

/// fr-SY: (fr-Latn-SY) français, latin, Syrie
pub const fn get_fr_sy() -> LangID {
  ID::new(29286, None, Some(22867)).get_id()
}

/// fr-TD: (fr-Latn-TD) français, latin, Tchad
pub const fn get_fr_td() -> LangID {
  ID::new(29286, None, Some(17492)).get_id()
}

/// fr-TG: (fr-Latn-TG) français, latin, Togo
pub const fn get_fr_tg() -> LangID {
  ID::new(29286, None, Some(18260)).get_id()
}

/// fr-TN: (fr-Latn-TN) français, latin, Tunisie
pub const fn get_fr_tn() -> LangID {
  ID::new(29286, None, Some(20052)).get_id()
}

/// fr-VU: (fr-Latn-VU) français, latin, Vanuatu
pub const fn get_fr_vu() -> LangID {
  ID::new(29286, None, Some(21846)).get_id()
}

/// fr-WF: (fr-Latn-WF) français, latin, Wallis-et-Futuna
pub const fn get_fr_wf() -> LangID {
  ID::new(29286, None, Some(18007)).get_id()
}

/// fr-YT: (fr-Latn-YT) français, latin, Mayotte
pub const fn get_fr_yt() -> LangID {
  ID::new(29286, None, Some(21593)).get_id()
}

/// frr: (frr-Latn-DE) Nordfriisk, Latn, DE
pub const fn get_frr() -> LangID {
  ID::new(7500390, None, None).get_id()
}

/// fur: (fur-Latn-IT) furlan, latin, Italie
pub const fn get_fur() -> LangID {
  ID::new(7501158, None, None).get_id()
}

/// fy: (fy-Latn-NL) Frysk, Latyn, Nederlân
pub const fn get_fy() -> LangID {
  ID::new(31078, None, None).get_id()
}

/// ga: (ga-Latn-IE) Gaeilge, Laidineach, Éire
pub const fn get_ga() -> LangID {
  ID::new(24935, None, None).get_id()
}

/// ga-GB: (ga-Latn-GB) Gaeilge, Laidineach, an Ríocht Aontaithe
pub const fn get_ga_gb() -> LangID {
  ID::new(24935, None, Some(16967)).get_id()
}

/// gd: (gd-Latn-GB) Gàidhlig, Laideann, An Rìoghachd Aonaichte
pub const fn get_gd() -> LangID {
  ID::new(25703, None, None).get_id()
}

/// gl: (gl-Latn-ES) galego, latino, España
pub const fn get_gl() -> LangID {
  ID::new(27751, None, None).get_id()
}

/// gsw: (gsw-Latn-CH) Schwiizertüütsch, Latiinisch, Schwiiz
pub const fn get_gsw() -> LangID {
  ID::new(7828327, None, None).get_id()
}

/// gsw-FR: (gsw-Latn-FR) Schwiizertüütsch, Latiinisch, Frankriich
pub const fn get_gsw_fr() -> LangID {
  ID::new(7828327, None, Some(21062)).get_id()
}

/// gsw-LI: (gsw-Latn-LI) Schwiizertüütsch, Latiinisch, Liächteschtäi
pub const fn get_gsw_li() -> LangID {
  ID::new(7828327, None, Some(18764)).get_id()
}

/// gu: (gu-Gujr-IN) ગુજરાતી, ગુજરાતી, ભારત
pub const fn get_gu() -> LangID {
  ID::new(30055, None, None).get_id()
}

/// guz: (guz-Latn-KE) Ekegusii, Latn, Kenya
pub const fn get_guz() -> LangID {
  ID::new(8025447, None, None).get_id()
}

/// gv: (gv-Latn-IM) Gaelg, Latn, Ellan Vannin
pub const fn get_gv() -> LangID {
  ID::new(30311, None, None).get_id()
}

/// ha: (ha-Latn-NG) Hausa, Latin, Nijeriya
pub const fn get_ha() -> LangID {
  ID::new(24936, None, None).get_id()
}

/// ha-GH: (ha-Latn-GH) Hausa, Latin, Gana
pub const fn get_ha_gh() -> LangID {
  ID::new(24936, None, Some(18503)).get_id()
}

/// ha-NE: (ha-Latn-NE) Hausa, Latin, Nijar
pub const fn get_ha_ne() -> LangID {
  ID::new(24936, None, Some(17742)).get_id()
}

/// haw: (haw-Latn-US) ʻŌlelo Hawaiʻi, Latn, ʻAmelika Hui Pū ʻIa
pub const fn get_haw() -> LangID {
  ID::new(7823720, None, None).get_id()
}

/// he: (he-Hebr-IL) עברית, עברי, ישראל
pub const fn get_he() -> LangID {
  ID::new(25960, None, None).get_id()
}

/// hi: (hi-Deva-IN) हिन्दी, देवनागरी, भारत
pub const fn get_hi() -> LangID {
  ID::new(26984, None, None).get_id()
}

/// hi-Latn: (hi-Latn-IN) Hindi (Latin), India
pub const fn get_hi_latn() -> LangID {
  ID::new(26984, Some(1853120844), None).get_id()
}

/// hr: (hr-Latn-HR) hrvatski, latinica, Hrvatska
pub const fn get_hr() -> LangID {
  ID::new(29288, None, None).get_id()
}

/// hr-BA: (hr-Latn-BA) hrvatski, latinica, Bosna i Hercegovina
pub const fn get_hr_ba() -> LangID {
  ID::new(29288, None, Some(16706)).get_id()
}

/// hsb: (hsb-Latn-DE) hornjoserbšćina, łaćonsce, Němska
pub const fn get_hsb() -> LangID {
  ID::new(6452072, None, None).get_id()
}

/// hu: (hu-Latn-HU) magyar, Latin, Magyarország
pub const fn get_hu() -> LangID {
  ID::new(30056, None, None).get_id()
}

/// hy: (hy-Armn-AM) հայերեն, հայկական, Հայաստան
pub const fn get_hy() -> LangID {
  ID::new(31080, None, None).get_id()
}

/// ia: (ia-Latn-001) interlingua, latin, Mundo
pub const fn get_ia() -> LangID {
  ID::new(24937, None, None).get_id()
}

/// id: (id-Latn-ID) Indonesia, Latin, Indonesia
pub const fn get_id() -> LangID {
  ID::new(25705, None, None).get_id()
}

/// ig: (ig-Latn-NG) Igbo, Latin, Naịjịrịa
pub const fn get_ig() -> LangID {
  ID::new(26473, None, None).get_id()
}

/// ii: (ii-Yiii-CN) ꆈꌠꉙ, ꆈꌠꁱꂷ, ꍏꇩ
pub const fn get_ii() -> LangID {
  ID::new(26985, None, None).get_id()
}

/// is: (is-Latn-IS) íslenska, latneskt, Ísland
pub const fn get_is() -> LangID {
  ID::new(29545, None, None).get_id()
}

/// it: (it-Latn-IT) italiano, latino, Italia
pub const fn get_it() -> LangID {
  ID::new(29801, None, None).get_id()
}

/// it-CH: (it-Latn-CH) italiano, latino, Svizzera
pub const fn get_it_ch() -> LangID {
  ID::new(29801, None, Some(18499)).get_id()
}

/// it-SM: (it-Latn-SM) italiano, latino, San Marino
pub const fn get_it_sm() -> LangID {
  ID::new(29801, None, Some(19795)).get_id()
}

/// it-VA: (it-Latn-VA) italiano, latino, Città del Vaticano
pub const fn get_it_va() -> LangID {
  ID::new(29801, None, Some(16726)).get_id()
}

/// ja: (ja-Jpan-JP) 日本語, 日本語の文字, 日本
pub const fn get_ja() -> LangID {
  ID::new(24938, None, None).get_id()
}

/// jgo: (jgo-Latn-CM) Ndaꞌa, mík -ŋwaꞌnɛ yi ɛ́ líŋɛ́nɛ Latɛ̂ŋ, Kamɛlûn
pub const fn get_jgo() -> LangID {
  ID::new(7300970, None, None).get_id()
}

/// jmc: (jmc-Latn-TZ) Kimachame, Latn, Tanzania
pub const fn get_jmc() -> LangID {
  ID::new(6516074, None, None).get_id()
}

/// jv: (jv-Latn-ID) Jawa, Latin, Indonésia
pub const fn get_jv() -> LangID {
  ID::new(30314, None, None).get_id()
}

/// ka: (ka-Geor-GE) ქართული, ქართული, საქართველო
pub const fn get_ka() -> LangID {
  ID::new(24939, None, None).get_id()
}

/// kab: (kab-Latn-DZ) Taqbaylit, Latn, Lezzayer
pub const fn get_kab() -> LangID {
  ID::new(6447467, None, None).get_id()
}

/// kam: (kam-Latn-KE) Kikamba, Latn, Kenya
pub const fn get_kam() -> LangID {
  ID::new(7168363, None, None).get_id()
}

/// kde: (kde-Latn-TZ) Chimakonde, Latn, Tanzania
pub const fn get_kde() -> LangID {
  ID::new(6644843, None, None).get_id()
}

/// kea: (kea-Latn-CV) kabuverdianu, latinu, Kabu Verdi
pub const fn get_kea() -> LangID {
  ID::new(6382955, None, None).get_id()
}

/// kgp: (kgp-Latn-BR) kanhgág, ratĩnh, Mrasir
pub const fn get_kgp() -> LangID {
  ID::new(7366507, None, None).get_id()
}

/// khq: (khq-Latn-ML) Koyra ciini, Latn, Maali
pub const fn get_khq() -> LangID {
  ID::new(7432299, None, None).get_id()
}

/// ki: (ki-Latn-KE) Gikuyu, Latn, Kenya
pub const fn get_ki() -> LangID {
  ID::new(26987, None, None).get_id()
}

/// kk: (kk-Cyrl-KZ) қазақ тілі, кирилл жазуы, Қазақстан
pub const fn get_kk() -> LangID {
  ID::new(27499, None, None).get_id()
}

/// kkj: (kkj-Latn-CM) kakɔ, Latn, Kamɛrun
pub const fn get_kkj() -> LangID {
  ID::new(6974315, None, None).get_id()
}

/// kl: (kl-Latn-GL) kalaallisut, Latn, Kalaallit Nunaat
pub const fn get_kl() -> LangID {
  ID::new(27755, None, None).get_id()
}

/// kln: (kln-Latn-KE) Kalenjin, Latn, Emetab Kenya
pub const fn get_kln() -> LangID {
  ID::new(7236715, None, None).get_id()
}

/// km: (km-Khmr-KH) ខ្មែរ, ខ្មែរ, កម្ពុជា
pub const fn get_km() -> LangID {
  ID::new(28011, None, None).get_id()
}

/// kn: (kn-Knda-IN) ಕನ್ನಡ, ಕನ್ನಡ, ಭಾರತ
pub const fn get_kn() -> LangID {
  ID::new(28267, None, None).get_id()
}

/// ko: (ko-Kore-KR) 한국어, 한국 문자, 대한민국
pub const fn get_ko() -> LangID {
  ID::new(28523, None, None).get_id()
}

/// ko-KP: (ko-Kore-KP) 한국어, 한국 문자, 조선민주주의인민공화국
pub const fn get_ko_kp() -> LangID {
  ID::new(28523, None, Some(20555)).get_id()
}

/// kok: (kok-Deva-IN) कोंकणी, देवनागरी, भारत
pub const fn get_kok() -> LangID {
  ID::new(7040875, None, None).get_id()
}

/// ks: (ks-Arab-IN) کٲشُر, عربی, ہِندوستان
pub const fn get_ks() -> LangID {
  ID::new(29547, None, None).get_id()
}

/// ks-Arab: (ks-Arab-IN) کٲشُر, عربی, ہِندوستان
pub const fn get_ks_arab() -> LangID {
  ID::new(29547, Some(1650553409), None).get_id()
}

/// ks-Deva: (ks-Deva-IN) कॉशुर, देवनागरी, हिंदोस्तान
pub const fn get_ks_deva() -> LangID {
  ID::new(29547, Some(1635149124), None).get_id()
}

/// ksb: (ksb-Latn-TZ) Kishambaa, Latn, Tanzania
pub const fn get_ksb() -> LangID {
  ID::new(6452075, None, None).get_id()
}

/// ksf: (ksf-Latn-CM) rikpa, Latn, kamɛrún
pub const fn get_ksf() -> LangID {
  ID::new(6714219, None, None).get_id()
}

/// ksh: (ksh-Latn-DE) Kölsch, lateinesche Schreff, Doütschland
pub const fn get_ksh() -> LangID {
  ID::new(6845291, None, None).get_id()
}

/// ku: (ku-Latn-TR) kurdî, latînî, Tirkiye
pub const fn get_ku() -> LangID {
  ID::new(30059, None, None).get_id()
}

/// kw: (kw-Latn-GB) kernewek, Latn, Rywvaneth Unys
pub const fn get_kw() -> LangID {
  ID::new(30571, None, None).get_id()
}

/// ky: (ky-Cyrl-KG) кыргызча, Кирилл, Кыргызстан
pub const fn get_ky() -> LangID {
  ID::new(31083, None, None).get_id()
}

/// lag: (lag-Latn-TZ) Kɨlaangi, Latn, Taansanía
pub const fn get_lag() -> LangID {
  ID::new(6775148, None, None).get_id()
}

/// lb: (lb-Latn-LU) Lëtzebuergesch, Laténgesch, Lëtzebuerg
pub const fn get_lb() -> LangID {
  ID::new(25196, None, None).get_id()
}

/// lg: (lg-Latn-UG) Luganda, Latn, Yuganda
pub const fn get_lg() -> LangID {
  ID::new(26476, None, None).get_id()
}

/// lkt: (lkt-Latn-US) Lakȟólʼiyapi, Latn, Mílahaŋska Tȟamákȟočhe
pub const fn get_lkt() -> LangID {
  ID::new(7629676, None, None).get_id()
}

/// ln: (ln-Latn-CD) lingála, Latn, Republíki ya Kongó Demokratíki
pub const fn get_ln() -> LangID {
  ID::new(28268, None, None).get_id()
}

/// ln-AO: (ln-Latn-AO) lingála, Latn, Angóla
pub const fn get_ln_ao() -> LangID {
  ID::new(28268, None, Some(20289)).get_id()
}

/// ln-CF: (ln-Latn-CF) lingála, Latn, Repibiki ya Afríka ya Káti
pub const fn get_ln_cf() -> LangID {
  ID::new(28268, None, Some(17987)).get_id()
}

/// ln-CG: (ln-Latn-CG) lingála, Latn, Kongo
pub const fn get_ln_cg() -> LangID {
  ID::new(28268, None, Some(18243)).get_id()
}

/// lo: (lo-Laoo-LA) ລາວ, ລາວ, ລາວ
pub const fn get_lo() -> LangID {
  ID::new(28524, None, None).get_id()
}

/// lrc: (lrc-Arab-IR) لۊری شومالی, عأرأڤی, IR
pub const fn get_lrc() -> LangID {
  ID::new(6517356, None, None).get_id()
}

/// lrc-IQ: (lrc-Arab-IQ) لۊری شومالی, عأرأڤی, IQ
pub const fn get_lrc_iq() -> LangID {
  ID::new(6517356, None, Some(20809)).get_id()
}

/// lt: (lt-Latn-LT) lietuvių, lotynų, Lietuva
pub const fn get_lt() -> LangID {
  ID::new(29804, None, None).get_id()
}

/// lu: (lu-Latn-CD) Tshiluba, Latn, Ditunga wa Kongu
pub const fn get_lu() -> LangID {
  ID::new(30060, None, None).get_id()
}

/// luo: (luo-Latn-KE) Dholuo, Latn, Kenya
pub const fn get_luo() -> LangID {
  ID::new(7304556, None, None).get_id()
}

/// luy: (luy-Latn-KE) Luluhia, Latn, Kenya
pub const fn get_luy() -> LangID {
  ID::new(7959916, None, None).get_id()
}

/// lv: (lv-Latn-LV) latviešu, latīņu, Latvija
pub const fn get_lv() -> LangID {
  ID::new(30316, None, None).get_id()
}

/// mai: (mai-Deva-IN) मैथिली, देवनागरी, भारत
pub const fn get_mai() -> LangID {
  ID::new(6906221, None, None).get_id()
}

/// mas: (mas-Latn-KE) Maa, Latn, Kenya
pub const fn get_mas() -> LangID {
  ID::new(7561581, None, None).get_id()
}

/// mas-TZ: (mas-Latn-TZ) Maa, Latn, Tansania
pub const fn get_mas_tz() -> LangID {
  ID::new(7561581, None, Some(23124)).get_id()
}

/// mdf: (mdf-Cyrl-RU) мокшень кяль, Cyrl, RU
pub const fn get_mdf() -> LangID {
  ID::new(6710381, None, None).get_id()
}

/// mer: (mer-Latn-KE) Kĩmĩrũ, Latn, Kenya
pub const fn get_mer() -> LangID {
  ID::new(7497069, None, None).get_id()
}

/// mfe: (mfe-Latn-MU) kreol morisien, Latn, Moris
pub const fn get_mfe() -> LangID {
  ID::new(6645357, None, None).get_id()
}

/// mg: (mg-Latn-MG) Malagasy, Latn, Madagasikara
pub const fn get_mg() -> LangID {
  ID::new(26477, None, None).get_id()
}

/// mgh: (mgh-Latn-MZ) Makua, Latn, Umozambiki
pub const fn get_mgh() -> LangID {
  ID::new(6842221, None, None).get_id()
}

/// mgo: (mgo-Latn-CM) metaʼ, ngam ŋwaʼri, Kamalun
pub const fn get_mgo() -> LangID {
  ID::new(7300973, None, None).get_id()
}

/// mi: (mi-Latn-NZ) Māori, Rātina, Aotearoa
pub const fn get_mi() -> LangID {
  ID::new(26989, None, None).get_id()
}

/// mk: (mk-Cyrl-MK) македонски, кирилско писмо, Северна Македонија
pub const fn get_mk() -> LangID {
  ID::new(27501, None, None).get_id()
}

/// ml: (ml-Mlym-IN) മലയാളം, മലയാളം, ഇന്ത്യ
pub const fn get_ml() -> LangID {
  ID::new(27757, None, None).get_id()
}

/// mn: (mn-Cyrl-MN) монгол, кирилл, Монгол
pub const fn get_mn() -> LangID {
  ID::new(28269, None, None).get_id()
}

/// mni: (mni-Beng-IN) মৈতৈলোন্, বাংলা, ইন্দিয়া
pub const fn get_mni() -> LangID {
  ID::new(6909549, None, None).get_id()
}

/// mni-Beng: (mni-Beng-IN) মৈতৈলোন্, বাংলা, ইন্দিয়া
pub const fn get_mni_beng() -> LangID {
  ID::new(6909549, Some(1735288130), None).get_id()
}

/// mr: (mr-Deva-IN) मराठी, देवनागरी, भारत
pub const fn get_mr() -> LangID {
  ID::new(29293, None, None).get_id()
}

/// ms: (ms-Latn-MY) Melayu, Latin, Malaysia
pub const fn get_ms() -> LangID {
  ID::new(29549, None, None).get_id()
}

/// ms-BN: (ms-Latn-BN) Melayu, Latin, Brunei
pub const fn get_ms_bn() -> LangID {
  ID::new(29549, None, Some(20034)).get_id()
}

/// ms-ID: (ms-Latn-ID) Melayu, Latin, Indonesia
pub const fn get_ms_id() -> LangID {
  ID::new(29549, None, Some(17481)).get_id()
}

/// ms-SG: (ms-Latn-SG) Melayu, Latin, Singapura
pub const fn get_ms_sg() -> LangID {
  ID::new(29549, None, Some(18259)).get_id()
}

/// mt: (mt-Latn-MT) Malti, Latin, Malta
pub const fn get_mt() -> LangID {
  ID::new(29805, None, None).get_id()
}

/// mua: (mua-Latn-CM) MUNDAŊ, Latn, kameruŋ
pub const fn get_mua() -> LangID {
  ID::new(6387053, None, None).get_id()
}

/// my: (my-Mymr-MM) မြန်မာ, မြန်မာ, မြန်မာ
pub const fn get_my() -> LangID {
  ID::new(31085, None, None).get_id()
}

/// mzn: (mzn-Arab-IR) مازرونی, عربی, ایران
pub const fn get_mzn() -> LangID {
  ID::new(7240301, None, None).get_id()
}

/// naq: (naq-Latn-NA) Khoekhoegowab, Latn, Namibiab
pub const fn get_naq() -> LangID {
  ID::new(7430510, None, None).get_id()
}

/// nb: (nb-Latn-NO) norsk bokmål, latinsk, Norge
pub const fn get_nb() -> LangID {
  ID::new(25198, None, None).get_id()
}

/// nb-SJ: (nb-Latn-SJ) norsk bokmål, latinsk, Svalbard og Jan Mayen
pub const fn get_nb_sj() -> LangID {
  ID::new(25198, None, Some(19027)).get_id()
}

/// nd: (nd-Latn-ZW) isiNdebele, Latn, Zimbabwe
pub const fn get_nd() -> LangID {
  ID::new(25710, None, None).get_id()
}

/// nds: (nds-Latn-DE) nds, Latn, DE
pub const fn get_nds() -> LangID {
  ID::new(7562350, None, None).get_id()
}

/// nds-NL: (nds-Latn-NL) nds, Latn, NL
pub const fn get_nds_nl() -> LangID {
  ID::new(7562350, None, Some(19534)).get_id()
}

/// ne: (ne-Deva-NP) नेपाली, देवानागरी, नेपाल
pub const fn get_ne() -> LangID {
  ID::new(25966, None, None).get_id()
}

/// ne-IN: (ne-Deva-IN) नेपाली, देवानागरी, भारत
pub const fn get_ne_in() -> LangID {
  ID::new(25966, None, Some(20041)).get_id()
}

/// nl: (nl-Latn-NL) Nederlands, Latijns, Nederland
pub const fn get_nl() -> LangID {
  ID::new(27758, None, None).get_id()
}

/// nl-AW: (nl-Latn-AW) Nederlands, Latijns, Aruba
pub const fn get_nl_aw() -> LangID {
  ID::new(27758, None, Some(22337)).get_id()
}

/// nl-BE: (nl-Latn-BE) Nederlands, Latijns, België
pub const fn get_nl_be() -> LangID {
  ID::new(27758, None, Some(17730)).get_id()
}

/// nl-BQ: (nl-Latn-BQ) Nederlands, Latijns, Caribisch Nederland
pub const fn get_nl_bq() -> LangID {
  ID::new(27758, None, Some(20802)).get_id()
}

/// nl-CW: (nl-Latn-CW) Nederlands, Latijns, Curaçao
pub const fn get_nl_cw() -> LangID {
  ID::new(27758, None, Some(22339)).get_id()
}

/// nl-SR: (nl-Latn-SR) Nederlands, Latijns, Suriname
pub const fn get_nl_sr() -> LangID {
  ID::new(27758, None, Some(21075)).get_id()
}

/// nl-SX: (nl-Latn-SX) Nederlands, Latijns, Sint-Maarten
pub const fn get_nl_sx() -> LangID {
  ID::new(27758, None, Some(22611)).get_id()
}

/// nmg: (nmg-Latn-CM) nmg, Latn, Kamerun
pub const fn get_nmg() -> LangID {
  ID::new(6778222, None, None).get_id()
}

/// nn: (nn-Latn-NO) norsk nynorsk, latinsk, Noreg
pub const fn get_nn() -> LangID {
  ID::new(28270, None, None).get_id()
}

/// nnh: (nnh-Latn-CM) Shwóŋò ngiembɔɔn, Latn, Kàmalûm
pub const fn get_nnh() -> LangID {
  ID::new(6844014, None, None).get_id()
}

/// no: (no-Latn-NO) norsk, latinsk, Norge
pub const fn get_no() -> LangID {
  ID::new(28526, None, None).get_id()
}

/// nus: (nus-Latn-SS) Thok Nath, Latn, SS
pub const fn get_nus() -> LangID {
  ID::new(7566702, None, None).get_id()
}

/// nyn: (nyn-Latn-UG) Runyankore, Latn, Uganda
pub const fn get_nyn() -> LangID {
  ID::new(7240046, None, None).get_id()
}

/// oc: (oc-Latn-FR) oc, Latn, FR
pub const fn get_oc() -> LangID {
  ID::new(25455, None, None).get_id()
}

/// oc-ES: (oc-Latn-ES) oc, Latn, ES
pub const fn get_oc_es() -> LangID {
  ID::new(25455, None, Some(21317)).get_id()
}

/// om: (om-Latn-ET) Oromoo, Latin, Itoophiyaa
pub const fn get_om() -> LangID {
  ID::new(28015, None, None).get_id()
}

/// om-KE: (om-Latn-KE) Oromoo, Latin, Keeniyaa
pub const fn get_om_ke() -> LangID {
  ID::new(28015, None, Some(17739)).get_id()
}

/// or: (or-Orya-IN) ଓଡ଼ିଆ, ଓଡ଼ିଆ, ଭାରତ
pub const fn get_or() -> LangID {
  ID::new(29295, None, None).get_id()
}

/// os: (os-Cyrl-GE) ирон, Киррилицӕ, Гуырдзыстон
pub const fn get_os() -> LangID {
  ID::new(29551, None, None).get_id()
}

/// os-RU: (os-Cyrl-RU) ирон, Киррилицӕ, Уӕрӕсе
pub const fn get_os_ru() -> LangID {
  ID::new(29551, None, Some(21842)).get_id()
}

/// pa: (pa-Guru-IN) ਪੰਜਾਬੀ, ਗੁਰਮੁਖੀ, ਭਾਰਤ
pub const fn get_pa() -> LangID {
  ID::new(24944, None, None).get_id()
}

/// pa-Arab: (pa-Arab-PK) پنجابی, عربی, پاکستان
pub const fn get_pa_arab() -> LangID {
  ID::new(24944, Some(1650553409), None).get_id()
}

/// pa-Guru: (pa-Guru-IN) ਪੰਜਾਬੀ, ਗੁਰਮੁਖੀ, ਭਾਰਤ
pub const fn get_pa_guru() -> LangID {
  ID::new(24944, Some(1970435399), None).get_id()
}

/// pcm: (pcm-Latn-NG) Naijíriá Píjin, Látin, Naijíria
pub const fn get_pcm() -> LangID {
  ID::new(7168880, None, None).get_id()
}

/// pis: (pis-Latn-SB) Pijin, Latin, Solomon Aelan
pub const fn get_pis() -> LangID {
  ID::new(7563632, None, None).get_id()
}

/// pl: (pl-Latn-PL) polski, łacińskie, Polska
pub const fn get_pl() -> LangID {
  ID::new(27760, None, None).get_id()
}

/// ps: (ps-Arab-AF) پښتو, عربي, افغانستان
pub const fn get_ps() -> LangID {
  ID::new(29552, None, None).get_id()
}

/// ps-PK: (ps-Arab-PK) پښتو, عربي, پاکستان
pub const fn get_ps_pk() -> LangID {
  ID::new(29552, None, Some(19280)).get_id()
}

/// pt: (pt-Latn-BR) português, latim, Brasil
pub const fn get_pt() -> LangID {
  ID::new(29808, None, None).get_id()
}

/// pt-AO: (pt-Latn-AO) português, latim, Angola
pub const fn get_pt_ao() -> LangID {
  ID::new(29808, None, Some(20289)).get_id()
}

/// pt-CH: (pt-Latn-CH) português, latim, Suíça
pub const fn get_pt_ch() -> LangID {
  ID::new(29808, None, Some(18499)).get_id()
}

/// pt-CV: (pt-Latn-CV) português, latim, Cabo Verde
pub const fn get_pt_cv() -> LangID {
  ID::new(29808, None, Some(22083)).get_id()
}

/// pt-GQ: (pt-Latn-GQ) português, latim, Guiné Equatorial
pub const fn get_pt_gq() -> LangID {
  ID::new(29808, None, Some(20807)).get_id()
}

/// pt-GW: (pt-Latn-GW) português, latim, Guiné-Bissau
pub const fn get_pt_gw() -> LangID {
  ID::new(29808, None, Some(22343)).get_id()
}

/// pt-LU: (pt-Latn-LU) português, latim, Luxemburgo
pub const fn get_pt_lu() -> LangID {
  ID::new(29808, None, Some(21836)).get_id()
}

/// pt-MO: (pt-Latn-MO) português, latim, Macau, RAE da China
pub const fn get_pt_mo() -> LangID {
  ID::new(29808, None, Some(20301)).get_id()
}

/// pt-MZ: (pt-Latn-MZ) português, latim, Moçambique
pub const fn get_pt_mz() -> LangID {
  ID::new(29808, None, Some(23117)).get_id()
}

/// pt-PT: (pt-Latn-PT) português, latim, Portugal
pub const fn get_pt_pt() -> LangID {
  ID::new(29808, None, Some(21584)).get_id()
}

/// pt-ST: (pt-Latn-ST) português, latim, São Tomé e Príncipe
pub const fn get_pt_st() -> LangID {
  ID::new(29808, None, Some(21587)).get_id()
}

/// pt-TL: (pt-Latn-TL) português, latim, Timor-Leste
pub const fn get_pt_tl() -> LangID {
  ID::new(29808, None, Some(19540)).get_id()
}

/// qu: (qu-Latn-PE) Runasimi, Latin Simi, Perú
pub const fn get_qu() -> LangID {
  ID::new(30065, None, None).get_id()
}

/// qu-BO: (qu-Latn-BO) Runasimi, Latin Simi, Bolivia
pub const fn get_qu_bo() -> LangID {
  ID::new(30065, None, Some(20290)).get_id()
}

/// qu-EC: (qu-Latn-EC) Runasimi, Latin Simi, Ecuador
pub const fn get_qu_ec() -> LangID {
  ID::new(30065, None, Some(17221)).get_id()
}

/// raj: (raj-Deva-IN) राजस्थानी, देवनागरी, भारत
pub const fn get_raj() -> LangID {
  ID::new(6971762, None, None).get_id()
}

/// rm: (rm-Latn-CH) rumantsch, latin, Svizra
pub const fn get_rm() -> LangID {
  ID::new(28018, None, None).get_id()
}

/// rn: (rn-Latn-BI) Ikirundi, Latn, Uburundi
pub const fn get_rn() -> LangID {
  ID::new(28274, None, None).get_id()
}

/// ro: (ro-Latn-RO) română, latină, România
pub const fn get_ro() -> LangID {
  ID::new(28530, None, None).get_id()
}

/// ro-MD: (ro-Latn-MD) română, latină, Republica Moldova
pub const fn get_ro_md() -> LangID {
  ID::new(28530, None, Some(17485)).get_id()
}

/// rof: (rof-Latn-TZ) Kihorombo, Latn, Tanzania
pub const fn get_rof() -> LangID {
  ID::new(6713202, None, None).get_id()
}

/// ru: (ru-Cyrl-RU) русский, кириллица, Россия
pub const fn get_ru() -> LangID {
  ID::new(30066, None, None).get_id()
}

/// ru-BY: (ru-Cyrl-BY) русский, кириллица, Беларусь
pub const fn get_ru_by() -> LangID {
  ID::new(30066, None, Some(22850)).get_id()
}

/// ru-KG: (ru-Cyrl-KG) русский, кириллица, Киргизия
pub const fn get_ru_kg() -> LangID {
  ID::new(30066, None, Some(18251)).get_id()
}

/// ru-KZ: (ru-Cyrl-KZ) русский, кириллица, Казахстан
pub const fn get_ru_kz() -> LangID {
  ID::new(30066, None, Some(23115)).get_id()
}

/// ru-MD: (ru-Cyrl-MD) русский, кириллица, Молдова
pub const fn get_ru_md() -> LangID {
  ID::new(30066, None, Some(17485)).get_id()
}

/// ru-UA: (ru-Cyrl-UA) русский, кириллица, Украина
pub const fn get_ru_ua() -> LangID {
  ID::new(30066, None, Some(16725)).get_id()
}

/// rw: (rw-Latn-RW) Kinyarwanda, Latn, U Rwanda
pub const fn get_rw() -> LangID {
  ID::new(30578, None, None).get_id()
}

/// rwk: (rwk-Latn-TZ) Kiruwa, Latn, Tanzania
pub const fn get_rwk() -> LangID {
  ID::new(7042930, None, None).get_id()
}

/// sa: (sa-Deva-IN) संस्कृत भाषा, Deva, भारतः
pub const fn get_sa() -> LangID {
  ID::new(24947, None, None).get_id()
}

/// sah: (sah-Cyrl-RU) саха тыла, Нууччалыы, Арассыыйа
pub const fn get_sah() -> LangID {
  ID::new(6840691, None, None).get_id()
}

/// saq: (saq-Latn-KE) Kisampur, Latn, Kenya
pub const fn get_saq() -> LangID {
  ID::new(7430515, None, None).get_id()
}

/// sat: (sat-Olck-IN) ᱥᱟᱱᱛᱟᱲᱤ, ᱚᱞ ᱪᱤᱠᱤ, ᱤᱱᱰᱤᱭᱟ
pub const fn get_sat() -> LangID {
  ID::new(7627123, None, None).get_id()
}

/// sat-Olck: (sat-Olck-IN) ᱥᱟᱱᱛᱟᱲᱤ, ᱚᱞ ᱪᱤᱠᱤ, ᱤᱱᱰᱤᱭᱟ
pub const fn get_sat_olck() -> LangID {
  ID::new(7627123, Some(1801677903), None).get_id()
}

/// sbp: (sbp-Latn-TZ) Ishisangu, Latn, Tansaniya
pub const fn get_sbp() -> LangID {
  ID::new(7365235, None, None).get_id()
}

/// sc: (sc-Latn-IT) sardu, latinu, Itàlia
pub const fn get_sc() -> LangID {
  ID::new(25459, None, None).get_id()
}

/// sd: (sd-Arab-PK) سنڌي, عربي, پاڪستان
pub const fn get_sd() -> LangID {
  ID::new(25715, None, None).get_id()
}

/// sd-Arab: (sd-Arab-PK) سنڌي, عربي, پاڪستان
pub const fn get_sd_arab() -> LangID {
  ID::new(25715, Some(1650553409), None).get_id()
}

/// sd-Deva: (sd-Deva-IN) सिन्धी, देवनागिरी, भारत
pub const fn get_sd_deva() -> LangID {
  ID::new(25715, Some(1635149124), None).get_id()
}

/// se: (se-Latn-NO) davvisámegiella, láhtenaš, Norga
pub const fn get_se() -> LangID {
  ID::new(25971, None, None).get_id()
}

/// se-FI: (se-Latn-FI) davvisámegiella, láhtenaš, Suopma
pub const fn get_se_fi() -> LangID {
  ID::new(25971, None, Some(18758)).get_id()
}

/// se-SE: (se-Latn-SE) davvisámegiella, láhtenaš, Ruoŧŧa
pub const fn get_se_se() -> LangID {
  ID::new(25971, None, Some(17747)).get_id()
}

/// seh: (seh-Latn-MZ) sena, Latn, Moçambique
pub const fn get_seh() -> LangID {
  ID::new(6841715, None, None).get_id()
}

/// ses: (ses-Latn-ML) Koyraboro senni, Latn, Maali
pub const fn get_ses() -> LangID {
  ID::new(7562611, None, None).get_id()
}

/// sg: (sg-Latn-CF) Sängö, Latn, Ködörösêse tî Bêafrîka
pub const fn get_sg() -> LangID {
  ID::new(26483, None, None).get_id()
}

/// shi: (shi-Tfng-MA) ⵜⴰⵛⵍⵃⵉⵜ, Tfng, ⵍⵎⵖⵔⵉⴱ
pub const fn get_shi() -> LangID {
  ID::new(6908019, None, None).get_id()
}

/// shi-Latn: (shi-Latn-MA) Tashelḥiyt, Latn, lmɣrib
pub const fn get_shi_latn() -> LangID {
  ID::new(6908019, Some(1853120844), None).get_id()
}

/// shi-Tfng: (shi-Tfng-MA) ⵜⴰⵛⵍⵃⵉⵜ, Tfng, ⵍⵎⵖⵔⵉⴱ
pub const fn get_shi_tfng() -> LangID {
  ID::new(6908019, Some(1735288404), None).get_id()
}

/// si: (si-Sinh-LK) සිංහල, සිංහල, ශ්‍රී ලංකාව
pub const fn get_si() -> LangID {
  ID::new(26995, None, None).get_id()
}

/// sk: (sk-Latn-SK) slovenčina, latinka, Slovensko
pub const fn get_sk() -> LangID {
  ID::new(27507, None, None).get_id()
}

/// sl: (sl-Latn-SI) slovenščina, latinica, Slovenija
pub const fn get_sl() -> LangID {
  ID::new(27763, None, None).get_id()
}

/// smn: (smn-Latn-FI) anarâškielâ, Latn, Suomâ
pub const fn get_smn() -> LangID {
  ID::new(7236979, None, None).get_id()
}

/// sms: (sms-Latn-FI) sms, Latn, FI
pub const fn get_sms() -> LangID {
  ID::new(7564659, None, None).get_id()
}

/// sn: (sn-Latn-ZW) chiShona, Latn, Zimbabwe
pub const fn get_sn() -> LangID {
  ID::new(28275, None, None).get_id()
}

/// so: (so-Latn-SO) Soomaali, Laatiin, Soomaaliya
pub const fn get_so() -> LangID {
  ID::new(28531, None, None).get_id()
}

/// so-DJ: (so-Latn-DJ) Soomaali, Laatiin, Jabuuti
pub const fn get_so_dj() -> LangID {
  ID::new(28531, None, Some(19012)).get_id()
}

/// so-ET: (so-Latn-ET) Soomaali, Laatiin, Itoobiya
pub const fn get_so_et() -> LangID {
  ID::new(28531, None, Some(21573)).get_id()
}

/// so-KE: (so-Latn-KE) Soomaali, Laatiin, Kenya
pub const fn get_so_ke() -> LangID {
  ID::new(28531, None, Some(17739)).get_id()
}

/// sq: (sq-Latn-AL) shqip, latin, Shqipëri
pub const fn get_sq() -> LangID {
  ID::new(29043, None, None).get_id()
}

/// sq-MK: (sq-Latn-MK) shqip, latin, Maqedonia e Veriut
pub const fn get_sq_mk() -> LangID {
  ID::new(29043, None, Some(19277)).get_id()
}

/// sq-XK: (sq-Latn-XK) shqip, latin, Kosovë
pub const fn get_sq_xk() -> LangID {
  ID::new(29043, None, Some(19288)).get_id()
}

/// sr: (sr-Cyrl-RS) српски, ћирилица, Србија
pub const fn get_sr() -> LangID {
  ID::new(29299, None, None).get_id()
}

/// sr-Cyrl: (sr-Cyrl-RS) српски, ћирилица, Србија
pub const fn get_sr_cyrl() -> LangID {
  ID::new(29299, Some(1819441475), None).get_id()
}

/// sr-Cyrl-BA: српски, ћирилица, Босна и Херцеговина
pub const fn get_sr_cyrl_ba() -> LangID {
  ID::new(29299, Some(1819441475), Some(16706)).get_id()
}

/// sr-Cyrl-ME: српски, ћирилица, Црна Гора
pub const fn get_sr_cyrl_me() -> LangID {
  ID::new(29299, Some(1819441475), Some(17741)).get_id()
}

/// sr-Cyrl-XK: српски, ћирилица, Косово
pub const fn get_sr_cyrl_xk() -> LangID {
  ID::new(29299, Some(1819441475), Some(19288)).get_id()
}

/// sr-Latn: (sr-Latn-RS) srpski, latinica, Srbija
pub const fn get_sr_latn() -> LangID {
  ID::new(29299, Some(1853120844), None).get_id()
}

/// sr-Latn-BA: srpski, latinica, Bosna i Hercegovina
pub const fn get_sr_latn_ba() -> LangID {
  ID::new(29299, Some(1853120844), Some(16706)).get_id()
}

/// sr-Latn-ME: srpski, latinica, Crna Gora
pub const fn get_sr_latn_me() -> LangID {
  ID::new(29299, Some(1853120844), Some(17741)).get_id()
}

/// sr-Latn-XK: srpski, latinica, Kosovo
pub const fn get_sr_latn_xk() -> LangID {
  ID::new(29299, Some(1853120844), Some(19288)).get_id()
}

/// su: (su-Latn-ID) Basa Sunda, Latin, Indonesia
pub const fn get_su() -> LangID {
  ID::new(30067, None, None).get_id()
}

/// su-Latn: (su-Latn-ID) Basa Sunda, Latin, Indonesia
pub const fn get_su_latn() -> LangID {
  ID::new(30067, Some(1853120844), None).get_id()
}

/// sv: (sv-Latn-SE) svenska, latinska, Sverige
pub const fn get_sv() -> LangID {
  ID::new(30323, None, None).get_id()
}

/// sv-AX: (sv-Latn-AX) svenska, latinska, Åland
pub const fn get_sv_ax() -> LangID {
  ID::new(30323, None, Some(22593)).get_id()
}

/// sv-FI: (sv-Latn-FI) svenska, latinska, Finland
pub const fn get_sv_fi() -> LangID {
  ID::new(30323, None, Some(18758)).get_id()
}

/// sw: (sw-Latn-TZ) Kiswahili, Kilatini, Tanzania
pub const fn get_sw() -> LangID {
  ID::new(30579, None, None).get_id()
}

/// sw-CD: (sw-Latn-CD) Kiswahili, Kilatini, Jamhuri ya Kidemokrasia ya Kongo
pub const fn get_sw_cd() -> LangID {
  ID::new(30579, None, Some(17475)).get_id()
}

/// sw-KE: (sw-Latn-KE) Kiswahili, Kilatini, Kenya
pub const fn get_sw_ke() -> LangID {
  ID::new(30579, None, Some(17739)).get_id()
}

/// sw-UG: (sw-Latn-UG) Kiswahili, Kilatini, Uganda
pub const fn get_sw_ug() -> LangID {
  ID::new(30579, None, Some(18261)).get_id()
}

/// ta: (ta-Taml-IN) தமிழ், தமிழ், இந்தியா
pub const fn get_ta() -> LangID {
  ID::new(24948, None, None).get_id()
}

/// ta-LK: (ta-Taml-LK) தமிழ், தமிழ், இலங்கை
pub const fn get_ta_lk() -> LangID {
  ID::new(24948, None, Some(19276)).get_id()
}

/// ta-MY: (ta-Taml-MY) தமிழ், தமிழ், மலேசியா
pub const fn get_ta_my() -> LangID {
  ID::new(24948, None, Some(22861)).get_id()
}

/// ta-SG: (ta-Taml-SG) தமிழ், தமிழ், சிங்கப்பூர்
pub const fn get_ta_sg() -> LangID {
  ID::new(24948, None, Some(18259)).get_id()
}

/// te: (te-Telu-IN) తెలుగు, తెలుగు, భారతదేశం
pub const fn get_te() -> LangID {
  ID::new(25972, None, None).get_id()
}

/// teo: (teo-Latn-UG) Kiteso, Latn, Uganda
pub const fn get_teo() -> LangID {
  ID::new(7300468, None, None).get_id()
}

/// teo-KE: (teo-Latn-KE) Kiteso, Latn, Kenia
pub const fn get_teo_ke() -> LangID {
  ID::new(7300468, None, Some(17739)).get_id()
}

/// tg: (tg-Cyrl-TJ) тоҷикӣ, Кириллӣ, Тоҷикистон
pub const fn get_tg() -> LangID {
  ID::new(26484, None, None).get_id()
}

/// th: (th-Thai-TH) ไทย, ไทย, ไทย
pub const fn get_th() -> LangID {
  ID::new(26740, None, None).get_id()
}

/// ti: (ti-Ethi-ET) ትግርኛ, ፊደል, ኢትዮጵያ
pub const fn get_ti() -> LangID {
  ID::new(26996, None, None).get_id()
}

/// ti-ER: (ti-Ethi-ER) ትግርኛ, ፊደል, ኤርትራ
pub const fn get_ti_er() -> LangID {
  ID::new(26996, None, Some(21061)).get_id()
}

/// tk: (tk-Latn-TM) türkmen dili, Latyn elipbiýi, Türkmenistan
pub const fn get_tk() -> LangID {
  ID::new(27508, None, None).get_id()
}

/// to: (to-Latn-TO) lea fakatonga, tohinima fakalatina, Tonga
pub const fn get_to() -> LangID {
  ID::new(28532, None, None).get_id()
}

/// tok: (tok-Latn-001) Toki Pona, Latn, 001
pub const fn get_tok() -> LangID {
  ID::new(7040884, None, None).get_id()
}

/// tr: (tr-Latn-TR) Türkçe, Latin, Türkiye
pub const fn get_tr() -> LangID {
  ID::new(29300, None, None).get_id()
}

/// tr-CY: (tr-Latn-CY) Türkçe, Latin, Kıbrıs
pub const fn get_tr_cy() -> LangID {
  ID::new(29300, None, Some(22851)).get_id()
}

/// tt: (tt-Cyrl-RU) татар, кирилл, Россия
pub const fn get_tt() -> LangID {
  ID::new(29812, None, None).get_id()
}

/// twq: (twq-Latn-NE) Tasawaq senni, Latn, Nižer
pub const fn get_twq() -> LangID {
  ID::new(7436148, None, None).get_id()
}

/// tzm: (tzm-Latn-MA) Tamaziɣt n laṭlaṣ, Latn, Meṛṛuk
pub const fn get_tzm() -> LangID {
  ID::new(7174772, None, None).get_id()
}

/// ug: (ug-Arab-CN) ئۇيغۇرچە, ئەرەب, جۇڭگو
pub const fn get_ug() -> LangID {
  ID::new(26485, None, None).get_id()
}

/// uk: (uk-Cyrl-UA) українська, кирилиця, Україна
pub const fn get_uk() -> LangID {
  ID::new(27509, None, None).get_id()
}

/// Undefined.
pub const fn get_und() -> LangID {
  ID::new(6581877, None, None).get_id()
}

/// ur: (ur-Arab-PK) اردو, عربی, پاکستان
pub const fn get_ur() -> LangID {
  ID::new(29301, None, None).get_id()
}

/// ur-IN: (ur-Arab-IN) اردو, عربی, بھارت
pub const fn get_ur_in() -> LangID {
  ID::new(29301, None, Some(20041)).get_id()
}

/// uz: (uz-Latn-UZ) o‘zbek, lotin, Oʻzbekiston
pub const fn get_uz() -> LangID {
  ID::new(31349, None, None).get_id()
}

/// uz-Arab: (uz-Arab-AF) اوزبیک, عربی, افغانستان
pub const fn get_uz_arab() -> LangID {
  ID::new(31349, Some(1650553409), None).get_id()
}

/// uz-Cyrl: (uz-Cyrl-UZ) ўзбекча, Кирил, Ўзбекистон
pub const fn get_uz_cyrl() -> LangID {
  ID::new(31349, Some(1819441475), None).get_id()
}

/// uz-Latn: (uz-Latn-UZ) o‘zbek, lotin, Oʻzbekiston
pub const fn get_uz_latn() -> LangID {
  ID::new(31349, Some(1853120844), None).get_id()
}

/// vai: (vai-Vaii-LR) ꕙꔤ, Vaii, ꕞꔤꔫꕩ
pub const fn get_vai() -> LangID {
  ID::new(6906230, None, None).get_id()
}

/// vai-Latn: (vai-Latn-LR) Vai, Latn, Laibhiya
pub const fn get_vai_latn() -> LangID {
  ID::new(6906230, Some(1853120844), None).get_id()
}

/// vai-Vaii: (vai-Vaii-LR) ꕙꔤ, Vaii, ꕞꔤꔫꕩ
pub const fn get_vai_vaii() -> LangID {
  ID::new(6906230, Some(1768513878), None).get_id()
}

/// vi: (vi-Latn-VN) Tiếng Việt, Chữ La tinh, Việt Nam
pub const fn get_vi() -> LangID {
  ID::new(26998, None, None).get_id()
}

/// vun: (vun-Latn-TZ) Kyivunjo, Latn, Tanzania
pub const fn get_vun() -> LangID {
  ID::new(7239030, None, None).get_id()
}

/// wae: (wae-Latn-CH) Walser, Latiniš, Schwiz
pub const fn get_wae() -> LangID {
  ID::new(6644087, None, None).get_id()
}

/// wo: (wo-Latn-SN) Wolof, Latin, Senegaal
pub const fn get_wo() -> LangID {
  ID::new(28535, None, None).get_id()
}

/// xh: (xh-Latn-ZA) IsiXhosa, IsiLatin, EMzantsi Afrika
pub const fn get_xh() -> LangID {
  ID::new(26744, None, None).get_id()
}

/// xog: (xog-Latn-UG) Olusoga, Latn, Yuganda
pub const fn get_xog() -> LangID {
  ID::new(6778744, None, None).get_id()
}

/// yav: (yav-Latn-CM) nuasue, Latn, Kemelún
pub const fn get_yav() -> LangID {
  ID::new(7758201, None, None).get_id()
}

/// yi: (yi-Hebr-001) ייִדיש, העברעיש, וועלט
pub const fn get_yi() -> LangID {
  ID::new(27001, None, None).get_id()
}

/// yo: (yo-Latn-NG) Èdè Yorùbá, Èdè Látìn, Nàìjíríà
pub const fn get_yo() -> LangID {
  ID::new(28537, None, None).get_id()
}

/// yo-BJ: (yo-Latn-BJ) Èdè Yorùbá, Èdè Látìn, Bɛ̀nɛ̀
pub const fn get_yo_bj() -> LangID {
  ID::new(28537, None, Some(19010)).get_id()
}

/// yrl: (yrl-Latn-BR) nheẽgatu, ratĩ, Brasiu
pub const fn get_yrl() -> LangID {
  ID::new(7107193, None, None).get_id()
}

/// yrl-CO: (yrl-Latn-CO) ñengatú, ratĩ, Kurũbiya
pub const fn get_yrl_co() -> LangID {
  ID::new(7107193, None, Some(20291)).get_id()
}

/// yrl-VE: (yrl-Latn-VE) ñengatú, ratĩ, Wenesuera
pub const fn get_yrl_ve() -> LangID {
  ID::new(7107193, None, Some(17750)).get_id()
}

/// yue: (yue-Hant-HK) 粵語, 繁體, 中華人民共和國香港特別行政區
pub const fn get_yue() -> LangID {
  ID::new(6649209, None, None).get_id()
}

/// yue-Hans: (yue-Hans-CN) 粤语, 简体, 中华人民共和国
pub const fn get_yue_hans() -> LangID {
  ID::new(6649209, Some(1936613704), None).get_id()
}

/// yue-Hant: (yue-Hant-HK) 粵語, 繁體, 中華人民共和國香港特別行政區
pub const fn get_yue_hant() -> LangID {
  ID::new(6649209, Some(1953390920), None).get_id()
}

/// zgh: (zgh-Tfng-MA) ⵜⴰⵎⴰⵣⵉⵖⵜ, Tfng, ⵍⵎⵖⵔⵉⴱ
pub const fn get_zgh() -> LangID {
  ID::new(6842234, None, None).get_id()
}

/// zh: (zh-Hans-CN) 简体中文, 中国
pub const fn get_zh() -> LangID {
  ID::new(26746, None, None).get_id()
}

/// zh-Hans: (zh-Hans-CN) 简体中文, 中国
pub const fn get_zh_hans() -> LangID {
  ID::new(26746, Some(1936613704), None).get_id()
}

/// zh-Hans-HK: 简体中文, 中国香港特别行政区
pub const fn get_zh_hans_hk() -> LangID {
  ID::new(26746, Some(1936613704), Some(19272)).get_id()
}

/// zh-Hans-MO: 简体中文, 中国澳门特别行政区
pub const fn get_zh_hans_mo() -> LangID {
  ID::new(26746, Some(1936613704), Some(20301)).get_id()
}

/// zh-Hans-SG: 华文, 新加坡
pub const fn get_zh_hans_sg() -> LangID {
  ID::new(26746, Some(1936613704), Some(18259)).get_id()
}

/// zh-Hant: (zh-Hant-TW) 正體中文, 中國台灣
pub const fn get_zh_hant() -> LangID {
  ID::new(26746, Some(1953390920), None).get_id()
}

/// zh-Hant-HK: 繁體中文, 中國香港
pub const fn get_zh_hant_hk() -> LangID {
  ID::new(26746, Some(1953390920), Some(19272)).get_id()
}

/// zh-Hant-MO: 繁體中文, 中國澳門
pub const fn get_zh_hant_mo() -> LangID {
  ID::new(26746, Some(1953390920), Some(20301)).get_id()
}

/// zu: (zu-Latn-ZA) isiZulu, isi-Latin, iNingizimu Afrika
pub const fn get_zu() -> LangID {
  ID::new(30074, None, None).get_id()
}

/// st: (st-Latn-ZA)
pub const fn get_st() -> LangID {
  ID::new(29811, None, None).get_id()
}

/// la: (la-Latn-VA)
pub const fn get_la() -> LangID {
  ID::new(24940, None, None).get_id()
}

/// ny: (ny-Latn-MW)
pub const fn get_ny() -> LangID {
  ID::new(31086, None, None).get_id()
}

/// sm: (sm-Latn-WS)
pub const fn get_sm() -> LangID {
  ID::new(28019, None, None).get_id()
}

/// jw: (jw-Latn-ID)
pub const fn get_jw() -> LangID {
  ID::new(30570, None, None).get_id()
}

/// ht: (ht-Latn-HT)
pub const fn get_ht() -> LangID {
  ID::new(29800, None, None).get_id()
}

/// co: (co-Latn-FR)
pub const fn get_co() -> LangID {
  ID::new(28515, None, None).get_id()
}

/// tl: (tl-Latn-PH)
pub const fn get_tl() -> LangID {
  ID::new(27764, None, None).get_id()
}

/// iw: (iw-Hebr-IL)
pub const fn get_iw() -> LangID {
  ID::new(30569, None, None).get_id()
}
