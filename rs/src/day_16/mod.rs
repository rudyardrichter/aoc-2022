use std::{collections::HashMap, num::ParseIntError};

use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case},
    character::complete::{alpha1, digit1, line_ending, space0, space1},
    combinator::{map, map_res, opt},
    multi::{many_till, separated_list0, separated_list1},
    sequence::{delimited, tuple},
    IResult,
};
use petgraph::{algo::floyd_warshall, prelude::DiGraphMap};

pub struct Valves {
    distances: HashMap<usize, HashMap<usize, usize>>,
    rates: HashMap<usize, usize>,
}

impl Valves {
    fn new(graph: DiGraphMap<usize, usize>, rates: HashMap<usize, usize>) -> Self {
        let fw = floyd_warshall(&graph, |(_, _, weight)| *weight);
        let mut distances: HashMap<usize, HashMap<usize, usize>> = HashMap::new();
        for ((src, dst), dist) in fw.unwrap().iter() {
            distances.entry(*src).or_default().insert(*dst, *dist);
        }
        Self { distances, rates }
    }
    fn release(&self, start: usize, t_0: usize) -> usize {
        // Set up DP matrix with dimension: t_0 x nodes x [bitmask],
        // where bitmask is a vector with length equal to n nodes.
        // So the third dimension actually is 2^nodes. Howveer, we can limit this only to nodes
        // which have a nonzero flow rate.
        // Define function flow which returns rate based on bitmask.
        // Base case:
        //     released[t_0][start][*] = 0
        // Recurrence relations from released[t][node][bitmask]:
        //     if node is not "open":
        //         bitmask_new = bitmask with node set to true/"opened"
        //         released[t - 1][node][bitmask_new]
        //             = released[t][node][bitmask] + (t - 1) * flow(bitmask_new)
        //     else:
        //         for dst in neighbors accessible from node:
        //             released[t - 1][dst][bitmask] = itself.min(released[t][node][bitmask])
        // At each time step, we need only check the reachable nodes, e.g. if released[t] contains
        // nonzero values only for node A then released[t - 1] should only have nonzero
        // values stored in released[t - 1][A] and released[t - 1][B] where B is any node rechable
        // from A.
        // Solution is the maximum entry in released[0].
        //
        // TODO: do it the not stupid way
        let nonzero: Vec<usize> = self
            .rates
            .iter()
            .filter_map(|(&valve, &rate)| (rate > 0).then_some(valve))
            .collect();
        let n_nonzero = nonzero.len();
        let mut released: Vec<Vec<Vec<usize>>> =
            vec![vec![vec![0; 1 << n_nonzero]; n_nonzero]; t_0 + 1];
        for t in (0..t_0).rev() {
            for (i, valve) in nonzero.iter().enumerate() {
                // We have to restrict each iteration to nodes reachable from the start within the
                // elapsed time.
                if let Some(dist) = self.distances[&start].get(&valve) {
                    if t + dist > t_0 {
                        continue;
                    }
                }
                let rate = self.rates[&valve];
                for bitmask in 0..(1 << n_nonzero) {
                    if bitmask & (1 << i) == 0 && t > 0 {
                        // Valve is closed
                        let bitmask_new = bitmask | (1 << i);
                        released[t][i][bitmask_new] = released[t + 1][i][bitmask] + (t - 1) * rate;
                    } else {
                        // Iterate over all nonzero, closed nodes reachable from this valve, and
                        // skip to time according to their distance from the current valve.
                        let distances = &self.distances[&valve];
                        for j in (0..n_nonzero).filter(|j| bitmask & (1 << j) == 0) {
                            let dst = nonzero[j];
                            if let Some(&dist) = distances.get(&dst) {
                                if t + dist <= t_0 {
                                    released[t][j][bitmask] =
                                        released[t][j][bitmask].max(released[t + dist][i][bitmask]);
                                }
                            }
                        }
                    }
                }
            }
        }
        *released[0]
            .iter()
            .map(|r| r.iter().max().unwrap())
            .max()
            .unwrap()
    }
}

fn valve_name_to_key(s: &&str) -> usize {
    s.bytes().fold(0, |acc, b| (acc << 8) + (b - b'A') as usize)
}

fn parse_valves(s: &str) -> IResult<&str, Valves> {
    let mut graph: DiGraphMap<usize, usize> = DiGraphMap::new();
    let mut rates: HashMap<usize, usize> = HashMap::new();
    let (rest, _) = map(separated_list1(line_ending, parse_valve), |lines| {
        lines.iter().for_each(|(valve, rate, tunnels)| {
            graph.add_node(*valve);
            rates.insert(*valve, *rate);
            for &dst in tunnels.iter() {
                graph.add_node(dst);
                graph.add_edge(*valve, dst, 1);
            }
        });
        ()
    })(s)?;
    Ok((rest, Valves::new(graph, rates)))
}

fn parse_valve(s: &str) -> IResult<&str, (usize, usize, Vec<usize>)> {
    map_res(
        tuple((
            tag_no_case("valve"),
            delimited(space0, alpha1, space0),
            tag_no_case("has flow rate="),
            digit1,
            tuple((
                tag(";"),
                many_till(alt((alpha1, space1)), tuple((tag("valve"), opt(tag("s"))))),
                space0,
            )),
            separated_list0(delimited(space0, tag(","), space0), alpha1),
        )),
        |(_, valve, _, rate_str, _, tunnels): (_, &str, _, &str, _, Vec<&str>)| {
            Ok::<_, ParseIntError>((
                valve_name_to_key(&valve),
                rate_str.parse::<usize>()?,
                tunnels.iter().map(valve_name_to_key).collect(),
            ))
        },
    )(s)
}

#[aoc_generator(day16)]
pub fn get_input(input: &str) -> Valves {
    parse_valves(input).unwrap().1
}

#[aoc(day16, part1)]
pub fn part_1(valves: &Valves) -> usize {
    valves.release(0, 30)
}

#[aoc(day16, part2)]
pub fn part_2(valves: &Valves) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../test_data/day_16.txt");

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input(INPUT)), 1651);
    }

    #[test]
    fn test_part_2() {}
}
