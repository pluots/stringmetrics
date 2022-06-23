/// Create a vector of unicode graphemes
/// Each &str within this array is a single unicode character, which
/// is composed of one to four 8-bit integers ("chars")
#[macro_export]
macro_rules! graph_vec {
    ($ex:expr) => {
        $ex.graphemes(true)
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| s.to_string())
            .collect()
    };
}

pub mod affix;
mod affix_serde;
// mod affix_tokens;
mod affix_types;
// pub mod dictionary;
