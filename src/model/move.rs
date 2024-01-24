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
    pub from: u8,
    pub to: u8,
    pub special: Option<MoveSpecial>,
}

impl Move {
    pub fn from_to(from: u8, to: u8) -> Move {
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

    pub fn promote_bishop(from: u8, to: u8) -> Move {
        Move {
            from,
            to,
            special: Some(MoveSpecial::PromoteBishop),
        }
    }

    pub fn promote_knight(from: u8, to: u8) -> Move {
        Move {
            from,
            to,
            special: Some(MoveSpecial::PromoteKnight),
        }
    }

    pub fn promote_queen(from: u8, to: u8) -> Move {
        Move {
            from,
            to,
            special: Some(MoveSpecial::PromoteQueen),
        }
    }

    pub fn promote_rook(from: u8, to: u8) -> Move {
        Move {
            from,
            to,
            special: Some(MoveSpecial::PromoteRook),
        }
    }
}
