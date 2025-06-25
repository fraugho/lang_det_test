use std::error::Error;
use whichlang::*;
use crate::record::*;

fn lang_to_str<'a>(language: &Lang) -> &'a str{
    match language {
        whichlang::Lang::Ara => {"ar"},
        whichlang::Lang::Cmn => {"zh"},
        whichlang::Lang::Deu => {"de"},
        whichlang::Lang::Eng => {"en"},
        whichlang::Lang::Fra => {"fr"},
        whichlang::Lang::Hin => {"hi"},
        whichlang::Lang::Ita => {"it"},
        whichlang::Lang::Jpn => {"ja"},
        whichlang::Lang::Kor => {"ko"},
        whichlang::Lang::Nld => {"nl"},
        whichlang::Lang::Por => {"pt"},
        whichlang::Lang::Rus => {"ru"},
        whichlang::Lang::Spa => {"es"},
        whichlang::Lang::Swe => {"sv"},
        whichlang::Lang::Tur => {"tr"},
        whichlang::Lang::Vie => {"vi"},
    }
}


pub fn test_whichlang(file: &mut csv::Reader<std::fs::File>) -> Result<(u32, u32), Box<dyn Error>> {
    let mut total = 0;
    let mut correct = 0;

    for result in file.deserialize() {
        let record: Record = result?;

        let answer = whichlang::detect_language(&record.text);
        let str_answer = lang_to_str(&answer);
        if str_answer == record.labels{
            correct += 1;
        }
        total += 1;
    }
    return Ok((correct, total));
}
