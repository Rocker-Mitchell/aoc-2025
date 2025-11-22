//! Utility functions for parsing input.

use crate::{ParseError, ParseResult};

/// Parse lines with a closure, wrapping any [`ParseError`] in a
/// [`ParseError::InvalidLine`] error. Allows specifying an offset for line
/// numbering.
///
/// This function is useful when parsing inputs that are chunks of a larger
/// input, where line numbers need to reflect their position in the full input.
///
/// # Arguments
/// * `input` - The input string to parse.
/// * `offset` - The offset to add to line indices.
/// * `parser` - A closure that takes a line and returns a [`ParseResult`].
///
/// # Errors
///
/// If parsing any line fails, a [`ParseError::InvalidLine`] error is returned,
/// wrapping the original error and indicating the line number (with offset
/// applied).
///
/// # Examples
///
/// ```
/// use aoc_framework::{ParseError, ParseResult};
/// use aoc_framework::util::parse::parse_lines_with_offset;
///
/// let input = "Ignore this header line\n\n42\n100\n";
/// let chunks: Vec<_> = input.split("\n\n").collect();
/// let header_chunk = chunks.get(0).unwrap();
/// let data_chunk = chunks.get(1).unwrap();
/// // +1 for the empty line separator
/// let data_offset = header_chunk.lines().count() + 1;
///
/// let parsed: Vec<u32> =
///     parse_lines_with_offset(data_chunk, data_offset, |line| {
///         line.parse::<u32>()
///             .map_err(|source| ParseError::parse_int_from_str(line, source))
///     })
///     .collect::<ParseResult<_>>()
///     .unwrap();
/// assert_eq!(parsed, vec![42, 100]);
/// ```
pub fn parse_lines_with_offset<T, F>(
    input: &str,
    offset: usize,
    mut parser: F,
) -> impl Iterator<Item = ParseResult<T>>
where
    F: FnMut(&str) -> ParseResult<T>,
{
    input.lines().enumerate().map(move |(i, line)| {
        parser(line).map_err(|source| {
            ParseError::invalid_line_from_zero_index(
                i.saturating_add(offset),
                source,
            )
        })
    })
}

/// Parse lines with a closure, wrapping any [`ParseError`] in a
/// [`ParseError::InvalidLine`] error.
///
/// This is a convenience wrapper around [`parse_lines_with_offset`] with
/// `offset` = 0. Use this for simple, non-chunked inputs.
///
/// # Arguments
/// * `input` - The input string to parse.
/// * `parser` - A closure that takes a line and returns a [`ParseResult`].
///
/// # Errors
///
/// If parsing any line fails, a [`ParseError::InvalidLine`] error is returned,
/// wrapping the original error and indicating the line number.
///
/// # Examples
///
/// ```
/// use aoc_framework::{ParseError, ParseResult};
/// use aoc_framework::util::parse::parse_lines;
///
/// let input = "10\n20\n30\n";
/// let parsed: Vec<u32> = parse_lines(input, |line| {
///     line.parse::<u32>()
///         .map_err(|source| ParseError::parse_int_from_str(line, source))
/// })
/// .collect::<ParseResult<_>>()
/// .unwrap();
/// assert_eq!(parsed, vec![10, 20, 30]);
/// ```
pub fn parse_lines<T, F>(
    input: &str,
    parser: F,
) -> impl Iterator<Item = ParseResult<T>>
where
    F: FnMut(&str) -> ParseResult<T>,
{
    parse_lines_with_offset(input, 0, parser)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_lines_with_offset_successfully() -> ParseResult<()> {
        let input = "100\n200\n300\n";
        let offset = 5;
        let parsed: Vec<u32> = parse_lines_with_offset(input, offset, |line| {
            line.parse::<u32>()
                .map_err(|source| ParseError::parse_int_from_str(line, source))
        })
        .collect::<ParseResult<_>>()?;
        assert_eq!(parsed, vec![100, 200, 300]);
        Ok(())
    }

    #[test]
    fn parse_lines_with_offset_errors() {
        let input = "100\nabc\n200\n";
        let offset = 5;
        let result: ParseResult<Vec<u32>> =
            parse_lines_with_offset(input, offset, |line| {
                line.parse::<u32>().map_err(|source| {
                    ParseError::parse_int_from_str(line, source)
                })
            })
            .collect();
        assert!(result.is_err(), "expected parse to fail");
        match result.unwrap_err() {
            ParseError::InvalidLine { line, source } => {
                assert_eq!(line, 7, "expected failure on line 7");
                match *source {
                    ParseError::ParseInt { ref string, .. } => {
                        assert_eq!(string, "abc");
                    }
                    not_parse_int => {
                        panic!(
                            "unexpected source error type: {not_parse_int:?}"
                        );
                    }
                }
            }
            not_invalid_line => {
                panic!("unexpected error type: {not_invalid_line:?}");
            }
        }
    }

    #[test]
    fn parse_lines_uses_offset_zero() {
        let input = "10\nbad\n20\n";
        let result: ParseResult<Vec<u32>> = parse_lines(input, |line| {
            line.parse::<u32>()
                .map_err(|source| ParseError::parse_int_from_str(line, source))
        })
        .collect();
        assert!(result.is_err(), "expected parse to fail");
        match result.unwrap_err() {
            ParseError::InvalidLine { line, source } => {
                assert_eq!(line, 2, "expected failure on line 2");
                match *source {
                    ParseError::ParseInt { ref string, .. } => {
                        assert_eq!(string, "bad");
                    }
                    not_parse_int => {
                        panic!(
                            "unexpected source error type: {not_parse_int:?}"
                        );
                    }
                }
            }
            not_invalid_line => {
                panic!("unexpected error type: {not_invalid_line:?}");
            }
        }
    }
}
