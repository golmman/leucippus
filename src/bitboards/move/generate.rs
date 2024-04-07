use crate::bitboards::model::position::Position;
use crate::model::r#move::Move;
use crate::model::types::SquareIndex;

use super::attacks::Direction;

#[derive(Clone, Copy, Debug, PartialEq)]
enum GenType {
    Captures,
    Quiets,
    QuietChecks,
    Evasions,
    NonEvasions,
    Legal,
}

fn generate_promotion_moves(
    gen_type: GenType,
    d: Direction,
    enemy: bool,
    move_list: &mut Vec<Move>,
    to: SquareIndex,
) {
    let all =
        gen_type == GenType::Evasions || gen_type == GenType::NonEvasions;

    if gen_type == GenType::Captures || all {
        // TODO: new model for move?
        move_list.push(Move::promote_queen_white(to - d as u8, to));
    }
}
