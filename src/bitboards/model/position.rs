use crate::bitboards::model::bitboard::Bitboard;
use crate::model::board_castle::BoardCastle;
use crate::model::color::Color;
use crate::model::piece::Piece;
use crate::model::types::SquareIndex;

pub struct Position {
    pub board: [Piece; 64],
    pub castling: BoardCastle,
    pub draw_by_repetition: bool,
    pub en_passant: Option<SquareIndex>,
    pub fullmove: usize,
    pub halfmove: usize,
    pub our_color: Color,
    pub pieces_by_color: [Bitboard; 2],
    pub pieces_by_type: [Bitboard; 6],
}
