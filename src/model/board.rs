use super::board_castle::BoardCastle;
use super::board_pieces::BoardPieces;
use super::color::Color;
use super::piece::Piece;
use super::squares::Squares;
use super::types::SquareIndex;

#[derive(Debug, PartialEq)]
pub struct Board {
    pub castle: BoardCastle,
    pub color: Color,
    pub draw_repeat_count: u32,
    pub en_passant: Option<SquareIndex>,
    pub fullmove: usize,
    pub halfmove: usize,
    pub pieces: BoardPieces,
}

impl Board {
    pub fn from_fen(fen: &str) -> Self {
        let fen_split = fen.split(' ').collect();

        let castle = Board::get_fen_castle(&fen_split);
        let color = Board::get_fen_color(&fen_split);
        let draw_repeat_count = 0;
        let en_passant = Board::get_fen_en_passant(&fen_split);
        let fullmove = Board::get_fen_fullmove(&fen_split);
        let halfmove = Board::get_fen_halfmove(&fen_split);
        let pieces = Board::get_fen_pieces(&fen_split);

        Self {
            castle,
            color,
            draw_repeat_count,
            en_passant,
            fullmove,
            halfmove,
            pieces,
        }
    }

    pub fn has_pawn_at(&self, at: SquareIndex) -> bool {
        self.pieces.squares.data[at as usize]
            .is_some_and(|p| p == Piece::BlackPawn || p == Piece::WhitePawn)
    }

    pub fn has_pawn_of_color_at(&self, color: Color, at: SquareIndex) -> bool {
        self.pieces.squares.data[at as usize].is_some_and(|p| {
            p.get_color() == color
                && (p == Piece::BlackPawn || p == Piece::WhitePawn)
        })
    }

    fn get_fen_castle(fen_split: &Vec<&str>) -> BoardCastle {
        let fen_castle = fen_split[2];
        BoardCastle {
            black_long: fen_castle.contains("q"),
            black_short: fen_castle.contains("k"),
            white_long: fen_castle.contains("Q"),
            white_short: fen_castle.contains("K"),
        }
    }

    fn get_fen_color(fen_split: &Vec<&str>) -> Color {
        match fen_split[1] {
            "b" => Color::Black,
            "w" => Color::White,
            _ => panic!("Invalid FEN: active color must be 'b' or 'w'"),
        }
    }

    fn get_fen_en_passant(fen_split: &Vec<&str>) -> Option<SquareIndex> {
        let fen_en_passant = fen_split[3];

        if fen_en_passant == "-" {
            return None;
        }

        let fen_en_passant_bytes: Vec<u8> = fen_en_passant.bytes().collect();
        let file = fen_en_passant_bytes[0] - b'a';
        let rank = fen_en_passant_bytes[1] - b'1';

        Some(8 * rank + file)
    }

    fn get_fen_fullmove(fen_split: &Vec<&str>) -> usize {
        usize::from_str_radix(fen_split[5], 10).unwrap()
    }

    fn get_fen_halfmove(fen_split: &Vec<&str>) -> usize {
        usize::from_str_radix(fen_split[4], 10).unwrap()
    }

    fn get_fen_pieces(fen_split: &Vec<&str>) -> BoardPieces {
        let fen_pieces_split = fen_split[0].split('/');
        let mut active_bishops = Vec::new();
        let mut active_kings = Vec::new();
        let mut active_knights = Vec::new();
        let mut active_pawns = Vec::new();
        let mut active_queens = Vec::new();
        let mut active_rooks = Vec::new();
        let mut squares = Squares::new([None; 64]);

        let mut y = 8;

        for row in fen_pieces_split {
            y -= 1;
            let mut x = 0;
            for col in row.bytes() {
                let i = 8 * y + x;

                match col {
                    b'b' => squares.data[i] = Some(Piece::BlackBishop),
                    b'k' => squares.data[i] = Some(Piece::BlackKing),
                    b'n' => squares.data[i] = Some(Piece::BlackKnight),
                    b'p' => squares.data[i] = Some(Piece::BlackPawn),
                    b'q' => squares.data[i] = Some(Piece::BlackQueen),
                    b'r' => squares.data[i] = Some(Piece::BlackRook),
                    b'B' => squares.data[i] = Some(Piece::WhiteBishop),
                    b'K' => squares.data[i] = Some(Piece::WhiteKing),
                    b'N' => squares.data[i] = Some(Piece::WhiteKnight),
                    b'P' => squares.data[i] = Some(Piece::WhitePawn),
                    b'Q' => squares.data[i] = Some(Piece::WhiteQueen),
                    b'R' => squares.data[i] = Some(Piece::WhiteRook),
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

        let active_color = Board::get_fen_color(fen_split);

        for i in 0..64 {
            if let Some(piece) = squares.data[i] {
                if piece.is_bishop_of_color(active_color) {
                    active_bishops.push(i as SquareIndex);
                } else if piece.is_king_of_color(active_color) {
                    active_kings.push(i as SquareIndex);
                } else if piece.is_knight_of_color(active_color) {
                    active_knights.push(i as SquareIndex);
                } else if piece.is_pawn_of_color(active_color) {
                    active_pawns.push(i as SquareIndex);
                } else if piece.is_queen_of_color(active_color) {
                    active_queens.push(i as SquareIndex);
                } else if piece.is_rook_of_color(active_color) {
                    active_rooks.push(i as SquareIndex);
                }
            }
        }

        BoardPieces {
            active_bishops,
            active_kings,
            active_knights,
            active_pawns,
            active_queens,
            active_rooks,
            squares,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_creates_a_board_from_fen_with_missing_castles() {
        let fen =
            "1nbqkb1r/3p3p/1p2ppp1/r7/3N2n1/NP2P2P/P1PP1PP1/R1B2RK1 b k - 0 11";
        let Board {
            castle,
            color,
            en_passant,
            fullmove,
            halfmove,
            pieces,
            ..
        } = Board::from_fen(fen);

        assert_eq!(castle.black_long, false);
        assert_eq!(castle.black_short, true);
        assert_eq!(castle.white_long, false);
        assert_eq!(castle.white_short, false);

        assert_eq!(color, Color::Black);
        assert_eq!(en_passant, None);
        assert_eq!(fullmove, 11);
        assert_eq!(halfmove, 0);

        #[rustfmt::skip]
        assert_eq!(
            pieces.squares,
            Squares::flipped([
                None, bn(), bb(), bq(), bk(), bb(), None, br(),
                None, None, None, bp(), None, None, None, bp(),
                None, bp(), None, None, bp(), bp(), bp(), None,
                br(), None, None, None, None, None, None, None,
                None, None, None, wn(), None, None, bn(), None,
                wn(), wp(), None, None, wp(), None, None, wp(),
                wp(), None, wp(), wp(), None, wp(), wp(), None,
                wr(), None, wb(), None, None, wr(), wk(), None,
            ]),
        );

        assert_eq!(pieces.active_bishops, vec![58, 61]);
        assert_eq!(pieces.active_kings, vec![60]);
        assert_eq!(pieces.active_knights, vec![30, 57]);
        assert_eq!(pieces.active_pawns, vec![41, 44, 45, 46, 51, 55]);
        assert_eq!(pieces.active_queens, vec![59]);
        assert_eq!(pieces.active_rooks, vec![32, 63]);
    }

    #[test]
    fn it_creates_a_board_from_fen_with_en_passant() {
        let fen =
            "rnbqkbnr/1pp1pppp/p7/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 3";
        let Board {
            castle,
            color,
            en_passant,
            fullmove,
            halfmove,
            pieces,
            ..
        } = Board::from_fen(fen);

        assert_eq!(castle.black_long, true);
        assert_eq!(castle.black_short, true);
        assert_eq!(castle.white_long, true);
        assert_eq!(castle.white_short, true);

        assert_eq!(color, Color::White);
        assert_eq!(en_passant, Some(43));
        assert_eq!(fullmove, 3);
        assert_eq!(halfmove, 0);

        #[rustfmt::skip]
        assert_eq!(
            pieces.squares,
            Squares::flipped([
                br(), bn(), bb(), bq(), bk(), bb(), bn(), br(),
                None, bp(), bp(), None, bp(), bp(), bp(), bp(),
                bp(), None, None, None, None, None, None, None,
                None, None, None, bp(), wp(), None, None, None,
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                wp(), wp(), wp(), wp(), None, wp(), wp(), wp(),
                wr(), wn(), wb(), wq(), wk(), wb(), wn(), wr(),
            ]),
        );

        assert_eq!(pieces.active_bishops, vec![2, 5]);
        assert_eq!(pieces.active_kings, vec![4]);
        assert_eq!(pieces.active_knights, vec![1, 6]);
        assert_eq!(pieces.active_pawns, vec![8, 9, 10, 11, 13, 14, 15, 36]);
        assert_eq!(pieces.active_queens, vec![3]);
        assert_eq!(pieces.active_rooks, vec![0, 7]);
    }

    #[test]
    fn it_creates_a_board_from_fen_with_the_starting_position() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let Board {
            castle,
            color,
            en_passant,
            fullmove,
            halfmove,
            pieces,
            ..
        } = Board::from_fen(fen);

        assert_eq!(castle.black_long, true);
        assert_eq!(castle.black_short, true);
        assert_eq!(castle.white_long, true);
        assert_eq!(castle.white_short, true);

        assert_eq!(color, Color::White);
        assert_eq!(en_passant, None);
        assert_eq!(fullmove, 1);
        assert_eq!(halfmove, 0);

        #[rustfmt::skip]
        assert_eq!(
            pieces.squares,
            Squares::flipped([
                br(), bn(), bb(), bq(), bk(), bb(), bn(), br(),
                bp(), bp(), bp(), bp(), bp(), bp(), bp(), bp(),
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                wp(), wp(), wp(), wp(), wp(), wp(), wp(), wp(),
                wr(), wn(), wb(), wq(), wk(), wb(), wn(), wr(),
            ]),
        );

        assert_eq!(pieces.active_bishops, vec![2, 5]);
        assert_eq!(pieces.active_kings, vec![4]);
        assert_eq!(pieces.active_knights, vec![1, 6]);
        assert_eq!(pieces.active_pawns, vec![8, 9, 10, 11, 12, 13, 14, 15]);
        assert_eq!(pieces.active_queens, vec![3]);
        assert_eq!(pieces.active_rooks, vec![0, 7]);
    }

    fn bb() -> Option<Piece> {
        Some(Piece::BlackBishop)
    }

    fn bk() -> Option<Piece> {
        Some(Piece::BlackKing)
    }

    fn bn() -> Option<Piece> {
        Some(Piece::BlackKnight)
    }

    fn bp() -> Option<Piece> {
        Some(Piece::BlackPawn)
    }

    fn bq() -> Option<Piece> {
        Some(Piece::BlackQueen)
    }

    fn br() -> Option<Piece> {
        Some(Piece::BlackRook)
    }

    fn wb() -> Option<Piece> {
        Some(Piece::WhiteBishop)
    }

    fn wk() -> Option<Piece> {
        Some(Piece::WhiteKing)
    }

    fn wn() -> Option<Piece> {
        Some(Piece::WhiteKnight)
    }

    fn wp() -> Option<Piece> {
        Some(Piece::WhitePawn)
    }

    fn wq() -> Option<Piece> {
        Some(Piece::WhiteQueen)
    }

    fn wr() -> Option<Piece> {
        Some(Piece::WhiteRook)
    }
}
