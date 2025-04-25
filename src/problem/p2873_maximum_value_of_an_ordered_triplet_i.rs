/**
 * [2873] Maximum Value of an Ordered Triplet I
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let nums: Vec<i64> = nums.into_iter().map(|x| x as i64).collect();

        let mut result = 0;
        let n = nums.len();

        for i in 0..n {
            for j in i + 1..n {
                for k in j + 1..n {
                    result = result.max((nums[i] - nums[j]) * nums[k])
                }
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
    fn test_2873() {
        assert_eq!(77, Solution::maximum_triplet_value(vec![12, 6, 1, 2, 7]));
        assert_eq!(133, Solution::maximum_triplet_value(vec![1, 10, 3, 4, 19]));
        assert_eq!(0, Solution::maximum_triplet_value(vec![1, 2, 3]));
    }
}
