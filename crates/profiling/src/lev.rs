use stringmetrics::levenshtein;

fn main() {
    for _ in 0..1000 {
        levenshtein("an orange cat", "an auburn bat");
    }
}

// fn main() {
//     let mut s1 = "an orange cat".to_string();
//     for _ in 0..1000 {
//         levenshtein(s1.as_str(), "an auburn bat");
//         s1.push('a');
//         // println!("{}", l);
//     }
// }
