#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BoardEvaluation {
    Draw,
    Inconclusive,
    WinBlack,
    WinWhite,
}
