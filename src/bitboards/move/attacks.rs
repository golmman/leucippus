// port of stockfishs bitboard.cpp

use crate::bitboards::model::bitboard::Bitboard;
use crate::model::types::SquareIndex;

//pub const fn get_square_distance(s1: SquareIndex, s2: SquareIndex) -> u8 {
//    SQUARE_DISTANCES[s1 as usize][s2 as usize]
//}
//
//const SQUARE_DISTANCES: [[u8; 64]; 64] = {
//    let square_distances = [[0; 64]; 64];
//    square_distances
//};

const fn max_u8(a: u8, b: u8) -> u8 {
    if a > b {
        return a;
    } else {
        return b;
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
    let mut f2 = 0i32;
    while f1 < 8 {
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
    let mut r2 = 0i32;
    while r1 < 8 {
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
    let mut s2 = 0;
    while s1 < 64 {
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
}
