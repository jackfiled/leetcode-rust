/**
 * [889] Construct Binary Tree from Preorder and Postorder Traversal
 */
pub struct Solution {}

use tokio::time::error::Elapsed;

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
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    pub fn construct_from_pre_post(
        preorder: Vec<i32>,
        postorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut post_order_map = HashMap::new();

        for (key, value) in postorder.iter().enumerate() {
            post_order_map.insert(*value, key);
        }

        fn dfs(
            pre_left: usize,
            pre_right: usize,
            post_left: usize,
            post_right: usize,
            post_order_map: &HashMap<i32, usize>,
            preorder: &Vec<i32>,
            postorder: &Vec<i32>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if pre_left > pre_right {
                return None;
            }

            let left_count = if pre_left < pre_right {
                post_order_map[&preorder[pre_left + 1]] - post_left + 1
            } else {
                0
            };

            let new_node = Rc::new(RefCell::new(TreeNode::new(preorder[pre_left])));
            new_node.borrow_mut().left = dfs(
                pre_left + 1,
                pre_left + left_count,
                post_left,
                post_left + left_count - 1,
                post_order_map,
                preorder,
                postorder,
            );
            new_node.borrow_mut().right = dfs(
                pre_left + left_count + 1,
                pre_right,
                post_left + left_count,
                post_right - 1,
                post_order_map,
                preorder,
                postorder,
            );

            Some(new_node)
        };

        let n = preorder.len();
        dfs(0, n - 1, 0, n - 1, &post_order_map, &preorder, &postorder)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_889() {}
}
