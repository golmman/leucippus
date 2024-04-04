// port of stockfishs bitboard.cpp

use std::ops::Deref;
use std::sync::OnceLock;

use crate::bitboards::model::bitboard::Bitboard;
use crate::bitboards::model::position::PositionColor;
use crate::bitboards::r#move::bishop_table::BISHOP_TABLE;
use crate::bitboards::r#move::rook_table::ROOK_TABLE;
use crate::model::piece_type::PieceType;
use crate::model::types::square_names::*;
use crate::model::types::SquareIndex;

// TODO: everything not public should probably just be a function to save memory

const fn rand(s: u64) -> (u64, u64) {
    let mut s0 = s;
    s0 ^= s0 >> 12;
    s0 ^= s0 << 25;
    s0 ^= s0 >> 27;
    (s0.wrapping_mul(2685821657736338717u64), s0)
}

const fn sparse_rand(s: u64) -> (Bitboard, u64) {
    let mut s0 = s;
    let (r1, s0) = rand(s0);
    let (r2, s0) = rand(s0);
    let (r3, s0) = rand(s0);
    (Bitboard(r1 & r2 & r3), s0)
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    North = 8,
    NorthNorth = 16,
    East = 1,
    South = -8,
    SouthSouth = -16,
    West = -1,
    NorthEast = 9,
    SouthEast = -7,
    SouthWest = -9,
    NorthWest = 7,
}

union Bitboard16 {
    b64: Bitboard,
    b16: [u16; 4],
}

pub const BISHOP_TABLE_SIZE: usize = 0x1480;
pub const ROOK_TABLE_SIZE: usize = 0x19000;

pub type BishopTable = MagicTable<BISHOP_TABLE_SIZE>;
pub type RookTable = MagicTable<ROOK_TABLE_SIZE>;

#[derive(Debug, PartialEq)]
pub struct MagicTable<const N: usize> {
    pub magics: [Magic; 64],
    pub table: [Bitboard; N],
}

impl<const N: usize> MagicTable<N> {
    const fn new() -> Self {
        const MAGIC_INIT: Magic = Magic {
            mask: Bitboard(0),
            magic: Bitboard(0),
            attacks: 0,
            shift: 0,
        };
        Self {
            magics: [MAGIC_INIT; 64],
            table: [Bitboard(0); N],
        }
    }

    /// corresponds to e.g. BishopMagics[square].attacks[magics_index] in stockfish's code
    const fn get_attack(
        &self,
        square: SquareIndex,
        magic_index: usize,
    ) -> Bitboard {
        self.table[self.magics[square as usize].attacks + magic_index]
    }
}

#[derive(Debug, PartialEq)]
pub struct Magic {
    pub mask: Bitboard,
    pub magic: Bitboard,
    pub attacks: usize,
    pub shift: u8,
}

impl Magic {
    // TODO: use the real pext
    const fn index(&self, occupied: Bitboard) -> usize {
        if HAS_PEXT {
            return pext(occupied, self.mask) as usize;
        }

        if IS_64_BIT {
            return (((occupied.0 & self.mask.0).wrapping_mul(self.magic.0))
                >> self.shift) as usize;
        }

        let lo = occupied.0 & self.mask.0;
        let hi = occupied.0 >> 32 & self.mask.0 >> 32;
        (lo * self.magic.0 ^ hi * (self.magic.0 >> 32) >> self.shift) as usize
    }
}

const FILE_A: Bitboard = Bitboard(0x0101010101010101);
const FILE_B: Bitboard = Bitboard(FILE_A.0 << 1);
const FILE_C: Bitboard = Bitboard(FILE_A.0 << 2);
const FILE_D: Bitboard = Bitboard(FILE_A.0 << 3);
const FILE_E: Bitboard = Bitboard(FILE_A.0 << 4);
const FILE_F: Bitboard = Bitboard(FILE_A.0 << 5);
const FILE_G: Bitboard = Bitboard(FILE_A.0 << 6);
const FILE_H: Bitboard = Bitboard(FILE_A.0 << 7);

const RANK_1: Bitboard = Bitboard(0xFF);
const RANK_2: Bitboard = Bitboard(RANK_1.0 << (8 * 1));
const RANK_3: Bitboard = Bitboard(RANK_1.0 << (8 * 2));
const RANK_4: Bitboard = Bitboard(RANK_1.0 << (8 * 3));
const RANK_5: Bitboard = Bitboard(RANK_1.0 << (8 * 4));
const RANK_6: Bitboard = Bitboard(RANK_1.0 << (8 * 5));
const RANK_7: Bitboard = Bitboard(RANK_1.0 << (8 * 6));
const RANK_8: Bitboard = Bitboard(RANK_1.0 << (8 * 7));

#[cfg(all(target_arch = "x86_64", target_feature = "bmi2"))]
const HAS_PEXT: bool = true;

#[cfg(not(all(target_arch = "x86_64", target_feature = "bmi2")))]
const HAS_PEXT: bool = false;

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
const IS_64_BIT: bool = true;

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
const IS_64_BIT: bool = false;

#[cfg(all(target_arch = "x86_64", target_feature = "bmi2"))]
fn pext2(a: Bitboard, mask: Bitboard) -> u64 {
    unsafe {
        return core::arch::x86_64::_pext_u64(a.0, mask.0);
    }
}

const fn pext(a: Bitboard, mask: Bitboard) -> u64 {
    // see https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_pext_u64&ig_expand=5088
    let mut dst = 0;
    let mut m = 0;
    let mut k = 0;
    while m < 64 {
        if 0 != mask.0 & 1 << m {
            if 0 != a.0 & 1 << m {
                dst |= 1 << k;
            }
            k += 1;
        }
        m += 1;
    }
    return dst;
}

const fn max_u8(a: u8, b: u8) -> u8 {
    if a > b {
        return a;
    } else {
        return b;
    }
}

const fn rank_bb_from_rank(r: u8) -> Bitboard {
    Bitboard(RANK_1.0 << (8 * r))
}

const fn rank_bb_from_square(s: SquareIndex) -> Bitboard {
    rank_bb_from_rank(RANK_OF[s as usize])
}

const fn file_bb_from_file(f: u8) -> Bitboard {
    Bitboard(FILE_A.0 << f)
}

const fn file_bb_from_square(s: SquareIndex) -> Bitboard {
    file_bb_from_file(FILE_OF[s as usize])
}

#[rustfmt::skip]
const FILE_OF: [SquareIndex; 64] = [
    0, 1, 2, 3, 4, 5, 6, 7,
    0, 1, 2, 3, 4, 5, 6, 7,
    0, 1, 2, 3, 4, 5, 6, 7,
    0, 1, 2, 3, 4, 5, 6, 7,
    0, 1, 2, 3, 4, 5, 6, 7,
    0, 1, 2, 3, 4, 5, 6, 7,
    0, 1, 2, 3, 4, 5, 6, 7,
    0, 1, 2, 3, 4, 5, 6, 7,
];

#[rustfmt::skip]
const RANK_OF: [SquareIndex; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0,
    1, 1, 1, 1, 1, 1, 1, 1,
    2, 2, 2, 2, 2, 2, 2, 2,
    3, 3, 3, 3, 3, 3, 3, 3,
    4, 4, 4, 4, 4, 4, 4, 4,
    5, 5, 5, 5, 5, 5, 5, 5,
    6, 6, 6, 6, 6, 6, 6, 6,
    7, 7, 7, 7, 7, 7, 7, 7,
];

const fn is_aligned(s1: SquareIndex, s2: SquareIndex, s3: SquareIndex) -> bool {
    LINE_BB[s1 as usize][s2 as usize].0 & SQUARE[s3 as usize].0 != 0
}

const FILE_DISTANCE: [[u8; 8]; 8] = {
    let mut file_distance = [[0; 8]; 8];
    let mut f1 = 0i32;
    while f1 < 8 {
        let mut f2 = 0i32;
        while f2 < 8 {
            file_distance[f1 as usize][f2 as usize] = (f1 - f2).abs() as u8;
            f2 += 1;
        }
        f1 += 1;
    }
    file_distance
};

const RANK_DISTANCE: [[u8; 8]; 8] = {
    let mut rank_distance = [[0; 8]; 8];
    let mut r1 = 0i32;
    while r1 < 8 {
        let mut r2 = 0i32;
        while r2 < 8 {
            rank_distance[r1 as usize][r2 as usize] = (r1 - r2).abs() as u8;
            r2 += 1;
        }
        r1 += 1;
    }
    rank_distance
};

const SQUARE_DISTANCE: [[u8; 64]; 64] = {
    let mut square_distance = [[0; 64]; 64];
    let mut s1 = 0;
    while s1 < 64 {
        let mut s2 = 0;
        while s2 < 64 {
            square_distance[s1 as usize][s2 as usize] = max_u8(
                RANK_DISTANCE[RANK_OF[s1] as usize][RANK_OF[s2] as usize],
                FILE_DISTANCE[FILE_OF[s1] as usize][FILE_OF[s2] as usize],
            );
            s2 += 1;
        }
        s1 += 1;
    }
    square_distance
};

const EDGE_DISTANCE: [u8; 8] = [0, 1, 2, 3, 3, 2, 1, 0];

const fn popcount(b: Bitboard) -> u8 {
    b.0.count_ones() as u8
}

const fn lsb(b: Bitboard) -> SquareIndex {
    debug_assert!(b.0 != 0);
    b.0.trailing_zeros() as SquareIndex
}

const fn msb(b: Bitboard) -> SquareIndex {
    debug_assert!(b.0 != 0);
    63 - b.0.leading_zeros() as SquareIndex
}

const fn least_significant_square_bb(b: Bitboard) -> Bitboard {
    debug_assert!(b.0 != 0);
    Bitboard(b.0 & 0u64.wrapping_sub(b.0))
}

pub fn pop_lsb(b: &mut Bitboard) -> SquareIndex {
    debug_assert!(b.0 != 0);
    let s = lsb(*b);
    *b = Bitboard(b.0 & (b.0 - 1));
    s
}

const SQUARE: [Bitboard; 64] = {
    let mut square = [Bitboard(0); 64];
    let mut s = 0;
    while s < 64 {
        square[s].0 = 1 << s;
        s += 1;
    }
    square
};

const fn is_more_than_one(b: Bitboard) -> bool {
    // equal to "b.0.count_ones() > 0"
    b.0 & b.0.wrapping_sub(1) != 0
}

const fn is_ok(s: SquareIndex) -> bool {
    s >= A1 && s <= H8
}

// TODO: why is the distance <= 2 check necessary?
const fn safe_destination(s: SquareIndex, step: i32) -> Bitboard {
    let to = s as i32 + step;
    if is_ok(to as SquareIndex) && SQUARE_DISTANCE[s as usize][to as usize] <= 2
    {
        SQUARE[to as usize]
    } else {
        Bitboard(0)
    }
}

const fn sliding_attack(
    p: PieceType,
    s: SquareIndex,
    occupied: Bitboard,
) -> Bitboard {
    let mut attacks = Bitboard(0);

    let rook_directions = [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];

    let bishop_directions = [
        Direction::NorthEast,
        Direction::SouthEast,
        Direction::SouthWest,
        Direction::NorthWest,
    ];

    let directions = if p as u8 == PieceType::Rook as u8 {
        rook_directions
    } else {
        bishop_directions
    };

    let mut d = 0;
    while d < 4 {
        let mut square = s as i32;
        let direction = directions[d];
        while safe_destination(square as SquareIndex, direction as i32).0 != 0
            && occupied.0 & SQUARE[square as usize].0 == 0
        {
            square += direction as i32;
            attacks.0 |= SQUARE[square as usize].0;
        }

        d += 1;
    }

    attacks
}

const fn shift(d: Direction, b: Bitboard) -> Bitboard {
    match d {
        Direction::North => Bitboard(b.0 << 8),
        Direction::South => Bitboard(b.0 >> 8),
        Direction::NorthNorth => Bitboard(b.0 << 16),
        Direction::SouthSouth => Bitboard(b.0 >> 16),
        Direction::East => Bitboard((b.0 & !FILE_H.0) << 1),
        Direction::West => Bitboard((b.0 & !FILE_A.0) >> 1),
        Direction::NorthEast => Bitboard((b.0 & !FILE_H.0) << 9),
        Direction::NorthWest => Bitboard((b.0 & !FILE_A.0) << 7),
        Direction::SouthEast => Bitboard((b.0 & !FILE_H.0) >> 7),
        Direction::SouthWest => Bitboard((b.0 & !FILE_A.0) >> 9),
    }
}

/// corresponds to stockfish's pawn_attacks_bb<color>(bitboard)
const fn pawn_attacks_by_bitboard(c: PositionColor, b: Bitboard) -> Bitboard {
    match c {
        PositionColor::Black => Bitboard(
            shift(Direction::SouthWest, b).0 | shift(Direction::SouthEast, b).0,
        ),
        PositionColor::White => Bitboard(
            shift(Direction::NorthWest, b).0 | shift(Direction::NorthEast, b).0,
        ),
    }
}

/// corresponds to stockfish's pawn_attacks_bb<color>(square)
const fn pawn_attacks_by_square(c: PositionColor, s: SquareIndex) -> Bitboard {
    debug_assert!(is_ok(s));
    PAWN_PSEUDO_ATTACKS[c as usize][s as usize]
}

/// corresponds to stockfish's PawnAttacks[color][square]
const PAWN_PSEUDO_ATTACKS: [[Bitboard; 64]; 2] = {
    let mut attacks = [[Bitboard(0); 64]; 2];

    let mut s = 0;
    while s < 64 {
        attacks[PositionColor::Black as usize][s as usize] =
            pawn_attacks_by_bitboard(PositionColor::Black, SQUARE[s as usize]);
        attacks[PositionColor::White as usize][s as usize] =
            pawn_attacks_by_bitboard(PositionColor::White, SQUARE[s as usize]);
        s += 1;
    }

    attacks
};

/// corresponds to stockfish's PseudoAttacks[KING][square]
const KING_PSEUDO_ATTACKS: [Bitboard; 64] = {
    let mut attacks = [Bitboard(0); 64];

    let mut s = 0;
    while s < 64 {
        let steps = [-9, -8, -7, -1, 1, 7, 8, 9];
        let mut step = 0;
        while step < steps.len() {
            attacks[s as usize].0 |= safe_destination(s, steps[step]).0;
            step += 1;
        }
        s += 1;
    }

    attacks
};

/// corresponds to stockfish's PseudoAttacks[KNIGHT][square]
const KNIGHT_PSEUDO_ATTACKS: [Bitboard; 64] = {
    let mut attacks = [Bitboard(0); 64];

    let mut s = 0;
    while s < 64 {
        let steps = [-17, -15, -10, -6, 6, 10, 15, 17];
        let mut step = 0;
        while step < steps.len() {
            attacks[s as usize].0 |= safe_destination(s, steps[step]).0;
            step += 1;
        }
        s += 1;
    }

    attacks
};

/// corresponds to stockfish's PseudoAttacks[BISHOP][square]
const BISHOP_PSEUDO_ATTACKS: [Bitboard; 64] = {
    let mut attacks = [Bitboard(0); 64];

    let mut s = 0;
    while s < 64 {
        attacks[s as usize] =
            get_piece_pseudo_attacks(PieceType::Bishop, s, Bitboard(0));
        s += 1;
    }

    attacks
};

/// corresponds to stockfish's PseudoAttacks[ROOK][square]
const ROOK_PSEUDO_ATTACKS: [Bitboard; 64] = {
    let mut attacks = [Bitboard(0); 64];

    let mut s = 0;
    while s < 64 {
        attacks[s as usize] =
            get_piece_pseudo_attacks(PieceType::Rook, s, Bitboard(0));
        s += 1;
    }

    attacks
};

/// corresponds to stockfish's PseudoAttacks[QUEEN][square]
const QUEEN_PSEUDO_ATTACKS: [Bitboard; 64] = {
    let mut attacks = [Bitboard(0); 64];

    let mut s = 0;
    while s < 64 {
        attacks[s as usize] = Bitboard(
            BISHOP_PSEUDO_ATTACKS[s as usize].0
                | ROOK_PSEUDO_ATTACKS[s as usize].0,
        );
        s += 1;
    }

    attacks
};

const LINE_BB: [[Bitboard; 64]; 64] = {
    let mut bb = [[Bitboard(0); 64]; 64];

    let mut s1 = 0;
    while s1 < 64 {
        let mut s2 = 0;
        while s2 < 64 {
            if BISHOP_PSEUDO_ATTACKS[s1 as usize].0 & SQUARE[s2 as usize].0 != 0
            {
                bb[s1 as usize][s2 as usize] = Bitboard(
                    (get_piece_pseudo_attacks(
                        PieceType::Bishop,
                        s1,
                        Bitboard(0),
                    )
                    .0 & get_piece_pseudo_attacks(
                        PieceType::Bishop,
                        s2,
                        Bitboard(0),
                    )
                    .0) | SQUARE[s1 as usize].0
                        | SQUARE[s2 as usize].0,
                );
            }

            if ROOK_PSEUDO_ATTACKS[s1 as usize].0 & SQUARE[s2 as usize].0 != 0 {
                bb[s1 as usize][s2 as usize] = Bitboard(
                    (get_piece_pseudo_attacks(
                        PieceType::Rook,
                        s1,
                        Bitboard(0),
                    )
                    .0 & get_piece_pseudo_attacks(
                        PieceType::Rook,
                        s2,
                        Bitboard(0),
                    )
                    .0) | SQUARE[s1 as usize].0
                        | SQUARE[s2 as usize].0,
                );
            }

            s2 += 1;
        }
        s1 += 1;
    }

    bb
};

const BETWEEN_BB: [[Bitboard; 64]; 64] = {
    let mut bb = [[Bitboard(0); 64]; 64];

    let mut s1 = 0;
    while s1 < 64 {
        let mut s2 = 0;
        while s2 < 64 {
            if BISHOP_PSEUDO_ATTACKS[s1 as usize].0 & SQUARE[s2 as usize].0 != 0
            {
                bb[s1 as usize][s2 as usize] = Bitboard(
                    get_piece_pseudo_attacks(
                        PieceType::Bishop,
                        s1,
                        SQUARE[s2 as usize],
                    )
                    .0 & get_piece_pseudo_attacks(
                        PieceType::Bishop,
                        s2,
                        SQUARE[s1 as usize],
                    )
                    .0,
                );
            }
            bb[s1 as usize][s2 as usize].0 |= SQUARE[s2 as usize].0;

            if ROOK_PSEUDO_ATTACKS[s1 as usize].0 & SQUARE[s2 as usize].0 != 0 {
                bb[s1 as usize][s2 as usize] = Bitboard(
                    get_piece_pseudo_attacks(
                        PieceType::Rook,
                        s1,
                        SQUARE[s2 as usize],
                    )
                    .0 & get_piece_pseudo_attacks(
                        PieceType::Rook,
                        s2,
                        SQUARE[s1 as usize],
                    )
                    .0,
                );
            }
            bb[s1 as usize][s2 as usize].0 |= SQUARE[s2 as usize].0;

            s2 += 1;
        }
        s1 += 1;
    }

    bb
};

/// corresponds to stockfish's attack_bb functions by square
const fn get_piece_pseudo_attacks_by_square(
    pt: PieceType,
    s: SquareIndex,
) -> Bitboard {
    let s = s as usize;
    match pt {
        PieceType::Bishop => BISHOP_PSEUDO_ATTACKS[s],
        PieceType::King => KING_PSEUDO_ATTACKS[s],
        PieceType::Knight => KNIGHT_PSEUDO_ATTACKS[s],
        PieceType::Pawn => panic!(),
        PieceType::Queen => QUEEN_PSEUDO_ATTACKS[s],
        PieceType::Rook => ROOK_PSEUDO_ATTACKS[s],
    }
}

/// corresponds to stockfish's attack_bb functions by square and occupation
const fn get_piece_pseudo_attacks(
    pt: PieceType,
    s: SquareIndex,
    occupied: Bitboard,
) -> Bitboard {
    match pt {
        PieceType::Bishop => BISHOP_TABLE
            .get_attack(s, BISHOP_TABLE.magics[s as usize].index(occupied)),
        PieceType::King => KING_PSEUDO_ATTACKS[s as usize],
        PieceType::Knight => KNIGHT_PSEUDO_ATTACKS[s as usize],
        PieceType::Pawn => panic!(),
        PieceType::Queen => Bitboard(
            get_piece_pseudo_attacks(PieceType::Bishop, s, occupied).0
                | get_piece_pseudo_attacks(PieceType::Rook, s, occupied).0,
        ),
        PieceType::Rook => ROOK_TABLE
            .get_attack(s, ROOK_TABLE.magics[s as usize].index(occupied)),
    }
}

pub const fn init_bishop_table() -> BishopTable {
    init_magic_table::<BISHOP_TABLE_SIZE>()
}

pub const fn init_rook_table() -> RookTable {
    init_magic_table::<ROOK_TABLE_SIZE>()
}

const fn init_magic_table<const TABLE_SIZE: usize>() -> MagicTable<TABLE_SIZE> {
    let pt = match TABLE_SIZE {
        BISHOP_TABLE_SIZE => PieceType::Bishop,
        ROOK_TABLE_SIZE => PieceType::Rook,
        _ => panic!(),
    };

    let mut mt = MagicTable::<TABLE_SIZE>::new();

    let seeds_32 = [8977, 44560, 54343, 38998, 5731, 95205, 104912, 17020];
    let seeds_64 = [728, 10316, 55013, 32803, 12281, 15100, 16645, 255];
    let mut occupancy = [Bitboard(0); 4096];
    let mut reference = [Bitboard(0); 4096];
    let mut edges = Bitboard(0);
    let mut b = Bitboard(0);
    let mut epoch = [0i32; 4096];
    let mut cnt = 0;
    let mut size = 0;

    let mut s = 0;
    while s < 64 {
        let si = s as usize;
        edges = Bitboard(
            ((RANK_1.0 | RANK_8.0) & !rank_bb_from_square(s).0)
                | ((FILE_A.0 | FILE_H.0) & !file_bb_from_square(s).0),
        );

        mt.magics[si].mask =
            Bitboard(sliding_attack(pt, s, Bitboard(0)).0 & !edges.0);

        mt.magics[si].shift =
            if IS_64_BIT { 64 } else { 32 } - popcount(mt.magics[si].mask);

        mt.magics[si].attacks = if s == A1 {
            0
        } else {
            mt.magics[si - 1].attacks + size
        };

        b = Bitboard(0);
        size = 0;
        loop {
            occupancy[size] = b;
            reference[size] = sliding_attack(pt, s, b);

            if HAS_PEXT {
                let a = mt.magics[si].attacks;
                let p = pext(b, mt.magics[si].mask) as usize;
                mt.table[a + p] = reference[size];
            }

            size += 1;
            b = Bitboard(
                (b.0 as i64 - mt.magics[si].mask.0 as i64) as u64
                    & mt.magics[si].mask.0,
            );

            if b.0 == 0 {
                break;
            }
        }

        if HAS_PEXT {
            s += 1;
            continue;
        }

        let mut seed = if IS_64_BIT {
            seeds_64[RANK_OF[s as usize] as usize]
        } else {
            seeds_32[RANK_OF[s as usize] as usize]
        };

        let mut i = 0;
        while i < size {
            mt.magics[si].magic = Bitboard(0);
            loop {
                let (r, seed0) = sparse_rand(seed);
                seed = seed0;
                mt.magics[si].magic = r;

                let multi =
                    mt.magics[si].magic.0.wrapping_mul(mt.magics[si].mask.0);
                if popcount(Bitboard(multi >> 56)) >= 6 {
                    break;
                }
            }

            cnt += 1;
            i = 0;
            while i < size {
                let idx = mt.magics[si].index(occupancy[i]);

                if epoch[idx] < cnt {
                    epoch[idx] = cnt;
                    mt.table[mt.magics[si].attacks + idx] = reference[i];
                } else if mt.table[mt.magics[si].attacks + idx].0
                    != reference[i].0
                {
                    break;
                }

                i += 1;
            }
        }

        s += 1;
    }

    mt
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn it_validates_pext_implementation() {
        assert_eq!(pext(Bitboard(0x37CDDC37E30E7B3A), Bitboard(0xEE50B60B2E764F85)), 1567040472);
        assert_eq!(pext(Bitboard(0x15E42D16FD015B95), Bitboard(0x4DB0F9F4059E2578)), 1012461586);
        assert_eq!(pext(Bitboard(0x5BF5022FB5A67802), Bitboard(0x24369106DD4E284A)), 14800505);
        assert_eq!(pext(Bitboard(0x15DD541E4A2F33A8), Bitboard(0xC424F7C8EAAEE373)), 5739232488);
        assert_eq!(pext(Bitboard(0x0000000000000000), Bitboard(0x0000000000000000)), 0);
        assert_eq!(pext(Bitboard(0x15DD541E4A2F33A8), Bitboard(0x0000000000000000)), 0);
        assert_eq!(pext(Bitboard(0xFFFFFFFFFFFFFFFF), Bitboard(0xFFFFFFFFFFFFFFFF)), 0xFFFFFFFFFFFFFFFF);
        assert_eq!(pext(Bitboard(0x15DD541E4A2F33A8), Bitboard(0xFFFFFFFFFFFFFFFF)), 0x15DD541E4A2F33A8);
    }

    #[test]
    fn it_gives_the_square_distance() {
        assert_eq!(SQUARE_DISTANCE[A1 as usize][A1 as usize], 0);
        assert_eq!(SQUARE_DISTANCE[A1 as usize][A8 as usize], 7);
        assert_eq!(SQUARE_DISTANCE[A1 as usize][H8 as usize], 7);
        assert_eq!(SQUARE_DISTANCE[G7 as usize][F2 as usize], 5);
        assert_eq!(SQUARE_DISTANCE[D4 as usize][E5 as usize], 1);
        assert_eq!(SQUARE_DISTANCE[D4 as usize][D2 as usize], 2);
    }

    #[test]
    fn it_gives_the_bitboard_representing_one_square() {
        assert_eq!(
            SQUARE[A1 as usize],
            Bitboard::from([
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [1, 0, 0, 0, 0, 0, 0, 0],
            ])
        );

        assert_eq!(
            SQUARE[E4 as usize],
            Bitboard::from([
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 1, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
            ])
        );

        assert_eq!(
            SQUARE[D5 as usize],
            Bitboard::from([
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 1, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
            ])
        );

        assert_eq!(
            SQUARE[H8 as usize],
            Bitboard::from([
                [0, 0, 0, 0, 0, 0, 0, 1],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
            ])
        );
    }

    #[test]
    fn it_checks_valid_square_indices() {
        assert!(is_ok(A1));
        assert!(is_ok(B4));
        assert!(is_ok(F3));
        assert!(is_ok(H8));

        assert!(!is_ok(111));
        assert!(!is_ok(64));
        assert!(!is_ok(-4i32 as SquareIndex));
    }

    #[test]
    fn it_calculates_the_sliding_attack_bitboards() {
        assert_eq!(
            sliding_attack(PieceType::Bishop, E4, FILE_G),
            Bitboard::from([
                [1, 0, 0, 0, 0, 0, 0, 0],
                [0, 1, 0, 0, 0, 0, 0, 0],
                [0, 0, 1, 0, 0, 0, 1, 0],
                [0, 0, 0, 1, 0, 1, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 1, 0, 1, 0, 0],
                [0, 0, 1, 0, 0, 0, 1, 0],
                [0, 1, 0, 0, 0, 0, 0, 0],
            ])
        );

        assert_eq!(
            sliding_attack(PieceType::Bishop, E4, FILE_C | FILE_G),
            Bitboard::from([
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 1, 0, 0, 0, 1, 0],
                [0, 0, 0, 1, 0, 1, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 1, 0, 1, 0, 0],
                [0, 0, 1, 0, 0, 0, 1, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
            ])
        );

        assert_eq!(
            sliding_attack(PieceType::Rook, D4, Bitboard(0)),
            Bitboard::from([
                [0, 0, 0, 1, 0, 0, 0, 0],
                [0, 0, 0, 1, 0, 0, 0, 0],
                [0, 0, 0, 1, 0, 0, 0, 0],
                [0, 0, 0, 1, 0, 0, 0, 0],
                [1, 1, 1, 0, 1, 1, 1, 1],
                [0, 0, 0, 1, 0, 0, 0, 0],
                [0, 0, 0, 1, 0, 0, 0, 0],
                [0, 0, 0, 1, 0, 0, 0, 0],
            ])
        );

        assert_eq!(
            sliding_attack(PieceType::Rook, D4, RANK_2 | RANK_5 | FILE_F),
            Bitboard::from([
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 1, 0, 0, 0, 0],
                [1, 1, 1, 0, 1, 1, 0, 0],
                [0, 0, 0, 1, 0, 0, 0, 0],
                [0, 0, 0, 1, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
            ])
        );
    }

    #[test]
    fn it_generates_random_numbers() {
        let mut s = 1111;
        let (r, s) = sparse_rand(s);
        assert_eq!(r.0, 1229484966019108928);

        let (r, s) = sparse_rand(s);
        assert_eq!(r.0, 4630554740693942336);

        let (r, s) = sparse_rand(s);
        assert_eq!(r.0, 1691049957266048);

        let (r, s) = sparse_rand(s);
        assert_eq!(r.0, 1729736300192366592);
    }

    #[test]
    fn it_confirms_idea_of_wrapping_sub() {
        let a = 123u64;
        let b = 999999u64;
        let c = 18446744073708551740u64;

        assert_eq!((a as i64 - b as i64) as u64, c);
        assert_eq!(a.wrapping_sub(b), c);

        assert_eq!((a as i64 - c as i64) as u64, b);
        assert_eq!(a.wrapping_sub(c), b);
    }

    mod magics {
        use super::*;
        #[test]
        fn it_generates_bishop_magics() {
            // values confirmed by running and inspecting stockfish's values
            assert_eq!(BISHOP_TABLE.get_attack(10, 11), Bitboard(655370));
            assert_eq!(BISHOP_TABLE.get_attack(12, 5), Bitboard(550899286056));
            assert_eq!(
                BISHOP_TABLE.get_attack(61, 100),
                Bitboard(18049651735265280)
            );
            assert_eq!(
                BISHOP_TABLE.get_attack(37, 17),
                Bitboard(38368559105573890)
            );
        }

        #[test]
        fn it_generates_rook_magics() {
            // values confirmed by running and inspecting stockfish's values
            assert_eq!(ROOK_TABLE.get_attack(1, 38), Bitboard(131613));
            assert_eq!(ROOK_TABLE.get_attack(10, 22), Bitboard(4415293753860));
            assert_eq!(ROOK_TABLE.get_attack(17, 501), Bitboard(33882624));
            assert_eq!(
                ROOK_TABLE.get_attack(52, 71),
                Bitboard(1166168420698292224)
            );
        }

        #[test]
        #[cfg(not(debug_assertions))]
        fn it_confirms_generated_bishop_values_match() {
            let table = init_bishop_table();
            assert_eq!(BISHOP_TABLE, table);
        }

        #[test]
        #[cfg(not(debug_assertions))]
        fn it_confirms_generated_rook_values_match() {
            let table = init_rook_table();
            assert_eq!(ROOK_TABLE, table);
        }
    }

    mod shift {
        use super::*;

        fn cross() -> Bitboard {
            Bitboard::from([
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 1, 1, 0, 0, 0],
                [0, 0, 1, 1, 1, 1, 0, 0],
                [0, 0, 1, 1, 1, 1, 0, 0],
                [0, 0, 0, 1, 1, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
            ])
        }

        #[test]
        fn it_shifts_bitboards_north() {
            assert_eq!(
                shift(Direction::North, cross()),
                Bitboard::from([
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 1, 1, 0, 0, 0],
                    [0, 0, 1, 1, 1, 1, 0, 0],
                    [0, 0, 1, 1, 1, 1, 0, 0],
                    [0, 0, 0, 1, 1, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                ])
            );
        }

        #[test]
        fn it_shifts_bitboards_south() {
            assert_eq!(
                shift(Direction::South, cross()),
                Bitboard::from([
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 1, 1, 0, 0, 0],
                    [0, 0, 1, 1, 1, 1, 0, 0],
                    [0, 0, 1, 1, 1, 1, 0, 0],
                    [0, 0, 0, 1, 1, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                ])
            );
        }

        #[test]
        fn it_shifts_bitboards_north_north() {
            assert_eq!(
                shift(Direction::NorthNorth, cross()),
                Bitboard::from([
                    [0, 0, 0, 1, 1, 0, 0, 0],
                    [0, 0, 1, 1, 1, 1, 0, 0],
                    [0, 0, 1, 1, 1, 1, 0, 0],
                    [0, 0, 0, 1, 1, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                ])
            );
        }

        #[test]
        fn it_shifts_bitboards_south_south() {
            assert_eq!(
                shift(Direction::SouthSouth, cross()),
                Bitboard::from([
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 1, 1, 0, 0, 0],
                    [0, 0, 1, 1, 1, 1, 0, 0],
                    [0, 0, 1, 1, 1, 1, 0, 0],
                    [0, 0, 0, 1, 1, 0, 0, 0],
                ])
            );
        }

        #[test]
        fn it_shifts_bitboards_east() {
            assert_eq!(
                shift(Direction::East, cross()),
                Bitboard::from([
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 1, 1, 0, 0],
                    [0, 0, 0, 1, 1, 1, 1, 0],
                    [0, 0, 0, 1, 1, 1, 1, 0],
                    [0, 0, 0, 0, 1, 1, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                ])
            );
        }

        #[test]
        fn it_shifts_bitboards_west() {
            assert_eq!(
                shift(Direction::West, cross()),
                Bitboard::from([
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 1, 1, 0, 0, 0, 0],
                    [0, 1, 1, 1, 1, 0, 0, 0],
                    [0, 1, 1, 1, 1, 0, 0, 0],
                    [0, 0, 1, 1, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                ])
            );
        }

        #[test]
        fn it_shifts_bitboards_north_east() {
            assert_eq!(
                shift(Direction::NorthEast, cross()),
                Bitboard::from([
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 1, 1, 0, 0],
                    [0, 0, 0, 1, 1, 1, 1, 0],
                    [0, 0, 0, 1, 1, 1, 1, 0],
                    [0, 0, 0, 0, 1, 1, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                ])
            );
        }

        #[test]
        fn it_shifts_bitboards_north_west() {
            assert_eq!(
                shift(Direction::NorthWest, cross()),
                Bitboard::from([
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 1, 1, 0, 0, 0, 0],
                    [0, 1, 1, 1, 1, 0, 0, 0],
                    [0, 1, 1, 1, 1, 0, 0, 0],
                    [0, 0, 1, 1, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                ])
            );
        }

        #[test]
        fn it_shifts_bitboards_south_east() {
            assert_eq!(
                shift(Direction::SouthEast, cross()),
                Bitboard::from([
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 1, 1, 0, 0],
                    [0, 0, 0, 1, 1, 1, 1, 0],
                    [0, 0, 0, 1, 1, 1, 1, 0],
                    [0, 0, 0, 0, 1, 1, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                ])
            );
        }

        #[test]
        fn it_shifts_bitboards_south_west() {
            assert_eq!(
                shift(Direction::SouthWest, cross()),
                Bitboard::from([
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 1, 1, 0, 0, 0, 0],
                    [0, 1, 1, 1, 1, 0, 0, 0],
                    [0, 1, 1, 1, 1, 0, 0, 0],
                    [0, 0, 1, 1, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                ])
            );
        }
    }

    mod pawn_pseudo_attacks {
        use super::*;
        #[test]
        fn it_generates_pawn_attacks_by_bitboard_for_black() {
            assert_eq!(
                pawn_attacks_by_bitboard(
                    PositionColor::Black,
                    Bitboard::from([
                        [0, 0, 0, 0, 0, 0, 0, 0],
                        [0, 0, 0, 0, 0, 0, 0, 0],
                        [0, 0, 0, 0, 0, 0, 0, 0],
                        [0, 1, 0, 0, 0, 0, 1, 0],
                        [0, 0, 0, 1, 0, 0, 0, 0],
                        [0, 0, 0, 0, 0, 0, 0, 0],
                        [0, 0, 0, 0, 0, 0, 0, 1],
                        [0, 0, 0, 0, 0, 0, 0, 0],
                    ])
                ),
                Bitboard::from([
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [1, 0, 1, 0, 0, 1, 0, 1],
                    [0, 0, 1, 0, 1, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 1, 0],
                ])
            );
        }

        #[test]
        fn it_generates_pawn_attacks_by_bitboard_for_white() {
            assert_eq!(
                pawn_attacks_by_bitboard(
                    PositionColor::White,
                    Bitboard::from([
                        [0, 0, 0, 0, 0, 0, 0, 0],
                        [0, 0, 0, 0, 0, 0, 0, 0],
                        [0, 0, 0, 0, 0, 0, 0, 0],
                        [0, 1, 0, 0, 0, 0, 1, 0],
                        [0, 0, 0, 1, 0, 0, 0, 0],
                        [0, 0, 0, 0, 0, 0, 0, 0],
                        [0, 0, 0, 0, 0, 0, 0, 1],
                        [0, 0, 0, 0, 0, 0, 0, 0],
                    ])
                ),
                Bitboard::from([
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [1, 0, 1, 0, 0, 1, 0, 1],
                    [0, 0, 1, 0, 1, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 1, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                ])
            );
        }

        #[test]
        fn it_generates_pawn_pseudo_attacks_by_square_for_black() {
            assert_eq!(
                pawn_attacks_by_square(PositionColor::Black, A7),
                Bitboard::from([
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 1, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                ])
            );
            assert_eq!(
                pawn_attacks_by_square(PositionColor::Black, E3),
                Bitboard::from([
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 1, 0, 1, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                ])
            );
        }

        #[test]
        fn it_generates_pawn_pseudo_attacks_by_square_for_white() {
            assert_eq!(
                pawn_attacks_by_square(PositionColor::White, A7),
                Bitboard::from([
                    [0, 1, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                ])
            );
            assert_eq!(
                pawn_attacks_by_square(PositionColor::White, E3),
                Bitboard::from([
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 1, 0, 1, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                ])
            );
        }
    }

    mod piece_pseudo_attacks {
        use super::*;

        #[test]
        fn it_generates_king_pseudo_attacks_on_an_empty_board() {
            assert_eq!(
                KING_PSEUDO_ATTACKS[E4 as usize],
                Bitboard::from([
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 1, 1, 1, 0, 0],
                    [0, 0, 0, 1, 0, 1, 0, 0],
                    [0, 0, 0, 1, 1, 1, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                ])
            );
            assert_eq!(
                KING_PSEUDO_ATTACKS[A1 as usize],
                Bitboard::from([
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [1, 1, 0, 0, 0, 0, 0, 0],
                    [0, 1, 0, 0, 0, 0, 0, 0],
                ])
            );
        }

        #[test]
        fn it_generates_knight_pseudo_attacks_on_an_empty_board() {
            assert_eq!(
                KNIGHT_PSEUDO_ATTACKS[E4 as usize],
                Bitboard::from([
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 1, 0, 1, 0, 0],
                    [0, 0, 1, 0, 0, 0, 1, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 1, 0, 0, 0, 1, 0],
                    [0, 0, 0, 1, 0, 1, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                ])
            );
            assert_eq!(
                KNIGHT_PSEUDO_ATTACKS[A1 as usize],
                Bitboard::from([
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 1, 0, 0, 0, 0, 0, 0],
                    [0, 0, 1, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                ])
            );
        }

        #[test]
        fn it_generates_knight_pseudo_attacks_on_an_occupied_board() {
            assert_eq!(
                get_piece_pseudo_attacks(
                    PieceType::Knight,
                    E4,
                    RANK_1 | RANK_2 | RANK_3
                ),
                Bitboard::from([
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 1, 0, 1, 0, 0],
                    [0, 0, 1, 0, 0, 0, 1, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 1, 0, 0, 0, 1, 0],
                    [0, 0, 0, 1, 0, 1, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                ])
            );
        }

        #[test]
        fn it_generates_bishop_pseudo_attacks_on_an_occupied_board() {
            assert_eq!(
                get_piece_pseudo_attacks(
                    PieceType::Bishop,
                    E4,
                    FILE_C | RANK_1,
                ),
                Bitboard::from([
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 1],
                    [0, 0, 1, 0, 0, 0, 1, 0],
                    [0, 0, 0, 1, 0, 1, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 1, 0, 1, 0, 0],
                    [0, 0, 1, 0, 0, 0, 1, 0],
                    [0, 0, 0, 0, 0, 0, 0, 1],
                ]),
            );
            assert_eq!(
                get_piece_pseudo_attacks(
                    PieceType::Bishop,
                    E4,
                    RANK_1
                        | RANK_2
                        | RANK_3
                        | RANK_4
                        | RANK_5
                        | RANK_6
                        | RANK_7
                        | RANK_8
                ),
                Bitboard::from([
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 1, 0, 1, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 1, 0, 1, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                ]),
            );
        }

        #[test]
        fn it_generates_rook_pseudo_attacks_on_an_occupied_board() {
            assert_eq!(
                get_piece_pseudo_attacks(PieceType::Rook, E4, FILE_C | RANK_1,),
                Bitboard::from([
                    [0, 0, 0, 0, 1, 0, 0, 0],
                    [0, 0, 0, 0, 1, 0, 0, 0],
                    [0, 0, 0, 0, 1, 0, 0, 0],
                    [0, 0, 0, 0, 1, 0, 0, 0],
                    [0, 0, 1, 1, 0, 1, 1, 1],
                    [0, 0, 0, 0, 1, 0, 0, 0],
                    [0, 0, 0, 0, 1, 0, 0, 0],
                    [0, 0, 0, 0, 1, 0, 0, 0],
                ]),
            );
            assert_eq!(
                get_piece_pseudo_attacks(
                    PieceType::Rook,
                    E4,
                    RANK_1
                        | RANK_2
                        | RANK_3
                        | RANK_4
                        | RANK_5
                        | RANK_6
                        | RANK_7
                        | RANK_8
                ),
                Bitboard::from([
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 1, 0, 0, 0],
                    [0, 0, 0, 1, 0, 1, 0, 0],
                    [0, 0, 0, 0, 1, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                ]),
            );
        }

        #[test]
        fn it_generates_queen_pseudo_attacks_on_an_occupied_board() {
            assert_eq!(
                get_piece_pseudo_attacks(PieceType::Queen, E4, FILE_C | RANK_1,),
                Bitboard::from([
                    [0, 0, 0, 0, 1, 0, 0, 0],
                    [0, 0, 0, 0, 1, 0, 0, 1],
                    [0, 0, 1, 0, 1, 0, 1, 0],
                    [0, 0, 0, 1, 1, 1, 0, 0],
                    [0, 0, 1, 1, 0, 1, 1, 1],
                    [0, 0, 0, 1, 1, 1, 0, 0],
                    [0, 0, 1, 0, 1, 0, 1, 0],
                    [0, 0, 0, 0, 1, 0, 0, 1],
                ]),
            );
            assert_eq!(
                get_piece_pseudo_attacks(
                    PieceType::Queen,
                    E4,
                    RANK_1
                        | RANK_2
                        | RANK_3
                        | RANK_4
                        | RANK_5
                        | RANK_6
                        | RANK_7
                        | RANK_8
                ),
                Bitboard::from([
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 1, 1, 1, 0, 0],
                    [0, 0, 0, 1, 0, 1, 0, 0],
                    [0, 0, 0, 1, 1, 1, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                ]),
            );
        }

        #[test]
        fn it_generates_bishop_pseudo_attacks_by_square() {
            assert_eq!(
                get_piece_pseudo_attacks_by_square(PieceType::Bishop, B2,),
                Bitboard::from([
                    [0, 0, 0, 0, 0, 0, 0, 1],
                    [0, 0, 0, 0, 0, 0, 1, 0],
                    [0, 0, 0, 0, 0, 1, 0, 0],
                    [0, 0, 0, 0, 1, 0, 0, 0],
                    [0, 0, 0, 1, 0, 0, 0, 0],
                    [1, 0, 1, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [1, 0, 1, 0, 0, 0, 0, 0],
                ]),
            );
        }

        #[test]
        fn it_generates_knight_pseudo_attacks_by_square() {
            assert_eq!(
                get_piece_pseudo_attacks_by_square(PieceType::Knight, B2,),
                Bitboard::from([
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [1, 0, 1, 0, 0, 0, 0, 0],
                    [0, 0, 0, 1, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 1, 0, 0, 0, 0],
                ]),
            );
        }

        #[test]
        fn it_generates_king_pseudo_attacks_by_square() {
            assert_eq!(
                get_piece_pseudo_attacks_by_square(PieceType::King, B2,),
                Bitboard::from([
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [1, 1, 1, 0, 0, 0, 0, 0],
                    [1, 0, 1, 0, 0, 0, 0, 0],
                    [1, 1, 1, 0, 0, 0, 0, 0],
                ]),
            );
        }

        #[test]
        fn it_generates_queen_pseudo_attacks_by_square() {
            assert_eq!(
                get_piece_pseudo_attacks_by_square(PieceType::Queen, B2,),
                Bitboard::from([
                    [0, 1, 0, 0, 0, 0, 0, 1],
                    [0, 1, 0, 0, 0, 0, 1, 0],
                    [0, 1, 0, 0, 0, 1, 0, 0],
                    [0, 1, 0, 0, 1, 0, 0, 0],
                    [0, 1, 0, 1, 0, 0, 0, 0],
                    [1, 1, 1, 0, 0, 0, 0, 0],
                    [1, 0, 1, 1, 1, 1, 1, 1],
                    [1, 1, 1, 0, 0, 0, 0, 0],
                ]),
            );
        }

        #[test]
        fn it_generates_rook_pseudo_attacks_by_square() {
            assert_eq!(
                get_piece_pseudo_attacks_by_square(PieceType::Rook, B2,),
                Bitboard::from([
                    [0, 1, 0, 0, 0, 0, 0, 0],
                    [0, 1, 0, 0, 0, 0, 0, 0],
                    [0, 1, 0, 0, 0, 0, 0, 0],
                    [0, 1, 0, 0, 0, 0, 0, 0],
                    [0, 1, 0, 0, 0, 0, 0, 0],
                    [0, 1, 0, 0, 0, 0, 0, 0],
                    [1, 0, 1, 1, 1, 1, 1, 1],
                    [0, 1, 0, 0, 0, 0, 0, 0],
                ]),
            );
        }
    }

    #[test]
    fn it_generates_line_bitboards() {
        // values confirmed by running and inspecting stockfish's values
        assert_eq!(LINE_BB[10][24], Bitboard(16909320));
        assert_eq!(LINE_BB[10][9], Bitboard(65280));
        assert_eq!(LINE_BB[54][11], Bitboard(0));
        assert_eq!(LINE_BB[15][16], Bitboard(0));
        assert_eq!(LINE_BB[17][21], Bitboard(16711680));
        assert_eq!(LINE_BB[13][27], Bitboard(283691315109952));
        assert_eq!(LINE_BB[60][31], Bitboard(0));
        assert_eq!(LINE_BB[39][11], Bitboard(0));
        assert_eq!(LINE_BB[61][17], Bitboard(0));
        assert_eq!(LINE_BB[49][16], Bitboard(0));
        assert_eq!(LINE_BB[26][3], Bitboard(0));
        assert_eq!(LINE_BB[7][2], Bitboard(255));
        assert_eq!(LINE_BB[29][60], Bitboard(0));
        assert_eq!(LINE_BB[44][55], Bitboard(0));
        assert_eq!(LINE_BB[36][1], Bitboard(0));
        assert_eq!(LINE_BB[35][19], Bitboard(578721382704613384));
        assert_eq!(LINE_BB[49][46], Bitboard(0));
        assert_eq!(LINE_BB[62][22], Bitboard(4629771061636907072));
        assert_eq!(LINE_BB[33][53], Bitboard(0));
        assert_eq!(LINE_BB[14][44], Bitboard(0));
    }

    #[test]
    fn it_generates_between_bitboards() {
        // values confirmed by running and inspecting stockfish's values
        assert_eq!(BETWEEN_BB[10][24], Bitboard(16908288));
        assert_eq!(BETWEEN_BB[10][9], Bitboard(512));
        assert_eq!(BETWEEN_BB[54][11], Bitboard(2048));
        assert_eq!(BETWEEN_BB[15][16], Bitboard(65536));
        assert_eq!(BETWEEN_BB[17][21], Bitboard(3932160));
        assert_eq!(BETWEEN_BB[13][27], Bitboard(135266304));
        assert_eq!(BETWEEN_BB[60][31], Bitboard(2147483648));
        assert_eq!(BETWEEN_BB[39][11], Bitboard(2048));
        assert_eq!(BETWEEN_BB[61][17], Bitboard(131072));
        assert_eq!(BETWEEN_BB[49][16], Bitboard(65536));
        assert_eq!(BETWEEN_BB[26][3], Bitboard(8));
        assert_eq!(BETWEEN_BB[7][2], Bitboard(124));
        assert_eq!(BETWEEN_BB[29][60], Bitboard(1152921504606846976));
        assert_eq!(BETWEEN_BB[44][55], Bitboard(36028797018963968));
        assert_eq!(BETWEEN_BB[36][1], Bitboard(2));
        assert_eq!(BETWEEN_BB[35][19], Bitboard(134742016));
        assert_eq!(BETWEEN_BB[49][46], Bitboard(70368744177664));
        assert_eq!(BETWEEN_BB[62][22], Bitboard(18085043209502720));
        assert_eq!(BETWEEN_BB[33][53], Bitboard(9007199254740992));
        assert_eq!(BETWEEN_BB[14][44], Bitboard(17592186044416));
    }

    #[test]
    fn if_checks_for_more_than_one_bit_set() {
        assert!(!is_more_than_one(Bitboard(1 << 10)));
        assert!(!is_more_than_one(Bitboard(1 << 23)));
        assert!(!is_more_than_one(Bitboard(1)));
        assert!(!is_more_than_one(Bitboard(0)));
        assert!(is_more_than_one(Bitboard(52345)));
        assert!(is_more_than_one(Bitboard(8342384342)));
        assert!(is_more_than_one(Bitboard(7777777777)));
        assert!(is_more_than_one(Bitboard(1 << 10 | 1 << 23)));

        for i in 0..1000 {
            assert_eq!(is_more_than_one(Bitboard(i)), i.count_ones() > 1);
        }
    }

    #[test]
    fn it_calculates_the_index_of_the_least_significant_bit() {
        assert_eq!(lsb(Bitboard(1 << 51)), 51);
        assert_eq!(lsb(Bitboard(1 << 10)), 10);
        assert_eq!(lsb(Bitboard(1 << 23)), 23);
        assert_eq!(lsb(Bitboard(1)), 0);
        assert_eq!(lsb(Bitboard(52345)), 0);
        assert_eq!(lsb(Bitboard(8342384342)), 1);
        assert_eq!(lsb(Bitboard(7777777777)), 0);
        assert_eq!(lsb(Bitboard(1 << 10 | 1 << 23)), 10);
    }

    #[test]
    #[should_panic]
    #[cfg(debug_assertions)]
    fn it_panics_when_checking_for_lsb_on_0_bitboard() {
        lsb(Bitboard(0));
    }

    #[test]
    fn it_calculates_the_index_of_the_most_significant_bit() {
        assert_eq!(msb(Bitboard(1 << 51)), 51);
        assert_eq!(msb(Bitboard(1 << 10)), 10);
        assert_eq!(msb(Bitboard(1 << 23)), 23);
        assert_eq!(msb(Bitboard(1)), 0);
        assert_eq!(msb(Bitboard(52345)), 15);
        assert_eq!(msb(Bitboard(8342384342)), 32);
        assert_eq!(msb(Bitboard(7777777777)), 32);
        assert_eq!(msb(Bitboard(1 << 10 | 1 << 23)), 23);
    }

    #[test]
    #[should_panic]
    #[cfg(debug_assertions)]
    fn it_panics_when_checking_for_msb_on_0_bitboard() {
        msb(Bitboard(0));
    }

    #[test]
    #[rustfmt::skip]
    fn it_calculates_the_bitboard_of_the_least_significant_square() {
        assert_eq!(least_significant_square_bb(Bitboard(1 << 51)), Bitboard(2251799813685248));
        assert_eq!(least_significant_square_bb(Bitboard(1 << 10)), Bitboard(1024));
        assert_eq!(least_significant_square_bb(Bitboard(1 << 23)), Bitboard(8388608));
        assert_eq!(least_significant_square_bb(Bitboard(1)), Bitboard(1));
        assert_eq!(least_significant_square_bb(Bitboard(52345)), Bitboard(1));
        assert_eq!(least_significant_square_bb(Bitboard(8342384342)), Bitboard(2));
        assert_eq!(least_significant_square_bb(Bitboard(7777777777)), Bitboard(1));
        assert_eq!(least_significant_square_bb(Bitboard(1 << 10 | 1 << 23)), Bitboard(1024));
    }

    #[test]
    #[should_panic]
    #[cfg(debug_assertions)]
    fn it_panics_when_checking_for_least_significant_square_on_0_bitboard() {
        least_significant_square_bb(Bitboard(0));
    }

    #[test]
    fn it_pops_and_returns_the_least_significant_bit() {
        let mut b = Bitboard(534253458324823408);
        assert_eq!(pop_lsb(&mut b), 4);
        assert_eq!(b, Bitboard(534253458324823392));
    }
}
