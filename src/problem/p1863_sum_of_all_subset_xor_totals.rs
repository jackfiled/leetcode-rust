/**
 * [1863] Sum of All Subset XOR Totals
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let mut result = 0;

        Self::search(0, 0, &nums, &mut result);

        result
    }

    fn search(mut sum: i32, i: usize, nums: &Vec<i32>, result: &mut i32) {
        if i == nums.len() {
            return;
        }

        // 不选择i
        Self::search(sum, i + 1, nums, result);
        // 选择i
        sum = sum ^ nums[i];
        *result += sum;
        Self::search(sum, i + 1, nums, result);
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1863() {
        assert_eq!(6, Solution::subset_xor_sum(vec![1, 3]));
        assert_eq!(28, Solution::subset_xor_sum(vec![5, 1, 6]));
        assert_eq!(480, Solution::subset_xor_sum(vec![3, 4, 5, 6, 7, 8]));
    }
}
