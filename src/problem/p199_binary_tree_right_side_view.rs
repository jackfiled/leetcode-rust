/**
 * [199] Binary Tree Right Side View
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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        let mut queue = VecDeque::new();

        if let Some(root) = root {
            queue.push_back(root);
        }

        while !queue.is_empty() {
            let length = queue.len();

            for i in 0..length {
                let node = queue.pop_front().unwrap();

                if i == length - 1 {
                    result.push(node.borrow().val);
                }

                if let Some(left) = &node.borrow().left {
                    queue.push_back(Rc::clone(left));
                };

                if let Some(right) = &node.borrow().right {
                    queue.push_back(Rc::clone(right));
                };
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_199() {
    }
}
