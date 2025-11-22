//! A trait for output event handling.

use std::fmt::Display;
use std::time::Duration;

use crate::SolutionPart;

/// A handler for output events when a solution runs.
///
/// # Examples
///
/// An example implementation:
/// ```
/// use aoc_framework::{OutputHandler, SolutionPart};
/// use std::fmt::Display;
/// use std::time::Duration;
///
/// struct SimpleHandler;
/// impl OutputHandler for SimpleHandler {
///     fn solution_name(&mut self, name: &str) {
///         println!("{name}");
///     }
///     fn parse_start(&mut self) {}
///     fn parse_end(&mut self) {}
///     fn parse_end_timed(&mut self, _d: Duration) {}
///     fn part_start(&mut self, _p: SolutionPart) {}
///     fn part_output(&mut self, part: SolutionPart, output: &dyn Display) {
///         println!("{}: {}", part.default_name(), output);
///     }
///     fn part_output_timed(
///         &mut self,
///         part: SolutionPart,
///         output: &dyn Display,
///         _d: Duration
///     ) {
///         self.part_output(part, output);
///     }
///     fn part_not_implemented(&mut self, part: SolutionPart) {
///         println!("{} not implemented", part.default_name());
///     }
/// }
/// // This simple handler will print output like:
/// // My Solution Name
/// // Part 1: 25
/// // Part 2 not implemented
/// ```
pub trait OutputHandler {
    /// Called to output the name of the solution, at the start of running the
    /// solution.
    fn solution_name(&mut self, name: &str);

    /// Called when parsing is starting.
    fn parse_start(&mut self);

    /// Called when parsing is finished.
    fn parse_end(&mut self);

    /// Called when parsing is finished along with the duration taken.
    fn parse_end_timed(&mut self, duration: Duration);

    /// Called when a part is starting, with a [`SolutionPart`] enum for which
    /// part it is.
    fn part_start(&mut self, part: SolutionPart);

    /// Called to output the results of a part, with a [`SolutionPart`] enum
    /// for which part it is.
    fn part_output(&mut self, part: SolutionPart, output: &dyn Display);

    /// Called to output the results of a part along with the duration taken,
    /// with a [`SolutionPart`] enum for which part it is.
    fn part_output_timed(
        &mut self,
        part: SolutionPart,
        output: &dyn Display,
        duration: Duration,
    );

    /// Called when a part was found to not yet be implemented, with a
    /// [`SolutionPart`] enum for which part it is.
    fn part_not_implemented(&mut self, part: SolutionPart);
}
