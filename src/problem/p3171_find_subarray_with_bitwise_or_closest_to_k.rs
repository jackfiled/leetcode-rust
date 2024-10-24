/**
 * [3171] Find Subarray With Bitwise OR Closest to K
 */
pub struct Solution {}

// submission codes start here
use std::cmp::Ordering;

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut bits_max_pos = vec![None; 31];
        let mut pos_to_bit = Vec::with_capacity(31);
        let mut result = i32::MAX;

        for i in 0..n {
            for j in 0..=30 {
                if nums[i] >> j & 1 == 1 {
                    bits_max_pos[j] = Some(i)
                }
            }

            pos_to_bit.clear();
            for j in 0..=30 {
                if let Some(pos) = bits_max_pos[j] {
                    pos_to_bit.push((pos, j));
                }
            }

            pos_to_bit.sort_by(|a, b| match b.0.cmp(&a.0) {
                Ordering::Less => Ordering::Less,
                Ordering::Equal => b.1.cmp(&a.1),
                Ordering::Greater => Ordering::Greater,
            });

            let mut value = 0;
            let (mut left, mut right) = (0, 0);

            while right < pos_to_bit.len() {
                while right < pos_to_bit.len() && pos_to_bit[right].0 == pos_to_bit[left].0 {
                    value |= 1 << pos_to_bit[right].1;
                    right += 1;
                }

                result = result.min((value - k).abs());
                left = right;
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
    fn test_3171() {
        assert_eq!(0, Solution::minimum_difference(vec![1, 2, 4, 5], 3));
        assert_eq!(1, Solution::minimum_difference(vec![1, 3, 1, 3], 2));
        assert_eq!(9, Solution::minimum_difference(vec![1], 10));
    }
}
