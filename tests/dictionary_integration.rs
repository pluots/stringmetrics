use std::collections::{BTreeSet, HashSet};
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use stringmetrics::spellcheck::affix::Affix;

#[test]
fn load_affix_file() {
    let mut afx = Affix::new();

    let content = fs::read_to_string("dictionaries/en.aff").unwrap();

    afx.load_from_str(content.as_str());
    // println!("{:?}",afx);
}
