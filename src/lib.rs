//! Decimal
//!
//! `Decimal` allows storing real number to arbitrary precision; which
//! avoids common floating point errors (such as 0.1 + 0.2 ≠ 0.3) at the
//! cost of complexity.
//!
//! Internally, `Decimal` uses a 256-bit integer, paired with a 64-bit
//! integer which determines the position of the decimal point. Therefore,
//! the precision *is not* actually arbitrary, but limited to 2<sup>63</sup>
//! decimal places.
//!
//! Common numerical operations are overloaded, so we can treat them
//! the same way we treat other numbers.
//!
//! It is not recommended to convert a floating point number to a decimal
//! directly, as the floating point representation may be unexpected.
//!
//! # Example
//!
//! ```
//! use fdecimal::Decimal;
//! use std::str::FromStr;
//!
//! let input = "0.8";
//! let dec = Decimal::from_str(&input).unwrap();
//! let float = f32::from_str(&input).unwrap();
//!
//! println!("Input ({}) with decimals: {} vs {})", input, dec, float);
//! ```

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct U256([u64;4]);

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct U512([u64;8]);

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Decimal {
    // A 256-bit integer
    value: U256,
    // A positive scale means a negative power of 10
    scale: i64,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ParseDecimalError;

impl Display for ParseDecimalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "xxx")
    }
}

impl Display for Decimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "xxx")
    }
}

// #[cfg(feature = "std")]
impl std::error::Error for ParseDecimalError {
    fn description(&self) -> &str {
        "failed to parse decimal"
    }
}

use std::fmt::Display;
use std::str::FromStr;

impl FromStr for Decimal {
    type Err = ParseDecimalError;

    #[inline]
    fn from_str(_s: &str) -> Result<Decimal, ParseDecimalError> {
        Ok(Decimal {
            value: U256(Default::default()),
            scale: 0,
        })
    }
}