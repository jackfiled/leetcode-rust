/**
 * [2708] Maximum Strength of a Group
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn max_strength(nums: Vec<i32>) -> i64 {
        if nums.len() == 1 {
            return nums[0] as i64;
        }

        let mut nums = nums;
        nums.sort();

        let mut now = 0;
        let mut result = 1;
        let mut found = false;

        while now < nums.len() {
            if nums[now] > 0 {
                break;
            }

            if now + 1 < nums.len() && nums[now] * nums[now + 1] > 0 {
                let r = nums[now] * nums[now + 1];
                result *= r as i64;
                found = true;
                now += 2;
            } else {
                now += 1;
            }
        }

        while now < nums.len() {
            result *= nums[now] as i64;
            found = true;

            now += 1;
        }

        match found {
            true => result,
            false => 0,
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2708() {
        assert_eq!(20, Solution::max_strength(vec![-4, -5, -4]));
        assert_eq!(1350, Solution::max_strength(vec![3, -1, -5, 2, 5, -9]));
    }
}
