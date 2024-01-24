#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MoveSpecial {
    CastleLong,
    CastleShort,
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
