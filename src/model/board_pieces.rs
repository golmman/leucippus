use super::squares::Squares;

pub struct BoardPieces {
    pub active_bishops: Vec<usize>,
    pub active_kings: Vec<usize>,
    pub active_knights: Vec<usize>,
    pub active_pawns: Vec<usize>,
    pub active_queens: Vec<usize>,
    pub active_rooks: Vec<usize>,
    pub squares: Squares,
}
