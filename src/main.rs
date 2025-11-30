//! A command line interface for Advent of Code 2025.

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
#![deny(clippy::expect_used, clippy::unwrap_used)]

use std::fmt::Display;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::{Context, Result};
use aoc_framework::{OutputHandler, SolutionPart};
use clap::{ArgAction, Parser};
use solutions::run_day;

// TODO possible packages to add later:
// - anstyle and anstream for styling clap and prints

mod format;

use format::format_duration;

/// Advent of Code 2025 challenge solver.
#[derive(Parser, Debug)]
struct Cli {
    /// The day's solution to run (e.g. 1, 2, etc.).
    day: u8,

    /// Sets an alternative input file to use over default input.
    #[arg(short, long, value_name = "FILE")]
    input: Option<PathBuf>,

    /// Measure the time of parsing and running parts.
    #[arg(short, long, action = ArgAction::SetTrue)]
    timed: bool,

    /// Minimum duration (in milliseconds) required to print timing.
    /// 0 = always print.
    #[arg(long, value_name = "NUMBER", default_value_t)]
    min_timing_ms: u64,
}

/// Read the default input file for the day to a string.
fn get_default_input(day: u8) -> Result<String> {
    let filename = format!("day{day:02}.txt");
    // define file path relative to current directory
    let path = PathBuf::from("inputs").join(filename);

    fs::read_to_string(&path).with_context(|| {
        format!(
            "default input file missing: {}\n\n\
            please create the file or provide the input file argument",
            path.display()
        )
    })
}

/// Try to read the given input file to a string, otherwise get the default
/// input for the day.
fn get_input(day: u8, input_file: Option<PathBuf>) -> Result<String> {
    input_file.map_or_else(
        || get_default_input(day),
        |path| {
            fs::read_to_string(&path).with_context(|| {
                format!("could not read input file at: {}", path.display())
            })
        },
    )
}

/// The output event handler for the Advent of Code CLI.
///
/// This tracks a minimum timing threshold to control printing timing
/// information. Any durations under the minimum will be omitted.
pub struct CliOutputHandler {
    /// The minimum timing threshold.
    min_timing: Duration,
}

impl CliOutputHandler {
    /// Construct an instance with the given minimum timing threshold.
    #[must_use]
    pub fn new(min_timing: Duration) -> Self {
        Self { min_timing }
    }

    /// Check if the given duration is above the minimum timing.
    fn duration_over_min(&self, duration: Duration) -> bool {
        duration >= self.min_timing
    }
}

impl OutputHandler for CliOutputHandler {
    fn solution_name(&mut self, name: &str) {
        println!("= {name} =");
    }

    fn parse_start(&mut self) {
        // do nothing
    }

    fn parse_end(&mut self) {
        // do nothing
    }

    fn parse_end_timed(&mut self, duration: Duration) {
        if self.duration_over_min(duration) {
            println!("Input parsed in {}", format_duration(duration));
        }
    }

    fn part_start(&mut self, part: SolutionPart) {
        println!("-- {} --", part.default_name());
    }

    fn part_output(&mut self, _part: SolutionPart, output: &dyn Display) {
        println!("{output}");
    }

    fn part_output_timed(
        &mut self,
        part: SolutionPart,
        output: &dyn Display,
        duration: Duration,
    ) {
        if self.duration_over_min(duration) {
            println!("{} ({})", output, format_duration(duration));
        } else {
            self.part_output(part, output);
        }
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let input_text = get_input(args.day, args.input)?;
    let mut handler =
        CliOutputHandler::new(Duration::from_millis(args.min_timing_ms));
    run_day(args.day, &mut handler, &input_text, args.timed)
        .with_context(|| "failed to run solution")
}
