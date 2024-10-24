/**
 * [235] Lowest Common Ancestor of a Binary Search Tree
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
        let (root, p, q) = (root?, p?, q?);
        let mut ancestor = root;

        loop {
            ancestor = if p.borrow().val < ancestor.borrow().val
                && q.borrow().val < ancestor.borrow().val
            {
                Rc::clone(&ancestor.borrow().left.as_ref().unwrap())
            } else if (p.borrow().val > ancestor.borrow().val
                && q.borrow().val > ancestor.borrow().val)
            {
                Rc::clone(&ancestor.borrow().right.as_ref().unwrap())
            } else {
                break;
            }
        }

        Some(ancestor)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_235() {}
}
