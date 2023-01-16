use num::Integer;

pub struct Snafu(String);

impl From<&Snafu> for isize {
    fn from(snafu: &Snafu) -> isize {
        snafu
            .0
            .as_str()
            .chars()
            .rev()
            .fold((0, 1), |(n, p), c| match c {
                '0' => (n, p * 5),
                '1' => (n + p, p * 5),
                '2' => (n + 2 * p, p * 5),
                '-' => (n - p, p * 5),
                '=' => (n - 2 * p, p * 5),
                _ => panic!("invalid snafu"),
            })
            .0
    }
}

impl From<isize> for Snafu {
    fn from(n: isize) -> Self {
        let mut n = n;
        let mut r;
        let mut result: Vec<char> = vec![];
        while n > 0 {
            (n, r) = (n + 2).div_rem(&5);
            result.push(['=', '-', '0', '1', '2'][r as usize]);
        }
        result.reverse();
        Self(String::from_iter(result))
    }
}

#[aoc_generator(day25)]
pub fn get_input(input: &str) -> Vec<Snafu> {
    input.lines().map(|line| Snafu(line.to_owned())).collect()
}

#[aoc(day25, part1)]
pub fn part_1(snafus: &[Snafu]) -> String {
    Snafu::from(snafus.iter().map(isize::from).sum::<isize>()).0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../test_data/day_25.txt");

    #[test]
    fn test_isize_from_snafu() {
        assert_eq!(isize::from(&Snafu("0".to_owned())), 0);
        assert_eq!(isize::from(&Snafu("12111".to_owned())), 906);
        assert_eq!(isize::from(&Snafu("2=0=".to_owned())), 198);
        assert_eq!(isize::from(&Snafu("21".to_owned())), 11);
        assert_eq!(isize::from(&Snafu("2=01".to_owned())), 201);
        assert_eq!(isize::from(&Snafu("111".to_owned())), 31);
        assert_eq!(isize::from(&Snafu("20012".to_owned())), 1257);
        assert_eq!(isize::from(&Snafu("112".to_owned())), 32);
        assert_eq!(isize::from(&Snafu("1=-1=".to_owned())), 353);
        assert_eq!(isize::from(&Snafu("1-12".to_owned())), 107);
        assert_eq!(isize::from(&Snafu("12".to_owned())), 7);
        assert_eq!(isize::from(&Snafu("1=".to_owned())), 3);
        assert_eq!(isize::from(&Snafu("122".to_owned())), 37);
        assert_eq!(isize::from(&Snafu("1=-0-2".to_owned())), 1747);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input(INPUT)), "2=-1=0");
    }
}
