#ifndef LANG_DET_H
#define LANG_DET_H

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
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
} LanguageEnum;


// Detect language of a single string
LanguageEnum lingua_process_string(const char* input);
LanguageEnum whichlang_process_string(const char* input);

#ifdef __cplusplus
}
#endif

#endif // LANG_DET_H
