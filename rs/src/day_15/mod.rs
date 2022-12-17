use std::collections::{HashMap, VecDeque};

use itertools::Itertools;
use num::Complex;
use sscanf::scanf;

pub struct Area {
    pairs: Vec<(Complex<isize>, Complex<isize>, isize)>,
}

impl Area {
    fn contains(&self, point: Complex<isize>) -> bool {
        self.pairs
            .iter()
            .any(|(s, _, r)| (s - point).l1_norm() <= *r)
    }
}

impl TryFrom<&str> for Area {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let pairs = s
            .lines()
            .map(|l| {
                let (sx, sy, bx, by) = scanf!(
                    l,
                    "Sensor at x={isize}, y={isize}: closest beacon is at x={isize}, y={isize}"
                )
                .map_err(|_| format!("couldn't parse line: {}", l).to_string())?;
                let s = complex!(sx, sy);
                let b = complex!(bx, by);
                Ok((s, b, (s - b).l1_norm()))
            })
            .collect::<Result<Vec<(Complex<isize>, Complex<isize>, isize)>, Self::Error>>()?;
        Ok(Area { pairs })
    }
}

fn line_overlap(area: &Area, y: isize) -> usize {
    let mut overlaps: VecDeque<(isize, isize)> = VecDeque::new();
    for (s, _, r) in area.pairs.iter() {
        let dy = (y - s.im).abs();
        if dy <= *r {
            overlaps.push_back((s.re - (r - dy), s.re + (r - dy)));
        }
    }
    overlaps.make_contiguous().sort();
    let mut current = overlaps.pop_front().unwrap();
    let mut result = Vec::new();
    for o in overlaps.iter() {
        if o.0 > current.1 {
            result.push(current);
            current = *o;
        } else if o.1 > current.1 {
            current.1 = o.1;
        }
    }
    result.push(current);
    result.iter().map(|(a, b)| (b - a) as usize).sum()
}

#[aoc_generator(day15)]
pub fn get_input(input: &str) -> Area {
    input.try_into().unwrap()
}

#[aoc(day15, part1)]
pub fn part_1(area: &Area) -> usize {
    line_overlap(area, 2_000_000)
}

fn sole_beacon(area: &Area, limits: ((isize, isize), (isize, isize))) -> Option<(isize, isize)> {
    let mut counts_t: HashMap<isize, usize> = HashMap::new();
    let mut counts_u: HashMap<isize, usize> = HashMap::new();
    for (s, _, r) in area.pairs.iter() {
        let a = s.re - r - 1;
        let b = s.re + r + 1;
        *counts_t.entry(a - s.im).or_insert(0) += 1;
        *counts_t.entry(b - s.im).or_insert(0) += 1;
        *counts_u.entry(a + s.im).or_insert(0) += 1;
        *counts_u.entry(b + s.im).or_insert(0) += 1;
    }
    counts_t
        .iter()
        .filter(|(_, n)| **n > 1)
        .cartesian_product(counts_u.iter().filter(|(_, n)| **n > 1))
        .find_map(|((t, _), (u, _))| {
            let (x, y) = (t + (u - t) / 2, (u - t) / 2);
            (limits.0 .0 <= x
                && x <= limits.1 .0
                && limits.0 .1 <= y
                && y <= limits.1 .1
                && !area.contains(complex!(x, y)))
            .then(|| (x, y))
        })
}

fn frequency(beacon: (isize, isize)) -> usize {
    (beacon.0 * 4_000_000isize + beacon.1) as usize
}

#[aoc(day15, part2)]
pub fn part_2(area: &Area) -> usize {
    frequency(sole_beacon(area, ((0, 0), (4_000_000, 4_000_000))).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../test_data/day_15.txt");

    #[test]
    fn test_part_1() {
        assert_eq!(line_overlap(&get_input(INPUT), 10), 26);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            frequency(sole_beacon(&get_input(INPUT), ((0, 0), (20, 20))).unwrap()),
            56000011
        );
    }
}
