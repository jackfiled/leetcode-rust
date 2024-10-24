/**
 * [3192] Minimum Operations to Make Binary Array Elements Equal to One II
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut change_time = 0;

        for i in 0..nums.len() {
            let num = if change_time % 2 == 1 {
                if nums[i] == 1 {
                    0
                } else {
                    1
                }
            } else {
                nums[i]
            };

            if num == 1 {
                continue;
            }

            change_time += 1;
            result += 1;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3192() {
        assert_eq!(4, Solution::min_operations(vec![0, 1, 1, 0, 1]));
        assert_eq!(1, Solution::min_operations(vec![1, 0, 0, 0]));
    }
}
