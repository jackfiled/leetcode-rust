/**
 * [938] Range Sum of BST
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
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        let root = if let Some(r) = root {
            r
        } else {
            return 0;
        };

        let mut sum = 0;

        let mut deque = VecDeque::new();
        deque.push_back(root);

        while !deque.is_empty() {
            let node = deque.pop_front().unwrap();

            if node.borrow().val >= low && node.borrow().val <= high {
                sum += node.borrow().val;
            }

            if node.borrow().val > low {
                if let Some(left) = &node.borrow().left {
                    deque.push_back(Rc::clone(left));
                }
            }

            if node.borrow().val < high {
                if let Some(right) = &node.borrow().right {
                    deque.push_back(Rc::clone(right));
                }
            }
        }

        sum
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_938() {
        assert_eq!(
            Solution::range_sum_bst(
                to_tree(vec![
                    Some(10),
                    Some(5),
                    Some(15),
                    Some(3),
                    Some(7),
                    None,
                    Some(18)
                ]),
                7,
                15
            ),
            32
        );
    }
}
