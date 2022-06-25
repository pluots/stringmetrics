//! A dictionary contains methods and a list of Entries
//! Load hunspell dicts
//! http://pwet.fr/man/linux/fichiers_speciaux/hunspell/

use crate::spellcheck::affix::Affix;
use std::collections::HashSet;

/// A dict has many entries, plus methods
/// try_chars: allowed to suggest words that replace these chars
/// nosuggest_flag:
pub struct Dictionary {
    // General word list
    wordlist: HashSet<String>,
    // Words to accept but never suggest
    wordlist_nosuggest: HashSet<String>,
    // Words forbidden by the personal dictionary
    wordlist_forbidden: HashSet<String>,
    pub affix: Affix,

    // These hold the files as loaded
    // Will be emptied upon compile
    raw_wordlist: Vec<String>,
    raw_wordlist_personal: Vec<String>,
}

impl Dictionary {
    pub fn new() -> Dictionary {
        Dictionary {
            wordlist: HashSet::new(),
            wordlist_nosuggest: HashSet::new(),
            wordlist_forbidden: HashSet::new(),
            affix: Affix::new(),
            raw_wordlist: Vec::new(),
            raw_wordlist_personal: Vec::new(),
        }
    }

    /// Can also be done with strings
    pub fn load_affix_from_str(&mut self, s: &str) -> Result<(), String> {
        self.affix.load_from_str(s)
    }

    pub fn load_dictionar_from_str(&mut self, s: &str) {
        self.raw_wordlist = s.lines().map(|l| l.to_string()).collect()
    }
    pub fn load_personal_dict_from_str(&mut self, s: &str) {
        self.raw_wordlist_personal = s.lines().map(|l| l.to_string()).collect()
    }

    /// Match affixes, personal dict, etc
    pub fn compile(&mut self) -> Result<(), String> {
        // Work through the personal word list
        for word in self.raw_wordlist_personal.iter() {
            // Words will be in the format "*word/otherword" where "word" is the
            // main word to add, but it will get all rules of "otherword"
            let split: Vec<&str> = word.split('/').collect();
            let forbidden = word.starts_with('*');

            match split.get(1) {
                Some(rootword) => {
                    // Find "otherword/" in main wordlist
                    let mut tmp = rootword.to_string();
                    tmp.push('/');
                    let filtval = tmp.trim_start_matches("*");

                    match self
                        .raw_wordlist
                        .iter()
                        .filter(|s| s.starts_with(&filtval))
                        .next()
                    {
                        Some(w) => (),
                        None => return Err("Root word not found".to_string()),
                    }
                }
                None => (),
            }
        }

        for word in self.raw_wordlist.iter() {
            let split: Vec<&str> = word.split('/').collect();
        }

        Ok(())
    }

    pub fn check(&self) {}
}

/// Apply affix rules to a given root word, based on what tokens it provides
fn generate_wordlist_from_afx(rootword: &str, tokens: &str, affix: &Affix) -> Vec<String> {
    for rule in &affix.affix_rules {
        if tokens.contains(&rule.ident) {}
    }
    Vec::new()
}
