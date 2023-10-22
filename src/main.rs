use crate::chess::Status;

mod chess;
mod mcts;

fn main() {
    println!("Make Your Opponent Make You WIN! Chess");
    println!("version 0.2.2 AI-mode made by Sunbread");
    println!();
    let mut board = chess::Chessboard::new();
    loop {
        println!();
        println!("{}", board);
        match board.check() {
            Status::Free(turn) => {
                println!("轮到 {} 走棋", turn);
                println!("AI 计算中");
                let (r, c, op) = mcts::search(&board, &turn);
                board = board.next(r, c, op).unwrap();
            }
            Status::Win(turn) => {
                println!("{} 获胜", turn);
                return;
            }
        }
    }
}