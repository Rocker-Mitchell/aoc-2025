//! Solutions implemented for Advent of Code 2025.
//!
//! This module provides [`run_day`] to dynamically run a solution by its day.
//!
//! Making a solution available to run requires implementing
//! [`RunnableSolution`] (likely via the
//! [`impl_runnable_solution!`][aoc_framework::impl_runnable_solution] macro),
//! exporting its module, and adding a match case for its day within
//! [`run_day`].

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
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::set_contains_or_insert,
    clippy::suboptimal_flops,
    clippy::suspicious_operation_groupings,
    clippy::trait_duplication_in_bounds,
    clippy::type_repetition_in_bounds,
    clippy::use_self,
    clippy::useless_let_if_seq
)]
#![deny(clippy::unwrap_used)]

use aoc_framework::{OutputHandler, ParseError, RunnableSolution};
use thiserror::Error;

// TODO possible packages to add later:
// - regex
// - nalgebra for matrix and vectors

mod util;

// --- EXPORT SOLUTION MODULES HERE ---
pub mod day00;
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;

/// Run a solution based on the day.
///
/// See [`RunnableSolution::run`] for arguments used.
///
/// # Errors
///
/// If the solution for the given day is not yet implemented, a
/// [`DaySolutionError::DayNotImplemented`] is returned.
///
/// If parsing the input for the solution fails, a
/// [`DaySolutionError::ParseError`] is returned.
pub fn run_day(
    day: u8,
    handler: &mut dyn OutputHandler,
    input: &str,
    timed: bool,
) -> Result<(), DaySolutionError> {
    match day {
        // --- MATCH SOLUTIONS HERE ---
        0 => day00::Day00::run(handler, input, timed),
        1 => day01::Day01::run(handler, input, timed),
        2 => day02::Day02::run(handler, input, timed),
        3 => day03::Day03::run(handler, input, timed),
        4 => day04::Day04::run(handler, input, timed),
        5 => day05::Day05::run(handler, input, timed),
        6 => day06::Day06::run(handler, input, timed),
        7 => day07::Day07::run(handler, input, timed),
        _ => return Err(DaySolutionError::DayNotImplemented(day)),
    }
    .map_err(DaySolutionError::from)
}

/// An error that can occur when running a day's solution.
#[derive(Error, Debug)]
pub enum DaySolutionError {
    /// The solution for the given day is not yet implemented.
    #[error("solution for day {0} not yet implemented")]
    DayNotImplemented(u8),

    /// The solution failed to parse input.
    #[error("solution failed to parse input")]
    ParseError(#[from] ParseError),
}
