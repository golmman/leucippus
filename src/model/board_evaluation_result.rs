use super::board_evaluation::BoardEvaluation;
use super::r#move::Move;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoardEvaluationResult {
    pub evaluation: BoardEvaluation,
    pub random_move: Option<Move>,
}
