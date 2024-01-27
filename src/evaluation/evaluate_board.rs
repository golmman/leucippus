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
        if board.color == Color::Black {
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
        fn it_evaluates_the_starting_position_as_inconclusive() {
            let mut board = Board::new();
            board.draw_by_repetition = true;
            assert_eq!(evaluate_board(&board), BoardEvaluation::Draw);
        }
    }

    mod inconclusive {
        use super::*;

        #[test]
        fn it_evaluates_the_starting_position_as_inconclusive() {
            let board = Board::new();
            assert_eq!(evaluate_board(&board), BoardEvaluation::Inconclusive);
        }
    }

    mod wins {
        use super::*;
    }
}
