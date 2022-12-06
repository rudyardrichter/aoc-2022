use sscanf::scanf;

#[derive(Debug, Clone)]
struct Stack {
    crates: Vec<char>,
}

#[derive(Debug, Clone)]
pub struct Cargo {
    stacks: Vec<Box<Stack>>,
    moves: Vec<(usize, usize, usize)>,
}

impl Cargo {
    fn tops(&self) -> String {
        self.stacks
            .iter()
            .filter_map(|s| s.crates.last().copied())
            .collect()
    }

    fn execute_moves_1(&self) -> Result<Self, &'static str> {
        let mut new = self.clone();
        for &(n, from, to) in &self.moves {
            for _ in 0..n {
                let crate_ = new.stacks[from - 1].crates.pop().ok_or("too many pops")?;
                new.stacks[to - 1].crates.push(crate_);
            }
        }
        Ok(new)
    }

    fn execute_moves_2(&self) -> Result<Self, &'static str> {
        let mut new = self.clone();
        for &(n, from, to) in &self.moves {
            let mut crates = Vec::new();
            for _ in 0..n {
                crates.push(
                    new.stacks[from - 1]
                        .crates
                        .pop()
                        .ok_or("too many pops")
                        .unwrap(),
                );
            }
            crates.reverse();
            new.stacks[to - 1].crates.extend(crates);
        }
        Ok(new)
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
        let mut stacks: Vec<Box<Stack>> = Vec::new();
        (0..n).for_each(|_| stacks.push(Box::new(Stack { crates: Vec::new() })));
        for line in stacks_lines.iter().rev() {
            for (i, chunk) in line.as_bytes().chunks(4).enumerate() {
                if chunk.get(0) == Some(&b'[') {
                    (*stacks[i]).crates.push(chunk[1] as char);
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
    cargo.execute_moves_1().unwrap().tops()
}

#[aoc(day5, part2)]
pub fn part_2(cargo: &Cargo) -> String {
    cargo.execute_moves_2().unwrap().tops()
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
