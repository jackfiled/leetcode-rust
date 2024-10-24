/**
 * [2809] Minimum Time to Make Array Sum At Most x
 */
pub struct Solution {}

// submission codes start here

struct Pair {
    num1: i32,
    num2: i32,
}

use std::cmp::max;

impl Solution {
    pub fn minimum_time(nums1: Vec<i32>, nums2: Vec<i32>, x: i32) -> i32 {
        let mut pairs = Vec::with_capacity(nums1.len());
        let sum1: i32 = nums1.iter().sum();
        let sum2: i32 = nums2.iter().sum();
        for (index, value) in nums1.iter().enumerate() {
            pairs.push(Pair {
                num1: *value,
                num2: nums2[index],
            });
        }

        pairs.sort_by(|a, b| a.num2.cmp(&b.num2));

        let mut dp = vec![0; nums1.len() + 1];
        for i in 1..=nums1.len() {
            let (num1, num2) = (pairs[i - 1].num1, pairs[i - 1].num2);
            for j in (1..=i).rev() {
                dp[j] = max(dp[j], dp[j - 1] + num2 * j as i32 + num1);
            }
        }

        for i in 0..=nums1.len() {
            let j = i as i32;

            if sum1 + sum2 * j - dp[i] <= x {
                return j;
            }
        }

        -1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2809() {
        assert_eq!(Solution::minimum_time(vec![1, 2, 3], vec![1, 2, 3], 4), 3);
        assert_eq!(Solution::minimum_time(vec![1, 2, 3], vec![3, 3, 3], 4), -1);
        assert_eq!(
            Solution::minimum_time(vec![4, 4, 9, 10], vec![4, 4, 1, 3], 16),
            4
        );
    }
}
