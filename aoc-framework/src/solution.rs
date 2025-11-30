//! Traits for Advent of Code solutions.

use std::fmt::Display;

use crate::{OutputHandler, ParseResult, SolutionPart, measure_time};

/// A trait to provide a name for a solution.
///
/// See the crate-level documentation for implementation examples.
pub trait SolutionName {
    /// The solution's display name.
    const NAME: &'static str;

    /// Output the solution's name using the given output handler.
    fn output_name(handler: &mut dyn OutputHandler) {
        handler.solution_name(Self::NAME);
    }
}

/// A trait for solutions that implement part 1.
///
/// It is expected that part 1 can be solved directly from the raw input. If
/// separate parsing is needed, consider using [`ParsedPart1`] instead.
///
/// For solutions that also implement part 2, see [`Part2`].
///
/// You can make a solution implement
/// [`RunnableSolution`][crate::RunnableSolution] to call [`Part1::run`] with
/// the macro [`impl_runnable_solution!`][crate::impl_runnable_solution].
///
/// # Examples
///
/// ```
/// use aoc_framework::{
///     ParseResult, Part1, SolutionName, impl_runnable_solution,
/// };
///
/// struct MySolution;
/// impl SolutionName for MySolution {
///     const NAME: &'static str = "My Solution";
/// }
/// impl Part1 for MySolution {
///     type Part1Output = usize;
///     fn part1(input: &str) -> ParseResult<Self::Part1Output> {
///         Ok(input.len())
///     }
/// }
/// impl_runnable_solution!(MySolution => Part1);
/// ```
///
/// See the crate-level documentation for more examples.
pub trait Part1: SolutionName {
    /// The type of the output of part 1.
    type Part1Output: Display;

    /// Solve part 1 of the solution, returning the `Part1Output` type.
    ///
    /// # Errors
    ///
    /// If parsing fails, a [`ParseError`][crate::ParseError] is returned.
    ///
    /// # Panics
    ///
    /// Implementors may panic if unexpected conditions occur, as Advent of
    /// Code problems generally expect correct inputs.
    fn part1(input: &str) -> ParseResult<Self::Part1Output>;

    /// Run part 1 of the solution, outputting results via the given output
    /// handler.
    ///
    /// If `timed` is true, running part 1 will be timed, with related output
    /// events called.
    ///
    /// # Errors
    ///
    /// If parsing fails, a [`ParseError`][crate::ParseError] is returned.
    ///
    /// # Panics
    ///
    /// A solution part's implementation may panic if unexpected conditions
    /// occur, as Advent of Code problems generally expect correct inputs.
    fn run_part1(
        handler: &mut dyn OutputHandler,
        input: &str,
        timed: bool,
    ) -> ParseResult<()> {
        let part = SolutionPart::Part1;
        handler.part_start(part);
        if timed {
            let (output, duration) = measure_time!(Self::part1(input)?);
            handler.part_output_timed(part, &output, duration);
        } else {
            let output = Self::part1(input)?;
            handler.part_output(part, &output);
        }
        Ok(())
    }

    /// Run the solution, outputting results via the given output handler.
    ///
    /// This will only run part 1 of this trait.
    ///
    /// If `timed` is true, running part 1 will be timed, with related output
    /// events called.
    ///
    /// # Errors
    ///
    /// If parsing fails, a [`ParseError`][crate::ParseError] is returned.
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
        Self::output_name(handler);
        Self::run_part1(handler, input, timed)
    }
}

/// A trait for solutions that implement part 2.
///
/// This trait requires that the solution also implements part 1 via
/// [`Part1`].
///
/// You can make a solution implement
/// [`RunnableSolution`][crate::RunnableSolution] to call [`Part2::run`] with
/// the macro [`impl_runnable_solution!`][crate::impl_runnable_solution].
///
/// # Examples
///
/// ```
/// use aoc_framework::{
///     ParseResult, Part1, Part2, SolutionName, impl_runnable_solution,
/// };
///
/// struct MySolution;
/// impl SolutionName for MySolution {
///     const NAME: &'static str = "My Solution";
/// }
/// impl Part1 for MySolution {
///     type Part1Output = usize;
///     fn part1(input: &str) -> ParseResult<Self::Part1Output> {
///         Ok(input.len())
///     }
/// }
/// impl Part2 for MySolution {
///     type Part2Output = usize;
///     fn part2(input: &str) -> ParseResult<Self::Part2Output> {
///         Ok(input.len())
///     }
/// }
/// impl_runnable_solution!(MySolution => Part2);
/// ```
///
/// See the crate-level documentation for more examples.
pub trait Part2: Part1 {
    /// The type of the output of part 2.
    type Part2Output: Display;

    /// Solve part 2 of the solution, returning the `Part2Output` type.
    ///
    /// # Errors
    ///
    /// If parsing fails, a [`ParseError`][crate::ParseError] is returned.
    ///
    /// # Panics
    ///
    /// Implementors may panic if unexpected conditions occur, as Advent of
    /// Code problems generally expect correct inputs.
    fn part2(input: &str) -> ParseResult<Self::Part2Output>;

    /// Run part 2 of the solution, outputting results via the given output
    /// handler.
    ///
    /// If `timed` is true, running part 2 will be timed, with related output
    /// events called.
    ///
    /// # Errors
    ///
    /// If parsing fails, a [`ParseError`][crate::ParseError] is returned.
    ///
    /// # Panics
    ///
    /// A solution part's implementation may panic if unexpected conditions
    /// occur, as Advent of Code problems generally expect correct inputs.
    fn run_part2(
        handler: &mut dyn OutputHandler,
        input: &str,
        timed: bool,
    ) -> ParseResult<()> {
        let part = SolutionPart::Part2;
        handler.part_start(part);
        if timed {
            let (output, duration) = measure_time!(Self::part2(input)?);
            handler.part_output_timed(part, &output, duration);
        } else {
            let output = Self::part2(input)?;
            handler.part_output(part, &output);
        }
        Ok(())
    }

    /// Run the solution, outputting results via the given output handler.
    ///
    /// This will run both part 1 and part 2 of this trait and its supertrait
    /// [`Part1`].
    ///
    /// If `timed` is true, running parts will be timed, with related output
    /// events called.
    ///
    /// # Errors
    ///
    /// If parsing fails, a [`ParseError`][crate::ParseError] is returned.
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
        Self::output_name(handler);
        Self::run_part1(handler, input, timed)?;
        Self::run_part2(handler, input, timed)
    }
}

/// A trait for solutions that implement part 1 with parsed input.
///
/// It is expected that input is parsed once so solutions can use the resulting
/// parsed data for both parts. If a solution only implements part 1, this
/// trait is useful to separate parsing from part solving. Otherwise, consider
/// using [`Part1`] if separate parsing is not needed, or [`Part2`] if both
/// parts require separate parsing.
///
/// For solutions that also implement part 2, see [`ParsedPart2`].
///
/// You can make a solution implement
/// [`RunnableSolution`][crate::RunnableSolution] to call [`ParsedPart1::run`]
/// with the macro [`impl_runnable_solution!`][crate::impl_runnable_solution].
///
/// # Examples
///
/// ```
/// use aoc_framework::{
///     ParseResult, ParsedPart1, SolutionName, impl_runnable_solution,
/// };
///
/// struct MySolution;
/// impl SolutionName for MySolution {
///     const NAME: &'static str = "My Solution";
/// }
/// impl ParsedPart1 for MySolution {
///     type ParsedInput = Vec<String>;
///     fn parse(input: &str) -> ParseResult<Self::ParsedInput> {
///         Ok(input.lines().map(|line| line.to_string()).collect())
///     }
///     type Part1Output = usize;
///     fn part1(parsed: &Self::ParsedInput) -> Self::Part1Output {
///         parsed.len()
///     }
/// }
/// impl_runnable_solution!(MySolution => ParsedPart1);
/// ```
///
/// See the crate-level documentation for more examples.
pub trait ParsedPart1: SolutionName {
    /// The data that represents the parsed input.
    type ParsedInput;

    /// Parse the raw input string into the solution's `ParsedInput` type.
    ///
    /// # Errors
    ///
    /// If parsing fails, a [`ParseError`][crate::ParseError] is returned.
    fn parse(input: &str) -> ParseResult<Self::ParsedInput>;

    /// Run parsing of the input, outputting progress via the given output
    /// handler.
    ///
    /// If `timed` is true, parsing will be timed, with related output events
    /// called.
    ///
    /// # Errors
    ///
    /// If parsing fails, a [`ParseError`][crate::ParseError] is returned.
    fn run_parse(
        handler: &mut dyn OutputHandler,
        input: &str,
        timed: bool,
    ) -> ParseResult<Self::ParsedInput> {
        handler.parse_start();
        if timed {
            let (parsed, duration) = measure_time!(Self::parse(input)?);
            handler.parse_end_timed(duration);
            Ok(parsed)
        } else {
            let parsed = Self::parse(input)?;
            handler.parse_end();
            Ok(parsed)
        }
    }

    /// The type of the output of part 1.
    type Part1Output: Display;

    /// Solve part 1 of the solution, returning the `Part1Output` type.
    ///
    /// # Panics
    ///
    /// Implementors may panic if unexpected conditions occur, as Advent of
    /// Code problems generally expect correct inputs.
    fn part1(parsed: &Self::ParsedInput) -> Self::Part1Output;

    /// Run part 1 of the solution, outputting results via the given output
    /// handler.
    ///
    /// If `timed` is true, running part 1 will be timed, with related output
    /// events called.
    ///
    /// # Panics
    ///
    /// A solution part's implementation may panic if unexpected conditions
    /// occur, as Advent of Code problems generally expect correct inputs.
    fn run_part1(
        handler: &mut dyn OutputHandler,
        parsed: &Self::ParsedInput,
        timed: bool,
    ) {
        let part = SolutionPart::Part1;
        handler.part_start(part);
        if timed {
            let (output, duration) = measure_time!(Self::part1(parsed));
            handler.part_output_timed(part, &output, duration);
        } else {
            let output = Self::part1(parsed);
            handler.part_output(part, &output);
        }
    }

    /// Run the solution, outputting results via the given output handler.
    ///
    /// This will run parsing and part 1 of this trait.
    ///
    /// If `timed` is true, parsing and running part 1 will be timed, with
    /// related output events called.
    ///
    /// # Errors
    ///
    /// If parsing fails, a [`ParseError`][crate::ParseError] is returned.
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
        Self::output_name(handler);
        let parsed = Self::run_parse(handler, input, timed)?;
        Self::run_part1(handler, &parsed, timed);
        Ok(())
    }
}

/// A trait for solutions that implement part 2 with parsed input.
///
/// It is expected that input is parsed once, then the resulting parsed data
/// is used for both parts.
///
/// This trait requires that the solution also implements part 1 via
/// [`ParsedPart1`].
///
/// You can make a solution implement
/// [`RunnableSolution`][crate::RunnableSolution] to call [`ParsedPart2::run`]
/// with the macro [`impl_runnable_solution!`][crate::impl_runnable_solution].
///
/// # Examples
///
/// ```
/// use aoc_framework::{
///     ParseResult, ParsedPart1, ParsedPart2, SolutionName,
///     impl_runnable_solution,
/// };
///
/// struct MySolution;
/// impl SolutionName for MySolution {
///     const NAME: &'static str = "My Solution";
/// }
/// impl ParsedPart1 for MySolution {
///     type ParsedInput = Vec<String>;
///     fn parse(input: &str) -> ParseResult<Self::ParsedInput> {
///         Ok(input.lines().map(|line| line.to_string()).collect())
///     }
///     type Part1Output = usize;
///     fn part1(parsed: &Self::ParsedInput) -> Self::Part1Output {
///         parsed.len()
///     }
/// }
/// impl ParsedPart2 for MySolution {
///     type Part2Output = usize;
///     fn part2(parsed: &Self::ParsedInput) -> Self::Part2Output {
///         parsed.len()
///     }
/// }
/// impl_runnable_solution!(MySolution => ParsedPart2);
/// ```
///
/// See the crate-level documentation for more examples.
pub trait ParsedPart2: ParsedPart1 {
    /// The type of the output of part 2.
    type Part2Output: Display;

    /// Solve part 2 of the solution, returning the `Part2Output` type.
    ///
    /// # Panics
    ///
    /// Implementors may panic if unexpected conditions occur, as Advent of
    /// Code problems generally expect correct inputs.
    fn part2(parsed: &Self::ParsedInput) -> Self::Part2Output;

    /// Run part 2 of the solution, outputting results via the given output
    /// handler.
    ///
    /// If `timed` is true, running part 2 will be timed, with related output
    /// events called.
    ///
    /// # Panics
    ///
    /// A solution part's implementation may panic if unexpected conditions
    /// occur, as Advent of Code problems generally expect correct inputs.
    fn run_part2(
        handler: &mut dyn OutputHandler,
        parsed: &Self::ParsedInput,
        timed: bool,
    ) {
        let part = SolutionPart::Part2;
        handler.part_start(part);
        if timed {
            let (output, duration) = measure_time!(Self::part2(parsed));
            handler.part_output_timed(part, &output, duration);
        } else {
            let output = Self::part2(parsed);
            handler.part_output(part, &output);
        }
    }

    /// Run the solution, outputting results via the given output handler.
    ///
    /// This will run parsing, part 1, and part 2 of this trait and its
    /// supertrait [`ParsedPart1`].
    ///
    /// If `timed` is true, parsing and running parts will be timed, with
    /// related output events called.
    ///
    /// # Errors
    ///
    /// If parsing fails, a [`ParseError`][crate::ParseError] is returned.
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
        Self::output_name(handler);
        let parsed = Self::run_parse(handler, input, timed)?;
        Self::run_part1(handler, &parsed, timed);
        Self::run_part2(handler, &parsed, timed);
        Ok(())
    }
}
