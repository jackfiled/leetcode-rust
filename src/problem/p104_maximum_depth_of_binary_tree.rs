/**
 * [104] Maximum Depth of Binary Tree
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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = VecDeque::new();

        let mut depth = 0;

        if let Some(node) = root {
            queue.push_back(node);
        }

        while !queue.is_empty() {
            let level = queue.len();
            depth += 1;

            for i in 0..level {
                let node = queue.pop_front().unwrap();

                if let Some(left) = &node.borrow().left {
                    queue.push_back(Rc::clone(left));
                };

                if let Some(right) = &node.borrow().right {
                    queue.push_back(Rc::clone(right));
                };
            }
        }

        depth
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_104() {}
}
