/**
 * [105] Construct Binary Tree from Preorder and Inorder Traversal
 */
pub struct Solution {}

use surf::middleware::logger::new;

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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.len() == 0 {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(preorder[0])));
        let mut stack = Vec::new();
        stack.push(Rc::clone(&root));
        let mut inorder_index = 0;

        for i in 1..preorder.len() {
            let mut node = Rc::clone(stack.last().unwrap());

            if node.borrow().val != inorder[inorder_index] {
                let new_node = Rc::new(RefCell::new(TreeNode::new(preorder[i])));
                stack.push(Rc::clone(&new_node));
                node.borrow_mut().left = Some(new_node);
            } else {
                while let Some(top) = stack.last() {
                    if top.borrow().val == inorder[inorder_index] {
                        node = Rc::clone(top);
                        stack.pop();
                        inorder_index += 1;
                    } else {
                        break;
                    }
                }

                let new_node = Rc::new(RefCell::new(TreeNode::new(preorder[i])));
                stack.push(Rc::clone(&new_node));
                node.borrow_mut().right = Some(new_node);
            }
        }

        Some(root)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_105() {
        assert_eq!(
            Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]),
            to_tree(vec![
                Some(3),
                Some(9),
                Some(20),
                None,
                None,
                Some(15),
                Some(7)
            ])
        );
    }
}
