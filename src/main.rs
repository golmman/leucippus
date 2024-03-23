use bitboards::r#move::attacks::{debug_magic_bishops, init_bishop_table, BishopTable};
use clap::Parser;
use model::args::Args;
use search::search::search;


pub mod bitboards {
    pub mod model {
        pub mod bitboard;
        pub mod position;
    }

    pub mod r#move {
        pub mod attacks;
        pub mod bishop_table;
        pub mod generate;
        pub mod knight_moves;
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
    //debug_magic_bishops();

    //println!("{:?}", bishop_table());
    //println!("{:?}", bt());

    //let s = 728;
    //let (r, s) = sparse_rand(s);
    //println!("rrr: {}", r.0);
    //let (r, s) = sparse_rand(s);
    //println!("rrr: {}", r.0);
    //let (r, s) = sparse_rand(s);
    //println!("rrr: {}", r.0);
    //let (r, s) = sparse_rand(s);
    //println!("rrr: {}", r.0);

    //let args = Args::parse();
    //search(args);

    let table = init_bishop_table();
    print_bishop_table(table);
}

fn print_bishop_table(table: BishopTable) {

    println!("use crate::bitboards::model::bitboard::Bitboard;");
    println!("use crate::bitboards::r#move::attacks::BishopTable;");
    println!("use crate::bitboards::r#move::attacks::Magic;");
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
    println!("const TABLE_DATA: [Bitboard; 0x1480] = [");
    for t in table.table {
        println!("Bitboard(0x{:X}),", t.0);
    }
    println!("];");
}

















