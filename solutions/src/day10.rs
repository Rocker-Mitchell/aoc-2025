use std::collections::HashSet;

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

/// A type for joltage numbers.
type Joltage = u16;

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

/// A representation of a machine with light indicators & buttons.
pub struct LightMachine {
    /// The goal configuration of light indicators.
    light_goal: Vec<bool>,
    /// A collection of buttons.
    ///
    /// A button can consist of a set of indexes to light indicators. Pressing
    /// the button toggles the lights by index.
    buttons: Vec<HashSet<usize>>,
    /// Joltage requirements for the machine.
    #[expect(dead_code, reason = "still working on solution")]
    joltage_requirements: Vec<Joltage>,
}

impl LightMachine {
    /// Calculate the resulting light configuration (starting all off) after
    /// pressing the given buttons by index once.
    fn calculate_resulting_light(
        &self,
        button_idxs_pressed: &HashSet<usize>,
    ) -> Vec<bool> {
        let mut lights = vec![false; self.light_goal.len()];
        for &button_idx in button_idxs_pressed {
            let button = &self.buttons[button_idx];
            for &light_idx in button {
                lights[light_idx] = !lights[light_idx];
            }
        }
        lights
    }

    /// Check pressing the given buttons by index once will match the light
    /// indicator goal.
    fn check_light_solution(
        &self,
        button_idxs_pressed: &HashSet<usize>,
    ) -> bool {
        let result = self.calculate_resulting_light(button_idxs_pressed);
        result == self.light_goal
    }

    /// Recursively determine a combination of buttons by index that will match
    /// the light indicator goal when pressed once.
    ///
    /// # Args
    /// - `presses_left` - how many presses left to apply in this recursion
    ///   step.
    /// - `start_idx` - the button index to start at and iterate after when
    ///   recursing for next step.
    /// - `current_combo` - the current combination of button indexes being
    ///   pressed once.
    ///
    /// # Returns
    ///
    /// An option that either holds `Some(combo)` for a found working combo, or
    /// `None` for no working combination found.
    fn find_button_combinations_for_light(
        &self,
        presses_left: usize,
        start_idx: usize,
        current_combo: &mut HashSet<usize>,
    ) -> Option<HashSet<usize>> {
        // base case
        if presses_left == 0 {
            // either the combo results in the goal or not
            if self.check_light_solution(current_combo) {
                return Some(current_combo.clone());
            }
            return None;
        }

        // iterate remaining buttons to press
        for idx in start_idx..self.buttons.len() {
            current_combo.insert(idx);

            // recurse with one less press left & start index after current index
            if let Some(result) = self.find_button_combinations_for_light(
                presses_left - 1,
                idx + 1,
                current_combo,
            ) {
                return Some(result);
            }

            // backtrack for next loop
            current_combo.remove(&idx);
        }

        // no successful combination found
        None
    }

    // Determine the minimum button presses to get the light indicator goal.
    fn find_minimum_button_presses_for_light_goal(&self) -> Option<usize> {
        /*
        Thanks Gemini for pointing out I don't need permutations of increasing
        presses to distribute as permutations:
        any button only needs to be pressed once or never

        I already intuited an even number of presses would be a net zero, but
        didn't catch on that odd number presses greater than one would be net
        zero to one press
        */

        for presses in 1..=self.buttons.len() {
            let mut current_combo = HashSet::new();
            if self
                .find_button_combinations_for_light(
                    presses,
                    0,
                    &mut current_combo,
                )
                .is_some()
            {
                return Some(presses);
            }
        }

        // failed to find min button presses to produce goal
        None
    }

    fn find_minimum_button_presses_for_joltage_requirements(&self) -> u64 {
        /*
        I'm stuck on getting anything to work, be performant, or be
        implementable
        - couldn't figure out what decomposition to use with nalgebra
        - Copilot guided me to BFS, then A* but both were very slow for even
          one machine from input
        - Copilot then wanted me to use good_lp, but both it and Google AI
          kept feeding me un-compilable code until I eventually coersed it to
          something valid, to then be met with it failing to link to a
          `link.exe`
        I can't solve this right now
        */
        todo!()

        /*
        use good_lp::{
            Expression, Solution, SolverModel, default_solver, variable,
            variables,
        };

        if self.joltage_requirements.is_empty() {
            // technically no presses needed for no requirements
            return 0;
        }

        let target = &self.joltage_requirements;
        let n_buttons = self.buttons.len();

        // create int vars: x[i] as presses of button i
        let mut vars = variables!();
        let x_vars: Vec<_> = (0..n_buttons)
            .map(|_| vars.add(variable().integer().min(0)))
            .collect();

        // objective: minimize sum of x[i]
        // use default solver
        let objective: Expression = x_vars.iter().sum();
        let mut model = vars.minimise(objective).using(default_solver);

        // build constraints: for each counter i, sum of
        // (button j affects i) * x[j] == target[i]
        for (i, &target_val) in target.iter().enumerate() {
            let mut expr: Expression = 0.into();
            for (j, button) in self.buttons.iter().enumerate() {
                if button.contains(&i) {
                    expr += x_vars[j];
                }
            }
            model = model.with(expr.eq(target_val));
        }

        // solve
        model.solve().map_or_else(
            |_| panic!("ILP solver failed to find solution"),
            |solution| {
                x_vars
                    .iter()
                    .map(|&var| {
                        #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "variables have lower bound 0 and shouldn't be aggressively big")]
                        let value = solution.value(var).round() as u64;
                        value
                    })
                    .sum()
            },
        )
        */

        /*
        // prepare button increments: index first by button, then by counter
        let button_vectors: Vec<Vec<Joltage>> = self
            .buttons
            .iter()
            .map(|btn| {
                (0..n_counters)
                    .map(|i| Joltage::from(btn.contains(&i)))
                    .collect()
            })
            .collect();

        // checking there isn't a counter that can't be affected by a button
        for (i, &t) in target.iter().enumerate() {
            assert!(
                t == 0 || button_vectors.iter().any(|bv| bv[i] != 0),
                "no button affects counter {i}, impossible to reach requirement"
            );
        }

        /*
        // precompute max number of counters any single button increments
        let max_button_width = button_vectors
            .iter()
            .map(|bv| bv.iter().map(|&v| u32::from(v)).sum::<u32>())
            .max()
            .unwrap_or(0);
        assert!(max_button_width > 0, "no usable buttons found");
        */

        // precompute: for each counter, the min presses to increment by 1
        let min_presses_per_counter: Vec<u64> = (0..n_counters)
            .map(|i| {
                button_vectors
                    .iter()
                    .filter(|bv| bv[i] != 0)
                    .map(|_| 1u64)
                    .min()
                    .unwrap_or(u64::MAX)
            })
            .collect();

        // heuristic for a state: ceil(sum_remaining / max_button_width)
        let heuristic = |state: &Vec<Joltage>| -> u64 {
            target
                .iter()
                .zip(state.iter())
                .zip(min_presses_per_counter.iter())
                .map(|((&t, &s), &min_press)| {
                    u64::from(t-s) * min_press
                })
                .sum()
            /*
            let sum_remaining: u32 = target
                .iter()
                .zip(state.iter())
                .map(|(t, s)| u32::from(t - s))
                .sum();
            if sum_remaining == 0 {
                0
            } else {
                u64::from(sum_remaining.div_ceil(max_button_width))
            }
            */
        };

        // start search with all 0's
        let start = vec![0u16; n_counters];
        if start == target {
            return 0;
        }

        //let mut queue = VecDeque::new();
        //let mut seen = HashSet::new();
        //queue.push_back((start.clone(), 0));
        //seen.insert(start);

        // A* priority queue: Reverse((priority, g, state)) so smallest
        // priority first
        let mut heap = BinaryHeap::new();
        let mut best_g = HashMap::new();

        let start_h = heuristic(&start);
        heap.push(Reverse((start_h, 0u64, start.clone())));
        best_g.insert(start, 0);

        while let Some(Reverse((_, g, state))) = heap.pop() {
            // skip any worse g than best known
            if let Some(&best) = best_g.get(&state) && g > best {
                continue;
            }

            // expand neighbors by pressing each button once
            for bv in &button_vectors {
                // calc next state and prune if any component would exceed
                // target
                let mut next = state.clone();
                let mut ok = true;
                for i in 0..n_counters {
                    let sum = next[i].saturating_add(bv[i]);
                    if sum > target[i] {
                        ok = false;
                        break;
                    }
                    next[i] = sum;
                }
                if !ok {
                    continue;
                }

                let next_g = g+1;
                if let Some(&existing_g) = best_g.get(&next) && next_g >= existing_g {
                    continue;
                }

                // check if target reached
                if next == target {
                    return next_g;
                }

                best_g.insert(next.clone(), next_g);
                let h = heuristic(&next);
                let priority = next_g + h;
                heap.push(Reverse((priority, next_g, next)));
            }
        }

        panic!("failed to find a solution for joltage requirements");

        /*
        // format buttons & requirements to matrices to solve as linear system:
        // Ax = b

        // a button will inform on a column of matrix A
        // - build column major slice
        let mut a_column_major = Vec::new();
        for button in self.buttons.iter().by_ref() {
            // want columns of 1's & 0's; if button affects counter (which will
            // map to row) then track 1
            for counter_idx in 0..self.joltage_requirements.len() {
                let factor = f64::from(button.contains(&counter_idx));
                a_column_major.push(factor);
            }
        }
        let a_matrix: DMatrix<f64> = DMatrix::from_column_slice(
            self.joltage_requirements.len(),
            self.buttons.len(),
            &a_column_major,
        );

        // matrix b will be a column of requirements
        let b_floats: Vec<_> = self
            .joltage_requirements
            .iter()
            .map(|&j| f64::from(j))
            .collect();
        let b_vector: DVector<f64> = DVector::from_column_slice(&b_floats);

        // BUG matrix A can be not-square, example has a case wider than tall
        let svd = a_matrix.svd();
        let x_vector = svd.solve(&b_vector).expect("failed to solve system");
        let eps = 1e-9f64;
        x_vector
            .iter()
            .map(|&x| {
                assert!(!x.is_nan(), "solution contains NaN");
                let rounded = x.round();
                assert!(
                    (x - rounded).abs() <= eps,
                    "solution value not whole number: {x}"
                );
                assert!(rounded >= 0.0, "solution value is negative: {x}");
                assert!(
                    rounded <= (u64::MAX as f64),
                    "solution value overflows u64: {x}"
                );
                rounded as u64
            })
            .try_fold(0u64, u64::checked_add)
            .expect("overflow occurred when summing solution vector")
        */
        */
    }
}

impl ParsedPart1 for Day10 {
    type ParsedInput = Vec<LightMachine>;

    fn parse(input: &str) -> aoc_framework::ParseResult<Self::ParsedInput> {
        let machines: Self::ParsedInput = parse_lines(input, |line| {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            assert!(
                tokens.len() >= 3,
                "expected at least 3 tokens across line: {tokens:?}"
            );

            let light_goal = strip_braces_panic(
                tokens.first().expect("failed to get first token"),
                &BraceType::SquareBrackets,
            )
            .chars()
            .map(|c| c == '#')
            .collect();

            let buttons = tokens[1..tokens.len() - 1]
                .iter()
                .map(|button_wiring| {
                    strip_braces_panic(button_wiring, &BraceType::Parentheses)
                        .split(',')
                        .map(|index| {
                            index.parse().map_err(|source| {
                                ParseError::parse_int_from_str(index, source)
                            })
                        })
                        .collect::<ParseResult<_>>()
                })
                .collect::<ParseResult<_>>()?;

            let joltage_requirements = strip_braces_panic(
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

            Ok(LightMachine {
                light_goal,
                buttons,
                joltage_requirements,
            })
        })
        .collect::<ParseResult<_>>()?;

        if machines.is_empty() {
            Err(ParseError::EmptyInput)
        } else {
            Ok(machines)
        }
    }

    type Part1Output = u32;

    fn part1(machines: &Self::ParsedInput) -> Self::Part1Output {
        machines
            .iter()
            .map(|machine| {
                machine.find_minimum_button_presses_for_light_goal().expect(
                    "failed to find minimum button presses for a machine",
                )
            })
            .try_fold(0u32, |acc, v| {
                acc.checked_add(v.try_into().expect(
                    "failed to cast a minimum button press for summing",
                ))
            })
            .expect("overflow occurred when summing")
    }
}

impl ParsedPart2 for Day10 {
    type Part2Output = u64;

    #[expect(clippy::print_stdout, reason = "debugging")]
    fn part2(machines: &Self::ParsedInput) -> Self::Part2Output {
        machines
            .iter()
            .map(|machine| {
                println!("going to press buttons...");
                let result = machine
                    .find_minimum_button_presses_for_joltage_requirements();
                println!("pressed buttons {result} times");
                result
            })
            .try_fold(0u64, u64::checked_add)
            .expect("overflow occurred when summing")
    }
}

// TODO still working on part 2
impl_runnable_solution!(Day10 => ParsedPart1);

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

    #[ignore = "still working on solution"]
    #[test]
    fn part2_solves_example() -> ParseResult<()> {
        let parsed = Day10::parse(EXAMPLE_INPUT)?;
        let result = Day10::part2(&parsed);
        assert_eq!(result, 33);
        Ok(())
    }
}
