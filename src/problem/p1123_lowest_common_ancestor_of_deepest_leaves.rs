/**
 * [1123] Lowest Common Ancestor of Deepest Leaves
 */
pub struct Solution {}

use crate::util::tree::{to_tree, TreeNode};

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
use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

impl Solution {
    pub fn lca_deepest_leaves(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root) = root {
            Self::search(&root).1
        } else {
            None
        }
    }

    // (子树的深度，lca）
    fn search(node: &Rc<RefCell<TreeNode>>) -> (i32, Option<Rc<RefCell<TreeNode>>>) {
        let (left_deep, left_lca) = if let Some(left) = node.borrow().left.as_ref() {
            Self::search(left)
        } else {
            (0, None)
        };

        let (right_deep, right_lca) = if let Some(right) = node.borrow().right.as_ref() {
            Self::search(right)
        } else {
            (0, None)
        };

        match left_deep.cmp(&right_deep) {
            Ordering::Greater => (left_deep + 1, left_lca),
            Ordering::Less => (right_deep + 1, right_lca),
            Ordering::Equal => (left_deep + 1, Some(node.clone())),
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1123() {
        let node = Solution::lca_deepest_leaves(to_tree(vec![
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(0),
            Some(8),
            None,
            None,
            Some(7),
            Some(4),
        ]));

        assert_eq!(to_tree(vec![Some(2), Some(7), Some(4)]), node);

        assert_eq!(
            to_tree(vec![Some(1)]),
            Solution::lca_deepest_leaves(to_tree(vec![Some(1)]))
        );
        assert_eq!(
            to_tree(vec![Some(2)]),
            Solution::lca_deepest_leaves(to_tree(vec![Some(0), Some(1), Some(3), None, Some(2)]))
        );
    }
}
