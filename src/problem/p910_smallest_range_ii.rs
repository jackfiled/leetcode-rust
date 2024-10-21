/**
 * [910] Smallest Range II
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn smallest_range_ii(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();

        let mut min = *nums.first().unwrap();
        let mut max = *nums.last().unwrap();
        let mut result = max - min;

        for i in 1..nums.len() {
            let m = (max - k).max(nums[i - 1] + k);
            let n = (min + k).min(nums[i] - k);

            result = result.min((m - n).abs());
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_910() {
        assert_eq!(3, Solution::smallest_range_ii(vec![1, 4, 6, 4], 3));
        assert_eq!(3, Solution::smallest_range_ii(vec![4, 7, 4], 4));
        assert_eq!(0, Solution::smallest_range_ii(vec![1], 0));
        assert_eq!(6, Solution::smallest_range_ii(vec![0, 10], 2));
        assert_eq!(3, Solution::smallest_range_ii(vec![1, 3, 6], 3));
    }
}
