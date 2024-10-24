/**
 * [1261] Find Elements in a Contaminated Binary Tree
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
use std::{cell::RefCell, collections::HashSet, collections::VecDeque, rc::Rc};
struct FindElements {
    collected_set: HashSet<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindElements {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut queue = VecDeque::new();
        let mut collected_set = HashSet::new();

        if let Some(r) = root {
            r.borrow_mut().val = 0;
            queue.push_back(Rc::clone(&r));
        }

        while !queue.is_empty() {
            let top = queue.pop_front().unwrap();
            collected_set.insert(top.borrow().val);

            if let Some(left) = &top.borrow().left {
                left.borrow_mut().val = top.borrow().val * 2 + 1;
                queue.push_back(Rc::clone(left));
            };

            if let Some(right) = &top.borrow().right {
                right.borrow_mut().val = top.borrow().val * 2 + 2;
                queue.push_back(Rc::clone(right));
            };
        }

        FindElements { collected_set }
    }

    fn find(&self, target: i32) -> bool {
        self.collected_set.contains(&target)
    }
}

/**
 * Your FindElements object will be instantiated and called as such:
 * let obj = FindElements::new(root);
 * let ret_1: bool = obj.find(target);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1261() {}
}
