/**
 * [993] Cousins in Binary Tree
 */
pub struct Solution {}

use crate::util::tree::{TreeNode, to_tree};

// submission codes start here

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;

struct Tree {
    root: Rc<RefCell<TreeNode>>,
    x: i32,
    x_height: usize,
    x_parent: i32,
    y: i32,
    y_height: usize,
    y_parent: i32
}

impl Tree {
    fn new(root: Rc<RefCell<TreeNode>>, x: i32, y: i32) -> Tree {
        return Tree {
            root,
            x,
            x_height: 0,
            x_parent: 0,
            y,
            y_height: 0,
            y_parent: 0
        }
    }

    fn dfs(&mut self, node: &Rc<RefCell<TreeNode>>, height: usize, parent: i32) {
        let (mut x_find, mut y_find) = (false, false);

        if node.borrow().val == self.x {
            self.x_height = height;
            self.x_parent = parent;

            x_find = true;
        }

        if node.borrow().val == self.y {
            self.y_height = height;
            self.y_parent = parent;

            y_find = true;
        }

        if x_find && y_find {
            return;
        }

        if let Some(left) = &node.borrow().left {
            self.dfs(left, height + 1, node.borrow().val);
        }

        if let Some(right) = &node.borrow().right {
            self.dfs(right, height + 1, node.borrow().val);
        }
    }
}

impl Solution {
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        let root = if let Some(r) = root {
            r
        } else {
            return false;
        };

        let mut tree = Tree::new(Rc::clone(&root), x, y);
        tree.dfs(&root, 0, -1);


        tree.x_height == tree.y_height && tree.x_parent != tree.y_parent
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_993() {
        assert!(!Solution::is_cousins(to_tree(vec![Some(1),Some(2),Some(3),Some(4)]),
                                      4, 3));
    }
}
