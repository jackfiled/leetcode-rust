/**
 * [2918] Minimum Equal Sum of Two Arrays After Replacing Zeros
 */
pub struct Solution {}

// submission codes start here
use std::cmp::Ordering;

impl Solution {
    pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let mut nums1_sum: i64 = nums1.iter().map(|x| *x as i64).sum();
        let mut nums2_sum: i64 = nums2.iter().map(|x| *x as i64).sum();
        let nums1_zero_count = nums1.iter().filter(|x| **x == 0).count() as i64;
        let nums2_zero_count = nums2.iter().filter(|x| **x == 0).count() as i64;

        match (nums1_sum + nums1_zero_count).cmp(&(nums2_sum + nums2_zero_count)) {
            Ordering::Less => {
                if nums1_zero_count == 0 {
                    -1
                } else {
                    nums2_sum + nums2_zero_count
                }
            }
            Ordering::Greater => {
                if nums2_zero_count == 0 {
                    -1
                } else {
                    nums1_sum + nums1_zero_count
                }
            }
            Ordering::Equal => nums1_sum + nums1_zero_count,
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2918() {
        assert_eq!(12, Solution::min_sum(vec![3, 2, 0, 1, 0], vec![6, 5, 0]));
        assert_eq!(-1, Solution::min_sum(vec![2, 0, 2, 0], vec![1, 4]));
    }
}
