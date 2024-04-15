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

    pub const fn make_default<const T: MoveType>(
        from: SquareIndex,
        to: SquareIndex,
    ) -> Self {
        Self::make::<T>(from, to, KNIGHT)
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

#[cfg(test)]
mod test {
    use crate::bitboards::model::move_type::CASTLING;
    use crate::bitboards::model::move_type::EN_PASSANT;
    use crate::bitboards::model::move_type::NORMAL;
    use crate::bitboards::model::move_type::PROMOTION;

    use super::*;

    #[test]
    fn it_makes_normal_knight_moves() {
        assert_eq!(Move::make::<NORMAL>(10, 24, KNIGHT).raw(), 664);
        assert_eq!(Move::make::<NORMAL>(10, 9, KNIGHT).raw(), 649);
        assert_eq!(Move::make::<NORMAL>(54, 11, KNIGHT).raw(), 3467);
        assert_eq!(Move::make::<NORMAL>(15, 16, KNIGHT).raw(), 976);
        assert_eq!(Move::make::<NORMAL>(17, 21, KNIGHT).raw(), 1109);
        assert_eq!(Move::make::<NORMAL>(13, 27, KNIGHT).raw(), 859);
        assert_eq!(Move::make::<NORMAL>(60, 31, KNIGHT).raw(), 3871);
        assert_eq!(Move::make::<NORMAL>(39, 11, KNIGHT).raw(), 2507);
        assert_eq!(Move::make::<NORMAL>(61, 17, KNIGHT).raw(), 3921);
        assert_eq!(Move::make::<NORMAL>(49, 16, KNIGHT).raw(), 3152);
        assert_eq!(Move::make::<NORMAL>(26, 3, KNIGHT).raw(), 1667);
        assert_eq!(Move::make::<NORMAL>(7, 2, KNIGHT).raw(), 450);
        assert_eq!(Move::make::<NORMAL>(29, 60, KNIGHT).raw(), 1916);
        assert_eq!(Move::make::<NORMAL>(44, 55, KNIGHT).raw(), 2871);
        assert_eq!(Move::make::<NORMAL>(36, 1, KNIGHT).raw(), 2305);
        assert_eq!(Move::make::<NORMAL>(35, 19, KNIGHT).raw(), 2259);
        assert_eq!(Move::make::<NORMAL>(49, 46, KNIGHT).raw(), 3182);
        assert_eq!(Move::make::<NORMAL>(62, 22, KNIGHT).raw(), 3990);
        assert_eq!(Move::make::<NORMAL>(33, 53, KNIGHT).raw(), 2165);
        assert_eq!(Move::make::<NORMAL>(14, 44, KNIGHT).raw(), 940);
    }

    #[test]
    fn it_makes_normal_default_moves() {
        assert_eq!(Move::make_default::<NORMAL>(10, 24).raw(), 664);
        assert_eq!(Move::make_default::<NORMAL>(10, 9).raw(), 649);
        assert_eq!(Move::make_default::<NORMAL>(54, 11).raw(), 3467);
        assert_eq!(Move::make_default::<NORMAL>(15, 16).raw(), 976);
        assert_eq!(Move::make_default::<NORMAL>(17, 21).raw(), 1109);
        assert_eq!(Move::make_default::<NORMAL>(13, 27).raw(), 859);
        assert_eq!(Move::make_default::<NORMAL>(60, 31).raw(), 3871);
        assert_eq!(Move::make_default::<NORMAL>(39, 11).raw(), 2507);
        assert_eq!(Move::make_default::<NORMAL>(61, 17).raw(), 3921);
        assert_eq!(Move::make_default::<NORMAL>(49, 16).raw(), 3152);
        assert_eq!(Move::make_default::<NORMAL>(26, 3).raw(), 1667);
        assert_eq!(Move::make_default::<NORMAL>(7, 2).raw(), 450);
        assert_eq!(Move::make_default::<NORMAL>(29, 60).raw(), 1916);
        assert_eq!(Move::make_default::<NORMAL>(44, 55).raw(), 2871);
        assert_eq!(Move::make_default::<NORMAL>(36, 1).raw(), 2305);
        assert_eq!(Move::make_default::<NORMAL>(35, 19).raw(), 2259);
        assert_eq!(Move::make_default::<NORMAL>(49, 46).raw(), 3182);
        assert_eq!(Move::make_default::<NORMAL>(62, 22).raw(), 3990);
        assert_eq!(Move::make_default::<NORMAL>(33, 53).raw(), 2165);
        assert_eq!(Move::make_default::<NORMAL>(14, 44).raw(), 940);
    }

    #[test]
    fn it_makes_promotion_default_moves() {
        assert_eq!(Move::make_default::<PROMOTION>(10, 24).raw(), 17048);
        assert_eq!(Move::make_default::<PROMOTION>(10, 9).raw(), 17033);
        assert_eq!(Move::make_default::<PROMOTION>(54, 11).raw(), 19851);
        assert_eq!(Move::make_default::<PROMOTION>(15, 16).raw(), 17360);
        assert_eq!(Move::make_default::<PROMOTION>(17, 21).raw(), 17493);
        assert_eq!(Move::make_default::<PROMOTION>(13, 27).raw(), 17243);
        assert_eq!(Move::make_default::<PROMOTION>(60, 31).raw(), 20255);
        assert_eq!(Move::make_default::<PROMOTION>(39, 11).raw(), 18891);
        assert_eq!(Move::make_default::<PROMOTION>(61, 17).raw(), 20305);
        assert_eq!(Move::make_default::<PROMOTION>(49, 16).raw(), 19536);
        assert_eq!(Move::make_default::<PROMOTION>(26, 3).raw(), 18051);
        assert_eq!(Move::make_default::<PROMOTION>(7, 2).raw(), 16834);
        assert_eq!(Move::make_default::<PROMOTION>(29, 60).raw(), 18300);
        assert_eq!(Move::make_default::<PROMOTION>(44, 55).raw(), 19255);
        assert_eq!(Move::make_default::<PROMOTION>(36, 1).raw(), 18689);
        assert_eq!(Move::make_default::<PROMOTION>(35, 19).raw(), 18643);
        assert_eq!(Move::make_default::<PROMOTION>(49, 46).raw(), 19566);
        assert_eq!(Move::make_default::<PROMOTION>(62, 22).raw(), 20374);
        assert_eq!(Move::make_default::<PROMOTION>(33, 53).raw(), 18549);
        assert_eq!(Move::make_default::<PROMOTION>(14, 44).raw(), 17324);
    }

    #[test]
    fn it_makes_en_passant_default_moves() {
        assert_eq!(Move::make_default::<EN_PASSANT>(10, 24).raw(), 33432);
        assert_eq!(Move::make_default::<EN_PASSANT>(10, 9).raw(), 33417);
        assert_eq!(Move::make_default::<EN_PASSANT>(54, 11).raw(), 36235);
        assert_eq!(Move::make_default::<EN_PASSANT>(15, 16).raw(), 33744);
        assert_eq!(Move::make_default::<EN_PASSANT>(17, 21).raw(), 33877);
        assert_eq!(Move::make_default::<EN_PASSANT>(13, 27).raw(), 33627);
        assert_eq!(Move::make_default::<EN_PASSANT>(60, 31).raw(), 36639);
        assert_eq!(Move::make_default::<EN_PASSANT>(39, 11).raw(), 35275);
        assert_eq!(Move::make_default::<EN_PASSANT>(61, 17).raw(), 36689);
        assert_eq!(Move::make_default::<EN_PASSANT>(49, 16).raw(), 35920);
        assert_eq!(Move::make_default::<EN_PASSANT>(26, 3).raw(), 34435);
        assert_eq!(Move::make_default::<EN_PASSANT>(7, 2).raw(), 33218);
        assert_eq!(Move::make_default::<EN_PASSANT>(29, 60).raw(), 34684);
        assert_eq!(Move::make_default::<EN_PASSANT>(44, 55).raw(), 35639);
        assert_eq!(Move::make_default::<EN_PASSANT>(36, 1).raw(), 35073);
        assert_eq!(Move::make_default::<EN_PASSANT>(35, 19).raw(), 35027);
        assert_eq!(Move::make_default::<EN_PASSANT>(49, 46).raw(), 35950);
        assert_eq!(Move::make_default::<EN_PASSANT>(62, 22).raw(), 36758);
        assert_eq!(Move::make_default::<EN_PASSANT>(33, 53).raw(), 34933);
        assert_eq!(Move::make_default::<EN_PASSANT>(14, 44).raw(), 33708);
    }

    #[test]
    fn it_makes_castling_default_moves() {
        assert_eq!(Move::make_default::<CASTLING>(10, 24).raw(), 49816);
        assert_eq!(Move::make_default::<CASTLING>(10, 9).raw(), 49801);
        assert_eq!(Move::make_default::<CASTLING>(54, 11).raw(), 52619);
        assert_eq!(Move::make_default::<CASTLING>(15, 16).raw(), 50128);
        assert_eq!(Move::make_default::<CASTLING>(17, 21).raw(), 50261);
        assert_eq!(Move::make_default::<CASTLING>(13, 27).raw(), 50011);
        assert_eq!(Move::make_default::<CASTLING>(60, 31).raw(), 53023);
        assert_eq!(Move::make_default::<CASTLING>(39, 11).raw(), 51659);
        assert_eq!(Move::make_default::<CASTLING>(61, 17).raw(), 53073);
        assert_eq!(Move::make_default::<CASTLING>(49, 16).raw(), 52304);
        assert_eq!(Move::make_default::<CASTLING>(26, 3).raw(), 50819);
        assert_eq!(Move::make_default::<CASTLING>(7, 2).raw(), 49602);
        assert_eq!(Move::make_default::<CASTLING>(29, 60).raw(), 51068);
        assert_eq!(Move::make_default::<CASTLING>(44, 55).raw(), 52023);
        assert_eq!(Move::make_default::<CASTLING>(36, 1).raw(), 51457);
        assert_eq!(Move::make_default::<CASTLING>(35, 19).raw(), 51411);
        assert_eq!(Move::make_default::<CASTLING>(49, 46).raw(), 52334);
        assert_eq!(Move::make_default::<CASTLING>(62, 22).raw(), 53142);
        assert_eq!(Move::make_default::<CASTLING>(33, 53).raw(), 51317);
        assert_eq!(Move::make_default::<CASTLING>(14, 44).raw(), 50092);
    }
}
