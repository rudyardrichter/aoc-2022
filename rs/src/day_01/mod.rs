use std::collections::BinaryHeap;

#[aoc_generator(day1)]
pub fn get_input(input: &str) -> BinaryHeap<usize> {
    input
        .split("\n\n")
        .map(|s| s.lines().map(|l| l.parse::<usize>().unwrap()).sum())
        .collect()
}

#[aoc(day1, part1)]
pub fn part_1(elves: &BinaryHeap<usize>) -> usize {
    *elves.peek().unwrap()
}

#[aoc(day1, part2)]
pub fn part_2(elves: &BinaryHeap<usize>) -> usize {
    elves.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input(INPUT)), 24000);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_input(INPUT)), 45000);
    }
}
