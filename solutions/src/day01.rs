use aoc_framework::{
    ParseError, ParseResult, ParsedPart1, ParsedPart2, SolutionName,
    impl_runnable_solution,
};

use crate::util::parse::parse_lines;

/// Solution for the first day's puzzle.
///
/// # Context/Input
///
/// There's a safe w/ a dial in the range 0 to 99.
///
/// Input is a sequence of rotations, line-separated. A rotation formats as a
/// direction char (`L` as decreasing numbers, or `R` as increasing numbers)
/// then a distance number (appears non-negative); ex: `R42`.
///
/// Worth noting is the behavior at the extremes: rotating left from 0 cycles
/// to 99, and rotating right from 99 cycles to 0.
///
/// The safe dial initially starts at 50.
///
/// # Part 1
///
/// Evaluate how many times the dial is left pointing at 0 after any rotation.
///
/// # Part 2
///
/// Evaluate whenever 0 is encountered during rotations, so both when a
/// rotation ends at 0 and when it passes over 0.
pub struct Day01;

impl SolutionName for Day01 {
    const NAME: &'static str = "Day 1: Secret Entrance";
}

/*
Inspecting distances given, 3 digits seem to be the largest they get.
So, u16 with a maximum 65,535 should be a good size.

With 99 as the physical max, u8 w/ maximum 255 should work for dial value,
but rotation math would need to convert for larger size of rotation & signed
values.
*/

type RotationDistance = u16;
type DialValue = u8;

/// Where the dial starts.
const DIAL_START: DialValue = 50;
/// The inclusive maximum of the dial.
const DIAL_MAX: DialValue = 99;

/// The direction of a rotation.
#[derive(Debug, PartialEq)]
pub enum Direction {
    /// Rotate as decreasing numbers.
    Left,
    /// Rotate as increasing numbers.
    Right,
}

impl TryFrom<char> for Direction {
    type Error = ParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err(ParseError::ParseChar(value)),
        }
    }
}

/// Representation of a rotation from input.
pub struct Rotation {
    /// The direction.
    pub direction: Direction,
    /// The distance number.
    pub distance: RotationDistance,
}

/// Calculate where a dial value will rotate to, and a count of how many times
/// the rotation passed over 0.
///
/// Starting or ending on 0 will not be counted.
///
/// Returns a tuple of `(new_value, zeros_passed)`.
fn rotate_dial_and_count_zeros_passed(
    value: DialValue,
    rotation: &Rotation,
) -> (DialValue, u16) {
    // want to avoid overflow by using signed type that can hold max rotation
    // distance
    let mut signed_value = i32::from(value);

    match rotation.direction {
        Direction::Left => {
            signed_value -= i32::from(rotation.distance);
        }
        Direction::Right => {
            signed_value += i32::from(rotation.distance);
        }
    }

    // add 1 because the max const is *inclusive*, but our math should be on
    // the exclusive max
    let max = i32::from(DIAL_MAX) + 1;

    // use modulo to mimic the cycling/wrap of the dial range
    // - don't want remainder op behavior with signed values, prefer
    //   mathematical modulo op
    let wrapped_value = signed_value.rem_euclid(max);
    let new_value: DialValue =
        u8::try_from(wrapped_value).expect("`wrapped_value` failed to cast");

    // the remainder was relevant to finding the new value, division should
    // inform how many times it cycled
    let signed_cycles = signed_value.div_euclid(max);
    let mut cycles = u16::try_from(signed_cycles.abs())
        .expect("`signed_cycles.abs()` failed to cast");
    // - this can overcount on left-from-zero and right-to-zero
    //   - not spending time to solve underlying reason; detect special cases
    if (rotation.direction == Direction::Left && value == 0)
        || (rotation.direction == Direction::Right && new_value == 0)
    {
        cycles -= 1;
    }

    (new_value, cycles)
}

impl ParsedPart1 for Day01 {
    type ParsedInput = Vec<Rotation>;

    fn parse(input: &str) -> aoc_framework::ParseResult<Self::ParsedInput> {
        let rotations: Self::ParsedInput = parse_lines(input, |line| {
            let first_char: char =
                line.chars().nth(0).ok_or(ParseError::EmptyLine)?;
            let direction = Direction::try_from(first_char)?;
            let distance: RotationDistance =
                line[1..].parse::<u16>().map_err(|source| {
                    ParseError::parse_int_from_str(line, source)
                })?;
            Ok(Rotation {
                direction,
                distance,
            })
        })
        .collect::<ParseResult<_>>()?;

        if rotations.is_empty() {
            Err(ParseError::EmptyInput)
        } else {
            Ok(rotations)
        }
    }

    type Part1Output = u32;

    fn part1(rotations: &Self::ParsedInput) -> Self::Part1Output {
        // iterate over rotations and track when result is 0
        let mut dial: DialValue = DIAL_START;
        let mut count_zeros: u32 = 0;
        for rot in rotations {
            let (new_dial, _) = rotate_dial_and_count_zeros_passed(dial, rot);
            dial = new_dial;
            if dial == 0 {
                count_zeros += 1;
            }
        }
        count_zeros
    }
}

impl ParsedPart2 for Day01 {
    type Part2Output = u32;

    fn part2(rotations: &Self::ParsedInput) -> Self::Part2Output {
        // iterate rotations and track 0's from function & when result is 0
        let mut dial: DialValue = DIAL_START;
        let mut count_zeros: u32 = 0;
        for rot in rotations {
            let (new_dial, zeros_passed) =
                rotate_dial_and_count_zeros_passed(dial, rot);
            dial = new_dial;
            count_zeros += u32::from(zeros_passed);
            if dial == 0 {
                count_zeros += 1;
            }
        }
        count_zeros
    }
}

impl_runnable_solution!(Day01 => ParsedPart2);

#[cfg(test)]
mod tests {
    use aoc_framework::ParseResult;

    use super::*;

    const EXAMPLE_INPUT: &str = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

    #[test]
    fn part1_solves_example() -> ParseResult<()> {
        let parsed = Day01::parse(EXAMPLE_INPUT)?;
        let result = Day01::part1(&parsed);
        assert_eq!(result, 3);
        Ok(())
    }

    #[test]
    fn rotate_left_to_zero_does_not_count() {
        let (_, zeros) = rotate_dial_and_count_zeros_passed(
            5,
            &Rotation {
                direction: Direction::Left,
                distance: 5,
            },
        );
        assert_eq!(zeros, 0);
    }

    #[test]
    fn rotate_left_from_zero_does_not_count() {
        let (_, zeros) = rotate_dial_and_count_zeros_passed(
            0,
            &Rotation {
                direction: Direction::Left,
                distance: 5,
            },
        );
        assert_eq!(zeros, 0);
    }

    #[test]
    fn over_rotate_left_to_zero_does_count() {
        let (_, zeros) = rotate_dial_and_count_zeros_passed(
            5,
            &Rotation {
                direction: Direction::Left,
                distance: 5 + u16::from(DIAL_MAX) + 1,
            },
        );
        assert_eq!(zeros, 1);
    }

    #[test]
    fn over_rotate_left_from_zero_does_count() {
        let (_, zeros) = rotate_dial_and_count_zeros_passed(
            0,
            &Rotation {
                direction: Direction::Left,
                distance: 5 + u16::from(DIAL_MAX) + 1,
            },
        );
        assert_eq!(zeros, 1);
    }

    #[test]
    fn rotate_right_to_zero_does_not_count() {
        let (_, zeros) = rotate_dial_and_count_zeros_passed(
            95,
            &Rotation {
                direction: Direction::Right,
                distance: 5,
            },
        );
        assert_eq!(zeros, 0);
    }

    #[test]
    fn rotate_right_from_zero_does_not_count() {
        let (_, zeros) = rotate_dial_and_count_zeros_passed(
            0,
            &Rotation {
                direction: Direction::Right,
                distance: 5,
            },
        );
        assert_eq!(zeros, 0);
    }

    #[test]
    fn over_rotate_right_to_zero_does_count() {
        let (_, zeros) = rotate_dial_and_count_zeros_passed(
            95,
            &Rotation {
                direction: Direction::Right,
                distance: 5 + u16::from(DIAL_MAX) + 1,
            },
        );
        assert_eq!(zeros, 1);
    }

    #[test]
    fn over_rotate_right_from_zero_does_count() {
        let (_, zeros) = rotate_dial_and_count_zeros_passed(
            0,
            &Rotation {
                direction: Direction::Right,
                distance: 5 + u16::from(DIAL_MAX) + 1,
            },
        );
        assert_eq!(zeros, 1);
    }

    #[test]
    fn part2_solves_example() -> ParseResult<()> {
        let parsed = Day01::parse(EXAMPLE_INPUT)?;
        let result = Day01::part2(&parsed);
        assert_eq!(result, 6);
        Ok(())
    }
}
