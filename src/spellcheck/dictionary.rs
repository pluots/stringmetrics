//! A dictionary contains methods and a list of Entries
//! An 

use std::collections::BTreeSet;


struct ReplaceRule {
    
}
struct ReplaceRule {

}

enum Rule {
    Pfx,
    Sfx,
    Rep
}

// An entry has a base string and many rules
struct Entry {
    base: String,


}

/// A dict has many entries, plus methods
struct Dictionary {
    entries: BTreeSet<Entry>,
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_jaccard_empty() {
        assert!(jaccard("".chars(), "".chars()).is_nan());
    }


}
