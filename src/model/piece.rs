use super::color::Color;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Piece {
    BlackBishop,
    BlackKing,
    BlackKnight,
    BlackPawn,
    BlackQueen,
    BlackRook,
    WhiteBishop,
    WhiteKing,
    WhiteKnight,
    WhitePawn,
    WhiteQueen,
    WhiteRook,
}

impl Piece {
    pub fn get_color(&self) -> Color {
        match self {
            Piece::BlackBishop => Color::Black,
            Piece::BlackKing => Color::Black,
            Piece::BlackKnight => Color::Black,
            Piece::BlackPawn => Color::Black,
            Piece::BlackQueen => Color::Black,
            Piece::BlackRook => Color::Black,
            Piece::WhiteBishop => Color::White,
            Piece::WhiteKing => Color::White,
            Piece::WhiteKnight => Color::White,
            Piece::WhitePawn => Color::White,
            Piece::WhiteQueen => Color::White,
            Piece::WhiteRook => Color::White,
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

    pub fn is_pawn(&self) -> bool {
        *self == Piece::BlackPawn || *self == Piece::WhitePawn
    }
}
