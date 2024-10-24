/**
 * [637] Average of Levels in Binary Tree
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
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut result = vec![];

        let mut queue = VecDeque::new();

        if let Some(root) = root {
            queue.push_back(root);
        }

        while !queue.is_empty() {
            let length = queue.len();

            let mut sum = 0f64;
            for _ in 0..length {
                let node = queue.pop_front().unwrap();

                sum += node.borrow().val as f64;

                if let Some(left) = &node.borrow().left {
                    queue.push_back(Rc::clone(left));
                };

                if let Some(right) = &node.borrow().right {
                    queue.push_back(Rc::clone(right));
                };
            }

            result.push(sum / length as f64);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_637() {}
}
