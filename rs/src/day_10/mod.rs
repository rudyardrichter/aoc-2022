#[aoc_generator(day10)]
pub fn get_input(input: &str) -> Vec<isize> {
    input
        .lines()
        .flat_map(|l| {
            l.split_whitespace()
                .map(|s| s.parse::<isize>().unwrap_or(0))
        })
        .collect()
}

#[aoc(day10, part1)]
pub fn part_1(ns: &[isize]) -> isize {
    (1..=220)
        .zip(ns.iter())
        .fold((0, 1), |(sum, x), (i, n)| {
            if (i - 20) % 40 == 0 {
                (sum + i * x, x + n)
            } else {
                (sum, x + n)
            }
        })
        .0
}

#[aoc(day10, part2)]
pub fn part_2(ns: &[isize]) -> &'static str {
    let letters: String = (0..)
        .zip(ns.iter())
        .fold((Vec::new(), 1), |(mut pixels, x), (i, n)| {
            pixels.push(i % 40 - 1 <= x && x <= i % 40 + 1);
            (pixels, x + n)
        })
        .0
        .iter()
        .map(|&pixel| if pixel { '█' } else { ' ' })
        .collect::<Vec<char>>()
        .chunks(40)
        .intersperse(&['\n'])
        .flatten()
        .collect();
    println!("{}", letters);
    ""
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../test_data/day_10.txt");

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input(INPUT)), 13140);
    }

    #[test]
    fn test_part_2() {
        // TODO
    }
}
