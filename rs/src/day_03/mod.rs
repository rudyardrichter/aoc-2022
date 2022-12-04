use std::{collections::HashSet, ops::Div};

#[derive(Debug)]
pub struct Rucksack {
    items: Vec<u8>,
}

impl Rucksack {
    fn overlap(&self) -> usize {
        let (left, right) = self.items.split_at(self.items.len().div(2));
        left.iter()
            .cloned()
            .collect::<HashSet<u8>>()
            .intersection(&right.iter().cloned().collect::<HashSet<u8>>())
            .map(|u| usize::from(*u))
            .sum()
    }

    fn items_set(&self) -> HashSet<u8> {
        self.items.iter().cloned().collect()
    }
}

impl From<&str> for Rucksack {
    fn from(s: &str) -> Self {
        Self {
            items: s
                .as_bytes()
                .iter()
                .map(|&c| if c > 96 { c - 96 } else { c - 38 })
                .collect::<Vec<u8>>(),
        }
    }
}

#[aoc_generator(day3)]
pub fn get_input(input: &str) -> Vec<Rucksack> {
    input.lines().map(Rucksack::from).collect()
}

#[aoc(day3, part1)]
pub fn part_1(rucksacks: &Vec<Rucksack>) -> usize {
    rucksacks.iter().map(|r| r.overlap()).sum()
}

#[aoc(day3, part2)]
pub fn part_2(rucksacks: &Vec<Rucksack>) -> usize {
    rucksacks
        .chunks(3)
        .map(|w| {
            usize::from(
                *w.iter()
                    .fold(w[0].items_set(), |acc, x| {
                        acc.intersection(&x.items_set()).cloned().collect()
                    })
                    .iter()
                    .next()
                    .unwrap(),
            )
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\n\
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
        PmmdzqPrVvPwwTWBwg\n\
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
        ttgJtRGJQctTZtZT\n\
        CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input(INPUT)), 157);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_input(INPUT)), 70);
    }
}
