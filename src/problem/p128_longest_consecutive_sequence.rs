/**
 * [128] Longest Consecutive Sequence
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let d: HashSet<i32> = nums.iter().map(|&i| i).collect();
        nums.into_iter()
            .map(|mut i| {
                let mut t = 1;
                if !d.contains(&(i - 1)) {
                    while d.contains(&(i + 1)) {
                        t += 1;
                        i += 1;
                    }
                }
                t
            })
            .max()
            .unwrap_or(0)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_128() {
        assert_eq!(4, Solution::longest_consecutive(vec![100, 1, 200, 4, 3, 2]));
    }
}
