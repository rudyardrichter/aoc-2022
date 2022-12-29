use std::{
    ops::{Add, Index, IndexMut, Mul, Sub},
    str::FromStr,
};

use nom::{
    bytes::complete::take_while,
    character::complete::{digit1, line_ending},
    combinator::{map, map_res, value},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};
use num::{CheckedSub, Zero};

#[derive(Debug)]
pub struct Blueprint<T> {
    n: usize,
    costs: [R4<T>; 4],
}

#[derive(Clone, Copy, Debug)]
struct R4<T>(T, T, T, T);

impl<T: Copy + Ord> R4<T> {
    fn element_max(&self, other: &R4<T>) -> R4<T> {
        R4(
            self.0.max(other.0),
            self.1.max(other.1),
            self.2.max(other.2),
            self.3.max(other.3),
        )
    }
}

impl<T: Zero> Zero for R4<T> {
    fn zero() -> Self {
        R4(T::zero(), T::zero(), T::zero(), T::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero() && self.1.is_zero() && self.2.is_zero() && self.3.is_zero()
    }
}

impl<T: Add<Output = T>> Add for R4<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
            self.3 + rhs.3,
        )
    }
}

impl<T: Sub<Output = T>> Sub for R4<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
            self.3 - rhs.3,
        )
    }
}

impl<T: CheckedSub<Output = T>> CheckedSub for R4<T> {
    fn checked_sub(&self, rhs: &Self) -> Option<Self> {
        Some(Self(
            self.0.checked_sub(&rhs.0)?,
            self.1.checked_sub(&rhs.1)?,
            self.2.checked_sub(&rhs.2)?,
            self.3.checked_sub(&rhs.3)?,
        ))
    }
}

impl<T: Mul<Output = T>> Mul for R4<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(
            self.0 * rhs.0,
            self.1 * rhs.1,
            self.2 * rhs.2,
            self.3 * rhs.3,
        )
    }
}

impl<T> Index<usize> for R4<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            _ => panic!("invalid index"),
        }
    }
}

impl<T> IndexMut<usize> for R4<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            _ => panic!("invalid index"),
        }
    }
}

impl Blueprint<usize> {
    fn max_geodes(&self, t_0: usize) -> usize {
        let mut q = vec![(t_0, R4(1, 0, 0, 0), R4::zero())];
        let mut result = 0;
        let max_costs = self
            .costs
            .iter()
            .fold(R4::zero(), |acc, cost| acc.element_max(cost));
        while let Some((t, r, n)) = q.pop() {
            for i in 0..4 {
                if max_costs[i] != 0 && r[i] >= max_costs[i] {
                    continue;
                }
                let t_cost_opt = (0..4).fold(Some(0), |acc, j| {
                    if self.costs[i][j] == 0 {
                        acc
                    } else if r[j] == 0 {
                        None
                    } else {
                        acc.map(|a| {
                            a.max((self.costs[i][j] + r[j] - 1).saturating_sub(n[j]) / r[j])
                        })
                    }
                });
                if let Some(t_cost) = t_cost_opt {
                    if t > t_cost {
                        let n_new = n + r + r * R4(t_cost, t_cost, t_cost, t_cost);
                        let mut r_new = r;
                        r_new[i] += 1;
                        result = result.max(n_new[3]);
                        let t_new = t - t_cost - 1;
                        let could_make = n_new[3] + r_new[3] * t_new + (t_new * t_new + t_new) / 2;
                        // TODO: add more pruning
                        if t_new > 0 && could_make > result + 1 {
                            q.push((t_new, r_new, n_new - self.costs[i]));
                        }
                    }
                }
            }
        }
        result
    }

    fn quality(&self, t_0: usize) -> usize {
        self.n * self.max_geodes(t_0)
    }
}

fn not_num(s: &str) -> IResult<&str, ()> {
    value((), take_while(|c: char| !c.is_ascii_digit() && c != '\n'))(s)
}

fn parse_num<T: FromStr>(s: &str) -> IResult<&str, T> {
    map_res(digit1, |s: &str| s.parse())(s)
}

fn parse_blueprint(s: &str) -> IResult<&str, Blueprint<usize>> {
    map(
        tuple((
            not_num, parse_num, not_num, parse_num, not_num, parse_num, not_num, parse_num,
            not_num, parse_num, not_num, parse_num, not_num, parse_num, not_num,
        )),
        |(_, n, _, ores, _, clays, _, obs0, _, obs1, _, geode0, _, geode1, _)| Blueprint {
            n,
            costs: [
                R4(ores, 0, 0, 0),
                R4(clays, 0, 0, 0),
                R4(obs0, obs1, 0, 0),
                R4(geode0, 0, geode1, 0),
            ],
        },
    )(s)
}

fn parse_blueprints(s: &str) -> IResult<&str, Vec<Blueprint<usize>>> {
    separated_list1(line_ending, parse_blueprint)(s)
}

#[aoc_generator(day19)]
pub fn get_input(input: &str) -> Vec<Blueprint<usize>> {
    parse_blueprints(input).unwrap().1
}

#[aoc(day19, part1)]
pub fn part_1(blueprints: &[Blueprint<usize>]) -> usize {
    blueprints.iter().map(|b| b.quality(24)).sum()
}

#[aoc(day19, part2)]
pub fn part_2(blueprints: &[Blueprint<usize>]) -> usize {
    blueprints
        .iter()
        .take(3)
        .map(|bp| bp.max_geodes(32))
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../test_data/day_19.txt");

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input(INPUT)), 33);
    }

    #[test]
    #[ignore] // slow
    fn test_part_2() {
        assert_eq!(part_2(&get_input(INPUT)), 3472);
    }
}
