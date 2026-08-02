//! Word preprocessing, Unicode normalization, padding, and slice indexing buffer.

use crate::constants::is_vowel;
use unicode_categories::UnicodeCategories;
use unicode_normalization::UnicodeNormalization;

/// Encapsulates string normalization, diacritic stripping, and a prepadded/postpadded character buffer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Word {
    /// Original raw input string.
    pub original: String,
    /// String after initial character replacements (e.g. C-cedilla `Ç`/`ç` -> `s`).
    pub decoded: String,
    /// String after NFD Unicode decomposition and Non-Spacing Mark (`Mn`) removal.
    pub normalized: String,
    /// Normalized string converted to uppercase.
    pub upper: String,
    /// Length of `upper` in Unicode scalar values (`char`).
    pub length: usize,
    /// Prepad string prefix (`"  "`).
    pub prepad: &'static str,
    /// Starting character index of the actual word inside `buffer`.
    pub start_index: usize,
    /// Ending character index of the actual word inside `buffer`.
    pub end_index: usize,
    /// Postpad string suffix (`"      "`).
    pub postpad: &'static str,
    /// Prepared vector of characters allowing $O(1)$ bounds-checked slice lookups.
    pub buffer: Vec<char>,
    /// Pre-computed Slavo-Germanic origin flag for $O(1)$ checks.
    pub slavo_germanic: bool,
}

impl Word {
    /// Preprocesses a raw input string into a normalized, padded `Word` instance.
    ///
    /// # Performance
    /// - Fast-path for ASCII inputs: 0 intermediate String allocations.
    /// - NFD Unicode pipeline for non-ASCII inputs.
    /// - Pre-computes `slavo_germanic` flag to make subsequent checks $O(1)$.
    pub fn new(input: &str) -> Self {
        let original = input.to_string();
        let prepad = "  ";
        let postpad = "      ";
        let start_index = 2;

        if input.is_ascii() {
            let length = input.len();
            let end_index = if length > 0 {
                start_index + length - 1
            } else {
                start_index - 1
            };

            let upper = input.to_ascii_uppercase();
            let decoded = original.clone();
            let normalized = original.clone();

            let mut buffer = Vec::with_capacity(2 + length + 6);
            buffer.push(' ');
            buffer.push(' ');
            for &b in upper.as_bytes() {
                buffer.push(b as char);
            }
            buffer.extend([' '; 6]);

            let slavo_germanic = upper.contains('W')
                || upper.contains('K')
                || upper.contains("CZ")
                || upper.contains("WITZ");

            Self {
                original,
                decoded,
                normalized,
                upper,
                length,
                prepad,
                start_index,
                end_index,
                postpad,
                buffer,
                slavo_germanic,
            }
        } else {
            // Non-ASCII path: C-cedilla replacement + NFD decomposition
            let decoded = if input.contains(['Ç', 'ç']) {
                input.replace(['Ç', 'ç'], "s")
            } else {
                original.clone()
            };

            let normalized: String = decoded.nfd().filter(|c| !c.is_mark_nonspacing()).collect();

            let upper = normalized.to_uppercase();
            let length = upper.chars().count();
            let end_index = if length > 0 {
                start_index + length - 1
            } else {
                start_index - 1
            };

            let mut buffer = Vec::with_capacity(2 + length + 6);
            buffer.push(' ');
            buffer.push(' ');
            buffer.extend(upper.chars());
            buffer.extend([' '; 6]);

            let slavo_germanic = upper.contains('W')
                || upper.contains('K')
                || upper.contains("CZ")
                || upper.contains("WITZ");

            Self {
                original,
                decoded,
                normalized,
                upper,
                length,
                prepad,
                start_index,
                end_index,
                postpad,
                buffer,
                slavo_germanic,
            }
        }
    }

    /// Reconstructs the full padded buffer as a `String` (primarily for testing and inspection).
    pub fn buffer_string(&self) -> String {
        self.buffer.iter().collect()
    }

    /// Returns the pre-computed Slavo-Germanic origin heuristic ($O(1)$).
    #[inline]
    pub fn is_slavo_germanic(&self) -> bool {
        self.slavo_germanic
    }

    /// Extracts a relative character slice from `buffer` offset from `start_index`.
    pub fn get_letters(&self, start: usize, end: Option<usize>) -> String {
        let end_val = end.unwrap_or(start + 1);
        let abs_start = self.start_index + start;
        let abs_end = self.start_index + end_val;

        if abs_start >= self.buffer.len() {
            return String::new();
        }
        let real_end = abs_end.min(self.buffer.len());
        self.buffer[abs_start..real_end].iter().collect()
    }

    /// Returns the character at absolute `index` in `buffer`, or `'\0'` if out of bounds.
    #[inline]
    pub fn at(&self, index: usize) -> char {
        self.buffer.get(index).copied().unwrap_or('\0')
    }

    /// Returns `true` if `buffer[index] == c`.
    #[inline]
    pub fn is_at(&self, index: usize, c: char) -> bool {
        self.at(index) == c
    }

    /// Returns `true` if `buffer[index]` matches any character in `chars`.
    #[inline]
    pub fn in_at(&self, index: usize, chars: &[char]) -> bool {
        chars.contains(&self.at(index))
    }

    /// Returns `true` if `buffer[index]` is an English vowel.
    #[inline]
    pub fn is_vowel_at(&self, index: usize) -> bool {
        is_vowel(self.at(index))
    }

    /// Checks if the buffer sequence starting at absolute `start` equals ASCII string `pat`.
    #[inline]
    pub fn eq_at(&self, start: usize, pat: &str) -> bool {
        let pat_bytes = pat.as_bytes();
        let end = match start.checked_add(pat_bytes.len()) {
            Some(e) => e,
            None => return false,
        };
        if end > self.buffer.len() {
            return false;
        }
        let slice = &self.buffer[start..end];
        for (i, &b) in pat_bytes.iter().enumerate() {
            if slice[i] as u32 != b as u32 {
                return false;
            }
        }
        true
    }

    /// Checks if the buffer sequence starting at absolute `start` matches any ASCII pattern in `pats`.
    #[inline]
    pub fn in_strs_at(&self, start: usize, pats: &[&str]) -> bool {
        pats.iter().any(|pat| self.eq_at(start, pat))
    }

    /// Safely checks if the buffer sequence starting at `pos - back` equals ASCII string `pat`.
    #[inline]
    pub fn eq_at_back(&self, pos: usize, back: usize, pat: &str) -> bool {
        if let Some(start) = pos.checked_sub(back) {
            self.eq_at(start, pat)
        } else {
            false
        }
    }

    /// Safely checks if the buffer sequence starting at `pos - back` matches any ASCII pattern in `pats`.
    #[inline]
    pub fn in_strs_at_back(&self, pos: usize, back: usize, pats: &[&str]) -> bool {
        if let Some(start) = pos.checked_sub(back) {
            self.in_strs_at(start, pats)
        } else {
            false
        }
    }
}
