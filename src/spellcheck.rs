///
#[macro_export]
macro_rules! graph_vec {
    ($ex:expr) => {
        $ex.graphemes(true).collect::<Vec<&str>>()
    };
}

pub mod affix;
mod affix_tokens;
mod affix_types;
// pub mod dictionary;
