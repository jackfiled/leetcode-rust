/**
 * [698] Partition to K Equal Sum Subsets
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        let target = nums.iter().sum::<i32>() / k;
        if target * k != nums.iter().sum::<i32>() {
            return false;
        }
        let mut nums = nums;

        nums.sort();
        if *nums.last().unwrap() > target {
            return false;
        }

        let state = (1 << nums.len()) - 1;
        let mut dp = vec![true; 1 << nums.len()];

        Self::search(state, 0, &mut dp, target, &nums)
    }

    fn search(state: usize, now: i32, dp: &mut Vec<bool>, target: i32, nums: &Vec<i32>) -> bool {
        if state == 0 {
            return true;
        }

        if !dp[state] {
            return dp[state];
        }

        dp[state] = false;
        for (i, &n) in nums.iter().enumerate() {
            if n + now > target {
                break;
            }

            if (state >> i) & 1 == 1
                && Self::search(state ^ (1 << i), (now + n) % target, dp, target, nums)
            {
                return true;
            }
        }

        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_698() {
        assert!(Solution::can_partition_k_subsets(
            vec![4, 3, 2, 3, 5, 2, 1],
            4
        ));
    }
}
