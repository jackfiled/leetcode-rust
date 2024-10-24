/**
 * [887] Super Egg Drop
 */
pub struct Solution {}

// submission codes start here
use std::cmp::Ordering;
use std::collections::HashMap;

impl Solution {
    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        let mut map = HashMap::new();

        Self::dp(k, n, &mut map)
    }

    fn dp(k: i32, n: i32, memory: &mut HashMap<i32, i32>) -> i32 {
        if let Some(value) = memory.get(&(n * 100 + k)) {
            return *value;
        }

        let result = if n == 0 {
            0
        } else if k == 1 {
            n
        } else {
            let (mut left, mut right) = (1, n);

            while left + 1 < right {
                let middle = (left + right) / 2;

                let left_value = Self::dp(k - 1, middle - 1, memory);
                let right_value = Self::dp(k, n - middle, memory);

                match left_value.cmp(&right_value) {
                    Ordering::Less => {
                        left = middle;
                    }
                    Ordering::Equal => {
                        left = middle;
                        right = middle;
                    }
                    Ordering::Greater => {
                        right = middle;
                    }
                }
            }

            1 + Self::dp(k - 1, left - 1, memory)
                .max(Self::dp(k, n - left, memory))
                .min(Self::dp(k - 1, right - 1, memory).max(Self::dp(k, n - right, memory)))
        };

        memory.insert(n * 100 + k, result);
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_887() {
        assert_eq!(2, Solution::super_egg_drop(1, 2));
        assert_eq!(3, Solution::super_egg_drop(2, 6));
        assert_eq!(4, Solution::super_egg_drop(3, 14));
    }
}
