/**
 * [2476] Closest Nodes Queries in a Binary Search Tree
 */
pub struct Solution {}

use surf::url::UrlQuery;

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
impl Solution {
    pub fn closest_nodes(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<Vec<i32>> {
        let mut array = Vec::new();
        let mut stack = Vec::new();
        let mut root = root;

        while root.is_some() || !stack.is_empty() {
            while let Some(r) = root {
                stack.push(Rc::clone(&r));
                root = r.borrow().left.clone();
            }

            root = stack.pop();
            if let Some(r) = root {
                array.push(r.borrow().val);
                root = r.borrow().right.clone();
            }
        }

        let mut result = Vec::with_capacity(queries.len());
        for query in queries {
            let pos = array.binary_search(&query);

            match pos {
                Ok(pos) => {
                    result.push(vec![array[pos], array[pos]]);
                },
                Err(pos) => {
                    if pos == 0 {
                        result.push(vec![-1, array[pos]]);
                    } else if pos == array.len() {
                        result.push(vec![array[pos - 1], -1])
                    } else {
                        result.push(vec![array[pos - 1], array[pos]]);
                    }
                }
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2476() {
    }
}
