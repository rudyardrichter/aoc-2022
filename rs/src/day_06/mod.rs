fn first_marker(s: &String, k: usize) -> Option<usize> {
    let bs = s.as_bytes();
    let mut mask: u32 = bs
        .iter()
        .take(k)
        .fold(0, |acc: u32, c| acc ^ 1 << (*c as u8 - 'a' as u8));
    for (i, (add, remove)) in bs.iter().skip(k).zip(bs.iter()).enumerate() {
        if mask.count_ones() as usize == k {
            return Some(i + k);
        }
        mask ^= 1 << *add as u8 - 'a' as u8;
        mask ^= 1 << *remove as u8 - 'a' as u8;
    }
    None
}

#[aoc_generator(day6)]
pub fn get_input(input: &str) -> String {
    input.to_owned()
}

#[aoc(day6, part1)]
pub fn part_1(s: &String) -> usize {
    first_marker(s, 4).unwrap()
}

#[aoc(day6, part2)]
pub fn part_2(s: &String) -> usize {
    first_marker(s, 14).unwrap()
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
