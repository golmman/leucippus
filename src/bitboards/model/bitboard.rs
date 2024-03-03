use std::fmt::Debug;
use std::ops::BitAnd;
use std::ops::BitOr;
use std::ops::Shl;

#[derive(Clone, Copy, PartialEq)]
pub struct Bitboard(pub u64);

impl BitAnd for Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitOr for Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl Shl<u8> for Bitboard {
    type Output = Bitboard;

    fn shl(self, rhs: u8) -> Self::Output {
        Bitboard(self.0 << rhs)
    }
}

impl From<[[u8; 8]; 8]> for Bitboard {
    /// Note that the input is flipped so it is easier to debug positions
    fn from(board: [[u8; 8]; 8]) -> Self {
        let mut bb = 0u64;

        for y in 0..8 {
            let rank = 7 - y;
            for file in 0..8 {
                let i = 8 * rank + file;
                if board[y][file] != 0 {
                    bb |= 1u64 << i;
                }
            }
        }

        Bitboard(bb)
    }
}

impl From<&str> for Bitboard {
    /// Note that the input is flipped so it is easier to debug positions
    fn from(board: &str) -> Self {
        let mut bb = 0u64;
        let mut i = 0;

        for ch in board.chars() {
            if i >= 64 {
                break;
            }
            if ch == '0' {
                i += 1;
            } else if ch == '1' {
                let x = i % 8;
                let y = i / 8;
                let j = 8 * (7 - y) + x;
                bb |= 1u64 << j;
                i += 1;
            }
        }

        Bitboard(bb)
    }
}

impl From<String> for Bitboard {
    fn from(board: String) -> Self {
        Bitboard::from(board.as_str())
    }
}

impl Debug for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        for y in 0..8 {
            let rank = 7 - y;
            for file in 0..8 {
                let i = 8 * rank + file;
                if self.0 & 1 << i == 0 {
                    result.push_str("0");
                } else {
                    result.push_str("1");
                }
            }
            result.push_str("\n");
        }

        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_reads_a_bitboard_from_a_byte_array() {
        let bb = Bitboard::from([
            [1, 0, 0, 0, 0, 0, 0, 0],
            [1, 1, 0, 0, 0, 0, 0, 0],
            [1, 1, 1, 1, 1, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 0, 0, 0],
            [1, 1, 0, 0, 0, 1, 1, 1],
        ]);
        let left = format!("{bb:?}");

        let mut right = String::new();
        right.push_str("10000000\n");
        right.push_str("11000000\n");
        right.push_str("11111000\n");
        right.push_str("00000000\n");
        right.push_str("00000000\n");
        right.push_str("10000000\n");
        right.push_str("10000000\n");
        right.push_str("11000111\n");

        assert_eq!(left, right);
    }

    #[test]
    fn it_reads_a_bitboard_from_a_string() {
        let mut s = String::new();
        s.push_str("10000000");
        s.push_str("11000000");
        s.push_str("11111000");
        s.push_str("00000000");
        s.push_str("00000000");
        s.push_str("10000000");
        s.push_str("10000000");
        s.push_str("11000111");
        let bb = Bitboard::from(s);
        let left = format!("{bb:?}");

        let mut right = String::new();
        right.push_str("10000000\n");
        right.push_str("11000000\n");
        right.push_str("11111000\n");
        right.push_str("00000000\n");
        right.push_str("00000000\n");
        right.push_str("10000000\n");
        right.push_str("10000000\n");
        right.push_str("11000111\n");

        assert_eq!(left, right);
    }

    #[test]
    fn it_reads_a_bitboard_from_a_string_and_ignores_other_chars() {
        let mut s = String::new();
        s.push_str("a 100  00000\n");
        s.push_str("  110  00000xxxx");
        s.push_str("  111  11000\n\n");
        s.push_str("  000cc00000   ");
        s.push_str("  000  00000  yyy");
        s.push_str("b 100  00000---");
        s.push_str("  100  000002222");
        s.push_str("  110  00111 hello world1111");
        let bb = Bitboard::from(s);
        let left = format!("{bb:?}");

        let mut right = String::new();
        right.push_str("10000000\n");
        right.push_str("11000000\n");
        right.push_str("11111000\n");
        right.push_str("00000000\n");
        right.push_str("00000000\n");
        right.push_str("10000000\n");
        right.push_str("10000000\n");
        right.push_str("11000111\n");

        assert_eq!(left, right);
    }
}
