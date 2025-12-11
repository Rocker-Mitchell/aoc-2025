use std::collections::{HashMap, HashSet};

use aoc_framework::{
    ParseError, ParseResult, ParsedPart1, ParsedPart2, SolutionName,
    impl_runnable_solution,
};

use crate::util::parse::parse_lines;

/// Solution for eleventh day's puzzle.
///
/// # Input
///
/// Input is a list of devices and outputs. Lines describe a device name, `: `,
/// and a space-separated list of device names that are attached as outputs.
///
/// # Part 1
///
/// Two device names are relevant: "you" as a start point and "out" as an end
/// point.
///
/// Data can follow paths through devices by following outputs. Find every path
/// available from "you" to "out". Return the number of paths found.
///
/// # Part 2
///
/// Some more device names are relevant: "svr" for a source server rack, and
/// "dac" & "fft" as critical processing devices.
///
/// Now find the number of paths from "svr" to "out", with the constraint that
/// a path must also visit "dac" & "fft" in any order.
pub struct Day11;

impl SolutionName for Day11 {
    const NAME: &'static str = "Day 11: Reactor";
}

type Connections = HashMap<String, HashMap<String, u64>>;

fn count_paths(connections: &Connections, start: &str, end: &str) -> u64 {
    fn dfs_count_paths_recursive(
        connections: &Connections,
        current: (&String, &u64),
        end_node: &str,
        visited: &mut HashSet<String>,
        count: &mut u64,
    ) {
        visited.insert(current.0.clone());

        // base case
        if current.0 == end_node {
            *count += current.1;
            // backtrack the node visit
            visited.remove(current.0);
            return;
        }

        // explore outputs of current node
        if let Some(outputs) = connections.get(current.0) {
            for next in outputs {
                if !visited.contains(next.0.as_str()) {
                    // recurse with next as current node
                    dfs_count_paths_recursive(
                        connections,
                        next,
                        end_node,
                        visited,
                        count,
                    );
                }
            }
        } else {
            panic!("failed to get outputs of current node: {current:?}");
        }

        // backtrack node visited
        visited.remove(current.0);
    }

    let mut count = 0;
    let mut visited = HashSet::new();

    dfs_count_paths_recursive(
        connections,
        (&start.to_string(), &1),
        end,
        &mut visited,
        &mut count,
    );
    count
}

impl ParsedPart1 for Day11 {
    type ParsedInput = Connections;

    fn parse(input: &str) -> ParseResult<Self::ParsedInput> {
        let mut connections: Self::ParsedInput = parse_lines(input, |line| {
            let (name, raw_outputs) = line
                .split_once(':')
                .ok_or_else(|| ParseError::NoDelimiter(':'.into()))?;
            assert!(!name.is_empty(), "no device name found before \":\"");
            let outputs: HashMap<String, u64> = raw_outputs
                .split_whitespace()
                .map(|s| (s.to_string(), 1))
                .collect();
            assert!(
                !outputs.is_empty(),
                "no output connections found after \":\""
            );
            Ok((name.to_string(), outputs))
        })
        .collect::<ParseResult<_>>()?;

        // friend pitched squashing nodes, as there's a good number of
        // connections to exactly one node
        // - specifically squashing as a map of an output node and the number
        //   of paths to it without traveling through other output nodes
        let squash_candidates: HashSet<String> = connections
            .iter()
            .filter(|(name, _)| {
                // don't squash special names needed for parts
                !matches!(name.as_str(), "you" | "svr" | "dac" | "fft")
            })
            .map(|(name, _)| name.clone())
            .collect();
        for name in squash_candidates.iter().by_ref() {
            if let Some(name_outputs) = connections.get(name).cloned() {
                for outputs in connections.values_mut() {
                    // swap any matching name in outputs with name's outputs
                    if let Some(count) = outputs.get(name).copied() {
                        for (name_out, name_count) in &name_outputs {
                            *outputs.entry(name_out.clone()).or_insert(0) +=
                                name_count * count;
                        }
                        outputs.remove(name);
                    }
                }
            } else {
                panic!(
                    "failed to find outputs of name in squash candidates: {name:?}"
                );
            }
            connections.remove(name);
        }

        Ok(connections)
    }

    type Part1Output = u64;

    fn part1(connections: &Self::ParsedInput) -> Self::Part1Output {
        count_paths(connections, "you", "out")
    }
}

fn count_paths_with_required_visits(
    connections: &Connections,
    start: &str,
    end: &str,
    required_visits: &HashSet<String>,
) -> u64 {
    fn dfs_count_paths_recursive(
        connections: &Connections,
        current: (&String, &u64),
        end_node: &str,
        required_visits: &HashSet<String>,
        visited: &mut HashMap<String, u64>,
        count: &mut u64,
    ) {
        visited.insert(current.0.clone(), *current.1);

        // base case
        if current.0 == end_node {
            // only count if we visited what was required
            if required_visits
                .iter()
                .all(|name| visited.contains_key(name))
            {
                // factor counts of the path traveled as multiplicative
                *count += visited.values().product::<u64>();
            }
            // backtrack the node visit
            visited.remove(current.0);
            return;
        }

        // explore outputs of current node
        if let Some(outputs) = connections.get(current.0) {
            for next in outputs {
                if !visited.contains_key(next.0.as_str()) {
                    // recurse with next as current node
                    dfs_count_paths_recursive(
                        connections,
                        next,
                        end_node,
                        required_visits,
                        visited,
                        count,
                    );
                }
            }
        } else {
            panic!("failed to get outputs of current node: {current:?}");
        }

        // backtrack node visited
        visited.remove(current.0);
    }

    let mut count = 0;
    let mut visited = HashMap::new();

    dfs_count_paths_recursive(
        connections,
        (&start.to_string(), &1),
        end,
        required_visits,
        &mut visited,
        &mut count,
    );
    count
}

impl ParsedPart2 for Day11 {
    type Part2Output = u64;

    fn part2(connections: &Self::ParsedInput) -> Self::Part2Output {
        // NOTE many more possible paths to calculate starting from "svr"
        // compared to "you", so needed to optimize connections
        let required_visits =
            HashSet::from(["dac".to_string(), "fft".to_string()]);
        count_paths_with_required_visits(
            connections,
            "svr",
            "out",
            &required_visits,
        )
    }
}

impl_runnable_solution!(Day11 => ParsedPart2);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &str = r"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

    #[test]
    fn part1_solves_example() -> ParseResult<()> {
        let parsed = Day11::parse(EXAMPLE_INPUT_1)?;
        let result = Day11::part1(&parsed);
        assert_eq!(result, 5);
        Ok(())
    }

    const EXAMPLE_INPUT_2: &str = r"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";

    #[test]
    fn part2_solves_example() -> ParseResult<()> {
        let parsed = Day11::parse(EXAMPLE_INPUT_2)?;
        let result = Day11::part2(&parsed);
        assert_eq!(result, 2);
        Ok(())
    }
}
