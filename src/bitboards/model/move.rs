use crate::model::types::SquareIndex;

use super::move_type::MoveType;
use super::piece_type::PieceType;
use super::piece_type::KNIGHT;

pub type MoveData = u16;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Move {
    data: MoveData,
}

impl From<Move> for bool {
    fn from(m: Move) -> Self {
        m.data != 0
    }
}

impl Move {
    pub const fn from_data(data: u16) -> Self {
        Self { data }
    }

    pub const fn from_squares(from: SquareIndex, to: SquareIndex) -> Self {
        let data = ((from << 6) + to) as u16;
        Self { data }
    }

    pub const fn make<const T: MoveType>(
        from: SquareIndex,
        to: SquareIndex,
        pt: PieceType,
    ) -> Self {
        debug_assert!(pt >= KNIGHT);
        let data = T
            + (((pt - KNIGHT) as u16) << 12)
            + ((from as u16) << 6)
            + to as u16;
        Self { data }
    }

    pub const fn from_sq(&self) -> SquareIndex {
        debug_assert!(self.is_ok());
        ((self.data >> 6) & 0x3F) as SquareIndex
    }

    pub const fn to_sq(&self) -> SquareIndex {
        debug_assert!(self.is_ok());
        (self.data & 0x3F) as SquareIndex
    }

    pub const fn from_to(&self) -> MoveData {
        self.data & 0xFFF
    }

    pub const fn type_of(&self) -> MoveType {
        self.data & (3 << 14)
    }

    pub const fn promotion_type(&self) -> PieceType {
        ((self.data >> 12) & 3) as PieceType + KNIGHT
    }

    pub const fn is_ok(&self) -> bool {
        Self::none().data != self.data && Self::null().data != self.data
    }

    pub const fn null() -> Self {
        Self { data: 65 }
    }

    pub const fn none() -> Self {
        Self { data: 0 }
    }

    pub const fn raw(&self) -> MoveData {
        self.data
    }

    pub const fn get_hash(&self) -> u64 {
        (self.data as u64)
            .wrapping_mul(6364136223846793005u64)
            .wrapping_add(1442695040888963407u64)
    }
}
