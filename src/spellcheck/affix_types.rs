// use super::affix::Affix;
// use super::affix_serde::{ENCODING_CLASS_LIST, TOKEN_CLASS_LIST};

// use std::string::ToString;
use strum::EnumString;
use strum_macros;

/// All possible types found in hunspell affix files
/// This represents a generic token type that will have associated
#[derive(Debug, Eq, PartialEq, EnumString, strum_macros::Display)]
pub enum TokenType {
    #[strum(to_string = "SET")]
    Encoding,

    #[strum(to_string = "FLAG")]
    FlagType,

    #[strum(to_string = "COMPLEXPREFIXES")]
    ComplexPrefixes,

    #[strum(to_string = "LANG")]
    Language,
    
    #[strum(to_string = "IGNORE")]
    IgnoreChars,
    
    #[strum(to_string = "AF")]
    AffixFlag,
    
    #[strum(to_string = "AM")]
    MorphAlias,

    // Suggestion-related
    
    #[strum(to_string = "KEY")]
    NeighborKeys,
    
    #[strum(to_string = "TRY")]
    TryCharacters,
    
    #[strum(to_string = "NOSUGGEST")]
    NoSuggestFlag,
    
    #[strum(to_string = "MAXCPDSUGS")]
    CompoundSuggestionsMax,
    
    #[strum(to_string = "MAXNGRAMSUGS")]
    NGramSuggestionsMax,
    
    #[strum(to_string = "MAXDIFF")]
    NGramDiffMax,
    
    #[strum(to_string = "ONLYMAXDIFF")]
    NGramLimitToDiffMax,
    
    #[strum(to_string = "NOSPLITSUGS")]
    NoSpaceSubs,
    
    #[strum(to_string = "SUGSWITHDOTS")]
    KeepTerminationDots,
    
    #[strum(to_string = "REP")]
    Replacement,
    
    #[strum(to_string = "MAP")]
    Mapping,
    
    #[strum(to_string = "PHONE")]
    Phonetic,
    
    #[strum(to_string = "WARN")]
    WarnRareFlag,
    
    #[strum(to_string = "FORBIDWARN")]
    ForbitWarnWords,
    
    #[strum(to_string = "BREAK")]
    Breakpoint,

    // Compound-related
    
    #[strum(to_string = "COMPOUNDRULE")]
    CompoundRule,
    
    #[strum(to_string = "COMPOUNDMIN")]
    CompoundMinLength,
    
    #[strum(to_string = "COMPOUNDFLAG")]
    CompoundFlag,
    
    #[strum(to_string = "COMPOUNDBEGIN")]
    CompoundBeginFlag,
    
    #[strum(to_string = "COMPOUNDLAST")]
    CompoundEndFlag,
    
    #[strum(to_string = "COMPOUNDMIDDLE")]
    CompoundMiddleFlag,
    
    #[strum(to_string = "ONLYINCOMPOUND")]
    CompoundOnlyFlag,
    
    #[strum(to_string = "COMPOUNDPERMITFLAG")]
    CompoundPermitFlag,
    
    #[strum(to_string = "COMPOUNDFORBIDFLAG")]
    CompoundForbidFlag,
    
    #[strum(to_string = "COMPOUNDMORESUFFIXES")]
    CompoundMoreSuffixes,
    
    #[strum(to_string = "COMPOUNDROOT")]
    CompoundRoot,
    
    #[strum(to_string = "COMPOUNDWORDMAX")]
    CompoundWordMax,
    
    #[strum(to_string = "CHECKCOMPOUNDDUP")]
    CompoundForbidDuplication,
    
    #[strum(to_string = "CHECKCOMPOUNDREP")]
    CompoundForbidRepeat,
    
    #[strum(to_string = "CHECKCOMPOUNDCASE")]
    CompoundForbidUpperBoundary,
    
    #[strum(to_string = "CHECKCOMPOUNDTRIPLE")]
    CompoundForbidTriple,
    
    #[strum(to_string = "SIMPLIFIEDTRIPLE")]
    CompoundSimplifyTriple,
    
    #[strum(to_string = "CHECKCOMPOUNDPATTERN")]
    CompoundForbidPatterns,
    
    #[strum(to_string = "FORCEUCASE")]
    CompoundForceUpper,
    
    #[strum(to_string = "COMPOUNDSYLLABLE")]
    CompoundForceSyllable,
    
    #[strum(to_string = "SYLLABLENUM")]
    CompoundSyllableNumber,

    // Affix-related
    
    #[strum(to_string = "PFX")]
    Prefix,
    
    #[strum(to_string = "SFX")]
    Suffix,
    
    #[strum(to_string = "CIRCUMFIX")]
    AffixCircumfixFlag,
    
    #[strum(to_string = "FORBIDDENWORD")]
    AffixForbiddenWordFlag,
    
    #[strum(to_string = "FULLSTRIP")]
    AffixFullStrip,
    
    #[strum(to_string = "KEEPCASE")]
    AffixKeepCase,
    
    #[strum(to_string = "ICONV")]
    AffixInputConversion,
    
    #[strum(to_string = "OCONV")]
    AffixOutputConversion,
    
    #[strum(to_string = "LEMMA_PRESENT")]
    AffixLemmaPresentDeprecated,
    
    #[strum(to_string = "NEEDAFFIX")]
    AffixNeededFlag,
    
    #[strum(to_string = "PSEUDOROOT")]
    AffixPseudoRootFlagDeprecated,
    
    #[strum(to_string = "SUBSTANDARD")]
    AffixSubstandardFlag,
    
    #[strum(to_string = "WORDCHARS")]
    AffixWordChars,
    
    #[strum(to_string = "CHECKSHARPS")]
    AffixCheckSharps,
}


#[derive(Debug, Eq, PartialEq, EnumString, strum_macros::Display)]
pub enum EncodingType {
    #[strum(to_string = "UTF-8")]
    Utf8,            // UTF-8
    #[strum(to_string = "ISO8859-1")]
    Iso8859t1,       // ISO8859-1
    #[strum(to_string = "ISO8859-10")]
    Iso8859t10,      // ISO8859-10
    #[strum(to_string = "ISO8859-13")]
    Iso8859t13,      // ISO8859-13
    #[strum(to_string = "ISO8859-15")]
    Iso8859t15,      // ISO8859-15
    #[strum(to_string = "KOI8-R")]
    Koi8r,           // KOI8-R
    #[strum(to_string = "KOI8-U")]
    Koi8u,           // KOI8-U
    #[strum(to_string = "cp1251")]
    Cp1251,          // cp1251
    #[strum(to_string = "ISCII-DEVANAGARI")]
    IsciiDevanagari, // ISCII-DEVANAGARI
}



#[cfg(test)]
mod tests {
    use super::*;

    // Spot check deserialization of encoding
    #[test]
    fn test_encoding_deser() {
        assert_eq!(EncodingType::try_from("UTF-8").unwrap(), EncodingType::Utf8);
        assert_eq!(EncodingType::try_from("ISCII-DEVANAGARI").unwrap(), EncodingType::IsciiDevanagari);
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
        assert_eq!(TokenType::try_from("KEEPCASE").unwrap(), TokenType::AffixKeepCase);
    }

    // Spot check serializatino of tokens
    #[test]
    fn test_token_ser() {
        assert_eq!(TokenType::IgnoreChars.to_string(), "IGNORE");
        assert_eq!(TokenType::MorphAlias.to_string(), "AM");
    }
}
