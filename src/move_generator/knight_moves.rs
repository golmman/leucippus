use crate::model::board::Board;
use crate::model::r#move::Move;

pub fn generate(board: &Board) -> Vec<Move> {
    let mut moves = Vec::new();

    for from in &board.pieces.active_knights {
        let from_x = (from % 8) as i32;
        let from_y = (from / 8) as i32;

        check_and_add(board, &mut moves, *from, from_x - 1, from_y - 2);
        check_and_add(board, &mut moves, *from, from_x + 1, from_y - 2);
        check_and_add(board, &mut moves, *from, from_x - 2, from_y - 1);
        check_and_add(board, &mut moves, *from, from_x + 2, from_y - 1);
        check_and_add(board, &mut moves, *from, from_x - 2, from_y + 1);
        check_and_add(board, &mut moves, *from, from_x + 2, from_y + 1);
        check_and_add(board, &mut moves, *from, from_x - 1, from_y + 2);
        check_and_add(board, &mut moves, *from, from_x + 1, from_y + 2);
    }

    moves
}

fn check_and_add(
    board: &Board,
    moves: &mut Vec<Move>,
    from: usize,
    to_x: i32,
    to_y: i32,
) {
    if to_x < 0 || to_x > 7 || to_y < 0 || to_y > 7 {
        return;
    }

    let to = (8 * to_y + to_x) as usize;
    if let Some(piece) = board.pieces.squares.data[to] {
        if piece.get_color() == board.color {
            return;
        }
    }

    moves.push(Move { from, to });
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_generates_knight_moves_from_the_starting_position() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 4);
        assert_eq!(
            moves,
            vec![
                Move { from: 1, to: 16 },
                Move { from: 1, to: 18 },
                Move { from: 6, to: 21 },
                Move { from: 6, to: 23 }
            ]
        );
    }

    #[test]
    fn it_generates_knight_moves_with_4_black_knights() {
        let fen = "rnbqkbnr/pppppppp/8/8/2n5/n7/PPPPPPPP/RNBQKBNR b KQkq - 0 1";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 14);
        assert_eq!(
            moves,
            vec![
                Move { from: 16, to: 1 },
                Move { from: 16, to: 10 },
                Move { from: 16, to: 33 },
                Move { from: 26, to: 9 },
                Move { from: 26, to: 11 },
                Move { from: 26, to: 20 },
                Move { from: 26, to: 32 },
                Move { from: 26, to: 36 },
                Move { from: 26, to: 41 },
                Move { from: 26, to: 43 },
                Move { from: 57, to: 40 },
                Move { from: 57, to: 42 },
                Move { from: 62, to: 45 },
                Move { from: 62, to: 47 }
            ]
        );
    }
}
