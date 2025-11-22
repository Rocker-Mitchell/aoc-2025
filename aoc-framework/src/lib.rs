//! Advent of Code framework library.
//!
//! Public API overview:
//!
//! - [`Solution`]: implement this trait for each day's solution.
//! - [`ParseError`] and [`ParseResult`]: structured parsing errors returned by
//!   parsers.
//! - [`OutputHandler`]: trait used by runner to receive output events.
//! - `measure_time!` macro: helper to measure duration of an expression.

#![warn(clippy::suspicious, clippy::complexity, clippy::perf, clippy::pedantic)]
#![warn(
    clippy::allow_attributes,
    clippy::allow_attributes_without_reason,
    clippy::branches_sharing_code,
    clippy::collection_is_never_read,
    clippy::equatable_if_let,
    clippy::needless_collect,
    clippy::needless_pass_by_ref_mut,
    clippy::option_if_let_else,
    clippy::set_contains_or_insert,
    clippy::suboptimal_flops,
    clippy::suspicious_operation_groupings,
    clippy::trait_duplication_in_bounds,
    clippy::type_repetition_in_bounds,
    clippy::use_self,
    clippy::useless_let_if_seq
)]
#![deny(
    clippy::expect_used,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::unwrap_used
)]

use std::fmt::Display;

pub mod error;
pub mod macros;
pub mod output;
pub mod util;

// re-export commonly used items
pub use error::{ParseError, ParseResult};
pub use output::OutputHandler;

/// An enum to identify parts of a solution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolutionPart {
    Part1,
    Part2,
}

impl SolutionPart {
    /// Get a default name for a part.
    #[must_use]
    pub fn default_name(self) -> &'static str {
        match self {
            Self::Part1 => "Part 1",
            Self::Part2 => "Part 2",
        }
    }
}

/// A trait for Advent of Code solutions.
pub trait Solution {
    /// The solution's display name.
    const NAME: &'static str;

    /// The data that represents the parsed input.
    type ParsedInput;

    /// Parse the raw input string into the solution's `ParsedInput` type.
    ///
    /// # Errors
    ///
    /// If parsing fails, a [`ParseError`] is returned.
    fn parse(input: &str) -> ParseResult<Self::ParsedInput>;

    /// The type of the output of part 1.
    type Part1Output: Display;

    /// Solve part 1 of the solution, returning the `Part1Output` type.
    ///
    /// # Panics
    ///
    /// Implementors may panic if unexpected conditions occur, as Advent of
    /// Code problems generally expect correct inputs.
    fn part1(input: &Self::ParsedInput) -> Self::Part1Output;

    /// The type of the output of part 2.
    type Part2Output: Display;

    /// Solve part 2 of the solution, returning the `Part2Output` type.
    ///
    /// Advent of Code requires part 1 to be correctly solved before revealing
    /// part 2, thus this function returns `None` as the default to indicate
    /// when it is not yet implemented. So, implementors should return
    /// `Some(output)`.
    ///
    /// # Panics
    ///
    /// Implementors may panic if unexpected conditions occur, as Advent of
    /// Code problems generally expect correct inputs.
    fn part2(_input: &Self::ParsedInput) -> Option<Self::Part2Output> {
        None
    }

    /// Run the solution, parsing input and running both parts.
    ///
    /// The output handler will be used to output progress events while running
    /// the solution.
    ///
    /// If `timed` is true, parsing and running parts will be timed, with
    /// related output events called.
    ///
    /// # Errors
    ///
    /// If parsing input fails, a [`ParseError`] is returned.
    ///
    /// # Panics
    ///
    /// A solution part's implementation may panic if unexpected conditions
    /// occur, as Advent of Code problems generally expect correct inputs.
    fn run(
        handler: &mut dyn OutputHandler,
        input: &str,
        timed: bool,
    ) -> ParseResult<()> {
        handler.solution_name(Self::NAME);

        handler.parse_start();
        let parsed: Self::ParsedInput = {
            if timed {
                let (parsed, duration) = measure_time!(Self::parse(input)?);
                handler.parse_end_timed(duration);
                parsed
            } else {
                let parsed = Self::parse(input)?;
                handler.parse_end();
                parsed
            }
        };

        let part = SolutionPart::Part1;
        handler.part_start(part);
        if timed {
            let (output, duration) = measure_time!(Self::part1(&parsed));
            handler.part_output_timed(part, &output, duration);
        } else {
            let output = Self::part1(&parsed);
            handler.part_output(part, &output);
        }

        let part = SolutionPart::Part2;
        handler.part_start(part);
        if timed {
            let (opt_output, duration) = measure_time!(Self::part2(&parsed));
            match opt_output {
                Some(output) => {
                    handler.part_output_timed(part, &output, duration);
                }
                None => {
                    handler.part_not_implemented(part);
                }
            }
        } else {
            match Self::part2(&parsed) {
                Some(output) => {
                    handler.part_output(part, &output);
                }
                None => {
                    handler.part_not_implemented(part);
                }
            }
        }

        Ok(())
    }
}
