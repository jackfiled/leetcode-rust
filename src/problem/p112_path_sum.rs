/**
 * [112] Path Sum
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
impl Solution {
    fn dfs(node: &Rc<RefCell<TreeNode>>, current_sum: i32, target_sum: i32) -> bool {
        // 到达叶子节点
        if node.borrow().left.is_none() && node.borrow().right.is_none() {
            return current_sum + node.borrow().val == target_sum;
        }

        if let Some(left) = &node.borrow().left {
            if Self::dfs(left, current_sum + node.borrow().val, target_sum) {
                return true;
            }
        }

        if let Some(right) = &node.borrow().right {
            if Self::dfs(right, current_sum + node.borrow().val, target_sum) {
                return true;
            }
        }

        false
    }


    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if let Some(root) = root {
            return Self::dfs(&root, 0, target_sum);
        }

        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_112() {
    }
}
