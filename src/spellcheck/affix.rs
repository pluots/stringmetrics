//! Classes needed for affix attributes

use super::affix_tokens::*;
use super::affix_types::*;

// fn build_ast(s: &str) {}

// lines that are
trait LineClassThing {
    // From a string
    fn from_str() -> Self;
}

pub struct ReplaceRule<'a> {
    from: &'a str,
    to: &'a str,
}

pub struct PhoneRule<'a> {
    from: &'a str,
    to: &'a str,
}
// impl ReplaceRule <'_> {
//     fn new<'a>(from: &'a str, to: &'a str) -> ReplaceRule<'a> {
//         ReplaceRule { from: from, to: to }
//     }
// }

/// The main affix item
///
/// This holds the entire contents of the affix file as an AST representation
/// and is intended to be used throughout program lifetime.
pub struct Affix<'a> {
    // We want to make sure all these items are mutable so we
    // can append/edit later

    // Charset to use
    pub encoding: &'a EncodingType,
    // Flag for right-to-left languages
    pub complex_prefixes: bool,
    // pub language: SomeType,
    // list of characters to ignore
    pub ignore_chars: String,
    // List of usable flag vectors. Defaults to all things after "/"" in a dict.
    pub afx_flag_vector: Vec<&'a str>,

    // Suggestion-related items //

    // List of e.g. "qwerty", "asdfg" that define neighbors
    pub keys: Vec<&'a str>,

    /// Suggest words that differe by 1 try character
    pub try_characters: String,
    pub nosuggest_flag: &'a str,
    pub compound_suggestions_max: i8,
    pub ngram_suggestions_max: i8,
    pub ngram_diff_max: i8,
    pub ngram_limit_to_diff_max: bool,
    pub no_split_suggestions: bool,
    pub keep_termination_dots: bool,
    pub warn_rare_flag: &'a str,
    pub forbid_warn_words: bool,

    pub replacements: Vec<&'a ReplaceRule<'a>>,
    // maps: Vec<>, // MAP
    // phones: Vec<>
    // Compounding opts
}

impl Affix<'_> {
    /// Create an empty affix object
    fn new() -> Affix<'static> {
        Affix {
            encoding: &EncodingType::Utf8,
            complex_prefixes: false,
            ignore_chars: String::new(),
            afx_flag_vector: Vec::new(),
            keys: vec!["qwertyuiop", "asdfghjkl", "zxcvbnm"],
            try_characters: "esianrtolcdugmphbyfvkwzESIANRTOLCDUGMPHBYFVKWZ'".to_string(),
            nosuggest_flag: "!",
            compound_suggestions_max: 2,
            ngram_suggestions_max: 2,
            ngram_diff_max: 5,
            ngram_limit_to_diff_max: true,
            keep_termination_dots: false,
            no_split_suggestions: false,
            warn_rare_flag
            forbid_warn_words:bool=false,
            replacements: Vec::new(),
        }
    }

    fn load_from_str(&mut self, s: &str) {
        let mut working_token = String::new();
        // Whether we are operating as normal or in a comment
        let mut accumulating = true;

        for c in s.chars() {
            // Stop accoumulating on comment until we hit a newline
            if c == '#' {
                accumulating = false;
                continue;
            }
            if !accumulating && c == '\n' {
                accumulating = true;
            } else if !accumulating {
                continue;
            }
            // If table_consumes,

            working_token.push(c);
            // if current_token
        }
    }

    // /// Accept a list of keys
    // fn load_keys_list<T:IntoIterator<Item=str>>(&mut self, keys: T) ->Result<(), &str>{

    // }

    // fn load_keys_aff(&mut self, line: &str) ->Result<(), &str>{

    //     self.load_keys_list()
    // }
}
