use aoc_framework::{
    ParseError, ParseResult, ParsedPart1, ParsedPart2, SolutionName,
    impl_runnable_solution,
};
use nalgebra::{DMatrix, Vector2};

use crate::util::matrix::{MatrixPoint, MatrixPointAccess};
use crate::util::parse::parse_grid;

/// Solution for the fourth day's puzzle.
///
/// # Input
///
/// Input is a diagram of locations of paper rolls on a grid: `@` for a roll,
/// `.` otherwise.
///
/// # Part 1
///
/// A forklift can get a roll if there's fewer than 4 rolls in the 8 adjacent
/// positions (so cardinal and diagonal directions).
///
/// Find how many rolls can be accessed in the diagram.
///
/// # Part 2
///
/// A forklift removing rolls can cause more rolls to be accessible.
///
/// Find how many rolls total can be removed; iterate removals as many times as
/// needed until no more rolls are accessible.
pub struct Day04;

impl SolutionName for Day04 {
    const NAME: &'static str = "Day 4: Printing Department";
}

/// Content of a grid cell.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GridCell {
    /// A paper roll.
    Roll,
    /// Empty space.
    Empty,
}

impl TryFrom<char> for GridCell {
    type Error = ParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '@' => Ok(Self::Roll),
            '.' => Ok(Self::Empty),
            _ => Err(ParseError::ParseChar(value)),
        }
    }
}

/// Count the adjacent rolls around a position in the grid.
///
/// Adjacency can be in cardinal directions or diagonal directions.
fn count_adjacent_rolls(
    grid: &DMatrix<GridCell>,
    target: MatrixPoint,
) -> usize {
    let neighbor_offsets: [Vector2<i32>; 8] = [
        Vector2::new(1, 0),
        Vector2::new(1, 1),
        Vector2::new(0, 1),
        Vector2::new(-1, 1),
        Vector2::new(-1, 0),
        Vector2::new(-1, -1),
        Vector2::new(0, -1),
        Vector2::new(1, -1),
    ];

    // iterate offsets, get neighbor values, check they're a roll
    neighbor_offsets
        .iter()
        .filter(|&offset| {
            grid.get_at_point(target + offset)
                .is_some_and(|&neighbor| neighbor == GridCell::Roll)
        })
        .count()
}

/// Check if a position in the grid holds an available roll.
fn is_available_roll(grid: &DMatrix<GridCell>, target: MatrixPoint) -> bool {
    // expect target is a roll & fewer than 4 adjacent rolls
    grid.get_at_point(target).is_some_and(|&v| {
        v == GridCell::Roll && count_adjacent_rolls(grid, target) < 4
    })
}

impl ParsedPart1 for Day04 {
    type ParsedInput = DMatrix<GridCell>;

    fn parse(input: &str) -> ParseResult<Self::ParsedInput> {
        let grid: Self::ParsedInput =
            parse_grid(input, |_point, character| {
                GridCell::try_from(character)
            })?;

        // TODO I want no rows or columns, not all values are 0
        if grid.is_empty() {
            Err(ParseError::EmptyInput)
        } else {
            Ok(grid)
        }
    }

    type Part1Output = usize;

    fn part1(grid: &Self::ParsedInput) -> Self::Part1Output {
        // iterate across points, check availability, then count what was found
        grid.points()
            .filter(|&point| is_available_roll(grid, point))
            .count()
    }
}

impl ParsedPart2 for Day04 {
    type Part2Output = usize;

    fn part2(grid: &Self::ParsedInput) -> Self::Part2Output {
        // need grid which we can modify during processing
        let mut grid = grid.clone();

        let mut count: Self::Part2Output = 0;
        loop {
            let available_rolls: Vec<MatrixPoint> = grid
                .points()
                .filter(|&point| is_available_roll(&grid, point))
                .collect();

            if available_rolls.is_empty() {
                break;
            }

            count += available_rolls.len();

            // remove the available rolls from the grid for next loop
            for point in available_rolls {
                if let Some(value_ref) = grid.get_at_point_mut(point) {
                    *value_ref = GridCell::Empty;
                }
            }
        }
        count
    }
}

impl_runnable_solution!(Day04 => ParsedPart2);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    #[test]
    fn count_adjacent_rolls_zero() {
        let matrix: DMatrix<GridCell> = DMatrix::from_row_slice(
            3,
            3,
            &[
                GridCell::Empty,
                GridCell::Empty,
                GridCell::Empty,
                GridCell::Empty,
                GridCell::Roll,
                GridCell::Empty,
                GridCell::Empty,
                GridCell::Empty,
                GridCell::Empty,
            ],
        );
        assert_eq!(count_adjacent_rolls(&matrix, MatrixPoint::new(1, 1)), 0);
    }

    #[test]
    fn count_adjacent_rolls_one() {
        let center_point = MatrixPoint::new(1, 1);

        let matrix_top_right: DMatrix<GridCell> = DMatrix::from_row_slice(
            3,
            3,
            &[
                GridCell::Empty,
                GridCell::Empty,
                GridCell::Roll,
                GridCell::Empty,
                GridCell::Roll,
                GridCell::Empty,
                GridCell::Empty,
                GridCell::Empty,
                GridCell::Empty,
            ],
        );
        assert_eq!(count_adjacent_rolls(&matrix_top_right, center_point), 1);

        let matrix_bottom: DMatrix<GridCell> = DMatrix::from_row_slice(
            3,
            3,
            &[
                GridCell::Empty,
                GridCell::Empty,
                GridCell::Empty,
                GridCell::Empty,
                GridCell::Roll,
                GridCell::Empty,
                GridCell::Empty,
                GridCell::Roll,
                GridCell::Empty,
            ],
        );
        assert_eq!(count_adjacent_rolls(&matrix_bottom, center_point), 1);
    }

    #[test]
    fn part1_solves_example() -> ParseResult<()> {
        let parsed = Day04::parse(EXAMPLE_INPUT)?;
        let result = Day04::part1(&parsed);
        assert_eq!(result, 13);
        Ok(())
    }

    #[test]
    fn part2_solves_example() -> ParseResult<()> {
        let parsed = Day04::parse(EXAMPLE_INPUT)?;
        let result = Day04::part2(&parsed);
        assert_eq!(result, 43);
        Ok(())
    }
}
