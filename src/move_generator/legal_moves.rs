use crate::model::board::Board;
use crate::model::r#move::Move;

use super::bishop_moves;
use super::king_moves;
use super::knight_moves;
use super::pawn_moves;
use super::queen_moves;
use super::rook_moves;

pub fn generate_pseudo_legal(board: &mut Board) -> Vec<Move> {
    let mut moves = Vec::new();

    moves.append(&mut bishop_moves::generate(board));
    moves.append(&mut king_moves::generate(board));
    moves.append(&mut knight_moves::generate(board));
    moves.append(&mut pawn_moves::generate(board));
    moves.append(&mut queen_moves::generate(board));
    moves.append(&mut rook_moves::generate(board));

    moves
}

pub fn generate(board: &mut Board) -> Vec<Move> {
    let mut moves = generate_pseudo_legal(board);
    moves
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_generates_all_legal_moves_from_the_starting_position() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let mut board = Board::from_fen(fen);
        let moves = generate(&mut board);
        assert_eq!(moves.len(), 20);
        assert_eq!(
            moves,
            vec![
                Move::from_to(1, 16),
                Move::from_to(1, 18),
                Move::from_to(6, 21),
                Move::from_to(6, 23),
                Move::from_to(8, 16),
                Move::from_to(8, 24),
                Move::from_to(9, 17),
                Move::from_to(9, 25),
                Move::from_to(10, 18),
                Move::from_to(10, 26),
                Move::from_to(11, 19),
                Move::from_to(11, 27),
                Move::from_to(12, 20),
                Move::from_to(12, 28),
                Move::from_to(13, 21),
                Move::from_to(13, 29),
                Move::from_to(14, 22),
                Move::from_to(14, 30),
                Move::from_to(15, 23),
                Move::from_to(15, 31),
            ]
        );
    }
}
