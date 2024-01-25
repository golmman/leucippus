use crate::model::board::Board;
use crate::model::color::Color;
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

    update_board_state(board, m);
}

fn make_simple_move(board: &mut Board, m: &Move) {
    let piece = board.pieces.squares.data[m.from as usize];
    todo!()
}

fn make_castle(board: &mut Board, m: &Move) {
    todo!()
}

fn make_capture(board: &mut Board, m: &Move) {
    todo!()
}

fn make_en_passant(board: &mut Board, m: &Move) {
    todo!()
}

fn make_promotion(board: &mut Board, m: &Move) {
    todo!()
}

fn is_capture(board: &mut Board, m: &Move) -> bool {
    todo!()
}

fn is_pawn_move(board: &mut Board, m: &Move) -> bool {
    todo!()
}

fn update_board_state(board: &mut Board, m: &Move) {
    let capture = is_capture(board, m);
    let pawn_move = is_pawn_move(board, m);

    // color and fullmove
    if board.color == Color::Black {
        board.color = Color::White;
        board.fullmove += 1;
    } else {
        board.color = Color::Black;
    }

    // halfmove
    if capture || pawn_move {
        board.halfmove = 0;
    } else {
        board.halfmove += 1;
    }

    // en_passant
    if pawn_move && (m.from as i8 - m.to as i8).abs() == 16 {
        board.en_passant = Some((m.from + m.to) / 2);
    } else {
        board.en_passant = None;
    }

    // castle
    if m.from == 0 || m.to == 0 {
        board.castle.white_long = false;
    } else if m.from == 4 {
        board.castle.white_long = false;
        board.castle.white_short = false;
    } else if m.from == 7 || m.to == 7 {
        board.castle.white_short = false;
    } else if m.from == 56 || m.to == 56 {
        board.castle.black_long = false;
    } else if m.from == 60 {
        board.castle.black_long = false;
        board.castle.black_short = false;
    } else if m.from == 63 || m.to == 63 {
        board.castle.black_short = false;
    }
}
