/**
 * [987] Vertical Order Traversal of a Binary Tree
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

#[derive(Debug)]
struct Node {
    x: i32,
    y: i32,
    val: i32,
}

struct DFS {
    nodes: Vec<Node>,
}

impl DFS {
    fn new() -> DFS {
        DFS { nodes: Vec::new() }
    }

    fn search(&mut self, node: &Rc<RefCell<TreeNode>>, x: i32, y: i32) {
        self.nodes.push(Node {
            val: node.borrow().val,
            x,
            y,
        });

        if let Some(left) = &node.borrow().left {
            self.search(left, x - 1, y + 1);
        }

        if let Some(right) = &node.borrow().right {
            self.search(right, x + 1, y + 1);
        }
    }
}

impl Solution {
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let root = if let Some(r) = root {
            r
        } else {
            return vec![];
        };

        let mut result = Vec::new();

        let mut dfs = DFS::new();
        dfs.search(&root, 0, 0);

        dfs.nodes.sort_unstable_by(|a, b| {
            if a.x != b.x {
                a.x.cmp(&b.x)
            } else if a.y != b.y {
                a.y.cmp(&b.y)
            } else {
                a.val.cmp(&b.val)
            }
        });

        let mut last = None;
        for node in dfs.nodes {
            if let Some((num, pos)) = last {
                if num == node.x {
                    let mut array: &mut Vec<i32> = &mut result[pos];
                    array.push(node.val);
                } else {
                    last = Some((node.x, pos + 1));
                    result.push(vec![node.val]);
                }
            } else {
                last = Some((node.x, 0));
                result.push(vec![node.val]);
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
    fn test_987() {
        assert_eq!(
            Solution::vertical_traversal(to_tree(vec![
                Some(3),
                Some(9),
                Some(20),
                None,
                None,
                Some(15),
                Some(7)
            ])),
            vec![vec![9], vec![3, 15], vec![20], vec![7]]
        );
    }
}
