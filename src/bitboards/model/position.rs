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

pub struct Position {
    pub board: [Option<Piece>; 64],
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

    fn get_board_from_fen(fen: &str) -> [Option<Piece>; 64] {
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

    fn get_pieces_by_color_from_board(
        board: &[Option<Piece>; 64],
    ) -> [Bitboard; 2] {
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

    fn get_pieces_by_type_from_board(
        board: &[Option<Piece>; 64],
    ) -> [Bitboard; 6] {
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
