/**
 * [2641] Cousins in Binary Tree II
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
    pub fn replace_value_in_tree(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let root = if let Some(r) = root {
            r
        } else {
            return None;
        };

        let mut queue = Vec::new();
        queue.push(Rc::clone(&root));

        while !queue.is_empty() {
            let mut next_queue = Vec::new();

            let mut sum = 0;
            for node in &queue {
                if let Some(left) = &node.borrow().left {
                    next_queue.push(Rc::clone(left));
                    sum += left.borrow().val;
                }

                if let Some(right) = &node.borrow().right {
                    next_queue.push(Rc::clone(right));
                    sum += right.borrow().val;
                }
            }

            for node in &queue {
                let mut children = 0;
                if let Some(left) = &node.borrow().left {
                    children += left.borrow().val;
                }

                if let Some(right) = &node.borrow().right {
                    children += right.borrow().val;
                }

                if let Some(left) = &node.borrow().left {
                    left.borrow_mut().val = sum - children;
                }

                if let Some(right) = &node.borrow().right {
                    right.borrow_mut().val = sum - children;
                }
            }

            queue = next_queue;
        }

        root.borrow_mut().val = 0;
        Some(root)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2641() {
        assert_eq!(
            Solution::replace_value_in_tree(to_tree(vec![
                Some(5),
                Some(4),
                Some(9),
                Some(1),
                Some(10),
                None,
                Some(7)
            ])),
            to_tree(vec![
                Some(0),
                Some(0),
                Some(0),
                Some(7),
                Some(7),
                None,
                Some(11)
            ])
        );
    }
}
