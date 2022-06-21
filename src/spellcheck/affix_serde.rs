use super::affix::Affix;
use super::affix_types::{EncodingType, TokenType};
use strum::{EnumProperty, VariantNames};

/// Populate an Affix class from the string version of a file
pub fn load_affix_from_str(_ax: &mut Affix, s: &str) -> () {
    // This will give us a list of tokens with no processing applied
    let raw_stripped = strip_comments(s);
    let raw_tokens = create_raw_tokens(raw_stripped.as_str());
    let processed_tokens = create_processed_tokens(raw_tokens);
}

/// Strip "#" comments from a &str. Breaks from a found "#" to the next newline;
/// does not remove the newlines
fn strip_comments(s: &str) -> String {
    let mut newstr = String::new();
    let mut paused = false;
    // Logic to skip from # to newline
    for ch in s.chars() {
        if paused && ch == '\n' {
            paused = false;
        } else if paused {
            continue;
        } else if ch == '#' {
            paused = true;
            continue;
        }
        newstr.push(ch);
    }
    newstr
}

/// A token for a specific affix option
/// Holds a type and the stream of words that follow
/// Words have been split on whitespace
#[derive(Debug, PartialEq)]
struct AffixRawToken<'a> {
    pub ttype: TokenType,
    pub content: Vec<&'a str>,
}

impl AffixRawToken<'_> {
    pub fn new<'a>(ttype: TokenType, content: Vec<&'a str>) -> AffixRawToken<'a> {
        AffixRawToken {
            ttype: ttype,
            content: content,
        }
    }
}

/// Create a list of tokens with matched type and string content; do not do
/// anything to analyze them
fn create_raw_tokens<'a>(s: &'a str) -> Vec<AffixRawToken> {
    // Temporarially hold the "next vector" rather than working one, blank until
    // needed
    let mut working_vec: Vec<&'a str> = Vec::new();
    let mut working_type = TokenType::FileStart;
    // let mut working_t: OptionToken<'a> = OptionToken::new(TokenType::FileStart, Vec::new());

    let mut ret: Vec<AffixRawToken> = Vec::new();

    // Each token is a "word", split by UTF-8 boundaries
    for word in s.split_whitespace() {
        if TokenType::VARIANTS.contains(&word) {
            // Push this token and start a new one
            ret.push(AffixRawToken::new(working_type, working_vec));
            working_vec = Vec::new();
            working_type = TokenType::try_from(word).unwrap();
        } else {
            // Not a delimiting key; just add this to our current token
            working_vec.push(word);
        }
    }

    ret.push(AffixRawToken::new(working_type, working_vec));

    ret
}

enum ProcessedTokenData<'a> {
    Bool(bool),
    String(Vec<&'a str>),
    Table(Vec<Vec<&'a str>>),
}

struct AffixProcessedToken<'a> {
    pub ttype: TokenType,
    pub data: ProcessedTokenData<'a>,
}

/// Use the first raw token to determine how many more to read into the table
/// Returns a u16 if successful, error otherwise
fn get_table_item_count(token: &AffixRawToken) -> Result<u16, String> {
    let temp = match token.ttype {
        // AF [n]
        TokenType::AffixFlag => token.content.get(1),
        // AM [n]
        TokenType::MorphAlias => token.content.get(1),
        // REP [n]
        TokenType::Replacement => token.content.get(1),
        // MAP [n]
        TokenType::Mapping => token.content.get(1),
        // PHONE [n]
        TokenType::Phonetic => token.content.get(1),
        // BREAK [n]
        TokenType::Breakpoint => token.content.get(1),
        // COMPOUNDRULE [n]
        TokenType::CompoundRule => token.content.get(1),
        // CHECKCOMPOUNDPATTERN [n]
        TokenType::CompoundForbidPatterns => token.content.get(1),
        // PFX flag cross_product number
        TokenType::Prefix => token.content.get(3),
        // SFX flag cross_product number
        TokenType::Suffix => token.content.get(3),
        // ICONV [n]
        TokenType::AffixInputConversion => token.content.get(1),
        // OCONV [n]
        TokenType::AffixOutputConversion => token.content.get(1),
        // Any tokens types that don't have a table
        _ => return Ok(0),
    };

    match temp {
        Some(num) => match num.parse() {
            Ok(val) => Ok(val),
            Err(_) => Err(format!("Bad number at {}", token.ttype).into()),
        },
        None => Err(format!("Incorrect syntax at {}", token.ttype).into()),
    }
}

/// Loop through a vector of raw tokens and create the processed version
///
fn create_processed_tokens(tokens: Vec<AffixRawToken>) 
-> Result<Vec<AffixProcessedToken>, String> {
    // Vector to hold what we will return
    let mut retvec = Vec::new();
    // If we need to accumulate values for a table, use these items
    let mut table_accum_count = 0u16;
    let mut table_accum_type = TokenType::FileStart; // FileStart is kind of a dummy token
    let mut table_accum_vec = Vec::new();

    for token in tokens {
        // Check accumulate logic first

        if table_accum_count > 0 {
            // If we ar eaccumulating, just push this token and go on to the next
            // one to our temp working vector
            // Validate token type first
            if token.ttype != table_accum_type {
                return Err(format!(
                    "Token of type {} did not match {} before \
                    table ended. Expected {} more token.",
                    token.ttype, table_accum_type, table_accum_count
                ));
            }

            table_accum_vec.push(token.content);
            table_accum_count -= 1;
            if table_accum_count > 0 {
                continue;
            }

            // If we are here, that means we are on our last token.
            // Finalie it to the return vec
            retvec.push(AffixProcessedToken {
                ttype: token.ttype,
                data: ProcessedTokenData::Table(table_accum_vec),
            });
            // And reset our status
            table_accum_vec = Vec::new();
            continue
        }

        // If we're not accumulating, just match based on token type
        match token.ttype.get_str("dtype").unwrap() {
            // String or bool are straightforward
            "str" => retvec.push(AffixProcessedToken {
                ttype: token.ttype,
                data: ProcessedTokenData::String(token.content),
            }),
            "bool" => retvec.push(AffixProcessedToken {
                ttype: token.ttype,
                data: ProcessedTokenData::Bool(true),
            }),
            // For table - figure out item count, push this token,
            "table" => {
                table_accum_count = match get_table_item_count(&token) {
                    Ok(val) => val,
                    Err(s) => return Err(s),
                };
                table_accum_vec.push(token.content);
                table_accum_type = token.ttype;
            }
            _ => panic!("Unexpected token type"),
        }
    }

    Ok(retvec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoding_deser() {
        let s = "abc\ndef\n# comment\nline with # comment\nendline";
        assert_eq!(strip_comments(s), "abc\ndef\n\nline with \nendline");
    }

    #[test]
    fn test_create_tokens() {
        let s = "SYLLABLENUM a b c PFX prefix CIRCUMFIX COMPOUNDLAST";
        let test_vec = vec![
            AffixRawToken::new(TokenType::FileStart, vec![]),
            AffixRawToken::new(TokenType::CompoundSyllableNumber, vec!["a", "b", "c"]),
            AffixRawToken::new(TokenType::Prefix, vec!["prefix"]),
            AffixRawToken::new(TokenType::AffixCircumfixFlag, vec![]),
            AffixRawToken::new(TokenType::CompoundEndFlag, vec![]),
        ];
        assert_eq!(create_raw_tokens(s), test_vec);
    }
}

// //! Affix tokens
// //!
// //! This module is used for things related to parsing from an affix file. It
// //! contains a lot of ugly syntax and wild macros to attempt to minimize
// //! boilerplate

// use super::affix_types::{EncodingType, TokenType};
// use super::affix::Affix;
// use lazy_static::lazy_static;
// use unicode_segmentation::UnicodeSegmentation;

// /// Macro that creates a closure based on the relevant type, used for
// /// TokenClass::set_parent
// ///
// /// Macros so powerful they should be illegal
// ///
// /// All of our finctions return Option<Callable -> Result>
// ///
// /// Usage:
// /// `parentify!(field_name, bool)` will just set the relevant field true
// ///
// macro_rules! parentify {
//     // Boolean field just assigns true and returns Ok (Flag is just either there
//     // or not)
//     ( $field:ident, bool ) => {
//         Some(|_, mut ax, _| Ok(ax.$field = true))
//     };

//     // Integer fields are a bit more complicated
//     // Need to parse the field if possible,
//     ( $field:ident, int ) => {
//         Some(|tc, ax, s| match tc.strip_key(s).parse() {
//             Ok(v) => Ok(ax.$field = v),
//             Err(_) => Err("Bad integer value at {}"),
//         })
//     };

//     // Use str_replace for any time we have `&str` as the `Affix` type. We will
//     // replace what exists with our new value
//     ( $field:ident, str_replace ) => {
//         Some(|tc, ax, s| {
//             let s1 = tc.strip_key(s);
//             match s1.is_empty() {
//                 false => Ok(ax.$field = s1),
//                 true => Err("No values found at field {}"),
//             }
//         })
//     };

//     // Same as above but place the result in an option
//     ( $field:ident, str_replace_option ) => {
//         Some(|tc, ax, s| {
//             let s1 = tc.strip_key(s);
//             match s1.is_empty() {
//                 false => Ok(ax.$field = Some(s1)),
//                 true => Err("No values found at field {}"),
//             }
//         })
//     };

//     // Use str_add any time we have a `String` that we want to append to.
//     // Basically the same as above except we append to the existing vector and
//     // sort rather than replacing what's there. Usable for `Vec<&str>`.
//     ( $field:ident, str_add ) => {
//         Some(|tc, ax, s| {
//             let s1 = tc.strip_key(s).to_string();
//             match s1.is_empty() {
//                 false => Ok({
//                     let mut tmp = s.graphemes(true).collect::<Vec<&str>>();
//                     tmp.sort();
//                     tmp.dedup();
//                     ax.$field = tmp
//                 }),
//                 true => Err("No values found at field {}"),
//             }
//         })
//     };
// }

// /// A structure holding information about a token and how to use it
// ///
// /// This is meant for internal use in parsing the file Note that in parsing via
// /// set_parent, APPEND mode is used wherever possible. Make sure if you come
// /// start with a pre-populated Affix class, you clear the relevant fields first.
//  struct TokenMatch<'a> {
//     // Kind of the token
//     class: TokenType,
//     // Token's name in the dict
//      key: &'a str,
//     // A function that takes in the token's string and determines
//     // how many of the following tokens to consume. If none, not a table.
//      table_consumes: Option<fn(s: &str) -> u16>,
//     // Set the parent when passed the foll text token
//     // Idiomatic fn(self, parent, str)
//     // Returns a result for nice error handling
//     // Use the macro above to make setting this easy
//      set_parent: Option<fn(&TokenMatch, &mut Affix, &'a str) -> Result<(), &'static str>>,
// }

// impl TokenMatch<'_> {
//     #[inline]
//     pub fn strip_key<'a>(&self, s: &'a str) -> &'a str {
//         s.strip_prefix(self.key).unwrap().trim()
//     }

//     #[inline]
//     pub fn split_key<'a>(&'a self, s: &'a str) -> impl Iterator<Item = &'a str> {
//         s.split(self.key).map(|x| x.trim())
//     }
// }

// lazy_static! {
//     /// All possible keys collected into a vector
//     static ref TOKEN_KEYS:Vec<&'static str> =
//         TOKEN_CLASS_LIST.into_iter().map(|x| x.key).collect();
// }

// /// A collection of all the possible tokens
// ///
// /// We don't use all of these, but that's OK. Just need to have the tokens
// /// defined so that we don't miss one. This is relevant because unfortunately,
// /// it seems like line breaks aren't strictly necesary
// /// http://pwet.fr/man/linux/fichiers_speciaux/hunspell/
// ///
// /// If something is unused, set_parent just has to be None
// ///
// /// Table consumes are implemented
// ///
// /// Everything that supplies a table_consumes function will receive all tokens
// /// as `s`, concatenated together.
// pub const TOKEN_CLASS_LIST: [TokenMatch; 57] = [
//     TokenMatch {
//         class: TokenType::Encoding,
//         key: "SET",
//         table_consumes: None,
//         set_parent: Some(
//             |tc, mut ax, s| match EncodingType::from_str(tc.strip_key(s)) {
//                 Some(et) => Ok(ax.encoding = et),
//                 None => Err("Encoding type not found"),
//             },
//         ),
//     },
//     // Boolean flag, default false
//     TokenMatch {
//         class: TokenType::FlagType,
//         key: "FLAG",
//         table_consumes: None,
//         set_parent: None,
//     },
//     TokenMatch {
//         class: TokenType::ComplexPrefixes,
//         key: "COMPLEXPREFIXES",
//         table_consumes: None,
//         set_parent: parentify!(complex_prefixes, bool),
//     },
//     TokenMatch {
//         class: TokenType::Language,
//         key: "LANG",
//         table_consumes: None,
//         set_parent: parentify!(lang, str_replace),
//     },
//     TokenMatch {
//         class: TokenType::IgnoreChars,
//         key: "IGNORE",
//         table_consumes: None,
//         set_parent: parentify!(ignore_chars, str_add),
//     },
//     TokenMatch {
//         class: TokenType::AffixFlag,
//         key: "AF",
//         table_consumes: Some(|s| re_exprs::apply_re_count(s, &re_exprs::AFFIX_FLAG_RE)),
//         set_parent: None, //Some(|tc, ax, s| ax.afx_flag_vector.extend(tc.split_key(s))),
//     },
//     TokenMatch {
//         class: TokenType::MorphAlias,
//         key: "AM",
//         table_consumes: Some(|s| re_exprs::apply_re_count(s, &re_exprs::MORPH_ALIAS_RE)),
//         set_parent: None,
//     },
//     TokenMatch {
//         class: TokenType::NeighborKeys,
//         key: "KEY",
//         table_consumes: None,
//         set_parent: Some(|tc, ax, s| {
//             // Ok this looks worse than it is, let's break it down
//             // Remember we start with e.g. qwerty|asdfg|zxcb
//             let mut s1 = tc
//                 // Remove "KEY" from the beginning, get a &str
//                 .strip_key(s)
//                 // Break this up by | into an iterator with Item=&str
//                 .split('|')
//                 // For each item, 1. remove whitespace 2. split into UTF-8
//                 // characters 3. collect this into a vector of &str
//                 .map(|x| x.trim().graphemes(true).collect::<Vec<&str>>())
//                 // Wind up with a vector of vectors of (unicode) strings
//                 .collect::<Vec<Vec<&str>>>();
//             match s1.is_empty() {
//                 false => Ok({
//                     s1.sort();
//                     s1.dedup();
//                     ax.keys = s1
//                 }),
//                 true => Err("No values found at field {}"),
//             }
//         }),
//     },
//     TokenMatch {
//         class: TokenType::TryCharacters,
//         key: "TRY",
//         table_consumes: None,
//         set_parent: parentify!(try_characters, str_add),
//     },
//     TokenMatch {
//         class: TokenType::NoSuggestFlag,
//         key: "NOSUGGEST",
//         table_consumes: None,
//         set_parent: parentify!(nosuggest_flag, str_replace),
//     },
//     TokenMatch {
//         class: TokenType::CompoundSuggestionsMax,
//         key: "MAXCPDSUGS",
//         table_consumes: None,
//         set_parent: parentify!(compound_suggestions_max, int),
//     },
//     TokenMatch {
//         class: TokenType::NGramSuggestionsMax,
//         key: "MAXNGRAMSUGS",
//         table_consumes: None,
//         set_parent: parentify!(ngram_suggestions_max, int),
//     },
//     TokenMatch {
//         class: TokenType::NGramDiffMax,
//         key: "MAXDIFF",
//         table_consumes: None,
//         set_parent: parentify!(ngram_diff_max, int),
//     },
//     TokenMatch {
//         class: TokenType::NGramLimitToDiffMax,
//         key: "ONLYMAXDIFF",
//         table_consumes: None,
//         set_parent: parentify!(ngram_limit_to_diff_max, bool),
//     },
//     TokenMatch {
//         class: TokenType::NoSpaceSubs,
//         key: "NOSPLITSUGS",
//         table_consumes: None,
//         set_parent: parentify!(no_split_suggestions, bool),
//     },
//     TokenMatch {
//         class: TokenType::KeepTerminationDots,
//         key: "SUGSWITHDOTS",
//         table_consumes: None,
//         set_parent: parentify!(keep_termination_dots, bool),
//     },
//     TokenMatch {
//         class: TokenType::Replacement,
//         key: "REP",
//         table_consumes: Some(|s| re_exprs::apply_re_count(s, &re_exprs::REPLACE_DEF_RE)),
//         set_parent: None,
//     },
//     TokenMatch {
//         class: TokenType::Mapping,
//         key: "MAP",
//         table_consumes: Some(|s| re_exprs::apply_re_count(s, &re_exprs::MAP_DEF_RE)),
//         set_parent: None,
//     },
//     TokenMatch {
//         class: TokenType::Phonetic,
//         key: "PHONE",
//         table_consumes: Some(|s| re_exprs::apply_re_count(s, &re_exprs::PHONETIC_DEF_RE)),
//         set_parent: None,
//     },
//     TokenMatch {
//         class: TokenType::WarnRareFlag,
//         key: "WARN",
//         table_consumes: None,
//         set_parent: parentify!(warn_rare_flag, str_replace),
//     },
//     TokenMatch {
//         class: TokenType::ForbitWarnWords,
//         key: "FORBIDWARN",
//         table_consumes: None,
//         set_parent: parentify!(forbid_warn_words, bool),
//     },
//     TokenMatch {
//         class: TokenType::Breakpoint,
//         key: "BREAK",
//         table_consumes: Some(|s| re_exprs::apply_re_count(s, &re_exprs::BREAK_DEF_RE)),
//         set_parent: None,
//     },
//     TokenMatch {
//         class: TokenType::CompoundRule,
//         key: "COMPOUNDRULE",
//         table_consumes: Some(|s| re_exprs::apply_re_count(s, &re_exprs::COMPOUND_RULE_DEF_RE)),
//         set_parent: None,
//     },
//     TokenMatch {
//         class: TokenType::CompoundMinLength,
//         key: "COMPOUNDMIN",
//         table_consumes: None,
//         set_parent: parentify!(compound_min_length, int),
//     },
//     TokenMatch {
//         class: TokenType::CompoundFlag,
//         key: "COMPOUNDFLAG",
//         table_consumes: None,
//         set_parent: parentify!(compound_flag, str_replace_option),
//     },
//     TokenMatch {
//         class: TokenType::CompoundBeginFlag,
//         key: "COMPOUNDBEGIN",
//         table_consumes: None,
//         set_parent: parentify!(compound_begin_flag, str_replace_option),
//     },
//     TokenMatch {
//         class: TokenType::CompoundEndFlag,
//         key: "COMPOUNDLAST",
//         table_consumes: None,
//         set_parent: parentify!(compound_end_flag, str_replace_option),
//     },
//     TokenMatch {
//         class: TokenType::CompoundMiddleFlag,
//         key: "COMPOUNDMIDDLE",
//         table_consumes: None,
//         set_parent: parentify!(compound_middle_flag, str_replace_option),
//     },
//     TokenMatch {
//         class: TokenType::CompoundOnlyFlag,
//         key: "ONLYINCOMPOUND",
//         table_consumes: None,
//         set_parent: parentify!(compound_only_flag, str_replace_option),
//     },
//     TokenMatch {
//         class: TokenType::CompoundPermitFlag,
//         key: "COMPOUNDPERMITFLAG",
//         table_consumes: None,
//         set_parent: None,
//     },
//     TokenMatch {
//         class: TokenType::CompoundForbidFlag,
//         key: "COMPOUNDFORBIDFLAG",
//         table_consumes: None,
//         set_parent: None,
//     },
//     TokenMatch {
//         class: TokenType::CompoundMoreSuffixes,
//         key: "COMPOUNDMORESUFFIXES",
//         table_consumes: None,
//         set_parent: None,
//     },
//     TokenMatch {
//         class: TokenType::CompoundRoot,
//         key: "COMPOUNDROOT",
//         table_consumes: None,
//         set_parent: None,
//     },
//     TokenMatch {
//         class: TokenType::CompoundWordMax,
//         key: "COMPOUNDWORDMAX",
//         table_consumes: None,
//         set_parent: None,
//     },
//     TokenMatch {
//         class: TokenType::CompoundForbidDuplication,
//         key: "CHECKCOMPOUNDDUP",
//         table_consumes: None,
//         set_parent: None,
//     },
//     TokenMatch {
//         class: TokenType::CompoundForbidRepeat,
//         key: "CHECKCOMPOUNDREP",
//         table_consumes: None,
//         set_parent: None,
//     },
//     TokenMatch {
//         class: TokenType::CompoundForbidUpperBoundary,
//         key: "CHECKCOMPOUNDCASE",
//         table_consumes: None,
//         set_parent: None,
//     },
//     TokenMatch {
//         class: TokenType::CompoundForbidTriple,
//         key: "CHECKCOMPOUNDTRIPLE",
//         table_consumes: None,
//         set_parent: None,
//     },
//     TokenMatch {
//         class: TokenType::CompoundSimplifyTriple,
//         key: "SIMPLIFIEDTRIPLE",
//         table_consumes: None,
//         set_parent: None,
//     },
//     TokenMatch {
//         class: TokenType::CompoundForbidPatterns,
//         key: "CHECKCOMPOUNDPATTERN",
//         table_consumes: Some(|s| re_exprs::apply_re_count(s, &re_exprs::COMPOUND_PATTERN_DEF_RE)),
//         set_parent: None,
//     },
//     TokenMatch {
//         class: TokenType::CompoundForceUpper,
//         key: "FORCEUCASE",
//         table_consumes: None,
//         set_parent: None,
//     },
//     TokenMatch {
//         class: TokenType::CompoundForceSyllable,
//         key: "COMPOUNDSYLLABLE",
//         table_consumes: None,
//         set_parent: None,
//     },
//     TokenMatch {
//         class: TokenType::CompoundSyllableNumber,
//         key: "SYLLABLENUM",
//         table_consumes: None,
//         set_parent: None,
//     },
//     TokenMatch {
//         class: TokenType::Prefix,
//         key: "PFX",
//         table_consumes: Some(|s| re_exprs::apply_re_count(s, &re_exprs::PFX_DEF_RE)),
//         set_parent: None,
//     },
//     TokenMatch {
//         class: TokenType::Suffix,
//         key: "SFX",
//         table_consumes: Some(|s| re_exprs::apply_re_count(s, &re_exprs::SFX_DEF_RE)),
//         set_parent: None,
//     },
//     TokenMatch {
//         class: TokenType::AffixCircumfixFlag,
//         key: "CIRCUMFIX",
//         table_consumes: None,
//         set_parent: None,
//     },
//     TokenMatch {
//         class: TokenType::AffixForbiddenWordFlag,
//         key: "FORBIDDENWORD",
//         table_consumes: None,
//         set_parent: None,
//     },
//     TokenMatch {
//         class: TokenType::AffixFullStrip,
//         key: "FULLSTRIP",
//         table_consumes: None,
//         set_parent: None,
//     },
//     TokenMatch {
//         class: TokenType::AffixKeepCase,
//         key: "KEEPCASE",
//         table_consumes: None,
//         set_parent: None,
//     },
//     TokenMatch {
//         class: TokenType::AffixInputConversion,
//         key: "ICONV",
//         table_consumes: Some(|s| re_exprs::apply_re_count(s, &re_exprs::ICONV_DEF_RE)),
//         set_parent: None,
//     },
//     TokenMatch {
//         class: TokenType::AffixOutputConversion,
//         key: "OCONV",
//         table_consumes: Some(|s| re_exprs::apply_re_count(s, &re_exprs::OCONV_DEF_RE)),
//         set_parent: None,
//     },
//     TokenMatch {
//         class: TokenType::AffixLemmaPresentDeprecated,
//         key: "LEMMA_PRESENT",
//         table_consumes: None,
//         set_parent: None,
//     },
//     TokenMatch {
//         class: TokenType::AffixNeededFlag,
//         key: "NEEDAFFIX",
//         table_consumes: None,
//         set_parent: None,
//     },
//     TokenMatch {
//         class: TokenType::AffixPseudoRootFlagDeprecated,
//         key: "PSEUDOROOT",
//         table_consumes: None,
//         set_parent: None,
//     },
//     TokenMatch {
//         class: TokenType::AffixSubstandardFlag,
//         key: "SUBSTANDARD",
//         table_consumes: None,
//         set_parent: None,
//     },
//     TokenMatch {
//         class: TokenType::AffixWordChars,
//         key: "WORDCHARS",
//         table_consumes: None,
//         set_parent: None,
//     },
//     TokenMatch {
//         class: TokenType::AffixCheckSharps,
//         key: "CHECKSHARPS",
//         table_consumes: None,
//         set_parent: None,
//     },
// ];

// pub struct EncodingMatch<'a> {
//     class: EncodingType,
//     key: &'a str,
// }
// impl EncodingMatch<'_> {
//     pub fn get_key(&self) -> &str {
//         self.key
//     }
//     pub fn get_class(&self)->&EncodingType{
//         &self.class
//     }
// }
