use reunion::{UnionFind, UnionFindTrait};

use crate::chess::{Chess, Chessboard, N};

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
pub struct Snake {
    tense: Tense,
    axis: Axis,
    index: usize,
    slice: (usize, usize),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cross<'a> {
    elements: Vec<&'a Snake>,
}

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
pub enum Tense {
    A,
    B,
    None,
}

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
enum Axis {
    Row,
    Column,
}

impl Snake {
    pub fn tense(&self) -> &Tense {
        &self.tense
    }

    pub fn flip(&self, board: &mut Chessboard) {
        for i in self.slice.0..self.slice.1 {
            let chess = match self.axis {
                Axis::Row => &mut board.chessboard[self.index][i],
                Axis::Column => &mut board.chessboard[i][self.index],
            };
            *chess = match self.tense {
                Tense::A => Chess::A,
                Tense::B => Chess::B,
                Tense::None => *chess,
            }
        }
    }
}

impl Cross<'_> {
    pub fn snakes(&self) -> &Vec<&Snake> {
        &self.elements
    }
}

pub fn count_snake(board: &Chessboard) -> Vec<Snake> {
    let mut snakes = vec![];
    // by row
    let mut snake_rows = vec![];
    for r in 0..N {
        let mut begin: (Chess, usize);
        let mut c = 0;
        while c < N {
            if board.chessboard[r][c] != Chess::Void {
                begin = (board.chessboard[r][c], c);
                while c < N && board.chessboard[r][c] != Chess::Void {
                    if ((c - begin.1) % 2 == 0) ^ (board.chessboard[r][c] == begin.0) {
                        snake_rows.push((r, begin.0, board.chessboard[r][c - 1], begin.1, c - 1));
                        begin = (board.chessboard[r][c], c);
                    }
                    c += 1;
                }
                snake_rows.push((r, begin.0, board.chessboard[r][c - 1], begin.1, c - 1));
            } else {
                c += 1;
            }
        }
    }
    // by column
    let mut snake_columns = vec![];
    for c in 0..N {
        let mut begin: (Chess, usize);
        let mut r = 0;
        while r < N {
            if board.chessboard[r][c] != Chess::Void {
                begin = (board.chessboard[r][c], r);
                while r < N && board.chessboard[r][c] != Chess::Void {
                    if ((r - begin.1) % 2 == 0) ^ (board.chessboard[r][c] == begin.0) {
                        snake_columns.push((c, begin.0, board.chessboard[r - 1][c], begin.1, r - 1));
                        begin = (board.chessboard[r][c], r);
                    }
                    r += 1;
                }
                snake_columns.push((c, begin.0, board.chessboard[r - 1][c], begin.1, r - 1));
            } else {
                r += 1;
            }
        }
    }
    // collect result
    snakes.append(&mut collect_snakes(snake_rows, &Axis::Row));
    snakes.append(&mut collect_snakes(snake_columns, &Axis::Column));
    snakes
}

fn collect_snakes(tuples: Vec<(usize, Chess, Chess, usize, usize)>, axis: &Axis) -> Vec<Snake> {
    let mut snakes = vec![];
    for (column, b_chess, e_chess, begin, end) in tuples {
        if begin >= end {
            continue;
        }
        let tense = match (b_chess, e_chess) {
            (Chess::A, Chess::A) => Tense::A,
            (Chess::B, Chess::B) => Tense::B,
            _ => Tense::None,
        };
        snakes.push(Snake {
            tense,
            axis: match axis {
                Axis::Row => Axis::Row,
                Axis::Column => Axis::Column,
            },
            index: column,
            slice: (begin, end + 1),
        })
    }
    snakes
}

pub fn count_cross(snakes: &Vec<Snake>) -> Vec<Cross> {
    let mut uf = UnionFind::<&Snake>::new();
    let mut horizontal = vec![];
    let mut vertical = vec![];
    for snake in snakes {
        uf.union(snake, snake);
        match snake.axis {
            Axis::Row => horizontal.push(snake),
            Axis::Column => vertical.push(snake),
        }
    }
    let less;
    let more;
    if horizontal.len() < vertical.len() {
        less = horizontal;
        more = vertical;
    } else {
        less = vertical;
        more = horizontal;
    }
    let mut buckets = vec![vec![]; N];
    for snake in less {
        buckets[snake.index].push(snake);
    }
    for snake1 in more {
        let (b, e) = snake1.slice;
        for bucket in &buckets[b..e] {
            for snake2 in bucket {
                let (b, e) = snake2.slice;
                if b <= snake1.index && snake1.index < e {
                    // intersect
                    uf.union(snake1, snake2);
                }
            }
        }
    }
    let mut crosses = vec![];
    for set in uf {
        let elements = Vec::from_iter(set);
        crosses.push(Cross { elements });
    }
    crosses
}