/**
 * [2789] Largest Element in an Array after Merge Operations
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn max_array_value(nums: Vec<i32>) -> i64 {
        let mut nums: Vec<i64> = nums.iter()
            .map(|x| *x as i64)
            .collect();

            let mut result = nums[0];
            for i in (0..(nums.len() - 1)).rev() {
                if nums[i] <= nums[i + 1] {
                    nums[i] = nums[i] + nums[i + 1];

                    result = result.max(nums[i]);
                }
            }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2789() {
        assert_eq!(21, Solution::max_array_value(vec![2,3,7,9,3]));
        assert_eq!(11, Solution::max_array_value(vec![5,3,3]));   
    }
}
