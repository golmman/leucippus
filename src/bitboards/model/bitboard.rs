use std::fmt::Debug;
use std::ops::BitAnd;

#[derive(PartialEq)]
pub struct Bitboard(u64);

impl BitAnd for Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl From<[[u8; 8]; 8]> for Bitboard {
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
    fn it_xx() {
        let a = [
            [1, 0, 0, 0, 0, 0, 0, 0],
            [1, 1, 0, 0, 0, 0, 0, 0],
            [1, 1, 1, 1, 1, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 0, 0, 0],
            [1, 1, 0, 0, 0, 1, 1, 1],
        ];

        let one = Bitboard::from(a);
        println!("{one:?}");
        println!("{}", one.0);

        let x = format!("{one:?}");
        let mut y = String::new();
        y.push_str("10000000\n");
        y.push_str("11000000\n");
        y.push_str("11111000\n");
        y.push_str("00000000\n");
        y.push_str("00000000\n");
        y.push_str("10000000\n");
        y.push_str("10000000\n");
        y.push_str("11000111\n");

        assert_eq!(x , y);
    }
}
