/**
 * [173] Binary Search Tree Iterator
 */
pub struct Solution {}

use tokio::time::error::Elapsed;

use crate::util::tree::{TreeNode, to_tree};

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

use std::rc::Rc;
use std::cell::RefCell;

struct BSTIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {

    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        if root.is_none() {
            return BSTIterator {
                stack: vec![]
            };
        }   

        let mut stack = vec![];
        let root = root.unwrap();
        let mut node = root;

        loop {
            stack.push(Rc::clone(&node));

            let left = if let Some(left) = &node.borrow().left {
                Rc::clone(left)
            } else {
                break;
            };

            node = left;
        }


        BSTIterator {
            stack
        }
    }
    
    fn next(&mut self) -> i32 {
        let node = self.stack.pop().unwrap();
        let val = node.borrow().val;
        
        if let Some(right) = &node.borrow().right {
            let mut node = Rc::clone(right);

            loop {
                self.stack.push(Rc::clone(&node));

                let left = if let Some(left) = &node.borrow().left {
                    Rc::clone(left)
                } else {
                    break;
                };

                node = left;
            }
        }

        val
    }
    
    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}

/**
 * Your BSTIterator object will be instantiated and called as such:
 * let obj = BSTIterator::new(root);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_173() {
    }
}
