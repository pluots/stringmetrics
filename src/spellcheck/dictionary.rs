//! A dictionary contains methods and a list of Entries
//! Load hunspell dicts
//! http://pwet.fr/man/linux/fichiers_speciaux/hunspell/

use std::collections::BTreeSet;
use crate::spellcheck::affix::{Affix}

// Check 
// trait RuleTrait {
//     fn apply
// }

struct PrefixRule {

}

struct SuffixRule {

}

/// MAP 1
/// MAP uü
struct MapRule{

}






// An entry has a base string and many rules
struct Entry {
    base: String
}

/// A dict has many entries, plus methods
/// try_chars: allowed to suggest words that replace these chars
/// nosuggest_flag: 
struct Dictionary {
    entries: BTreeSet<Entry>,
    rules: vec<Rule>,
    try_chars: str, // from TRY
    nosuggest_flag:str,
    
}

impl Dictionary{ 

    // Match rules to the relevant entries
    fn match_rules () {
        
    }

    /// Read the dict in from an iterator
    /// Usually with something like
    /// 
    /// let file = File::open(filename)?;
    /// Ok(io::BufReader::new(file).lines())
    /// 
    /// Can also be done with strings
    fn load_affix_from_str(s:&str) {
        afx = Affix::new();
        afx.load_from_str(s);
        // for line in lines {
        //     if line.starts_with('#')      { println!("Heading") }
        //     else if line.starts_with('>') { println!("Quotation") }
        //     else if line.starts_with('-') { println!("Inline list") }
        //     else if line.starts_with('`') { println!("Code") }
        //     else                          { println!("Other")
        // }
        }
    

    /// Note: make sure 
    fn load_dictionary<T:IntoIterator<Item=&str>>(lines:T){
        for line in lines {

        }
    }
    fn load_personal_dict<T:IntoIterator<Item=&str>>(lines:T){
        for line in lines {

        }
    }


    /// Match affixes, personal dict, etc
    fn prepare() {

    }
}


// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::fs;

//     #[test]
//     fn test_jaccard_empty() {
//         assert!(jaccard("".chars(), "".chars()).is_nan());
//     }


// }
