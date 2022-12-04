use sscanf::sscanf;

struct Elf {
    a: usize,
    b: usize,
}

impl Elf {
    fn fully_contains(&self, other: &Elf) -> bool {
        self.a <= other.a && self.b >= other.b
    }

    fn overlaps(&self, other: &Elf) -> bool {
        self.a <= other.b && self.b >= other.a
    }
}

pub struct ElfPair {
    elf_1: Elf,
    elf_2: Elf,
}

impl ElfPair {
    fn one_fully_contains(&self) -> bool {
        self.elf_1.fully_contains(&self.elf_2) || self.elf_2.fully_contains(&self.elf_1)
    }

    fn overlaps(&self) -> bool {
        self.elf_1.overlaps(&self.elf_2)
    }
}

impl From<&str> for ElfPair {
    fn from(s: &str) -> Self {
        let (elf_1_a, elf_1_b, elf_2_a, elf_2_b) =
            sscanf!(s, "{}-{},{}-{}", usize, usize, usize, usize).unwrap();
        ElfPair {
            elf_1: Elf {
                a: elf_1_a,
                b: elf_1_b,
            },
            elf_2: Elf {
                a: elf_2_a,
                b: elf_2_b,
            },
        }
    }
}

#[aoc_generator(day4)]
pub fn get_input(input: &str) -> Vec<ElfPair> {
    input.lines().map(ElfPair::from).collect()
}

#[aoc(day4, part1)]
pub fn part_1(elf_pairs: &Vec<ElfPair>) -> usize {
    elf_pairs
        .iter()
        .filter(|&pair| pair.one_fully_contains())
        .count()
}

#[aoc(day4, part2)]
pub fn part_2(elf_pairs: &Vec<ElfPair>) -> usize {
    elf_pairs.iter().filter(|&pair| pair.overlaps()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8\n\
        2-3,4-5\n\
        5-7,7-9\n\
        2-8,3-7\n\
        6-6,4-6\n\
        2-6,4-8";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input(INPUT)), 2);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_input(INPUT)), 4);
    }
}
