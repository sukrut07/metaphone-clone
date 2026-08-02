//! Constants used throughout the Double Metaphone phonetic encoding engine.

/// Vowel characters used for initial vowel checks and contextual consonant rules.
pub const VOWELS: &[char] = &['A', 'E', 'I', 'O', 'U', 'Y'];

/// Silent consonant pairs at the start of a word (e.g. "GN", "KN", "PN", "WR", "PS").
pub const SILENT_STARTERS: &[&str] = &["GN", "KN", "PN", "WR", "PS"];

/// Returns whether a given character is considered a vowel in the Metaphone rules.
#[inline]
pub fn is_vowel(c: char) -> bool {
    VOWELS.contains(&c)
}
