//! Macros for the Advent of Code framework.

/// Measure the time to calculate an expression.
///
/// The macro evaluates the expression once and returns a tuple of the
/// expression's result and the elapsed [`Duration`][std::time::Duration].
///
/// Note, the macro measures the evaluation of the expression passed to it.
/// If the expression has side effects or consumes variables, that will still
/// be part of the measured time.
///
/// # Examples
///
/// ```
/// # fn main() {
/// use std::time::Duration;
/// use aoc_framework::measure_time;
///
/// fn calc() -> u32 { 10 + 20 }
/// let (result, duration): (u32, Duration) = measure_time!(calc());
/// assert_eq!(result, 30);
/// # }
/// ```
#[macro_export]
macro_rules! measure_time {
    ($expr:expr) => {{
        let start = ::std::time::Instant::now();
        let result = $expr;
        let elapsed = start.elapsed();
        (result, elapsed)
    }};
}

/// Implement [`RunnableSolution`][crate::RunnableSolution] for a solution type.
///
/// This macro takes the solution type and the trait it implements
/// (e.g., [`Part1`][crate::Part1], [`Part2`][crate::Part2],
/// [`ParsedPart1`][crate::ParsedPart1], [`ParsedPart2`][crate::ParsedPart2])
/// and generates the necessary implementation of
/// [`RunnableSolution`][crate::RunnableSolution].
///
/// # Examples
///
/// Implementing for a solution that has both parts:
/// ```
/// use aoc_framework::{ParseResult, Part1, Part2, SolutionName};
/// use aoc_framework::impl_runnable_solution;
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
/// Implementing for a solution that has parsing and both parts:
/// ```
/// use aoc_framework::{ParseResult, ParsedPart1, ParsedPart2, SolutionName};
/// use aoc_framework::impl_runnable_solution;
///
/// struct MyParsedSolution;
/// impl SolutionName for MyParsedSolution {
///     const NAME: &'static str = "My Parsed Solution";
/// }
/// impl ParsedPart1 for MyParsedSolution {
///     type ParsedInput = String;
///     fn parse(input: &str) -> ParseResult<Self::ParsedInput> {
///         Ok(input.to_string())
///     }
///     type Part1Output = usize;
///     fn part1(parsed: &Self::ParsedInput) -> Self::Part1Output {
///         parsed.len()
///     }
/// }
/// impl ParsedPart2 for MyParsedSolution {
///     type Part2Output = usize;
///     fn part2(parsed: &Self::ParsedInput) -> Self::Part2Output {
///         parsed.len()
///     }
/// }
/// impl_runnable_solution!(MyParsedSolution => ParsedPart2);
/// ```
#[macro_export]
macro_rules! impl_runnable_solution {
    ($solution:ty => Part1) => {
        impl $crate::RunnableSolution for $solution {
            fn run(
                handler: &mut dyn $crate::OutputHandler,
                input: &str,
                timed: bool,
            ) -> $crate::ParseResult<()> {
                <$solution as $crate::Part1>::run(handler, input, timed)
            }
        }
    };
    ($solution:ty => Part2) => {
        impl $crate::RunnableSolution for $solution {
            fn run(
                handler: &mut dyn $crate::OutputHandler,
                input: &str,
                timed: bool,
            ) -> $crate::ParseResult<()> {
                <$solution as $crate::Part2>::run(handler, input, timed)
            }
        }
    };
    ($solution:ty => ParsedPart1) => {
        impl $crate::RunnableSolution for $solution {
            fn run(
                handler: &mut dyn $crate::OutputHandler,
                input: &str,
                timed: bool,
            ) -> $crate::ParseResult<()> {
                <$solution as $crate::ParsedPart1>::run(handler, input, timed)
            }
        }
    };
    ($solution:ty => ParsedPart2) => {
        impl $crate::RunnableSolution for $solution {
            fn run(
                handler: &mut dyn $crate::OutputHandler,
                input: &str,
                timed: bool,
            ) -> $crate::ParseResult<()> {
                <$solution as $crate::ParsedPart2>::run(handler, input, timed)
            }
        }
    };
}
