/**
 * [202] Happy Number
 */
pub struct Solution {}

// submission codes start here
use std::{collections::HashSet, ops::Add};

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut s = HashSet::new();

        let mut n = Solution::calculate_square(n);

        while n != 1 {
            dbg!(n);
            if s.contains(&n) {
                return false;
            }

            s.insert(n);

            n = Solution::calculate_square(n);
        }

        true
    }

    fn calculate_square(n: i32) -> i32 {
        let mut n = n;

        let mut result = 0;

        while n != 0 {
            result += (n % 10).pow(2);
            n = n / 10;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_202() {
        assert!(Solution::is_happy(19));
        assert!(!Solution::is_happy(2));
    }
}
