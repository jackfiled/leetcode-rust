pub struct Solution {}

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn magic_tower(nums: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::new();

        let mut count = 0;
        let (mut now, mut delay) = (1i64, 0i64);

        for num in nums {
            if num < 0 {
                heap.push(Reverse(num));
            }

            now += num as i64;
            if now <= 0 {
                count += 1;

                let m = heap.pop().unwrap().0 as i64;
                now -= m;
                delay += m;
            }
        }

        now += delay;

        if now <= 0 {
            -1
        } else {
            count
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcp30() {
        assert_eq!(
            Solution::magic_tower(vec![100, 100, 100, -250, -60, -140, -50, -50, 100, 150]),
            1
        );
        assert_eq!(Solution::magic_tower(vec![-200, -300, 400, 0]), -1);
    }
}
