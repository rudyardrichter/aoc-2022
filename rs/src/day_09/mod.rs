use std::collections::{HashMap, HashSet};

use num::Complex;
use sscanf::scanf;

#[aoc_generator(day9)]
pub fn get_input(input: &str) -> Vec<Complex<isize>> {
    input
        .lines()
        .map(|l| {
            let (dir, dist) = scanf!(l, "{} {}", char, isize).unwrap();
            match dir {
                'L' => Complex::new(-dist, 0),
                'R' => Complex::new(dist, 0),
                'U' => Complex::new(0, dist),
                'D' => Complex::new(0, -dist),
                _ => panic!("Invalid direction"),
            }
        })
        .collect()
}

struct Rope {
    head: Complex<isize>,
    tail: Complex<isize>,
}

impl Default for Rope {
    fn default() -> Self {
        Self {
            head: Complex::new(0, 0),
            tail: Complex::new(0, 0),
        }
    }
}

fn sign(c: &Complex<isize>) -> Complex<isize> {
    Complex::new(c.re.signum(), c.im.signum())
}

impl Rope {
    fn tail_positions(&mut self, moves: &Vec<Complex<isize>>) -> HashSet<Complex<isize>> {
        let mut result = HashSet::from([Complex::new(0, 0)]);
        moves.iter().for_each(|m| {
            for _ in 0..m.l1_norm() {
                self.head += sign(m);
                let d = self.head - self.tail;
                if d.norm_sqr() >= 4 {
                    self.tail += sign(&d);
                    result.insert(self.tail);
                }
            }
        });
        result
    }
}

struct LongRope {
    knots: Vec<Complex<isize>>,
}

impl LongRope {
    fn with_len(len: usize) -> Self {
        Self {
            knots: vec![Complex::new(0, 0); len],
        }
    }

    fn tail_positions(
        &mut self,
        moves: &Vec<Complex<isize>>,
        only_track: Option<usize>,
    ) -> HashMap<usize, HashSet<Complex<isize>>> {
        let mut results = HashMap::from_iter(
            self.knots
                .iter()
                .cloned()
                .enumerate()
                .map(|(i, k)| (i, HashSet::from([k]))),
        );
        moves.iter().for_each(|m| {
            for _ in 0..m.l1_norm() {
                self.knots[0] += sign(m);
                for i in 1..self.knots.len() {
                    let d = self.knots[i - 1] - self.knots[i];
                    if d.norm_sqr() >= 4 {
                        self.knots[i] += sign(&d);
                        if only_track.map_or(true, |o| o == i) {
                            results
                                .entry(i)
                                .or_insert(HashSet::new())
                                .insert(self.knots[i]);
                        }
                    }
                }
            }
        });
        results
    }
}

#[aoc(day9, part1)]
pub fn part_1(moves: &Vec<Complex<isize>>) -> usize {
    Rope::default().tail_positions(moves).len()
}

#[aoc(day9, part2)]
pub fn part_2(moves: &Vec<Complex<isize>>) -> usize {
    LongRope::with_len(10).tail_positions(moves, Some(9))[&9].len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2\n";
    const INPUT_2: &'static str = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20\n";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input(INPUT)), 13);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_input(INPUT)), 1);
        assert_eq!(part_2(&get_input(INPUT_2)), 36);
    }
}
