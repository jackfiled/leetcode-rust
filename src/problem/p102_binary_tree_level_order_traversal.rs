/**
 * [102] Binary Tree Level Order Traversal
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
use std::collections::VecDeque;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let root = if let Some(r) = root {
            r
        } else {
            return vec![];
        };

        let mut result = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(root);

        while !queue.is_empty() {
            let mut level = Vec::with_capacity(queue.len());
            for _ in 0..queue.len() {
                if let Some(node) = queue.pop_front() {
                    level.push(node.borrow().val);

                    if let Some(left) = &node.borrow().left {
                        queue.push_back(Rc::clone(left));
                    }

                    if let Some(right) = &node.borrow().right {
                        queue.push_back(Rc::clone(right));
                    }
                }
            }

            result.push(level);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_102() {
        assert_eq!(Solution::level_order(
            to_tree(vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)])
        ), vec![vec![3], vec![9, 20], vec![15, 7]]);
    }
}
