pub struct Solution;

use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        let k = k as i64;
        
        for i in nums {
            let i = i as i64;
            heap.push(Reverse(i));
        }

        let mut result = 0;

        while heap.peek().unwrap().0 < k {
            let x = heap.pop().unwrap().0;
            let y = heap.pop().unwrap().0;

            heap.push(Reverse(x + y + x.min(y)));
            result += 1;
        }


        result
    }
}