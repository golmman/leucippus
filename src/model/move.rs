use super::types::SquareIndex;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MoveSpecial {
    CastleLongBlack,
    CastleShortBlack,
    CastleLongWhite,
    CastleShortWhite,
    EnPassant,
    PromoteBishop,
    PromoteKnight,
    PromoteQueen,
    PromoteRook,
}

#[derive(Clone, Copy, Debug, PartialEq)]
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

    pub fn promote_bishop(from: SquareIndex, to: SquareIndex) -> Move {
        Move {
            from,
            to,
            special: Some(MoveSpecial::PromoteBishop),
        }
    }

    pub fn promote_knight(from: SquareIndex, to: SquareIndex) -> Move {
        Move {
            from,
            to,
            special: Some(MoveSpecial::PromoteKnight),
        }
    }

    pub fn promote_queen(from: SquareIndex, to: SquareIndex) -> Move {
        Move {
            from,
            to,
            special: Some(MoveSpecial::PromoteQueen),
        }
    }

    pub fn promote_rook(from: SquareIndex, to: SquareIndex) -> Move {
        Move {
            from,
            to,
            special: Some(MoveSpecial::PromoteRook),
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
        self.special.is_some_and(|s| {
            s == MoveSpecial::CastleLongBlack || s == MoveSpecial::CastleShortBlack
            || s == MoveSpecial::CastleLongWhite || s == MoveSpecial::CastleShortWhite
        })
    }

    pub fn is_promotion(&self) -> bool {
        self.special.is_some_and(|s| {
            s == MoveSpecial::PromoteBishop
                || s == MoveSpecial::PromoteKnight
                || s == MoveSpecial::PromoteQueen
                || s == MoveSpecial::PromoteRook
        })
    }
}
