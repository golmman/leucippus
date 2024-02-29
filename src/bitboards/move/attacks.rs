// port of stockfishs bitboard.cpp

use crate::bitboards::model::bitboard::Bitboard;
use crate::model::piece_type::PieceType;
use crate::model::types::square_names::*;
use crate::model::types::SquareIndex;

const fn max_u8(a: u8, b: u8) -> u8 {
    if a > b {
        return a;
    } else {
        return b;
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    North = 8,
    East = 1,
    South = -8,
    West = -1,
    NorthEast = 9,
    SouthEast = -7,
    SouthWest = -9,
    NorthWest = 7,
}

struct Magic {
    mask: Bitboard,
    magic: Bitboard,
    attacks: usize,
    shift: u8,
}

impl Magic {
    #[cfg(all(target_arch = "x86_64", target_feature = "bmi2"))]
    const fn index(&self, occupied: Bitboard) -> usize {
        core::arch::x86_64::_pext_u64(occupied.0, self.mask.0) as usize
    }

    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
    const fn index(&self, occupied: Bitboard) -> usize {
        (((occupied.0 & self.mask.0) * self.magic.0) >> self.shift) as usize
    }

    // TODO: untested
    #[cfg(target_arch = "x86")]
    const fn index(&self, occupied: Bitboard) -> usize {
        let lo = occupied.0 & self.mask.0;
        let hi = occupied.0 >> 32 & self.mask.0 >> 32;
        lo * self.magic.0 ^ hi * (self.magic.0 >> 32) >> shift
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

const POP_CNT_16: [u8; u16::MAX as usize + 1] = {
    let mut pop_cnt_16 = [0; u16::MAX as usize + 1];
    let mut i = 0;
    while i <= u16::MAX as usize {
        pop_cnt_16[i] = (i as u16).count_ones() as u8;
        i += 1;
    }
    pop_cnt_16
};

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

const SQUARE_DISTANCE: [[SquareIndex; 64]; 64] = {
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

// TODO
//extern Bitboard BetweenBB[SQUARE_NB][SQUARE_NB];
//extern Bitboard LineBB[SQUARE_NB][SQUARE_NB];
//extern Bitboard PseudoAttacks[PIECE_TYPE_NB][SQUARE_NB];
//extern Bitboard PawnAttacks[COLOR_NB][SQUARE_NB];

const SQUARE: [Bitboard; 64] = {
    let mut square = [Bitboard(0); 64];
    let mut s = 0;
    while s < 64 {
        square[s].0 = 1 << s;
        s += 1;
    }
    square
};

// corresponds to 'is_ok' function in stockfishs code
const fn is_valid_square_index(s: SquareIndex) -> bool {
    s >= A1 && s <= H8
}

// TODO: why is the distance <= 2 check necessary?
const fn safe_destination(s: SquareIndex, step: i32) -> Bitboard {
    let to = s as i32 + step;
    if is_valid_square_index(to as SquareIndex)
        && SQUARE_DISTANCE[s as usize][to as usize] <= 2
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_counts_ones_a_u16_bit_representation() {
        assert_eq!(POP_CNT_16[0b1111111111111111 as usize], 16);
        assert_eq!(POP_CNT_16[0b1111110111111111 as usize], 15);
        assert_eq!(POP_CNT_16[0b1110111111110111 as usize], 14);
        assert_eq!(POP_CNT_16[0b1111011011101111 as usize], 13);
        assert_eq!(POP_CNT_16[0b1111100001111111 as usize], 12);
        assert_eq!(POP_CNT_16[0b110011 as usize], 4);
        assert_eq!(POP_CNT_16[0], 0);
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
        assert!(is_valid_square_index(A1));
        assert!(is_valid_square_index(B4));
        assert!(is_valid_square_index(F3));
        assert!(is_valid_square_index(H8));

        assert!(!is_valid_square_index(111));
        assert!(!is_valid_square_index(64));
        assert!(!is_valid_square_index(-4i32 as SquareIndex));
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
}
