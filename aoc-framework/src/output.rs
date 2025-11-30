//! A trait for output event handling.

use std::fmt::Display;
use std::time::Duration;

use crate::SolutionPart;

/// A handler for output events when a solution runs.
///
/// See the crate-level documentation for implementation examples.
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
}
