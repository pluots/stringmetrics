#[derive(PartialEq, Debug)]
enum TokenClass {
    Encoding,
    Flag,
    ComplexPrefixes,
    Language,
    IgnoreChars,
    AffixFlag,
    MorphAlias,

    // suggestion-based options
    NeighborKeys,
    TryChars,
    NoSuggestFlag,
    CompoundSuggestionsMax,
    NGramSuggestionsMax,
    NGramDiffMax,
    NGramOnlyKeepOne,
}

const TOKEN_MAP: [(TokenClass, &str); 14] = [
    (TokenClass::Encoding, "SET"),
    (TokenClass::Flag, "FLAG"),
    (TokenClass::ComplexPrefixes, "COMPLEXPREFIXES"),
    (TokenClass::Language, "LANG"),
    (TokenClass::IgnoreChars, "IGNORE"),
    (TokenClass::AffixFlag, "AF"),
    (TokenClass::MorphAlias, "AM"),
    (TokenClass::NeighborKeys, "KEY"),
    (TokenClass::TryChars, "TRY"),
    (TokenClass::NoSuggestFlag, "NOSUGGEST"),
    (TokenClass::CompoundSuggestionsMax, "MAXCPDSUGS"),
    (TokenClass::NGramSuggestionsMax, "MAXNGRAMSUGS"),
    (TokenClass::NGramDiffMax, "MAXDIFF"),
    (TokenClass::NGramOnlyKeepOne, "ONLYMAXDIFF"),
];

// }

impl TokenClass {
    /// Load a `TokenClass` from a token string
    ///
    /// # Example
    ///
    /// ```
    /// use stringmetrics::spellcheck::affix_tokens::TokenClass;
    /// let s = "NOSUGGEST";
    /// assert_eq!(TokenClass::from_token_str(s), TokenClass::NoSuggestFlag);
    /// ```
    fn from_token_str(s: &str) -> Option<&TokenClass> {
        match TOKEN_MAP.iter().find(|&&(_, v)| v == s) {
            Some(tmap) => Some(&tmap.0),
            None => None,
        }
    }

    ///
    fn to_token_str(self) -> &'static str {
        TOKEN_MAP.iter().find(|&(k, _)| k == &self).unwrap().1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(
            TokenClass::from_token_str("NOSUGGEST").unwrap(),
            &TokenClass::NoSuggestFlag
        );
    }

    #[test]
    fn test_to_str() {
        assert_eq!(
            TokenClass::to_token_str(TokenClass::NoSuggestFlag),
            "NOSUGGEST"
        );
    }
}
