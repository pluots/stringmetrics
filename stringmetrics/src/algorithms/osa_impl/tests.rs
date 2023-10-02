use super::*;

// #[test]
// fn test_levweights_swap() {
//     let mut w = LevWeights::new(10, 20, 30);
//     w.swap();
//     let expected = LevWeights {
//         insertion: 20,
//         deletion: 10,
//         substitution: 30,
//     };
//     assert_eq!(w, expected);
// }

#[test]
fn test_osa_equal() {
    assert_eq!(osa_distance("abcdef", "abcdef"), 0);
}

#[test]
fn test_osa_empty() {
    assert_eq!(osa_distance("", ""), 0);
    assert_eq!(osa_distance("abcdef", ""), 6);
    assert_eq!(osa_distance("", "abcdef"), 6);
}

#[test]
fn test_osa_basic() {
    assert_eq!(osa_distance("abcd", "ab"), 2);
    assert_eq!(osa_distance("ab", "abcd"), 2);
    assert_eq!(osa_distance("abcd", "ad"), 2);
    assert_eq!(osa_distance("abcd", "cd"), 2);
    assert_eq!(osa_distance("abcd", "a"), 3);
    assert_eq!(osa_distance("abcd", "c"), 3);
    assert_eq!(osa_distance("abcd", "accd"), 1);
    assert_eq!(osa_distance("kitten", "sitting"), 3);
    assert_eq!(osa_distance("sitting", "kitten"), 3);
    assert_eq!(osa_distance("not", "to a"), 3);
    assert_eq!(osa_distance("to be a bee", "not to bee"), 6);
}

#[test]
fn test_osa_trick_skips() {
    // Try to trick the part that skips forward and backward
    assert_eq!(osa_distance("abcd", "abcd"), 0);
    assert_eq!(osa_distance("abcd", "ad"), 2);
    assert_eq!(osa_distance("abcd", "cd"), 2);
    assert_eq!(osa_distance("abcd", "a"), 3);
    assert_eq!(osa_distance("abcd", "b"), 3);
    assert_eq!(osa_distance("abcd", "c"), 3);
    assert_eq!(osa_distance("abcd", "d"), 3);
    assert_eq!(osa_distance("a", "abcd"), 3);
    assert_eq!(osa_distance("d", "abcd"), 3);
    assert_eq!(osa_distance("notate", "to ate"), 2);
    assert_eq!(osa_distance("to ate", "notate"), 2);
    assert_eq!(osa_distance("to be a", "not to"), 6);
    assert_eq!(osa_distance("not to", "to be a"), 6);
    assert_eq!(osa_distance("abccc", "accc"), 1);
}

#[test]
fn test_osa_limit_one_empty() {
    assert_eq!(osa_limit("abcdef", "", 3), 3);
    assert_eq!(osa_limit("", "abcdef", 3), 3);
    assert_eq!(osa_limit("abcdef", "", 8), 6);
    assert_eq!(osa_limit("", "abcdef", 8), 6);
}

#[test]
fn test_osa_limit() {
    // Most of this is tested via damerau()
    // just need to validate limits
    assert_eq!(osa_limit("abcdef", "000000", 3), 3);
    assert_eq!(osa_limit("ab", "0000", 3), 3);
}

#[test]
fn test_osa_transpose() {
    // Target the transpose cost
    // assert_eq!(osa_distance("ab", "ba"), 1);
    assert_eq!(osa_distance("ab", "bac"), 2);
    assert_eq!(osa_distance("xcb", "abc"), 2);
    assert_eq!(osa_distance("sitting", "sittign"), 1);
    assert_eq!(osa_distance("sitting", "istting"), 1);
    assert_eq!(osa_distance("sitting", "isttign"), 2);
    assert_eq!(osa_distance("siting", "isteign"), 4);
    assert_eq!(osa_distance("kitten", "kitetn"), 1);
    // damerau will be different here
    assert_eq!(osa_distance("abc", "ca"), 3);
}

#[test]
fn test_osa_weights() {
    assert_eq!(
        osa_weight("ab", "ba", 50, &DamerauWeights::new(100, 100, 100, 5)),
        5
    );
}

// #[test]
// fn test_osa_weight_insertion() {
//     let weights = LevWeights::new(10, 1, 1);
//     assert_eq!(osa_weight("", "a", 100, &weights), 10);
//     assert_eq!(osa_weight("a", "", 100, &weights), 1);
//     assert_eq!(osa_weight("", "ab", 100, &weights), 20);
//     assert_eq!(osa_weight("ab", "", 100, &weights), 2);
//     assert_eq!(osa_weight("ab", "abcd", 100, &weights), 20);
//     assert_eq!(osa_weight("kitten", "sitting", 100, &weights), 12);
// }

// #[test]
// fn test_osa_weight_deletion() {
//     let weights = LevWeights::new(1, 10, 1);
//     assert_eq!(osa_weight("", "a", 100, &weights), 1);
//     assert_eq!(osa_weight("a", "", 100, &weights), 10);
//     assert_eq!(osa_weight("", "ab", 100, &weights), 2);
//     assert_eq!(osa_weight("ab", "", 100, &weights), 20);
//     assert_eq!(osa_weight("kitten", "sitting", 100, &weights), 3);

//     let weights = LevWeights::new(1, 10, 2);
//     assert_eq!(osa_weight("abc", "ac", 100, &weights), 10);
//     assert_eq!(osa_weight("abcd", "ac", 100, &weights), 20);
// }

// #[test]
// fn test_osa_weight_substitution() {
//     // Note that when substitution cost is high, the algorithm will prefer
//     // a deletion and insertion
//     let weights = LevWeights::new(10, 10, 5);
//     assert_eq!(osa_weight("a", "b", 100, &weights), 5);
//     let weights = LevWeights::new(10, 10, 2);
//     assert_eq!(osa_weight("abcd", "acc", 100, &weights), 12);
//     let weights = LevWeights::new(4, 3, 2);
//     assert_eq!(osa_weight("kitten", "sitting", 100, &weights), 8);
// }
