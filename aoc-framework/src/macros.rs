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
