// //! Classes needed for affix attributes

// use super::affix_types::*;
// use crate::graph_vec;
// use unicode_segmentation::UnicodeSegmentation;

// // lines that are
// trait LineClassThing {
//     // From a string
//     fn from_str() -> Self;
// }

// pub struct ReplaceRule<'a> {
//     from: &'a str,
//     to: &'a str,
// }

// pub struct PhoneRule<'a> {
//     from: &'a str,
//     to: &'a str,
// }
// // impl ReplaceRule <'_> {
// //     fn new<'a>(from: &'a str, to: &'a str) -> ReplaceRule<'a> {
// //         ReplaceRule { from: from, to: to }
// //     }
// // }

// /// The main affix item
// ///
// /// This holds the entire contents of the affix file as an AST representation
// /// and is intended to be used throughout program lifetime.
// ///
// /// Any type that can be modified must be owned (e.g. String, Vec), others may
// /// be borrowed.
// ///
// /// IMPORTANT NOTE: we are talking about Unicode here, so a lot of the times a
// /// "character" in text and a "character" in code are not the same; a Unicode
// /// character can be up to four character codes. As this is a string processing
// /// library, we choose that "character" means a character in text, which will be
// /// comprised of one to four unsigned 8-bit integers (4-byte UTF8).
// ///
// /// With that in mind, a basic string for us is `Vec<&str>` - not `String` or
// /// `&str` than `String` that would otherwise seem to make sense. This is
// /// because we frequently work with individual characers.
// ///
// /// So, an actual vector of strings is Vec<Vec<&str>>
// pub struct Affix<'a> {
//     // We want to make sure all these items are mutable so we
//     // can append/edit later
//     /// Charset to use, reference to an EncodingType Currently this is unused;
//     /// only UTF-8 is supported. However, the affix file must still have an
//     /// accurate definition.
//     pub encoding: &'a EncodingType,

//     /// Twofold prefix skipping for e.g. right-to-left languages
//     pub complex_prefixes: bool,

//     /// Language code, currently unused. Consider this unstable as it may change
//     /// to be an object reference.
//     pub lang: &'a str,

//     /// List of characters to ignore
//     pub ignore_chars: Vec<&'a str>,

//     /// List of usable flag vectors. Defaults to all things after "/"" in a dict.
//     pub afx_flag_vector: Vec<&'a str>,

//     /// ## Suggestion-related items
//     // List of e.g. "qwerty", "asdfg" that define neighbors
//     pub keys: Vec<Vec<&'a str>>,

//     /// Suggest words that differe by 1 try character
//     pub try_characters: Vec<&'a str>,

//     /// Flag used to indicate words that should not be suggested
//     pub nosuggest_flag: &'a str,

//     /// Maximum compound word suggestions
//     pub compound_suggestions_max: u8,

//     /// Max number of ngram suggestions
//     pub ngram_suggestions_max: u8,

//     /// N-gram similarity limit
//     pub ngram_diff_max: u8,

//     /// Remove all suggestions except the diff max
//     pub ngram_limit_to_diff_max: bool,

//     /// Don't suggest anything with spaces
//     pub no_split_suggestions: bool,

//     /// If a dot comes with the spellcheck, return one with a suggestion word
//     pub keep_termination_dots: bool,

//     /// Note rare (i.e. commonly misspelled) words with this flag
//     pub warn_rare_flag: &'a str,

//     /// Whether to never suggest words with the warn flag (above)
//     pub forbid_warn_words: bool,

//     pub replacements: Vec<&'a ReplaceRule<'a>>,
//     // maps: Vec<>, // MAP
//     // phones: Vec<>
//     /// ## Compounding-related items
//     // break_points: Vec<>
//     // compound_rules: Vec<>

//     /// Minimum length of words used in a compound
//     pub compound_min_length: u8,

//     /// Words with this flag may be in compounds
//     pub compound_flag: Option<&'a str>,

//     /// Words with this flag may start a compound
//     pub compound_begin_flag: Option<&'a str>,

//     /// Words with this flag may end a compound
//     pub compound_end_flag: Option<&'a str>,

//     /// Words with this flag may be in the middle of a compound
//     pub compound_middle_flag: Option<&'a str>,

//     /// Words with this flag can't be on their own, only in compounds
//     pub compound_only_flag: Option<&'a str>,
//     // There are lots of compound flags that haven't yet been implemented

//     // ## Affix-related items

//     // Rules for setting prefixes and suffixes
//     // affix_rules: Vec<>
// }

// impl Affix<'_> {
//     /// Create an empty affix object
//     fn new() -> Affix<'static> {
//         Affix {
//             encoding: &EncodingType::Utf8,
//             complex_prefixes: false,
//             ignore_chars: Vec::new(),
//             afx_flag_vector: Vec::new(),
//             keys: vec![
//                 graph_vec!("qwertyuiop"),
//                 graph_vec!("asdfghjkl"),
//                 graph_vec!("zxcvbnm"),
//             ],
//             try_characters: graph_vec!("esianrtolcdugmphbyfvkwzESIANRTOLCDUGMPHBYFVKWZ'"),
//             nosuggest_flag: "!",
//             compound_suggestions_max: 2,
//             ngram_suggestions_max: 2,
//             ngram_diff_max: 5,
//             ngram_limit_to_diff_max: true,
//             keep_termination_dots: false,
//             no_split_suggestions: false,
//             warn_rare_flag: "",
//             forbid_warn_words: false,
//             replacements: Vec::new(),
//             lang: "",
//             compound_min_length: 3,
//             compound_flag: None,
//             compound_begin_flag: None,
//             compound_end_flag: None,
//             compound_middle_flag: None,
//             compound_only_flag: None,
//         }
//     }

//     fn load_from_str(&mut self, s: &str) {
//         let mut working_token = String::new();
//         // Whether we are operating as normal or in a comment
//         let mut accumulating = true;

//         for c in s.chars() {
//             // Stop accoumulating on comment until we hit a newline
//             if c == '#' {
//                 accumulating = false;
//                 continue;
//             }
//             if !accumulating && c == '\n' {
//                 accumulating = true;
//             } else if !accumulating {
//                 continue;
//             }
//             // If table_consumes,

//             working_token.push(c);
//             // if current_token
//         }
//     }

//     // /// Accept a list of keys
//     // fn load_keys_list<T:IntoIterator<Item=str>>(&mut self, keys: T) ->Result<(), &str>{

//     // }

//     // fn load_keys_aff(&mut self, line: &str) ->Result<(), &str>{

//     //     self.load_keys_list()
//     // }
// }
