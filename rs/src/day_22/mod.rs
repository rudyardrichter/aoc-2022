// this is so ugly

use std::{collections::HashMap, num::ParseIntError};

use itertools::Itertools;
use nom::{
    self,
    branch::alt,
    character::complete::{digit1, line_ending, one_of},
    combinator::{map, map_res},
    error::ErrorKind,
    multi::{many1, separated_list1},
    IResult,
};
use num::{Complex, Integer};

use crate::grid::Grid;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Void,
    Open,
    Wall,
}

fn parse_tile(s: &str) -> IResult<&str, Tile> {
    map_res(one_of(" .#"), |c: char| {
        Ok::<_, nom::Err<(&str, nom::error::ErrorKind)>>(match c {
            ' ' => Tile::Void,
            '.' => Tile::Open,
            '#' => Tile::Wall,
            _ => Err(nom::Err::Error((s, ErrorKind::OneOf)))?,
        })
    })(s)
}

fn parse_tile_grid(s: &str) -> IResult<&str, Grid<Tile>> {
    map(separated_list1(line_ending, many1(parse_tile)), |v| {
        let w = v.iter().map(|l| l.len()).max().unwrap().max(v.len());
        let items = (0..w)
            .cartesian_product(0..w)
            .map(|(i, j)| *v.get(i).and_then(|r| r.get(j)).unwrap_or(&Tile::Void))
            .collect();
        Grid { items, w }
    })(s)
}

#[derive(Clone, Copy, Debug)]
enum LR {
    L,
    R,
}

impl From<LR> for C {
    fn from(lr: LR) -> Self {
        match lr {
            LR::L => complex!(0, 1),
            LR::R => complex!(0, -1),
        }
    }
}

fn parse_lr(s: &str) -> IResult<&str, LR> {
    map_res(one_of("LR"), |c: char| {
        Ok::<_, nom::Err<(&str, nom::error::ErrorKind)>>(match c {
            'L' => LR::L,
            'R' => LR::R,
            _ => Err(nom::Err::Error((s, ErrorKind::OneOf)))?,
        })
    })(s)
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Go(usize),
    Turn(LR),
}

fn parse_direction(s: &str) -> IResult<&str, Direction> {
    alt((
        map(parse_lr, Direction::Turn),
        map_res(digit1, |s: &str| {
            Ok::<_, ParseIntError>(Direction::Go(s.parse::<usize>()?))
        }),
    ))(s)
}

fn parse_directions(s: &str) -> IResult<&str, Vec<Direction>> {
    many1(parse_direction)(s)
}

type C = Complex<isize>;
type FlatWrapMap = HashMap<C, HashMap<C, C>>;
type CubeWrapMap = HashMap<C, HashMap<C, (C, C)>>;

#[derive(Clone, Debug)]
pub struct State {
    map: Grid<Tile>,
    // (position, direction)
    sprite: (C, C),
    directions: Vec<Direction>,
}

impl State {
    fn c_concave_corner(&self, c: C) -> Option<C> {
        if let Some(Tile::Void) = self.get(c) {
            return None;
        }
        let p = self
            .map
            .neighbors(self.map.xy_to_i((c.re as usize, c.im as usize)))
            .iter()
            .filter(|(_, tile)| *tile != Tile::Void)
            .count()
            == 4;
        let diagonals = self
            .map
            .neighbors_diagonal(self.map.xy_to_i((c.re as usize, c.im as usize)));
        let voids: Vec<&(usize, Tile)> = diagonals
            .iter()
            .filter(|(_, tile)| *tile == Tile::Void)
            .collect();
        if p && voids.len() == 1 {
            let (y, x) = voids[0].0.div_rem(&self.map.w);
            Some(complex!(x as isize, y as isize))
        } else {
            None
        }
    }

    // TODO: both follow_directions_X functions should accept a wrap map
    fn follow_directions_flat(&mut self) -> &mut Self {
        // {position => (direction => new position)}
        let mut wrap_map: FlatWrapMap = HashMap::new();
        for i_row in (0..self.map.items.len()).step_by(self.map.w) {
            let row = &self.map.items[i_row..(i_row + self.map.w)];
            if let Some(j_l) = row.iter().position(|&tile| tile != Tile::Void) {
                let y = (i_row / self.map.w) as isize;
                let j_r = self.map.w
                    - row
                        .iter()
                        .rev()
                        .position(|&tile| tile != Tile::Void)
                        .unwrap()
                    - 1;
                wrap_map
                    .entry(complex!(j_l as isize, y))
                    .or_default()
                    .insert(complex!(-1, 0), complex!(j_r as isize, y));
                wrap_map
                    .entry(complex!(j_r as isize, y))
                    .or_default()
                    .insert(complex!(1, 0), complex!(j_l as isize, y));
            }
        }
        for i_col in 0..self.map.w {
            let col_iter = self.map.items.iter().skip(i_col).step_by(self.map.w);
            if let Some(j_u) = col_iter.clone().position(|&tile| tile != Tile::Void) {
                let j_d = self.map.w
                    - col_iter
                        .clone()
                        .rev()
                        .position(|&tile| tile != Tile::Void)
                        .unwrap()
                    - 1;
                let y_a = j_u as isize;
                let y_b = j_d as isize;
                wrap_map
                    .entry(complex!(i_col as isize, y_a))
                    .or_default()
                    .insert(complex!(0, -1), complex!(i_col as isize, y_b));
                wrap_map
                    .entry(complex!(i_col as isize, y_b))
                    .or_default()
                    .insert(complex!(0, 1), complex!(i_col as isize, y_a));
            }
        }
        for d in self.directions.iter().copied() {
            match d {
                Direction::Go(n) => {
                    'go: for _ in 0..n {
                        let dst = self.sprite.0 + self.sprite.1;
                        if dst.re < 0
                            || dst.im < 0
                            || self.map.xy_to_i((dst.re as usize, dst.im as usize))
                                >= self.map.items.len()
                        {
                            let dst_wrap = wrap_map[&self.sprite.0][&self.sprite.1];
                            self.sprite.0 = match self.get(dst_wrap).unwrap() {
                                Tile::Open => dst_wrap,
                                Tile::Wall => {
                                    // can stop this Go early
                                    break 'go;
                                }
                                // shouldn't be void if we set up wrap correctly
                                _ => unreachable!(),
                            }
                        } else {
                            self.sprite.0 = match self.get(dst).unwrap() {
                                Tile::Void => {
                                    let dst_wrap = wrap_map[&self.sprite.0][&self.sprite.1];
                                    let i_wrap = self
                                        .map
                                        .xy_to_i((dst_wrap.re as usize, dst_wrap.im as usize));
                                    match self.map.items[i_wrap] {
                                        Tile::Open => dst_wrap,
                                        Tile::Wall => {
                                            // can stop this Go early
                                            break 'go;
                                        }
                                        // shouldn't be void if we set up wrap correctly
                                        _ => unreachable!(),
                                    }
                                }
                                Tile::Open => dst,
                                Tile::Wall => self.sprite.0,
                            }
                        }
                    }
                }
                Direction::Turn(lr) => {
                    self.sprite.1 *= -<LR as std::convert::Into<C>>::into(lr);
                }
            }
        }
        self
    }

    fn get(&self, c: C) -> Option<Tile> {
        if c.re < 0
            || c.im < 0
            || self.map.xy_to_i((c.re as usize, c.im as usize)) >= self.map.items.len()
        {
            None
        } else {
            Some(self.map.items[self.map.xy_to_i((c.re as usize, c.im as usize))])
        }
    }

    // CAN'T BELIEVE THIS WORKED ON THE FIRST TRY
    fn follow_directions_cube(&mut self) -> &mut Self {
        // {position => (direction => (new position, new direction))}
        let mut wrap_map: CubeWrapMap = HashMap::new();
        for (i, j) in (0..self.map.w).cartesian_product(0..self.map.w) {
            let c = complex!(i as isize, j as isize);
            if let Some(void) = self.c_concave_corner(c) {
                let diff = void - c;
                let mut direction_a = complex!(diff.re, 0);
                let mut direction_b = complex!(0, diff.im);
                let mut direction_a_norm = direction_b;
                let mut direction_b_norm = direction_a;
                let mut a = c + direction_a;
                let mut b = c + direction_b;
                loop {
                    wrap_map
                        .entry(a)
                        .or_default()
                        .insert(direction_a_norm, (b, -direction_b_norm));
                    wrap_map
                        .entry(b)
                        .or_default()
                        .insert(direction_b_norm, (a, -direction_a_norm));
                    let mut a_turned = false;
                    let mut b_turned = false;
                    if self
                        .get(a + direction_a)
                        .map(|a| a == Tile::Void)
                        .unwrap_or(true)
                    {
                        a_turned = true;
                        if self
                            .get(a + direction_a * complex!(0, 1))
                            .map(|a| a != Tile::Void)
                            .unwrap_or(false)
                        {
                            direction_a *= complex!(0, 1);
                            direction_a_norm *= complex!(0, 1);
                        } else {
                            direction_a *= complex!(0, -1);
                            direction_a_norm *= complex!(0, -1);
                        }
                    }
                    if self
                        .get(b + direction_b)
                        .map(|b| b == Tile::Void)
                        .unwrap_or(true)
                    {
                        b_turned = true;
                        if self
                            .get(b + direction_b * complex!(0, 1))
                            .map(|b| b != Tile::Void)
                            .unwrap_or(false)
                        {
                            direction_b *= complex!(0, 1);
                            direction_b_norm *= complex!(0, 1);
                        } else {
                            direction_b *= complex!(0, -1);
                            direction_b_norm *= complex!(0, -1);
                        }
                    }
                    match (a_turned, b_turned) {
                        (true, true) => break,
                        (true, false) => b += direction_b,
                        (false, true) => a += direction_a,
                        (false, false) => {
                            a += direction_a;
                            b += direction_b;
                        }
                    }
                }
            };
        }
        for d in self.directions.iter().copied() {
            match d {
                Direction::Go(n) => {
                    'go: for _ in 0..n {
                        let dst = self.sprite.0 + self.sprite.1;
                        if dst.re < 0
                            || dst.im < 0
                            || self.map.xy_to_i((dst.re as usize, dst.im as usize))
                                >= self.map.items.len()
                        {
                            let dst = wrap_map[&self.sprite.0][&self.sprite.1];
                            match self.get(dst.0).unwrap() {
                                Tile::Open => {
                                    self.sprite = dst;
                                }
                                Tile::Wall => {
                                    // can stop this Go early
                                    break 'go;
                                }
                                // shouldn't be void if we set up wrap correctly
                                _ => unreachable!(),
                            }
                        } else {
                            match self.get(dst).unwrap() {
                                Tile::Void => {
                                    let dst = wrap_map[&self.sprite.0][&self.sprite.1];
                                    match self.get(dst.0).unwrap() {
                                        Tile::Open => {
                                            self.sprite = dst;
                                        }
                                        Tile::Wall => {
                                            // can stop this Go early
                                            break 'go;
                                        }
                                        // shouldn't be void if we set up wrap correctly
                                        _ => unreachable!(),
                                    }
                                }
                                Tile::Open => self.sprite.0 = dst,
                                Tile::Wall => break 'go,
                            }
                        }
                    }
                }
                Direction::Turn(lr) => {
                    self.sprite.1 *= -<LR as std::convert::Into<C>>::into(lr);
                }
            }
        }
        self
    }

    fn password(&self) -> isize {
        1000 * (self.sprite.0.im + 1)
            + 4 * (self.sprite.0.re + 1)
            + match (self.sprite.1.re, self.sprite.1.im) {
                (1, 0) => 0,
                (0, 1) => 1,
                (-1, 0) => 2,
                (0, -1) => 3,
                _ => panic!("invalid direction"),
            }
    }
}

impl TryFrom<&str> for State {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let (s_map, s_directions) = s.split_once("\n\n").ok_or("")?;
        let (_, map) = parse_tile_grid(s_map).map_err(|e| format!("error parsing map: {:?}", e))?;
        let (_, directions) = parse_directions(s_directions)
            .map_err(|e| format!("error parsing directions: {:?}", e))?;
        let start_c = complex!(
            map.items
                .iter()
                .position(|&t| t == Tile::Open)
                .ok_or("no open tile on first line")? as isize,
            0
        );
        let sprite = (start_c, complex!(1, 0));
        Ok(State {
            map,
            sprite,
            directions,
        })
    }
}

#[aoc_generator(day22)]
pub fn get_input(input: &str) -> State {
    input.try_into().unwrap()
}

#[aoc(day22, part1)]
pub fn part_1(state: &State) -> isize {
    state.clone().follow_directions_flat().password()
}

#[aoc(day22, part2)]
pub fn part_2(state: &State) -> isize {
    state.clone().follow_directions_cube().password()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../test_data/day_22.txt");

    #[test]
    fn test_state_impl() {
        let state = get_input(INPUT);
        assert!(state.c_concave_corner(complex!(8, 0)).is_none());
        assert!(state.c_concave_corner(complex!(8, 4)).is_some());
        assert!(state.c_concave_corner(complex!(9, 4)).is_none());
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input(INPUT)), 6032);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_input(INPUT)), 5031);
    }
}
