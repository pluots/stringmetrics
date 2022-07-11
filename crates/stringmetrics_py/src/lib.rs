use pyo3::prelude::*;
// use pyo3::types::PyIterator;
use stringmetrics::algorithms;

/// Formats the sum of two numbers as string.
#[pyfunction]
#[inline]
fn levenshtein(a: &str, b: &str, limit: Option<u32>) -> u32 {
    match limit {
        Some(v) => algorithms::levenshtein_limit(a, b, v),
        None => algorithms::levenshtein(a, b),
    }
}

/// Formats the sum of two numbers as string.
// #[pyfunction]
// #[inline]
// fn levenshtein_advanced(
//     a: &PyIterator,
//     b: &PyIterator,
//     limit: Option<u32>,
//     w_ins: u32,
//     w_del: u32,
//     w_sub: u32,
// ) -> u32 {
//     let uselimit = match limit {
//         Some(v) => v,
//         None => u32::MAX,
//     };
//     let weights = algorithms::LevWeights::new(w_ins, w_del, w_sub);
//     algorithms::levenshtein_limit_weight_iter(a, b, uselimit, &weights)
// }

/// A Python module implemented in Rust.
#[pymodule]
#[inline]
fn stringmetrics(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(levenshtein, m)?)?;
    // m.add_function(wrap_pyfunction!(levenshtein_limit, m)?)?;
    Ok(())
}
