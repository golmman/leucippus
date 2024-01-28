use super::board::Board;
use super::tree_node::TreeNode;
use super::types::TreeNodeIndex;
use super::types::TREE_NODE_ROOT_INDEX;

pub struct Tree {
    nodes: Vec<TreeNode>,
}

impl Tree {
    pub fn new(board: Board) -> Self {
        Self {
            nodes: vec![TreeNode::new(board, None)],
        }
    }

    pub fn get_size(&self) -> usize {
        self.nodes.len()
    }

    pub fn get_root(&self) -> &TreeNode {
        return &self.nodes[TREE_NODE_ROOT_INDEX];
    }

    pub fn get_node(&self, index: TreeNodeIndex) -> &TreeNode {
        return &self.nodes[index];
    }

    pub fn get_node_mut(&mut self, index: TreeNodeIndex) -> &mut TreeNode {
        return &mut self.nodes[index];
    }

    pub fn add_node(&mut self, board: Board, parent_index: TreeNodeIndex) {
        let child_index = self.nodes.len();
        self.nodes.push(TreeNode::new(board, Some(parent_index)));
        self.nodes[parent_index].child_indices.push(child_index);
    }
}
