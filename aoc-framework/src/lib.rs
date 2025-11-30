//! Advent of Code framework library.
//!
//! Public API overview:
//!
//! - [`RunnableSolution`]: trait for solutions that can be run dynamically.
//! - [`SolutionName`]: trait to provide a name for a solution.
//! - [`Part1`], [`Part2`]: traits for solutions that implement part 1 and part
//!   2 respectively.
//! - [`ParsedPart1`], [`ParsedPart2`]: traits for solutions that implement
//!   part 1 and part 2 respectively, with separate parsing of input.
//! - [`ParseError`] and [`ParseResult`]: structured parsing errors returned by
//!   parsers.
//! - [`OutputHandler`]: trait used by runner to receive output events.
//! - [`measure_time!`] macro: helper to measure duration of an expression.
//! - [`impl_runnable_solution!`] macro: helper to implement
//!   [`RunnableSolution`] for solution types.
//!
//! # Quick Start
//!
//! Implement [`SolutionName`] for your solution type to provide a name. Then,
//! pick your pattern of implementing solution traits:
//!
//! - Without distinct parsing step: implement [`Part1`] (and [`Part2`] if
//!   needed).
//! - With distinct parsing step: implement [`ParsedPart1`] (and
//!   [`ParsedPart2`] if needed).
//!
//! Then, use the [`impl_runnable_solution!`] macro to implement
//! [`RunnableSolution`].
//!
//! See examples below for concrete implementations.
//!
//! # Examples
//!
//! Implementing a simple solution with both parts:
//! ```
//! use aoc_framework::{
//!     ParseResult, Part1, Part2, SolutionName, impl_runnable_solution,
//! };
//!
//! struct MySolution;
//! impl SolutionName for MySolution {
//!     // name the solution
//!     const NAME: &'static str = "My Solution";
//! }
//! impl Part1 for MySolution {
//!     // define the output type for part 1
//!     type Part1Output = usize;
//!     fn part1(input: &str) -> ParseResult<Self::Part1Output> {
//!         // solve part 1
//!         // for example, return the length of the input
//!         Ok(input.len())
//!     }
//! }
//! impl Part2 for MySolution {
//!     // define the output type for part 2
//!     type Part2Output = usize;
//!     fn part2(input: &str) -> ParseResult<Self::Part2Output> {
//!         // solve part 2
//!         // for example, return the count of lines in the input
//!         Ok(input.lines().count())
//!     }
//! }
//! // implement RunnableSolution for MySolution
//! impl_runnable_solution!(MySolution => Part2);
//! // now you can run MySolution dynamically via RunnableSolution
//! // <MySolution as RunnableSolution>::run(handler, input, timed);
//! ```
//!
//! Implementing a solution with parsing and both parts:
//! ```
//! use aoc_framework::{
//!     ParseError, ParseResult, ParsedPart1, ParsedPart2, SolutionName,
//!     impl_runnable_solution,
//! };
//!
//! struct MyParsedSolution;
//! impl SolutionName for MyParsedSolution {
//!     // name the solution
//!     const NAME: &'static str = "My Parsed Solution";
//! }
//! impl ParsedPart1 for MyParsedSolution {
//!     // define the type of parsed input
//!     type ParsedInput = Vec<u32>;
//!     fn parse(input: &str) -> ParseResult<Self::ParsedInput> {
//!         // parse input
//!         // for example, parse input into a vector of unsigned integers
//!         let numbers: Vec<u32> = input
//!             .lines()
//!             .map(|line| {
//!                 line.parse::<u32>()
//!                     .map_err(|source| {
//!                         ParseError::parse_int_from_str(line, source)
//!                     })
//!             })
//!             .collect::<ParseResult<_>>()?;
//!         Ok(numbers)
//!     }
//!     // define the output type for part 1
//!     type Part1Output = u32;
//!     fn part1(numbers: &Self::ParsedInput) -> Self::Part1Output {
//!         // solve part 1
//!         // for example, return the sum of the numbers
//!         numbers.iter().sum()
//!     }
//! }
//! impl ParsedPart2 for MyParsedSolution {
//!     // define the output type for part 2
//!     type Part2Output = u32;
//!     fn part2(numbers: &Self::ParsedInput) -> Self::Part2Output {
//!         // solve part 2
//!         // for example, return the product of the numbers
//!         numbers.iter().product()
//!     }
//! }
//! // implement RunnableSolution for MyParsedSolution
//! impl_runnable_solution!(MyParsedSolution => ParsedPart2);
//! // now you can run MyParsedSolution dynamically via RunnableSolution
//! // <MyParsedSolution as RunnableSolution>::run(handler, input, timed);
//! ```
//!
//! Implementing a custom output handler:
//! ```
//! use aoc_framework::{OutputHandler, SolutionPart};
//! use std::fmt::Display;
//! use std::time::Duration;
//!
//! struct MyHandler;
//! impl MyHandler {
//!     fn format_duration(d: Duration) -> String {
//!         format!("{} seconds, {} nanoseconds", d.as_secs(), d.subsec_nanos())
//!     }
//! }
//! impl OutputHandler for MyHandler {
//!     fn solution_name(&mut self, name: &str) {
//!         println!("{name}");
//!     }
//!     fn parse_start(&mut self) {}
//!     fn parse_end(&mut self) {}
//!     fn parse_end_timed(&mut self, duration: Duration) {
//!         println!("Parsing completed in {}", Self::format_duration(duration));
//!     }
//!     fn part_start(&mut self, _p: SolutionPart) {}
//!     fn part_output(&mut self, part: SolutionPart, output: &dyn Display) {
//!         println!("{}: {}", part.default_name(), output);
//!     }
//!     fn part_output_timed(
//!         &mut self,
//!         part: SolutionPart,
//!         output: &dyn Display,
//!         duration: Duration
//!     ) {
//!         println!(
//!             "{}: {} (completed in {})",
//!             part.default_name(),
//!             output,
//!             Self::format_duration(duration)
//!         );
//!     }
//! }
//! // This custom handler will print output like this when not timed:
//! //   My Solution Name
//! //   Part 1: 25
//! //   Part 2: 100
//! // This custom handler will print output like this when timed:
//! //   My Solution Name
//! //   Parsing completed in 0 seconds, 123456789 nanoseconds
//! //   Part 1: 25 (completed in 0 seconds, 98765432 nanoseconds)
//! //   Part 2: 100 (completed in 0 seconds, 87654321 nanoseconds)
//! ```
//!
//! In real code, parsing errors are propagated up to the runner, which won't
//! call output methods if parsing fails; no error handling is needed in the
//! output handler itself.

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

pub mod error;
pub mod macros;
pub mod output;
pub mod solution;

// re-export commonly used items
pub use error::{ParseError, ParseResult};
pub use output::OutputHandler;
pub use solution::{ParsedPart1, ParsedPart2, Part1, Part2, SolutionName};

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

/// A trait for Advent of Code solutions that can be run.
///
/// This trait is implemented for solution types via the
/// [`impl_runnable_solution!`] macro.
pub trait RunnableSolution {
    /// Run the solution, parsing input and running parts if implemented.
    ///
    /// The output handler will be used to output progress events while running
    /// the solution.
    ///
    /// If `timed` is true, parsing and running parts will be timed if
    /// implemented, with related output events called.
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
    ) -> ParseResult<()>;
}
