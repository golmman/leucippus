use crate::model::board::Board;
use crate::model::r#move::Move;
use crate::model::types::SquareIndex;

const DIR: [(i8, i8); 4] = [(-1, -1), (1, -1), (-1, 1), (1, 1)];

pub fn generate(board: &Board) -> Vec<Move> {
    let mut moves = Vec::new();

    for from in &board.pieces.active_bishops {
        let from_x = (from % 8) as i8;
        let from_y = (from / 8) as i8;

        add_direction_moves(board, &mut moves, *from, from_x, from_y, DIR[0]);
        add_direction_moves(board, &mut moves, *from, from_x, from_y, DIR[1]);
        add_direction_moves(board, &mut moves, *from, from_x, from_y, DIR[2]);
        add_direction_moves(board, &mut moves, *from, from_x, from_y, DIR[3]);
    }

    moves
}

fn add_direction_moves(
    board: &Board,
    moves: &mut Vec<Move>,
    from: u8,
    from_x: i8,
    from_y: i8,
    direction: (i8, i8),
) {
    for i in 1..8 {
        let to_x = from_x + i * direction.0;
        let to_y = from_y + i * direction.1;

        if to_x < 0 || to_x > 7 || to_y < 0 || to_y > 7 {
            break;
        }

        let to = (8 * to_y + to_x) as SquareIndex;

        if board.pieces.squares.data[to as usize].is_none() {
            moves.push(Move::from_to(from, to));
            continue;
        }

        if let Some(piece) = board.pieces.squares.data[to as usize] {
            if piece.get_color() != board.color {
                moves.push(Move::from_to(from, to));
            }
            break;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_generates_bishop_moves_from_the_starting_position() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 0);
    }

    #[test]
    fn it_generates_black_bishop_moves_on_an_empty_board() {
        let fen = "8/8/8/8/2b2b2/8/8/8 b - - 0 1";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 22);
        assert_eq!(
            moves,
            vec![
                Move::from_to(26, 17),
                Move::from_to(26, 8),
                Move::from_to(26, 19),
                Move::from_to(26, 12),
                Move::from_to(26, 5),
                Move::from_to(26, 33),
                Move::from_to(26, 40),
                Move::from_to(26, 35),
                Move::from_to(26, 44),
                Move::from_to(26, 53),
                Move::from_to(26, 62),
                Move::from_to(29, 20),
                Move::from_to(29, 11),
                Move::from_to(29, 2),
                Move::from_to(29, 22),
                Move::from_to(29, 15),
                Move::from_to(29, 36),
                Move::from_to(29, 43),
                Move::from_to(29, 50),
                Move::from_to(29, 57),
                Move::from_to(29, 38),
                Move::from_to(29, 47),
            ]
        );
    }

    #[test]
    fn it_generates_white_bishop_moves_on_an_empty_board() {
        let fen = "8/8/8/8/2B2B2/8/8/8 w - - 0 1";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 22);
        assert_eq!(
            moves,
            vec![
                Move::from_to(26, 17),
                Move::from_to(26, 8),
                Move::from_to(26, 19),
                Move::from_to(26, 12),
                Move::from_to(26, 5),
                Move::from_to(26, 33),
                Move::from_to(26, 40),
                Move::from_to(26, 35),
                Move::from_to(26, 44),
                Move::from_to(26, 53),
                Move::from_to(26, 62),
                Move::from_to(29, 20),
                Move::from_to(29, 11),
                Move::from_to(29, 2),
                Move::from_to(29, 22),
                Move::from_to(29, 15),
                Move::from_to(29, 36),
                Move::from_to(29, 43),
                Move::from_to(29, 50),
                Move::from_to(29, 57),
                Move::from_to(29, 38),
                Move::from_to(29, 47),
            ]
        );
    }

    #[test]
    fn it_generates_black_bishop_capture_moves_on_an_otherwise_empty_board() {
        let fen = "1N4N1/8/N6N/8/2b2b2/8/N6N/2N2N2 b - - 0 1";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 22);
        assert_eq!(
            moves,
            vec![
                Move::from_to(26, 17),
                Move::from_to(26, 8),
                Move::from_to(26, 19),
                Move::from_to(26, 12),
                Move::from_to(26, 5),
                Move::from_to(26, 33),
                Move::from_to(26, 40),
                Move::from_to(26, 35),
                Move::from_to(26, 44),
                Move::from_to(26, 53),
                Move::from_to(26, 62),
                Move::from_to(29, 20),
                Move::from_to(29, 11),
                Move::from_to(29, 2),
                Move::from_to(29, 22),
                Move::from_to(29, 15),
                Move::from_to(29, 36),
                Move::from_to(29, 43),
                Move::from_to(29, 50),
                Move::from_to(29, 57),
                Move::from_to(29, 38),
                Move::from_to(29, 47),
            ]
        );
    }

    #[test]
    fn it_generates_white_bishop_capture_moves_on_an_otherwise_empty_board() {
        let fen = "1n4n1/8/n6n/8/2B2B2/8/n6n/2n2n2 w - - 0 1";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 22);
        assert_eq!(
            moves,
            vec![
                Move::from_to(26, 17),
                Move::from_to(26, 8),
                Move::from_to(26, 19),
                Move::from_to(26, 12),
                Move::from_to(26, 5),
                Move::from_to(26, 33),
                Move::from_to(26, 40),
                Move::from_to(26, 35),
                Move::from_to(26, 44),
                Move::from_to(26, 53),
                Move::from_to(26, 62),
                Move::from_to(29, 20),
                Move::from_to(29, 11),
                Move::from_to(29, 2),
                Move::from_to(29, 22),
                Move::from_to(29, 15),
                Move::from_to(29, 36),
                Move::from_to(29, 43),
                Move::from_to(29, 50),
                Move::from_to(29, 57),
                Move::from_to(29, 38),
                Move::from_to(29, 47),
            ]
        );
    }

    #[test]
    fn it_generates_black_bishop_in_the_giuoco_piano() {
        let fen = "r1bq1rk1/pppp1ppp/2n2n2/2b1p3/2B1P3/2PP1N2/PP3PPP/RNBQ1RK1 b - - 0 6";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 8);
        assert_eq!(
            moves,
            vec![
                Move::from_to(34, 25),
                Move::from_to(34, 16),
                Move::from_to(34, 27),
                Move::from_to(34, 20),
                Move::from_to(34, 13),
                Move::from_to(34, 41),
                Move::from_to(34, 43),
                Move::from_to(34, 52),
            ]
        );
    }

    #[test]
    fn it_generates_white_bishop_in_the_giuoco_piano() {
        let fen = "r1bq1rk1/pppp1ppp/2n2n2/2b1p3/2B1P3/3P1N2/PPP2PPP/RNBQ1RK1 w - - 1 6";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 11);
        assert_eq!(
            moves,
            vec![
                Move::from_to(2, 11),
                Move::from_to(2, 20),
                Move::from_to(2, 29),
                Move::from_to(2, 38),
                Move::from_to(2, 47),
                Move::from_to(26, 17),
                Move::from_to(26, 33),
                Move::from_to(26, 40),
                Move::from_to(26, 35),
                Move::from_to(26, 44),
                Move::from_to(26, 53),
            ]
        );
    }
}
