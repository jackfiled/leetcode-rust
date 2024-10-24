/**
 * [3164] Find the Number of Good Pairs II
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        use std::collections::HashMap;

        let mut nums1_count = HashMap::with_capacity(nums1.len());
        let mut nums1_max = 0;

        for i in nums1 {
            let entry = nums1_count.entry(i).or_insert(0);
            *entry += 1;

            nums1_max = nums1_max.max(i);
        }

        let mut nums2_count = HashMap::with_capacity(nums2.len());

        for i in nums2 {
            let entry = nums2_count.entry(i).or_insert(0);
            *entry += 1;
        }

        let mut result = 0;

        for (i, count) in nums2_count {
            let mut value = i * k;

            while value <= nums1_max {
                if let Some(&count1) = nums1_count.get(&value) {
                    result += count1 * count;
                }

                value += i * k;
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
    fn test_3164() {
        assert_eq!(
            5,
            Solution::number_of_pairs(vec![1, 3, 4], vec![1, 3, 4], 1)
        );
        assert_eq!(
            2,
            Solution::number_of_pairs(vec![1, 2, 4, 12], vec![2, 4], 3)
        );
    }
}
