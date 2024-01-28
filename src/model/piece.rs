use super::color::Color;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub enum Piece {
    BlackBishop = 0,
    BlackKing = 1,
    BlackKnight = 2,
    BlackPawn = 3,
    BlackQueen = 4,
    BlackRook = 5,
    WhiteBishop = 6,
    WhiteKing = 7,
    WhiteKnight = 8,
    WhitePawn = 9,
    WhiteQueen = 10,
    WhiteRook = 11,
}

impl Piece {
    pub const fn get_color(&self) -> Color {
        if (*self as u8) < 6 {
            Color::Black
        } else {
            Color::White
        }
    }

    pub fn is_bishop_of_color(&self, color: Color) -> bool {
        match color {
            Color::Black => *self == Piece::BlackBishop,
            Color::White => *self == Piece::WhiteBishop,
        }
    }

    pub fn is_king_of_color(&self, color: Color) -> bool {
        match color {
            Color::Black => *self == Piece::BlackKing,
            Color::White => *self == Piece::WhiteKing,
        }
    }

    pub fn is_knight_of_color(&self, color: Color) -> bool {
        match color {
            Color::Black => *self == Piece::BlackKnight,
            Color::White => *self == Piece::WhiteKnight,
        }
    }

    pub fn is_pawn_of_color(&self, color: Color) -> bool {
        match color {
            Color::Black => *self == Piece::BlackPawn,
            Color::White => *self == Piece::WhitePawn,
        }
    }

    pub fn is_queen_of_color(&self, color: Color) -> bool {
        match color {
            Color::Black => *self == Piece::BlackQueen,
            Color::White => *self == Piece::WhiteQueen,
        }
    }

    pub fn is_rook_of_color(&self, color: Color) -> bool {
        match color {
            Color::Black => *self == Piece::BlackRook,
            Color::White => *self == Piece::WhiteRook,
        }
    }

    pub fn is_bishop(self) -> bool {
        self == Piece::BlackBishop || self == Piece::WhiteBishop
    }

    pub fn is_knight(self) -> bool {
        self == Piece::BlackKnight || self == Piece::WhiteKnight
    }

    pub fn is_pawn(self) -> bool {
        self == Piece::BlackPawn || self == Piece::WhitePawn
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_determines_the_color_of_a_piece() {
        assert_eq!(Piece::BlackBishop.get_color(), Color::Black);
        assert_eq!(Piece::BlackKing.get_color(), Color::Black);
        assert_eq!(Piece::BlackKnight.get_color(), Color::Black);
        assert_eq!(Piece::BlackPawn.get_color(), Color::Black);
        assert_eq!(Piece::BlackQueen.get_color(), Color::Black);
        assert_eq!(Piece::BlackRook.get_color(), Color::Black);
        assert_eq!(Piece::WhiteBishop.get_color(), Color::White);
        assert_eq!(Piece::WhiteKing.get_color(), Color::White);
        assert_eq!(Piece::WhiteKnight.get_color(), Color::White);
        assert_eq!(Piece::WhitePawn.get_color(), Color::White);
        assert_eq!(Piece::WhiteQueen.get_color(), Color::White);
        assert_eq!(Piece::WhiteRook.get_color(), Color::White);
    }
}
