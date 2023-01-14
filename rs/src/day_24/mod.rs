use std::{collections::HashSet, iter::once};

use num::Complex;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Dir {
    U,
    D,
    L,
    R,
}

const ALL_DS: [Dir; 4] = [Dir::U, Dir::D, Dir::L, Dir::R];

impl TryFrom<char> for Dir {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            '^' => Self::U,
            'v' => Self::D,
            '<' => Self::L,
            '>' => Self::R,
            _ => Err(format!("invalid direction: {}", c))?,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Map {
    bs: HashSet<(Complex<usize>, Dir)>,
    w: usize,
    h: usize,
}

impl Map {
    fn move_blizzard(&self, (c, d): (Complex<usize>, Dir)) -> (Complex<usize>, Dir) {
        match d {
            Dir::U => {
                if c.im == 0 {
                    (complex!(c.re, self.h - 1), d)
                } else {
                    (complex!(c.re, c.im - 1), d)
                }
            }
            Dir::D => {
                if c.im == self.h - 1 {
                    (complex!(c.re, 0), d)
                } else {
                    (complex!(c.re, c.im + 1), d)
                }
            }
            Dir::L => {
                if c.re == 0 {
                    (complex!(self.w - 1, c.im), d)
                } else {
                    (complex!(c.re - 1, c.im), d)
                }
            }
            Dir::R => {
                if c.re == self.w - 1 {
                    (complex!(0, c.im), d)
                } else {
                    (complex!(c.re + 1, c.im), d)
                }
            }
        }
    }

    fn move_elf(&self, c: Complex<usize>, d: Dir) -> Option<Complex<usize>> {
        match d {
            Dir::U => c.im.checked_sub(1).map(|im| complex!(c.re, im)),
            Dir::D => (c.im < self.h - 1).then_some(complex!(c.re, c.im + 1)),
            Dir::L => c.re.checked_sub(1).map(|re| complex!(re, c.im)),
            Dir::R => (c.re < self.w - 1).then_some(complex!(c.re + 1, c.im)),
        }
    }

    fn min_to_goal(&mut self, start: Complex<usize>, goal: Complex<usize>) -> usize {
        let mut bs = self.bs.clone();
        let mut i = 0;
        while ALL_DS.iter().any(|d| bs.contains(&(start, *d))) {
            self.update_blizzards(&mut bs);
            i += 1;
        }
        let mut cs = HashSet::from([start]);
        loop {
            self.update_blizzards(&mut bs);
            self.do_legal_moves(&mut cs, &bs);
            cs.insert(start);
            i += 1;
            if cs.contains(&goal) {
                break;
            }
        }
        self.bs = bs;
        i + 1
    }

    fn do_legal_moves(
        &self,
        cs: &mut HashSet<Complex<usize>>,
        bs: &HashSet<(Complex<usize>, Dir)>,
    ) {
        *cs = cs
            .drain()
            .flat_map(|c| {
                let mut moves: Vec<Complex<usize>> = ALL_DS
                    .iter()
                    .filter_map(|d| {
                        self.move_elf(c, *d).and_then(|dst| {
                            (!ALL_DS.iter().any(|&d_check| bs.contains(&(dst, d_check))))
                                .then_some(dst)
                        })
                    })
                    .collect();
                if !ALL_DS.iter().any(|&d_check| bs.contains(&(c, d_check))) {
                    moves.push(c);
                }
                moves
            })
            .collect();
    }

    fn update_blizzards(&self, bs: &mut HashSet<(Complex<usize>, Dir)>) {
        *bs = bs.drain().map(|b| self.move_blizzard(b)).collect();
    }

    fn step_blizzards(&mut self) {
        self.bs = self
            .bs
            .clone()
            .drain()
            .map(|b| self.move_blizzard(b))
            .collect();
    }
}

impl TryFrom<&str> for Map {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let w = s.split_once('\n').unwrap().0.len() - 2;
        let h = s.lines().count() - 2;
        let bs = s
            .lines()
            .skip(1)
            .enumerate()
            .flat_map(|(i, l)| {
                l.trim_matches('#')
                    .chars()
                    .enumerate()
                    .filter_map(move |(j, c)| {
                        "^v<>"
                            .contains(c)
                            .then(|| Dir::try_from(c).map(|d| (complex!(j, i), d)))
                    })
            })
            .collect::<Result<HashSet<_>, Self::Error>>()?;
        Ok(Self { bs, w, h })
    }
}

#[aoc_generator(day24)]
pub fn get_input(input: &str) -> Map {
    input.try_into().unwrap()
}

#[aoc(day24, part1)]
pub fn part_1(map: &Map) -> usize {
    map.clone()
        .min_to_goal(complex!(0, 0), complex!(map.w - 1, map.h - 1))
}

#[aoc(day24, part2)]
pub fn part_2(map: &Map) -> usize {
    let mut map = map.clone();
    let (start, end) = (complex!(0, 0), complex!(map.w - 1, map.h - 1));
    let goal_1 = map.min_to_goal(start, end);
    map.step_blizzards();
    let snack = map.min_to_goal(end, start);
    map.step_blizzards();
    let goal_2 = map.min_to_goal(start, end);
    goal_1 + snack + goal_2
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../test_data/day_24.txt");

    #[test]
    fn test_map_simple() {
        let mut bs = HashSet::from([
            (complex!(0, 0), Dir::U),
            (complex!(0, 0), Dir::D),
            (complex!(0, 0), Dir::L),
            (complex!(0, 0), Dir::R),
        ]);
        let map = Map {
            bs: bs.clone(),
            w: 2,
            h: 2,
        };
        map.update_blizzards(&mut bs);
        let expect_1 = HashSet::from([
            (complex!(0, 1), Dir::U),
            (complex!(0, 1), Dir::D),
            (complex!(1, 0), Dir::L),
            (complex!(1, 0), Dir::R),
        ]);
        assert_eq!(bs, expect_1);
        map.update_blizzards(&mut bs);
        let expect_2 = HashSet::from([
            (complex!(0, 0), Dir::U),
            (complex!(0, 0), Dir::D),
            (complex!(0, 0), Dir::L),
            (complex!(0, 0), Dir::R),
        ]);
        assert_eq!(bs, expect_2);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input(INPUT)), 18);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_input(INPUT)), 54);
    }
}
