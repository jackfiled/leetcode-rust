pub struct Solution {}

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn nums_game(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(nums.len());
        let m = 1000000007i64;

        let (mut lower, mut upper) = (BinaryHeap::new(), BinaryHeap::new());
        let (mut lower_sum, mut upper_sum) = (0i64, 0i64);

        for (index, value) in nums.iter().enumerate() {
            let value = *value - index as i32;

            let peek = lower.peek();
            if peek.is_none() || *peek.unwrap() >= value {
                lower_sum += value as i64;
                lower.push(value);

                if lower.len() > upper.len() + 1 {
                    let peek = lower.pop().unwrap();

                    upper.push(Reverse(peek));
                    upper_sum += peek as i64;
                    lower_sum -= peek as i64;
                }
            } else {
                upper_sum += value as i64;
                upper.push(Reverse(value));

                if lower.len() < upper.len() {
                    let peek = upper.pop().unwrap().0;

                    lower.push(peek);
                    lower_sum += peek as i64;
                    upper_sum -= peek as i64;
                }
            }

            if (index + 1) % 2 == 0 {
                result.push(((upper_sum - lower_sum) % m) as i32);
            } else {
                result.push(((upper_sum - lower_sum + *lower.peek().unwrap() as i64) % m) as i32);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcp24() {
        assert_eq!(
            Solution::nums_game(vec![3, 4, 5, 1, 6, 7]),
            vec![0, 0, 0, 5, 6, 7]
        );
        assert_eq!(
            Solution::nums_game(vec![1, 2, 3, 4, 5]),
            vec![0, 0, 0, 0, 0]
        );
        assert_eq!(Solution::nums_game(vec![471, 626, 848]), vec![0, 154, 375]);
    }
}
