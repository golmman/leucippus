use crate::model::board::Board;
use crate::model::board_evaluation::BoardEvaluation;
use crate::model::color::Color;

/// Draws:
/// * insufficient material
///     * kk
///     * kkb
///     * kkn
/// * repetition
/// * 50 move rule
/// * stalemate
///
/// Wins/Losses:
/// * normal Checkmate
/// * exploded kings
///
/// Inconclusive:
/// * everything else
pub fn evaluate_board(board: &Board) -> BoardEvaluation {
    if is_draw(board) {
        return BoardEvaluation::Draw;
    }

    if is_win(board) {
        if board.our_color == Color::Black {
            return BoardEvaluation::WinBlack;
        } else {
            return BoardEvaluation::WinWhite;
        }
    }

    BoardEvaluation::Inconclusive
}

fn is_draw(board: &Board) -> bool {
    if board.draw_by_repetition {
        // TODO: implement logic behind this like e.g. so:
        // * put board hashes on the move list
        // * after making a move check the number of boards in that list
        // * if 3 then set the flag
        return true;
    }

    if is_insufficient_material(board) {
        return true;
    }

    false
}

fn is_insufficient_material(board: &Board) -> bool {
    if 0 == board.pieces.our_pawns.len()
        + board.pieces.our_queens.len()
        + board.pieces.our_rooks.len()
    {
        let our_minor_piece_sum =
            board.pieces.our_bishops.len() + board.pieces.our_knights.len();

        if our_minor_piece_sum <= 1 {
            let mut total_pieces = 0;
            let mut total_minor_pieces = 0;
            for i in 0..64 {
                if let Some(piece) = board.pieces.squares.data[i] {
                    total_pieces += 1;
                    if piece.is_bishop() || piece.is_knight() {
                        total_minor_pieces += 1;
                    }
                }
            }

            if total_pieces <= 3 && total_minor_pieces <= 1 {
                return true;
            }
        }
    }

    false
}

fn is_win(board: &Board) -> bool {
    false
}

#[cfg(test)]
mod test {
    use super::*;

    mod draw {
        use super::*;

        #[test]
        fn it_evaluates_a_board_flagged_with_repetition_as_draw() {
            let mut board = Board::new();
            board.draw_by_repetition = true;
            assert_eq!(evaluate_board(&board), BoardEvaluation::Draw);
        }

        mod insufficient_material {
            use super::*;

            #[test]
            fn it_evaluates_a_board_with_only_two_kings_as_draw_with_black_to_move(
            ) {
                let board = Board::from_fen("8/2k5/8/8/8/8/3K4/8 b - - 0 1");
                assert_eq!(evaluate_board(&board), BoardEvaluation::Draw);
            }

            #[test]
            fn it_evaluates_a_board_with_only_two_kings_as_draw_with_white_to_move(
            ) {
                let board = Board::from_fen("8/2k5/8/8/8/8/3K4/8 w - - 0 1");
                assert_eq!(evaluate_board(&board), BoardEvaluation::Draw);
            }

            #[test]
            fn it_evaluates_a_board_with_one_black_bishop_as_draw_with_black_to_move(
            ) {
                let board = Board::from_fen("8/2k2b2/8/8/8/8/3K4/8 b - - 0 1");
                assert_eq!(evaluate_board(&board), BoardEvaluation::Draw);
            }

            #[test]
            fn it_evaluates_a_board_with_one_white_bishop_as_draw_with_black_to_move(
            ) {
                let board = Board::from_fen("8/2k2B2/8/8/8/8/3K4/8 b - - 0 1");
                assert_eq!(evaluate_board(&board), BoardEvaluation::Draw);
            }

            #[test]
            fn it_evaluates_a_board_with_one_black_bishop_as_draw_with_white_to_move(
            ) {
                let board = Board::from_fen("8/2k2b2/8/8/8/8/3K4/8 w - - 0 1");
                assert_eq!(evaluate_board(&board), BoardEvaluation::Draw);
            }

            #[test]
            fn it_evaluates_a_board_with_one_white_bishop_as_draw_with_white_to_move(
            ) {
                let board = Board::from_fen("8/2k2B2/8/8/8/8/3K4/8 w - - 0 1");
                assert_eq!(evaluate_board(&board), BoardEvaluation::Draw);
            }

            #[test]
            fn it_evaluates_a_board_with_one_black_knight_as_draw_with_black_to_move(
            ) {
                let board = Board::from_fen("8/2k2n2/8/8/8/8/3K4/8 b - - 0 1");
                assert_eq!(evaluate_board(&board), BoardEvaluation::Draw);
            }

            #[test]
            fn it_evaluates_a_board_with_one_white_knight_as_draw_with_black_to_move(
            ) {
                let board = Board::from_fen("8/2k2N2/8/8/8/8/3K4/8 b - - 0 1");
                assert_eq!(evaluate_board(&board), BoardEvaluation::Draw);
            }

            #[test]
            fn it_evaluates_a_board_with_one_black_knight_as_draw_with_white_to_move(
            ) {
                let board = Board::from_fen("8/2k2n2/8/8/8/8/3K4/8 w - - 0 1");
                assert_eq!(evaluate_board(&board), BoardEvaluation::Draw);
            }

            #[test]
            fn it_evaluates_a_board_with_one_white_knight_as_draw_with_white_to_move(
            ) {
                let board = Board::from_fen("8/2k2N2/8/8/8/8/3K4/8 w - - 0 1");
                assert_eq!(evaluate_board(&board), BoardEvaluation::Draw);
            }
        }
    }

    mod inconclusive {
        use super::*;

        #[test]
        fn it_evaluates_the_starting_position_as_inconclusive() {
            let board = Board::new();
            assert_eq!(evaluate_board(&board), BoardEvaluation::Inconclusive);
        }

        #[test]
        fn it_evaluates_a_board_with_only_one_rook_as_inconclusive() {
            let board = Board::from_fen("8/2k5/8/8/5R2/8/3K4/8 w - - 0 1");
            assert_eq!(evaluate_board(&board), BoardEvaluation::Inconclusive);
        }
    }

    mod wins {
        use super::*;
    }
}
