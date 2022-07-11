//! # Hamming Distance Computations
//!
//! The hamming distance between two equal length strings is the number of
//! substitutions required to change one string into the other. The functions in
//! this module help to calculate that.

/// Hamming distance computations
///
/// Calculate the number of substitutions required to change one string into the
/// other.
///
/// # Arguments
///
/// * `a`: A `&str` or `String` to compare
/// * `b`: A `&str` or `String` to compare
///
/// # Panics
///
/// If the given strings are not equal length, this function will panic
///
/// # Example
///
/// ```
/// use stringmetrics::hamming;
/// let a = "abcdefg";
/// let b = "aaadefa";
/// assert_eq!(hamming(a, b), 3);
/// ```
pub fn hamming(a: &str, b: &str) -> u32 {
    assert_eq!(
        a.len(),
        b.len(),
        "Hamming distance requires slices of equal length, lengths {} and {} given",
        a.len(),
        b.len()
    );

    // Error case already handled
    hamming_iter(&mut a.chars(), &mut b.chars()).unwrap()
}

/// Apply the hamming function to slices
///
/// Similar to the above function but can work on something like an array or
/// previously collected vector.
///
/// # Panics
///
/// If the given slices are not equal length, this function will panic
///
/// # Example
///
/// ```
/// use stringmetrics::hamming_slice;
/// let a: Vec<char> = "abcdefg".chars().collect();
/// let b: Vec<char> = "aaadefa".chars().collect();
/// assert_eq!(hamming_slice(&a[..6], &b[..6]), 2);
///
/// assert_eq!(hamming_slice(&[1, 2, 3], &[2, 2, 3]), 1);
/// ```
pub fn hamming_slice<T: PartialEq>(a: &[T], b: &[T]) -> u32 {
    assert_eq!(
        a.len(),
        b.len(),
        "Hamming distance requires slices of equal length, lengths {} and {} given",
        a.len(),
        b.len()
    );

    let mut distance = 0;

    let zipped = a.iter().zip(b.iter());

    for (aa, bb) in zipped {
        if aa != bb {
            distance += 1;
        }
    }

    distance
}

/// Apply the hamming function to iterators
///
/// This is useful for avoiding the need to collect data before performing the
/// calculation. `a` and `b` are mutable iterators; mutability is required as
/// the iterator will be consumed.
///
/// Returns a [`Result`] with `Ok(v)` if the function completed successfully, or
/// `Err(s)` if there was an iterator length mismatch.
///
/// # Example
///
/// ```
/// use stringmetrics::hamming_iter;
/// let a = "abcdefg";
/// let b = "aaadefa";
/// assert_eq!(hamming_iter(&mut a.chars(), &mut b.chars()).unwrap(), 3);
///
/// assert_eq!(hamming_iter(&mut (1..3), &mut (2..4)).unwrap(), 2);
/// ```
pub fn hamming_iter<I: Iterator<Item = T>, T: PartialEq>(
    a: &mut I,
    b: &mut I,
) -> Result<u32, String> {
    let mut distance = 0;

    loop {
        let aa = a.next();
        let bb = b.next();

        if aa == None && bb == None {
            break;
        } else if aa == None || bb == None {
            return Err("Length mismatch".to_owned());
        } else if aa != bb {
            distance += 1;
        }
        // Final case of aa == bb, do nothing
    }

    Ok(distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_panic_on_not_equal() {
        hamming("abc", "ab");
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(hamming("", ""), 0);
    }

    #[test]
    fn test_basic() {
        assert_eq!(hamming("abcdefg", "0bc1ef2"), 3);
    }

    #[test]
    fn test_slice() {
        assert_eq!(hamming_slice(&[1, 2, 3], &[2, 2, 3]), 1);
    }

    #[test]
    fn test_iter() {
        assert_eq!(hamming_iter(&mut (1..3), &mut (2..4)).unwrap(), 2);
    }
}
