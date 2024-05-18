/**
 * [155] Min Stack
 */
pub struct Solution {}


// submission codes start here
use std::{cmp::Reverse, collections::BinaryHeap};

struct MinStack {
    heap : BinaryHeap<Reverse<i32>>,
    stack: Vec<i32>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self {
        MinStack {
            heap: BinaryHeap::new(),
            stack: vec![]
        }
    }
    
    fn push(&mut self, val: i32) {
        self.heap.push(Reverse(val));
        self.stack.push(val);
    }
    
    fn pop(&mut self) {
        let n = self.stack.pop().unwrap_or(0);

        if !self.stack.contains(&n) {
            self.heap.retain(|i| i.0 != n);
        }    
    }
    
    fn top(&self) -> i32 {
        *self.stack.last().unwrap_or(&0)
    }
    
    fn get_min(&self) -> i32 {
        self.heap.peek().unwrap_or(&Reverse(0)).0
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_155() {
    }
}
