//! Classes needed for affix attributes

// use super::affix_types::*;
use crate::graph_vec;
use crate::spellcheck::affix_types::EncodingType;
use unicode_segmentation::UnicodeSegmentation;

use super::affix_serde::load_affix_from_str;

/// The main affix item
///
/// This holds the entire contents of the affix file as an AST representation
/// and is intended to be used throughout program lifetime.
///
/// Any type that can be modified must be owned (e.g. String, Vec), others may
/// be borrowed.
///
/// IMPORTANT NOTE: we are talking about Unicode here, so a lot of the times a
/// "character" in text and a "character" in code are not the same; a Unicode
/// character can be up to four character codes. As this is a string processing
/// library, we choose that "character" means a character in text, which will be
/// comprised of one to four unsigned 8-bit integers (4-byte UTF8).
///
/// With that in mind, a basic string for us is `Vec<&str>` - not `String` or
/// `&str` than `String` that would otherwise seem to make sense. This is
/// because we frequently work with individual characers.
///
/// So, an actual vector of strings is Vec<Vec<&str>>
#[derive(Debug, PartialEq)]
pub struct Affix {
    // We want to make sure all these items are mutable so we
    // can append/edit later
    /// Charset to use, reference to an EncodingType Currently this is unused;
    /// only UTF-8 is supported. However, the affix file must still have an
    /// accurate definition.
    pub encoding: EncodingType,

    /// Twofold prefix skipping for e.g. right-to-left languages
    pub complex_prefixes: bool,

    /// Language code, currently unused. Consider this unstable as it may change
    /// to be an object reference.
    pub lang: String,

    /// List of characters to ignore
    pub ignore_chars: Vec<String>,

    /// List of usable flag vectors. Defaults to all things after "/"" in a dict.
    pub afx_flag_vector: Vec<String>,

    /// ## Suggestion-related items
    // List of e.g. "qwerty", "asdfg" that define neighbors
    pub keys: Vec<Vec<String>>,

    /// Suggest words that differe by 1 try character
    pub try_characters: Vec<String>,

    /// Flag used to indicate words that should not be suggested
    pub nosuggest_flag: String,

    /// Maximum compound word suggestions
    pub compound_suggestions_max: u8,

    /// Max number of ngram suggestions
    pub ngram_suggestions_max: u8,

    /// N-gram similarity limit
    pub ngram_diff_max: u8,

    /// Remove all suggestions except the diff max
    pub ngram_limit_to_diff_max: bool,

    /// Don't suggest anything with spaces
    pub no_split_suggestions: bool,

    /// If a dot comes with the spellcheck, return one with a suggestion word
    pub keep_termination_dots: bool,

    /// Note rare (i.e. commonly misspelled) words with this flag
    pub warn_rare_flag: String,

    /// Whether to never suggest words with the warn flag (above)
    pub forbid_warn_words: bool,

    // pub replacements: Vec<&'a ReplaceRule<'a>>,
    // maps: Vec<>, // MAP
    // phones: Vec<>
    /// ## Compounding-related items
    // break_points: Vec<>
    // compound_rules: Vec<>

    /// Minimum length of words used in a compound
    pub compound_min_length: u8,

    /// Words with this flag may be in compounds
    pub compound_flag: Option<String>,

    /// Words with this flag may start a compound
    pub compound_begin_flag: Option<String>,

    /// Words with this flag may end a compound
    pub compound_end_flag: Option<String>,

    /// Words with this flag may be in the middle of a compound
    pub compound_middle_flag: Option<String>,

    /// Words with this flag can't be on their own, only in compounds
    pub compound_only_flag: Option<String>,
    // There are lots of compound flags that haven't yet been implemented

    // ## Affix-related items

    // Rules for setting prefixes and suffixes
    // affix_rules: Vec<>
}

impl Affix {
    /// Create an empty affix object
    pub fn new() -> Affix {
        Affix {
            encoding: EncodingType::Utf8,
            complex_prefixes: false,
            lang: String::new(),
            ignore_chars: Vec::new(),
            afx_flag_vector: Vec::new(),
            keys: vec![
                graph_vec!("qwertyuiop"),
                graph_vec!("asdfghjkl"),
                graph_vec!("zxcvbnm"),
            ],
            try_characters: graph_vec!("esianrtolcdugmphbyfvkwzESIANRTOLCDUGMPHBYFVKWZ'"),
            nosuggest_flag: String::from("!"),
            compound_suggestions_max: 2,
            ngram_suggestions_max: 2,
            ngram_diff_max: 5,
            ngram_limit_to_diff_max: false,
            no_split_suggestions: false,
            keep_termination_dots: false,
            warn_rare_flag: String::new(),
            forbid_warn_words: false,
            compound_min_length: 3,
            compound_flag: None,
            compound_begin_flag: None,
            compound_end_flag: None,
            compound_middle_flag: None,
            compound_only_flag: None,
        }
    }

    /// Load this affix from a string, i.e. one loaded from an affix file
    pub fn load_from_str(&mut self, s: &str) -> Result<(), String> {
        load_affix_from_str(self, s)
    }
}

impl Default for Affix {
    fn default() -> Self {
        Affix::new()
    }
}
