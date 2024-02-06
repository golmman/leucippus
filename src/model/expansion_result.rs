use super::board::Board;
use super::types::TreeNodeIndex;

#[derive(Debug, PartialEq)]
pub struct ExpansionResult {
    pub board: Board,
    pub node_index: TreeNodeIndex,
}
