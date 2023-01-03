use std::{
    collections::HashMap,
    fmt::Debug,
    hash::Hash,
    ops::{Add, Div, Index, Mul, Sub},
    str::FromStr,
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, anychar, digit1, line_ending, space0},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

#[derive(Clone, Copy, Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl TryFrom<char> for Op {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '+' => Ok(Op::Add),
            '-' => Ok(Op::Sub),
            '*' => Ok(Op::Mul),
            '/' => Ok(Op::Div),
            _ => Err(()),
        }
    }
}

impl Op {
    fn eval<T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>>(
        &self,
        a: T,
        b: T,
    ) -> T {
        match self {
            Op::Add => a + b,
            Op::Sub => a - b,
            Op::Mul => a * b,
            Op::Div => a / b,
        }
    }

    fn inverse_left<
        T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
    >(
        &self,
        a: T,
        image: T,
    ) -> T {
        match self {
            Op::Add => image - a,
            Op::Sub => a - image,
            Op::Mul => image / a,
            Op::Div => a / image,
        }
    }

    fn inverse_right<
        T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
    >(
        &self,
        b: T,
        image: T,
    ) -> T {
        match self {
            Op::Add => image - b,
            Op::Sub => b + image,
            Op::Mul => image / b,
            Op::Div => b * image,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Job<T, I> {
    Const(T),
    Expr(Op, I, I),
}

impl<
        T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
        I: Eq + Hash,
    > Job<T, I>
{
    fn eval(&self, id_table: &HashMap<I, Self>) -> T {
        match self {
            Self::Const(n) => *n,
            Self::Expr(op, job_a, job_b) => op.eval(
                id_table[job_a].eval(id_table),
                id_table[job_b].eval(id_table),
            ),
        }
    }
}

impl<
        T: Copy + Debug + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
        I: Debug + Eq + Hash,
    > Job<Option<T>, I>
{
    fn eval_opt(&self, id_table: &HashMap<I, Self>) -> Option<T> {
        match self {
            Self::Const(n) => *n,
            Self::Expr(op, job_a, job_b) => {
                match (
                    id_table[job_a].eval_opt(id_table),
                    id_table[job_b].eval_opt(id_table),
                ) {
                    (Some(a), Some(b)) => match op {
                        Op::Add => Some(a + b),
                        Op::Sub => Some(a - b),
                        Op::Mul => Some(a * b),
                        Op::Div => Some(a / b),
                    },
                    _ => None,
                }
            }
        }
    }

    fn solve_for_none(&self, id_table: &HashMap<I, Self>, ans: T) -> T {
        match self {
            Self::Const(n) => match n {
                Some(n) => *n,
                None => ans,
            },
            Self::Expr(op, job_a_ix, job_b_ix) => {
                let (job_a, job_b) = (&id_table[job_a_ix], &id_table[job_b_ix]);
                if let Some(result) = self.eval_opt(id_table) {
                    result
                } else {
                    match (job_a.eval_opt(id_table), job_b.eval_opt(id_table)) {
                        (Some(a), None) => job_b.solve_for_none(id_table, op.inverse_left(a, ans)),
                        (None, Some(b)) => job_a.solve_for_none(id_table, op.inverse_right(b, ans)),
                        _ => unreachable!(),
                    }
                }
            }
        }
    }
}

struct Monkey<T, I> {
    id: Vec<u8>,
    job: Job<T, I>,
}

fn parse_monkeys<T: FromStr>(s: &str) -> IResult<&str, Vec<Monkey<T, Vec<u8>>>> {
    separated_list1(line_ending, parse_monkey)(s)
}

fn parse_monkey<T: FromStr>(s: &str) -> IResult<&str, Monkey<T, Vec<u8>>> {
    map(separated_pair(alpha1, tag(": "), parse_job), |(id, job)| {
        Monkey {
            id: id.as_bytes().to_vec(),
            job,
        }
    })(s)
}

fn parse_job<T: FromStr>(s: &str) -> IResult<&str, Job<T, Vec<u8>>> {
    alt((
        map_res(digit1, |n: &str| {
            Ok::<Job<T, Vec<u8>>, <T as FromStr>::Err>(Job::Const(n.parse()?))
        }),
        map_res(
            tuple((alpha1, delimited(space0, anychar, space0), alpha1)),
            |(a, op, b): (&str, char, &str)| {
                Ok::<Job<T, Vec<u8>>, <Op as TryFrom<char>>::Error>(Job::Expr(
                    op.try_into()?,
                    a.as_bytes().to_vec(),
                    b.as_bytes().to_vec(),
                ))
            },
        ),
    ))(s)
}

#[aoc_generator(day21)]
pub fn get_input(input: &str) -> HashMap<Vec<u8>, Job<isize, Vec<u8>>> {
    parse_monkeys(input)
        .unwrap()
        .1
        .iter()
        .map(|monkey| (monkey.id.clone(), monkey.job.clone()))
        .collect()
}

#[aoc(day21, part1)]
pub fn part_1(monkey_table: &HashMap<Vec<u8>, Job<isize, Vec<u8>>>) -> isize {
    monkey_table["root".as_bytes()].eval(&monkey_table)
}

#[aoc(day21, part2)]
pub fn part_2(monkey_table: &HashMap<Vec<u8>, Job<isize, Vec<u8>>>) -> isize {
    let mut monkey_table: HashMap<&[u8], Job<Option<isize>, &[u8]>> =
        HashMap::from_iter(monkey_table.iter().map(|(k, v)| {
            (
                k.as_slice(),
                match v {
                    Job::Const(n) => Job::Const(Some(*n)),
                    Job::Expr(op, a, b) => Job::Expr(*op, a.as_slice(), b.as_slice()),
                },
            )
        }));
    monkey_table
        .entry(b"humn")
        .and_modify(|h| *h = Job::Const(None));
    match &monkey_table[b"root".as_slice()] {
        Job::Const(_) => panic!("root is const"),
        Job::Expr(_, ix_a, ix_b) => match (
            monkey_table[ix_a].eval_opt(&monkey_table),
            monkey_table[ix_b].eval_opt(&monkey_table),
        ) {
            (None, Some(b)) => monkey_table[ix_a].solve_for_none(&monkey_table, b),
            (Some(a), None) => monkey_table[ix_b].solve_for_none(&monkey_table, a),
            _ => panic!("neither a nor b is none"),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../test_data/day_21.txt");

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input(INPUT)), 152);
    }

    #[test]
    fn test_job_solve_for_none() {
        // set up simple expression tree corresponding to (1 + (2 * x)) == 7
        let mut id_table: HashMap<&[u8], Job<Option<isize>, &[u8]>> = HashMap::new();
        let (a, b, c, d, x) = (b"a", b"b", b"c", b"d", b"x");
        id_table.insert(x, Job::Const(None));
        id_table.insert(a, Job::Const(Some(1)));
        id_table.insert(b, Job::Const(Some(2)));
        id_table.insert(c, Job::Expr(Op::Mul, b, x));
        id_table.insert(d, Job::Expr(Op::Add, a, c));
        assert_eq!(id_table[&d.as_slice()].solve_for_none(&id_table, 7), 3);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_input(INPUT)), 301);
    }
}
