use std::collections::HashSet;

use num::Integer;

type Point = (isize, isize);
const CHECKS: [([Point; 3], Point); 4] = [
    // first three tuples are where to check in the grid, last is where to move to
    // N
    ([(-1, -1), (0, -1), (1, -1)], (0, -1)),
    // S
    ([(-1, 1), (0, 1), (1, 1)], (0, 1)),
    // W
    ([(-1, -1), (-1, 0), (-1, 1)], (-1, 0)),
    // E
    ([(1, -1), (1, 0), (1, 1)], (1, 0)),
];

#[derive(Clone, Debug)]
pub struct ElfMap {
    es: HashSet<(isize, isize)>,
}

impl ElfMap {
    fn do_rounds(&mut self, n: usize) -> &mut Self {
        for i in 0..n {
            self.step(i);
        }
        self
    }

    fn first_no_move_round(&mut self) -> usize {
        1 + (0..).map(|i| self.step(i)).take_while(|b| *b).count()
    }

    fn step(&mut self, n: usize) -> bool {
        let moves: Vec<((isize, isize), (isize, isize))> = self
            .es
            .iter()
            .filter_map(|&(x, y)| {
                if CHECKS
                    .iter()
                    .any(|(ss, _)| ss.iter().any(|s| self.es.contains(&(x + s.0, y + s.1))))
                {
                    for j in 0..4 {
                        let (ss, d) = CHECKS[(n + j) % 4];
                        if ss.iter().all(|s| !self.es.contains(&(x + s.0, y + s.1))) {
                            return Some(((x, y), (x + d.0, y + d.1)));
                        }
                    }
                }
                None
            })
            .collect();
        let mut moved = moves.len();
        for (src, dst) in moves.iter() {
            if !self.es.insert(*dst) {
                self.es.remove(dst);
                self.es
                    .insert((dst.0 + dst.0 - src.0, dst.1 + dst.1 - src.1));
                moved -= 2;
            } else {
                self.es.remove(src);
            }
        }
        moved > 0
    }

    fn bounds(&self) -> (isize, isize, isize, isize, usize) {
        self.es.iter().fold(
            (isize::MAX, isize::MAX, isize::MIN, isize::MIN, 0),
            |acc, (x, y)| {
                (
                    acc.0.min(*x as isize),
                    acc.1.min(*y as isize),
                    acc.2.max(*x as isize),
                    acc.3.max(*y as isize),
                    acc.4 + 1,
                )
            },
        )
    }

    fn empty_ground(&self) -> usize {
        let (min_x, min_y, max_x, max_y, n) = self.bounds();
        (1 + max_x - min_x) as usize * (1 + max_y - min_y) as usize - n
    }
}

impl TryFrom<&str> for ElfMap {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let n = s.split_once('\n').ok_or("")?.0.len();
        let es = HashSet::from_iter(s.bytes().filter(|c| *c != b'\n').enumerate().filter_map(
            |(i, c)| {
                (c == b'#').then(|| {
                    let (y, x) = i.div_rem(&n);
                    (x as isize, y as isize)
                })
            },
        ));
        Ok(Self { es })
    }
}

impl From<&ElfMap> for String {
    fn from(elves: &ElfMap) -> Self {
        let mut result: Vec<char> = Vec::new();
        let (min_x, min_y, max_x, max_y, _) = elves.bounds();
        for x in min_x..max_x {
            for y in min_y..max_y {
                if elves.es.contains(&(x, y)) {
                    result.push('#')
                } else {
                    result.push('.')
                }
            }
            result.push('\n')
        }
        result.into_iter().collect()
    }
}

#[aoc_generator(day23)]
pub fn get_input(input: &str) -> ElfMap {
    input.try_into().unwrap()
}

#[aoc(day23, part1)]
pub fn part_1(map: &ElfMap) -> usize {
    map.clone().do_rounds(10).empty_ground()
}

#[aoc(day23, part2)]
pub fn part_2(map: &ElfMap) -> usize {
    map.clone().first_no_move_round()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../test_data/day_23.txt");

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input(INPUT)), 110);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_input(INPUT)), 20);
    }
}
