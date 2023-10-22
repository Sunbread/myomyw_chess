use std::fmt;
use std::fmt::{Display, Formatter};

use crate::chess::rules::{count_cross, count_snake, Tense};

pub mod rules;

pub const N: usize = 6;

#[derive(Hash, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chess {
    A,
    B,
    Void,
}

#[derive(Hash, Clone, Debug)]
pub struct Chessboard {
    chessboard: [[Chess; N]; N],
    next_turn: Turn,
}

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
pub enum Turn {
    A,
    B,
}

#[derive(Clone, Debug)]
pub enum Operation {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug)]
pub enum Status {
    Win(Turn),
    Free(Turn),
}

#[derive(Debug)]
pub enum Errors {
    OutOfBound,
    WrongTurn,
    Stuck,
}

impl Chess {
    fn check(&self, turn: &Turn) -> bool {
        *self == match turn {
            Turn::A => Chess::A,
            Turn::B => Chess::B,
        }
    }
}

impl Display for Chessboard {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for (i, row) in self.chessboard.iter().enumerate() {
            if i != 0 {
                writeln!(f)?;
            }
            for (j, chess) in row.iter().enumerate() {
                if j != 0 {
                    write!(f, " ")?;
                }
                write!(f, "{}", match chess {
                    Chess::A => "A",
                    Chess::B => "B",
                    Chess::Void => ".",
                })?;
            }
        };
        Ok(())
    }
}

impl Chessboard {
    pub fn new() -> Chessboard {
        let mut board = [[Chess::Void; N]; N];
        // place chess
        board[2][0] = Chess::A;
        board[1][0] = Chess::A;
        board[0][0] = Chess::A;
        board[0][1] = Chess::A;
        board[0][2] = Chess::A;
        board[N - 3][N - 1] = Chess::B;
        board[N - 2][N - 1] = Chess::B;
        board[N - 1][N - 1] = Chess::B;
        board[N - 1][N - 2] = Chess::B;
        board[N - 1][N - 3] = Chess::B;
        Chessboard {
            chessboard: board,
            next_turn: Turn::A,
        }
    }
    pub fn next(&self, r: i32, c: i32, op: Operation) -> Result<Chessboard, Errors> {
        let dest = match op {
            Operation::Up => (r - 1, c),
            Operation::Down => (r + 1, c),
            Operation::Left => (r, c - 1),
            Operation::Right => (r, c + 1),
        };
        if !(0..N as i32).contains(&r) || !(0..N as i32).contains(&c) || !(0..N as i32).contains(&dest.0) || !(0..N as i32).contains(&dest.1) {
            return Err(Errors::OutOfBound);
        };
        let orig = (r as usize, c as usize);
        let dest = (dest.0 as usize, dest.1 as usize);
        if !self.chessboard[orig.0][orig.1].check(&self.next_turn) {
            return Err(Errors::WrongTurn);
        };
        if self.chessboard[dest.0][dest.1] != Chess::Void {
            return Err(Errors::Stuck);
        };
        let mut board = self.clone();
        board.chessboard[orig.0][orig.1] = Chess::Void;
        board.chessboard[dest.0][dest.1] = match self.next_turn {
            Turn::A => Chess::A,
            Turn::B => Chess::B,
        };
        board.next_turn = match self.next_turn {
            Turn::A => Turn::B,
            Turn::B => Turn::A,
        };
        Ok(flip(board))
    }
    pub fn check(&self) -> Status {
        let mut count = (0, 0);
        for row in self.chessboard {
            for chess in row {
                match chess {
                    Chess::A => count.0 += 1,
                    Chess::B => count.1 += 1,
                    Chess::Void => (),
                }
            }
        };
        match count {
            (0, x) if x > 0 => Status::Win(Turn::A),
            (x, 0) if x > 0 => Status::Win(Turn::B),
            _ => Status::Free(self.next_turn.clone()),
        }
    }
    pub fn available(&self) -> Vec<(i32, i32, Operation)> {
        let mut result = Vec::new();
        for r in 0..N {
            for c in 0..N {
                if self.chessboard[r][c] != match self.next_turn {
                    Turn::A => Chess::A,
                    Turn::B => Chess::B,
                } {
                    continue;
                }
                if (1..N).contains(&r) && self.chessboard[r - 1][c] == Chess::Void {
                    result.push((r as i32, c as i32, Operation::Up));
                }
                if (0..N - 1).contains(&r) && self.chessboard[r + 1][c] == Chess::Void {
                    result.push((r as i32, c as i32, Operation::Down));
                }
                if (1..N).contains(&c) && self.chessboard[r][c - 1] == Chess::Void {
                    result.push((r as i32, c as i32, Operation::Left));
                }
                if (0..N - 1).contains(&c) && self.chessboard[r][c + 1] == Chess::Void {
                    result.push((r as i32, c as i32, Operation::Right));
                }
            }
        }
        result
    }
    pub fn state(&self) -> (i32, i32) {
        let mut num_a = 0;
        let mut num_b = 0;
        for row in self.chessboard {
            for chess in row {
                match chess {
                    Chess::A => num_a += 1,
                    Chess::B => num_b += 1,
                    Chess::Void => (),
                }
            }
        }
        (num_a, num_b)
    }
}

impl Display for Turn {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Turn::A => "A",
            Turn::B => "B",
        })
    }
}

fn flip(mut board: Chessboard) -> Chessboard {
    // count snakes
    let snakes = count_snake(&board);
    // count crosses
    let crosses = count_cross(&snakes);
    'cross_loop: for cross in crosses {
        let mut tense_cross = Tense::None;
        for snake in cross.snakes() {
            let tense_snake = snake.tense();
            if tense_cross == Tense::None {
                tense_cross = tense_snake.clone();
            } else if *tense_snake != Tense::None && tense_cross != *tense_snake {
                continue 'cross_loop;
            }
        }
        for snake in cross.snakes().iter().filter(|s| *s.tense() != Tense::None) {
            snake.flip(&mut board);
        }
    }
    board
}