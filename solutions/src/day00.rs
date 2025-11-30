use aoc_framework::{
    ParseError, ParseResult, ParsedPart1, ParsedPart2, SolutionName,
    impl_runnable_solution,
};

use crate::util::parse::parse_lines;

/// An example of an implemented solution.
///
/// This solution demonstrates parsing numbers from input lines through a
/// utility function, then returning the count of numbers for part 1 and the
/// sum of numbers for part 2.
pub struct Day00;

impl SolutionName for Day00 {
    const NAME: &'static str = "Day 0: Example Solution";
}

impl ParsedPart1 for Day00 {
    type ParsedInput = Vec<u32>;

    fn parse(input: &str) -> ParseResult<Self::ParsedInput> {
        // expect lines of unsigned numbers
        // - ignore trailing whitespace
        let numbers: Vec<u32> = parse_lines(input.trim_end(), |line| {
            line.parse::<u32>()
                .map_err(|source| ParseError::parse_int_from_str(line, source))
        })
        .collect::<ParseResult<_>>()?;

        if numbers.is_empty() {
            Err(ParseError::EmptyInput)
        } else {
            Ok(numbers)
        }
    }

    type Part1Output = usize;

    fn part1(numbers: &Self::ParsedInput) -> Self::Part1Output {
        // count of numbers
        numbers.len()
    }
}

impl ParsedPart2 for Day00 {
    type Part2Output = u32;

    fn part2(numbers: &Self::ParsedInput) -> Self::Part2Output {
        // sum of numbers
        numbers.iter().sum::<u32>()
    }
}

impl_runnable_solution!(Day00 => ParsedPart2);

#[cfg(test)]
mod tests {
    use std::num::IntErrorKind;

    use super::*;

    const EXAMPLE_INPUT: &str = "\
        10\n\
        20\n\
        30\n\
        40\n";

    #[test]
    fn part1_solves_example() -> ParseResult<()> {
        let parsed = Day00::parse(EXAMPLE_INPUT)?;
        let result = Day00::part1(&parsed);
        assert_eq!(result, 4);
        Ok(())
    }

    #[test]
    fn part2_solves_example() -> ParseResult<()> {
        let parsed = Day00::parse(EXAMPLE_INPUT)?;
        let result = Day00::part2(&parsed);
        assert_eq!(result, 100);
        Ok(())
    }

    #[test]
    fn parse_error_on_empty_input() {
        let result = Day00::parse("\n\n");
        assert!(result.is_err(), "expected parse to fail");
        match result.unwrap_err() {
            ParseError::EmptyInput => {}
            not_empty_input => {
                panic!("unexpected error type: {not_empty_input:?}");
            }
        }
    }

    #[test]
    fn parse_error_on_non_number() {
        let result = Day00::parse("10\n15bad\n20\n");
        assert!(result.is_err(), "expected parse to fail");
        match result.unwrap_err() {
            ParseError::InvalidLine { line, source } => {
                assert_eq!(line, 2, "expected failure on line 2");
                match *source {
                    ParseError::ParseInt { string, source } => {
                        assert_eq!(
                            string, "15bad",
                            "wrong string reported for error"
                        );
                        match source.kind() {
                            IntErrorKind::InvalidDigit => {}
                            not_invalid_digit => {
                                panic!(
                                    "unexpected int error kind type: {not_invalid_digit:?}"
                                );
                            }
                        }
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
    fn parse_error_on_empty_line() {
        let result = Day00::parse("10\n20\n\n30\n");
        assert!(result.is_err(), "expected parse to fail");
        match result.unwrap_err() {
            ParseError::InvalidLine { line, source } => {
                assert_eq!(line, 3, "expected failure on line 3");
                match *source {
                    ParseError::ParseInt { string, source } => {
                        assert_eq!(
                            string, "",
                            "wrong string reported for error"
                        );
                        match source.kind() {
                            IntErrorKind::Empty => {}
                            not_empty => {
                                panic!(
                                    "unexpected int error kind type: {not_empty:?}"
                                );
                            }
                        }
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
