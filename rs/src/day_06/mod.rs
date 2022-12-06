use std::collections::HashSet;

fn first_marker(s: &String, k: usize) -> usize {
    s.as_bytes()
        .windows(k)
        .enumerate()
        .filter(|(_, w)| HashSet::<_>::from_iter(w.iter()).len() == k)
        .next()
        .unwrap()
        .0
        + k
}

#[aoc_generator(day6)]
pub fn get_input(input: &str) -> String {
    input.to_owned()
}

#[aoc(day6, part1)]
pub fn part_1(s: &String) -> usize {
    first_marker(s, 4)
}

#[aoc(day6, part2)]
pub fn part_2(s: &String) -> usize {
    first_marker(s, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input("mjqjpqmgbljsphdztnvjfqwrcgsmlb")), 7);
        assert_eq!(part_1(&get_input("bvwbjplbgvbhsrlpgdmjqwftvncz")), 5);
        assert_eq!(part_1(&get_input("nppdvjthqldpwncqszvftbrmjlhg")), 6);
        assert_eq!(part_1(&get_input("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")), 10);
        assert_eq!(part_1(&get_input("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")), 11);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_input("mjqjpqmgbljsphdztnvjfqwrcgsmlb")), 19);
        assert_eq!(part_2(&get_input("bvwbjplbgvbhsrlpgdmjqwftvncz")), 23);
        assert_eq!(part_2(&get_input("nppdvjthqldpwncqszvftbrmjlhg")), 23);
        assert_eq!(part_2(&get_input("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")), 29);
        assert_eq!(part_2(&get_input("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")), 26);
    }
}
