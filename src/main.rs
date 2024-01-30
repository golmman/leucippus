use model::board::Board;

pub mod common {
    pub mod random;
}

pub mod evaluation {
    pub mod evaluate_board;
}

pub mod model {
    pub mod board;
    pub mod board_castle;
    pub mod board_evaluation;
    pub mod board_pieces;
    pub mod color;
    pub mod r#move;
    pub mod piece;
    pub mod simulation_result;
    pub mod simulation_step;
    pub mod squares;
    pub mod tree;
    pub mod tree_node;
    pub mod types;
}

pub mod move_generator {
    pub mod bishop_moves;
    pub mod check;
    pub mod king_moves;
    pub mod knight_moves;
    pub mod legal_moves;
    pub mod make_move;
    pub mod pawn_moves;
    pub mod queen_moves;
    pub mod rook_moves;
    pub mod sliding_moves;
}

pub mod search {
    pub mod backpropagate;
    pub mod expand;
    pub mod search;
    pub mod select;
    pub mod simulate;
}

fn main() {
    let b = Board::from_fen(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
    );
    println!("{:?}", b.pieces.our_knights);
}
