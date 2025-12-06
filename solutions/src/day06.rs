use aoc_framework::{
    ParseError, ParseResult, Part1, Part2, SolutionName, impl_runnable_solution,
};
use nalgebra::DMatrix;

use crate::util::parse::parse_grid;

/// Solution for the sixth day's puzzle.
///
/// # Part 1
///
/// Input is a math worksheet. It formats a list of problems; each hold a group
/// of numbers and an addition (`+`) or multiplication (`*`) operation to
/// collect the numbers by. The numbers and operation are in a column, with the
/// operation at the bottom. Alignment of the columns varies, so columns could
/// be described as whitespace separated.
///
/// Calculate the results of problems then return their sum.
///
/// # Part 2
///
/// Input column alignment is now relevant: a number occupies a character
/// column, numbers read right-to-left, reading most significant digit to least
/// significant digit downward.
///
/// Example:
///
/// ```ignore
/// 123
///  45
///   6
/// ```
///
/// This reads as numbers: `356`, `24`, and `1`.
///
/// It seems implied that operations are expected to be left-aligned, with the
/// last number.
///
/// Recalculate the sum of problem results.
pub struct Day06;

impl SolutionName for Day06 {
    const NAME: &'static str = "Day 6: Trash Compactor";
}

/// A problem's number type.
///
/// Observed largest numbers in 10^3, so should fit in u16.
/// Should still apply for Part 2 as it had 4 rows for numbers.
type Number = u16;
/// A number type for problem results.
///
/// Needs to be large to accumulate products and sums without overflow.
type ProblemResult = u64;

/// An operation to apply to a problem's numbers.
#[derive(Debug, PartialEq)]
enum Operation {
    /// Add numbers.
    Add,
    /// Multiply numbers.
    Multiply,
}

impl TryFrom<&str> for Operation {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "+" => Ok(Self::Add),
            "*" => Ok(Self::Multiply),
            _ => Err(ParseError::ParseString(value.into())),
        }
    }
}

impl TryFrom<char> for Operation {
    type Error = ParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Self::Add),
            '*' => Ok(Self::Multiply),
            _ => Err(ParseError::ParseChar(value)),
        }
    }
}

/// Representation of a problem, from a group of numbers and an operation.
#[derive(Debug)]
pub struct Problem(Vec<Number>, Operation);

impl Problem {
    /// Calculate the result of the problem, applying the operation to all
    /// numbers.
    fn calculate(&self) -> ProblemResult {
        let large_numbers_iter = self.0.iter().map(|&n| ProblemResult::from(n));
        match self.1 {
            Operation::Add => large_numbers_iter.sum(),
            Operation::Multiply => large_numbers_iter.product(),
        }
    }
}

impl Part1 for Day06 {
    type Part1Output = ProblemResult;

    fn part1(input: &str) -> ParseResult<Self::Part1Output> {
        // let's collect parse-able strings, left to right, top to bottom,
        // whitespace-separated as cells so a matrix can be built from it

        let lines: Vec<&str> = input.lines().collect();

        let rows = lines.len();
        if rows == 0 {
            return Err(ParseError::EmptyInput);
        }

        let cols = lines
            .first()
            .map_or(0, |line| line.split_whitespace().count());

        let mut cells: Vec<String> =
            Vec::with_capacity(rows.saturating_mul(cols));

        for (line_idx, &line) in lines.iter().enumerate() {
            let line_cells: Vec<&str> = line.split_whitespace().collect();
            if line_cells.is_empty() {
                return Err(ParseError::invalid_line_from_zero_index(
                    line_idx,
                    ParseError::EmptyLine,
                ));
            }
            if line_cells.len() != cols {
                return Err(ParseError::invalid_line_from_zero_index(
                    line_idx,
                    ParseError::LineLength {
                        expected: cols,
                        actual: line_cells.len(),
                    },
                ));
            }

            for cell in line_cells {
                cells.push(cell.into());
            }
        }

        let matrix = DMatrix::from_row_iterator(rows, cols, cells);

        // then let's iterate matrix by columns, working across the column to
        // parse numbers & operation to create problems from

        let mut problems: Vec<Problem> = Vec::with_capacity(cols);

        for column in matrix.column_iter() {
            let mut numbers: Vec<Number> = Vec::with_capacity(rows - 1);
            let mut operation_opt: Option<ParseResult<Operation>> = None;

            let mut column_iter = column.iter().peekable();
            while let Some(cell) = column_iter.next() {
                if column_iter.peek().is_none() {
                    // expect operation as last cell
                    operation_opt = Some(Operation::try_from(cell.as_str()));
                } else {
                    // expect number
                    let num: Number = cell.parse().map_err(|source| {
                        ParseError::parse_int_from_str(cell, source)
                    })?;
                    numbers.push(num);
                }
            }

            let operation = operation_opt.expect("operation was not parsed")?;
            problems.push(Problem(numbers, operation));
        }

        // all that's left is calculating problems and finding sum
        Ok(problems.iter().map(Problem::calculate).sum())
    }
}

impl Part2 for Day06 {
    type Part2Output = ProblemResult;

    fn part2(input: &str) -> ParseResult<Self::Part2Output> {
        fn digits_to_number(digits: &[&char]) -> ParseResult<Number> {
            digits
                .iter()
                .filter(|c| !c.is_whitespace())
                .map(|&c| {
                    c.to_digit(10)
                        .map(|d: u32| {
                            Number::try_from(d)
                                .expect("failed to cast u32 to Number")
                        })
                        .ok_or(ParseError::ParseChar(*c))
                })
                .try_fold(0, |acc, digit_result| {
                    digit_result.map(|digit| acc * 10 + digit)
                })
        }

        // let's format input to character matrix; keep characters as-is
        let matrix = parse_grid(input, |_position, character| Ok(character))?;

        let mut problems: Vec<Problem> = Vec::new();

        // let's walk over each character column
        // - problem's first column should have operator on last line,
        //   otherwise should be whitespace
        // - problems are separated if all lines have whitespace
        let mut numbers: Vec<Number> = Vec::with_capacity(4);
        let mut operator_opt: Option<Operation> = None;
        for column in matrix.column_iter() {
            let column_vec: Vec<&char> = column.iter().collect();

            if column_vec.iter().all(|c| c.is_whitespace()) {
                // finished a problem
                let operation =
                    operator_opt.expect("operation not resolved for problem");
                problems.push(Problem(numbers.clone(), operation));

                // clear tracking
                operator_opt = None;
                numbers.clear();
            } else {
                if column_vec.last().is_some_and(|c| !c.is_whitespace()) {
                    // starting a new problem
                    assert!(
                        operator_opt.is_none(),
                        "still tracking a previous operation"
                    );

                    operator_opt = Some(
                        column_vec
                            .last()
                            .map(|&&c| Operation::try_from(c))
                            .expect("failed to get last character of column")?,
                    );
                }

                // collect numbers for a problem
                numbers.push(digits_to_number(
                    &column_vec[..column_vec.len() - 1],
                )?);
            }
        }
        // handle any trailing problem data not finished in loop
        if let Some(operation) = operator_opt {
            problems.push(Problem(numbers.clone(), operation));
        }

        Ok(problems.iter().map(Problem::calculate).sum())
    }
}

impl_runnable_solution!(Day06 => Part2);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

    #[test]
    fn part1_solves_example() -> ParseResult<()> {
        let result = Day06::part1(EXAMPLE_INPUT)?;
        assert_eq!(result, 4_277_556);
        Ok(())
    }

    #[test]
    fn part2_calculates_individual_problem() -> ParseResult<()> {
        let problem = r"123
 45
  6
*  
";
        let result = Day06::part2(problem)?;
        assert_eq!(result, 8544);
        Ok(())
    }

    #[test]
    fn part2_solves_example() -> ParseResult<()> {
        let result = Day06::part2(EXAMPLE_INPUT)?;
        assert_eq!(result, 3_263_827);
        Ok(())
    }
}
