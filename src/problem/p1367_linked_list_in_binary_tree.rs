/**
 * [1367] Linked List in Binary Tree
 */
pub struct Solution {}

use crate::util::linked_list::{to_list, ListNode};
use crate::util::tree::{to_tree, TreeNode};

// submission codes start here

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
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
    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let head = head.unwrap();
        let root = root.unwrap();
        let mut queue = VecDeque::from([root]);

        while let Some(node) = queue.pop_front() {
            if Self::search(&head, &node) {
                return true;
            }

            if let Some(left) = &node.borrow().left {
                queue.push_back(left.clone());
            }
            if let Some(right) = &node.borrow().right {
                queue.push_back(right.clone());
            }
        }

        false
    }

    fn search(node: &Box<ListNode>, tree_node: &Rc<RefCell<TreeNode>>) -> bool {
        if node.val != tree_node.borrow().val {
            return false;
        }

        if let Some(next) = &node.next {
            if let Some(left) = &tree_node.borrow().left {
                if Self::search(next, left) {
                    return true;
                }
            }

            if let Some(right) = &tree_node.borrow().right {
                if Self::search(next, right) {
                    return true;
                }
            }

            false
        } else {
            true
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1367() {
        assert!(Solution::is_sub_path(
            to_list(vec![4, 2, 8]),
            to_tree(vec![
                Some(1),
                Some(4),
                Some(4),
                None,
                Some(2),
                Some(2),
                None,
                Some(1),
                None,
                Some(6),
                Some(8),
                None,
                None,
                None,
                None,
                Some(1),
                Some(3)
            ])
        ));
    }
}
