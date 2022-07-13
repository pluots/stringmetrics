use stringmetrics::levenshtein;

fn main() {
    let l = levenshtein("an orange cat", "an auburn bat");
    println!("{}", l);
}
