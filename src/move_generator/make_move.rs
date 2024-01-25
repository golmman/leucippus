use crate::model::board::Board;
use crate::model::r#move::Move;
use crate::model::r#move::MoveSpecial;

pub fn make_move(board: &mut Board, m: &Move) {
    if m.is_castle() {
        make_castle(board, m);
    } else if m.is_en_passant() {
        make_en_passant(board, m);
    } else if m.is_promotion() {
        make_promotion(board, m);
    } else if is_capture(board, m) {
        make_capture(board, m);
    } else {
        make_simple_move(board, m);
    }
}

fn make_simple_move(board: &mut Board, m: &Move) {}

fn make_castle(board: &mut Board, m: &Move) {}

fn make_capture(board: &mut Board, m: &Move) {}

fn make_en_passant(board: &mut Board, m: &Move) {}

fn make_promotion(board: &mut Board, m: &Move) {}

fn is_capture(board: &mut Board, m: &Move) -> bool {
    true
}
