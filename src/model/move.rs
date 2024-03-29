use super::types::SquareIndex;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MoveSpecial {
    CastleLongBlack = 0,
    CastleShortBlack = 1,
    CastleLongWhite = 2,
    CastleShortWhite = 3,
    EnPassant = 4,
    PromoteBishopBlack = 5,
    PromoteKnightBlack = 6,
    PromoteQueenBlack = 7,
    PromoteRookBlack = 8,
    PromoteBishopWhite = 9,
    PromoteKnightWhite = 10,
    PromoteQueenWhite = 11,
    PromoteRookWhite = 12,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Move {
    pub from: SquareIndex,
    pub to: SquareIndex,
    pub special: Option<MoveSpecial>,
}

impl Move {
    pub fn from_to(from: SquareIndex, to: SquareIndex) -> Move {
        Move {
            from,
            to,
            special: None,
        }
    }

    pub fn castle_long_black() -> Move {
        Move {
            from: 60,
            to: 58,
            special: Some(MoveSpecial::CastleLongBlack),
        }
    }

    pub fn castle_short_black() -> Move {
        Move {
            from: 60,
            to: 62,
            special: Some(MoveSpecial::CastleShortBlack),
        }
    }

    pub fn castle_long_white() -> Move {
        Move {
            from: 4,
            to: 2,
            special: Some(MoveSpecial::CastleLongWhite),
        }
    }

    pub fn castle_short_white() -> Move {
        Move {
            from: 4,
            to: 6,
            special: Some(MoveSpecial::CastleShortWhite),
        }
    }

    pub fn promote_bishop_black(from: SquareIndex, to: SquareIndex) -> Move {
        Move {
            from,
            to,
            special: Some(MoveSpecial::PromoteBishopBlack),
        }
    }

    pub fn promote_knight_black(from: SquareIndex, to: SquareIndex) -> Move {
        Move {
            from,
            to,
            special: Some(MoveSpecial::PromoteKnightBlack),
        }
    }

    pub fn promote_queen_black(from: SquareIndex, to: SquareIndex) -> Move {
        Move {
            from,
            to,
            special: Some(MoveSpecial::PromoteQueenBlack),
        }
    }

    pub fn promote_rook_black(from: SquareIndex, to: SquareIndex) -> Move {
        Move {
            from,
            to,
            special: Some(MoveSpecial::PromoteRookBlack),
        }
    }

    pub fn promote_bishop_white(from: SquareIndex, to: SquareIndex) -> Move {
        Move {
            from,
            to,
            special: Some(MoveSpecial::PromoteBishopWhite),
        }
    }

    pub fn promote_knight_white(from: SquareIndex, to: SquareIndex) -> Move {
        Move {
            from,
            to,
            special: Some(MoveSpecial::PromoteKnightWhite),
        }
    }

    pub fn promote_queen_white(from: SquareIndex, to: SquareIndex) -> Move {
        Move {
            from,
            to,
            special: Some(MoveSpecial::PromoteQueenWhite),
        }
    }

    pub fn promote_rook_white(from: SquareIndex, to: SquareIndex) -> Move {
        Move {
            from,
            to,
            special: Some(MoveSpecial::PromoteRookWhite),
        }
    }

    pub fn en_passant(from: SquareIndex, to: SquareIndex) -> Move {
        Move {
            from,
            to,
            special: Some(MoveSpecial::EnPassant),
        }
    }

    pub fn is_en_passant(&self) -> bool {
        self.special.is_some_and(|s| s == MoveSpecial::EnPassant)
    }

    pub fn is_castle(&self) -> bool {
        self.special.is_some_and(|s| s as u8 <= 3)
    }

    pub fn is_promotion(&self) -> bool {
        self.special.is_some_and(|s| s as u8 >= 5)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_recognizes_en_passant_moves() {
        assert!(Move::en_passant(0, 0).is_en_passant());

        assert!(!Move::castle_long_black().is_en_passant());
        assert!(!Move::castle_short_black().is_en_passant());
        assert!(!Move::castle_long_white().is_en_passant());
        assert!(!Move::castle_short_white().is_en_passant());
        assert!(!Move::promote_bishop_black(0, 0).is_en_passant());
        assert!(!Move::promote_knight_black(0, 0).is_en_passant());
        assert!(!Move::promote_queen_black(0, 0).is_en_passant());
        assert!(!Move::promote_rook_black(0, 0).is_en_passant());
        assert!(!Move::promote_bishop_white(0, 0).is_en_passant());
        assert!(!Move::promote_knight_white(0, 0).is_en_passant());
        assert!(!Move::promote_queen_white(0, 0).is_en_passant());
        assert!(!Move::promote_rook_white(0, 0).is_en_passant());
    }

    #[test]
    fn it_recognizes_castling_moves() {
        assert!(Move::castle_long_black().is_castle());
        assert!(Move::castle_short_black().is_castle());
        assert!(Move::castle_long_white().is_castle());
        assert!(Move::castle_short_white().is_castle());

        assert!(!Move::en_passant(0, 0).is_castle());
        assert!(!Move::promote_bishop_black(0, 0).is_castle());
        assert!(!Move::promote_knight_black(0, 0).is_castle());
        assert!(!Move::promote_queen_black(0, 0).is_castle());
        assert!(!Move::promote_rook_black(0, 0).is_castle());
        assert!(!Move::promote_bishop_white(0, 0).is_castle());
        assert!(!Move::promote_knight_white(0, 0).is_castle());
        assert!(!Move::promote_queen_white(0, 0).is_castle());
        assert!(!Move::promote_rook_white(0, 0).is_castle());
    }

    #[test]
    fn it_recognizes_promotion_moves() {
        assert!(Move::promote_bishop_black(0, 0).is_promotion());
        assert!(Move::promote_knight_black(0, 0).is_promotion());
        assert!(Move::promote_queen_black(0, 0).is_promotion());
        assert!(Move::promote_rook_black(0, 0).is_promotion());
        assert!(Move::promote_bishop_white(0, 0).is_promotion());
        assert!(Move::promote_knight_white(0, 0).is_promotion());
        assert!(Move::promote_queen_white(0, 0).is_promotion());
        assert!(Move::promote_rook_white(0, 0).is_promotion());

        assert!(!Move::en_passant(0, 0).is_promotion());
        assert!(!Move::castle_long_black().is_promotion());
        assert!(!Move::castle_short_black().is_promotion());
        assert!(!Move::castle_long_white().is_promotion());
        assert!(!Move::castle_short_white().is_promotion());
    }
}
