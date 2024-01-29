use super::color::Color;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BoardEvaluation {
    Draw,
    Inconclusive,
    WinBlack,
    WinWhite,
}

impl BoardEvaluation {
    pub fn is_conclusive(&self) -> bool {
        *self != BoardEvaluation::Inconclusive
    }

    pub fn get_win_color(&self) -> Option<Color> {
        match *self {
            BoardEvaluation::WinBlack => Some(Color::Black),
            BoardEvaluation::WinWhite => Some(Color::White),
            _ => None,
        }
    }
}
