/**
 * [908] Smallest Range I
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
        let (mut min, mut max) = (i32::MAX, i32::MIN);

        for num in nums {
            min = min.min(num);
            max = max.max(num);
        }

        let delta = max - min;

        if delta <= 2 * k {
            0
        } else {
            delta - 2 * k
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_908() {
        assert_eq!(0, Solution::smallest_range_i(vec![1], 0));
        assert_eq!(6, Solution::smallest_range_i(vec![0, 10], 2));
        assert_eq!(0, Solution::smallest_range_i(vec![1, 3, 6], 3));
    }
}
