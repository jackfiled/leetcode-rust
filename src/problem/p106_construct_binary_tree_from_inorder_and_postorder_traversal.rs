/**
 * [106] Construct Binary Tree from Inorder and Postorder Traversal
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
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.len() == 0 {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(postorder[postorder.len() - 1])));

        let mut stack = Vec::new();
        stack.push(Rc::clone(&root));

        let mut inorder_index = inorder.len() - 1;

        for i in (0..postorder.len() - 1).rev() {
            let post_order_value = postorder[i];
            let mut node = Rc::clone(stack.last().unwrap());

            if node.borrow().val != inorder[inorder_index] {
                let new_node = Rc::new(RefCell::new(TreeNode::new(post_order_value)));
                stack.push(Rc::clone(&new_node));
                node.borrow_mut().right = Some(new_node);
            } else {
                while let Some(top) = stack.last() {
                    if top.borrow().val == inorder[inorder_index] {
                        node = Rc::clone(top);
                        stack.pop();
                        inorder_index -= 1;
                    } else {
                        break;
                    }
                }

                let new_node = Rc::new(RefCell::new(TreeNode::new(post_order_value)));
                stack.push(Rc::clone(&new_node));
                node.borrow_mut().left = Some(new_node);
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
    fn test_106() {
        assert_eq!(
            Solution::build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3]),
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

        assert_eq!(
            Solution::build_tree(vec![-1], vec![-1]),
            to_tree(vec![Some(-1)])
        );
    }
}
