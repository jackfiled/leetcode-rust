/**
 * [3191] Minimum Operations to Make Binary Array Elements Equal to One I
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut nums = nums;

        let mut pos = 0;

        while pos < nums.len() - 3 {
            if nums[pos] == 1 {
                pos += 1;
                continue;
            }

            nums[pos] = 1;
            Self::reverse(pos + 1, &mut nums);
            Self::reverse(pos + 2, &mut nums);

            result += 1;
            pos += 1;
        }

        if nums[pos] ^ nums[pos + 1] == 0 && nums[pos + 1] ^ nums[pos + 2] == 0 {
            if nums[pos] == 1 {
                result
            } else {
                result + 1
            }
        } else {
            -1
        }
    }

    fn reverse(pos: usize, nums: &mut Vec<i32>) {
        nums[pos] = if nums[pos] == 1 { 0 } else { 1 }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3191() {
        assert_eq!(-1, Solution::min_operations(vec![0, 1, 1, 0, 1, 0, 0]));
        assert_eq!(3, Solution::min_operations(vec![0, 1, 1, 1, 0, 0]));
        assert_eq!(-1, Solution::min_operations(vec![0, 1, 1, 1]));
    }
}
