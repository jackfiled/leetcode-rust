/**
 * [236] Lowest Common Ancestor of a Binary Tree
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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() || root == p || root == q {
            return root;
        }

        let left = Solution::lowest_common_ancestor(
            root.as_ref().unwrap().borrow_mut().left.take(),
            p.clone(),
            q.clone(),
        );

        let right = Solution::lowest_common_ancestor(
            root.as_ref().unwrap().borrow_mut().right.take(),
            p.clone(),
            q.clone(),
        );

        if left.is_some() && right.is_some() {
            return root;
        }

        if left.is_some() {
            left
        } else {
            right
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_236() {}
}
