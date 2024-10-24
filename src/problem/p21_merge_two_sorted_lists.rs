/**
 * [21] Merge Two Sorted Lists
 */
pub struct Solution {}

use std::borrow::{Borrow, BorrowMut};

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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut now = &mut head;

        let (mut list1, mut list2) = (list1, list2);

        *now = loop {
            match (list1, list2) {
                (Some(mut a), Some(mut b)) => {
                    if a.val < b.val {
                        list1 = a.next.take();
                        list2 = Some(b);

                        now = &mut now.insert(a).next;
                    } else {
                        list1 = Some(a);
                        list2 = b.next.take();

                        now = &mut now.insert(b).next
                    }
                }
                (x, y) => break x.or(y),
            }
        };

        head
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_21() {}
}
