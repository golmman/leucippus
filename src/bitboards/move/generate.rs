use crate::bitboards::model::position::Position;
use crate::model::r#move::Move;

pub struct MoveGenerator {
    pub moves: Vec<Move>,
}

impl MoveGenerator {
    pub fn new() -> Self {
        Self {
            moves: Vec::with_capacity(40),
        }
    }

    pub fn generate(&mut self, position: &Position) {
        self.generate_knight_moves(position);
    }

    pub fn get_random_legal_move(&self, position: &Position) -> Option<Move> {
        None
    }
}
