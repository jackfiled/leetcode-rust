/**
 * [2929] Distribute Candies Among Children II
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i64 {
        // 0 <= i <= min(n, limit)
        // 0 <= j <= min(n, limit)
        // 0 <= n - i - j <= min(n, limit) -> i + j <= n
        let (n, limit) = (n as i64, limit as i64);
        (0..=n.min(limit))
            .map(|i| (limit.min(n - i) - (n - limit - i).max(0) + 1).max(0))
            .sum()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2929() {
        assert_eq!(3, Solution::distribute_candies(5, 2));
        assert_eq!(10, Solution::distribute_candies(3, 3));
    }
}
