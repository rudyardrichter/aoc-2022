use std::{cell::RefCell, rc::Rc};

use sscanf::scanf;

#[derive(Debug, Clone)]
pub struct Cargo {
    stacks: Vec<Rc<RefCell<Vec<char>>>>,
    moves: Vec<(usize, usize, usize)>,
}

impl Cargo {
    fn tops(&self) -> String {
        self.stacks
            .iter()
            .filter_map(|s| s.borrow().last().copied())
            .collect()
    }

    fn execute_moves_1(&mut self) -> &Self {
        for &(n, from, to) in &self.moves {
            let mut stack_from = self.stacks[from - 1].borrow_mut();
            let mut stack_to = self.stacks[to - 1].borrow_mut();
            let drain_range = stack_from.len().saturating_sub(n)..;
            stack_to.extend(stack_from.drain(drain_range).rev())
        }
        self
    }

    fn execute_moves_2(&mut self) -> &Self {
        for &(n, from, to) in &self.moves {
            let mut stack_from = self.stacks[from - 1].borrow_mut();
            let mut stack_to = self.stacks[to - 1].borrow_mut();
            let drain_range = stack_from.len().saturating_sub(n)..;
            stack_to.extend(stack_from.drain(drain_range))
        }
        self
    }
}

impl TryFrom<&str> for Cargo {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let (stacks_chunk, moves_chunk) = s.split_once("\n\n").ok_or("parse error")?;
        let mut stacks_lines: Vec<&str> = stacks_chunk.lines().collect();
        let numbers_line = stacks_lines.pop().ok_or("bad input")?;
        let n = numbers_line
            .split_whitespace()
            .last()
            .map(|x| x.parse::<usize>().ok())
            .flatten()
            .ok_or("parse error")?;
        let mut stacks = Vec::new();
        (0..n).for_each(|_| stacks.push(Rc::new(RefCell::new(Vec::new()))));
        for line in stacks_lines.iter().rev() {
            for (i, chunk) in line.as_bytes().chunks(4).enumerate() {
                if chunk.get(0) == Some(&b'[') {
                    (*stacks[i]).borrow_mut().push(chunk[1] as char);
                }
            }
        }
        let moves: Vec<(usize, usize, usize)> = moves_chunk
            .lines()
            .map(|l| scanf!(l, "move {} from {} to {}", usize, usize, usize))
            .collect::<Result<Vec<(usize, usize, usize)>, _>>()
            .map_err(|_| "parse error")?;
        Ok(Cargo { stacks, moves })
    }
}

#[aoc_generator(day5)]
pub fn get_input(input: &str) -> Cargo {
    input.try_into().unwrap()
}

#[aoc(day5, part1)]
pub fn part_1(cargo: &Cargo) -> String {
    cargo.clone().execute_moves_1().tops()
}

#[aoc(day5, part2)]
pub fn part_2(cargo: &Cargo) -> String {
    cargo.clone().execute_moves_2().tops()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../../test_data/day_05.txt");

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input(INPUT)), "CMZ");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_input(INPUT)), "MCD");
    }
}
