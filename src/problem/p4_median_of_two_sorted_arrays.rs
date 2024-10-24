/**
 * [4] Median of Two Sorted Arrays
 *
 * Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.
 * The overall run time complexity should be O(log (m+n)).
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums1 = [1,3], nums2 = [2]
 * Output: 2.00000
 * Explanation: merged array = [1,2,3] and median is 2.
 *
 * <strong class="example">Example 2:
 *
 * Input: nums1 = [1,2], nums2 = [3,4]
 * Output: 2.50000
 * Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
 *
 *  
 * Constraints:
 *
 * 	nums1.length == m
 * 	nums2.length == n
 * 	0 <= m <= 1000
 * 	0 <= n <= 1000
 * 	1 <= m + n <= 2000
 * 	-10^6 <= nums1[i], nums2[i] <= 10^6
 *
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/median-of-two-sorted-arrays/
// discuss: https://leetcode.cn/problems/median-of-two-sorted-arrays/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cmp::min;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let len1 = nums1.len() as i32;
        let len2 = nums2.len() as i32;
        let left = (len1 + len2 + 1) / 2;
        let right = (len1 + len2 + 2) / 2;

        (Solution::get_k_th(&nums1, 0, len1 - 1, &nums2, 0, len2 - 1, left) as f64
            + Solution::get_k_th(&nums1, 0, len1 - 1, &nums2, 0, len2 - 1, right) as f64)
            * 0.5
    }

    fn get_k_th(
        nums1: &Vec<i32>,
        start1: i32,
        end1: i32,
        nums2: &Vec<i32>,
        start2: i32,
        end2: i32,
        k: i32,
    ) -> i32 {
        let len1 = end1 + 1 - start1;
        let len2 = end2 + 1 - start2;

        if len1 > len2 {
            return Solution::get_k_th(nums2, start2, end2, nums1, start1, end1, k);
        }

        if len1 == 0 {
            return nums2[(start2 + k - 1) as usize];
        }

        if k == 1 {
            return min(nums1[start1 as usize], nums2[start2 as usize]);
        }

        let i = start1 + min(len1, k / 2) - 1;
        let j = start2 + min(len2, k / 2) - 1;

        return if nums1[i as usize] > nums2[j as usize] {
            Solution::get_k_th(
                nums1,
                start1,
                end1,
                nums2,
                j + 1,
                end2,
                k - (j - start2 + 1),
            )
        } else {
            Solution::get_k_th(
                nums1,
                i + 1,
                end1,
                nums2,
                start2,
                end2,
                k - (i - start1 + 1),
            )
        };
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4() {
        assert_eq!(
            2.0,
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2])
        );
        assert_eq!(
            2.5,
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4])
        );
        assert_eq!(2.0, Solution::find_median_sorted_arrays(vec![2], vec![]));
    }
}
