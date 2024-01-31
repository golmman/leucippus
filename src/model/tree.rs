use std::f64::consts::SQRT_2;

use super::board::Board;
use super::board_evaluation::BoardEvaluation;
use super::color::Color;
use super::r#move::Move;
use super::tree_node::TreeNode;
use super::tree_node::TreeNodeScore;
use super::types::TreeNodeIndex;
use super::types::TREE_NODE_ROOT_INDEX;

pub struct Tree {
    nodes: Vec<TreeNode>,
}

impl Tree {
    pub fn new(board: Board) -> Self {
        Self {
            nodes: vec![Tree::construct_node(
                board,
                Move::from_to(0, 0), // TODO: make Option?
                None,
                TREE_NODE_ROOT_INDEX,
            )],
        }
    }

    pub fn get_size(&self) -> usize {
        self.nodes.len()
    }

    pub fn get_root(&self) -> &TreeNode {
        &self.nodes[TREE_NODE_ROOT_INDEX]
    }

    pub fn get_node(&self, index: TreeNodeIndex) -> &TreeNode {
        &self.nodes[index]
    }

    pub fn get_node_mut(&mut self, index: TreeNodeIndex) -> &mut TreeNode {
        &mut self.nodes[index]
    }

    pub fn get_parent(&self, index: TreeNodeIndex) -> Option<&TreeNode> {
        self.nodes[index].parent_index.map(|pi| &self.nodes[pi])
    }

    pub fn get_parent_mut(
        &mut self,
        index: TreeNodeIndex,
    ) -> Option<&mut TreeNode> {
        self.nodes[index].parent_index.map(|pi| &mut self.nodes[pi])
    }

    pub fn get_sibling_indices(
        &self,
        index: TreeNodeIndex,
    ) -> &[TreeNodeIndex] {
        self.get_parent(index).map_or(&[], |p| &p.child_indices)
    }

    pub fn add_node(
        &mut self,
        board: Board,
        last_move: Move,
        parent_index: TreeNodeIndex,
    ) {
        let node_index = self.nodes.len();
        self.nodes.push(Tree::construct_node(
            board,
            last_move,
            Some(parent_index),
            node_index,
        ));
        self.nodes[parent_index].child_indices.push(node_index);
    }

    /// Calculates the upper confidence bound for trees
    /// See:
    /// https://en.wikipedia.org/wiki/Monte_Carlo_tree_search
    /// https://www.chessprogramming.org/UCT
    pub fn calculate_uct(&self, index: TreeNodeIndex) -> u32 {
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

        let uct = child_win_ratio + SQRT_2 * (parent_visits.ln() / child_visits).sqrt();

        (uct * 1000.0) as u32
    }

    fn construct_node(
        board: Board,
        last_move: Move,
        parent_index: Option<TreeNodeIndex>,
        self_index: TreeNodeIndex,
    ) -> TreeNode {
        let board_hash = board.get_hash();
        TreeNode {
            board,
            board_hash,
            child_indices: Vec::new(),
            evaluation: BoardEvaluation::Inconclusive,
            last_move,
            parent_index,
            score: TreeNodeScore {
                draws: 0,
                wins_black: 0,
                wins_white: 0,
            },
            self_index,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_returns_no_parent_node_for_root() {
        let tree = Tree::new(Board::new());
        assert!(tree.get_parent(TREE_NODE_ROOT_INDEX).is_none());
    }

    #[test]
    fn it_returns_the_parent_node() {
        let mut tree = Tree::new(Board::new());
        tree.add_node(Board::new(), Move::from_to(0, 0), TREE_NODE_ROOT_INDEX);
        assert_eq!(tree.get_parent(1).unwrap().child_indices, vec![1]);
    }

    #[test]
    fn it_proves_that_root_has_no_siblings() {
        let mut tree = Tree::new(Board::new());
        assert_eq!(tree.get_sibling_indices(TREE_NODE_ROOT_INDEX), vec![]);
    }

    #[test]
    fn it_finds_all_sibling_indices() {
        let mut tree = Tree::new(Board::new());
        tree.add_node(Board::new(), Move::from_to(0, 0), TREE_NODE_ROOT_INDEX);
        tree.add_node(Board::new(), Move::from_to(0, 0), TREE_NODE_ROOT_INDEX);
        tree.add_node(Board::new(), Move::from_to(0, 0), TREE_NODE_ROOT_INDEX);
        assert_eq!(tree.get_sibling_indices(1), vec![1, 2, 3]);
        assert_eq!(tree.get_sibling_indices(2), vec![1, 2, 3]);
        assert_eq!(tree.get_sibling_indices(3), vec![1, 2, 3]);
    }
}
