/**
 * [124] Binary Tree Maximum Path Sum
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
    fn max_sum(node: &Rc<RefCell<TreeNode>>, result: &mut i32) -> i32 {
        if node.borrow().left.is_none() && node.borrow().right.is_none() {
            *result = (*result).max(node.borrow().val);

            return node.borrow().val;
        }

        let left_sum = if let Some(left) = &node.borrow().left {
            Self::max_sum(left, result)
        } else {
            0
        }
        .max(0);

        let right_sum = if let Some(right) = &node.borrow().right {
            Self::max_sum(right, result)
        } else {
            0
        }
        .max(0);

        *result = (*result).max(node.borrow().val + left_sum + right_sum);

        node.borrow().val + left_sum.max(right_sum)
    }

    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = i32::MIN;

        if let Some(root) = root {
            Self::max_sum(&root, &mut result);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_124() {}
}
