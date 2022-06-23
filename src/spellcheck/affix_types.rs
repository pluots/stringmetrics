// use super::affix::Affix;
// use super::affix_serde::{ENCODING_CLASS_LIST, TOKEN_CLASS_LIST};

// use std::string::ToString;
// use strum::{EnumProperty, EnumString, VariantNames};
use strum::EnumString;
use strum_macros;

/// All possible types found in hunspell affix files
/// This represents a generic token type that will have associated
#[derive(
    Debug,
    Eq,
    PartialEq,
    EnumString,
    strum_macros::Display,
    strum_macros::EnumVariantNames,
    strum_macros::EnumProperty,
)]
pub enum TokenType {
    #[strum(to_string = "SET", props(dtype = "str"))]
    Encoding,

    #[strum(to_string = "FLAG", props(dtype = "str"))]
    FlagType,

    #[strum(to_string = "COMPLEXPREFIXES", props(dtype = "bool"))]
    ComplexPrefixes,

    #[strum(to_string = "LANG", props(dtype = "str"))]
    Language,

    #[strum(to_string = "IGNORE", props(dtype = "str"))]
    IgnoreChars,

    #[strum(to_string = "AF", props(dtype = "table"))]
    AffixFlag,

    #[strum(to_string = "AM", props(dtype = "table"))]
    MorphAlias,

    // Suggestion-related
    #[strum(to_string = "KEY", props(dtype = "str"))]
    NeighborKeys,

    #[strum(to_string = "TRY", props(dtype = "str"))]
    TryCharacters,

    #[strum(to_string = "NOSUGGEST", props(dtype = "str"))]
    NoSuggestFlag,

    #[strum(to_string = "MAXCPDSUGS", props(dtype = "int"))]
    CompoundSuggestionsMax,

    #[strum(to_string = "MAXNGRAMSUGS", props(dtype = "int"))]
    NGramSuggestionsMax,

    #[strum(to_string = "MAXDIFF", props(dtype = "int"))]
    NGramDiffMax,

    #[strum(to_string = "ONLYMAXDIFF", props(dtype = "bool"))]
    NGramLimitToDiffMax,

    #[strum(to_string = "NOSPLITSUGS", props(dtype = "bool"))]
    NoSpaceSubs,

    #[strum(to_string = "SUGSWITHDOTS", props(dtype = "bool"))]
    KeepTerminationDots,

    #[strum(to_string = "REP", props(dtype = "table"))]
    Replacement,

    #[strum(to_string = "MAP", props(dtype = "table"))]
    Mapping,

    #[strum(to_string = "PHONE", props(dtype = "table"))]
    Phonetic,

    #[strum(to_string = "WARN", props(dtype = "str"))]
    WarnRareFlag,

    #[strum(to_string = "FORBIDWARN", props(dtype = "bool"))]
    ForbitWarnWords,

    #[strum(to_string = "BREAK", props(dtype = "table"))]
    Breakpoint,

    // Compound-related
    #[strum(to_string = "COMPOUNDRULE", props(dtype = "table"))]
    CompoundRule,

    #[strum(to_string = "COMPOUNDMIN", props(dtype = "int"))]
    CompoundMinLength,

    #[strum(to_string = "COMPOUNDFLAG", props(dtype = "str"))]
    CompoundFlag,

    #[strum(to_string = "COMPOUNDBEGIN", props(dtype = "str"))]
    CompoundBeginFlag,

    #[strum(to_string = "COMPOUNDLAST", props(dtype = "str"))]
    CompoundEndFlag,

    #[strum(to_string = "COMPOUNDMIDDLE", props(dtype = "str"))]
    CompoundMiddleFlag,

    #[strum(to_string = "ONLYINCOMPOUND", props(dtype = "str"))]
    CompoundOnlyFlag,

    #[strum(to_string = "COMPOUNDPERMITFLAG", props(dtype = "str"))]
    CompoundPermitFlag,

    #[strum(to_string = "COMPOUNDFORBIDFLAG", props(dtype = "str"))]
    CompoundForbidFlag,

    #[strum(to_string = "COMPOUNDMORESUFFIXES", props(dtype = "bool"))]
    CompoundMoreSuffixes,

    #[strum(to_string = "COMPOUNDROOT", props(dtype = "str"))]
    CompoundRoot,

    #[strum(to_string = "COMPOUNDWORDMAX", props(dtype = "int"))]
    CompoundWordMax,

    #[strum(to_string = "CHECKCOMPOUNDDUP", props(dtype = "bool"))]
    CompoundForbidDuplication,

    #[strum(to_string = "CHECKCOMPOUNDREP", props(dtype = "bool"))]
    CompoundForbidRepeat,

    #[strum(to_string = "CHECKCOMPOUNDCASE", props(dtype = "bool"))]
    CompoundForbidUpperBoundary,

    #[strum(to_string = "CHECKCOMPOUNDTRIPLE", props(dtype = "bool"))]
    CompoundForbidTriple,

    #[strum(to_string = "SIMPLIFIEDTRIPLE", props(dtype = "bool"))]
    CompoundSimplifyTriple,

    #[strum(to_string = "CHECKCOMPOUNDPATTERN", props(dtype = "table"))]
    CompoundForbidPatterns,

    #[strum(to_string = "FORCEUCASE", props(dtype = "str"))]
    CompoundForceUpper,

    #[strum(to_string = "COMPOUNDSYLLABLE", props(dtype = "str"))]
    CompoundForceSyllable,

    #[strum(to_string = "SYLLABLENUM", props(dtype = "str"))]
    CompoundSyllableNumber,

    // Affix-related
    #[strum(to_string = "PFX", props(dtype = "table"))]
    Prefix,

    #[strum(to_string = "SFX", props(dtype = "table"))]
    Suffix,

    #[strum(to_string = "CIRCUMFIX", props(dtype = "str"))]
    AffixCircumfixFlag,

    #[strum(to_string = "FORBIDDENWORD", props(dtype = "str"))]
    AffixForbiddenWordFlag,

    #[strum(to_string = "FULLSTRIP", props(dtype = "bool"))]
    AffixFullStrip,

    #[strum(to_string = "KEEPCASE", props(dtype = "str"))]
    AffixKeepCase,

    #[strum(to_string = "ICONV", props(dtype = "table"))]
    AffixInputConversion,

    #[strum(to_string = "OCONV", props(dtype = "table"))]
    AffixOutputConversion,

    #[strum(to_string = "LEMMA_PRESENT", props(dtype = "str"))]
    AffixLemmaPresentDeprecated,

    #[strum(to_string = "NEEDAFFIX", props(dtype = "str"))]
    AffixNeededFlag,

    #[strum(to_string = "PSEUDOROOT", props(dtype = "str"))]
    AffixPseudoRootFlagDeprecated,

    #[strum(to_string = "SUBSTANDARD", props(dtype = "str"))]
    AffixSubstandardFlag,

    #[strum(to_string = "WORDCHARS", props(dtype = "str"))]
    AffixWordChars,

    #[strum(to_string = "CHECKSHARPS", props(dtype = "bool"))]
    AffixCheckSharps,

    // Used to indicate start of token stream
    FileStart,
}

#[derive(
    Debug, Eq, PartialEq, EnumString, strum_macros::Display, strum_macros::EnumVariantNames,
)]
pub enum EncodingType {
    #[strum(to_string = "UTF-8")]
    Utf8, // UTF-8
    #[strum(to_string = "ISO8859-1")]
    Iso8859t1, // ISO8859-1
    #[strum(to_string = "ISO8859-10")]
    Iso8859t10, // ISO8859-10
    #[strum(to_string = "ISO8859-13")]
    Iso8859t13, // ISO8859-13
    #[strum(to_string = "ISO8859-15")]
    Iso8859t15, // ISO8859-15
    #[strum(to_string = "KOI8-R")]
    Koi8r, // KOI8-R
    #[strum(to_string = "KOI8-U")]
    Koi8u, // KOI8-U
    #[strum(to_string = "cp1251")]
    Cp1251, // cp1251
    #[strum(to_string = "ISCII-DEVANAGARI")]
    IsciiDevanagari, // ISCII-DEVANAGARI
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::{EnumProperty, VariantNames};

    // Spot check deserialization of encoding
    #[test]
    fn test_encoding_deser() {
        assert_eq!(EncodingType::try_from("UTF-8").unwrap(), EncodingType::Utf8);
        assert_eq!(
            EncodingType::try_from("ISCII-DEVANAGARI").unwrap(),
            EncodingType::IsciiDevanagari
        );
    }

    // Spot check serializatino of encoding
    #[test]
    fn test_encoding_ser() {
        assert_eq!(EncodingType::Utf8.to_string(), "UTF-8");
        assert_eq!(EncodingType::Iso8859t15.to_string(), "ISO8859-15");
    }

    // Spot check deserialization of tokens
    #[test]
    fn test_token_deser() {
        assert_eq!(TokenType::try_from("PFX").unwrap(), TokenType::Prefix);
        assert_eq!(
            TokenType::try_from("KEEPCASE").unwrap(),
            TokenType::AffixKeepCase
        );
    }

    // Spot check serialization of tokens
    #[test]
    fn test_token_ser() {
        assert_eq!(TokenType::IgnoreChars.to_string(), "IGNORE");
        assert_eq!(TokenType::MorphAlias.to_string(), "AM");
        println!("{:?}", TokenType::VARIANTS);
    }

    // Spot check deserialization of tokens
    #[test]
    fn test_token_props() {
        assert_eq!(TokenType::Encoding.get_str("dtype"), Some("str"));
    }
}
