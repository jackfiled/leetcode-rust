/**
 * [2829] Determine the Minimum Sum of a k-avoiding Array
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashSet;

impl Solution {
    pub fn minimum_sum(n: i32, k: i32) -> i32 {
        let mut avoid_set = HashSet::new();

        let mut result = 0;
        let mut num = 1;

        for _ in 0..n {
            while avoid_set.contains(&num) {
                num += 1;
            }

            result += num;
            if k - num > 0 {
                avoid_set.insert(k - num);
            }
            num += 1;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2829() {
        assert_eq!(18, Solution::minimum_sum(5, 4));
        assert_eq!(3, Solution::minimum_sum(2, 6));
    }
}
