/**
 * [2583] Kth Largest Sum in a Binary Tree
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
    pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        let root = if let Some(r) = root {
            r
        } else {
            return -1;
        };
        let k = k as usize;

        let mut levels = Vec::new();
        let mut level = vec![root];

        while !level.is_empty() {
            let mut new_level = Vec::new();
            let mut sum = 0;

            for node in level {
                sum += node.borrow().val as i64;

                if let Some(left) = &node.borrow().left {
                    new_level.push(Rc::clone(left));
                }

                if let Some(right) = &node.borrow().right {
                    new_level.push(Rc::clone(right));
                }
            }

            levels.push(sum);
            level = new_level;
        }

        if levels.len() < k {
            return -1;
        }
        levels.sort_unstable();

        levels[levels.len() - k]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2583() {}
}
