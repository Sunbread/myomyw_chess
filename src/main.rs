use crate::chess::{Operation, Status, Turn};

mod chess;
mod mcts;

fn main() {
    println!("Make Your Opponent Make You WIN! Chess");
    println!("version 0.2.2 made by Sunbread");
    println!();
    rules();
    println!();
    let mut ai = None;
    loop {
        println!("输入 begin 开始，ai 进入 AI 对战");
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        match line.to_lowercase().trim() {
            "begin" => {
                break;
            }
            "ai" => {
                println!("请选择AI先后手 A 先手 B 后手");
                let mut line = String::new();
                std::io::stdin().read_line(&mut line).unwrap();
                match line.to_lowercase().trim() {
                    "a" => ai = Some(Turn::A),
                    "b" => ai = Some(Turn::B),
                    _ => continue,
                }
                break;
            }
            _ => {}
        }
    }
    let ai = ai;
    let mut board = chess::Chessboard::new();
    loop {
        println!();
        println!("{}", board);
        match board.check() {
            Status::Free(turn) => {
                println!("轮到 {} 走棋", turn);
                let (r, c, op) = if matches!((ai.clone(), board.check()), (Some(turn1), Status::Free(turn2)) if turn1 == turn2) {
                    println!("AI 计算中");
                    mcts::search(&board, &ai.clone().unwrap())
                } else {
                    read_loc()
                };
                board = match board.next(r, c, op) {
                    Ok(board) => board,
                    Err(chess::Errors::OutOfBound) => {
                        println!("棋子出界！");
                        continue;
                    }
                    Err(chess::Errors::WrongTurn) => {
                        println!("请选择己方棋子！");
                        continue;
                    }
                    Err(chess::Errors::Stuck) => {
                        println!("目标有子！");
                        continue;
                    }
                };
            }
            Status::Win(turn) => {
                println!("{} 获胜", turn);
                return;
            }
        }
    }
}

/// (r, c, op)
fn read_loc() -> (i32, i32, Operation) {
    let location;
    let operation;
    loop {
        println!("输入坐标 行[1-{0}] 列[1-{0}]", chess::N);
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let subs: Vec<_> = line.trim().split_ascii_whitespace().map(String::from).collect();
        if subs.len() != 2 {
            println!("输入错误！");
            continue;
        }
        let mut loc = [0; 2];
        for (i, sub) in subs.iter().enumerate() {
            loc[i] = match sub.parse::<i32>() {
                Ok(pos) => pos,
                Err(_) => {
                    println!("输入错误！");
                    continue;
                }
            };
        }
        if loc[0] < 1 || loc[0] > chess::N as i32 || loc[1] < 1 || loc[1] > chess::N as i32 {
            println!("输入错误！");
            continue;
        }
        location = (loc[0] - 1, loc[1] - 1);
        break;
    }
    loop {
        println!("输入 U 向上 D 向下 L 向左 R 向右");
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        match line.to_lowercase().trim() {
            "u" => operation = Operation::Up,
            "d" => operation = Operation::Down,
            "l" => operation = Operation::Left,
            "r" => operation = Operation::Right,
            _ => {
                println!("输入错误！");
                continue;
            }
        }
        break;
    }
    (location.0, location.1, operation)
}

fn rules() {
    println!("规则：");
    println!("1、棋盘{0}x{0}，开局时双方有5个棋子，分别在左上和右下角", chess::N);
    println!("2、双方交替移动棋子，一次只能沿横竖方向移动一格");
    println!("3、胜负条件是【让对方翻转掉自己的所有棋子】");
    println!("4、在一个方向上一串交替的棋子被称为蛇（snake），如：ABA、ABABA");
    println!("如，ABAAB中ABA构成蛇");
    println!("5、若蛇中某颗棋子在另外一个方向也构成了蛇，那么就把这些交叉的蛇成为叉（cross），如：");
    println!("..A..");
    println!("..BAB");
    println!("..A..");
    println!("6、双方每走一步，都要重新计出棋盘上所有的蛇，根据蛇两端的棋子决定蛇翻转的趋势（tense）：");
    println!("如，ABAAB中ABA构成蛇，两端均为A，则snake具有把B翻转为A的趋势");
    println!("而ABABB中ABAB构成蛇，一端为A，一端为B，则snake没有任何趋势");
    println!("7、之后，把所有蛇组成的叉计出，若叉中所有蛇的趋势都不冲突，则按照每个蛇的趋势翻转棋子，如");
    println!("...B.      ...B.");
    println!("BABAB  ->  BBBBB");
    println!("...B.      ...B.");
    println!("...A.      ...A.");
    println!("在以下情况，蛇的趋势互相冲突，称这个叉被卡住（stuck）：");
    println!("..AB.     ");
    println!("BABAB  -\\>");
    println!("..AB.     ");
    println!("...A.     ");
    println!("8、在一次翻转操作结束后，不会再进行翻转，以免同形反复");
    println!(".A.      .A.       .A.");
    println!("BBB  ->  BAB  -\\>  BBB");
    println!(".A.      .A.       .A.");
    println!("但在下一步后：");
    println!(".A.      .A.       .A.");
    println!("BAB  ->  BBB  -\\>  BAB");
    println!(".A.      .A.       .A.");
}