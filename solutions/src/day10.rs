use std::collections::{HashMap, HashSet};

use aoc_framework::{
    ParseError, ParseResult, ParsedPart1, ParsedPart2, SolutionName,
    impl_runnable_solution,
};

use crate::util::parse::parse_lines;

/// Solution for tenth day's puzzle.
///
/// # Input
///
/// Input is lines describing machines.
///
/// A line starts with an indicator light diagram: wrapped in square brackets,
/// holding a sequence of "off"/`.` and "on"/`#`. All machines start with
/// lights off, so this is the goal of lights to turn on.
///
/// With a space separating the light diagram, the line continues with
/// space-separated button wiring schematics. A schematic is wrapped in
/// parentheses, holding a comma separated list of light indexes. A button will
/// toggle the lights listed by index.
///
/// With a space separating the button schematics, the line ends with joltage
/// requirements: wrapped in curly braces, holding a comma separated sequence
/// of numbers. Ignored for part 1.
///
/// # Part 1
///
/// Determine the fewest presses of buttons to get the given light diagrams.
/// Sum the minimum button presses for each machine.
///
/// # Part 2
///
/// Machines are now mode-switched to have buttons increase joltage levels.
///
/// A machine has numeric counters for the joltage levels, one counter per
/// joltage requirement, and initialized to zero.
///
/// Buttons now add 1 jolt to a counter with each press, with the input
/// defining indexes to counters that a button affects.
///
/// Determine fewest button presses to get the joltage requirements. Sum the
/// minimum presses for each machine.
pub struct Day10;

impl SolutionName for Day10 {
    const NAME: &'static str = "Day 10: Factory";
}

/// A button on a machine, represented as a set of indexes that the button
/// modifies.
type Button = HashSet<usize>;

/// Indicator lights, represented as an indexable sequence of
/// boolean values.
type IndicatorLights = Vec<bool>;

/// A type for joltage numbers.
type Joltage = u16;
/// Joltage counters, represented by an indexable sequence of joltage numbers.
type JoltageCounters = Vec<Joltage>;

/// Types of braces used in input.
enum BraceType {
    Parentheses,
    SquareBrackets,
    CurlyBraces,
}

/// Strip braces from start & end of string, panic if the braces aren't
/// available to strip.
fn strip_braces_panic(s: &str, braces: &BraceType) -> String {
    match braces {
        BraceType::Parentheses => {
            if s.starts_with('(') && s.ends_with(')') {
                s[1..s.len() - 1].to_string()
            } else {
                panic!("string not wrapped with parentheses: {s:?}");
            }
        }
        BraceType::SquareBrackets => {
            if s.starts_with('[') && s.ends_with(']') {
                s[1..s.len() - 1].to_string()
            } else {
                panic!("string not wrapped with square brackets: {s:?}");
            }
        }
        BraceType::CurlyBraces => {
            if s.starts_with('{') && s.ends_with('}') {
                s[1..s.len() - 1].to_string()
            } else {
                panic!("string not wrapped with curly braces: {s:?}");
            }
        }
    }
}

/// A representation of a machine with indicator lights, joltage counters, and
/// buttons.
pub struct Machine {
    /// A collection of buttons on the machine.
    buttons: Vec<Button>,
    /// The expected length of a sequence that buttons can modify, either for
    /// indicator lights or joltage counters.
    length: usize,
}

impl Machine {
    /// Calculate the resulting indicator lights (starting all off) after
    /// pressing the given buttons by index once each.
    fn calculate_lights_from_buttons(
        &self,
        button_indexes: &HashSet<usize>,
    ) -> IndicatorLights {
        // init a vector to hold light states
        let mut lights = vec![false; self.length];
        for &button_idx in button_indexes {
            let button = &self.buttons[button_idx];
            // iterate indexes the button modifies
            for &light_idx in button {
                // toggle the current value
                lights[light_idx] = !lights[light_idx];
            }
        }
        lights
    }

    /// Calculate the resulting joltage counter after pressing the given
    /// buttons by index once each.
    fn calculate_counters_from_single_buttons(
        &self,
        button_indexes: &HashSet<usize>,
    ) -> JoltageCounters {
        // init a vector to hold counters
        let mut counters = vec![0; self.length];
        for &button_idx in button_indexes {
            let button = &self.buttons[button_idx];
            // iterate indexes the button modifies
            for &counter_idx in button {
                // increment the counter
                counters[counter_idx] += 1;
            }
        }
        counters
    }
}

/// Recursively determine if a combination of buttons by index pressed once
/// each can match the indicator lights goal.
///
/// # Arguments
///
/// - `lights_goal` - The indicator lights to match.
/// - `machine` - The machine with buttons available to press.
/// - `presses_left` - How many presses left to apply in this recursion step.
/// - `start_index` - The button index to start at and iterate after in this
///   recursion step.
/// - `current_buttons` - The current combination of buttons by index to work
///   from in this recursion step.
fn recursive_lights_goal_solvable(
    lights_goal: &IndicatorLights,
    machine: &Machine,
    presses_left: usize,
    start_index: usize,
    current_buttons: &mut HashSet<usize>,
) -> bool {
    // base case
    if presses_left == 0 {
        // either the combo satisfies the goal or not
        let lights = machine.calculate_lights_from_buttons(current_buttons);
        return lights == *lights_goal;
    }

    // iterate remaining buttons to press
    for index in start_index..machine.buttons.len() {
        current_buttons.insert(index);

        // recurse with one less press left & start after current index
        if recursive_lights_goal_solvable(
            lights_goal,
            machine,
            presses_left - 1,
            index + 1,
            current_buttons,
        ) {
            return true;
        }

        // backtrack for the next loop
        current_buttons.remove(&index);
    }

    // no successful combination found
    false
}

/// Determine the minimum button presses to match the indicator light diagram.
///
/// # Returns
///
/// An Option that either is `Some(presses)` for the number of presses
/// determined, or `None` for no solution found.
fn minimum_button_presses_for_light_diagram(
    light_diagram: &IndicatorLights,
    machine: &Machine,
) -> Option<usize> {
    /*
    Thanks Gemini for pointing out I don't need permutations of increasing
    presses to distribute as permutations:
    any button only needs to be pressed once or never

    I already intuited an even number of presses would be a net zero, but
    didn't catch on that odd number presses greater than one would be net zero
    to one press
    */

    for presses in 1..=machine.buttons.len() {
        let mut current_buttons = HashSet::new();
        if recursive_lights_goal_solvable(
            light_diagram,
            machine,
            presses,
            0,
            &mut current_buttons,
        ) {
            return Some(presses);
        }
    }

    // failed to find min button presses to produce goal
    None
}

impl ParsedPart1 for Day10 {
    /// A sequence of tuples of a machine with buttons, an indicator light
    /// diagram, and joltage requirements.
    type ParsedInput = Vec<(Machine, IndicatorLights, JoltageCounters)>;

    fn parse(input: &str) -> aoc_framework::ParseResult<Self::ParsedInput> {
        let parsed: Self::ParsedInput = parse_lines(input, |line| {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            assert!(
                tokens.len() >= 3,
                "expected at least 3 tokens across line: {tokens:?}"
            );

            let light_diagram: IndicatorLights = strip_braces_panic(
                tokens.first().expect("failed to get first token"),
                &BraceType::SquareBrackets,
            )
            .chars()
            .map(|c| c == '#')
            .collect();

            // track maximum indexes expected across lights, joltage counters,
            // and buttons
            let indexes_length = light_diagram.len();

            let buttons = tokens[1..tokens.len() - 1]
                .iter()
                .map(|button_wiring| {
                    strip_braces_panic(button_wiring, &BraceType::Parentheses)
                        .split(',')
                        .map(|index| {
                            // TODO would assert value not bigger than
                            // indexes_length
                            index.parse().map_err(|source| {
                                ParseError::parse_int_from_str(index, source)
                            })
                        })
                        .collect::<ParseResult<_>>()
                })
                .collect::<ParseResult<_>>()?;

            let joltage_requirements: JoltageCounters = strip_braces_panic(
                tokens.last().expect("failed to get last token"),
                &BraceType::CurlyBraces,
            )
            .split(',')
            .map(|number| {
                number.parse().map_err(|source| {
                    ParseError::parse_int_from_str(number, source)
                })
            })
            .collect::<ParseResult<_>>()?;

            assert_eq!(
                joltage_requirements.len(),
                indexes_length,
                "joltage requirements length does not match indicator lights length"
            );

            Ok((Machine {
                buttons,
                length: indexes_length,
            }, light_diagram, joltage_requirements))
        })
        .collect::<ParseResult<_>>()?;

        if parsed.is_empty() {
            Err(ParseError::EmptyInput)
        } else {
            Ok(parsed)
        }
    }

    type Part1Output = u32;

    fn part1(parsed: &Self::ParsedInput) -> Self::Part1Output {
        parsed
            .iter()
            .map(|(machine, light_diagram, _)| {
                minimum_button_presses_for_light_diagram(light_diagram, machine)
                    .expect(
                        "failed to find minimum button presses for a machine",
                    )
            })
            .try_fold(0u32, |acc, v| {
                acc.checked_add(v.try_into().expect(
                    "failed to cast a machine's minimum button presses for summing",
                ))
            })
            .expect("overflow occurred when summing")
    }
}

/// Recursively determine multiple combinations of buttons by index pressed
/// once each that can match the indicator lights goal.
///
/// # Arguments
///
/// - `lights_goal` - The indicator lights to match.
/// - `machine` - The machine with buttons available to press.
/// - `current_index` - The current button index to use in this recursion step.
/// - `current_buttons` - The current combination of buttons by index to work
///   from in this recursion step.
///
/// # Returns
///
/// A vector of combinations of buttons by index found to solve for the
/// indicator lights.
fn recursive_lights_goal_button_combinations(
    lights_goal: &IndicatorLights,
    machine: &Machine,
    current_index: usize,
    current_buttons: &mut HashSet<usize>,
) -> Vec<HashSet<usize>> {
    // base case
    if current_index >= machine.buttons.len() {
        // we have a combo to test
        let lights = machine.calculate_lights_from_buttons(current_buttons);
        if lights == *lights_goal {
            return vec![current_buttons.clone()];
        }
        return vec![];
    }

    // recursion should modify current_buttons with current index, and recurse
    // to the next index

    // get combinations without current index
    let mut without_index = recursive_lights_goal_button_combinations(
        lights_goal,
        machine,
        current_index + 1,
        current_buttons,
    );

    // get combinations with current index
    current_buttons.insert(current_index);
    let with_index = recursive_lights_goal_button_combinations(
        lights_goal,
        machine,
        current_index + 1,
        current_buttons,
    );
    // backtrack the insertion
    current_buttons.remove(&current_index);

    // combine results; have one result extend the other & be returned
    without_index.extend(with_index);
    without_index
}

/// Recursively determine minimum button presses to match a joltage counter.
///
/// # Arguments
///
/// - `counters_goal` - The joltage counters to match.
/// - `machine` - The machine with buttons available to press.
/// - `memo` - A memoization cache for dynamic programming.
///
/// # Returns
///
/// An option that either holds `Some(presses)` for the number of presses, or
/// `None` for no working combination found.
fn recursive_minimum_button_presses_for_counters(
    counters_goal: &JoltageCounters,
    machine: &Machine,
    memo: &mut HashMap<JoltageCounters, Option<usize>>,
) -> Option<usize> {
    /*
    inspired from: https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/
    - whatever solution there is for the joltage counters, it can produce a
      light display; odd joltage -> on, otherwise off
    - solving for this light display gives possible button combos used in
      solution
      - just buttons pressed an odd number of times; there can be extra even
        presses on any more buttons, including the buttons found
    - subtract the single button presses result from the counters; this should
      cause counters to be even numbers
    - halve the counters, setting up a recursive case to solve
      - calculating `2*f(half_counters) + count_buttons`
      - recursion excellent candidate for dynamic programming
    - result is minimum of recursive calculations
    */

    // memoization
    if let Some(&result) = memo.get(counters_goal) {
        return result;
    }

    // base case
    if counters_goal.iter().all(|c| *c == 0) {
        return Some(0);
    }

    // map to light display
    let lights: Vec<bool> = counters_goal.iter().map(|c| c % 2 != 0).collect();
    // calculate button combos that would solve lights
    let mut current_buttons = HashSet::new();
    // TODO worth caching combos from lights? would want another memo
    let button_combos = recursive_lights_goal_button_combinations(
        &lights,
        machine,
        0,
        &mut current_buttons,
    );

    // calculate minimums from button combos and return the smallest found, or
    // None for no solution
    let result = button_combos
        .iter()
        .filter_map(|buttons| {
            // subtract the values the buttons would contribute as single
            // presses from the counters goal
            // - if subtraction overflows, filter out as unsolvable
            let subtraction =
                machine.calculate_counters_from_single_buttons(buttons);
            let new_counters_opt: Option<JoltageCounters> = subtraction
                .into_iter()
                .zip(counters_goal)
                .map(|(sub, counter)| counter.checked_sub(sub))
                .collect();
            new_counters_opt.and_then(|new_counters| {
                assert!(
                    new_counters.iter().all(|c| c % 2 == 0),
                    "not all values in new counter are even: {new_counters:?}"
                );
                // halve the counter's even numbers and recursively solve its
                // minimum; filter out if recursion fails
                let half_counters: JoltageCounters = new_counters
                    .into_iter()
                    .map(|counter| counter / 2)
                    .collect();
                recursive_minimum_button_presses_for_counters(
                    &half_counters,
                    machine,
                    memo,
                )
            }).map(|recursive_min| {
                // the button presses will be the recursive min found times two,
                // plus the number of buttons pressed this recursive step
                2 * recursive_min + buttons.len()
            })
        })
        .min();

    // cache result then return
    memo.insert(counters_goal.clone(), result);
    result
}

/// Determine the minimum button presses to match the joltage requirements.
///
/// # Returns
///
/// An Option that either is `Some(presses)` for the number of presses
/// determined, or `None` for no solution found.
fn minimum_button_presses_for_joltage_requirements(
    joltage_requirements: &JoltageCounters,
    machine: &Machine,
) -> Option<usize> {
    let mut memo = HashMap::new();
    recursive_minimum_button_presses_for_counters(
        joltage_requirements,
        machine,
        &mut memo,
    )
}

impl ParsedPart2 for Day10 {
    type Part2Output = u32;

    fn part2(parsed: &Self::ParsedInput) -> Self::Part2Output {
        parsed
            .iter()
            .map(|(machine, _, joltage_requirements)| {
                minimum_button_presses_for_joltage_requirements(
                    joltage_requirements,
                    machine,
                ).expect("failed to find minimum button presses for a machine")
            })
            .try_fold(0u32, |acc, v| {
                acc.checked_add(v.try_into().expect(
                    "failed to cast a machine's minimum button presses for summing"
                ))
            })
            .expect("overflow occurred when summing")
    }
}

impl_runnable_solution!(Day10 => ParsedPart2);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

    #[test]
    fn part1_solves_example() -> ParseResult<()> {
        let parsed = Day10::parse(EXAMPLE_INPUT)?;
        let result = Day10::part1(&parsed);
        assert_eq!(result, 7);
        Ok(())
    }

    #[test]
    fn part2_solves_example() -> ParseResult<()> {
        let parsed = Day10::parse(EXAMPLE_INPUT)?;
        let result = Day10::part2(&parsed);
        assert_eq!(result, 33);
        Ok(())
    }
}
