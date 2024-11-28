/**
 * [3250] Find the Count of Monotonic Pairs I
 */
pub struct Solution {}

// submission codes start here

const MOD: i32 = 1_000_000_007;

impl Solution {
    pub fn count_of_pairs(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let nums: Vec<usize> = nums.into_iter().map(|x| x as usize).collect();
        let max_num = *nums.iter().max().unwrap();

        let mut dp = vec![vec![0; max_num + 1]; n];

        for i in 0..=nums[0] {
            dp[0][i] = 1;
        }

        for i in 1..n {
            for v2 in 0..=nums[i] {
                for v1 in 0..=v2 {
                    if nums[i - 1] >= v1 && nums[i - 1] - v1 >= nums[i] - v2 {
                        dp[i][v2] = (dp[i][v2] + dp[i - 1][v1]) % MOD;
                    }
                }
            }
        }

        dp[n - 1].iter().fold(0, |acc, e| (acc + *e) % MOD)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3250() {
        assert_eq!(4, Solution::count_of_pairs(vec![2, 3, 2]));
        assert_eq!(126, Solution::count_of_pairs(vec![5, 5, 5, 5]));
    }
}
