use std::collections::HashSet;

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space0},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};
use num::Complex;

fn parse_scans(s: &str) -> IResult<&str, Vec<Vec<Complex<usize>>>> {
    separated_list1(
        line_ending,
        separated_list1(
            delimited(space0, tag("->"), space0),
            map(
                separated_pair(parse_int, delimited(space0, tag(","), space0), parse_int),
                |(a, b)| Complex::new(a, b),
            ),
        ),
    )(s)
}

fn parse_int(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse())(input)
}

fn scans_to_coords(scans: Vec<Vec<Complex<usize>>>) -> HashSet<Complex<usize>> {
    HashSet::from_iter(
        scans
            .iter()
            .map(|s| {
                s.iter().zip(s.iter().skip(1)).map(|(src, dst)| {
                    (src.re.min(dst.re)..=src.re.max(dst.re))
                        .cartesian_product(src.im.min(dst.im)..=src.im.max(dst.im))
                        .map(|(re, im)| Complex::new(re, im))
                })
            })
            .flatten()
            .flatten(),
    )
}

fn sand_rests_at(
    start: Complex<usize>,
    rocks: &HashSet<Complex<usize>>,
    floor: bool,
) -> HashSet<Complex<usize>> {
    let bottom = rocks.iter().map(|c| c.im).max().unwrap();
    let mut sand = HashSet::new();
    let mut stack = vec![start];
    'outer: while let Some(mut current) = stack.pop() {
        while (!floor && current.im < bottom) || (floor && current.im < bottom + 2) {
            let mut blocked = true;
            'inner: for next in vec![
                current + complex!(0, 1),
                current + complex!(0, 1) - complex!(1, 0),
                current + complex!(1, 1),
            ] {
                if !sand.contains(&next) && !rocks.contains(&next) && next.im < bottom + 2 {
                    blocked = false;
                    stack.push(current);
                    current = next;
                    break 'inner;
                }
            }
            if !floor && current.im == bottom {
                break 'outer;
            }
            if blocked {
                sand.insert(current);
                break;
            }
        }
        if current == start {
            break;
        }
    }
    sand
}

#[aoc_generator(day14)]
pub fn get_input(input: &str) -> HashSet<Complex<usize>> {
    scans_to_coords(parse_scans(input).unwrap().1)
}

#[aoc(day14, part1)]
pub fn part_1(rocks: &HashSet<Complex<usize>>) -> usize {
    sand_rests_at(Complex::new(500, 0), rocks, false).len()
}

#[aoc(day14, part2)]
pub fn part_2(rocks: &HashSet<Complex<usize>>) -> usize {
    sand_rests_at(Complex::new(500, 0), rocks, true).len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input(INPUT)), 24);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_input(INPUT)), 93);
    }
}
