/**
 * [100] Same Tree
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
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut p_queue = VecDeque::new();
        let mut q_queue = VecDeque::new();

        if let Some(node) = p {
            p_queue.push_back(node);
        }

        if let Some(node) = q {
            q_queue.push_back(node);
        }

        while !p_queue.is_empty() || !q_queue.is_empty() {
            if q_queue.len() != p_queue.len() {
                return false;
            }

            let length = p_queue.len();
            for _ in 0..length {
                let p_node = p_queue.pop_back().unwrap();
                let q_node = q_queue.pop_back().unwrap();

                if p_node.borrow().val != q_node.borrow().val {
                    return false;
                };

                if p_node.borrow().left.is_some() ^ q_node.borrow().left.is_some() {
                    return false;
                }

                if p_node.borrow().right.is_some() ^ q_node.borrow().right.is_some() {
                    return false;
                }

                if let Some(p_left) = &p_node.borrow().left {
                    if let Some(q_left) = &q_node.borrow().left {
                        p_queue.push_back(Rc::clone(p_left));
                        q_queue.push_back(Rc::clone(q_left));
                    };
                };

                if let Some(p_right) = &p_node.borrow().right {
                    if let Some(q_right) = &q_node.borrow().right {
                        p_queue.push_back(Rc::clone(p_right));
                        q_queue.push_back(Rc::clone(q_right));
                    };
                };
            }
        }

        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_100() {}
}
