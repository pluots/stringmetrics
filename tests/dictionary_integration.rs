use std::collections::{BTreeSet, HashSet};
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use stringmetrics::spellcheck::Affix;
use stringmetrics::spellcheck::Dictionary;

#[test]
fn load_affix_file() {
    let mut afx = Affix::new();

    let content = fs::read_to_string("dictionaries/en.aff").unwrap();

    afx.load_from_str(content.as_str());
    // println!("{:?}",afx);
}

// #[test]
// fn load_dict() {
//     let mut dic = Dictionary::new();

//     let content = fs::read_to_string("dictionaries/en.aff").unwrap();
// }
#[test]
fn test_short_compile() {
    let mut dic = Dictionary::new();

    let aff_content = fs::read_to_string("tests/files/short.aff").unwrap();
    let dic_content = fs::read_to_string("tests/files/short.dic").unwrap();

    dic.affix.load_from_str(aff_content.as_str()).unwrap();
    dic.load_dictionar_from_str(dic_content.as_str());
    dic.compile();

    println!("{:?}", dic.wordlist);
    // println!("{:?}", dic.wordlist_forbidden);
}
