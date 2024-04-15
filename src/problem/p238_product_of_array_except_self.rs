/**
 * [238] Product of Array Except Self
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![1; nums.len()];

        for i in 1..nums.len() {
            result[i] = result[i - 1] * nums[i - 1];
        }

        let mut suffix = 1;

        for i in (0..nums.len() - 1).rev() {
            suffix *= nums[i + 1];

            result[i] *= suffix;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_238() {
        assert_eq!(
            vec![24, 12, 8, 6],
            Solution::product_except_self(vec![1, 2, 3, 4])
        );
        assert_eq!(
            vec![0, 0, 9, 0, 0],
            Solution::product_except_self(vec![-1, 1, 0, -3, 3])
        );
    }
}
