/**
 * [136] Single Number
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut result = 0;

        for i in nums {
            result ^= i;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_136() {
        assert_eq!(1, Solution::single_number(vec![1, 2, 2]));
        assert_eq!(1, Solution::single_number(vec![1]));
    }
}
