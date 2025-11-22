//! Formatting utilities for the CLI.

use std::time::Duration;

/// Formats a `Duration` for display.
///
/// - Uses microseconds (µs), milliseconds (ms), or seconds (s) depending on
///   the magnitude of the duration.
///   - Under 1 millisecond -> microseconds
///   - Under 1 second -> milliseconds
///   - Otherwise -> seconds
/// - Rounds to 3 decimal places.
///
/// Note, the microseconds symbol uses the Unicode "µ", which may not render
/// correctly in some environments.
#[must_use]
pub fn format_duration(duration: Duration) -> String {
    let nanoseconds = duration.as_nanos();
    #[expect(
        clippy::cast_precision_loss,
        reason = "float will be formatted to 3 decimal places"
    )]
    let nano_float = nanoseconds as f64;
    if nanoseconds < 1_000_000 {
        format!("{:.3} µs", nano_float / 1_000.0)
    } else if nanoseconds < 1_000_000_000 {
        format!("{:.3} ms", nano_float / 1_000_000.0)
    } else {
        format!("{:.3} s", nano_float / 1_000_000_000.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_duration_in_micros() {
        let duration = Duration::from_nanos(207_800);
        assert_eq!(format_duration(duration), "207.800 µs");
    }

    #[test]
    fn format_duration_in_millis() {
        let duration = Duration::from_nanos(14_735_398);
        assert_eq!(format_duration(duration), "14.735 ms");
    }

    #[test]
    fn format_duration_in_seconds() {
        let duration = Duration::from_nanos(1_523_121_031);
        assert_eq!(format_duration(duration), "1.523 s");
    }

    #[test]
    fn format_duration_with_trailing_zeros() {
        let duration = Duration::from_secs(2);
        assert_eq!(format_duration(duration), "2.000 s");
    }

    #[test]
    fn format_duration_rounds_up() {
        let duration = Duration::from_micros(1_999_500);
        assert_eq!(format_duration(duration), "2.000 s");
    }
}
