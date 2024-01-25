use crate::model::board::Board;
use crate::model::color::Color;
use crate::model::r#move::Move;
use crate::model::types::SquareIndex;

pub fn generate(board: &Board) -> Vec<Move> {
    let mut moves = Vec::new();

    for from in &board.pieces.active_kings {
        let from_x = (from % 8) as i8;
        let from_y = (from / 8) as i8;

        add_move(board, &mut moves, *from, from_x - 1, from_y - 1);
        add_move(board, &mut moves, *from, from_x + 0, from_y - 1);
        add_move(board, &mut moves, *from, from_x + 1, from_y - 1);
        add_move(board, &mut moves, *from, from_x - 1, from_y + 0);
        add_move(board, &mut moves, *from, from_x + 1, from_y + 0);
        add_move(board, &mut moves, *from, from_x - 1, from_y + 1);
        add_move(board, &mut moves, *from, from_x + 0, from_y + 1);
        add_move(board, &mut moves, *from, from_x + 1, from_y + 1);

        add_castles(board, &mut moves);
    }

    moves
}

fn add_move(
    board: &Board,
    moves: &mut Vec<Move>,
    from: SquareIndex,
    to_x: i8,
    to_y: i8,
) {
    if to_x < 0 || to_x > 7 || to_y < 0 || to_y > 7 {
        return;
    }

    let to = (8 * to_y + to_x) as u8;
    if let Some(piece) = board.pieces.squares.data[to as usize] {
        if piece.get_color() == board.color {
            return;
        }
    }

    moves.push(Move::from_to(from, to));
}

fn add_castles(board: &Board, moves: &mut Vec<Move>) {
    match board.color {
        Color::Black => {
            if board.castle.black_long
                && board.pieces.squares.data[57] == None
                && board.pieces.squares.data[58] == None
                && board.pieces.squares.data[59] == None
            {
                moves.push(Move::castle_long());
            }

            if board.castle.black_short
                && board.pieces.squares.data[61] == None
                && board.pieces.squares.data[62] == None
            {
                moves.push(Move::castle_short());
            }
        }
        Color::White => {
            if board.castle.white_long
                && board.pieces.squares.data[1] == None
                && board.pieces.squares.data[2] == None
                && board.pieces.squares.data[3] == None
            {
                moves.push(Move::castle_long());
            }

            if board.castle.white_short
                && board.pieces.squares.data[5] == None
                && board.pieces.squares.data[6] == None
            {
                moves.push(Move::castle_short());
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_generates_no_king_moves_from_the_starting_position() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 0);
    }

    #[test]
    fn it_generates_one_move_almost_surrounded_by_queens() {
        let fen = "8/8/8/8/8/QQQ5/QKQ5/1QQ5 w - - 0 1";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 1);
        assert_eq!(moves[0], Move::from_to(9, 0));
    }

    #[test]
    fn it_generates_8_moves_when_alone() {
        let fen = "8/8/8/8/8/8/1K6/8 w - - 0 1";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 8);
        assert_eq!(
            moves,
            vec![
                Move::from_to(9, 0),
                Move::from_to(9, 1),
                Move::from_to(9, 2),
                Move::from_to(9, 8),
                Move::from_to(9, 10),
                Move::from_to(9, 16),
                Move::from_to(9, 17),
                Move::from_to(9, 18),
            ]
        );
    }

    #[test]
    fn it_generates_8_moves_when_surrounded_by_enemies() {
        let fen = "8/8/8/8/8/qqq5/qKq5/qqq5 w - - 0 1";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 8);
        assert_eq!(
            moves,
            vec![
                Move::from_to(9, 0),
                Move::from_to(9, 1),
                Move::from_to(9, 2),
                Move::from_to(9, 8),
                Move::from_to(9, 10),
                Move::from_to(9, 16),
                Move::from_to(9, 17),
                Move::from_to(9, 18),
            ]
        );
    }

    #[test]
    fn it_generates_3_moves_when_in_a_corner() {
        let fen = "8/8/8/8/8/8/8/K7 w - - 0 1";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 3);
        assert_eq!(
            moves,
            vec![
                Move::from_to(0, 1),
                Move::from_to(0, 8),
                Move::from_to(0, 9),
            ]
        );
    }

    #[test]
    fn it_generates_moves_in_a_complex_position() {
        let fen = "8/8/8/6b1/6Nk/6P1/8/8 b - - 0 1";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 4);
        assert_eq!(
            moves,
            vec![
                Move::from_to(31, 22),
                Move::from_to(31, 23),
                Move::from_to(31, 30),
                Move::from_to(31, 39),
            ]
        );
    }

    #[test]
    fn it_generates_white_short_castles() {
        let fen =
            "rnbqkb1r/pp1pp1pp/2p2p1n/1B6/8/4PN2/PPPP1PPP/RNBQK2R w KQkq - 0 4";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 3);
        assert_eq!(
            moves,
            vec![
                Move::from_to(4, 5),
                Move::from_to(4, 12),
                Move::castle_short(),
            ]
        );
    }

    #[test]
    fn it_generates_both_white_castles() {
        let fen =
            "rnb1k2r/pp4pp/1qpp1p1n/1Bb1p3/8/BPN1PN2/P1PPQPPP/R3K2R w KQkq - 0 8";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 4);
        assert_eq!(
            moves,
            vec![
                Move::from_to(4, 3),
                Move::from_to(4, 5),
                Move::castle_long(),
                Move::castle_short(),
            ]
        );
    }

    #[test]
    fn it_generates_moves_without_castles_after_castling() {
        let fen =
            "rn2k2r/pp4pp/1qppbp1n/1Bb1p3/8/BPN1PN2/P1PPQPPP/2KR3R w kq - 2 9";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 2);
        assert_eq!(moves, vec![Move::from_to(2, 1), Move::from_to(2, 9),]);
    }

    #[test]
    fn it_generates_both_black_castles() {
        let fen = "r3k2r/pp4pp/3p1p1n/8/8/BPN1P3/P1PPQPPP/2KR3R b kq - 0 10";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 7);
        assert_eq!(
            moves,
            vec![
                Move::from_to(60, 51),
                Move::from_to(60, 52),
                Move::from_to(60, 53),
                Move::from_to(60, 59),
                Move::from_to(60, 61),
                Move::castle_long(),
                Move::castle_short(),
            ]
        );
    }
}
