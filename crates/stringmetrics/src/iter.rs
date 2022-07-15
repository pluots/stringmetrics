use std::cmp::min;

#[derive(Debug, PartialEq)]
pub struct IterPairInfo {
    a_len: usize,
    b_len: usize,
    start_same: usize,
    end_same: usize,
}

impl IterPairInfo {
    pub fn new(a_len: usize, b_len: usize, start_same: usize, end_same: usize) -> Self {
        IterPairInfo {
            a_len,
            b_len,
            start_same,
            end_same,
        }
    }

    pub fn expand(&self) -> (usize, usize, usize, usize) {
        (self.a_len, self.b_len, self.start_same, self.end_same)
    }
}

#[inline]
pub fn find_eq_end_items<I, T, D>(a: I, b: I) -> IterPairInfo
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

    IterPairInfo {
        a_len,
        b_len,
        start_same,
        end_same,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_eq_items() {
        assert_eq!(
            find_eq_end_items("".chars(), "".chars()),
            IterPairInfo::new(0, 0, 0, 0)
        );
        assert_eq!(
            find_eq_end_items("aaaa".chars(), "".chars()),
            IterPairInfo::new(4, 0, 0, 0)
        );
        assert_eq!(
            find_eq_end_items("".chars(), "aaaa".chars()),
            IterPairInfo::new(0, 4, 0, 0)
        );
        assert_eq!(
            find_eq_end_items("abcd".chars(), "abcd".chars()),
            IterPairInfo::new(4, 4, 4, 0)
        );
        assert_eq!(
            find_eq_end_items("aaaa".chars(), "aa".chars()),
            IterPairInfo::new(4, 2, 2, 0)
        );
        assert_eq!(
            find_eq_end_items("aaaa".chars(), "aabbbb".chars()),
            IterPairInfo::new(4, 6, 2, 0)
        );
        assert_eq!(
            find_eq_end_items("xxaa".chars(), "yyyyaa".chars()),
            IterPairInfo::new(4, 6, 0, 2)
        );
        assert_eq!(
            find_eq_end_items("aaxxxxbb".chars(), "aaaabbbb".chars()),
            IterPairInfo::new(8, 8, 2, 2)
        );
        assert_eq!(
            find_eq_end_items("aaaa".chars(), "bbbb".chars()),
            IterPairInfo::new(4, 4, 0, 0)
        );
    }
}
