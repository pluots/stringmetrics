//! Classes needed for affix attributes
use crate::spellcheck::affix_tokens::*;
use regex::Regex;

// const CONTROL_ITEMS  = [
// "SET",
// "FLAG",
// "COMPLEXPREFIXES",
// "LANG",
// "IGNORE",
// "AF",
// "AM",
// "KEY",
// "TRY",
// "NOSUGGEST",
// "MAXCPDSUGS",
// "MAXNGRAMSUGS",
// "MAXDIFF",
// "ONLYMAXDIFF",
// "NOSPLITSUGS",
// ];

fn build_ast(s: &str) {}

#[derive(PartialEq, Debug)]
pub enum Encoding {
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

/// match encoding from e.g. "SET UTF-8"
#[inline]
pub fn match_encoding(s: &str) -> Result<Encoding, &str> {
    let mut split = s.split_whitespace();

    match split.next() {
        Some(val) => {
            if val != "SET" {
                return Err("Doesn't appear to be a \"SET\"");
            }
        }
        None => return Err("No text found"),
    }

    let name = match split.next() {
        Some(val) => val.trim(),
        None => return Err("No encoding listed after \"SET\""),
    };

    match name {
        "UTF-8" => Ok(Encoding::Utf8),
        "ISO8859-1" => Ok(Encoding::Iso8859t1),
        "ISO8859-10" => Ok(Encoding::Iso8859t10),
        "ISO8859-13" => Ok(Encoding::Iso8859t13),
        "ISO8859-15" => Ok(Encoding::Iso8859t15),
        "KOI8-R" => Ok(Encoding::Koi8r),
        "KOI8-U" => Ok(Encoding::Koi8u),
        "cp1251" => Ok(Encoding::Cp1251),
        "ISCII-DEVANAGARI" => Ok(Encoding::IsciiDevanagari),
        _ => Err("Encoding not found"),
    }
}

mod parsers {}

// lines that are
trait LineClassThing {
    // From a string
    fn from_str() -> Self;
}

struct ReplaceRule<'a> {
    from: &'a str,
    to: &'a str,
}

struct PhoneRule<'a> {
    from: &'a str,
    to: &'a str,
}
// impl ReplaceRule <'_> {
//     fn new<'a>(from: &'a str, to: &'a str) -> ReplaceRule<'a> {
//         ReplaceRule { from: from, to: to }
//     }
// }

/// Main thingy
pub struct Affix<'a> {
    // We want to make sure all these items are mutable so we
    // can append/edit later

    // Charset to use
    encoding: Encoding,
    // Flag for right-to-left languages
    complex_prefixes: bool,
    // list of characters to ignore
    ignore: String,
    // List of usable flag vectors. Defaults to all things after "/"" in a dict.
    flag_vector: Vec<&'a str>,

    // Suggestion-related items //

    // List of e.g. "qwerty", "asdfg" that define neighbors
    keys: Vec<&'a str>,

    // TRY xyz
    // Suggest words that differe by 1 try character
    try_characters: String,
    // NOSUGGEST x
    // Flag indicating words should be accepted but not suggested
    nosuggest_flag: String,

    // MAXCPDSUGS
    max_compound_suggestions: i8,

    // MAXNGRAMSUGS
    max_ngram_suggestions: i8,

    // MAXDIFF [0-10]
    max_diff: i8,

    // ONLYMAXDIFF

    // NOSPLITSUGS
    // Don't suggest words with spaces
    no_split_suggestions: bool,

    // Add dot to suggestion if word terminates with one
    suggest_with_dots: bool, // SUGSWITHDOTS

    replacements: Vec<&'a ReplaceRule<'a>>, // REP

                                            // maps: Vec<>, // MAP

                                            // phones: Vec<>
                                            // Compounding opts
}

impl Affix<'_> {
    /// Create an empty affix object
    fn new() -> Affix<'static> {
        Affix {
            encoding: Encoding::Utf8,
            complex_prefixes: false,
            ignore: String::new(),
            flag_vector: Vec::new(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_encoding() {
        assert_eq!(match_encoding("SET UTF-8").unwrap(), Encoding::Utf8);
        assert_eq!(
            match_encoding("SET ISO8859-1").unwrap(),
            Encoding::Iso8859t1
        );
        assert_eq!(
            match_encoding("SET ISO8859-10").unwrap(),
            Encoding::Iso8859t10
        );
        assert_eq!(
            match_encoding("SET ISO8859-13").unwrap(),
            Encoding::Iso8859t13
        );
        assert_eq!(
            match_encoding("SET ISO8859-15").unwrap(),
            Encoding::Iso8859t15
        );
        assert_eq!(match_encoding("SET KOI8-R").unwrap(), Encoding::Koi8r);
        assert_eq!(match_encoding("SET KOI8-U").unwrap(), Encoding::Koi8u);
        assert_eq!(match_encoding("SET cp1251").unwrap(), Encoding::Cp1251);
        assert_eq!(
            match_encoding("SET ISCII-DEVANAGARI").unwrap(),
            Encoding::IsciiDevanagari
        );

        // Test whitespace differences
        assert_eq!(match_encoding(" SET\tUTF-8 ").unwrap(), Encoding::Utf8);
    }

    #[test]
    fn test_match_encoding_errors() {
        assert_eq!(
            match_encoding("SET").unwrap_err(),
            "No encoding listed after \"SET\""
        );
        assert_eq!(
            match_encoding("SET iojsdoifjodi").unwrap_err(),
            "Encoding not found"
        );
        assert_eq!(
            match_encoding("not SET").unwrap_err(),
            "Doesn't appear to be a \"SET\""
        );
        assert_eq!(match_encoding("").unwrap_err(), "No text found");
    }
}
