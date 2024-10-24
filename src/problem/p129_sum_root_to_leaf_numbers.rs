/**
 * [129] Sum Root to Leaf Numbers
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
    fn dfs_sum(node: &Rc<RefCell<TreeNode>>, last_num: i32, result: &mut i32) {
        if node.borrow().left.is_none() && node.borrow().right.is_none() {
            *result += last_num * 10 + node.borrow().val;
        }

        if let Some(left) = &node.borrow().left {
            Self::dfs_sum(left, last_num * 10 + node.borrow().val, result);
        }

        if let Some(right) = &node.borrow().right {
            Self::dfs_sum(right, last_num * 10 + node.borrow().val, result)
        }
    }

    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = 0;

        if let Some(root) = root {
            Self::dfs_sum(&root, 0, &mut result);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_129() {}
}
