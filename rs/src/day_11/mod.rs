use std::{cell::RefCell, rc::Rc};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, digit1, line_ending, space0},
    combinator::{map, map_res, opt, value},
    multi::{many1, many_till, separated_list1},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};
use num::integer::lcm;

// Monkey {usize}:
//   Starting items: {str}
//   Operation: new = old {op} {operand}
//   Test: divisible by {usize}
//     If true: throw to monkey {usize}
//     If false: throw to monkey {usize}

fn parse_monkeys(s: &str) -> IResult<&str, Vec<Monkey>> {
    many1(terminated(parse_monkey, opt(line_ending)))(s)
}

fn parse_monkey(s: &str) -> IResult<&str, Monkey> {
    map(
        tuple((
            many_till(anychar, line_ending), // skip first line of monkey
            delimited(space0, parse_items, line_ending),
            delimited(space0, parse_operation, line_ending),
            delimited(space0, parse_div, line_ending),
            delimited(space0, parse_dst_true, line_ending),
            delimited(space0, parse_dst_false, opt(line_ending)),
        )),
        |(_, items, operation, test_div, dst_true, dst_false)| Monkey {
            items,
            operation,
            test_div,
            dst_true,
            dst_false,
        },
    )(s)
}

fn parse_items(s: &str) -> IResult<&str, Vec<usize>> {
    preceded(
        tag("Starting items: "),
        separated_list1(tag(", "), map_res(digit1, |n: &str| n.parse::<usize>())),
    )(s)
}

fn parse_operation(s: &str) -> IResult<&str, Operation> {
    preceded(
        tag("Operation: new = old"),
        map(
            tuple((
                alt((
                    value(Operator::Plus, tag(" + ")),
                    value(Operator::Times, tag(" * ")),
                )),
                alt((
                    value(Operand::Old, tag("old")),
                    map_res(digit1, |n: &str| n.parse::<usize>().map(Operand::Num)),
                )),
            )),
            |(operator, operand)| Operation {
                op: operator,
                arg_a: Operand::Old,
                arg_b: operand,
            },
        ),
    )(s)
}

fn parse_div(s: &str) -> IResult<&str, usize> {
    preceded(
        tag("Test: divisible by "),
        map_res(digit1, |n: &str| n.parse::<usize>()),
    )(s)
}

fn parse_dst_true(s: &str) -> IResult<&str, usize> {
    preceded(
        tag("If true: throw to monkey "),
        map_res(digit1, |n: &str| n.parse::<usize>()),
    )(s)
}

fn parse_dst_false(s: &str) -> IResult<&str, usize> {
    preceded(
        tag("If false: throw to monkey "),
        map_res(digit1, |n: &str| n.parse::<usize>()),
    )(s)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Operator {
    Plus,
    Times,
}

impl TryFrom<&str> for Operator {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "+" => Ok(Operator::Plus),
            "*" => Ok(Operator::Times),
            _ => Err(format!("invalid operator: {}", s)),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Operand {
    Old,
    Num(usize),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Operation {
    op: Operator,
    arg_a: Operand,
    arg_b: Operand,
}

impl Operation {
    fn compute(&self, old: usize) -> usize {
        let a = match self.arg_a {
            Operand::Old => old,
            Operand::Num(n) => n,
        };
        let b = match self.arg_b {
            Operand::Old => old,
            Operand::Num(n) => n,
        };
        match self.op {
            Operator::Plus => a + b,
            Operator::Times => a * b,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test_div: usize,
    dst_true: usize,
    dst_false: usize,
}

impl TryFrom<&str> for Monkey {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        parse_monkey(s).map(|(_, m)| m).map_err(|e| e.to_string())
    }
}

fn do_rounds(monkeys: &mut Vec<Rc<RefCell<Monkey>>>, n: usize, worry_div: usize) -> Vec<usize> {
    let mut results = vec![0; monkeys.len()];
    let l = monkeys.iter().map(|m| m.borrow().test_div).fold(1, lcm);
    for _ in 0..n {
        for i in 0..monkeys.len() {
            let mut m = monkeys[i].borrow_mut();
            while let Some(old) = m.items.pop() {
                results[i] += 1;
                let new = ((m.operation.compute(old) % l) / worry_div) as usize;
                if new % m.test_div == 0 {
                    monkeys[m.dst_true].borrow_mut().items.push(new);
                } else {
                    monkeys[m.dst_false].borrow_mut().items.push(new);
                }
            }
        }
    }
    results
}

#[aoc_generator(day11)]
pub fn get_input(input: &str) -> Vec<Rc<RefCell<Monkey>>> {
    parse_monkeys(input)
        .unwrap()
        .1
        .into_iter()
        .map(|m| Rc::new(RefCell::new(m)))
        .collect()
}

fn monkey_business(monkeys: &mut [Rc<RefCell<Monkey>>], r: usize, w: usize) -> usize {
    let mut inspections = do_rounds(&mut monkeys.to_owned(), r, w);
    inspections.sort();
    inspections.pop().unwrap() * inspections.pop().unwrap()
}

#[aoc(day11, part1)]
pub fn part_1(monkeys: &[Rc<RefCell<Monkey>>]) -> usize {
    monkey_business(&mut monkeys.to_owned(), 20, 3)
}

#[aoc(day11, part2)]
pub fn part_2(monkeys: &[Rc<RefCell<Monkey>>]) -> usize {
    monkey_business(&mut monkeys.to_owned(), 10_000, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../test_data/day_11.txt");

    #[test]
    fn test_parse_monkey() {
        let s = concat!(
            "Monkey 0:\n",
            "  Starting items: 79, 98\n",
            "  Operation: new = old * 19\n",
            "  Test: divisible by 23\n",
            "    If true: throw to monkey 2\n",
            "    If false: throw to monkey 3\n",
        );
        assert_eq!(
            parse_monkey(s),
            Ok((
                "",
                Monkey {
                    items: vec![79, 98],
                    operation: Operation {
                        op: Operator::Times,
                        arg_a: Operand::Old,
                        arg_b: Operand::Num(19),
                    },
                    test_div: 23,
                    dst_true: 2,
                    dst_false: 3,
                }
            ))
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input(INPUT)), 10605);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_input(INPUT)), 2713310158);
    }
}
