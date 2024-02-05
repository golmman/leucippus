use clap::Parser;

use super::metrics_level::MetricsLevel;

/// MCTS Atomic chess
#[derive(Clone, Debug, Parser)]
#[command(author, version, verbatim_doc_comment)]
pub struct Args {
    /// Starting position as FEN.
    #[arg(short, long, value_parser, default_value_t = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"))]
    pub fen: String,

    /// Maximal number of search iterations to be analyzed.
    #[arg(short = 'i', long, value_parser, default_value_t = std::u64::MAX)]
    pub max_iterations: u64,

    /// Level of metrics to show
    #[arg(short, long, value_enum, default_value_t = MetricsLevel::Full)]
    pub metrics_level: MetricsLevel,

    /// Random number seed used for pseudo random number generation.
    #[arg(short, long, value_parser, default_value_t = 19870826)]
    pub seed: u64,
}

// Ideally we would set the Args default values in its Default impl,
// unfortunatly clap does not support this at the moment
// (https://github.com/clap-rs/clap/issues/3116) so we have to do it the other
// way around.
impl Default for Args {
    fn default() -> Self {
        Self::parse_from(Vec::<String>::new().into_iter())
    }
}
