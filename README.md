# unicode-math-class
[![Crates.io](https://img.shields.io/crates/v/unicode-math-class.svg)](https://crates.io/crates/unicode-math-class)
[![Documentation](https://docs.rs/unicode-math-class/badge.svg)](https://docs.rs/unicode-math-class)

Determine the Unicode class of a mathematical character in Rust.

```toml
[dependencies]
unicode-math-class = "0.1"
```

## Example
```rust
use unicode_math_class::{class, MathClass};

assert_eq!(class('0'), Some(MathClass::Normal));
assert_eq!(class('a'), Some(MathClass::Alphabetic));
assert_eq!(class('𝔸'), Some(MathClass::Alphabetic));
assert_eq!(class('+'), Some(MathClass::Vary));
assert_eq!(class('×'), Some(MathClass::Binary));
assert_eq!(class('('), Some(MathClass::Opening));
assert_eq!(class(','), Some(MathClass::Punctuation));
assert_eq!(class('|'), Some(MathClass::Fence));
assert_eq!(class('😃'), None);
```

For more details, see [Section 5.1 of Unicode Technical Report #25][report]
and [this data file][data].

## License
This crate is dual-licensed under the MIT and Apache 2.0 licenses.

[report]: https://www.unicode.org/reports/tr25/tr25-15.pdf
[data]: https://www.unicode.org/Public/math/revision-15/MathClass-15.txt
