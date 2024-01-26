use std::collections::HashSet;
use std::hash::Hash;

use super::squares::Squares;
use super::types::SquareIndex;

#[derive(Clone, Debug)]
pub struct BoardPieces {
    pub active_bishops: Vec<SquareIndex>,
    pub active_kings: Vec<SquareIndex>,
    pub active_knights: Vec<SquareIndex>,
    pub active_pawns: Vec<SquareIndex>,
    pub active_queens: Vec<SquareIndex>,
    pub active_rooks: Vec<SquareIndex>,
    pub squares: Squares,
}

impl PartialEq for BoardPieces {
    fn eq(&self, other: &Self) -> bool {
        make_set(&self.active_bishops) == make_set(&other.active_bishops)
            && make_set(&self.active_kings) == make_set(&other.active_kings)
            && make_set(&self.active_knights) == make_set(&other.active_knights)
            && make_set(&self.active_pawns) == make_set(&other.active_pawns)
            && make_set(&self.active_queens) == make_set(&other.active_queens)
            && make_set(&self.active_rooks) == make_set(&other.active_rooks)
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
            active_bishops: vec![1, 2, 3],
            active_kings: Vec::new(),
            active_knights: Vec::new(),
            active_pawns: Vec::new(),
            active_queens: Vec::new(),
            active_rooks: Vec::new(),
            squares: Squares::new([None; 64]),
        };

        let right = BoardPieces {
            active_bishops: vec![1, 2, 3, 4],
            active_kings: Vec::new(),
            active_knights: Vec::new(),
            active_pawns: Vec::new(),
            active_queens: Vec::new(),
            active_rooks: Vec::new(),
            squares: Squares::new([None; 64]),
        };

        assert!(left != right);
    }

    #[test]
    fn it_determines_the_inequality_of_board_pieces_when_elements_dont_match() {
        let left = BoardPieces {
            active_bishops: vec![1, 2, 3],
            active_kings: Vec::new(),
            active_knights: Vec::new(),
            active_pawns: Vec::new(),
            active_queens: Vec::new(),
            active_rooks: Vec::new(),
            squares: Squares::new([None; 64]),
        };

        let right = BoardPieces {
            active_bishops: vec![1, 2, 4],
            active_kings: Vec::new(),
            active_knights: Vec::new(),
            active_pawns: Vec::new(),
            active_queens: Vec::new(),
            active_rooks: Vec::new(),
            squares: Squares::new([None; 64]),
        };

        assert!(left != right);
    }

    #[test]
    fn it_determines_the_equality_of_board_pieces_with_mixed_order() {
        let left = BoardPieces {
            active_bishops: vec![1, 2, 3, 4],
            active_kings: vec![1, 2],
            active_knights: vec![10, 100, 99],
            active_pawns: vec![7, 8, 9],
            active_queens: vec![2, 3, 4, 5, 6, 7],
            active_rooks: vec![1],
            squares: Squares::new([None; 64]),
        };

        let right = BoardPieces {
            active_bishops: vec![2, 3, 4, 1],
            active_kings: vec![2, 1],
            active_knights: vec![100, 99, 10],
            active_pawns: vec![9, 8, 7],
            active_queens: vec![3, 5, 7, 2, 4, 6],
            active_rooks: vec![1],
            squares: Squares::new([None; 64]),
        };

        assert_eq!(left, right);
    }
}
