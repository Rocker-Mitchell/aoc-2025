//! Utility functions for parsing input.

use aoc_framework::{ParseError, ParseResult};
use nalgebra::{DMatrix, Scalar};

use crate::util::matrix::{MatrixPoint, matrix_point_from_usize};

/// Parse lines with a closure, wrapping any [`ParseError`] in a
/// [`ParseError::InvalidLine`] error. Allows specifying an offset for line
/// numbering.
///
/// This function is useful when parsing inputs that are chunks of a larger
/// input, where line numbers need to reflect their position in the full input.
///
/// # Arguments
/// - `input` - The input string to parse.
/// - `offset` - The offset to add to line indices.
/// - `parser` - A closure that takes a line and returns a [`ParseResult`].
///
/// # Errors
///
/// If parsing any line fails, a [`ParseError::InvalidLine`] error is returned,
/// wrapping the original error and indicating the line number (with offset
/// applied).
///
/// # Examples
///
/// ```ignore
/// use aoc_framework::{ParseError, ParseResult};
/// use crate::util::parse::parse_lines_with_offset;
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
/// - `input` - The input string to parse.
/// - `parser` - A closure that takes a line and returns a [`ParseResult`].
///
/// # Errors
///
/// If parsing any line fails, a [`ParseError::InvalidLine`] error is returned,
/// wrapping the original error and indicating the line number.
pub fn parse_lines<T, F>(
    input: &str,
    parser: F,
) -> impl Iterator<Item = ParseResult<T>>
where
    F: FnMut(&str) -> ParseResult<T>,
{
    parse_lines_with_offset(input, 0, parser)
}

/// Parse a character grid with a closure, wrapping any [`ParseError`] in a
/// [`ParseError::InvalidLine`] error. Allows specifying an offset for line
/// numbering.
///
/// This function is useful when parsing inputs that are chunks of a larger
/// input, where line numbers need to reflect their position in the full input.
///
/// # Arguments
/// - `input` - The input string to parse.
/// - `offset` - The offset to add to line indices.
/// - `parser` - A closure that takes a grid position & character, and returns
///   a [`ParseResult`]. Position considers top-left as origin, x-axis along
///   columns, and y-axis along rows.
///
/// # Errors
///
/// If the input has no lines, a [`ParseError::EmptyInput`] error is returned.
///
/// If any line is empty, a [`ParseError::EmptyLine`] error is created and
/// wrapped in a [`ParseError::InvalidLine`] for return.
///
/// If a line length does not match the first line length, a
/// [`ParseError::LineLength`] error is created and wrapped in a
/// [`ParseError::InvalidLine`] for return.
///
/// If parsing any character fails, a [`ParseError::InvalidLine`] error is
/// returned, wrapping the original error.
///
/// For all [`ParseError::InvalidLine`], the line number will have `offset`
/// applied.
pub fn parse_grid_with_offset<T, F>(
    input: &str,
    offset: usize,
    mut parser: F,
) -> ParseResult<DMatrix<T>>
where
    T: Scalar,
    F: FnMut(MatrixPoint, char) -> ParseResult<T>,
{
    let lines: Vec<_> = input.lines().collect();

    let rows = lines.len();
    if rows == 0 {
        return Err(ParseError::EmptyInput);
    }

    let cols = lines.first().map_or(0, |l| l.len());

    let mut values: Vec<T> = Vec::with_capacity(rows.saturating_mul(cols));

    for (y, &line) in lines.iter().enumerate() {
        if line.is_empty() {
            return Err(ParseError::invalid_line_from_zero_index(
                y.saturating_add(offset),
                ParseError::EmptyLine,
            ));
        }
        if line.len() != cols {
            return Err(ParseError::invalid_line_from_zero_index(
                y.saturating_add(offset),
                ParseError::LineLength {
                    expected: cols,
                    actual: line.len(),
                },
            ));
        }

        for (x, character) in line.char_indices() {
            let position: MatrixPoint = matrix_point_from_usize(x, y);
            match parser(position, character) {
                Ok(v) => values.push(v),
                Err(source) => {
                    return Err(ParseError::invalid_line_from_zero_index(
                        y.saturating_add(offset),
                        source,
                    ));
                }
            }
        }
    }

    Ok(DMatrix::from_row_iterator(rows, cols, values))
}

/// Parse a character grid with a closure, wrapping any [`ParseError`] in a
/// [`ParseError::InvalidLine`] error.
///
/// This is a convenience wrapper around [`parse_grid_with_offset`] with
/// `offset` = 0. Use this for simple, non-chunked inputs.
///
/// # Arguments
/// - `input` - The input string to parse.
/// - `parser` - A closure that takes a grid position & character, and returns
///   a [`ParseResult`]. Position considers top-left as origin, x-axis along
///   columns, and y-axis along rows.
///
/// # Errors
///
/// If the input has no lines, a [`ParseError::EmptyInput`] error is returned.
///
/// If any line is empty, a [`ParseError::EmptyLine`] error is created and
/// wrapped in a [`ParseError::InvalidLine`] for return.
///
/// If a line length does not match the first line length, a
/// [`ParseError::LineLength`] error is created and wrapped in a
/// [`ParseError::InvalidLine`] for return.
///
/// If parsing any character fails, a [`ParseError::InvalidLine`] error is
/// returned, wrapping the original error.
pub fn parse_grid<T, F>(input: &str, parser: F) -> ParseResult<DMatrix<T>>
where
    T: Scalar,
    F: FnMut(MatrixPoint, char) -> ParseResult<T>,
{
    parse_grid_with_offset(input, 0, parser)
}

#[cfg(test)]
mod tests {
    use nalgebra::Matrix4x3;

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
    fn parse_lines_with_offset_creates_error_from_parser() {
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

    #[test]
    fn parse_grid_with_offset_successfully() -> ParseResult<()> {
        let input = "..-\n.--\n-..\n-.-\n";
        let offset = 3;
        let parsed =
            parse_grid_with_offset(input, offset, |_position, character| {
                Ok(character == '-')
            })?;
        assert_eq!(
            parsed,
            Matrix4x3::new(
                false, false, true, false, true, true, true, false, false,
                true, false, true
            )
        );
        Ok(())
    }

    #[test]
    fn parse_grid_with_offset_passes_correct_sequence() -> ParseResult<()> {
        let input = "ab\ncd\n";
        let offset = 0;
        let mut positions = Vec::new();
        let mut characters = Vec::new();

        parse_grid_with_offset(input, offset, |position, character| {
            positions.push(position);
            characters.push(character);
            Ok(true)
        })?;

        assert_eq!(
            positions,
            vec![
                MatrixPoint::new(0, 0),
                MatrixPoint::new(1, 0),
                MatrixPoint::new(0, 1),
                MatrixPoint::new(1, 1),
            ]
        );
        assert_eq!(characters, vec!['a', 'b', 'c', 'd']);
        Ok(())
    }

    #[test]
    fn parse_grid_with_offset_creates_empty_input_error() {
        let input = "";
        let offset = 0;
        let result: ParseResult<DMatrix<bool>> =
            parse_grid_with_offset(input, offset, |_position, character| {
                Ok(character == '-')
            });
        assert!(result.is_err(), "expected parse to fail");
        match result.unwrap_err() {
            ParseError::EmptyInput => {}
            not_empty_input => {
                panic!("unexpected error type: {not_empty_input:?}");
            }
        }
    }

    #[test]
    fn parse_grid_with_offset_creates_empty_line_error() {
        let input = "..-\n\n-.-\n";
        let offset = 2;
        let result: ParseResult<DMatrix<bool>> =
            parse_grid_with_offset(input, offset, |_position, character| {
                Ok(character == '-')
            });
        assert!(result.is_err(), "expected parse to fail");
        match result.unwrap_err() {
            ParseError::InvalidLine { line, source } => {
                assert_eq!(line, 4, "expected failure on line 4");
                match *source {
                    ParseError::EmptyLine => {}
                    not_empty_line => {
                        panic!(
                            "unexpected source error type: {not_empty_line:?}"
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
    fn parse_grid_with_offset_creates_line_length_error() {
        let input = "..-\n.--\n-.\n";
        let offset = 1;
        let result: ParseResult<DMatrix<bool>> =
            parse_grid_with_offset(input, offset, |_position, character| {
                Ok(character == '-')
            });
        assert!(result.is_err(), "expected parse to fail");
        match result.unwrap_err() {
            ParseError::InvalidLine { line, source } => {
                assert_eq!(line, 4, "expected failure on line 4");
                match *source {
                    ParseError::LineLength { expected, actual } => {
                        assert_eq!(expected, 3);
                        assert_eq!(actual, 2);
                    }
                    not_line_length => {
                        panic!(
                            "unexpected source error type: {not_line_length:?}"
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
    fn parse_grid_with_offset_creates_error_from_parser() {
        let input = "..-\n.b-\n-.-\n";
        let offset = 3;
        let result =
            parse_grid_with_offset(input, offset, |_position, character| {
                match character {
                    '-' => Ok(true),
                    '.' => Ok(false),
                    other => Err(ParseError::ParseChar(other)),
                }
            });
        assert!(result.is_err(), "expected parse to fail");
        match result.unwrap_err() {
            ParseError::InvalidLine { line, source } => {
                assert_eq!(line, 5, "expected failure on line 5");
                match *source {
                    ParseError::ParseChar(character) => {
                        assert_eq!(character, 'b');
                    }
                    not_parse_char => {
                        panic!(
                            "unexpected source error type: {not_parse_char:?}"
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
