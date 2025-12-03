use aoc_framework::{
    ParseError, ParseResult, ParsedPart1, ParsedPart2, SolutionName,
    impl_runnable_solution,
};

use crate::util::parse::parse_lines;

/// Solution for the third day's puzzle.
///
/// # Input
///
/// Input holds "joltage" ratings of batteries, where a joltage is a digit from
/// 1 to 9. The input formats as lines of digit sequences, representing banks
/// of batteries.
///
/// # Part 1
///
/// In each bank, exactly 2 batteries should be turned on; the bank joltage
/// will equal the number formed of those batteries. Example: with `12345`,
/// turn on `2` & `4` for `24` jolts. Note, batteries can't be rearranged.
///
/// Find the largest possible joltage per bank, then sum the results.
///
/// # Part 2
///
/// Scale up to turning on exactly 12 batteries per bank.
///
/// Continue to find max bank joltages and return the sum.
pub struct Day03;

impl SolutionName for Day03 {
    const NAME: &'static str = "Day 3: Lobby";
}

/// A battery rating. Expect digits 1 to 9.
type Joltage = u8;
/// A bank of batteries, represented as a sequence of [`Joltage`] ratings.
type Bank = Vec<Joltage>;

/// Calculate the maximum joltage of a battery bank.
///
/// Two batteries will be turned on in the bank to determine its joltage.
fn max_joltage(bank: &Bank) -> u8 {
    assert!(
        bank.len() >= 2,
        "can't calculate max joltage with bank length less than 2: len = {}",
        bank.len()
    );

    // I'm thinking: find largest number in 0..len-1 as first battery, then
    // largest number in first_idx+1..len as second

    // max_by_key() returns last largest element, but want earliest index;
    // reverse the iterable after enumerate and before max_by_key
    let &(first_idx, &first_max) = &bank[..bank.len() - 1]
        .iter()
        .enumerate()
        .rev()
        .max_by_key(|&(_, &value)| value)
        .expect("failed to find first max");

    let &&second_max = &bank[first_idx + 1..]
        .iter()
        .max()
        .expect("failed to find second max");

    // format max joltage as first max in tens position then second max in ones
    // position
    (first_max * 10) + second_max
}

impl ParsedPart1 for Day03 {
    type ParsedInput = Vec<Bank>;

    fn parse(input: &str) -> aoc_framework::ParseResult<Self::ParsedInput> {
        // not trying char::to_digit() as that compounds into needing to try
        // converting its u32 down, error handling both, and basically needing
        // to add new ParseError's around these cases that'd need to abstract
        // what task was attempted & the source char; ParseError::ParseInt was
        // already working, keep using it
        let banks: Self::ParsedInput = parse_lines(input, |line| {
            if line.is_empty() {
                Err(ParseError::EmptyLine)
            } else {
                line.chars()
                    .map(|c| {
                        let string = c.to_string();
                        string.parse::<Joltage>().map_err(|source| {
                            ParseError::parse_int_from_str(&string, source)
                        })
                    })
                    .collect::<ParseResult<Bank>>()
            }
        })
        .collect::<ParseResult<_>>()?;

        if banks.is_empty() {
            Err(ParseError::EmptyInput)
        } else {
            Ok(banks)
        }
    }

    type Part1Output = u32;

    fn part1(banks: &Self::ParsedInput) -> Self::Part1Output {
        let mut sum: Self::Part1Output = 0;
        for bank in banks {
            let max = max_joltage(bank);
            sum += Self::Part1Output::from(max);
        }
        sum
    }
}

/// Calculate the maximum joltage of a battery bank.
///
/// This takes a slice of a battery bank and the number of batteries to turn on
/// in that slice.
///
/// The function recursively calculates with `batteries-1` for a result.
fn recursive_max_joltage(bank: &[Joltage], batteries: u8) -> u64 {
    assert!(
        batteries != 0,
        "can't have a max joltage with zero batteries on"
    );
    assert!(
        bank.len() >= batteries as usize,
        "can't calculate max joltage, expected {} batteries in bank but got {}",
        batteries,
        bank.len()
    );
    // with up to 12 batteries, need to return type large enough to hold 10^11
    // - u64 should handle up to 1.84x10^19

    if batteries > 1 {
        // find the maximum & its index in the upper sub-slice of bank
        // - exclude batteries - 1 items from end of slice
        // - max_by_key() returns latest largest element, but we want earliest
        //   index; reverse iterable after enumerating
        let end = bank.len() - batteries as usize + 1;
        let &(max_index, &max) = &bank[..end]
            .iter()
            .enumerate()
            .rev()
            .max_by_key(|&(_, &value)| value)
            .expect("failed to find maximum");

        // recursively calculate from sub-slice after the found maximum
        let recursive_max =
            recursive_max_joltage(&bank[max_index + 1..], batteries - 1);

        // multiply our max by 10^(batteries-1) for digit position, then add
        // the recursive result
        (u64::from(max) * 10_u64.pow(u32::from(batteries - 1))) + recursive_max
    } else {
        // base case
        // don't care about sub-slicing, indexes, or digit position; just find
        //   maximum in bank
        let &result = bank
            .iter()
            .max()
            .expect("failed to find maximum in base case");
        u64::from(result)
    }
}

impl ParsedPart2 for Day03 {
    type Part2Output = u64;

    fn part2(banks: &Self::ParsedInput) -> Self::Part2Output {
        let mut sum: Self::Part2Output = 0;
        for bank in banks {
            // turn on 12 batteries in bank
            let max = recursive_max_joltage(bank, 12);
            sum += max;
        }
        sum
    }
}

impl_runnable_solution!(Day03 => ParsedPart2);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"987654321111111
811111111111119
234234234234278
818181911112111
";

    #[test]
    fn part1_solves_example() -> ParseResult<()> {
        let parsed = Day03::parse(EXAMPLE_INPUT)?;
        let result = Day03::part1(&parsed);
        assert_eq!(result, 357);
        Ok(())
    }

    #[test]
    fn part2_solves_example() -> ParseResult<()> {
        let parsed = Day03::parse(EXAMPLE_INPUT)?;
        let result = Day03::part2(&parsed);
        assert_eq!(result, 3_121_910_778_619);
        Ok(())
    }
}
