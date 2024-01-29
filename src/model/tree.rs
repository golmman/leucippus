use std::f64::consts::SQRT_2;

use super::board::Board;
use super::color::Color;
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

    /// Calculates the upper confidence bound for trees
    /// See:
    /// https://en.wikipedia.org/wiki/Monte_Carlo_tree_search
    /// https://www.chessprogramming.org/UCT
    pub fn calculate_uct(&self, index: TreeNodeIndex) -> f64 {
        let child = &self.nodes[index];
        let parent_index = child
            .parent_index
            .expect("UCT calculation is not applicable to root nodes.");
        let parent = &self.nodes[parent_index];

        let child_visits = (child.score.draws
            + child.score.wins_black
            + child.score.wins_white) as f64;
        let parent_visits = (parent.score.draws
            + parent.score.wins_black
            + parent.score.wins_white) as f64;

        let our_color = &self.nodes[TREE_NODE_ROOT_INDEX].board.our_color;
        let child_win_ratio = if *our_color == Color::Black {
            (child.score.wins_black as f64) / child_visits
        } else {
            (child.score.wins_white as f64) / child_visits
        };

        child_win_ratio + SQRT_2 * (parent_visits.ln() / child_visits).sqrt()
    }
}
