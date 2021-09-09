//! Contains CountryError, an error type for Country (ISO 3166-1) conversion 
//! and parsing.

use std::error::Error;
use std::fmt::Formatter;
use std::fmt::{Debug, Display};

/// Errors for converting Alpha-2/-3 & Numeric codes into Country.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CountryError {
    /// The input str len didn't match what the alpha-X code parser expected.
    UnexpectedLen { 
        /// Length of input &str.
        was: usize, 
        /// Expected length.
        expected: usize 
    },
    /// The input alpha-2 code was not recognized as a known ISO 3166-1 code.
    InvalidA2 { 
        /// Input alpha-2 code, which was not recognized.
        was: String 
    },
    /// The input alpha-3 code was not recognized as a known ISO 3166-1 code.
    InvalidA3 { 
        /// Input alpha-3 code, which was not recognized.
        was: String 
    },
    /// The input numeric code was not recognized as a known ISO 3166-1 code.
    InvalidNum { 
        /// Input numeric code, which was not recognized.
        was: u32 
    },
}

// Display for CFIError converts all u8 into chars to make it more humanly 
// readable.

impl Display for CountryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CountryError::UnexpectedLen { was, expected } => {
                write!(f, "unexpected length of input `{}`, expected `{}`", 
                    was, expected)
            },
            CountryError::InvalidA2 { was } => {
                write!(f, "invalid alpha-2 code: `{}`", was)
            },
            CountryError::InvalidA3 { was } => {
                write!(f, "invalid alpha-3 code: `{}`", was)
            },
            CountryError::InvalidNum { was } => {
                write!(f, "invalid numeric code: `{}`", was)
            },
        }
    }
}

impl Error for CountryError {}