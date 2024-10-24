/**
 * [232] Implement Queue using Stacks
 */
pub struct Solution {}

// submission codes start here
use std::collections::VecDeque;

struct MyQueue {
    in_stack: VecDeque<i32>,
    out_stack: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    fn new() -> Self {
        MyQueue {
            in_stack: VecDeque::new(),
            out_stack: VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.in_stack.push_back(x);
    }

    fn pop(&mut self) -> i32 {
        if self.out_stack.is_empty() {
            while let Some(top) = self.in_stack.pop_back() {
                self.out_stack.push_back(top);
            }
        }

        self.out_stack.pop_back().unwrap()
    }

    fn peek(&mut self) -> i32 {
        if self.out_stack.is_empty() {
            while let Some(top) = self.in_stack.pop_back() {
                self.out_stack.push_back(top);
            }
        }

        *self.out_stack.back().unwrap()
    }

    fn empty(&self) -> bool {
        self.out_stack.is_empty() && self.in_stack.is_empty()
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_232() {
        let mut queue = MyQueue::new();

        queue.push(1);
        queue.push(2);

        assert_eq!(1, queue.peek());
        assert_eq!(1, queue.pop());
        assert!(!queue.empty());
    }
}
