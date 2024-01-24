#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MoveSpecial {
    BlackCastleLong,
    BlackCastleShort,
    WhiteCastleLong,
    WhiteCastleShort,
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

    pub fn black_castle_long() -> Move {
        Move {
            from: 0,
            to: 0,
            special: Some(MoveSpecial::BlackCastleLong),
        }
    }

    pub fn black_castle_short() -> Move {
        Move {
            from: 0,
            to: 0,
            special: Some(MoveSpecial::BlackCastleShort),
        }
    }

    pub fn white_castle_long() -> Move {
        Move {
            from: 0,
            to: 0,
            special: Some(MoveSpecial::WhiteCastleLong),
        }
    }

    pub fn white_castle_short() -> Move {
        Move {
            from: 0,
            to: 0,
            special: Some(MoveSpecial::WhiteCastleShort),
        }
    }
}
