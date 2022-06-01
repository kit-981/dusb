#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Arabic {
    SaudiArabia,
    Iraq,
    Egypt,
    Libya,
    Algeria,
    Morocco,
    Tunisia,
    Oman,
    Yemen,
    Syria,
    Jordan,
    Lebanon,
    Kuwait,
    Uae,
    Bahrain,
    Qatar,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Azeri {
    Latin,
    Cyrillic,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Chinese {
    Taiwan,
    Prc,
    HongKong,
    Singapore,
    Macau,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Dutch {
    Netherlands,
    Belgium,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum English {
    UnitedStates,
    UnitedKingdom,
    Australian,
    Canadian,
    NewZealand,
    Ireland,
    SouthAfrica,
    Jamaica,
    Caribbean,
    Belize,
    Trinidad,
    Zimbabwe,
    Philippines,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum French {
    Standard,
    Belgian,
    Canadian,
    Switzerland,
    Luxembourg,
    Monaco,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum German {
    Standard,
    Switzerland,
    Austria,
    Luxembourg,
    Liechtenstein,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Italian {
    Standard,
    Switzerland,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Kashmiri {
    India,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Korean {
    Johab,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Lithuanian {
    Classic,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Malay {
    Malaysian,
    BruneiDarussalam,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Nepali {
    India,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Norwegian {
    Bokmal,
    Nynorsk,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Portuguese {
    Brazil,
    Standard,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Serbian {
    Cyrillic,
    Latin,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Spanish {
    TraditionalSort,
    Mexican,
    ModernSort,
    Guatemala,
    CostaRica,
    Panama,
    DominicanRepublic,
    Venezuela,
    Colombia,
    Peru,
    Argentina,
    Ecuador,
    Chile,
    Uruguay,
    Paraguay,
    Bolivia,
    ElSalvador,
    Honduras,
    Nicaragua,
    PuertoRico,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Swahili {
    Kenya,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Swedish {
    Finland,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Tatar {
    Tatarstan,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Urdu {
    Pakistan,
    India,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Uzbek {
    Latin,
    Cyrillic,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Hid {
    UsageDataDescriptor,
    VendorDefined1,
    VendorDefined2,
    VendorDefined3,
    VendorDefined4,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Language {
    Afrikaans,
    Albanian,
    Arabic(Arabic),
    Armenian,
    Assamese,
    Azeri(Azeri),
    Basque,
    Belarussian,
    Bengali,
    Bulgarian,
    Burmese,
    Catalan,
    Chinese(Chinese),
    Croatian,
    Czech,
    Danish,
    Dutch(Dutch),
    English(English),
    Estonian,
    Faeroese,
    Farsi,
    Finnish,
    French(French),
    Georgian,
    German(German),
    Greek,
    Gujarati,
    Hebrew,
    Hindi,
    Hungarian,
    Icelandic,
    Indonesian,
    Italian(Italian),
    Japanese,
    Kannada,
    Kashmiri(Kashmiri),
    Kazakh,
    Konkani,
    Korean(Option<Korean>),
    Latvian,
    Lithuanian(Option<Lithuanian>),
    Macedonian,
    Malay(Malay),
    Malayalam,
    Manipuri,
    Marathi,
    Nepali(Nepali),
    Norwegian(Norwegian),
    Oriya,
    Polish,
    Portuguese(Portuguese),
    Punjabi,
    Romanian,
    Russian,
    Sanskrit,
    Serbian(Serbian),
    Sindhi,
    Slovak,
    Slovenian,
    Spanish(Spanish),
    Sutu,
    Swahili(Swahili),
    Swedish(Option<Swedish>),
    Tamil,
    Tatar(Tatar),
    Telugu,
    Thai,
    Turkish,
    Ukrainian,
    Urdu(Urdu),
    Uzbek(Uzbek),
    Vietnamese,
    Hid(Hid),
}

pub const LANGUAGE_IDENTIFIER_AFRIKAANS: u16 = 0x436;
pub const LANGUAGE_IDENTIFIER_ALBANIAN: u16 = 0x41c;
pub const LANGUAGE_IDENTIFIER_ARABIC_SAUDI_ARABIA: u16 = 0x401;
pub const LANGUAGE_IDENTIFIER_ARABIC_IRAQ: u16 = 0x801;
pub const LANGUAGE_IDENTIFIER_ARABIC_EGYPT: u16 = 0xc01;
pub const LANGUAGE_IDENTIFIER_ARABIC_LIBYA: u16 = 0x1001;
pub const LANGUAGE_IDENTIFIER_ARABIC_ALGERIA: u16 = 0x1401;
pub const LANGUAGE_IDENTIFIER_ARABIC_MOROCCO: u16 = 0x1801;
pub const LANGUAGE_IDENTIFIER_ARABIC_TUNISIA: u16 = 0x1c01;
pub const LANGUAGE_IDENTIFIER_ARABIC_OMAN: u16 = 0x2001;
pub const LANGUAGE_IDENTIFIER_ARABIC_YEMEN: u16 = 0x2401;
pub const LANGUAGE_IDENTIFIER_ARABIC_SYRIA: u16 = 0x2801;
pub const LANGUAGE_IDENTIFIER_ARABIC_JORDAN: u16 = 0x2c01;
pub const LANGUAGE_IDENTIFIER_ARABIC_LEBANON: u16 = 0x3001;
pub const LANGUAGE_IDENTIFIER_ARABIC_KUWAIT: u16 = 0x3401;
pub const LANGUAGE_IDENTIFIER_ARABIC_UAE: u16 = 0x3801;
pub const LANGUAGE_IDENTIFIER_ARABIC_BAHRAIN: u16 = 0x3c01;
pub const LANGUAGE_IDENTIFIER_ARABIC_QATAR: u16 = 0x4001;
pub const LANGUAGE_IDENTIFIER_ARMENIAN: u16 = 0x42b;
pub const LANGUAGE_IDENTIFIER_ASSAMESE: u16 = 0x44d;
pub const LANGUAGE_IDENTIFIER_AZERI_LATIN: u16 = 0x42c;
pub const LANGUAGE_IDENTIFIER_AZERI_CYRILLIC: u16 = 0x82c;
pub const LANGUAGE_IDENTIFIER_BASQUE: u16 = 0x42d;
pub const LANGUAGE_IDENTIFIER_BELARUSSIAN: u16 = 0x423;
pub const LANGUAGE_IDENTIFIER_BENGALI: u16 = 0x445;
pub const LANGUAGE_IDENTIFIER_BULGARIAN: u16 = 0x402;
pub const LANGUAGE_IDENTIFIER_BURMESE: u16 = 0x455;
pub const LANGUAGE_IDENTIFIER_CATALAN: u16 = 0x403;
pub const LANGUAGE_IDENTIFIER_CHINESE_TAIWAN: u16 = 0x404;
pub const LANGUAGE_IDENTIFIER_CHINESE_PRC: u16 = 0x804;
pub const LANGUAGE_IDENTIFIER_CHINESE_HONG_KONG_SAR_PRC: u16 = 0xc04;
pub const LANGUAGE_IDENTIFIER_CHINESE_SINGAPORE: u16 = 0x1004;
pub const LANGUAGE_IDENTIFIER_CHINESE_MACAU_SAR: u16 = 0x1404;
pub const LANGUAGE_IDENTIFIER_CROATIAN: u16 = 0x41a;
pub const LANGUAGE_IDENTIFIER_CZECH: u16 = 0x405;
pub const LANGUAGE_IDENTIFIER_DANISH: u16 = 0x406;
pub const LANGUAGE_IDENTIFIER_DUTCH_NETHERLANDS: u16 = 0x413;
pub const LANGUAGE_IDENTIFIER_DUTCH_BELGIAN: u16 = 0x813;
pub const LANGUAGE_IDENTIFIER_ENGLISH_UNITED_STATES: u16 = 0x409;
pub const LANGUAGE_IDENTIFIER_ENGLISH_UNITED_KINGDOM: u16 = 0x809;
pub const LANGUAGE_IDENTIFIER_ENGLISH_AUSTRALIAN: u16 = 0xc09;
pub const LANGUAGE_IDENTIFIER_ENGLISH_CANADIAN: u16 = 0x1009;
pub const LANGUAGE_IDENTIFIER_ENGLISH_NEW_ZEALAND: u16 = 0x1409;
pub const LANGUAGE_IDENTIFIER_ENGLISH_IRELAND: u16 = 0x1809;
pub const LANGUAGE_IDENTIFIER_ENGLISH_SOUTH_AFRICA: u16 = 0x1c09;
pub const LANGUAGE_IDENTIFIER_ENGLISH_JAMAICA: u16 = 0x2009;
pub const LANGUAGE_IDENTIFIER_ENGLISH_CARIBBEAN: u16 = 0x2409;
pub const LANGUAGE_IDENTIFIER_ENGLISH_BELIZE: u16 = 0x2809;
pub const LANGUAGE_IDENTIFIER_ENGLISH_TRINIDAD: u16 = 0x2c09;
pub const LANGUAGE_IDENTIFIER_ENGLISH_ZIMBABWE: u16 = 0x3009;
pub const LANGUAGE_IDENTIFIER_ENGLISH_PHILIPPINES: u16 = 0x3409;
pub const LANGUAGE_IDENTIFIER_ESTONIAN: u16 = 0x425;
pub const LANGUAGE_IDENTIFIER_FAEROESE: u16 = 0x438;
pub const LANGUAGE_IDENTIFIER_FARSI: u16 = 0x429;
pub const LANGUAGE_IDENTIFIER_FINNISH: u16 = 0x40b;
pub const LANGUAGE_IDENTIFIER_FRENCH_STANDARD: u16 = 0x40c;
pub const LANGUAGE_IDENTIFIER_FRENCH_BELGIAN: u16 = 0x80c;
pub const LANGUAGE_IDENTIFIER_FRENCH_CANADIAN: u16 = 0xc0c;
pub const LANGUAGE_IDENTIFIER_FRENCH_SWITZERLAND: u16 = 0x100c;
pub const LANGUAGE_IDENTIFIER_FRENCH_LUXEMBOURG: u16 = 0x140c;
pub const LANGUAGE_IDENTIFIER_FRENCH_MONACO: u16 = 0x180c;
pub const LANGUAGE_IDENTIFIER_GEORGIAN: u16 = 0x437;
pub const LANGUAGE_IDENTIFIER_GERMAN_STANDARD: u16 = 0x407;
pub const LANGUAGE_IDENTIFIER_GERMAN_SWITZERLAND: u16 = 0x807;
pub const LANGUAGE_IDENTIFIER_GERMAN_AUSTRIA: u16 = 0xc07;
pub const LANGUAGE_IDENTIFIER_GERMAN_LUXEMBOURG: u16 = 0x1007;
pub const LANGUAGE_IDENTIFIER_GERMAN_LIECHTENSTEIN: u16 = 0x1407;
pub const LANGUAGE_IDENTIFIER_GREEK: u16 = 0x408;
pub const LANGUAGE_IDENTIFIER_GUJARATI: u16 = 0x447;
pub const LANGUAGE_IDENTIFIER_HEBREW: u16 = 0x40d;
pub const LANGUAGE_IDENTIFIER_HINDI: u16 = 0x439;
pub const LANGUAGE_IDENTIFIER_HUNGARIAN: u16 = 0x40e;
pub const LANGUAGE_IDENTIFIER_ICELANDIC: u16 = 0x40f;
pub const LANGUAGE_IDENTIFIER_INDONESIAN: u16 = 0x421;
pub const LANGUAGE_IDENTIFIER_ITALIAN_STANDARD: u16 = 0x410;
pub const LANGUAGE_IDENTIFIER_ITALIAN_SWITZERLAND: u16 = 0x810;
pub const LANGUAGE_IDENTIFIER_JAPANESE: u16 = 0x411;
pub const LANGUAGE_IDENTIFIER_KANNADA: u16 = 0x44b;
pub const LANGUAGE_IDENTIFIER_KASHMIRI_INDIA: u16 = 0x860;
pub const LANGUAGE_IDENTIFIER_KAZAKH: u16 = 0x43f;
pub const LANGUAGE_IDENTIFIER_KONKANI: u16 = 0x457;
pub const LANGUAGE_IDENTIFIER_KOREAN: u16 = 0x412;
pub const LANGUAGE_IDENTIFIER_KOREAN_JOHAB: u16 = 0x812;
pub const LANGUAGE_IDENTIFIER_LATVIAN: u16 = 0x426;
pub const LANGUAGE_IDENTIFIER_LITHUANIAN: u16 = 0x427;
pub const LANGUAGE_IDENTIFIER_LITHUANIAN_CLASSIC: u16 = 0x827;
pub const LANGUAGE_IDENTIFIER_MACEDONIAN: u16 = 0x42f;
pub const LANGUAGE_IDENTIFIER_MALAY_MALAYSIAN: u16 = 0x43e;
pub const LANGUAGE_IDENTIFIER_MALAY_BRUNEI_DARUSSALAM: u16 = 0x83e;
pub const LANGUAGE_IDENTIFIER_MALAYALAM: u16 = 0x44c;
pub const LANGUAGE_IDENTIFIER_MANIPURI: u16 = 0x458;
pub const LANGUAGE_IDENTIFIER_MARATHI: u16 = 0x44e;
pub const LANGUAGE_IDENTIFIER_NEPALI_INDIA: u16 = 0x861;
pub const LANGUAGE_IDENTIFIER_NORWEGIAN_BOKMAL: u16 = 0x414;
pub const LANGUAGE_IDENTIFIER_NORWEGIAN_NYNORSK: u16 = 0x814;
pub const LANGUAGE_IDENTIFIER_ORIYA: u16 = 0x448;
pub const LANGUAGE_IDENTIFIER_POLISH: u16 = 0x415;
pub const LANGUAGE_IDENTIFIER_PORTUGUESE_BRAZIL: u16 = 0x416;
pub const LANGUAGE_IDENTIFIER_PORTUGUESE_STANDARD: u16 = 0x816;
pub const LANGUAGE_IDENTIFIER_PUNJABI: u16 = 0x446;
pub const LANGUAGE_IDENTIFIER_ROMANIAN: u16 = 0x418;
pub const LANGUAGE_IDENTIFIER_RUSSIAN: u16 = 0x419;
pub const LANGUAGE_IDENTIFIER_SANSKRIT: u16 = 0x44f;
pub const LANGUAGE_IDENTIFIER_SERBIAN_CYRILLIC: u16 = 0xc1a;
pub const LANGUAGE_IDENTIFIER_SERBIAN_LATIN: u16 = 0x81a;
pub const LANGUAGE_IDENTIFIER_SINDHI: u16 = 0x459;
pub const LANGUAGE_IDENTIFIER_SLOVAK: u16 = 0x41b;
pub const LANGUAGE_IDENTIFIER_SLOVENIAN: u16 = 0x424;
pub const LANGUAGE_IDENTIFIER_SPANISH_TRADITIONAL_SORT: u16 = 0x40a;
pub const LANGUAGE_IDENTIFIER_SPANISH_MEXICAN: u16 = 0x80a;
pub const LANGUAGE_IDENTIFIER_SPANISH_MODERN_SORT: u16 = 0xc0a;
pub const LANGUAGE_IDENTIFIER_SPANISH_GUATEMALA: u16 = 0x100a;
pub const LANGUAGE_IDENTIFIER_SPANISH_COSTA_RICA: u16 = 0x140a;
pub const LANGUAGE_IDENTIFIER_SPANISH_PANAMA: u16 = 0x180a;
pub const LANGUAGE_IDENTIFIER_SPANISH_DOMINICAN_REPUBLIC: u16 = 0x1c0a;
pub const LANGUAGE_IDENTIFIER_SPANISH_VENEZUELA: u16 = 0x200a;
pub const LANGUAGE_IDENTIFIER_SPANISH_COLUMBIA: u16 = 0x240a;
pub const LANGUAGE_IDENTIFIER_SPANISH_PERU: u16 = 0x280a;
pub const LANGUAGE_IDENTIFIER_SPANISH_ARGENTINA: u16 = 0x2c0a;
pub const LANGUAGE_IDENTIFIER_SPANISH_ECUADOR: u16 = 0x300a;
pub const LANGUAGE_IDENTIFIER_SPANISH_CHILE: u16 = 0x340a;
pub const LANGUAGE_IDENTIFIER_SPANISH_URUGUAY: u16 = 0x380a;
pub const LANGUAGE_IDENTIFIER_SPANISH_PARAGUAY: u16 = 0x3c0a;
pub const LANGUAGE_IDENTIFIER_SPANISH_BOLIVIA: u16 = 0x400a;
pub const LANGUAGE_IDENTIFIER_SPANISH_EL_SALVADOR: u16 = 0x440a;
pub const LANGUAGE_IDENTIFIER_SPANISH_HONDURAS: u16 = 0x480a;
pub const LANGUAGE_IDENTIFIER_SPANISH_NICARAGUA: u16 = 0x4c0a;
pub const LANGUAGE_IDENTIFIER_SPANISH_PUERTO_RICO: u16 = 0x500a;
pub const LANGUAGE_IDENTIFIER_SUTU: u16 = 0x430;
pub const LANGUAGE_IDENTIFIER_SWAHILI_KENYA: u16 = 0x441;
pub const LANGUAGE_IDENTIFIER_SWEDISH: u16 = 0x41d;
pub const LANGUAGE_IDENTIFIER_SWEDISH_FINLAND: u16 = 0x81d;
pub const LANGUAGE_IDENTIFIER_TAMIL: u16 = 0x449;
pub const LANGUAGE_IDENTIFIER_TATAR_TATARSTAN: u16 = 0x444;
pub const LANGUAGE_IDENTIFIER_TELUGU: u16 = 0x44a;
pub const LANGUAGE_IDENTIFIER_THAI: u16 = 0x41e;
pub const LANGUAGE_IDENTIFIER_TURKISH: u16 = 0x41f;
pub const LANGUAGE_IDENTIFIER_UKRAINIAN: u16 = 0x422;
pub const LANGUAGE_IDENTIFIER_URDU_PAKISTAN: u16 = 0x420;
pub const LANGUAGE_IDENTIFIER_URDU_INDIA: u16 = 0x820;
pub const LANGUAGE_IDENTIFIER_UZBEK_LATIN: u16 = 0x443;
pub const LANGUAGE_IDENTIFIER_UZBEK_CYRILLIC: u16 = 0x843;
pub const LANGUAGE_IDENTIFIER_VIETNAMESE: u16 = 0x42a;
pub const LANGUAGE_IDENTIFIER_HID_USAGE_DATA_DESCRIPTOR: u16 = 0x4ff;
pub const LANGUAGE_IDENTIFIER_HID_VENDOR_DEFINED_1: u16 = 0xf0ff;
pub const LANGUAGE_IDENTIFIER_HID_VENDOR_DEFINED_2: u16 = 0xf4ff;
pub const LANGUAGE_IDENTIFIER_HID_VENDOR_DEFINED_3: u16 = 0xf8ff;
pub const LANGUAGE_IDENTIFIER_HID_VENDOR_DEFINED_4: u16 = 0xfcff;

impl TryFrom<u16> for Language {
    type Error = ();

    #[allow(clippy::too_many_lines)]
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            LANGUAGE_IDENTIFIER_AFRIKAANS => Ok(Language::Afrikaans),
            LANGUAGE_IDENTIFIER_ALBANIAN => Ok(Language::Albanian),
            LANGUAGE_IDENTIFIER_ARABIC_SAUDI_ARABIA => Ok(Language::Arabic(Arabic::SaudiArabia)),
            LANGUAGE_IDENTIFIER_ARABIC_IRAQ => Ok(Language::Arabic(Arabic::Iraq)),
            LANGUAGE_IDENTIFIER_ARABIC_EGYPT => Ok(Language::Arabic(Arabic::Egypt)),
            LANGUAGE_IDENTIFIER_ARABIC_LIBYA => Ok(Language::Arabic(Arabic::Libya)),
            LANGUAGE_IDENTIFIER_ARABIC_ALGERIA => Ok(Language::Arabic(Arabic::Algeria)),
            LANGUAGE_IDENTIFIER_ARABIC_MOROCCO => Ok(Language::Arabic(Arabic::Morocco)),
            LANGUAGE_IDENTIFIER_ARABIC_TUNISIA => Ok(Language::Arabic(Arabic::Tunisia)),
            LANGUAGE_IDENTIFIER_ARABIC_OMAN => Ok(Language::Arabic(Arabic::Oman)),
            LANGUAGE_IDENTIFIER_ARABIC_YEMEN => Ok(Language::Arabic(Arabic::Yemen)),
            LANGUAGE_IDENTIFIER_ARABIC_SYRIA => Ok(Language::Arabic(Arabic::Syria)),
            LANGUAGE_IDENTIFIER_ARABIC_JORDAN => Ok(Language::Arabic(Arabic::Jordan)),
            LANGUAGE_IDENTIFIER_ARABIC_LEBANON => Ok(Language::Arabic(Arabic::Lebanon)),
            LANGUAGE_IDENTIFIER_ARABIC_KUWAIT => Ok(Language::Arabic(Arabic::Kuwait)),
            LANGUAGE_IDENTIFIER_ARABIC_UAE => Ok(Language::Arabic(Arabic::Uae)),
            LANGUAGE_IDENTIFIER_ARABIC_BAHRAIN => Ok(Language::Arabic(Arabic::Bahrain)),
            LANGUAGE_IDENTIFIER_ARABIC_QATAR => Ok(Language::Arabic(Arabic::Qatar)),
            LANGUAGE_IDENTIFIER_ARMENIAN => Ok(Language::Armenian),
            LANGUAGE_IDENTIFIER_ASSAMESE => Ok(Language::Assamese),
            LANGUAGE_IDENTIFIER_AZERI_LATIN => Ok(Language::Azeri(Azeri::Latin)),
            LANGUAGE_IDENTIFIER_AZERI_CYRILLIC => Ok(Language::Azeri(Azeri::Cyrillic)),
            LANGUAGE_IDENTIFIER_BASQUE => Ok(Language::Basque),
            LANGUAGE_IDENTIFIER_BELARUSSIAN => Ok(Language::Belarussian),
            LANGUAGE_IDENTIFIER_BENGALI => Ok(Language::Bengali),
            LANGUAGE_IDENTIFIER_BULGARIAN => Ok(Language::Bulgarian),
            LANGUAGE_IDENTIFIER_BURMESE => Ok(Language::Burmese),
            LANGUAGE_IDENTIFIER_CATALAN => Ok(Language::Catalan),
            LANGUAGE_IDENTIFIER_CHINESE_TAIWAN => Ok(Language::Chinese(Chinese::Taiwan)),
            LANGUAGE_IDENTIFIER_CHINESE_PRC => Ok(Language::Chinese(Chinese::Prc)),
            LANGUAGE_IDENTIFIER_CHINESE_HONG_KONG_SAR_PRC => {
                Ok(Language::Chinese(Chinese::HongKong))
            }
            LANGUAGE_IDENTIFIER_CHINESE_SINGAPORE => Ok(Language::Chinese(Chinese::Singapore)),
            LANGUAGE_IDENTIFIER_CHINESE_MACAU_SAR => Ok(Language::Chinese(Chinese::Macau)),
            LANGUAGE_IDENTIFIER_CROATIAN => Ok(Language::Croatian),
            LANGUAGE_IDENTIFIER_CZECH => Ok(Language::Czech),
            LANGUAGE_IDENTIFIER_DANISH => Ok(Language::Danish),
            LANGUAGE_IDENTIFIER_DUTCH_NETHERLANDS => Ok(Language::Dutch(Dutch::Netherlands)),
            LANGUAGE_IDENTIFIER_DUTCH_BELGIAN => Ok(Language::Dutch(Dutch::Belgium)),
            LANGUAGE_IDENTIFIER_ENGLISH_UNITED_STATES => {
                Ok(Language::English(English::UnitedStates))
            }
            LANGUAGE_IDENTIFIER_ENGLISH_UNITED_KINGDOM => {
                Ok(Language::English(English::UnitedKingdom))
            }
            LANGUAGE_IDENTIFIER_ENGLISH_AUSTRALIAN => Ok(Language::English(English::Australian)),
            LANGUAGE_IDENTIFIER_ENGLISH_CANADIAN => Ok(Language::English(English::Canadian)),
            LANGUAGE_IDENTIFIER_ENGLISH_NEW_ZEALAND => Ok(Language::English(English::NewZealand)),
            LANGUAGE_IDENTIFIER_ENGLISH_IRELAND => Ok(Language::English(English::Ireland)),
            LANGUAGE_IDENTIFIER_ENGLISH_SOUTH_AFRICA => Ok(Language::English(English::SouthAfrica)),
            LANGUAGE_IDENTIFIER_ENGLISH_JAMAICA => Ok(Language::English(English::Jamaica)),
            LANGUAGE_IDENTIFIER_ENGLISH_CARIBBEAN => Ok(Language::English(English::Caribbean)),
            LANGUAGE_IDENTIFIER_ENGLISH_BELIZE => Ok(Language::English(English::Belize)),
            LANGUAGE_IDENTIFIER_ENGLISH_TRINIDAD => Ok(Language::English(English::Trinidad)),
            LANGUAGE_IDENTIFIER_ENGLISH_ZIMBABWE => Ok(Language::English(English::Zimbabwe)),
            LANGUAGE_IDENTIFIER_ENGLISH_PHILIPPINES => Ok(Language::English(English::Philippines)),
            LANGUAGE_IDENTIFIER_ESTONIAN => Ok(Language::Estonian),
            LANGUAGE_IDENTIFIER_FAEROESE => Ok(Language::Faeroese),
            LANGUAGE_IDENTIFIER_FARSI => Ok(Language::Farsi),
            LANGUAGE_IDENTIFIER_FINNISH => Ok(Language::Finnish),
            LANGUAGE_IDENTIFIER_FRENCH_STANDARD => Ok(Language::French(French::Standard)),
            LANGUAGE_IDENTIFIER_FRENCH_BELGIAN => Ok(Language::French(French::Belgian)),
            LANGUAGE_IDENTIFIER_FRENCH_CANADIAN => Ok(Language::French(French::Canadian)),
            LANGUAGE_IDENTIFIER_FRENCH_SWITZERLAND => Ok(Language::French(French::Switzerland)),
            LANGUAGE_IDENTIFIER_FRENCH_LUXEMBOURG => Ok(Language::French(French::Luxembourg)),
            LANGUAGE_IDENTIFIER_FRENCH_MONACO => Ok(Language::French(French::Monaco)),
            LANGUAGE_IDENTIFIER_GEORGIAN => Ok(Language::Georgian),
            LANGUAGE_IDENTIFIER_GERMAN_STANDARD => Ok(Language::German(German::Standard)),
            LANGUAGE_IDENTIFIER_GERMAN_SWITZERLAND => Ok(Language::German(German::Switzerland)),
            LANGUAGE_IDENTIFIER_GERMAN_AUSTRIA => Ok(Language::German(German::Austria)),
            LANGUAGE_IDENTIFIER_GERMAN_LUXEMBOURG => Ok(Language::German(German::Luxembourg)),
            LANGUAGE_IDENTIFIER_GERMAN_LIECHTENSTEIN => Ok(Language::German(German::Liechtenstein)),
            LANGUAGE_IDENTIFIER_GREEK => Ok(Language::Greek),
            LANGUAGE_IDENTIFIER_GUJARATI => Ok(Language::Gujarati),
            LANGUAGE_IDENTIFIER_HEBREW => Ok(Language::Hebrew),
            LANGUAGE_IDENTIFIER_HINDI => Ok(Language::Hindi),
            LANGUAGE_IDENTIFIER_HUNGARIAN => Ok(Language::Hungarian),
            LANGUAGE_IDENTIFIER_ICELANDIC => Ok(Language::Icelandic),
            LANGUAGE_IDENTIFIER_INDONESIAN => Ok(Language::Indonesian),
            LANGUAGE_IDENTIFIER_ITALIAN_STANDARD => Ok(Language::Italian(Italian::Standard)),
            LANGUAGE_IDENTIFIER_ITALIAN_SWITZERLAND => Ok(Language::Italian(Italian::Switzerland)),
            LANGUAGE_IDENTIFIER_JAPANESE => Ok(Language::Japanese),
            LANGUAGE_IDENTIFIER_KANNADA => Ok(Language::Kannada),
            LANGUAGE_IDENTIFIER_KASHMIRI_INDIA => Ok(Language::Kashmiri(Kashmiri::India)),
            LANGUAGE_IDENTIFIER_KAZAKH => Ok(Language::Kazakh),
            LANGUAGE_IDENTIFIER_KONKANI => Ok(Language::Konkani),
            LANGUAGE_IDENTIFIER_KOREAN => Ok(Language::Korean(None)),
            LANGUAGE_IDENTIFIER_KOREAN_JOHAB => Ok(Language::Korean(Some(Korean::Johab))),
            LANGUAGE_IDENTIFIER_LATVIAN => Ok(Language::Latvian),
            LANGUAGE_IDENTIFIER_LITHUANIAN => Ok(Language::Lithuanian(None)),
            LANGUAGE_IDENTIFIER_LITHUANIAN_CLASSIC => {
                Ok(Language::Lithuanian(Some(Lithuanian::Classic)))
            }
            LANGUAGE_IDENTIFIER_MACEDONIAN => Ok(Language::Macedonian),
            LANGUAGE_IDENTIFIER_MALAY_MALAYSIAN => Ok(Language::Malay(Malay::Malaysian)),
            LANGUAGE_IDENTIFIER_MALAY_BRUNEI_DARUSSALAM => {
                Ok(Language::Malay(Malay::BruneiDarussalam))
            }
            LANGUAGE_IDENTIFIER_MALAYALAM => Ok(Language::Malayalam),
            LANGUAGE_IDENTIFIER_MANIPURI => Ok(Language::Manipuri),
            LANGUAGE_IDENTIFIER_MARATHI => Ok(Language::Marathi),
            LANGUAGE_IDENTIFIER_NEPALI_INDIA => Ok(Language::Nepali(Nepali::India)),
            LANGUAGE_IDENTIFIER_NORWEGIAN_BOKMAL => Ok(Language::Norwegian(Norwegian::Bokmal)),
            LANGUAGE_IDENTIFIER_NORWEGIAN_NYNORSK => Ok(Language::Norwegian(Norwegian::Nynorsk)),
            LANGUAGE_IDENTIFIER_ORIYA => Ok(Language::Oriya),
            LANGUAGE_IDENTIFIER_POLISH => Ok(Language::Polish),
            LANGUAGE_IDENTIFIER_PORTUGUESE_BRAZIL => Ok(Language::Portuguese(Portuguese::Brazil)),
            LANGUAGE_IDENTIFIER_PORTUGUESE_STANDARD => {
                Ok(Language::Portuguese(Portuguese::Standard))
            }
            LANGUAGE_IDENTIFIER_PUNJABI => Ok(Language::Punjabi),
            LANGUAGE_IDENTIFIER_ROMANIAN => Ok(Language::Romanian),
            LANGUAGE_IDENTIFIER_RUSSIAN => Ok(Language::Russian),
            LANGUAGE_IDENTIFIER_SANSKRIT => Ok(Language::Sanskrit),
            LANGUAGE_IDENTIFIER_SERBIAN_CYRILLIC => Ok(Language::Serbian(Serbian::Cyrillic)),
            LANGUAGE_IDENTIFIER_SERBIAN_LATIN => Ok(Language::Serbian(Serbian::Latin)),
            LANGUAGE_IDENTIFIER_SINDHI => Ok(Language::Sindhi),
            LANGUAGE_IDENTIFIER_SLOVAK => Ok(Language::Slovak),
            LANGUAGE_IDENTIFIER_SLOVENIAN => Ok(Language::Slovenian),
            LANGUAGE_IDENTIFIER_SPANISH_TRADITIONAL_SORT => {
                Ok(Language::Spanish(Spanish::TraditionalSort))
            }
            LANGUAGE_IDENTIFIER_SPANISH_MEXICAN => Ok(Language::Spanish(Spanish::Mexican)),
            LANGUAGE_IDENTIFIER_SPANISH_MODERN_SORT => Ok(Language::Spanish(Spanish::ModernSort)),
            LANGUAGE_IDENTIFIER_SPANISH_GUATEMALA => Ok(Language::Spanish(Spanish::Guatemala)),
            LANGUAGE_IDENTIFIER_SPANISH_COSTA_RICA => Ok(Language::Spanish(Spanish::CostaRica)),
            LANGUAGE_IDENTIFIER_SPANISH_PANAMA => Ok(Language::Spanish(Spanish::Panama)),
            LANGUAGE_IDENTIFIER_SPANISH_DOMINICAN_REPUBLIC => {
                Ok(Language::Spanish(Spanish::DominicanRepublic))
            }
            LANGUAGE_IDENTIFIER_SPANISH_VENEZUELA => Ok(Language::Spanish(Spanish::Venezuela)),
            LANGUAGE_IDENTIFIER_SPANISH_COLUMBIA => Ok(Language::Spanish(Spanish::Colombia)),
            LANGUAGE_IDENTIFIER_SPANISH_PERU => Ok(Language::Spanish(Spanish::Peru)),
            LANGUAGE_IDENTIFIER_SPANISH_ARGENTINA => Ok(Language::Spanish(Spanish::Argentina)),
            LANGUAGE_IDENTIFIER_SPANISH_ECUADOR => Ok(Language::Spanish(Spanish::Ecuador)),
            LANGUAGE_IDENTIFIER_SPANISH_CHILE => Ok(Language::Spanish(Spanish::Chile)),
            LANGUAGE_IDENTIFIER_SPANISH_URUGUAY => Ok(Language::Spanish(Spanish::Uruguay)),
            LANGUAGE_IDENTIFIER_SPANISH_PARAGUAY => Ok(Language::Spanish(Spanish::Paraguay)),
            LANGUAGE_IDENTIFIER_SPANISH_BOLIVIA => Ok(Language::Spanish(Spanish::Bolivia)),
            LANGUAGE_IDENTIFIER_SPANISH_EL_SALVADOR => Ok(Language::Spanish(Spanish::ElSalvador)),
            LANGUAGE_IDENTIFIER_SPANISH_HONDURAS => Ok(Language::Spanish(Spanish::Honduras)),
            LANGUAGE_IDENTIFIER_SPANISH_NICARAGUA => Ok(Language::Spanish(Spanish::Nicaragua)),
            LANGUAGE_IDENTIFIER_SPANISH_PUERTO_RICO => Ok(Language::Spanish(Spanish::PuertoRico)),
            LANGUAGE_IDENTIFIER_SUTU => Ok(Language::Sutu),
            LANGUAGE_IDENTIFIER_SWAHILI_KENYA => Ok(Language::Swahili(Swahili::Kenya)),
            LANGUAGE_IDENTIFIER_SWEDISH => Ok(Language::Swedish(None)),
            LANGUAGE_IDENTIFIER_SWEDISH_FINLAND => Ok(Language::Swedish(Some(Swedish::Finland))),
            LANGUAGE_IDENTIFIER_TAMIL => Ok(Language::Tamil),
            LANGUAGE_IDENTIFIER_TATAR_TATARSTAN => Ok(Language::Tatar(Tatar::Tatarstan)),
            LANGUAGE_IDENTIFIER_TELUGU => Ok(Language::Telugu),
            LANGUAGE_IDENTIFIER_THAI => Ok(Language::Thai),
            LANGUAGE_IDENTIFIER_TURKISH => Ok(Language::Turkish),
            LANGUAGE_IDENTIFIER_UKRAINIAN => Ok(Language::Ukrainian),
            LANGUAGE_IDENTIFIER_URDU_PAKISTAN => Ok(Language::Urdu(Urdu::Pakistan)),
            LANGUAGE_IDENTIFIER_URDU_INDIA => Ok(Language::Urdu(Urdu::India)),
            LANGUAGE_IDENTIFIER_UZBEK_LATIN => Ok(Language::Uzbek(Uzbek::Latin)),
            LANGUAGE_IDENTIFIER_UZBEK_CYRILLIC => Ok(Language::Uzbek(Uzbek::Cyrillic)),
            LANGUAGE_IDENTIFIER_VIETNAMESE => Ok(Language::Vietnamese),
            LANGUAGE_IDENTIFIER_HID_USAGE_DATA_DESCRIPTOR => {
                Ok(Language::Hid(Hid::UsageDataDescriptor))
            }
            LANGUAGE_IDENTIFIER_HID_VENDOR_DEFINED_1 => Ok(Language::Hid(Hid::VendorDefined1)),
            LANGUAGE_IDENTIFIER_HID_VENDOR_DEFINED_2 => Ok(Language::Hid(Hid::VendorDefined2)),
            LANGUAGE_IDENTIFIER_HID_VENDOR_DEFINED_3 => Ok(Language::Hid(Hid::VendorDefined3)),
            LANGUAGE_IDENTIFIER_HID_VENDOR_DEFINED_4 => Ok(Language::Hid(Hid::VendorDefined4)),
            _ => Err(()),
        }
    }
}
