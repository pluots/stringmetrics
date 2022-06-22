use super::affix::Affix;
use super::affix_types::{EncodingType, TokenType};
use strum::{EnumProperty, VariantNames};
use unicode_segmentation::UnicodeSegmentation;

macro_rules! parentify {
    // Boolean field just assigns true and returns Ok (Flag is just either there
    // or not)
    ( $parent:ident.$field:ident, $token:ident, bool ) => {
        match $token.data {
            ProcessedTokenData::Bool(b) => $parent.$field = b,
            _ => panic!("Bad token type specified!"),
        }
    };
    ( $parent:ident.$field:ident, $token:ident, str_replace ) => {
        match $token.data {
            ProcessedTokenData::String(s) => $parent.$field = s,
            _ => panic!("Bad token type specified!"),
        }
    }; 

    // Use str_add any time we have a `String` that we want to append to.
    // Basically the same as above except we append to the existing vector and
    // sort rather than replacing what's there. Usable for `Vec<&str>`.
    ( $parent:ident.$field:ident, $token:ident, str_append ) => {
        match $token.data {
            ProcessedTokenData::String(s) => {
                let mut tmp = s.graphemes(true).collect::<Vec<&str>>();
                    tmp.sort();
                    tmp.dedup();
                $parent.$field.append(&mut tmp)

            },
            _ => panic!("Bad token type specified!"),
        }
    };
}

// Unwrap a TokenData type
macro_rules! t_data_unwrap {
    ( $token:ident, $field:ident ) => {
        match $token.data {
            ProcessedTokenData::$field(f) => f,
            _ => panic!("Bad token type specified!"),
        }
    };
}

/// Populate an Affix class from the string version of a file
pub fn load_affix_from_str<'a>(ax: &mut Affix, s: &str) -> Result<(), String> {
    // This will give us a list of tokens with no processing applied
    let raw_stripped = strip_comments(s);
    let raw_tokens = create_raw_tokens(raw_stripped.as_str());
    match create_processed_tokens(raw_tokens) {
        Ok(tokens) => set_parent(ax, tokens),
        Err(e) => Err(e),
    }
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
/// Words have been split on whitespace and the token prefix has been stripped
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
///
/// Note: this strips prefixes
fn create_raw_tokens<'a>(s: &'a str) -> Vec<AffixRawToken> {
    // Temporarially hold the "next vector" rather than working one, blank until
    // needed
    let mut working_vec: Vec<&'a str> = Vec::new();
    let mut working_type = TokenType::FileStart;

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

#[derive(Debug, PartialEq)]
enum ProcessedTokenData<'a> {
    Bool(bool),
    String(&'a str),
    Table(Vec<Vec<&'a str>>),
}

#[derive(Debug, PartialEq)]
struct AffixProcessedToken<'a> {
    pub ttype: TokenType,
    pub data: ProcessedTokenData<'a>,
}

/// Use the first raw token to determine how many more to read into the table
/// Returns a u16 if successful, error otherwise
fn get_table_item_count<'a>(token: &AffixRawToken) -> Result<u16, String> {
    // Recall: our tokens have the token prefix stripped

    let temp = match token.ttype {
        // AF [n]
        TokenType::AffixFlag => token.content.get(0),
        // AM [n]
        TokenType::MorphAlias => token.content.get(0),
        // REP [n]
        TokenType::Replacement => token.content.get(0),
        // MAP [n]
        TokenType::Mapping => token.content.get(0),
        // PHONE [n]
        TokenType::Phonetic => token.content.get(0),
        // BREAK [n]
        TokenType::Breakpoint => token.content.get(0),
        // COMPOUNDRULE [n]
        TokenType::CompoundRule => token.content.get(0),
        // CHECKCOMPOUNDPATTERN [n]
        TokenType::CompoundForbidPatterns => token.content.get(0),
        // PFX flag cross_product number
        TokenType::Prefix => token.content.get(2),
        // SFX flag cross_product number
        TokenType::Suffix => token.content.get(2),
        // ICONV [n]
        TokenType::AffixInputConversion => token.content.get(0),
        // OCONV [n]
        TokenType::AffixOutputConversion => token.content.get(0),
        // Any tokens types that don't have a table
        _ => return Ok(0),
    };

    match temp {
        Some(num) => match num.parse() {
            Ok(val) => Ok(val),
            Err(_) => Err(format!("Bad number at {}", token.ttype)),
        },
        None => Err(format!("Incorrect syntax at {}", token.ttype)),
    }
}

/// Loop through a vector of raw tokens and create the processed version
fn create_processed_tokens(tokens: Vec<AffixRawToken>) -> Result<Vec<AffixProcessedToken>, String> {
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
                    "Token of type {} did not match {}. Expected {} more token.",
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
            continue;
        }

        // If we're not accumulating, just match based on token type
        match token.ttype.get_str("dtype").unwrap() {
            // String or bool are straightforward
            "str" => {
                if token.content.len() != 1 {
                    return Err(format!(
                        "{} is a sting; only one value allowed on line",
                        token.ttype
                    )
);
                };
                retvec.push(AffixProcessedToken {
                    ttype: token.ttype,
                    data: ProcessedTokenData::String(token.content[0]),
                })
            }
            "bool" => {
                if token.content.len() != 0 {
                    return Err(format!(
                        "{} is a boolean; nothing else allowed on line",
                        token.ttype
                    )
);
                };
                retvec.push(AffixProcessedToken {
                    ttype: token.ttype,
                    data: ProcessedTokenData::Bool(true),
                })
            }
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

// Actually go through and set the parent here
fn set_parent<'a>(ax: &mut Affix<'a>, tokens: Vec<AffixProcessedToken>) -> Result<(), String> {
    for token in tokens {
        match token.ttype {
            TokenType::Encoding => match EncodingType::try_from(t_data_unwrap!(token, String)) {
                Ok(et) => ax.encoding = &et,
                Err(e) => return Err("Bad encoding type specified".to_string()),
            },
            TokenType::FlagType => todo!(),
            TokenType::ComplexPrefixes => {
                parentify!(ax.complex_prefixes, token,bool)
                // match token.data {
                //     ProcessedTokenData::Bool(b)=>ax.complex_prefixes = b,
                //     _ => panic!("Bad token type specified!")
                // }
            }
            TokenType::Language => parentify!(ax.lang,token, str_replace),
            TokenType::IgnoreChars => parentify!(ax.ignore_chars,token, str_append),
            TokenType::AffixFlag => parentify!(ax.afx_flag_vector,token, str_append),
            TokenType::MorphAlias => todo!(),
            TokenType::NeighborKeys => todo!(),
            TokenType::TryCharacters => todo!(),
            TokenType::NoSuggestFlag => todo!(),
            TokenType::CompoundSuggestionsMax => todo!(),
            TokenType::NGramSuggestionsMax => todo!(),
            TokenType::NGramDiffMax => todo!(),
            TokenType::NGramLimitToDiffMax => todo!(),
            TokenType::NoSpaceSubs => todo!(),
            TokenType::KeepTerminationDots => todo!(),
            TokenType::Replacement => todo!(),
            TokenType::Mapping => todo!(),
            TokenType::Phonetic => todo!(),
            TokenType::WarnRareFlag => todo!(),
            TokenType::ForbitWarnWords => todo!(),
            TokenType::Breakpoint => todo!(),
            TokenType::CompoundRule => todo!(),
            TokenType::CompoundMinLength => todo!(),
            TokenType::CompoundFlag => todo!(),
            TokenType::CompoundBeginFlag => todo!(),
            TokenType::CompoundEndFlag => todo!(),
            TokenType::CompoundMiddleFlag => todo!(),
            TokenType::CompoundOnlyFlag => todo!(),
            TokenType::CompoundPermitFlag => todo!(),
            TokenType::CompoundForbidFlag => todo!(),
            TokenType::CompoundMoreSuffixes => todo!(),
            TokenType::CompoundRoot => todo!(),
            TokenType::CompoundWordMax => todo!(),
            TokenType::CompoundForbidDuplication => todo!(),
            TokenType::CompoundForbidRepeat => todo!(),
            TokenType::CompoundForbidUpperBoundary => todo!(),
            TokenType::CompoundForbidTriple => todo!(),
            TokenType::CompoundSimplifyTriple => todo!(),
            TokenType::CompoundForbidPatterns => todo!(),
            TokenType::CompoundForceUpper => todo!(),
            TokenType::CompoundForceSyllable => todo!(),
            TokenType::CompoundSyllableNumber => todo!(),
            TokenType::Prefix => todo!(),
            TokenType::Suffix => todo!(),
            TokenType::AffixCircumfixFlag => todo!(),
            TokenType::AffixForbiddenWordFlag => todo!(),
            TokenType::AffixFullStrip => todo!(),
            TokenType::AffixKeepCase => todo!(),
            TokenType::AffixInputConversion => todo!(),
            TokenType::AffixOutputConversion => todo!(),
            TokenType::AffixLemmaPresentDeprecated => todo!(),
            TokenType::AffixNeededFlag => todo!(),
            TokenType::AffixPseudoRootFlagDeprecated => todo!(),
            TokenType::AffixSubstandardFlag => todo!(),
            TokenType::AffixWordChars => todo!(),
            TokenType::AffixCheckSharps => todo!(),
            TokenType::FileStart => todo!(),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_comments() {
        let s = "abc\ndef\n# comment\nline with # comment\nendline";
        assert_eq!(strip_comments(s), "abc\ndef\n\nline with \nendline");
    }

    #[test]
    fn test_create_raw_token() {
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

    #[test]
    fn test_get_table_item_count() {
        let count_res0 = get_table_item_count(&AffixRawToken {
            ttype: TokenType::Mapping,
            content: vec!["1"],
        });
        assert_eq!(count_res0, Ok(1));

        let count_res1 = get_table_item_count(&AffixRawToken {
            ttype: TokenType::Prefix,
            content: vec!["A", "Y", "6"],
        });
        assert_eq!(count_res1, Ok(6));

        let count_res2 = get_table_item_count(&AffixRawToken {
            ttype: TokenType::Prefix,
            content: vec!["A", "Y"],
        });
        assert_eq!(count_res2, Err("Incorrect syntax at PFX".to_string()));

        let count_res3 = get_table_item_count(&AffixRawToken {
            ttype: TokenType::Prefix,
            content: vec!["A", "Y", "-80"],
        });
        assert_eq!(count_res3, Err("Bad number at PFX".to_string()));
    }

    #[test]
    fn test_create_processed_tokens() {
        // Test boolean
        let t0 = AffixRawToken {
            ttype: TokenType::NoSpaceSubs,
            content: vec![],
        };
        let res0 = Ok(vec![AffixProcessedToken {
            ttype: TokenType::NoSpaceSubs,
            data: ProcessedTokenData::Bool(true),
        }]);
        assert_eq!(create_processed_tokens(vec![t0]), res0);

        // Test string
        let t1 = AffixRawToken {
            ttype: TokenType::Language,
            content: vec!["mylanguage"],
        };
        let res1 = Ok(vec![AffixProcessedToken {
            ttype: TokenType::Language,
            data: ProcessedTokenData::String("mylanguage"),
        }]);
        assert_eq!(create_processed_tokens(vec![t1]), res1);

        // Test table
        let t20 = AffixRawToken {
            ttype: TokenType::Prefix,
            content: vec!["V", "N", "2"],
        };
        let t21 = AffixRawToken {
            ttype: TokenType::Prefix,
            content: vec!["V", "e"],
        };
        let t22 = AffixRawToken {
            ttype: TokenType::Prefix,
            content: vec!["V", "0"],
        };
        let res1 = Ok(vec![AffixProcessedToken {
            ttype: TokenType::Prefix,
            data: ProcessedTokenData::Table(vec![
                vec!["V", "N", "2"],
                vec!["V", "e"],
                vec!["V", "0"],
            ]),
        }]);
        assert_eq!(create_processed_tokens(vec![t20, t21, t22]), res1);
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

// // Integer fields are a bit more complicated
// // Need to parse the field if possible,
// ( $field:ident, int ) => {
//     Some(|tc, ax, s| match tc.strip_key(s).parse() {
//         Ok(v) => Ok(ax.$field = v),
//         Err(_) => Err("Bad integer value at {}"),
//     })
// };

// // Use str_replace for any time we have `&str` as the `Affix` type. We will
// // replace what exists with our new value
// ( $field:ident, str_replace ) => {
//     Some(|tc, ax, s| {
//         let s1 = tc.strip_key(s);
//         match s1.is_empty() {
//             false => Ok(ax.$field = s1),
//             true => Err("No values found at field {}"),
//         }
//     })
// };

// // Same as above but place the result in an option
// ( $field:ident, str_replace_option ) => {
//     Some(|tc, ax, s| {
//         let s1 = tc.strip_key(s);
//         match s1.is_empty() {
//             false => Ok(ax.$field = Some(s1)),
//             true => Err("No values found at field {}"),
//         }
//     })
// };
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
