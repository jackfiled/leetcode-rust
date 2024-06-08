/**
 * [114] Flatten Binary Tree to Linked List
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
    fn flatten_tree(root: &Rc<RefCell<TreeNode>>) -> Rc<RefCell<TreeNode>> {
        if (root.borrow().left.is_none() && root.borrow().right.is_none()) {
            // 只有一个节点
            return Rc::clone(root);
        }

        if root.borrow().right.is_some() {
            let right = Rc::clone(&root.borrow().right.as_ref().unwrap());

            if root.borrow().left.is_some() {
                let left = Rc::clone(&root.borrow().left.as_ref().unwrap());
                root.borrow_mut().right = Some(Rc::clone(&left));
                root.borrow_mut().left = None;

                let right_right = Self::flatten_tree(&left);
                right_right.borrow_mut().right = Some(Rc::clone(&right));

                return Self::flatten_tree(&right);
            } else {
                return Self::flatten_tree(&right);
            }
        } else {
            let left = Rc::clone(&root.borrow().left.as_ref().unwrap());

            root.borrow_mut().right = Some(Rc::clone(&left));
            root.borrow_mut().left = None;

            return Self::flatten_tree(&left);
        }
    }

    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(root) = root {
            Self::flatten_tree(root);
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_114() {}
}
