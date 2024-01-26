use std::fmt::Debug;

use super::color::Color;
use super::piece::Piece;
use super::types::SquareIndex;

#[derive(Clone, PartialEq)]
pub struct Squares {
    pub data: [Option<Piece>; 64],
}

impl Squares {
    /// assumes data in 'natural' order, i.e.
    /// a1 is index 0 and h8 is index 63
    pub fn new(data: [Option<Piece>; 64]) -> Self {
        Self { data }
    }

    /// flips the board along the x axis which makes it easier to create e.g.
    /// test data
    pub fn flipped(flipped_data: [Option<Piece>; 64]) -> Self {
        let mut data = [None; 64];

        for y in 0..8 {
            let rank = 7 - y;
            for file in 0..8 {
                let i = 8 * rank + file;
                let j = 8 * y + file;
                data[i] = flipped_data[j];
            }
        }

        Self { data }
    }
}

impl Debug for Squares {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        result.push_str("\n");

        for y in 0..8 {
            let rank = 7 - y;
            for file in 0..8 {
                let i = 8 * rank + file;
                match self.data[i] {
                    Some(Piece::BlackBishop) => result.push_str("b"),
                    Some(Piece::BlackKing) => result.push_str("k"),
                    Some(Piece::BlackKnight) => result.push_str("n"),
                    Some(Piece::BlackPawn) => result.push_str("p"),
                    Some(Piece::BlackQueen) => result.push_str("q"),
                    Some(Piece::BlackRook) => result.push_str("r"),
                    Some(Piece::WhiteBishop) => result.push_str("B"),
                    Some(Piece::WhiteKing) => result.push_str("K"),
                    Some(Piece::WhiteKnight) => result.push_str("N"),
                    Some(Piece::WhitePawn) => result.push_str("P"),
                    Some(Piece::WhiteQueen) => result.push_str("Q"),
                    Some(Piece::WhiteRook) => result.push_str("R"),
                    None => result.push_str("-"),
                }
            }
            result.push_str("\n");
        }

        write!(f, "{}", result)
    }
}
