use bitboards::r#move::attacks::init_bishop_table;
use bitboards::r#move::attacks::BishopTable;
use clap::Parser;
use model::args::Args;
use search::search::search;

use crate::bitboards::r#move::attacks::init_rook_table;

pub mod bitboards {
    pub mod model {
        pub mod bitboard;
        pub mod move_type;
        pub mod piece_type;
        pub mod position;
        pub mod r#move;
    }

    pub mod r#move {
        pub mod attacks;
        pub mod bishop_table;
        pub mod generate;
        pub mod rook_table;
    }
}

pub mod common {
    pub mod random;
}

pub mod evaluation {
    pub mod evaluate_board;
}

pub mod model {
    pub mod args;
    pub mod board;
    pub mod board_castle;
    pub mod board_evaluation;
    pub mod board_evaluation_result;
    pub mod board_pieces;
    pub mod color;
    pub mod expansion_result;
    pub mod metrics_level;
    pub mod r#move;
    pub mod piece;
    pub mod piece_type;
    pub mod selection_result;
    pub mod simulation_result;
    pub mod squares;
    pub mod tree;
    pub mod tree_node;
    pub mod tree_node_metrics;
    pub mod types;
}

pub mod move_generator {
    pub mod bishop_moves;
    pub mod check;
    pub mod king_moves;
    pub mod knight_moves;
    pub mod legal_moves;
    pub mod make_move;
    pub mod pawn_moves;
    pub mod queen_moves;
    pub mod rook_moves;
    pub mod sliding_moves;
}

pub mod search {
    pub mod backpropagate;
    pub mod expand;
    pub mod interpret;
    pub mod search;
    pub mod select;
    pub mod simulate;
}

pub mod view {
    pub mod print_metrics;
}

fn main() {
    #[cfg(bishop_magics)]
    {
        print_bishop_table();
        return;
    }
    #[cfg(rook_magics)]
    {
        print_rook_table();
        return;
    }

    let args = Args::parse();
    search(args);
}

// rusts' const evaluation interpreter is slow (takes 50s on raspi5), so
// for debug builds the tables are initalized in advance
fn print_bishop_table() {
    let table = init_bishop_table();
    println!("use crate::bitboards::model::bitboard::Bitboard;");
    println!("use crate::bitboards::r#move::attacks::BishopTable;");
    println!("use crate::bitboards::r#move::attacks::Magic;");
    println!("use crate::bitboards::r#move::attacks::BISHOP_TABLE_SIZE;");
    println!();
    println!("#[rustfmt::skip]");
    println!("pub const BISHOP_TABLE: BishopTable = BishopTable {{ magics: MAGIC_DATA, table: TABLE_DATA }};");
    println!();
    println!("#[rustfmt::skip]");
    println!("const MAGIC_DATA: [Magic; 64] = [");
    for m in table.magics {
        println!("Magic {{ mask: Bitboard(0x{:X}), magic: Bitboard(0x{:X}), attacks: {}, shift: {} }},", m.mask.0, m.magic.0, m.attacks, m.shift);
    }
    println!("];");
    println!();
    println!("#[rustfmt::skip]");
    println!("const TABLE_DATA: [Bitboard; BISHOP_TABLE_SIZE] = [");
    for t in table.table {
        println!("Bitboard(0x{:X}),", t.0);
    }
    println!("];");
}

fn print_rook_table() {
    let table = init_rook_table();
    println!("use crate::bitboards::model::bitboard::Bitboard;");
    println!("use crate::bitboards::r#move::attacks::RookTable;");
    println!("use crate::bitboards::r#move::attacks::Magic;");
    println!("use crate::bitboards::r#move::attacks::ROOK_TABLE_SIZE;");
    println!();
    println!("#[rustfmt::skip]");
    println!("pub const ROOK_TABLE: RookTable = RookTable {{ magics: MAGIC_DATA, table: TABLE_DATA }};");
    println!();
    println!("#[rustfmt::skip]");
    println!("const MAGIC_DATA: [Magic; 64] = [");
    for m in table.magics {
        println!("Magic {{ mask: Bitboard(0x{:X}), magic: Bitboard(0x{:X}), attacks: {}, shift: {} }},", m.mask.0, m.magic.0, m.attacks, m.shift);
    }
    println!("];");
    println!();
    println!("#[rustfmt::skip]");
    println!("const TABLE_DATA: [Bitboard; ROOK_TABLE_SIZE] = [");
    for t in table.table {
        println!("Bitboard(0x{:X}),", t.0);
    }
    println!("];");
}
