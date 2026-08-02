//! # Metaphone
//!
//! A fast, safe, and idiomatic Rust implementation of the Metaphone and Double Metaphone phonetic encoding algorithms.
//!
//! ## Quick Start
//!
//! ```rust
//! use metaphone::{double_metaphone, dm};
//!
//! // Compute primary and secondary Double Metaphone codes
//! let (primary, secondary) = double_metaphone("richard");
//! assert_eq!(primary, "RXRT");
//! assert_eq!(secondary, "RKRT");
//!
//! // Use short alias `dm`
//! let (p, s) = dm("aubrey");
//! assert_eq!(p, "APR");
//! assert_eq!(s, "");
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod constants;
pub mod double_metaphone;
pub mod error;
pub mod word;

pub use double_metaphone::DoubleMetaphone;
pub use error::MetaphoneError;
pub use word::Word;

/// Computes the Double Metaphone primary and secondary phonetic codes for a given string.
///
/// Accepts any type implementing `AsRef<str>` (e.g. `&str`, `String`, `&String`, `Cow<str>`).
///
/// Returns a 2-tuple `(primary_code, secondary_code)`. If the secondary code is identical
/// to the primary code, the second tuple element will be an empty string `""`.
///
/// # Examples
///
/// ```rust
/// use metaphone::double_metaphone;
///
/// let (primary, secondary) = double_metaphone("catherine");
/// assert_eq!(primary, "K0RN");
/// assert_eq!(secondary, "KTRN");
/// ```
pub fn double_metaphone(input: impl AsRef<str>) -> (String, String) {
    DoubleMetaphone::parse(input.as_ref())
}

/// Short alias for [`double_metaphone`].
///
/// # Examples
///
/// ```rust
/// use metaphone::dm;
///
/// let (primary, secondary) = dm("Jose");
/// assert_eq!(primary, "HS");
/// assert_eq!(secondary, "");
/// ```
pub fn dm(input: impl AsRef<str>) -> (String, String) {
    double_metaphone(input)
}
