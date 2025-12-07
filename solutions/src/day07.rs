use std::collections::{HashMap, HashSet};

use aoc_framework::{
    ParseError, ParseResult, ParsedPart1, ParsedPart2, SolutionName,
    impl_runnable_solution,
};
use nalgebra::DMatrix;

use crate::util::parse::parse_grid;

/// Solution for seventh day's puzzle.
///
/// # Input
///
/// Input is a diagram of a manifold as a character grid.
///
/// `S` marks the entry of a beam, `^` a splitter, and `.` open space.
///
/// # Part 1
///
/// Beams travel down through open space, but encountering a splitter will stop
/// the beam to create new beams: one left & one right of the splitter.
///
/// > From the example, it appears if two splitters cause a split that overlaps
/// > two beams in the space between, it counts as one beam.
///
/// Determine how many times the beams will split.
///
/// # Part 2
///
/// Now treat the beam as a quantum particle, so a splitter directs the
/// particle either to the left or right.
///
/// Determine the unique permutations of paths a particle takes through the
/// manifold.
pub struct Day07;

impl SolutionName for Day07 {
    const NAME: &'static str = "Day 7: Laboratories";
}

/// Representation of a grid cell in the manifold.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ManifoldCell {
    /// The beam entry point.
    Start,
    /// A beam splitter.
    Splitter,
    /// Open space.
    Open,
}

impl TryFrom<char> for ManifoldCell {
    type Error = ParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'S' => Ok(Self::Start),
            '^' => Ok(Self::Splitter),
            '.' => Ok(Self::Open),
            _ => Err(ParseError::ParseChar(value)),
        }
    }
}

impl ParsedPart1 for Day07 {
    /// A tuple of the parsed manifold and the start column.
    type ParsedInput = (DMatrix<ManifoldCell>, usize);

    fn parse(input: &str) -> ParseResult<Self::ParsedInput> {
        let mut start_col_opt: Option<usize> = None;
        let manifold: DMatrix<ManifoldCell> =
            parse_grid(input, |position, character| {
                let cell = ManifoldCell::try_from(character)?;
                if cell == ManifoldCell::Start {
                    // TODO should have errors if we parse more than one start,
                    // or start not on first row
                    start_col_opt = Some(
                        position
                            .x
                            .try_into()
                            .expect("failed to convert x coordinate to usize"),
                    );
                }
                Ok(cell)
            })?;

        // TODO should have error instead of panic
        let start_col = start_col_opt.expect("did not parse a start in input");
        Ok((manifold, start_col))
    }

    type Part1Output = u32;

    fn part1(parsed: &Self::ParsedInput) -> Self::Part1Output {
        let (manifold, start_col) = parsed;

        // track beams by column
        // - example suggests overlapping generated beams count as one, so
        //   using a set
        // - init with start column
        let mut beams_cols: HashSet<usize> = HashSet::from([*start_col]);

        let mut count_splits: Self::Part1Output = 0;

        // scan across rows
        for row in 0..manifold.nrows() {
            // eke some performance by only iterating known beam columns
            let current_beams: Vec<usize> =
                beams_cols.iter().copied().collect();
            for col in current_beams {
                if manifold[(row, col)] == ManifoldCell::Splitter {
                    // split beam
                    beams_cols.remove(&col);
                    beams_cols.insert(col - 1);
                    beams_cols.insert(col + 1);

                    // count splits; can't use size of beams set as some
                    // beams can combine
                    count_splits += 1;
                }
            }
        }

        count_splits
    }
}

impl ParsedPart2 for Day07 {
    type Part2Output = u64;

    fn part2(parsed: &Self::ParsedInput) -> Self::Part2Output {
        let (manifold, start_col) = parsed;

        // count how many particles are in a column, handling overlaps unlike
        // the previous part
        let mut particles_in_column: HashMap<usize, Self::Part2Output> =
            HashMap::from([(*start_col, 1)]);

        for row in 0..manifold.nrows() {
            // iterate columns tracking non-zero particles
            let current_columns: Vec<(usize, Self::Part2Output)> =
                particles_in_column
                    .iter()
                    .filter(|&(_, &v)| v > 0)
                    .map(|(&k, &v)| (k, v))
                    .collect();
            for (col, particles) in current_columns {
                if manifold[(row, col)] == ManifoldCell::Splitter {
                    // subtract all particles from column
                    particles_in_column.insert(col, 0);
                    // add the number of particles to sides
                    let left_ref =
                        particles_in_column.entry(col - 1).or_insert(0);
                    *left_ref = left_ref
                        .checked_add(particles)
                        .expect("overflow adding particles to left");
                    let right_ref =
                        particles_in_column.entry(col + 1).or_insert(0);
                    *right_ref = right_ref
                        .checked_add(particles)
                        .expect("overflow adding particles to right");
                }
            }
        }

        // count particles generated as unique paths
        particles_in_column.values().sum()
    }
}

impl_runnable_solution!(Day07 => ParsedPart2);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    #[test]
    fn part1_solves_example() -> ParseResult<()> {
        let parsed = Day07::parse(EXAMPLE_INPUT)?;
        let result = Day07::part1(&parsed);
        assert_eq!(result, 21);
        Ok(())
    }

    #[test]
    fn part2_solves_example() -> ParseResult<()> {
        let parsed = Day07::parse(EXAMPLE_INPUT)?;
        let result = Day07::part2(&parsed);
        assert_eq!(result, 40);
        Ok(())
    }
}
