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
        let mut pressure = 0;
        let nonzero_ix: HashMap<usize, usize> = self
            .rates
            .iter()
            .enumerate()
            .filter_map(|(i, (&valve, &rate))| (rate > 0).then_some((valve, i)))
            .collect();
        if nonzero_ix.len() > usize::BITS as usize {
            panic!("too many nonzero valves for bitmask");
        }
        let mut stack: Vec<(usize, usize, usize, usize)> = vec![(t_0, 0, start, 0)];
        while let Some((t, released, valve, visited_mask)) = stack.pop() {
            pressure = pressure.max(released);
            stack.extend(
                self.distances[&valve]
                    .iter()
                    .filter(|(valve, &dist)| {
                        dist < t - 2
                            && nonzero_ix
                                .get(valve)
                                .map_or(false, |i| visited_mask & 1 << i == 0)
                    })
                    .map(|(valve, &dist)| {
                        (
                            t - dist - 1,
                            released + self.rates[valve] * (t - dist - 1),
                            *valve,
                            visited_mask | 1 << nonzero_ix[valve],
                        )
                    }),
            );
        }
        pressure
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
    valves.release(valve_name_to_key(&"AA"), 30)
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
