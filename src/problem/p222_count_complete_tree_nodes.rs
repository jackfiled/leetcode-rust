/**
 * [222] Count Complete Tree Nodes
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
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = 0;
        let mut queue = VecDeque::new();

        if let Some(root) = root {
            queue.push_back(root);
        }

        while !queue.is_empty() {
            result += 1;

            let node = queue.pop_front().unwrap();

            if let Some(left) = &node.borrow().left {
                queue.push_back(Rc::clone(left));
            };

            if let Some(right) = &node.borrow().right {
                queue.push_back(Rc::clone(right));
            };
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_222() {}
}
