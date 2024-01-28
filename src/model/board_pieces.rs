use std::collections::HashSet;
use std::hash::Hash;

use super::squares::Squares;
use super::types::SquareIndex;

#[derive(Clone, Debug)]
pub struct BoardPieces {
    pub our_bishops: Vec<SquareIndex>,
    pub our_kings: Vec<SquareIndex>,
    pub our_knights: Vec<SquareIndex>,
    pub our_pawns: Vec<SquareIndex>,
    pub our_queens: Vec<SquareIndex>,
    pub our_rooks: Vec<SquareIndex>,
    pub squares: Squares,
}

impl PartialEq for BoardPieces {
    fn eq(&self, other: &Self) -> bool {
        make_set(&self.our_bishops) == make_set(&other.our_bishops)
            && make_set(&self.our_kings) == make_set(&other.our_kings)
            && make_set(&self.our_knights) == make_set(&other.our_knights)
            && make_set(&self.our_pawns) == make_set(&other.our_pawns)
            && make_set(&self.our_queens) == make_set(&other.our_queens)
            && make_set(&self.our_rooks) == make_set(&other.our_rooks)
            && self.squares == other.squares
    }
}

fn make_set<T: Eq + Hash + Clone>(v: &Vec<T>) -> HashSet<T> {
    let set: HashSet<_> = v.iter().cloned().collect();
    set
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_determines_the_inequality_of_board_pieces_when_vec_sizes_dont_match()
    {
        let left = BoardPieces {
            our_bishops: vec![1, 2, 3],
            our_kings: Vec::new(),
            our_knights: Vec::new(),
            our_pawns: Vec::new(),
            our_queens: Vec::new(),
            our_rooks: Vec::new(),
            squares: Squares::new([None; 64]),
        };

        let right = BoardPieces {
            our_bishops: vec![1, 2, 3, 4],
            our_kings: Vec::new(),
            our_knights: Vec::new(),
            our_pawns: Vec::new(),
            our_queens: Vec::new(),
            our_rooks: Vec::new(),
            squares: Squares::new([None; 64]),
        };

        assert!(left != right);
    }

    #[test]
    fn it_determines_the_inequality_of_board_pieces_when_elements_dont_match() {
        let left = BoardPieces {
            our_bishops: vec![1, 2, 3],
            our_kings: Vec::new(),
            our_knights: Vec::new(),
            our_pawns: Vec::new(),
            our_queens: Vec::new(),
            our_rooks: Vec::new(),
            squares: Squares::new([None; 64]),
        };

        let right = BoardPieces {
            our_bishops: vec![1, 2, 4],
            our_kings: Vec::new(),
            our_knights: Vec::new(),
            our_pawns: Vec::new(),
            our_queens: Vec::new(),
            our_rooks: Vec::new(),
            squares: Squares::new([None; 64]),
        };

        assert!(left != right);
    }

    #[test]
    fn it_determines_the_equality_of_board_pieces_with_mixed_order() {
        let left = BoardPieces {
            our_bishops: vec![1, 2, 3, 4],
            our_kings: vec![1, 2],
            our_knights: vec![10, 100, 99],
            our_pawns: vec![7, 8, 9],
            our_queens: vec![2, 3, 4, 5, 6, 7],
            our_rooks: vec![1],
            squares: Squares::new([None; 64]),
        };

        let right = BoardPieces {
            our_bishops: vec![2, 3, 4, 1],
            our_kings: vec![2, 1],
            our_knights: vec![100, 99, 10],
            our_pawns: vec![9, 8, 7],
            our_queens: vec![3, 5, 7, 2, 4, 6],
            our_rooks: vec![1],
            squares: Squares::new([None; 64]),
        };

        assert_eq!(left, right);
    }
}
