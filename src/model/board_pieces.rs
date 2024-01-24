use super::squares::Squares;

pub struct BoardPieces {
    pub active_bishops: Vec<u8>,
    pub active_kings: Vec<u8>,
    pub active_knights: Vec<u8>,
    pub active_pawns: Vec<u8>,
    pub active_queens: Vec<u8>,
    pub active_rooks: Vec<u8>,
    pub squares: Squares,
}
