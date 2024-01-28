#[derive(Clone, Debug, Hash, PartialEq)]
pub struct BoardCastle {
    pub black_long: bool,
    pub black_short: bool,
    pub white_long: bool,
    pub white_short: bool,
}
