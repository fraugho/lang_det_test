use std::error::Error;
use lingua::LanguageDetectorBuilder;
use crate::record::*;

fn lang_to_str<'a>(language: &lingua::Language) -> &'a str{
    match language{
        lingua::Language::Afrikaans => {"af"},
        lingua::Language::Albanian => {"sq"},
        lingua::Language::Arabic => {"ar"},
        lingua::Language::Armenian => {"hy"},
        lingua::Language::Azerbaijani => {"az"},
        lingua::Language::Basque => {"eu"},
        lingua::Language::Belarusian => {"be"},
        lingua::Language::Bengali => {"bn"},
        lingua::Language::Bokmal => {"nb"},
        lingua::Language::Bosnian => {"bs"},
        lingua::Language::Bulgarian => {"bg"},
        lingua::Language::Catalan => {"ca"},
        lingua::Language::Chinese => {"zh"},
        lingua::Language::Croatian => {"hr"},
        lingua::Language::Czech => {"cs"},
        lingua::Language::Danish => {"da"},
        lingua::Language::Dutch => {"nl"},
        lingua::Language::English => {"en"},
        lingua::Language::Esperanto => {"eo"},
        lingua::Language::Estonian => {"et"},
        lingua::Language::Finnish => {"fi"},
        lingua::Language::French => {"fr"},
        lingua::Language::Ganda => {"lg"},
        lingua::Language::Georgian => {"ka"},
        lingua::Language::German => {"de"},
        lingua::Language::Greek => {"el"},
        lingua::Language::Gujarati => {"gu"},
        lingua::Language::Hebrew => {"he"},
        lingua::Language::Hindi => {"hi"},
        lingua::Language::Hungarian => {"hu"},
        lingua::Language::Icelandic => {"is"},
        lingua::Language::Indonesian => {"id"},
        lingua::Language::Irish => {"ga"},
        lingua::Language::Italian => {"it"},
        lingua::Language::Japanese => {"ja"},
        lingua::Language::Kazakh => {"kk"},
        lingua::Language::Korean => {"ko"},
        lingua::Language::Latin => {"la"},
        lingua::Language::Latvian => {"lv"},
        lingua::Language::Lithuanian => {"lt"},
        lingua::Language::Macedonian => {"mk"},
        lingua::Language::Malay => {"ms"},
        lingua::Language::Maori => {"mi"},
        lingua::Language::Marathi => {"mr"},
        lingua::Language::Mongolian => {"mn"},
        lingua::Language::Nynorsk => {"nn"},
        lingua::Language::Persian => {"fa"},
        lingua::Language::Polish => {"pl"},
        lingua::Language::Portuguese => {"pt"},
        lingua::Language::Punjabi => {"pa"},
        lingua::Language::Romanian => {"ro"},
        lingua::Language::Russian => {"ru"},
        lingua::Language::Serbian => {"sr"},
        lingua::Language::Shona => {"sn"},
        lingua::Language::Slovak => {"sk"},
        lingua::Language::Slovene => {"sl"},
        lingua::Language::Somali => {"so"},
        lingua::Language::Sotho => {"st"},
        lingua::Language::Spanish => {"es"},
        lingua::Language::Swahili => {"sw"},
        lingua::Language::Swedish => {"sv"},
        lingua::Language::Tagalog => {"tl"},
        lingua::Language::Tamil => {"ta"},
        lingua::Language::Telugu => {"te"},
        lingua::Language::Thai => {"th"},
        lingua::Language::Tsonga => {"ts"},
        lingua::Language::Tswana => {"tn"},
        lingua::Language::Turkish => {"tr"},
        lingua::Language::Ukrainian => {"uk"},
        lingua::Language::Urdu => {"ur"},
        lingua::Language::Vietnamese => {"vi"},
        lingua::Language::Welsh => {"cy"},
        lingua::Language::Xhosa => {"xh"},
        lingua::Language::Yoruba => {"yo"},
        lingua::Language::Zulu => {"zu"},
    }
}

pub fn test_lingua(file: &mut csv::Reader<std::fs::File>) -> Result<(u32, u32), Box<dyn Error>> {
    let mut total = 0;
    let mut correct = 0;
    let detector = LanguageDetectorBuilder::from_all_languages().build();

    for result in file.deserialize() {
        let record: Record = result?;

        let answer = match detector.detect_language_of(record.text){
            Some(x) => {x},
            None => {lingua::Language::English}
        };
        let str_answer = lang_to_str(&answer);
        if str_answer == record.labels{
            correct += 1;
        }
        total += 1;
    }
    return Ok((correct, total));
}
