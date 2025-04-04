/**
 * [530] Minimum Absolute Difference in BST
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
use std::rc::Rc;
impl Solution {
    fn inorder_iterate(node: &Rc<RefCell<TreeNode>>, arrap: &mut Vec<i32>) {
        if let Some(left) = node.borrow().left.as_ref() {
            Self::inorder_iterate(left, arrap);
        }

        arrap.push(node.borrow().val);

        if let Some(right) = node.borrow().right.as_ref() {
            Self::inorder_iterate(right, arrap);
        }
    }

    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut array = vec![];

        if let Some(root) = root {
            Self::inorder_iterate(&root, &mut array);
        }

        let mut result = i32::MAX;

        for i in 1..array.len() {
            result = result.min(array[i] - array[i - 1]);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_530() {}
}
