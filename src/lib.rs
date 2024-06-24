#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

pub mod util;
#[cfg(feature = "z3")]
pub mod z3;

/// Endianness used during conversions
pub enum Endianness {
    /// Little endian
    Little,
    /// Big endian
    Big,
}
