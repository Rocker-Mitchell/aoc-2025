use aoc_framework::{
    ParseError, ParseResult, ParsedPart1, ParsedPart2, SolutionName,
    impl_runnable_solution,
};

use crate::util::parse::{parse_lines, parse_lines_with_offset};

/// Solution for the fifth day's puzzle.
///
/// # Input
///
/// Input is an inventory of ingredients, formatted as a list of fresh
/// ingredient ID ranges (dash (`-`) separated), an empty line, and a list of
/// available ingredient IDs.
///
/// A fresh ID range is inclusive, and ranges can overlap.
///
/// # Part 1
///
/// Count how many available ingredient IDs are fresh.
///
/// # Part 2
///
/// Evaluate how many possible unique ingredient IDs are fresh according to the
/// ranges.
pub struct Day05;

impl SolutionName for Day05 {
    const NAME: &'static str = "Day 5: Cafeteria";
}

/// An ingredient ID.
type IngredientId = u64;
/// A range of fresh ingredient IDs, inclusive.
type FreshIngredientRange = (IngredientId, IngredientId);

/// An inventory of ingredients, made from a list of fresh ingredient ID ranges
/// and a list of available ingredient IDs.
pub struct Inventory(Vec<FreshIngredientRange>, Vec<IngredientId>);

impl Inventory {
    #[expect(
        clippy::print_stdout,
        dead_code,
        reason = "helper code used for inspecting input traits, shouldn't be in final code"
    )]
    fn inspect(&self) {
        println!("inspecting inventory...");
        println!(
            "- any range in backwards order: {:?}",
            self.0.iter().any(|range| range.0 > range.1)
        );
        println!(
            "- lowest range bound: {}",
            self.0
                .iter()
                .map(|range| range.0)
                .min()
                .expect("lowest bound could not be found")
        );
        println!(
            "- highest range bound: {}",
            self.0
                .iter()
                .map(|range| range.1)
                .max()
                .expect("highest bound could not be found")
        );
        let ranges_with_sizes: Vec<_> = self
            .0
            .iter()
            .map(|range| (range, range.1.saturating_sub(range.0)))
            .collect();
        let min_size_range = ranges_with_sizes
            .iter()
            .min_by_key(|(_, size)| size)
            .expect("min by size could not be found");
        println!(
            "- smallest range size: {}-{} at size {}",
            min_size_range.0.0, min_size_range.0.1, min_size_range.1
        );
        let max_size_range = ranges_with_sizes
            .iter()
            .max_by_key(|(_, size)| size)
            .expect("max by size could not be found");
        println!(
            "- largest range size: {}-{} at size {}",
            max_size_range.0.0, max_size_range.0.1, max_size_range.1
        );
        println!(
            "- smallest available ID: {}",
            self.1
                .iter()
                .min()
                .expect("min available could not be found")
        );
        println!(
            "- largest available ID: {}",
            self.1
                .iter()
                .max()
                .expect("max available could not be found")
        );
    }
}

/// Collapse the overlaps between ranges into a new collection of ranges.
fn collapse_ranges(
    mut ranges: Vec<FreshIngredientRange>,
) -> Vec<FreshIngredientRange> {
    // sort by range start, reverse so we pop in ascending order
    ranges.sort_by_key(|range| range.0);
    ranges.reverse();

    let mut new_ranges: Vec<FreshIngredientRange> = Vec::new();
    while let Some((old_start, old_end)) = ranges.pop() {
        if let Some((_new_start, new_end)) = new_ranges.last_mut() {
            // check if old start is within the last new range
            if old_start <= *new_end {
                // check old end is larger than new
                if old_end > *new_end {
                    // update new range's end to old end
                    *new_end = old_end;
                }
            } else {
                // need a new range added
                new_ranges.push((old_start, old_end));
            }
        } else {
            // create new range from old
            new_ranges.push((old_start, old_end));
        }
    }
    new_ranges
}

impl ParsedPart1 for Day05 {
    type ParsedInput = Inventory;

    fn parse(input: &str) -> ParseResult<Self::ParsedInput> {
        // detect windows-style CRLF, fallback to LF
        let delimiter = if input.contains("\r\n\r\n") {
            "\r\n\r\n"
        } else {
            "\n\n"
        };
        let (ranges_input, ids_input) = input
            .split_once(delimiter)
            .ok_or_else(|| ParseError::NoChunkDelimiter(delimiter.into()))?;

        if ranges_input.is_empty() {
            return Err(ParseError::EmptyChunk {
                chunk_number: 1,
                description: "fresh ingredient ranges".into(),
            });
        }
        if ids_input.is_empty() {
            return Err(ParseError::EmptyChunk {
                chunk_number: 2,
                description: "available ingredient IDs".into(),
            });
        }

        let ranges: Vec<FreshIngredientRange> =
            parse_lines(ranges_input, |line| {
                let (first_id_str, second_id_str) = line
                    .split_once('-')
                    .ok_or_else(|| ParseError::NoDelimiter('-'.into()))?;

                let first_id =
                    first_id_str.parse::<IngredientId>().map_err(|source| {
                        ParseError::parse_int_from_str(first_id_str, source)
                    })?;

                let second_id = second_id_str.parse::<IngredientId>().map_err(
                    |source| {
                        ParseError::parse_int_from_str(second_id_str, source)
                    },
                )?;

                Ok((first_id, second_id))
            })
            .collect::<ParseResult<_>>()?;

        // line offset should be length of ranges plus 1 for empty line
        let available_ids: Vec<IngredientId> =
            parse_lines_with_offset(ids_input, ranges.len() + 1, |line| {
                line.parse::<IngredientId>().map_err(|source| {
                    ParseError::parse_int_from_str(line, source)
                })
            })
            .collect::<ParseResult<_>>()?;

        let inventory: Self::ParsedInput = Inventory(ranges, available_ids);
        /*
        inspection notes
        - lowest bound was in 10^12, highest in 10^14
        - smallest range size was 0, largest in 10^12
        - smallest available in 10^12
        - largest available in 10^14
        */
        //inventory.inspect();

        Ok(inventory)
    }

    type Part1Output = usize;

    fn part1(inventory: &Self::ParsedInput) -> Self::Part1Output {
        let Inventory(fresh_ranges, available_ids) = inventory;

        // friend shared to collapse ranges so there's no overlaps, better
        // performance
        let collapsed_ranges = collapse_ranges(fresh_ranges.clone());

        available_ids
            .iter()
            .filter(|&&id| {
                collapsed_ranges
                    .iter()
                    .any(|(start, end)| id >= *start && id <= *end)
            })
            .count()
    }
}

impl ParsedPart2 for Day05 {
    type Part2Output = u64;

    fn part2(inventory: &Self::ParsedInput) -> Self::Part2Output {
        let Inventory(fresh_ranges, _) = inventory;

        // this part feels easier than first, I already got code to collapse
        // ranges to be unique
        let collapsed_ranges = collapse_ranges(fresh_ranges.clone());

        // knowing there are ranges of size 0, they'd have the one ID to count
        // but end - start would miss that; so, add 1 to difference
        collapsed_ranges
            .iter()
            .map(|(start, end)| end - start + 1)
            .sum()
    }
}

impl_runnable_solution!(Day05 => ParsedPart2);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

    #[test]
    fn part1_solves_example() -> ParseResult<()> {
        let parsed = Day05::parse(EXAMPLE_INPUT)?;
        let result = Day05::part1(&parsed);
        assert_eq!(result, 3);
        Ok(())
    }

    #[test]
    fn part2_solves_example() -> ParseResult<()> {
        let parsed = Day05::parse(EXAMPLE_INPUT)?;
        let result = Day05::part2(&parsed);
        assert_eq!(result, 14);
        Ok(())
    }
}
