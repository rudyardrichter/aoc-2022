use std::{collections::VecDeque, iter::once};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space0},
    combinator::{map, map_res},
    multi::separated_list0,
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Clone, Eq, Ord, PartialEq)]
pub enum Packet {
    Int(usize),
    List(VecDeque<Packet>),
}

impl std::fmt::Debug for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::Int(n) => write!(f, "{}", n),
            Packet::List(v) => write!(f, "{:?}", v),
        }
    }
}

impl std::fmt::Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::Int(n) => write!(f, "{}", n),
            Packet::List(l) => {
                write!(f, "[")?;
                for (i, p) in l.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, "]")
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Packet::Int(a), Packet::Int(b)) => a.partial_cmp(b),
            (Packet::Int(_), Packet::List(_)) => {
                Packet::List(vec![self.clone()].into()).partial_cmp(other)
            }
            (Packet::List(_), Packet::Int(_)) => {
                self.partial_cmp(&Packet::List(vec![other.clone()].into()))
            }
            (Packet::List(a), Packet::List(b)) => {
                for (aa, bb) in a.iter().zip(b.iter()) {
                    if aa.partial_cmp(bb) != Some(std::cmp::Ordering::Equal) {
                        return aa.partial_cmp(bb);
                    }
                }
                a.len().partial_cmp(&b.len())
            }
        }
    }
}

fn parse_all_packets(s: &str) -> IResult<&str, Vec<(Packet, Packet)>> {
    separated_list0(tag("\n\n"), parse_packet_pair)(s)
}

fn parse_packet_pair(s: &str) -> IResult<&str, (Packet, Packet)> {
    separated_pair(parse_packet, tag("\n"), parse_packet)(s)
}

fn parse_packet(s: &str) -> IResult<&str, Packet> {
    alt((
        map(
            delimited(
                tag("["),
                separated_list0(
                    delimited(space0, tag(","), space0),
                    alt((
                        map_res(digit1, |n: &str| n.parse::<usize>().map(Packet::Int)),
                        parse_packet,
                    )),
                ),
                tag("]"),
            ),
            |packets| Packet::List(packets.into()),
        ),
        map_res(digit1, |n: &str| n.parse::<usize>().map(Packet::Int)),
    ))(s)
}

#[aoc_generator(day13)]
pub fn get_input(input: &str) -> Vec<(Packet, Packet)> {
    parse_all_packets(input).unwrap().1
}

#[aoc(day13, part1)]
pub fn part_1(packets: &Vec<(Packet, Packet)>) -> usize {
    packets
        .iter()
        .enumerate()
        .filter_map(|(i, (a, b))| {
            // println!("{}: {:?} < {:?} ==> {}", i + 1, a, b, a < b);
            (a < b).then_some(i + 1)
        })
        .sum()
}

#[aoc(day13, part2)]
pub fn part_2(packets: &Vec<(Packet, Packet)>) -> usize {
    let divider = |n: usize| Packet::List(vec![Packet::List(vec![Packet::Int(n)].into())].into());
    let mut packets: Vec<Packet> = packets
        .iter()
        .cloned()
        .chain(once((divider(2), divider(6))))
        .flat_map(|(p1, p2)| once(p1).chain(once(p2)))
        .collect();
    packets.sort();
    let d1 = packets.iter().position(|p| p == &divider(2)).unwrap() + 1;
    let d2 = packets.iter().position(|p| p == &divider(6)).unwrap() + 1;
    d1 * d2
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../test_data/day_13.txt");

    #[test]
    fn test_parse_packet() {
        assert_eq!(parse_packet("1"), Ok(("", Packet::Int(1))));
        assert_eq!(parse_packet("[]"), Ok(("", Packet::List(VecDeque::new()))));
        assert_eq!(
            parse_packet("[1, 2, 3]"),
            Ok((
                "",
                Packet::List(vec![Packet::Int(1), Packet::Int(2), Packet::Int(3)].into())
            ))
        );
        assert_eq!(
            parse_packet("[[]]"),
            Ok(("", Packet::List(vec![Packet::List(VecDeque::new())].into())))
        );
        assert_eq!(
            parse_packet_pair("0\n1"),
            Ok(("", (Packet::Int(0), Packet::Int(1))))
        );
        assert_eq!(
            parse_all_packets("0\n1\n\n2\n3"),
            Ok((
                "",
                vec![
                    (Packet::Int(0), Packet::Int(1)),
                    (Packet::Int(2), Packet::Int(3))
                ]
            ))
        )
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input(INPUT)), 13);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_input(INPUT)), 140);
    }
}
