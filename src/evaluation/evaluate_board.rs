use crate::common::random::Random;
use crate::model::board::Board;
use crate::model::board_evaluation::BoardEvaluation;
use crate::model::board_evaluation_result::BoardEvaluationResult;
use crate::model::color::Color;
use crate::move_generator::check::is_check;
use crate::move_generator::legal_moves::generate_move;

/// Draws:
/// * insufficient material
///     * kk
///     * kkb
///     * kkn
///     * kkr
///     * kknn
/// * repetition
/// * 50 move rule
/// * stalemate
///
/// Wins/Losses:
/// * checkmate
/// * exploded kings
///
/// Inconclusive:
/// * everything else
pub fn evaluate_board(
    board: &mut Board,
    random: &mut Random,
) -> BoardEvaluationResult {
    if is_simple_win(board) {
        return BoardEvaluationResult {
            evaluation: make_win_by_board_color(board),
            random_move: None,
        };
    }

    if is_simple_draw(board) {
        return BoardEvaluationResult {
            evaluation: BoardEvaluation::Draw,
            random_move: None,
        };
    }

    check_stalemate_or_checkmate(board, random)
}

fn check_stalemate_or_checkmate(
    board: &mut Board,
    random: &mut Random,
) -> BoardEvaluationResult {
    let random_move = generate_move(board, random);
    let no_moves = random_move.is_none();
    if no_moves == false {
        return BoardEvaluationResult {
            evaluation: BoardEvaluation::Inconclusive,
            random_move,
        };
    }

    let in_check = is_check(board);

    if no_moves && in_check {
        return BoardEvaluationResult {
            evaluation: make_win_by_board_color(board),
            random_move,
        };
    }

    if no_moves && !in_check {
        return BoardEvaluationResult {
            evaluation: BoardEvaluation::Draw,
            random_move,
        };
    }

    BoardEvaluationResult {
        evaluation: BoardEvaluation::Inconclusive,
        random_move,
    }
}

fn make_win_by_board_color(board: &Board) -> BoardEvaluation {
    if board.our_color == Color::Black {
        return BoardEvaluation::WinWhite;
    } else {
        return BoardEvaluation::WinBlack;
    }
}

fn is_simple_draw(board: &Board) -> bool {
    board.draw_by_repetition
        || is_insufficient_material(board)
        || is_draw_by_50_moves_rule(board)
}

fn is_draw_by_50_moves_rule(board: &Board) -> bool {
    board.halfmove == 100
}

fn is_insufficient_material(board_with_2_kings: &Board) -> bool {
    // TODO: this function assumes that the board has 2 kings on the board
    if board_with_2_kings.pieces.our_pawns.len()
        + board_with_2_kings.pieces.our_queens.len()
        > 0
    {
        return false;
    }

    if board_with_2_kings.pieces.our_bishops.len()
        + board_with_2_kings.pieces.our_knights.len()
        + board_with_2_kings.pieces.our_rooks.len()
        > 1
    {
        return false;
    }

    let mut total_pieces = 0;
    let mut total_nbr_pieces = 0;
    for i in 0..64 {
        if let Some(piece) = board_with_2_kings.pieces.squares.data[i] {
            total_pieces += 1;
            if piece.is_bishop() || piece.is_knight() || piece.is_rook() {
                total_nbr_pieces += 1;
            }
        }
    }

    if total_pieces <= 3 && total_nbr_pieces <= 1 {
        return true;
    }

    false
}

fn is_simple_win(board: &Board) -> bool {
    board.pieces.our_kings.is_empty()
}

#[cfg(test)]
mod test {
    use crate::common::random::Random;
    use crate::model::board::Board;
    use crate::model::board_evaluation::BoardEvaluation;

    fn evaluate_board(board: &mut Board) -> BoardEvaluation {
        let mut random = Random::from_seed(777);
        super::evaluate_board(board, &mut random).evaluation
    }

    #[test]
    fn it_evaluates_a_board_but_does_not_change_it() {
        let mut left = Board::new();
        let right = Board::new();
        assert_eq!(evaluate_board(&mut left), BoardEvaluation::Inconclusive);
        assert_eq!(left, right);
    }

    #[test]
    fn it_evaluates_a_board_and_generates_moves() {
        let mut board = Board::new();
        let mut random = Random::from_seed(777);
        let random_move =
            super::evaluate_board(&mut board, &mut random).random_move;
        assert!(random_move.is_some());
    }

    #[test]
    fn it_evaluates_a_board_and_generates_no_moves_for_a_simple_win_condition()
    {
        let mut board = Board::from_fen("8/8/8/8/8/8/1K1N4/8 b - - 0 1");
        let mut random = Random::from_seed(777);
        let random_move =
            super::evaluate_board(&mut board, &mut random).random_move;
        assert!(random_move.is_none());
    }

    mod draw {
        use super::*;

        #[test]
        fn it_evaluates_a_board_flagged_with_repetition_as_draw() {
            let mut board = Board::new();
            board.draw_by_repetition = true;
            assert_eq!(evaluate_board(&mut board), BoardEvaluation::Draw);
        }

        #[test]
        fn it_evaluates_a_board_with_100_halfmoves_as_draw() {
            let mut board =
                Board::from_fen("4k3/4n2p/1n5P/r7/p7/8/K7/8 w - - 100 81");
            assert_eq!(evaluate_board(&mut board), BoardEvaluation::Draw);
        }

        mod stalemate {
            use super::*;

            #[test]
            fn it_evaluates_a_black_stalemate_as_draw() {
                let mut board =
                    Board::from_fen("7k/6R1/8/8/8/6P1/8/5K2 b - - 0 31");
                assert_eq!(evaluate_board(&mut board), BoardEvaluation::Draw);
            }

            #[test]
            fn it_evaluates_a_white_stalemate_as_draw() {
                let mut board =
                    Board::from_fen("4n2K/7b/8/1k6/6p1/5pP1/5P2/7N w - - 0 1");
                assert_eq!(evaluate_board(&mut board), BoardEvaluation::Draw);
            }
        }

        mod insufficient_material {
            use super::*;

            #[test]
            fn it_evaluates_a_board_with_only_two_kings_as_draw_with_black_to_move(
            ) {
                let mut board =
                    Board::from_fen("8/2k5/8/8/8/8/3K4/8 b - - 0 1");
                assert_eq!(evaluate_board(&mut board), BoardEvaluation::Draw);
            }

            #[test]
            fn it_evaluates_a_board_with_only_two_kings_as_draw_with_white_to_move(
            ) {
                let mut board =
                    Board::from_fen("8/2k5/8/8/8/8/3K4/8 w - - 0 1");
                assert_eq!(evaluate_board(&mut board), BoardEvaluation::Draw);
            }

            #[test]
            fn it_evaluates_a_board_with_one_black_bishop_as_draw_with_black_to_move(
            ) {
                let mut board =
                    Board::from_fen("8/2k2b2/8/8/8/8/3K4/8 b - - 0 1");
                assert_eq!(evaluate_board(&mut board), BoardEvaluation::Draw);
            }

            #[test]
            fn it_evaluates_a_board_with_one_white_bishop_as_draw_with_black_to_move(
            ) {
                let mut board =
                    Board::from_fen("8/2k2B2/8/8/8/8/3K4/8 b - - 0 1");
                assert_eq!(evaluate_board(&mut board), BoardEvaluation::Draw);
            }

            #[test]
            fn it_evaluates_a_board_with_one_black_bishop_as_draw_with_white_to_move(
            ) {
                let mut board =
                    Board::from_fen("8/2k2b2/8/8/8/8/3K4/8 w - - 0 1");
                assert_eq!(evaluate_board(&mut board), BoardEvaluation::Draw);
            }

            #[test]
            fn it_evaluates_a_board_with_one_white_bishop_as_draw_with_white_to_move(
            ) {
                let mut board =
                    Board::from_fen("8/2k2B2/8/8/8/8/3K4/8 w - - 0 1");
                assert_eq!(evaluate_board(&mut board), BoardEvaluation::Draw);
            }

            #[test]
            fn it_evaluates_a_board_with_one_black_knight_as_draw_with_black_to_move(
            ) {
                let mut board =
                    Board::from_fen("8/2k2n2/8/8/8/8/3K4/8 b - - 0 1");
                assert_eq!(evaluate_board(&mut board), BoardEvaluation::Draw);
            }

            #[test]
            fn it_evaluates_a_board_with_one_white_knight_as_draw_with_black_to_move(
            ) {
                let mut board =
                    Board::from_fen("8/2k2N2/8/8/8/8/3K4/8 b - - 0 1");
                assert_eq!(evaluate_board(&mut board), BoardEvaluation::Draw);
            }

            #[test]
            fn it_evaluates_a_board_with_one_black_knight_as_draw_with_white_to_move(
            ) {
                let mut board =
                    Board::from_fen("8/2k2n2/8/8/8/8/3K4/8 w - - 0 1");
                assert_eq!(evaluate_board(&mut board), BoardEvaluation::Draw);
            }

            #[test]
            fn it_evaluates_a_board_with_one_white_knight_as_draw_with_white_to_move(
            ) {
                let mut board =
                    Board::from_fen("8/2k2N2/8/8/8/8/3K4/8 w - - 0 1");
                assert_eq!(evaluate_board(&mut board), BoardEvaluation::Draw);
            }
            #[test]
            fn it_evaluates_a_board_with_one_black_rook_as_draw_with_black_to_move(
            ) {
                let mut board =
                    Board::from_fen("8/2k2r2/8/8/8/8/3K4/8 b - - 0 1");
                assert_eq!(evaluate_board(&mut board), BoardEvaluation::Draw);
            }

            #[test]
            fn it_evaluates_a_board_with_one_white_rook_as_draw_with_black_to_move(
            ) {
                let mut board =
                    Board::from_fen("8/2k2R2/8/8/8/8/3K4/8 b - - 0 1");
                assert_eq!(evaluate_board(&mut board), BoardEvaluation::Draw);
            }

            #[test]
            fn it_evaluates_a_board_with_one_black_rook_as_draw_with_white_to_move(
            ) {
                let mut board =
                    Board::from_fen("8/2k2r2/8/8/8/8/3K4/8 w - - 0 1");
                assert_eq!(evaluate_board(&mut board), BoardEvaluation::Draw);
            }

            #[test]
            fn it_evaluates_a_board_with_one_white_rook_as_draw_with_white_to_move(
            ) {
                let mut board =
                    Board::from_fen("8/2k2R2/8/8/8/8/3K4/8 w - - 0 1");
                assert_eq!(evaluate_board(&mut board), BoardEvaluation::Draw);
            }
        }
    }

    mod inconclusive {
        use super::*;

        #[test]
        fn it_evaluates_the_starting_position_as_inconclusive() {
            let mut board = Board::new();
            assert_eq!(
                evaluate_board(&mut board),
                BoardEvaluation::Inconclusive
            );
        }

        #[test]
        fn it_evaluates_a_board_with_only_one_pawn_as_inconclusive() {
            let mut board = Board::from_fen("8/2k5/8/8/5P2/8/3K4/8 w - - 0 1");
            assert_eq!(
                evaluate_board(&mut board),
                BoardEvaluation::Inconclusive
            );
        }

        #[test]
        fn it_evaluates_a_board_with_99_halfmoves_as_inconclusive() {
            let mut board =
                Board::from_fen("5k2/4n2p/1n5P/r7/p7/8/K7/8 b - - 99 80");
            assert_eq!(
                evaluate_board(&mut board),
                BoardEvaluation::Inconclusive
            );
        }

        #[test]
        fn it_evaluates_a_classic_checkmate_as_inconclusive_because_a_king_touch_move_is_available(
        ) {
            let mut board =
                Board::from_fen("8/8/8/8/6N1/k7/6r1/2K2r2 w - - 0 1");
            assert_eq!(
                evaluate_board(&mut board),
                BoardEvaluation::Inconclusive
            );
        }

        #[test]
        fn it_evaluates_a_classic_checkmate_as_inconclusive_because_their_king_can_be_exploded(
        ) {
            let mut board =
                Board::from_fen("5k2/4p3/8/3N4/8/2n5/r7/K7 w - - 0 1");
            assert_eq!(
                evaluate_board(&mut board),
                BoardEvaluation::Inconclusive
            );
        }
    }

    mod wins {
        use super::*;

        #[test]
        fn it_evaluates_a_checkmated_black_king_as_win_for_white() {
            let mut board = Board::from_fen(
                "R7/kB1r4/1N2p3/p2p3p/8/B2PP3/P4P2/4K3 b - - 3 25",
            );
            assert_eq!(evaluate_board(&mut board), BoardEvaluation::WinWhite);
        }

        #[test]
        fn it_evaluates_a_checkmated_white_king_as_win_for_black() {
            let mut board =
                Board::from_fen("2k5/8/8/8/6N1/8/6r1/2K2r2 w - - 0 1");
            assert_eq!(evaluate_board(&mut board), BoardEvaluation::WinBlack);
        }

        #[test]
        fn it_evaluates_a_peculiarly_checkmated_white_king_as_win_for_black() {
            let mut board =
                Board::from_fen("5k2/8/8/4N3/8/2n5/1b6/K7 w - - 0 1");
            assert_eq!(evaluate_board(&mut board), BoardEvaluation::WinBlack);
        }

        #[test]
        fn it_evaluates_an_exploded_black_king_as_a_win_for_white() {
            let mut board = Board::from_fen(
                "r4bnr/ppppp1pp/5p2/8/3n4/8/PPPPPPPP/RNBQKB1R b KQkq - 0 4",
            );
            assert_eq!(evaluate_board(&mut board), BoardEvaluation::WinWhite);
        }

        #[test]
        fn it_evaluates_an_exploded_white_king_as_a_win_for_black() {
            let mut board =
                Board::from_fen("rnbqkb1r/pp1pp1pp/2p2p1B/8/3P4/2N4P/PPP1P1P1/R2Q3R w KQkq - 0 6");
            assert_eq!(evaluate_board(&mut board), BoardEvaluation::WinBlack);
        }

        #[test]
        fn it_evaluates_an_exploded_black_king_with_insufficient_material_as_win_for_white(
        ) {
            let mut board = Board::from_fen("8/8/8/8/8/8/1K1N4/8 b - - 0 1");
            assert_eq!(evaluate_board(&mut board), BoardEvaluation::WinWhite);
        }
    }
}
