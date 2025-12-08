use std::collections::BinaryHeap;

use aoc_framework::{
    ParseError, ParseResult, ParsedPart1, ParsedPart2, SolutionName,
    impl_runnable_solution,
};
use nalgebra::Point3;
use ordered_float::NotNan;

use crate::util::parse::parse_lines;

/// Solution for eighth day's puzzle.
///
/// # Input
///
/// Input is a collection of 3D coordinates of junction boxes; they are line
/// separated, with dimensions comma separated (X,Y,Z).
///
/// # Part 1
///
/// Connect pairs of junctions by strait-line/euclidean distance:
///
/// `distance = sqrt( (p_1 - q_1)^2 + (p_2 - q_2)^2 + (p_3 - q_3)^2 )`
///
/// Connecting junctions form a circuit, sized by how many junctions are in it.
///
/// Connect 1000 pairs of junctions by shortest distance, find the 3 largest
/// circuits, and calculate their product.
///
/// # Part 2
///
/// Continue connecting junctions to form one large circuit. With the last
/// connection that forms the circuit, multiply the X-coordinates of the paired
/// junctions as the solution.
pub struct Day08;

impl SolutionName for Day08 {
    const NAME: &'static str = "Day 8: Playground";
}

/// The number type for coordinate dimensions.
///
/// This also covers the type of calculated distances, which must be a floating
/// point number.
type Dimension = f64;

/// A structure of a distance between a point pair and the points of the pair.
#[derive(Debug, Clone)]
struct DistancePointPair {
    distance: Dimension,
    p: Point3<Dimension>,
    q: Point3<Dimension>,
}

impl DistancePointPair {
    fn new(p: Point3<Dimension>, q: Point3<Dimension>) -> Self {
        let distance = nalgebra::distance(&p, &q);
        Self { distance, p, q }
    }
}

/// A struct managing a collection of circuit groups.
#[derive(Default)]
struct Circuits {
    /// A collection of groups of circuits, defined as a collection of points
    /// that connect together.
    groups: Vec<Vec<Point3<Dimension>>>,
    // can't use HashSet of points due to floats not implementing Eq
    // NOTE this isn't including how unconnected points form 1-sized circuits;
    // seems fine though as its relevant values would have contributed to a
    // product, which a*1 = a.
}

impl Circuits {
    /// Add a connection which can extend, create, or merge circuit groups.
    fn add_connection(&mut self, p: Point3<Dimension>, q: Point3<Dimension>) {
        // need to determine if p and/or q are already in groups
        let p_idx_search = self.groups.iter().position(|g| g.contains(&p));
        let q_idx_search = self.groups.iter().position(|g| g.contains(&q));

        match (p_idx_search, q_idx_search) {
            (Some(p_idx), Some(q_idx)) => {
                if p_idx != q_idx {
                    // both in different groups, merge together
                    // - make sure the index removed is the larger one, or get
                    //   index shifting errors!
                    let (keep_idx, remove_idx) = if p_idx < q_idx {
                        (p_idx, q_idx)
                    } else {
                        (q_idx, p_idx)
                    };

                    let removed_group = self.groups.remove(remove_idx);
                    self.groups[keep_idx].extend(removed_group);
                }
                // else both in same group, no change
            }
            (Some(p_idx), None) => {
                // q not in group, add to p's group
                self.groups[p_idx].push(q);
            }
            (None, Some(q_idx)) => {
                // p not in group, add to q's group
                self.groups[q_idx].push(p);
            }
            (None, None) => {
                // neither in groups, create new group
                self.groups.push(vec![p, q]);
            }
        }
    }

    /// Get an ascending sorted vector of circuit group sizes.
    fn sorted_circuit_sizes(&self) -> Vec<usize> {
        let mut sizes: Vec<usize> = self.groups.iter().map(Vec::len).collect();
        sizes.sort_unstable();
        sizes
    }

    /// Get the count of groups of circuits.
    fn circuit_count(&self) -> usize {
        self.groups.len()
    }

    /// Get the count of points tracked across circuits.
    fn point_count(&self) -> usize {
        self.groups.iter().map(Vec::len).sum()
    }
}

/// Create an iterator of pairs of points.
fn iterate_pairs(
    junctions: &[Point3<Dimension>],
) -> impl Iterator<Item = DistancePointPair> {
    (0..junctions.len()).flat_map(move |i| {
        ((i + 1)..junctions.len())
            .map(move |j| DistancePointPair::new(junctions[i], junctions[j]))
    })
}

/// Create a group of circuits by connecting a given number of shortest
/// connections.
fn create_circuits_from_shortest_connections(
    junctions: &[Point3<Dimension>],
    connections: usize,
) -> Circuits {
    let mut heap = BinaryHeap::with_capacity(connections + 1);
    let mut pairs = Vec::with_capacity(junctions.len());

    for pair in iterate_pairs(junctions) {
        // - want max-heap behavior to pop largest out while iterating
        // - track distance with index of source pair
        heap.push((
            NotNan::new(pair.distance)
                .expect("failed to wrap float for ordering"),
            pairs.len(),
        ));
        pairs.push(pair);

        if heap.len() > connections {
            // drop largest distances to keep length to `connections`
            heap.pop();
        }
    }

    // heap iteration doesn't guarantee order, but shouldn't matter
    let indexes: Vec<usize> = heap.into_iter().map(|(_, idx)| idx).collect();
    assert_eq!(
        indexes.len(),
        connections,
        "number of indexes found under expected value"
    );
    let shortest_pairs: Vec<DistancePointPair> =
        indexes.into_iter().map(|idx| pairs[idx].clone()).collect();

    let mut circuits = Circuits::default();

    for pair in shortest_pairs {
        circuits.add_connection(pair.p, pair.q);
    }

    circuits
}

/// Get a given number of largest circuit group sizes after connecting a given
/// number of shortest connections.
fn get_largest_circuit_sizes_from_shortest_connections(
    junctions: &[Point3<Dimension>],
    connections: usize,
    count_sizes: usize,
) -> impl Iterator<Item = usize> {
    let circuits =
        create_circuits_from_shortest_connections(junctions, connections);
    let sizes = circuits.sorted_circuit_sizes();
    // got an ascending sort, so iterate backwards
    sizes.into_iter().rev().take(count_sizes)
}

impl ParsedPart1 for Day08 {
    type ParsedInput = Vec<Point3<Dimension>>;

    fn parse(input: &str) -> ParseResult<Self::ParsedInput> {
        let junctions: Self::ParsedInput = parse_lines(input, |line| {
            let mut dimensions = line.splitn(3, ',').map(|d| {
                d.parse::<Dimension>().map_err(|source| {
                    ParseError::parse_float_from_str(d, source)
                })
            });
            let x: Dimension =
                dimensions.next().expect("failed to identify x dimension")?;
            let y: Dimension =
                dimensions.next().expect("failed to identify y dimension")?;
            let z: Dimension =
                dimensions.next().expect("failed to identify z dimension")?;
            Ok(Point3::new(x, y, z))
        })
        .collect::<ParseResult<_>>()?;

        if junctions.is_empty() {
            Err(ParseError::EmptyInput)
        } else {
            Ok(junctions)
        }
    }

    type Part1Output = usize;

    fn part1(junctions: &Self::ParsedInput) -> Self::Part1Output {
        // need to calculate with 1000 pairs and 3 largest circuits
        get_largest_circuit_sizes_from_shortest_connections(junctions, 1000, 3)
            .product()
    }
}

impl ParsedPart2 for Day08 {
    type Part2Output = Dimension;

    fn part2(junctions: &Self::ParsedInput) -> Self::Part2Output {
        let mut pairs: Vec<DistancePointPair> =
            iterate_pairs(junctions).collect();
        pairs.sort_by(|a, b| {
            a.distance
                .partial_cmp(&b.distance)
                .expect("failed to compare distances")
        });

        let mut circuits = Circuits::default();

        for pair in pairs {
            circuits.add_connection(pair.p, pair.q);

            if circuits.circuit_count() == 1
                && circuits.point_count() == junctions.len()
            {
                // just connected last pair needed
                return pair.p.x * pair.q.x;
            }
        }
        panic!("failed to form single large circuit");
    }
}

impl_runnable_solution!(Day08 => ParsedPart2);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

    #[test]
    fn part1_logic_solves_example() -> ParseResult<()> {
        let parsed = Day08::parse(EXAMPLE_INPUT)?;
        let result: <Day08 as ParsedPart1>::Part1Output =
            get_largest_circuit_sizes_from_shortest_connections(&parsed, 10, 3)
                .product();
        assert_eq!(result, 40);
        Ok(())
    }

    #[test]
    fn part2_solves_example() -> ParseResult<()> {
        let parsed = Day08::parse(EXAMPLE_INPUT)?;
        let result = Day08::part2(&parsed);
        let expected = 25272.0;
        let epsilon = 1e-9;
        assert!(
            (result - expected).abs() < epsilon,
            "expected {expected}, got {result}"
        );
        Ok(())
    }
}
