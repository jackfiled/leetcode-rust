/**
 * [201] Bitwise AND of Numbers Range
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        let mut right = right;

        while left < right {
            right = right & (right - 1);
        }

        right
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_201() {
        assert_eq!(4, Solution::range_bitwise_and(5, 7));
        assert_eq!(0, Solution::range_bitwise_and(0, 0));
    }
}
