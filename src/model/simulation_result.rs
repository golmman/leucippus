use super::board_evaluation::BoardEvaluation;

pub struct SimulationResult {
    pub depth: usize,
    pub evaluation: BoardEvaluation,
}
