use crate::algorithms::lev_impl::LevState;
use core::fmt::Debug;
use std::cmp::min;
use std::mem;

#[inline]
pub fn try_osa_iter<I, T, D>(a: I, b: I, limit: u32) -> Option<u32>
where
    I: IntoIterator<IntoIter = D>,
    D: DoubleEndedIterator<Item = T> + Clone,
    T: PartialEq + Clone + Debug,
{
    let state = LevState::new(a.into_iter(), b.into_iter());
    let LevState {
        a_iter,
        b_iter,
        a_len,
        b_len,
    } = state;

    // Only check b_len because if a_len is 0, the loop won't happen
    if b_len == 0 {
        return Some(min(a_len, limit));
    }

    if b_len - a_len > limit {
        return None;
    }
    if b_len - a_len >= limit {
        return Some(limit);
    }

    let mut last_cache: Vec<u32> = (1..=b_len).collect();
    let mut cache: Vec<u32> = vec![0; b_len as usize];
    let mut tmp_res = b_len;
    let mut last_a: Option<T> = None;
    let mut last_b: Option<T> = None;

    for (i, a_item) in a_iter.enumerate().take_while(|&(i, _)| i < a_len as usize) {
        // Our "horizontal" iterations always start with the leftmost column,
        // which is the insertion cost (or substitution above)
        // temp_res is also our insertion cost base
        let mut sub_base = i as u32;
        tmp_res = sub_base + 1;

        // Go through and do our calculations. we need to preserve the "up left"
        // (sub_base) and "left" (tmp_res) values, the rest can be overwritten
        for (j, b_item) in b_iter
            .clone()
            .enumerate()
            .take_while(|&(j, _)| j < b_len as usize)
        {
            let del_base = last_cache[j];

            // Insertion costs and deletion costs are their bases + 1
            // i.e., the value to the left or above plus 1
            // Substitution cost is equal to the up-left (sub_base) cost if equal,
            // otherwise up-left value + 1.
            if a_item == b_item {
                tmp_res = min(min(tmp_res, del_base) + 1, sub_base);
            } else {
                tmp_res = min(min(tmp_res, del_base), sub_base) + 1;
            }

            // SAFETY: if we have gone through the loop once, these have values
            if i > 0
                && j > 0
                && unsafe { a_item == last_b.clone().unwrap_unchecked() }
                && unsafe { b_item == last_a.clone().unwrap_unchecked() }
            {
                // Evaluate transpose cost
                tmp_res = min(tmp_res, last_cache[j - 1]);
            }

            // As we shift to the right, our deletion square becomes our
            // substitution square
            sub_base = del_base;

            // Save our insertion cost for the next iteration
            cache[j] = tmp_res;

            last_b = Some(b_item);
        }

        if tmp_res > limit {
            return None;
        }

        last_a = Some(a_item);
        mem::swap(&mut last_cache, &mut cache);
    }

    Some(tmp_res)
}
