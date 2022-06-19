// Macro import
use stringmetrics_derive::AffixToken;

trait AffixToken {
    fn load_str(&mut self, s: &str) -> ();
    fn update_parent(&self) -> ();
}

#[derive(AffixToken)]
#[affix_key = "MYKEY"]
#[affix_format = "bool"]
struct Encoding;

fn x() -> () {
    let e = Encoding {};
    e.update_parent();
}
