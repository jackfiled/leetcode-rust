/**
 * [2181] Merge Nodes in Between Zeros
 */
pub struct Solution {}

use crate::util::linked_list::{ListNode, to_list};

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
impl Solution {
    pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut array = Vec::new();
        let mut value = 0;
        let mut node = head;

        while let Some(n) = node {
            if n.val == 0 {
                if value != 0 {
                    array.push(value);
                    value = 0;
                }
            } else {
                value += n.val;
            }

            node = n.next;
        }

        let mut current = None;

        for &i in array.iter().rev() {
            let mut n = ListNode::new(i);
            n.next = current;
            current = Some(Box::new(n));
        }

        current
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2181() {
        assert_eq!(to_list(vec![4, 11]), Solution::merge_nodes(to_list(vec![0, 3, 1, 0, 4, 5, 2, 0])));
        assert_eq!(to_list(vec![1, 3, 4]), Solution::merge_nodes(to_list(vec![0, 1, 0, 3, 0, 2, 2, 0])));
    }
}
