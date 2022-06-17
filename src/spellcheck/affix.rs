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
/// This holds the entire contents of the affix file, in AST format
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

    // TRY xyz
    // Suggest words that differe by 1 try character
    pub try_characters: String,
    // NOSUGGEST x
    // Flag indicating words should be accepted but not suggested
    pub nosuggest_flag: String,

    // MAXCPDSUGS
    pub max_compound_suggestions: i8,

    // MAXNGRAMSUGS
    pub max_ngram_suggestions: i8,

    // MAXDIFF [0-10]
    pub max_diff: i8,

    // ONLYMAXDIFF

    // NOSPLITSUGS
    // Don't suggest words with spaces
    pub no_split_suggestions: bool,

    // Add dot to suggestion if word terminates with one
    pub suggest_with_dots: bool, // SUGSWITHDOTS

    pub replacements: Vec<&'a ReplaceRule<'a>>, // REP

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
            nosuggest_flag: "!".to_string(),
            max_compound_suggestions: 2,
            max_ngram_suggestions: 2,
            max_diff: 5,
            no_split_suggestions: false,
            suggest_with_dots: false,
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
