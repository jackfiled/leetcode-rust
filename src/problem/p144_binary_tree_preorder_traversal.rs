/**
 * [144] Binary Tree Preorder Traversal
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
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut stack = Vec::new();
        let mut root = root;

        while root.is_some() || !stack.is_empty() {
            while let Some(r) = root {
                result.push(r.borrow().val);
                stack.push(Rc::clone(&r));
                root = r.borrow().left.clone();
            }

            let node = stack.pop().unwrap();
            root = node.borrow().right.clone();
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_144() {
        assert_eq!(Solution::preorder_traversal(to_tree(
            vec![Some(1), None, Some(2), Some(3)])), vec![1,2,3]);
    }
}
