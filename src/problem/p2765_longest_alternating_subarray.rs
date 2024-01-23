/**
 * [2765] Longest Alternating Subarray
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn alternating_subarray(nums: Vec<i32>) -> i32 {
        let mut result = -1;

        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[j] - nums[i] != ((j - i) % 2) as i32 {
                    break;
                }

                result = result.max((j - i + 1) as i32);
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
    fn test_2765() {
        assert_eq!(Solution::alternating_subarray(vec![2,3,4,3,4]), 4);
        assert_eq!(Solution::alternating_subarray(vec![4,5,6]), 2);
        assert_eq!(Solution::alternating_subarray(vec![21,9,5]), -1);
        assert_eq!(Solution::alternating_subarray(vec![21,22]), 2);
    }
}
