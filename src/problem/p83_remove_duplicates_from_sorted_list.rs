/**
 * [83] Remove Duplicates from Sorted List
 */
pub struct Solution {}

use crate::util::linked_list::{to_list, ListNode};

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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut now = head.as_mut();

        while let Some(node) = now.take() {
            while let Some(next) = node.next.as_mut() {
                if node.val == next.val {
                    node.next = next.next.take();
                } else {
                    break;
                }
            }

            now = node.next.as_mut();
        }

        head
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_83() {}
}
