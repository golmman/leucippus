use super::types::SquareIndex;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MoveSpecial {
    CastleLong,
    CastleShort,
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

    pub fn castle_long() -> Move {
        Move {
            from: 0,
            to: 0,
            special: Some(MoveSpecial::CastleLong),
        }
    }

    pub fn castle_short() -> Move {
        Move {
            from: 0,
            to: 0,
            special: Some(MoveSpecial::CastleShort),
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
            s == MoveSpecial::CastleLong || s == MoveSpecial::CastleShort
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
