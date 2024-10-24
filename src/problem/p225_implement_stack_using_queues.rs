/**
 * [225] Implement Stack using Queues
 */
pub struct Solution {}

// submission codes start here
use std::collections::VecDeque;

struct MyStack {
    queue: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {
    fn new() -> Self {
        return MyStack {
            queue: VecDeque::new(),
        };
    }

    fn push(&mut self, x: i32) {
        let mut new_queue = VecDeque::with_capacity(self.queue.capacity() + 1);
        new_queue.push_back(x);

        for i in &self.queue {
            new_queue.push_back(*i);
        }

        self.queue = new_queue;
    }

    fn pop(&mut self) -> i32 {
        self.queue.pop_front().unwrap()
    }

    fn top(&self) -> i32 {
        *self.queue.front().unwrap()
    }

    fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}

/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_225() {
        let mut stack = MyStack::new();

        stack.push(1);
        stack.push(2);

        assert_eq!(2, stack.top());
        assert_eq!(2, stack.pop());
        assert!(!stack.empty());
    }
}
