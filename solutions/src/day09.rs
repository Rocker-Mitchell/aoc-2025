use std::collections::{HashSet, VecDeque};

use aoc_framework::{
    ParseError, ParseResult, ParsedPart1, ParsedPart2, SolutionName,
    impl_runnable_solution,
};
use nalgebra::{DMatrix, Point2};

use crate::util::parse::parse_lines;

/// Solution for ninth day's puzzle.
///
/// # Input
///
/// Input is grid coordinates of red tiles on a floor: coordinates line
/// separated, components comma separated (X,Y).
///
/// # Part 1
///
/// Picking out two tiles as opposite corners of a rectangle, find the largest
/// rectangle (by area) that can be made and return its area.
///
/// # Part 2
///
/// There are green tiles that connect the red tiles, specifically in the order
/// of the input's sequence including connecting the last red tile back to the
/// first. Connections form horizontal or vertical lines, no diagonals. So, the
/// red tile coordinate sequence only has one component change per step. Also,
/// the space inside this "path" of tiles is filled with green tiles.
///
/// Now picking out rectangles with red opposing corners requires all tiles in
/// the rectangle are red or green. The solution is still the largest area.
pub struct Day09;

impl SolutionName for Day09 {
    const NAME: &'static str = "Day 9: Movie Theater";
}

/// Data type of a coordinate dimension.
///
/// Needs large size for multiplications.
type Dimension = u64;

impl ParsedPart1 for Day09 {
    type ParsedInput = Vec<Point2<Dimension>>;

    fn parse(input: &str) -> ParseResult<Self::ParsedInput> {
        let coords: Self::ParsedInput = parse_lines(input, |line| {
            let (x_str, y_str) = line
                .split_once(',')
                .ok_or(ParseError::NoDelimiter(",".into()))?;
            let x: Dimension = x_str.parse().map_err(|source| {
                ParseError::parse_int_from_str(x_str, source)
            })?;
            let y: Dimension = y_str.parse().map_err(|source| {
                ParseError::parse_int_from_str(y_str, source)
            })?;
            Ok(Point2::new(x, y))
        })
        .collect::<ParseResult<_>>()?;

        if coords.is_empty() {
            Err(ParseError::EmptyInput)
        } else {
            Ok(coords)
        }
    }

    type Part1Output = Dimension;

    fn part1(coords: &Self::ParsedInput) -> Self::Part1Output {
        // pair through points, calculate area, find maximum
        (0..(coords.len() - 1))
            .flat_map(|i| ((i + 1)..coords.len()).map(move |j| (i, j)))
            .map(|(p_idx, q_idx)| {
                // example shows a case of a thinnest rectangle with matching
                //   y-component treating the height as 1; points aren't to
                //   corners, they're tiles that already have dimensions 1 by 1
                // so, compensate by adding 1 after each difference
                let width = coords[p_idx].x.abs_diff(coords[q_idx].x) + 1;
                let height = coords[p_idx].y.abs_diff(coords[q_idx].y) + 1;
                width
                    .checked_mul(height)
                    .expect("overflow when multiplying for area")
            })
            .max()
            .expect("failed to find maximum")
    }
}

struct Grid {
    x_mapping: Vec<Dimension>,
    y_mapping: Vec<Dimension>,
    matrix: DMatrix<bool>,
}

impl Grid {
    fn new(coords: &[Point2<Dimension>]) -> Self {
        let mut unique_x = HashSet::new();
        let mut unique_y = HashSet::new();
        for point in coords {
            unique_x.insert(point.x);
            unique_y.insert(point.y);
        }
        let mut x_mapping = Vec::from_iter(unique_x);
        x_mapping.sort_unstable();
        let mut y_mapping = Vec::from_iter(unique_y);
        y_mapping.sort_unstable();
        let matrix = DMatrix::repeat(y_mapping.len(), x_mapping.len(), false);
        let mut grid = Self {
            x_mapping,
            y_mapping,
            matrix,
        };
        grid.populate_matrix(coords);
        grid
    }

    fn to_mapped_row_col(&self, point: Point2<Dimension>) -> (usize, usize) {
        let row_opt = self
            .y_mapping
            .iter()
            .enumerate()
            .find(|&(_, &y)| point.y == y)
            .map(|(i, _)| i);
        let col_opt = self
            .x_mapping
            .iter()
            .enumerate()
            .find(|&(_, &x)| point.x == x)
            .map(|(i, _)| i);
        match (row_opt, col_opt) {
            (Some(row), Some(col)) => (row, col),
            _ => panic!("failed to find mapped row-col for point: {point:?}"),
        }
    }

    fn populate_matrix(&mut self, coords: &[Point2<Dimension>]) {
        let nrows = self.matrix.nrows();
        let ncols = self.matrix.ncols();

        // make sure we start with `false`
        self.matrix.fill(false);

        // generate borders formed by coord sequence
        let mut border_matrix = self.matrix.clone_owned();
        let mut mapped_coords: Vec<(usize, usize)> =
            coords.iter().map(|&p| self.to_mapped_row_col(p)).collect();
        // append first item to end so windows will iterate the wraparound to start
        mapped_coords.push(mapped_coords[0]);
        for window in mapped_coords.windows(2) {
            let (start_row, start_col) = window[0];
            let (end_row, end_col) = window[1];

            let is_horizontal =
                (start_row == end_row) && (start_col != end_col);
            let is_vertical = (start_col == end_col) && (start_row != end_row);
            assert!(
                is_horizontal || is_vertical,
                "processed border that isn't horizontal or vertical: {:?} -> {:?}",
                window[0],
                window[1]
            );

            let min_row = start_row.min(end_row);
            let min_col = start_col.min(end_col);
            let max_row = start_row.max(end_row);
            let max_col = start_col.max(end_col);
            // already observed need to add 1 to compensate for 1 by 1 tile
            // size
            let width = max_col - min_col + 1;
            let height = max_row - min_row + 1;

            let mut view =
                border_matrix.view_mut((min_row, min_col), (height, width));
            view.fill(true);
        }

        // so it seems a flood fill / BFS / DFS is needed to figure out filling

        let mut queue = VecDeque::new();
        // modify self.matrix to act as a mask of outer cells

        // iterate outer boundary and push `false` cells
        let outer_boundary = (0..nrows)
            .flat_map(|row| vec![(row, 0), (row, ncols - 1)])
            .chain(
                (1..(ncols - 1))
                    .flat_map(|col| vec![(0, col), (nrows - 1, col)]),
            );
        for index in outer_boundary {
            if !border_matrix[index] && !self.matrix[index] {
                queue.push_back(index);
                self.matrix[index] = true;
            }
        }

        // BFS from outer cells
        while let Some((row, col)) = queue.pop_front() {
            let neighbors = [
                (row.wrapping_sub(1), col),
                (row + 1, col),
                (row, col.wrapping_sub(1)),
                (row, col + 1),
            ];

            for &(neighbor_row, neighbor_col) in &neighbors {
                if neighbor_row < nrows
                    && neighbor_col < ncols
                    && !border_matrix[(neighbor_row, neighbor_col)]
                    && !self.matrix[(neighbor_row, neighbor_col)]
                {
                    self.matrix[(neighbor_row, neighbor_col)] = true;
                    queue.push_back((neighbor_row, neighbor_col));
                }
            }
        }

        // invert mask that was made for filled shape
        for cell_ref in self.matrix.iter_mut() {
            *cell_ref = !*cell_ref;
        }
    }

    fn contains_valid_tiles(
        &self,
        p: Point2<Dimension>,
        q: Point2<Dimension>,
    ) -> bool {
        let (p_row, p_col) = self.to_mapped_row_col(p);
        let (q_row, q_col) = self.to_mapped_row_col(q);
        let start_row = p_row.min(q_row);
        let start_col = p_col.min(q_col);
        let end_row = p_row.max(q_row);
        let end_col = p_col.max(q_col);
        let width = end_col - start_col + 1;
        let height = end_row - start_row + 1;

        let view = self.matrix.view((start_row, start_col), (height, width));
        view.iter().all(|&cell| cell)
    }
}

impl ParsedPart2 for Day09 {
    type Part2Output = Dimension;

    fn part2(coords: &Self::ParsedInput) -> Self::Part2Output {
        let grid = Grid::new(coords);

        (0..(coords.len() - 1))
            .flat_map(|i| ((i + 1)..coords.len()).map(move |j| (i, j)))
            .filter(|&(p_idx, q_idx)| {
                grid.contains_valid_tiles(coords[p_idx], coords[q_idx])
            })
            .map(|(p_idx, q_idx)| {
                let width = coords[p_idx].x.abs_diff(coords[q_idx].x) + 1;
                let height = coords[p_idx].y.abs_diff(coords[q_idx].y) + 1;
                width
                    .checked_mul(height)
                    .expect("overflow when multiplying for area")
            })
            .max()
            .expect("failed to find maximum")
    }
}

impl_runnable_solution!(Day09 => ParsedPart2);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

    #[test]
    fn part1_solves_example() -> ParseResult<()> {
        let parsed = Day09::parse(EXAMPLE_INPUT)?;
        let result = Day09::part1(&parsed);
        assert_eq!(result, 50);
        Ok(())
    }

    #[test]
    fn part2_solves_example() -> ParseResult<()> {
        let parsed = Day09::parse(EXAMPLE_INPUT)?;
        let result = Day09::part2(&parsed);
        assert_eq!(result, 24);
        Ok(())
    }
}
