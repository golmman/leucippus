use crate::model::board::Board;
use crate::model::types::SQUARES_TOUCH;

use super::legal_moves;

pub fn is_check(board: &mut Board) -> bool {
    let our_king_index = board.pieces.active_kings[0];

    board.swap_color();
    let their_king_index = board.pieces.active_kings[0];

    if SQUARES_TOUCH[our_king_index as usize][their_king_index as usize] == 1 {
        board.swap_color();
        return false;
    }

    let moves = legal_moves::generate_pseudo_legal(board);
    for m in moves {
        if m.to == our_king_index {
            board.swap_color();
            return true;
        }
    }
    board.swap_color();

    false
}
