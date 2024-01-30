use crate::model::board::Board;
use crate::model::tree::Tree;

pub fn search(board: Board) {
    let mut tree = Tree::new(board);

    //select(&mut tree);
}

#[cfg(test)]
mod test {
    use super::*;
}
