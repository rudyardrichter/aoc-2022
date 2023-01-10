use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    L,
    R,
}

#[derive(Debug)]
enum Piece {
    F, // Flat
    X, // Pentomino name
    V, // Pentomino name
    I,
    Q, // Square
}

const PIECES: [Piece; 5] = [Piece::F, Piece::X, Piece::V, Piece::I, Piece::Q];

impl Piece {
    fn width(&self) -> usize {
        match self {
            Piece::F => 4,
            Piece::X => 3,
            Piece::V => 3,
            Piece::I => 1,
            Piece::Q => 2,
        }
    }

    /// NOTE that masks are ordered going up, and the bits are the reverse of the left/right
    /// position of the rock in the cave.
    fn mask(&self, x: usize) -> [u8; 4] {
        match self {
            Piece::F => [0b1111 << x, 0, 0, 0],
            Piece::X => [1 << (x + 1), 0b111 << x, 1 << (x + 1), 0],
            Piece::V => [0b111 << x, 1 << (x + 2), 1 << (x + 2), 0],
            Piece::I => [1 << x, 1 << x, 1 << x, 1 << x],
            Piece::Q => [0b11 << x, 0b11 << x, 0, 0],
        }
    }

    fn can_shift(&self, x: usize, s: Move, cave_slice: &[u8]) -> bool {
        if s == Move::L && x == 0 || s == Move::R && x + self.width() >= 7 {
            return false;
        }
        let x_new = match s {
            Move::L => x - 1,
            Move::R => x + 1,
        };
        self.mask(x_new)
            .as_slice()
            .iter()
            .zip(cave_slice.iter())
            .all(|(a, b)| a & b == 0)
    }

    /// The `y` variable is the difference in height from the bottom of the piece to the topmost
    /// rock, increasing going downwards, with 0 at one row above the stopped rocks. E.g. if `y`
    /// could hypothetically be -1, then one row would separate the new piece and the topmost rock.
    ///
    /// Thus, to figure out if the piece should stop, we check the intersection of the piece's
    /// 4-row mask with the 1-to-4-row slice of the cave starting at `y` from the "top" (end of the
    /// cave vector).
    fn stops_on(&self, x: usize, y: usize, cave: &Vec<u8>) -> bool {
        let h = cave.len();
        y == h
            || self
                .mask(x)
                .iter()
                .zip(cave[(h - y - 1)..(h - y + 3).clamp(0, h)].iter())
                .any(|(a, b)| a & b != 0)
    }
}

struct Tetris {
    rows: Vec<u8>,
}

impl Tetris {
    fn new() -> Self {
        Self { rows: vec![] }
    }

    fn do_moves(&mut self, moves: &[Move], n_rocks: usize) -> usize {
        let ms = &mut moves.iter().cycle();
        for p in PIECES.iter().cycle().take(n_rocks) {
            let l = self.rows.len();
            let mut x = ms.take(3).fold(2, |x, m| match m {
                Move::R => (7 - p.width()).min(x + 1),
                Move::L => x.saturating_sub(1),
            });
            let mut y = 0;
            loop {
                let m = ms.next().unwrap();
                if p.can_shift(
                    x,
                    *m,
                    self.rows.as_slice()[l - y..(l - y + 4).clamp(0, l)]
                        .try_into()
                        .unwrap(),
                ) {
                    x = match m {
                        Move::L => x - 1,
                        Move::R => x + 1,
                    };
                }
                if p.stops_on(x, y, &self.rows) {
                    break;
                }
                y += 1;
            }
            // If y == 0 then the piece stopped exactly on top and we append the new rows.
            // For y > 0, update the rows the piece is intersecting with.
            // TODO: this is a mess, brain is not working today
            let to_update = 4.min(y);
            let to_append = 4 - 4.min(y);
            let mask = p.mask(x);
            for (i, m) in mask.iter().enumerate().take(to_update) {
                self.rows[l - y + i] |= m;
            }
            for i in 0..to_append {
                if mask[i + to_update] != 0 {
                    self.rows.push(mask[i + to_update]);
                }
            }
        }
        self.rows.len()
    }

    fn do_moves_but_smarter(&mut self, moves: &[Move], n_rocks: usize) -> usize {
        let mut n = 0;
        let mut i_piece = 0;
        let mut i_move = 0;
        let mut seen = HashMap::new();
        let mut skipped = 0;
        let ms = &mut moves.iter().cycle();
        let pieces = &mut PIECES.iter().cycle();
        while n < n_rocks {
            let p = pieces.next().unwrap();
            i_piece += 1;
            let l = self.rows.len();
            let mut x = ms.take(3).fold(2, |x, m| match m {
                Move::R => (7 - p.width()).min(x + 1),
                Move::L => x.saturating_sub(1),
            });
            i_move += 3;
            let mut y = 0;
            loop {
                let m = ms.next().unwrap();
                i_move += 1;
                if p.can_shift(
                    x,
                    *m,
                    self.rows.as_slice()[l - y..(l - y + 4).clamp(0, l)]
                        .try_into()
                        .unwrap(),
                ) {
                    x = match m {
                        Move::L => x - 1,
                        Move::R => x + 1,
                    };
                }
                if p.stops_on(x, y, &self.rows) {
                    break;
                }
                y += 1;
            }
            let to_update = 4.min(y);
            let to_append = 4 - 4.min(y);
            let mask = p.mask(x);
            for (i, m) in mask.iter().enumerate().take(to_update) {
                self.rows[l - y + i] |= m;
            }
            for i in 0..to_append {
                if mask[i + to_update] != 0 {
                    self.rows.push(mask[i + to_update]);
                }
            }
            i_piece %= PIECES.len();
            i_move %= moves.len();
            n += 1;
            let latest = self
                .rows
                .iter()
                .rev()
                .take(16)
                .fold(0u128, |acc, r| (acc << 8) + *r as u128);
            // this input sucks
            if let Some((m, h)) = seen.get(&(i_piece, i_move, latest)) {
                let n_cycles_to_skip = (n_rocks - n) / (n - m);
                skipped += (self.rows.len() - h) * n_cycles_to_skip;
                n += (n - m) * n_cycles_to_skip;
            } else {
                seen.insert((i_piece, i_move, latest), (n, self.rows.len()));
            }
        }
        self.rows.len() + skipped
    }
}

#[aoc_generator(day17)]
pub fn get_input(input: &str) -> Vec<Move> {
    input
        .bytes()
        .map(|b| if b == b'>' { Move::R } else { Move::L })
        .collect()
}

#[aoc(day17, part1)]
pub fn part_1(moves: &[Move]) -> usize {
    Tetris::new().do_moves(moves, 2022)
}

#[aoc(day17, part2)]
pub fn part_2(moves: &[Move]) -> usize {
    Tetris::new().do_moves_but_smarter(moves, 1000000000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input(INPUT)), 3068);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_input(INPUT)), 1514285714288);
    }
}
