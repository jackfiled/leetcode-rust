/**
 * [292] Nim Game
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        n % 4 != 0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_292() {
        assert!(Solution::can_win_nim(5));
        assert!(!Solution::can_win_nim(4));
    }
}
