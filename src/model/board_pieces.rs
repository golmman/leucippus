use super::squares::Squares;
use super::types::SquareIndex;

pub struct BoardPieces {
    pub active_bishops: Vec<SquareIndex>,
    pub active_kings: Vec<SquareIndex>,
    pub active_knights: Vec<SquareIndex>,
    pub active_pawns: Vec<SquareIndex>,
    pub active_queens: Vec<SquareIndex>,
    pub active_rooks: Vec<SquareIndex>,
    pub squares: Squares,
}
