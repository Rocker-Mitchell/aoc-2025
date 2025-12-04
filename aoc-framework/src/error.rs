//! Error and result types for parsing inputs from Advent of Code.

use std::num::ParseIntError;

use thiserror::Error;

/// A return type for results related to input parsing.
pub type ParseResult<T> = core::result::Result<T, ParseError>;

/// An error with parsing input.
#[derive(Error, Debug)]
pub enum ParseError {
    /// The input received was empty.
    #[error("input was empty")]
    EmptyInput,

    /// The input contains an unexpected empty line.
    #[error("line was empty")]
    EmptyLine,

    /// The input contains a line with an unexpected length.
    #[error("incorrect line length: expected {expected}, got {actual}")]
    LineLength {
        /// The expected length.
        expected: usize,
        /// The actual length.
        actual: usize,
    },

    /// An invalid character was parsed.
    #[error("invalid character: {0:?}")]
    ParseChar(char),

    /// Failed to parse string into an integer.
    #[error("failed to parse string into integer: {string:?}")]
    ParseInt {
        /// The string that failed to parse.
        string: String,
        source: ParseIntError,
    },

    /// A line in the input caused a parsing error.
    #[error("failure parsing line {line}")]
    InvalidLine {
        /// The line number. This should be one-indexed (the first line is 1).
        line: usize,
        source: Box<ParseError>,
    },
}

impl ParseError {
    /// Create a parse int error from a string slice and source error.
    #[must_use]
    pub fn parse_int_from_str(string: &str, source: ParseIntError) -> Self {
        Self::ParseInt {
            string: String::from(string),
            source,
        }
    }

    /// Create an invalid line error from a zero-based line index and source
    /// error.
    #[must_use]
    pub fn invalid_line_from_zero_index(index: usize, source: Self) -> Self {
        Self::invalid_line_from_one_based(index.saturating_add(1), source)
    }

    /// Create an invalid line error from a one-based line number and source
    /// error.
    #[must_use]
    pub fn invalid_line_from_one_based(line: usize, source: Self) -> Self {
        Self::InvalidLine {
            line,
            source: Box::new(source),
        }
    }
}
