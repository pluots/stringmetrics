//! Affix tokens
//!
//! This module is used for things related to parsing from a file It contains a
//! lot of ugly syntax and wild macros to attempt to minimize boilerplate

use super::affix::Affix;
use super::affix_types::*;
use lazy_static::lazy_static;
use unicode_segmentation::UnicodeSegmentation;

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

mod re_exprs {
    use lazy_static::lazy_static;
    use regex::Regex;
    lazy_static! {
        /// All possible keys collected into a vector
        ///
        pub static ref AFFIX_FLAG_RE: Regex = Regex::new(r"AF\s+(\d+)").unwrap();
        pub static ref MORPH_ALIAS_RE: Regex = Regex::new(r"AM\s+(\d+)").unwrap();
        pub static ref REPLACE_DEF_RE: Regex = Regex::new(r"REP\s+(\d+)").unwrap();
        pub static ref MAP_DEF_RE: Regex = Regex::new(r"MAP\s+(\d+)").unwrap();
        pub static ref PHONETIC_DEF_RE: Regex = Regex::new(r"PHONE\s+(\d+)").unwrap();
        pub static ref BREAK_DEF_RE: Regex = Regex::new(r"BREAK\s+(\d+)").unwrap();
        pub static ref COMPOUND_RULE_DEF_RE: Regex = Regex::new(r"COMPOUNDRULE\s+(\d+)").unwrap();
        pub static ref COMPOUND_PATTERN_DEF_RE: Regex = Regex::new(r"CHECKCOMPOUNDPATTERN\s+(\d+)").unwrap();
        pub static ref ICONV_DEF_RE: Regex = Regex::new(r"ICONV\s+(\d+)").unwrap();
        pub static ref OCONV_DEF_RE: Regex = Regex::new(r"OCONV\s+(\d+)").unwrap();
        pub static ref PFX_DEF_RE: Regex = Regex::new(r"PFX\s+\w+\s+\w+\s+(\d+)").unwrap();
        pub static ref SFX_DEF_RE: Regex = Regex::new(r"SFX\s+\w+\s+\w+\s+(\d+)").unwrap();

    }

    /// Apply a regex pattern to a string, return integer capturing group
    pub fn apply_re_count(s: &str, re: &Regex) -> u16 {
        re.captures_iter(s)
            .next()
            .unwrap_or_else(|| panic!("Bad formatting at {}", s))[1]
            .parse()
            .unwrap()
    }
}

/// Macro that creates a closure based on the relevant type, used for
/// TokenClass::set_parent
///
/// Macros so powerful they should be illegal
///
/// All of our finctions return Option<Callable -> Result>
///
/// Usage:
/// `parentify!(field_name, bool)` will just set the relevant field true
///
macro_rules! parentify {
    // Boolean field just assigns true and returns Ok (Flag is just either there
    // or not)
    ( $field:ident, bool ) => {
        Some(|_, mut ax, _| Ok(ax.$field = true))
    };

    // Integer fields are a bit more complicated
    // Need to parse the field if possible,
    ( $field:ident, int ) => {
        Some(|tc, ax, s| match tc.strip_key(s).parse() {
            Ok(v) => Ok(ax.$field = v),
            Err(_) => Err("Bad integer value at {}"),
        })
    };

    // Use str_replace for any time we have `&str` as the `Affix` type. We will
    // replace what exists with our new value
    ( $field:ident, str_replace ) => {
        Some(|tc, ax, s| {
            let s1 = tc.strip_key(s);
            match s1.is_empty() {
                false => Ok(ax.$field = s1),
                true => Err("No values found at field {}"),
            }
        })
    };

    // Same as above but place the result in an option
    ( $field:ident, str_replace_option ) => {
        Some(|tc, ax, s| {
            let s1 = tc.strip_key(s);
            match s1.is_empty() {
                false => Ok(ax.$field = Some(s1)),
                true => Err("No values found at field {}"),
            }
        })
    };

    // Use str_add any time we have a `String` that we want to append to.
    // Basically the same as above except we append to the existing vector and
    // sort rather than replacing what's there. Usable for `Vec<&str>`.
    ( $field:ident, str_add ) => {
        Some(|tc, ax, s| {
            let s1 = tc.strip_key(s).to_string();
            match s1.is_empty() {
                false => Ok({
                    let mut tmp = s.graphemes(true).collect::<Vec<&str>>();
                    tmp.sort();
                    tmp.dedup();
                    ax.$field = tmp
                }),
                true => Err("No values found at field {}"),
            }
        })
    };
}

/// A structure holding information about a token and how to use it
///
/// This is meant for internal use in parsing the file Note that in parsing via
/// set_parent, APPEND mode is used wherever possible. Make sure if you come
/// start with a pre-populated Affix class, you clear the relevant fields first.
struct TokenClass<'a> {
    // Kind of the token
    class: TokenType,
    // Token's name in the dict
    key: &'a str,
    // A function that takes in the token's string and determines
    // how many of the following tokens to consume. If none, not a table.
    table_consumes: Option<fn(s: &str) -> u16>,
    // Set the parent when passed the foll text token
    // Idiomatic fn(self, parent, str)
    // Returns a result for nice error handling
    // Use the macro above to make setting this easy
    set_parent: Option<fn(&TokenClass, &mut Affix, &'a str) -> Result<(), &'static str>>,
}

impl TokenClass<'_> {
    #[inline]
    fn strip_key<'a>(&self, s: &'a str) -> &'a str {
        s.strip_prefix(self.key).unwrap().trim()
    }

    #[inline]
    fn split_key<'a>(&'a self, s: &'a str) -> impl Iterator<Item = &'a str> {
        s.split(self.key).map(|x| x.trim())
    }
}

/// A collection of all the possible tokens
///
/// We don't use all of these, but that's OK. Just need to have the tokens
/// defined so that we don't miss one. This is relevant because unfortunately,
/// it seems like line breaks aren't strictly necesary
/// http://pwet.fr/man/linux/fichiers_speciaux/hunspell/
///
/// If something is unused, set_parent just has to be None
///
/// Table consumes are implemented
///
/// Everything that supplies a table_consumes function will receive all tokens
/// as `s`, concatenated together.
///
/// It's macro time!
///

const TOKEN_CLASS_LIST: [TokenClass; 57] = [
    TokenClass {
        class: TokenType::Encoding,
        key: "SET",
        table_consumes: None,
        set_parent: Some(
            |tc, mut ax, s| match EncodingType::from_str(tc.strip_key(s)) {
                Some(et) => Ok(ax.encoding = et),
                None => Err("Encoding type not found"),
            },
        ),
    },
    // Boolean flag, default false
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
        set_parent: parentify!(complex_prefixes, bool),
    },
    TokenClass {
        class: TokenType::Language,
        key: "LANG",
        table_consumes: None,
        set_parent: parentify!(lang, str_replace),
    },
    TokenClass {
        class: TokenType::IgnoreChars,
        key: "IGNORE",
        table_consumes: None,
        set_parent: parentify!(ignore_chars, str_add),
    },
    TokenClass {
        class: TokenType::AffixFlag,
        key: "AF",
        table_consumes: Some(|s| re_exprs::apply_re_count(s, &re_exprs::AFFIX_FLAG_RE)),
        set_parent: None, //Some(|tc, ax, s| ax.afx_flag_vector.extend(tc.split_key(s))),
    },
    TokenClass {
        class: TokenType::MorphAlias,
        key: "AM",
        table_consumes: Some(|s| re_exprs::apply_re_count(s, &re_exprs::MORPH_ALIAS_RE)),
        set_parent: None,
    },
    TokenClass {
        class: TokenType::NeighborKeys,
        key: "KEY",
        table_consumes: None,
        set_parent: Some(|tc, ax, s| {
            // Ok this looks worse than it is, let's break it down
            // Remember we start with e.g. qwerty|asdfg|zxcb
            let mut s1 = tc
                // Remove "KEY" from the beginning, get a &str
                .strip_key(s)
                // Break this up by | into an iterator with Item=&str
                .split('|')
                // For each item, 1. remove whitespace 2. split into UTF-8
                // characters 3. collect this into a vector of &str
                .map(|x| x.trim().graphemes(true).collect::<Vec<&str>>())
                // Wind up with a vector of vectors of (unicode) strings
                .collect::<Vec<Vec<&str>>>();
            match s1.is_empty() {
                false => Ok({
                    s1.sort();
                    s1.dedup();
                    ax.keys = s1
                }),
                true => Err("No values found at field {}"),
            }
        }),
    },
    TokenClass {
        class: TokenType::TryCharacters,
        key: "TRY",
        table_consumes: None,
        set_parent: parentify!(try_characters, str_add),
    },
    TokenClass {
        class: TokenType::NoSuggestFlag,
        key: "NOSUGGEST",
        table_consumes: None,
        set_parent: parentify!(nosuggest_flag, str_replace),
    },
    TokenClass {
        class: TokenType::CompoundSuggestionsMax,
        key: "MAXCPDSUGS",
        table_consumes: None,
        set_parent: parentify!(compound_suggestions_max, int),
    },
    TokenClass {
        class: TokenType::NGramSuggestionsMax,
        key: "MAXNGRAMSUGS",
        table_consumes: None,
        set_parent: parentify!(ngram_suggestions_max, int),
    },
    TokenClass {
        class: TokenType::NGramDiffMax,
        key: "MAXDIFF",
        table_consumes: None,
        set_parent: parentify!(ngram_diff_max, int),
    },
    TokenClass {
        class: TokenType::NGramLimitToDiffMax,
        key: "ONLYMAXDIFF",
        table_consumes: None,
        set_parent: parentify!(ngram_limit_to_diff_max, bool),
    },
    TokenClass {
        class: TokenType::NoSpaceSubs,
        key: "NOSPLITSUGS",
        table_consumes: None,
        set_parent: parentify!(no_split_suggestions, bool),
    },
    TokenClass {
        class: TokenType::KeepTerminationDots,
        key: "SUGSWITHDOTS",
        table_consumes: None,
        set_parent: parentify!(keep_termination_dots, bool),
    },
    TokenClass {
        class: TokenType::Replacement,
        key: "REP",
        table_consumes: Some(|s| re_exprs::apply_re_count(s, &re_exprs::REPLACE_DEF_RE)),
        set_parent: None,
    },
    TokenClass {
        class: TokenType::Mapping,
        key: "MAP",
        table_consumes: Some(|s| re_exprs::apply_re_count(s, &re_exprs::MAP_DEF_RE)),
        set_parent: None,
    },
    TokenClass {
        class: TokenType::Phonetic,
        key: "PHONE",
        table_consumes: Some(|s| re_exprs::apply_re_count(s, &re_exprs::PHONETIC_DEF_RE)),
        set_parent: None,
    },
    TokenClass {
        class: TokenType::WarnRareFlag,
        key: "WARN",
        table_consumes: None,
        set_parent: parentify!(warn_rare_flag, str_replace),
    },
    TokenClass {
        class: TokenType::ForbitWarnWords,
        key: "FORBIDWARN",
        table_consumes: None,
        set_parent: parentify!(forbid_warn_words, bool),
    },
    TokenClass {
        class: TokenType::Breakpoint,
        key: "BREAK",
        table_consumes: Some(|s| re_exprs::apply_re_count(s, &re_exprs::BREAK_DEF_RE)),
        set_parent: None,
    },
    TokenClass {
        class: TokenType::CompoundRule,
        key: "COMPOUNDRULE",
        table_consumes: Some(|s| re_exprs::apply_re_count(s, &re_exprs::COMPOUND_RULE_DEF_RE)),
        set_parent: None,
    },
    TokenClass {
        class: TokenType::CompoundMinLength,
        key: "COMPOUNDMIN",
        table_consumes: None,
        set_parent: parentify!(compound_min_length, int),
    },
    TokenClass {
        class: TokenType::CompoundFlag,
        key: "COMPOUNDFLAG",
        table_consumes: None,
        set_parent: parentify!(compound_flag, str_replace_option),
    },
    TokenClass {
        class: TokenType::CompoundBeginFlag,
        key: "COMPOUNDBEGIN",
        table_consumes: None,
        set_parent: parentify!(compound_begin_flag, str_replace_option),
    },
    TokenClass {
        class: TokenType::CompoundEndFlag,
        key: "COMPOUNDLAST",
        table_consumes: None,
        set_parent: parentify!(compound_end_flag, str_replace_option),
    },
    TokenClass {
        class: TokenType::CompoundMiddleFlag,
        key: "COMPOUNDMIDDLE",
        table_consumes: None,
        set_parent: parentify!(compound_middle_flag, str_replace_option),
    },
    TokenClass {
        class: TokenType::CompoundOnlyFlag,
        key: "ONLYINCOMPOUND",
        table_consumes: None,
        set_parent: parentify!(compound_only_flag, str_replace_option),
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
        table_consumes: Some(|s| re_exprs::apply_re_count(s, &re_exprs::COMPOUND_PATTERN_DEF_RE)),
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
        table_consumes: Some(|s| re_exprs::apply_re_count(s, &re_exprs::PFX_DEF_RE)),
        set_parent: None,
    },
    TokenClass {
        class: TokenType::Suffix,
        key: "SFX",
        table_consumes: Some(|s| re_exprs::apply_re_count(s, &re_exprs::SFX_DEF_RE)),
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
        table_consumes: Some(|s| re_exprs::apply_re_count(s, &re_exprs::ICONV_DEF_RE)),
        set_parent: None,
    },
    TokenClass {
        class: TokenType::AffixOutputConversion,
        key: "OCONV",
        table_consumes: Some(|s| re_exprs::apply_re_count(s, &re_exprs::OCONV_DEF_RE)),
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
        class: TokenType::AffixWordChars,
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
    static ref TOKEN_KEYS:Vec<&'static str> =
        TOKEN_CLASS_LIST.into_iter().map(|x| x.key).collect();
}

pub const ENCODING_CLASS_LIST: [EncodingMatch; 9] = [
    EncodingMatch {
        class: EncodingType::Utf8,
        key: "UTF-8",
    },
    EncodingMatch {
        class: EncodingType::Iso8859t1,
        key: "ISO8859-1",
    },
    EncodingMatch {
        class: EncodingType::Iso8859t10,
        key: "ISO8859-10",
    },
    EncodingMatch {
        class: EncodingType::Iso8859t13,
        key: "ISO8859-13",
    },
    EncodingMatch {
        class: EncodingType::Iso8859t15,
        key: "ISO8859-15",
    },
    EncodingMatch {
        class: EncodingType::Koi8r,
        key: "KOI8-R",
    },
    EncodingMatch {
        class: EncodingType::Koi8u,
        key: "KOI8-U",
    },
    EncodingMatch {
        class: EncodingType::Cp1251,
        key: "cp1251",
    },
    EncodingMatch {
        class: EncodingType::IsciiDevanagari,
        key: "ISCII-DEVANAGARI",
    },
];

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
            TokenType::to_token_str(&TokenType::NoSuggestFlag),
            "NOSUGGEST"
        );
    }
}
