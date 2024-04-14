pub type MoveType = u16;
pub const NORMAL: MoveType = 0;
pub const PROMOTION: MoveType = 1 << 14;
pub const EN_PASSANT: MoveType = 2 << 14;
pub const CASTLING: MoveType = 3 << 14;
