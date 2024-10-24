/**
 * [137] Single Number II
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let (mut a, mut b) = (0, 0);

        for i in nums {
            b = !a & (b ^ i);
            a = !b & (a ^ i);
        }

        b
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_137() {
        assert_eq!(1, Solution::single_number(vec![1, 2, 2, 2]));
    }
}
