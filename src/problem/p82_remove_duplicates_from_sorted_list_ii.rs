/**
 * [82] Remove Duplicates from Sorted List II
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
        // 创建新链表的头结点
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;

        let mut prev = None;

        while let Some(mut node) = head {
            head = node.next.take();

            let now = node.val;

            if head.as_ref().map_or(true, |next| next.val != now)
                && prev.map_or(true, |pre| pre != now)
            {
                tail.next = Some(node);
                tail = tail.next.as_mut()?;
            }

            prev = Some(now);
        }

        dummy.next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_82() {}
}
