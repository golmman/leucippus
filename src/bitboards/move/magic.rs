use crate::bitboards::model::bitboard::Bitboard;
use crate::bitboards::r#move::attacks::file_bb_from_square;
use crate::bitboards::r#move::attacks::pext;
use crate::bitboards::r#move::attacks::rank_bb_from_square;
use crate::bitboards::r#move::attacks::sliding_attack;
use crate::bitboards::r#move::attacks::sparse_rand;
use crate::bitboards::r#move::attacks::Magic;
use crate::bitboards::r#move::attacks::FILE_A;
use crate::bitboards::r#move::attacks::FILE_H;
use crate::bitboards::r#move::attacks::HAS_PEXT;
use crate::bitboards::r#move::attacks::IS_64_BIT;
use crate::bitboards::r#move::attacks::RANK_1;
use crate::bitboards::r#move::attacks::RANK_8;
use crate::bitboards::r#move::attacks::RANK_OF;
use crate::model::piece_type::PieceType;
use crate::model::types::square_names::A1;

use super::attacks::BishopTable;

//#[cfg(not(debug_assertions))]
//#[allow(long_running_const_eval)]
//pub const BISHOP_TABLE: BishopTable = {
//    let pt = PieceType::Bishop;
//
//    const MAGIC_INIT: Magic = Magic {
//        mask: Bitboard(0),
//        magic: Bitboard(0),
//        attacks: 0,
//        shift: 0,
//    };
//    let mut bt = BishopTable {
//        magics: [MAGIC_INIT; 64],
//        table: [Bitboard(0); 0x1480],
//    };
//
//    let seeds_32 = [8977, 44560, 54343, 38998, 5731, 95205, 104912, 17020];
//    let seeds_64 = [728, 10316, 55013, 32803, 12281, 15100, 16645, 255];
//    let mut occupancy = [Bitboard(0); 4096];
//    let mut reference = [Bitboard(0); 4096];
//    let mut edges = Bitboard(0);
//    let mut b = Bitboard(0);
//    let mut epoch = [0i32; 4096];
//    let mut cnt = 0;
//    let mut size = 0;
//
//    let mut s = 0;
//    while s < 64 {
//        let si = s as usize;
//        edges = Bitboard(
//            ((RANK_1.0 | RANK_8.0) & !rank_bb_from_square(s).0)
//                | ((FILE_A.0 | FILE_H.0) & !file_bb_from_square(s).0),
//        );
//
//        bt.magics[si].mask =
//            Bitboard(sliding_attack(pt, s, Bitboard(0)).0 & !edges.0);
//
//        bt.magics[si].shift = if IS_64_BIT { 64 } else { 32 }
//            - bt.magics[si].mask.0.count_ones() as u8;
//
//        bt.magics[si].attacks = if s == A1 {
//            0
//        } else {
//            bt.magics[si - 1].attacks + size
//        };
//
//        //println!("{} {} {}", bt.magics[si].mask.0, bt.magics[si].shift, bt.magics[si].attacks);
//
//        b = Bitboard(0);
//        size = 0;
//        loop {
//            occupancy[size] = b;
//            reference[size] = sliding_attack(pt, s, b);
//
//            //println!("{} {}", occupancy[size].0, reference[size].0);
//
//            if HAS_PEXT {
//                let a = bt.magics[si].attacks;
//                let p = pext(b, bt.magics[si].mask) as usize;
//                bt.table[a + p] = reference[size];
//            }
//
//            size += 1;
//            b = Bitboard(
//                (b.0 as i64 - bt.magics[si].mask.0 as i64) as u64
//                    & bt.magics[si].mask.0,
//            );
//
//            if b.0 == 0 {
//                break;
//            }
//        }
//
//        if HAS_PEXT {
//            s += 1;
//            continue;
//        }
//
//        let mut seed = if IS_64_BIT {
//            seeds_64[RANK_OF[s as usize] as usize]
//        } else {
//            seeds_32[RANK_OF[s as usize] as usize]
//        };
//
//        //println!("seed: {}", seed);
//
//        //return bt;
//
//        //s += 1;
//        //continue;
//
//        let mut i = 0;
//        while i < size {
//            bt.magics[si].magic = Bitboard(0);
//            loop {
//                let (r, seed0) = sparse_rand(seed);
//                seed = seed0;
//                bt.magics[si].magic = r;
//                //println!("rand: {}", bt.magics[si].magic.0);
//
//                let multi =
//                    bt.magics[si].magic.0.wrapping_mul(bt.magics[si].mask.0);
//                if (multi >> 56).count_ones() >= 6 {
//                    break;
//                }
//            }
//
//            cnt += 1;
//            i = 0;
//            while i < size {
//                let idx = bt.magics[si].index(occupancy[i]);
//
//                //println!(
//                //    "inner: {} {} {} {} {} --- {} {} {}",
//                //     i, cnt, idx, epoch[idx], occupancy[i].0,
//                //     bt.magics[si].mask.0, bt.magics[si].magic.0, bt.magics[si].shift,
//                //);
//
//                if epoch[idx] < cnt {
//                    epoch[idx] = cnt;
//                    bt.table[bt.magics[si].attacks + idx] = reference[i];
//                } else if bt.table[bt.magics[si].attacks + idx].0
//                    != reference[i].0
//                {
//                    //i += 1;
//                    break;
//                }
//
//                i += 1;
//            }
//
//            //println!("outer: {} {}", i, cnt);
//
//            //
//            if cnt > 4000000 {
//                //return bt;
//            }
//        }
//
//        s += 1;
//    }
//
//    bt
//};
