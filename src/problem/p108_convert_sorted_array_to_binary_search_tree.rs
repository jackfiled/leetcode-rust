/**
 * [108] Convert Sorted Array to Binary Search Tree
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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Self::array_to_bst(&nums[..]))
    }

    fn array_to_bst(array: &[i32]) -> Rc<RefCell<TreeNode>> {
        let length = array.len();

        if length == 1 {
            return Rc::new(RefCell::new(TreeNode::new(array[0])));
        }

        let middle = length / 2;

        let node = Rc::new(RefCell::new(TreeNode::new(array[middle])));

        if middle != 0 {
            // 左边
            let left = &array[..middle];
            node.borrow_mut().left = Some(Self::array_to_bst(left));
        }

        if middle != length - 1 {
            let right = &array[middle + 1..];
            node.borrow_mut().right = Some(Self::array_to_bst(right));
        }

        return node;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_108() {}
}
