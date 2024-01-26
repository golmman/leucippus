use crate::model::board::Board;
use crate::model::color::Color;
use crate::model::piece::Piece;
use crate::model::r#move::Move;
use crate::model::r#move::MoveSpecial;
use crate::model::types::SquareIndex;

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
    board.pieces.squares.data[m.from as usize] = None;
    board.pieces.squares.data[m.to as usize] = piece;
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
    board.pieces.squares.data[m.to as usize]
        .as_ref()
        .map(Piece::get_color)
        .is_some_and(|c| c != board.color)
}

fn is_pawn_move(board: &mut Board, m: &Move) -> bool {
    board.pieces.squares.data[m.to as usize]
        .as_ref()
        .map_or(false, Piece::is_pawn)
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

    // active pieces
    board.pieces.active_bishops.clear();
    board.pieces.active_kings.clear();
    board.pieces.active_knights.clear();
    board.pieces.active_pawns.clear();
    board.pieces.active_queens.clear();
    board.pieces.active_rooks.clear();

    for i in 0..64 {
        if let Some(piece) = board.pieces.squares.data[i] {
            if piece.is_bishop_of_color(board.color) {
                board.pieces.active_bishops.push(i as SquareIndex);
            } else if piece.is_king_of_color(board.color) {
                board.pieces.active_kings.push(i as SquareIndex);
            } else if piece.is_knight_of_color(board.color) {
                board.pieces.active_knights.push(i as SquareIndex);
            } else if piece.is_pawn_of_color(board.color) {
                board.pieces.active_pawns.push(i as SquareIndex);
            } else if piece.is_queen_of_color(board.color) {
                board.pieces.active_queens.push(i as SquareIndex);
            } else if piece.is_rook_of_color(board.color) {
                board.pieces.active_rooks.push(i as SquareIndex);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::model::types::square_names::*;

    mod simple_moves {
        use super::*;

        #[test]
        fn it_makes_a_knight_move_from_the_starting_position() {
            let fen =
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
            let mut board = Board::from_fen(fen);
            let m = Move::from_to(G1, F3);

            make_move(&mut board, &m);
        }
    }
}
