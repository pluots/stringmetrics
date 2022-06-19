use super::affix::Affix;
use super::affix_serde::{ENCODING_CLASS_LIST, TOKEN_CLASS_LIST};

/// All possible types found in hunspell affix files
/// This represents a generic token type that will have associated 
#[derive(PartialEq, Debug)]
pub enum TokenType {
    Encoding,
    FlagType,
    ComplexPrefixes,
    Language,
    IgnoreChars,
    AffixFlag,
    MorphAlias,

    // Suggestion-related
    NeighborKeys,
    TryCharacters,
    NoSuggestFlag,
    CompoundSuggestionsMax,
    NGramSuggestionsMax,
    NGramDiffMax,
    NGramLimitToDiffMax,
    NoSpaceSubs,
    KeepTerminationDots,
    Replacement,
    Mapping,
    Phonetic,
    WarnRareFlag,
    ForbitWarnWords,
    Breakpoint,

    // Compound-related
    CompoundRule,
    CompoundMinLength,
    CompoundFlag,
    CompoundBeginFlag,
    CompoundEndFlag,
    CompoundMiddleFlag,
    CompoundOnlyFlag,
    CompoundPermitFlag,
    CompoundForbidFlag,
    CompoundMoreSuffixes,
    CompoundRoot,
    CompoundWordMax,
    CompoundForbidDuplication,
    CompoundForbidRepeat,
    CompoundForbidUpperBoundary,
    CompoundForbidTriple,
    CompoundSimplifyTriple,
    CompoundForbidPatterns,
    CompoundForceUpper,
    CompoundForceSyllable,
    CompoundSyllableNumber,

    // Affix-related
    Prefix,
    Suffix,
    AffixCircumfixFlag,
    AffixForbiddenWordFlag,
    AffixFullStrip,
    AffixKeepCase,
    AffixInputConversion,
    AffixOutputConversion,
    AffixLemmaPresentDeprecated,
    AffixNeededFlag,
    AffixPseudoRootFlagDeprecated,
    AffixSubstandardFlag,
    AffixWordChars,
    AffixCheckSharps,
}


/// Basic enum methods to locate from a string
/// DONE
impl TokenType {
    /// Find a `TokenClass` from a token string
    fn from_token_key(key: &str) -> Option<&TokenType> {
        match TOKEN_CLASS_LIST.iter().find(|x| x.key == key) {
            Some(token_class) => Some(&token_class.class),
            None => None,
        }
    }

    /// Produce the token string of a token class
    fn to_token_str(&self) -> &'static str {
        TOKEN_CLASS_LIST
            .iter()
            .find(|x| x.class == *self)
            .unwrap()
            .key
    }
}


#[derive(PartialEq, Debug)]
pub enum EncodingType {
    Utf8,            // UTF-8
    Iso8859t1,       // ISO8859-1
    Iso8859t10,      // ISO8859-10
    Iso8859t13,      // ISO8859-13
    Iso8859t15,      // ISO8859-15
    Koi8r,           // KOI8-R
    Koi8u,           // KOI8-U
    Cp1251,          // cp1251
    IsciiDevanagari, // ISCII-DEVANAGARI
}

/// Basic enum methods to locate from a string
/// DONE
impl EncodingType {
    /// Find a `EncodingType` from a token string
    pub fn from_str(key: &str) -> Option<&EncodingType> {
        match ENCODING_CLASS_LIST.iter().find(|x| x.get_key() == key) {
            Some(x) => Some(&x.get_class()),
            None => None,
        }
    }

    /// Produce the token string of a token class
    pub fn to_token_str(&self) -> &'static str {
        ENCODING_CLASS_LIST
            .iter()
            .find(|x| x.get_class() == self)
            .unwrap()
            .get_key()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_from_str() {
        assert_eq!(
            TokenType::from_token_key("NOSUGGEST").unwrap(),
            &TokenType::NoSuggestFlag
        );
    }

    #[test]
    fn test_token_to_str() {
        assert_eq!(
            TokenType::to_token_str(&TokenType::NoSuggestFlag),
            "NOSUGGEST"
        );
    }

    fn test_match_encoding() {
        assert_eq!(
            *EncodingType::from_str("SET UTF-8").unwrap(),
            EncodingType::Utf8
        );
        assert_eq!(
            *EncodingType::from_str("SET ISO8859-1").unwrap(),
            EncodingType::Iso8859t1
        );
        assert_eq!(
            *EncodingType::from_str("SET ISO8859-10").unwrap(),
            EncodingType::Iso8859t10
        );
        assert_eq!(
            *EncodingType::from_str("SET ISO8859-13").unwrap(),
            EncodingType::Iso8859t13
        );
        assert_eq!(
            *EncodingType::from_str("SET ISO8859-15").unwrap(),
            EncodingType::Iso8859t15
        );
        assert_eq!(
            *EncodingType::from_str("SET KOI8-R").unwrap(),
            EncodingType::Koi8r
        );
        assert_eq!(
            *EncodingType::from_str("SET KOI8-U").unwrap(),
            EncodingType::Koi8u
        );
        assert_eq!(
            *EncodingType::from_str("SET cp1251").unwrap(),
            EncodingType::Cp1251
        );
        assert_eq!(
            *EncodingType::from_str("SET ISCII-DEVANAGARI").unwrap(),
            EncodingType::IsciiDevanagari
        );

        // Test whitespace differences
        assert_eq!(
            *EncodingType::from_str(" SET\tUTF-8 ").unwrap(),
            EncodingType::Utf8
        );
    }

    #[test]
    fn test_match_encoding_errors() {
        assert!(EncodingType::from_str("SET").is_none());
        assert!(EncodingType::from_str("SET iojsdoifjodi").is_none());
        assert!(EncodingType::from_str("not SET").is_none());
        assert!(EncodingType::from_str("").is_none());
    }
}
