/**
 * [3097] Shortest Subarray With OR at Least K II
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut bits = [0; 31];
        let mut result = i32::MAX;

        let check = |b: &[i32; 31]| {
            let mut real_k = 0;

            for (i, &v) in b.iter().enumerate() {
                if v > 0 {
                    real_k |= 1 << i;
                }
            }

            real_k >= k
        };

        let mut left = 0;
        for right in 0..nums.len() {
            for i in 0..31 {
                bits[i] += nums[right] >> i & 1;
            }

            while left <= right && check(&bits) {
                result = result.min((right - left + 1) as i32);

                for i in 0..31 {
                    bits[i] -= nums[left] >> i & 1;
                }
                left += 1;
            }
        }

        if result == i32::MAX {
            -1
        } else {
            result
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3097() {
        assert_eq!(1, Solution::minimum_subarray_length(vec![1, 2, 3], 2));
        assert_eq!(3, Solution::minimum_subarray_length(vec![2, 1, 8], 10));
        assert_eq!(1, Solution::minimum_subarray_length(vec![1, 2], 0));
    }
}
