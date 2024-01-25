use model::board::Board;

pub mod model {
    pub mod board;
    pub mod board_castle;
    pub mod board_pieces;
    pub mod color;
    pub mod r#move;
    pub mod piece;
    pub mod squares;
    pub mod types;
}

pub mod move_generator {
    pub mod bishop_moves;
    pub mod king_moves;
    pub mod knight_moves;
    pub mod pawn_moves;
    pub mod queen_moves;
    pub mod rook_moves;
    pub mod sliding_moves;
}

fn main() {
    let b = Board::from_fen(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
    );
    println!("{:?}", b.pieces.active_knights);
}
