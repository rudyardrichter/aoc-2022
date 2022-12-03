#[derive(Debug)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

impl TryFrom<char> for Outcome {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Self::Lose),
            'Y' => Ok(Self::Draw),
            'Z' => Ok(Self::Win),
            _ => Err("parse error"),
        }
    }
}

impl Outcome {
    fn score(&self) -> usize {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn score(&self) -> usize {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }

    fn get_outcome(&self, outcome: Outcome) -> Self {
        match outcome {
            Outcome::Win => match self {
                Self::Rock => Self::Paper,
                Self::Paper => Self::Scissors,
                Self::Scissors => Self::Rock,
            },
            Outcome::Draw => self.clone(),
            Outcome::Lose => match self {
                Self::Rock => Self::Scissors,
                Self::Paper => Self::Rock,
                Self::Scissors => Self::Paper,
            },
        }
    }

    fn against(&self, other: &RPS) -> Outcome {
        match self {
            Self::Rock => match other {
                Self::Rock => Outcome::Draw,
                Self::Paper => Outcome::Lose,
                Self::Scissors => Outcome::Win,
            },
            Self::Paper => match other {
                Self::Rock => Outcome::Win,
                Self::Paper => Outcome::Draw,
                Self::Scissors => Outcome::Lose,
            },
            Self::Scissors => match other {
                Self::Rock => Outcome::Lose,
                Self::Paper => Outcome::Win,
                Self::Scissors => Outcome::Draw,
            },
        }
    }
}

impl TryFrom<char> for RPS {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(RPS::Rock),
            'B' => Ok(RPS::Paper),
            'C' => Ok(RPS::Scissors),
            'X' => Ok(RPS::Rock),
            'Y' => Ok(RPS::Paper),
            'Z' => Ok(RPS::Scissors),
            _ => Err("invalid"),
        }
    }
}

#[derive(Debug)]
pub struct Match {
    me: RPS,
    op: RPS,
}

impl Match {
    fn score(&self) -> usize {
        self.me.score() + self.me.against(&self.op).score()
    }

    fn from_str_1(value: &str) -> Self {
        Self {
            op: value.chars().nth(0).unwrap().try_into().unwrap(),
            me: value.chars().nth(2).unwrap().try_into().unwrap(),
        }
    }

    fn from_str_2(value: &str) -> Self {
        let op = value.chars().nth(0).unwrap().try_into().unwrap();
        Self {
            op,
            me: op.get_outcome(Outcome::try_from(value.chars().nth(2).unwrap()).unwrap()),
        }
    }
}

#[aoc_generator(day2)]
pub fn get_input(input: &str) -> String {
    input.to_owned()
}

#[aoc(day2, part1)]
pub fn part_1(input: &String) -> usize {
    input.lines().map(|l| Match::from_str_1(l).score()).sum()
}

#[aoc(day2, part2)]
pub fn part_2(input: &String) -> usize {
    input.lines().map(|l| Match::from_str_2(l).score()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y\nB X\nC Z";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input(INPUT)), 15);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_input(INPUT)), 12);
    }
}
