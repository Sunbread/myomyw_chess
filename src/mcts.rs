use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use mcts::{CycleBehaviour, Evaluator, GameState, MCTS, MCTSManager, MoveEvaluation, MoveList, Player, SearchHandle};
use mcts::transposition_table::{ApproxTable, TranspositionHash};
use mcts::tree_policy::UCTPolicy;

use crate::chess::{Chessboard, Operation, Status, Turn};

#[derive(Clone)]
struct ChessGame {
    board: Chessboard,
}

impl ChessGame {
    fn from(board: Chessboard) -> ChessGame {
        ChessGame { board }
    }
}

impl GameState for ChessGame {
    type Move = (i32, i32, Operation);
    type Player = Option<Turn>;
    type MoveList = Vec<(i32, i32, Operation)>;

    fn current_player(&self) -> Self::Player {
        match self.board.check() {
            Status::Win(_) => None,
            Status::Free(turn) => Some(turn),
        }
    }

    fn available_moves(&self) -> Self::MoveList {
        match self.current_player() {
            Some(_) => self.board.available(),
            None => vec![],
        }
    }

    fn make_move(&mut self, mov: &Self::Move) {
        self.board = self.board.next(mov.0, mov.1, mov.2.clone()).unwrap();
    }
}

impl TranspositionHash for ChessGame {
    fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.board.hash(&mut hasher);
        hasher.finish()
    }
}

#[derive(Clone)]
enum StateEval {
    Win(Turn),
    Eval(i32),
}

struct ChessEvaluator {
    computer: Turn,
}

impl ChessEvaluator {
    fn new(computer: &Turn) -> ChessEvaluator {
        ChessEvaluator { computer: computer.clone() }
    }
}

impl Evaluator<ChessMCTS> for ChessEvaluator {
    type StateEvaluation = StateEval;

    fn evaluate_new_state(&self, state: &ChessGame, moves: &MoveList<ChessMCTS>, _handle: Option<SearchHandle<ChessMCTS>>) -> (Vec<MoveEvaluation<ChessMCTS>>, Self::StateEvaluation) {
        match state.board.check() {
            Status::Win(who) => (vec![(); moves.len()], StateEval::Win(who)),
            Status::Free(_) => {
                let (num_a, num_b) = state.board.state();
                let (num_comp, num_player) = match self.computer {
                    Turn::A => (num_a, num_b),
                    Turn::B => (num_b, num_a),
                };
                (vec![(); moves.len()], StateEval::Eval((num_player - num_comp) + 5))
            }
        }
    }

    fn evaluate_existing_state(&self, _: &ChessGame, existing_evaln: &Self::StateEvaluation, _: SearchHandle<ChessMCTS>) -> Self::StateEvaluation {
        existing_evaln.clone()
    }

    fn interpret_evaluation_for_player(&self, evaluation: &Self::StateEvaluation, player: &Player<ChessMCTS>) -> i64 {
        let factor = if *player.as_ref().unwrap() == self.computer { 1 } else { -1 };
        let eval = match evaluation {
            StateEval::Win(_) => 1e9 as i64,
            StateEval::Eval(x) => *x as i64,
        };
        factor * eval
    }
}

#[derive(Default)]
struct ChessMCTS;

impl MCTS for ChessMCTS {
    type State = ChessGame;
    type Eval = ChessEvaluator;
    type TreePolicy = UCTPolicy;
    type NodeData = ();
    type TranspositionTable = ApproxTable<Self>;
    type ExtraThreadData = ();

    fn cycle_behaviour(&self) -> CycleBehaviour<Self> {
        CycleBehaviour::UseCurrentEvalWhenCycleDetected
    }
}

pub fn search(board: &Chessboard, computer: &Turn) -> (i32, i32, Operation) {
    let game = ChessGame::from(board.clone());
    let mut mcts = MCTSManager::new(game, ChessMCTS, ChessEvaluator::new(computer), UCTPolicy::new(2_f64.sqrt()), ApproxTable::new(1048576));
    mcts.playout_n_parallel(1e6 as u32, 16);
    mcts.best_move().unwrap()
}