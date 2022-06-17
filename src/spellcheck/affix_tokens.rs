use super::affix::Affix;
use lazy_static::lazy_static;

/// All possible types found in hunspell affix files
#[derive(PartialEq, Debug)]
enum TokenType {
    Encoding,
    FlagType,
    ComplexPrefixes,
    Language,
    IgnoreChars,
    AffixFlag,
    MorphAlias,

    // Suggestion-related
    NeighborKeys,
    TryChars,
    NoSuggestFlag,
    CompoundSuggestionsMax,
    NGramSuggestionsMax,
    NGramDiffMax,
    NGramOnlyKeepOne,
    NoSpaceSubs,
    KeepTerminatingDots,
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
    AffixWordCharacters,
    AffixCheckSharps,
}

/// A structure holding information about a token and how to use it
struct TokenClass<'a> {
    // Kind of the token
    class: TokenType,
    // Token's name in the dict
    key: &'a str,
    // A function that takes in the token's string and determines
    // how many of the following tokens to consume. If none, not a table.
    table_consumes: Option<fn(s: &str) -> u16>,
    set_parent: Option<fn(&TokenClass, &Affix)>,
}

/// A collection of all the possible tokens
///
/// We don't use all of these, but that's OK. Just need to have the tokens
/// defined so that we don't miss one. This is relevant because unfortunately,
/// it seems like line breaks aren't strictly necesary
/// http://pwet.fr/man/linux/fichiers_speciaux/hunspell/
///
/// If something is unused, set_parent just has to be None
const TOKEN_CLASS_LIST: [TokenClass; 57] = [
    TokenClass {
        class: TokenType::Encoding,
        key: "SET",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::FlagType,
        key: "FLAG",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::ComplexPrefixes,
        key: "COMPLEXPREFIXES",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::Language,
        key: "LANG",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::IgnoreChars,
        key: "IGNORE",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::AffixFlag,
        key: "AF",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::MorphAlias,
        key: "AM",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::NeighborKeys,
        key: "KEY",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::TryChars,
        key: "TRY",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::NoSuggestFlag,
        key: "NOSUGGEST",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::CompoundSuggestionsMax,
        key: "MAXCPDSUGS",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::NGramSuggestionsMax,
        key: "MAXNGRAMSUGS",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::NGramDiffMax,
        key: "MAXDIFF",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::NGramOnlyKeepOne,
        key: "ONLYMAXDIFF",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::NoSpaceSubs,
        key: "NOSPLITSUGS",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::KeepTerminatingDots,
        key: "SUGSWITHDOTS",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::Replacement,
        key: "REP",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::Mapping,
        key: "MAP",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::Phonetic,
        key: "PHONE",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::WarnRareFlag,
        key: "WARN",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::ForbitWarnWords,
        key: "FORBIDWARN",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::Breakpoint,
        key: "BREAK",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::CompoundRule,
        key: "COMPOUNDRULE",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::CompoundMinLength,
        key: "COMPOUNDMIN",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::CompoundFlag,
        key: "COMPOUNDFLAG",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::CompoundBeginFlag,
        key: "COMPOUNDBEGIN",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::CompoundEndFlag,
        key: "COMPOUNDLAST",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::CompoundMiddleFlag,
        key: "COMPOUNDMIDDLE",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::CompoundOnlyFlag,
        key: "ONLYINCOMPOUND",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::CompoundPermitFlag,
        key: "COMPOUNDPERMITFLAG",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::CompoundForbidFlag,
        key: "COMPOUNDFORBIDFLAG",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::CompoundMoreSuffixes,
        key: "COMPOUNDMORESUFFIXES",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::CompoundRoot,
        key: "COMPOUNDROOT",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::CompoundWordMax,
        key: "COMPOUNDWORDMAX",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::CompoundForbidDuplication,
        key: "CHECKCOMPOUNDDUP",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::CompoundForbidRepeat,
        key: "CHECKCOMPOUNDREP",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::CompoundForbidUpperBoundary,
        key: "CHECKCOMPOUNDCASE",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::CompoundForbidTriple,
        key: "CHECKCOMPOUNDTRIPLE",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::CompoundSimplifyTriple,
        key: "SIMPLIFIEDTRIPLE",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::CompoundForbidPatterns,
        key: "CHECKCOMPOUNDPATTERN",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::CompoundForceUpper,
        key: "FORCEUCASE",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::CompoundForceSyllable,
        key: "COMPOUNDSYLLABLE",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::CompoundSyllableNumber,
        key: "SYLLABLENUM",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::Prefix,
        key: "PFX",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::Suffix,
        key: "SFX",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::AffixCircumfixFlag,
        key: "CIRCUMFIX",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::AffixForbiddenWordFlag,
        key: "FORBIDDENWORD",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::AffixFullStrip,
        key: "FULLSTRIP",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::AffixKeepCase,
        key: "KEEPCASE",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::AffixInputConversion,
        key: "ICONV",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::AffixOutputConversion,
        key: "OCONV",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::AffixLemmaPresentDeprecated,
        key: "LEMMA_PRESENT",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::AffixNeededFlag,
        key: "NEEDAFFIX",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::AffixPseudoRootFlagDeprecated,
        key: "PSEUDOROOT",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::AffixSubstandardFlag,
        key: "SUBSTANDARD",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::AffixWordCharacters,
        key: "WORDCHARS",
        table_consumes: None,
        set_parent: None,
    },
    TokenClass {
        class: TokenType::AffixCheckSharps,
        key: "CHECKSHARPS",
        table_consumes: None,
        set_parent: None,
    },
];

lazy_static! {
    /// All possible keys collected into a vector
    static ref TOKEN_KEYS:Vec<&'static str> = TOKEN_CLASS_LIST.into_iter().map(|x| x.key).collect();
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
    fn to_token_str(self) -> &'static str {
        TOKEN_CLASS_LIST
            .iter()
            .find(|x| x.class == self)
            .unwrap()
            .key
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(
            TokenType::from_token_key("NOSUGGEST").unwrap(),
            &TokenType::NoSuggestFlag
        );
    }

    #[test]
    fn test_to_str() {
        assert_eq!(
            TokenType::to_token_str(TokenType::NoSuggestFlag),
            "NOSUGGEST"
        );
    }
}
