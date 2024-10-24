/**
 * [2908] Minimum Sum of Mountain Triplets I
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn minimum_sum(nums: Vec<i32>) -> i32 {
        let mut result = i32::MAX;

        for j in 1..nums.len() - 1 {
            let mut first = i32::MAX;

            for i in 0..j {
                if nums[i] < nums[j] {
                    first = first.min(nums[i]);
                }
            }

            let mut last = i32::MAX;

            for k in j + 1..nums.len() {
                if nums[k] < nums[j] {
                    last = last.min(nums[k]);
                }
            }

            if first == i32::MAX || last == i32::MAX {
                continue;
            }

            result = result.min(first + nums[j] + last);
        }

        return if result == i32::MAX { -1 } else { result };
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2908() {
        assert_eq!(9, Solution::minimum_sum(vec![8, 6, 1, 5, 3]));
    }
}
