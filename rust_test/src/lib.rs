use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use lingua::LanguageDetectorBuilder;
use whichlang::*;

#[repr(C)]
pub enum LanguageEnum {
    AF, // Afrikaans
    SQ, // Albanian
    AR, // Arabic
    HY, // Armenian
    AZ, // Azerbaijani
    EU, // Basque
    BE, // Belarusian
    BN, // Bengali
    NB, // Bokmål
    BS, // Bosnian
    BG, // Bulgarian
    CA, // Catalan
    ZH, // Chinese
    HR, // Croatian
    CS, // Czech
    DA, // Danish
    NL, // Dutch
    EN, // English
    EO, // Esperanto
    ET, // Estonian
    FI, // Finnish
    FR, // French
    LG, // Ganda
    KA, // Georgian
    DE, // German
    EL, // Greek
    GU, // Gujarati
    HE, // Hebrew
    HI, // Hindi
    HU, // Hungarian
    IS, // Icelandic
    ID, // Indonesian
    GA, // Irish
    IT, // Italian
    JA, // Japanese
    KK, // Kazakh
    KO, // Korean
    LA, // Latin
    LV, // Latvian
    LT, // Lithuanian
    MK, // Macedonian
    MS, // Malay
    MI, // Maori
    MR, // Marathi
    MN, // Mongolian
    NN, // Nynorsk
    FA, // Persian
    PL, // Polish
    PT, // Portuguese
    PA, // Punjabi
    RO, // Romanian
    RU, // Russian
    SR, // Serbian
    SN, // Shona
    SK, // Slovak
    SL, // Slovene
    SO, // Somali
    ST, // Sotho
    ES, // Spanish
    SW, // Swahili
    SV, // Swedish
    TL, // Tagalog
    TA, // Tamil
    TE, // Telugu
    TH, // Thai
    TS, // Tsonga
    TN, // Tswana
    TR, // Turkish
    UK, // Ukrainian
    UR, // Urdu
    VI, // Vietnamese
    CY, // Welsh
    XH, // Xhosa
    YO, // Yoruba
    ZU  // Zulu
}

fn lingua_to_language_enum(language: lingua::Language) -> LanguageEnum {
    match language {
        lingua::Language::Afrikaans => LanguageEnum::AF,
        lingua::Language::Albanian => LanguageEnum::SQ,
        lingua::Language::Arabic => LanguageEnum::AR,
        lingua::Language::Armenian => LanguageEnum::HY,
        lingua::Language::Azerbaijani => LanguageEnum::AZ,
        lingua::Language::Basque => LanguageEnum::EU,
        lingua::Language::Belarusian => LanguageEnum::BE,
        lingua::Language::Bengali => LanguageEnum::BN,
        lingua::Language::Bokmal => LanguageEnum::NB,
        lingua::Language::Bosnian => LanguageEnum::BS,
        lingua::Language::Bulgarian => LanguageEnum::BG,
        lingua::Language::Catalan => LanguageEnum::CA,
        lingua::Language::Chinese => LanguageEnum::ZH,
        lingua::Language::Croatian => LanguageEnum::HR,
        lingua::Language::Czech => LanguageEnum::CS,
        lingua::Language::Danish => LanguageEnum::DA,
        lingua::Language::Dutch => LanguageEnum::NL,
        lingua::Language::English => LanguageEnum::EN,
        lingua::Language::Esperanto => LanguageEnum::EO,
        lingua::Language::Estonian => LanguageEnum::ET,
        lingua::Language::Finnish => LanguageEnum::FI,
        lingua::Language::French => LanguageEnum::FR,
        lingua::Language::Ganda => LanguageEnum::LG,
        lingua::Language::Georgian => LanguageEnum::KA,
        lingua::Language::German => LanguageEnum::DE,
        lingua::Language::Greek => LanguageEnum::EL,
        lingua::Language::Gujarati => LanguageEnum::GU,
        lingua::Language::Hebrew => LanguageEnum::HE,
        lingua::Language::Hindi => LanguageEnum::HI,
        lingua::Language::Hungarian => LanguageEnum::HU,
        lingua::Language::Icelandic => LanguageEnum::IS,
        lingua::Language::Indonesian => LanguageEnum::ID,
        lingua::Language::Irish => LanguageEnum::GA,
        lingua::Language::Italian => LanguageEnum::IT,
        lingua::Language::Japanese => LanguageEnum::JA,
        lingua::Language::Kazakh => LanguageEnum::KK,
        lingua::Language::Korean => LanguageEnum::KO,
        lingua::Language::Latin => LanguageEnum::LA,
        lingua::Language::Latvian => LanguageEnum::LV,
        lingua::Language::Lithuanian => LanguageEnum::LT,
        lingua::Language::Macedonian => LanguageEnum::MK,
        lingua::Language::Malay => LanguageEnum::MS,
        lingua::Language::Maori => LanguageEnum::MI,
        lingua::Language::Marathi => LanguageEnum::MR,
        lingua::Language::Mongolian => LanguageEnum::MN,
        lingua::Language::Nynorsk => LanguageEnum::NN,
        lingua::Language::Persian => LanguageEnum::FA,
        lingua::Language::Polish => LanguageEnum::PL,
        lingua::Language::Portuguese => LanguageEnum::PT,
        lingua::Language::Punjabi => LanguageEnum::PA,
        lingua::Language::Romanian => LanguageEnum::RO,
        lingua::Language::Russian => LanguageEnum::RU,
        lingua::Language::Serbian => LanguageEnum::SR,
        lingua::Language::Shona => LanguageEnum::SN,
        lingua::Language::Slovak => LanguageEnum::SK,
        lingua::Language::Slovene => LanguageEnum::SL,
        lingua::Language::Somali => LanguageEnum::SO,
        lingua::Language::Sotho => LanguageEnum::ST,
        lingua::Language::Spanish => LanguageEnum::ES,
        lingua::Language::Swahili => LanguageEnum::SW,
        lingua::Language::Swedish => LanguageEnum::SV,
        lingua::Language::Tagalog => LanguageEnum::TL,
        lingua::Language::Tamil => LanguageEnum::TA,
        lingua::Language::Telugu => LanguageEnum::TE,
        lingua::Language::Thai => LanguageEnum::TH,
        lingua::Language::Tsonga => LanguageEnum::TS,
        lingua::Language::Tswana => LanguageEnum::TN,
        lingua::Language::Turkish => LanguageEnum::TR,
        lingua::Language::Ukrainian => LanguageEnum::UK,
        lingua::Language::Urdu => LanguageEnum::UR,
        lingua::Language::Vietnamese => LanguageEnum::VI,
        lingua::Language::Welsh => LanguageEnum::CY,
        lingua::Language::Xhosa => LanguageEnum::XH,
        lingua::Language::Yoruba => LanguageEnum::YO,
        lingua::Language::Zulu => LanguageEnum::ZU,
    }
}

fn whichlang_to_language_enum(language: whichlang::Lang) -> LanguageEnum {
    match language {
        whichlang::Lang::Ara => LanguageEnum::AR,
        whichlang::Lang::Cmn => LanguageEnum::ZH,
        whichlang::Lang::Deu => LanguageEnum::DE,
        whichlang::Lang::Eng => LanguageEnum::EN,
        whichlang::Lang::Fra => LanguageEnum::FR,
        whichlang::Lang::Hin => LanguageEnum::HI,
        whichlang::Lang::Ita => LanguageEnum::IT,
        whichlang::Lang::Jpn => LanguageEnum::JA,
        whichlang::Lang::Kor => LanguageEnum::KO,
        whichlang::Lang::Nld => LanguageEnum::NL,
        whichlang::Lang::Por => LanguageEnum::PT,
        whichlang::Lang::Rus => LanguageEnum::RU,
        whichlang::Lang::Spa => LanguageEnum::ES,
        whichlang::Lang::Swe => LanguageEnum::SV,
        whichlang::Lang::Tur => LanguageEnum::TR,
        whichlang::Lang::Vie => LanguageEnum::VI,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn lingua_process_string(input: *const c_char)-> LanguageEnum {
    if input.is_null() {
        return LanguageEnum::EN;
    }

    let c_str = unsafe { CStr::from_ptr(input) };

    let rust_str = match c_str.to_str() {
        Ok(str) => str,
        Err(_) => return LanguageEnum::EN
    };

    let detector = LanguageDetectorBuilder::from_all_languages().build();
    match detector.detect_language_of(rust_str){
        Some(x) => return lingua_to_language_enum(x),
        None => return LanguageEnum::EN
    };
}

#[unsafe(no_mangle)]
pub extern "C" fn whichlang_process_string(input: *const c_char)-> LanguageEnum {
    if input.is_null() {
        return LanguageEnum::EN;
    }
    let c_str = unsafe { CStr::from_ptr(input) };

    let rust_str = match c_str.to_str() {
        Ok(str) => str,
        Err(_) => return LanguageEnum::EN
    };

    let result = whichlang::detect_language(rust_str);
    return whichlang_to_language_enum(result);
}
