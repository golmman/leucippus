use crate::model::board::Board;
use crate::model::r#move::Move;
use crate::model::types::SquareIndex;

pub fn add_sliding_moves(
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
