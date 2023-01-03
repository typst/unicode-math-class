//! Determine the Unicode class of a mathematical character.
//!
//! # Example
//! ```
//! use unicode_math_class::{class, MathClass};
//!
//! assert_eq!(class('0'), Some(MathClass::Normal));
//! assert_eq!(class('a'), Some(MathClass::Alphabetic));
//! assert_eq!(class('𝔸'), Some(MathClass::Alphabetic));
//! assert_eq!(class('+'), Some(MathClass::Vary));
//! assert_eq!(class('×'), Some(MathClass::Binary));
//! assert_eq!(class('('), Some(MathClass::Opening));
//! assert_eq!(class(','), Some(MathClass::Punctuation));
//! assert_eq!(class('|'), Some(MathClass::Fence));
//! assert_eq!(class('😃'), None);
//! ```
//!
//! For more details, see [Section 5.1 of Unicode Technical Report #25][report]
//! and [this data file][data].
//!
//! [report]: https://www.unicode.org/reports/tr25/tr25-15.pdf
//! [data]: https://www.unicode.org/Public/math/revision-15/MathClass-15.txt

use MathClass::*;

/// The revision of the used data file.
///
/// This crate does not specify a Unicode version because the math classes are
/// not formally part of the Unicode character database.
pub const REVISION: u8 = 15;

/// Determine the class of a mathematical character.
///
/// Returns `None` if the character isn't part of any class.
pub fn class(c: char) -> Option<MathClass> {
    let i = CLASSES.binary_search_by_key(&c, |pair| pair.0).ok()?;
    Some(CLASSES[i].1)
}

/// Classification of a mathematical character.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum MathClass {
    Normal,
    Alphabetic,
    Binary,
    Closing,
    Diacritic,
    Fence,
    GlyphPart,
    Large,
    Opening,
    Punctuation,
    Relation,
    Space,
    Unary,
    Vary,
    Special,
}

include!("classes.rs");
