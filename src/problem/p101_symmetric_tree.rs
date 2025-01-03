/**
 * [101] Symmetric Tree
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
    fn is_same(
        left: &Option<Rc<RefCell<TreeNode>>>,
        right: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if left.is_some() ^ right.is_some() {
            return false;
        }

        if left.is_none() && right.is_none() {
            return true;
        }

        let left = left.as_ref().unwrap();
        let right = right.as_ref().unwrap();

        if left.borrow().val != right.borrow().val {
            return false;
        }

        Self::is_same(&left.borrow().left, &right.borrow().right)
            && Self::is_same(&left.borrow().right, &right.borrow().left)
    }

    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root) = root {
            return Self::is_same(&root.borrow().left, &root.borrow().right);
        }

        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_101() {
        assert!(Solution::is_symmetric(tree![1, 2, 2, 3, 4, 4, 3]));
        assert!(!Solution::is_symmetric(tree!(
            1, 2, 2, "null", 3, "null", 3
        )))
    }
}
