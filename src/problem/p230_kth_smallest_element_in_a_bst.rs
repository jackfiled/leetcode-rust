/**
 * [230] Kth Smallest Element in a BST
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
    fn inorder_iterate(node: &Rc<RefCell<TreeNode>>, count: &mut i32, target: i32) -> Option<i32> {
        if let Some(left) = node.borrow().left.as_ref() {
            if let Some(result) = Self::inorder_iterate(left, count, target) {
                return Some(result);
            }
        };

        *count += 1;
        if *count == target {
            return Some(node.borrow().val);
        }

        if let Some(right) = node.borrow().right.as_ref() {
            if let Some(result) =  Self::inorder_iterate(right, count, target) {
                return Some(result);
            }
        }

        None
    }

    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut count = 0;

        if let Some(root) = root {
            return Self::inorder_iterate(&root, &mut count, k).unwrap()
        }

        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_230() {
    }
}
