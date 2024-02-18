use crate::bitboards::model::bitboard::Bitboard;
use crate::model::board_castle::BoardCastle;
use crate::model::color::Color;
use crate::model::piece::Piece;
use crate::model::types::SquareIndex;

enum PositionColor {
    Black = 0,
    White = 1,
}

enum PositionPiece {
    Bishop = 0,
    King = 1,
    Knight = 2,
    Pawn = 3,
    Queen = 4,
    Rook = 5,
}

type Board = [Option<Piece>; 64];

pub struct Position {
    pub board: Board,
    pub castling: BoardCastle,
    pub draw_by_repetition: bool,
    pub en_passant: Option<SquareIndex>,
    pub fullmove: usize,
    pub halfmove: usize,
    pub our_color: Color,
    pieces_by_color: [Bitboard; 2],
    pieces_by_type: [Bitboard; 6],
}

impl Position {
    pub fn from_fen(fen: &str) -> Position {
        let board = Self::get_board_from_fen(fen);
        let castling = Self::get_casting_from_fen(fen);
        let draw_by_repetition = false;
        let en_passant = Self::get_en_passant_from_fen(fen);
        let fullmove = Self::get_fullmove_from_fen(fen);
        let halfmove = Self::get_halfmove_from_fen(fen);
        let our_color = Self::get_our_color_from_fen(fen);
        let pieces_by_color = Self::get_pieces_by_color_from_board(&board);
        let pieces_by_type = Self::get_pieces_by_type_from_board(&board);

        Position {
            board,
            castling,
            draw_by_repetition,
            en_passant,
            fullmove,
            halfmove,
            our_color,
            pieces_by_color,
            pieces_by_type,
        }
    }

    pub fn black(&self) -> Bitboard {
        self.pieces_by_color[PositionColor::Black as usize]
    }

    pub fn white(&self) -> Bitboard {
        self.pieces_by_color[PositionColor::White as usize]
    }

    pub fn bishops(&self) -> Bitboard {
        self.pieces_by_type[PositionPiece::Bishop as usize]
    }

    pub fn kings(&self) -> Bitboard {
        self.pieces_by_type[PositionPiece::King as usize]
    }

    pub fn knights(&self) -> Bitboard {
        self.pieces_by_type[PositionPiece::Knight as usize]
    }

    pub fn pawns(&self) -> Bitboard {
        self.pieces_by_type[PositionPiece::Pawn as usize]
    }

    pub fn queens(&self) -> Bitboard {
        self.pieces_by_type[PositionPiece::Queen as usize]
    }

    pub fn rooks(&self) -> Bitboard {
        self.pieces_by_type[PositionPiece::Rook as usize]
    }

    fn get_board_from_fen(fen: &str) -> Board {
        let fen_split: Vec<_> = fen.split(' ').collect();
        let fen_pieces_split = fen_split[0].split('/');
        let mut board = [None; 64];

        let mut y = 8;

        for row in fen_pieces_split {
            y -= 1;
            let mut x = 0;
            for col in row.bytes() {
                let i = 8 * y + x;

                match col {
                    b'b' => board[i] = Some(Piece::BlackBishop),
                    b'k' => board[i] = Some(Piece::BlackKing),
                    b'n' => board[i] = Some(Piece::BlackKnight),
                    b'p' => board[i] = Some(Piece::BlackPawn),
                    b'q' => board[i] = Some(Piece::BlackQueen),
                    b'r' => board[i] = Some(Piece::BlackRook),
                    b'B' => board[i] = Some(Piece::WhiteBishop),
                    b'K' => board[i] = Some(Piece::WhiteKing),
                    b'N' => board[i] = Some(Piece::WhiteKnight),
                    b'P' => board[i] = Some(Piece::WhitePawn),
                    b'Q' => board[i] = Some(Piece::WhiteQueen),
                    b'R' => board[i] = Some(Piece::WhiteRook),
                    b'1'..=b'8' => x += (col - b'1') as usize,
                    _ => panic!(
                        "Invalid FEN: numbers in piece data must be 1 to 8"
                    ),
                }

                x += 1;
                if x >= 8 {
                    break;
                }
            }
        }

        board
    }

    fn get_casting_from_fen(fen: &str) -> BoardCastle {
        let fen_split: Vec<_> = fen.split(' ').collect();
        let fen_castle = fen_split[2];

        BoardCastle {
            black_long: fen_castle.contains("q"),
            black_short: fen_castle.contains("k"),
            white_long: fen_castle.contains("Q"),
            white_short: fen_castle.contains("K"),
        }
    }

    fn get_en_passant_from_fen(fen: &str) -> Option<SquareIndex> {
        let fen_split: Vec<_> = fen.split(' ').collect();
        let fen_en_passant = fen_split[3];

        if fen_en_passant == "-" {
            return None;
        }

        let fen_en_passant_bytes: Vec<u8> = fen_en_passant.bytes().collect();
        let file = fen_en_passant_bytes[0] - b'a';
        let rank = fen_en_passant_bytes[1] - b'1';

        Some(8 * rank + file)
    }

    fn get_fullmove_from_fen(fen: &str) -> usize {
        let fen_split: Vec<_> = fen.split(' ').collect();
        usize::from_str_radix(fen_split[5], 10).unwrap()
    }

    fn get_halfmove_from_fen(fen: &str) -> usize {
        let fen_split: Vec<_> = fen.split(' ').collect();
        usize::from_str_radix(fen_split[4], 10).unwrap()
    }

    fn get_our_color_from_fen(fen: &str) -> Color {
        let fen_split: Vec<_> = fen.split(' ').collect();

        match fen_split[1] {
            "b" => Color::Black,
            "w" => Color::White,
            _ => panic!("Invalid FEN: active color must be 'b' or 'w'"),
        }
    }

    fn get_pieces_by_color_from_board(board: &Board) -> [Bitboard; 2] {
        let mut black = Bitboard(0);
        let mut white = Bitboard(0);

        for i in 0..64 {
            if let Some(piece) = board[i] {
                if piece.is_black() {
                    black.0 |= 1 << i;
                } else {
                    white.0 |= 1 << i;
                }
            }
        }

        [black, white]
    }

    fn get_pieces_by_type_from_board(board: &Board) -> [Bitboard; 6] {
        let mut bishops = Bitboard(0);
        let mut kings = Bitboard(0);
        let mut knights = Bitboard(0);
        let mut pawns = Bitboard(0);
        let mut queens = Bitboard(0);
        let mut rooks = Bitboard(0);

        for i in 0..64 {
            match board[i] {
                Some(Piece::BlackBishop) | Some(Piece::WhiteBishop) => {
                    bishops.0 |= 1 << i
                }
                Some(Piece::BlackKing) | Some(Piece::WhiteKing) => {
                    kings.0 |= 1 << i
                }
                Some(Piece::BlackKnight) | Some(Piece::WhiteKnight) => {
                    knights.0 |= 1 << i
                }
                Some(Piece::BlackPawn) | Some(Piece::WhitePawn) => {
                    pawns.0 |= 1 << i
                }
                Some(Piece::BlackQueen) | Some(Piece::WhiteQueen) => {
                    queens.0 |= 1 << i
                }
                Some(Piece::BlackRook) | Some(Piece::WhiteRook) => {
                    rooks.0 |= 1 << i
                }
                _ => {}
            }
        }

        [bishops, kings, knights, pawns, queens, rooks]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const NP: Option<Piece> = None;
    const BB: Option<Piece> = Some(Piece::BlackBishop);
    const BK: Option<Piece> = Some(Piece::BlackKing);
    const BN: Option<Piece> = Some(Piece::BlackKnight);
    const BP: Option<Piece> = Some(Piece::BlackPawn);
    const BQ: Option<Piece> = Some(Piece::BlackQueen);
    const BR: Option<Piece> = Some(Piece::BlackRook);
    const WB: Option<Piece> = Some(Piece::WhiteBishop);
    const WK: Option<Piece> = Some(Piece::WhiteKing);
    const WN: Option<Piece> = Some(Piece::WhiteKnight);
    const WP: Option<Piece> = Some(Piece::WhitePawn);
    const WQ: Option<Piece> = Some(Piece::WhiteQueen);
    const WR: Option<Piece> = Some(Piece::WhiteRook);

    pub fn flip(flipped_board: Board) -> Board {
        let mut board = [None; 64];

        for y in 0..8 {
            let rank = 7 - y;
            for file in 0..8 {
                let i = 8 * rank + file;
                let j = 8 * y + file;
                board[i] = flipped_board[j];
            }
        }

        board
    }

    mod fen_starting_position {
        use super::*;

        const FEN: &str =
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        #[test]
        fn it_sets_up_the_board() {
            let position = Position::from_fen(FEN);
            #[rustfmt::skip]
            assert_eq!(
                position.board,
                flip([
                    BR, BN, BB, BQ, BK, BB, BN, BR,
                    BP, BP, BP, BP, BP, BP, BP, BP,
                    NP, NP, NP, NP, NP, NP, NP, NP,
                    NP, NP, NP, NP, NP, NP, NP, NP,
                    NP, NP, NP, NP, NP, NP, NP, NP,
                    NP, NP, NP, NP, NP, NP, NP, NP,
                    WP, WP, WP, WP, WP, WP, WP, WP,
                    WR, WN, WB, WQ, WK, WB, WN, WR,
                ]),
            );
        }

        #[test]
        fn it_sets_up_the_castling() {
            let position = Position::from_fen(FEN);
            assert!(position.castling.black_long);
            assert!(position.castling.black_short);
            assert!(position.castling.white_long);
            assert!(position.castling.white_short);
        }

        #[test]
        fn it_ignores_the_draw_by_repetition() {
            let position = Position::from_fen(FEN);
            assert_eq!(position.draw_by_repetition, false);
        }

        #[test]
        fn it_sets_up_the_en_passant() {
            let position = Position::from_fen(FEN);
            assert_eq!(position.en_passant, None);
        }

        #[test]
        fn it_sets_up_the_fullmove() {
            let position = Position::from_fen(FEN);
            assert_eq!(position.fullmove, 1);
        }

        #[test]
        fn it_sets_up_the_halfmove() {
            let position = Position::from_fen(FEN);
            assert_eq!(position.halfmove, 0);
        }

        #[test]
        fn it_sets_up_our_color() {
            let position = Position::from_fen(FEN);
            assert_eq!(position.our_color, Color::White);
        }

        #[test]
        fn it_sets_up_the_black_pieces() {
            let position = Position::from_fen(FEN);
            assert_eq!(
                position.black(),
                Bitboard::from([
                    [1, 1, 1, 1, 1, 1, 1, 1],
                    [1, 1, 1, 1, 1, 1, 1, 1],
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
        fn it_sets_up_the_white_pieces() {
            let position = Position::from_fen(FEN);
            assert_eq!(
                position.white(),
                Bitboard::from([
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [1, 1, 1, 1, 1, 1, 1, 1],
                    [1, 1, 1, 1, 1, 1, 1, 1],
                ])
            );
        }

        #[test]
        fn it_sets_up_the_bishops() {
            let position = Position::from_fen(FEN);
            assert_eq!(
                position.bishops(),
                Bitboard::from([
                    [0, 0, 1, 0, 0, 1, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 1, 0, 0, 1, 0, 0],
                ])
            );
        }

        #[test]
        fn it_sets_up_the_kings() {
            let position = Position::from_fen(FEN);
            assert_eq!(
                position.kings(),
                Bitboard::from([
                    [0, 0, 0, 0, 1, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 1, 0, 0, 0],
                ])
            );
        }

        #[test]
        fn it_sets_up_the_knights() {
            let position = Position::from_fen(FEN);
            assert_eq!(
                position.knights(),
                Bitboard::from([
                    [0, 1, 0, 0, 0, 0, 1, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 1, 0, 0, 0, 0, 1, 0],
                ])
            );
        }

        #[test]
        fn it_sets_up_the_pawns() {
            let position = Position::from_fen(FEN);
            assert_eq!(
                position.pawns(),
                Bitboard::from([
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [1, 1, 1, 1, 1, 1, 1, 1],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [1, 1, 1, 1, 1, 1, 1, 1],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                ])
            );
        }

        #[test]
        fn it_sets_up_the_queens() {
            let position = Position::from_fen(FEN);
            assert_eq!(
                position.queens(),
                Bitboard::from([
                    [0, 0, 0, 1, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 1, 0, 0, 0, 0],
                ])
            );
        }

        #[test]
        fn it_sets_up_the_rooks() {
            let position = Position::from_fen(FEN);
            assert_eq!(
                position.rooks(),
                Bitboard::from([
                    [1, 0, 0, 0, 0, 0, 0, 1],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [1, 0, 0, 0, 0, 0, 0, 1],
                ])
            );
        }
    }
}
