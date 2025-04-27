/**
 * [3392] Count Subarrays of Length Three With a Condition
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>) -> i32 {
        nums.iter()
            .enumerate()
            .skip(2)
            .filter_map(|(i, &v)| {
                if (nums[i - 2] + v) * 2 == nums[i - 1] {
                    Some(())
                } else {
                    None
                }
            })
            .count() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3392() {
        assert_eq!(1, Solution::count_subarrays(vec![1, 2, 1, 4, 1]));
        assert_eq!(0, Solution::count_subarrays(vec![1, 1, 1]));
    }
}
