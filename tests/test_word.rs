//! 1:1 Port of Python `metaphone/tests/test_word.py` test suite.

use metaphone::word::Word;

#[test]
fn test_init() {
    let word = Word::new("stupendous");
    assert_eq!(word.original, "stupendous");
    assert_eq!(word.decoded, "stupendous");
    assert_eq!(word.normalized, "stupendous");
    assert_eq!(word.upper, "STUPENDOUS");
    assert_eq!(word.length, 10);
    assert_eq!(word.buffer_string(), "  STUPENDOUS      ");
}

#[test]
fn test_init_unicode() {
    let word = Word::new("Çç");
    assert_eq!(word.decoded, "ss");
    assert_eq!(word.normalized, "ss");
    assert_eq!(word.upper, "SS");
    assert_eq!(word.length, 2);
    assert_eq!(word.buffer_string(), "  SS      ");

    let word = Word::new("naïve");
    assert_eq!(word.decoded, "naïve");
    assert_eq!(word.normalized, "naive");
    assert_eq!(word.upper, "NAIVE");
    assert_eq!(word.length, 5);
    assert_eq!(word.buffer_string(), "  NAIVE      ");
}

#[test]
fn test_is_slavo_germanic() {
    let word = Word::new("Berkowitz");
    assert!(word.is_slavo_germanic());
    let word = Word::new("Czeck");
    assert!(word.is_slavo_germanic());
    let word = Word::new("Bob");
    assert!(!word.is_slavo_germanic());
}

#[test]
fn test_get_first_letter() {
    let word = Word::new("naïve");
    assert_eq!(word.get_letters(0, None), "N");
    assert_eq!(word.get_letters(0, Some(1)), "N");
}

#[test]
fn test_first_2_letters() {
    let word = Word::new("naïve");
    assert_eq!(word.get_letters(0, Some(2)), "NA");
}

#[test]
fn test_first_3_letters() {
    let word = Word::new("naïve");
    assert_eq!(word.get_letters(0, Some(3)), "NAI");
}

#[test]
fn test_get_4th_letter() {
    let word = Word::new("naïve");
    assert_eq!(word.get_letters(3, None), "V");
}

// Edge case tests added beyond Python suite:
#[test]
fn test_empty_string_edge_case() {
    let word = Word::new("");
    assert_eq!(word.original, "");
    assert_eq!(word.normalized, "");
    assert_eq!(word.length, 0);
    assert_eq!(word.buffer_string(), "        ");
}

#[test]
fn test_out_of_bounds_slice_edge_case() {
    let word = Word::new("hi");
    assert_eq!(word.get_letters(100, None), "");
    assert_eq!(word.at(100), '\0');
}
