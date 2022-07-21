use std::cmp::min;

/// This function has an unstable API
///
/// It is designed to identify the longest similar start and end sequences in a
/// pair of iterators.
///
/// Unfortunately, we return an ugly tuple rather than a struct since that is
/// about 5% faster. Use this via destructuring rather than indexing.
#[inline]
pub fn find_eq_end_items<I, T, D>(a: I, b: I) -> (usize, usize, usize, usize)
where
    I: IntoIterator<IntoIter = D>,
    D: DoubleEndedIterator<Item = T> + Clone,
    T: PartialEq,
{
    let a_len;
    let b_len;

    let mut start_same = 0usize;
    let mut end_same = 0usize;
    let mut counting = true;

    // Weirdly, this outperformed manual += 1 counting
    let mut r_iter = 0..;
    let mut a_iter = a.into_iter();
    let mut b_iter = b.into_iter();

    // We need to recreate the iterator here so we can use it later
    // Note: this is likely doable without cloning if we work from the front,
    // then work from the back, and add the difference.
    let a_iter2 = a_iter.clone();
    let b_iter2 = b_iter.clone();

    loop {
        match (a_iter.next(), b_iter.next()) {
            (Some(a_v), Some(b_v)) => {
                r_iter.next();
                if counting && a_v == b_v {
                    start_same += 1
                } else if counting {
                    counting = false
                }
            }
            (Some(_), None) => {
                let common_len = r_iter.next().unwrap();
                a_len = common_len + 1 + a_iter.count();
                b_len = common_len;
                break;
            }
            (None, Some(_)) => {
                let common_len = r_iter.next().unwrap();
                b_len = common_len + 1 + b_iter.count();
                a_len = common_len;
                break;
            }
            (None, None) => {
                let common_len = r_iter.next().unwrap();
                a_len = common_len;
                b_len = common_len;
                break;
            }
        }
    }

    let end_max_iters = min(a_len, b_len) - start_same;

    for (i, (a_char, b_char)) in a_iter2.rev().zip(b_iter2.rev()).enumerate() {
        if i >= end_max_iters {
            break;
        }
        if a_char == b_char {
            end_same += 1
        } else {
            break;
        }
    }

    (a_len, b_len, start_same, end_same)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_eq_items() {
        assert_eq!(find_eq_end_items("".chars(), "".chars()), (0, 0, 0, 0));
        assert_eq!(find_eq_end_items("aaaa".chars(), "".chars()), (4, 0, 0, 0));
        assert_eq!(find_eq_end_items("".chars(), "aaaa".chars()), (0, 4, 0, 0));
        assert_eq!(
            find_eq_end_items("abcd".chars(), "abcd".chars()),
            (4, 4, 4, 0)
        );
        assert_eq!(
            find_eq_end_items("aaaa".chars(), "aa".chars()),
            (4, 2, 2, 0)
        );
        assert_eq!(
            find_eq_end_items("aaaa".chars(), "aabbbb".chars()),
            (4, 6, 2, 0)
        );
        assert_eq!(
            find_eq_end_items("xxaa".chars(), "yyyyaa".chars()),
            (4, 6, 0, 2)
        );
        assert_eq!(
            find_eq_end_items("aaxxxxbb".chars(), "aaaabbbb".chars()),
            (8, 8, 2, 2)
        );
        assert_eq!(
            find_eq_end_items("aaaa".chars(), "bbbb".chars()),
            (4, 4, 0, 0)
        );
    }
}
