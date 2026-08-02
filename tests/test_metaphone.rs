//! 1:1 Port of Python `metaphone/tests/test_metaphone.py` test suite.

use metaphone::{dm, double_metaphone};
use std::collections::HashSet;

#[test]
fn test_single_result() {
    let result = double_metaphone("aubrey");
    assert_eq!(result, ("APR".to_string(), "".to_string()));
}

#[test]
fn test_double_result() {
    let result = double_metaphone("richard");
    assert_eq!(result, ("RXRT".to_string(), "RKRT".to_string()));
}

#[test]
fn test_general_word_list() {
    assert_eq!(double_metaphone("Jose"), ("HS".to_string(), "".to_string()));
    assert_eq!(
        double_metaphone("cambrillo"),
        ("KMPRL".to_string(), "KMPR".to_string())
    );
    assert_eq!(double_metaphone("otto"), ("AT".to_string(), "".to_string()));
    assert_eq!(
        double_metaphone("aubrey"),
        ("APR".to_string(), "".to_string())
    );
    assert_eq!(
        double_metaphone("maurice"),
        ("MRS".to_string(), "".to_string())
    );
    assert_eq!(double_metaphone("auto"), ("AT".to_string(), "".to_string()));
    assert_eq!(
        double_metaphone("maisey"),
        ("MS".to_string(), "".to_string())
    );
    assert_eq!(
        double_metaphone("catherine"),
        ("K0RN".to_string(), "KTRN".to_string())
    );
    assert_eq!(
        double_metaphone("geoff"),
        ("JF".to_string(), "KF".to_string())
    );
    assert_eq!(
        double_metaphone("Chile"),
        ("XL".to_string(), "".to_string())
    );
    assert_eq!(
        double_metaphone("katherine"),
        ("K0RN".to_string(), "KTRN".to_string())
    );
    assert_eq!(
        double_metaphone("steven"),
        ("STFN".to_string(), "".to_string())
    );
    assert_eq!(
        double_metaphone("zhang"),
        ("JNK".to_string(), "".to_string())
    );
    assert_eq!(double_metaphone("bob"), ("PP".to_string(), "".to_string()));
    assert_eq!(double_metaphone("ray"), ("R".to_string(), "".to_string()));
    assert_eq!(double_metaphone("Tux"), ("TKS".to_string(), "".to_string()));
    assert_eq!(
        double_metaphone("bryan"),
        ("PRN".to_string(), "".to_string())
    );
    assert_eq!(
        double_metaphone("bryce"),
        ("PRS".to_string(), "".to_string())
    );
    assert_eq!(
        double_metaphone("Rapelje"),
        ("RPL".to_string(), "".to_string())
    );
    assert_eq!(
        double_metaphone("richard"),
        ("RXRT".to_string(), "RKRT".to_string())
    );
    assert_eq!(
        double_metaphone("solilijs"),
        ("SLLS".to_string(), "".to_string())
    );
    assert_eq!(
        double_metaphone("Dallas"),
        ("TLS".to_string(), "".to_string())
    );
    assert_eq!(
        double_metaphone("Schwein"),
        ("XN".to_string(), "XFN".to_string())
    );
    assert_eq!(double_metaphone("dave"), ("TF".to_string(), "".to_string()));
    assert_eq!(
        double_metaphone("eric"),
        ("ARK".to_string(), "".to_string())
    );
    assert_eq!(
        double_metaphone("Parachute"),
        ("PRKT".to_string(), "".to_string())
    );
    assert_eq!(
        double_metaphone("brian"),
        ("PRN".to_string(), "".to_string())
    );
    assert_eq!(
        double_metaphone("randy"),
        ("RNT".to_string(), "".to_string())
    );
    assert_eq!(
        double_metaphone("Through"),
        ("0R".to_string(), "TR".to_string())
    );
    assert_eq!(
        double_metaphone("Nowhere"),
        ("NR".to_string(), "".to_string())
    );
    assert_eq!(
        double_metaphone("heidi"),
        ("HT".to_string(), "".to_string())
    );
    assert_eq!(
        double_metaphone("Arnow"),
        ("ARN".to_string(), "ARNF".to_string())
    );
    assert_eq!(
        double_metaphone("Thumbail"),
        ("0MPL".to_string(), "TMPL".to_string())
    );
}

#[test]
fn test_homophones() {
    assert_eq!(double_metaphone("tolled"), double_metaphone("told"));
    assert_eq!(double_metaphone("katherine"), double_metaphone("catherine"));
    assert_eq!(double_metaphone("brian"), double_metaphone("bryan"));
}

#[test]
fn test_similar_names() {
    assert_eq!(
        double_metaphone("Bartoš"),
        ("PRTS".to_string(), "".to_string())
    );
    assert_eq!(
        double_metaphone("Bartosz"),
        ("PRTS".to_string(), "PRTX".to_string())
    );
    assert_eq!(
        double_metaphone("Bartosch"),
        ("PRTX".to_string(), "".to_string())
    );
    assert_eq!(
        double_metaphone("Bartos"),
        ("PRTS".to_string(), "".to_string())
    );

    let (p1, s1) = double_metaphone("Jablonski");
    let (p2, s2) = double_metaphone("Yablonsky");
    let set1: HashSet<String> = [p1, s1].into_iter().filter(|s| !s.is_empty()).collect();
    let set2: HashSet<String> = [p2, s2].into_iter().filter(|s| !s.is_empty()).collect();
    let intersection: Vec<_> = set1.intersection(&set2).cloned().collect();
    assert_eq!(intersection, vec!["APLNSK".to_string()]);

    let (p1, s1) = double_metaphone("Smith");
    let (p2, s2) = double_metaphone("Schmidt");
    let set1: HashSet<String> = [p1, s1].into_iter().filter(|s| !s.is_empty()).collect();
    let set2: HashSet<String> = [p2, s2].into_iter().filter(|s| !s.is_empty()).collect();
    let intersection: Vec<_> = set1.intersection(&set2).cloned().collect();
    assert_eq!(intersection, vec!["XMT".to_string()]);
}

#[test]
fn test_non_english_unicode() {
    assert_eq!(
        double_metaphone("andestādītu"),
        ("ANTSTTT".to_string(), "".to_string())
    );
}

#[test]
fn test_c_cedilla() {
    assert_eq!(
        double_metaphone("français"),
        ("FRNS".to_string(), "FRNSS".to_string())
    );
    assert_eq!(
        double_metaphone("garçon"),
        ("KRSN".to_string(), "".to_string())
    );
    assert_eq!(
        double_metaphone("leçon"),
        ("LSN".to_string(), "".to_string())
    );
}

#[test]
fn test_various_german() {
    assert_eq!(double_metaphone("ach"), ("AK".to_string(), "".to_string()));
    assert_eq!(
        double_metaphone("bacher"),
        ("PKR".to_string(), "".to_string())
    );
    assert_eq!(
        double_metaphone("macher"),
        ("MKR".to_string(), "".to_string())
    );
}

#[test]
fn test_various_italian() {
    assert_eq!(
        double_metaphone("bacci"),
        ("PX".to_string(), "".to_string())
    );
    assert_eq!(
        double_metaphone("bertucci"),
        ("PRTX".to_string(), "".to_string())
    );
    assert_eq!(
        double_metaphone("bellocchio"),
        ("PLX".to_string(), "".to_string())
    );
    assert_eq!(
        double_metaphone("bacchus"),
        ("PKS".to_string(), "".to_string())
    );
    assert_eq!(
        double_metaphone("focaccia"),
        ("FKX".to_string(), "".to_string())
    );
    assert_eq!(
        double_metaphone("chianti"),
        ("KNT".to_string(), "".to_string())
    );
    assert_eq!(
        double_metaphone("tagliaro"),
        ("TKLR".to_string(), "TLR".to_string())
    );
    assert_eq!(
        double_metaphone("biaggi"),
        ("PJ".to_string(), "PK".to_string())
    );
}

#[test]
fn test_various_spanish() {
    assert_eq!(
        double_metaphone("bajador"),
        ("PJTR".to_string(), "PHTR".to_string())
    );
    assert_eq!(
        double_metaphone("cabrillo"),
        ("KPRL".to_string(), "KPR".to_string())
    );
    assert_eq!(
        double_metaphone("gallegos"),
        ("KLKS".to_string(), "KKS".to_string())
    );
    assert_eq!(
        double_metaphone("San Jacinto"),
        ("SNHSNT".to_string(), "".to_string())
    );
}

#[test]
fn test_various_french() {
    assert_eq!(
        double_metaphone("rogier"),
        ("RJ".to_string(), "RJR".to_string())
    );
    assert_eq!(
        double_metaphone("breaux"),
        ("PR".to_string(), "".to_string())
    );
}

#[test]
fn test_various_slavic() {
    assert_eq!(
        double_metaphone("Wewski"),
        ("ASK".to_string(), "FFSK".to_string())
    );
}

#[test]
fn test_various_chinese() {
    assert_eq!(double_metaphone("zhao"), ("J".to_string(), "".to_string()));
}

#[test]
fn test_dutch_origin() {
    assert_eq!(
        double_metaphone("school"),
        ("SKL".to_string(), "".to_string())
    );
    assert_eq!(
        double_metaphone("schooner"),
        ("SKNR".to_string(), "".to_string())
    );
    assert_eq!(
        double_metaphone("schermerhorn"),
        ("XRMRRN".to_string(), "SKRMRRN".to_string())
    );
    assert_eq!(
        double_metaphone("schenker"),
        ("XNKR".to_string(), "SKNKR".to_string())
    );
}

#[test]
fn test_ch_words() {
    assert_eq!(
        double_metaphone("Charac"),
        ("KRK".to_string(), "".to_string())
    );
    assert_eq!(
        double_metaphone("Charis"),
        ("KRS".to_string(), "".to_string())
    );
    assert_eq!(
        double_metaphone("chord"),
        ("KRT".to_string(), "".to_string())
    );
    assert_eq!(double_metaphone("Chym"), ("KM".to_string(), "".to_string()));
    assert_eq!(double_metaphone("Chia"), ("K".to_string(), "".to_string()));
    assert_eq!(double_metaphone("chem"), ("KM".to_string(), "".to_string()));
    assert_eq!(
        double_metaphone("chore"),
        ("XR".to_string(), "".to_string())
    );
    assert_eq!(
        double_metaphone("orchestra"),
        ("ARKSTR".to_string(), "".to_string())
    );
    assert_eq!(
        double_metaphone("architect"),
        ("ARKTKT".to_string(), "".to_string())
    );
    assert_eq!(
        double_metaphone("orchid"),
        ("ARKT".to_string(), "".to_string())
    );
}

#[test]
fn test_cc_words() {
    assert_eq!(
        double_metaphone("accident"),
        ("AKSTNT".to_string(), "".to_string())
    );
    assert_eq!(
        double_metaphone("accede"),
        ("AKST".to_string(), "".to_string())
    );
    assert_eq!(
        double_metaphone("succeed"),
        ("SKST".to_string(), "".to_string())
    );
}

#[test]
fn test_mc_words() {
    assert_eq!(
        double_metaphone("mac caffrey"),
        ("MKFR".to_string(), "".to_string())
    );
    assert_eq!(
        double_metaphone("mac gregor"),
        ("MKRKR".to_string(), "".to_string())
    );
    assert_eq!(
        double_metaphone("mc crae"),
        ("MKR".to_string(), "".to_string())
    );
    assert_eq!(
        double_metaphone("mcclain"),
        ("MKLN".to_string(), "".to_string())
    );
}

#[test]
fn test_gh_words() {
    assert_eq!(
        double_metaphone("laugh"),
        ("LF".to_string(), "".to_string())
    );
    assert_eq!(
        double_metaphone("cough"),
        ("KF".to_string(), "".to_string())
    );
    assert_eq!(
        double_metaphone("rough"),
        ("RF".to_string(), "".to_string())
    );
}

#[test]
fn test_g3_words() {
    assert_eq!(double_metaphone("gya"), ("K".to_string(), "J".to_string()));
    assert_eq!(
        double_metaphone("ges"),
        ("KS".to_string(), "JS".to_string())
    );
    assert_eq!(
        double_metaphone("gep"),
        ("KP".to_string(), "JP".to_string())
    );
    assert_eq!(
        double_metaphone("geb"),
        ("KP".to_string(), "JP".to_string())
    );
    assert_eq!(
        double_metaphone("gel"),
        ("KL".to_string(), "JL".to_string())
    );
    assert_eq!(double_metaphone("gey"), ("K".to_string(), "J".to_string()));
    assert_eq!(
        double_metaphone("gib"),
        ("KP".to_string(), "JP".to_string())
    );
    assert_eq!(
        double_metaphone("gil"),
        ("KL".to_string(), "JL".to_string())
    );
    assert_eq!(
        double_metaphone("gin"),
        ("KN".to_string(), "JN".to_string())
    );
    assert_eq!(double_metaphone("gie"), ("K".to_string(), "J".to_string()));
    assert_eq!(double_metaphone("gei"), ("K".to_string(), "J".to_string()));
    assert_eq!(
        double_metaphone("ger"),
        ("KR".to_string(), "JR".to_string())
    );
    assert_eq!(
        double_metaphone("danger"),
        ("TNJR".to_string(), "TNKR".to_string())
    );
    assert_eq!(
        double_metaphone("manager"),
        ("MNKR".to_string(), "MNJR".to_string())
    );
    assert_eq!(
        double_metaphone("dowager"),
        ("TKR".to_string(), "TJR".to_string())
    );
}

#[test]
fn test_pb_words() {
    assert_eq!(
        double_metaphone("Campbell"),
        ("KMPL".to_string(), "".to_string())
    );
    assert_eq!(
        double_metaphone("raspberry"),
        ("RSPR".to_string(), "".to_string())
    );
}

#[test]
fn test_th_words() {
    assert_eq!(
        double_metaphone("Thomas"),
        ("TMS".to_string(), "".to_string())
    );
    assert_eq!(
        double_metaphone("Thames"),
        ("TMS".to_string(), "".to_string())
    );
}

#[test]
fn test_alias() {
    assert_eq!(dm("aubrey"), ("APR".to_string(), "".to_string()));
}

// Edge cases beyond Python test suite:
#[test]
fn test_empty_input_edge_case() {
    assert_eq!(double_metaphone(""), ("".to_string(), "".to_string()));
}

#[test]
fn test_whitespace_only_edge_case() {
    assert_eq!(double_metaphone("    "), ("".to_string(), "".to_string()));
}

#[test]
fn test_punctuation_and_numbers() {
    assert_eq!(
        double_metaphone("123-abc"),
        ("PK".to_string(), "".to_string())
    );
}
