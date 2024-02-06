use super::board::Board;
use super::types::TreeNodeIndex;

#[derive(Debug, PartialEq)]
pub struct SelectionResult {
    pub board: Board,
    pub node_index: TreeNodeIndex,
}
