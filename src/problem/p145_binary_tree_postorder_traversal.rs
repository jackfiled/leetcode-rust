/**
 * [145] Binary Tree Postorder Traversal
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
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut stack = Vec::new();
        let mut root = root;
        let mut prev = None;

        while root.is_some() || !stack.is_empty() {
            while let Some(r) = root {
                stack.push(Rc::clone(&r));
                root = r.borrow().left.clone();
            }

            root = stack.pop();
            if let Some(r) = root {
                if r.borrow().right.is_none() || r.borrow().right == prev {
                    result.push(r.borrow().val);
                    prev = Some(r);
                    root = None;
                } else {
                    stack.push(Rc::clone(&r));
                    root = r.borrow().right.clone();
                }
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
    fn test_145() {
        assert_eq!(Solution::postorder_traversal(
            to_tree(vec![Some(1), None, Some(2), Some(3)])),
                   vec![3,2,1]);
        assert_eq!(Solution::postorder_traversal(
            to_tree(vec![Some(2), None, Some(3), None, Some(1)])
        ), vec![1,3,2]);
    }
}
