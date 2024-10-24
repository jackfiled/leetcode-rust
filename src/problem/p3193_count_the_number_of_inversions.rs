/**
 * [3193] Count the Number of Inversions
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

const MOD: i32 = 1_000_000_007;

impl Solution {
    pub fn number_of_permutations(n: i32, requirements: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut requirements_map = HashMap::new();
        let mut max_pair_count = 0;

        requirements_map.insert(0, 0);
        for requirement in requirements {
            requirements_map.insert(requirement[0] as usize, requirement[1] as usize);
            max_pair_count = max_pair_count.max(requirement[1]);
        }

        if let Some(count) = requirements_map.get(&0) {
            if *count != 0 {
                return 0;
            }
        }

        let mut dp = vec![vec![-1; max_pair_count as usize + 1]; n];

        if let Some(&count) = requirements_map.get(&(n - 1)) {
            Self::dfs(n - 1, count, &requirements_map, &mut dp)
        } else {
            Self::dfs(n - 1, 0, &requirements_map, &mut dp)
        }
    }

    fn dfs(
        end: usize,
        count: usize,
        requirements: &HashMap<usize, usize>,
        dp: &mut Vec<Vec<i32>>,
    ) -> i32 {
        if end == 0 {
            return 1;
        }

        if dp[end][count] != -1 {
            return dp[end][count];
        }

        if let Some(&c) = requirements.get(&(end - 1)) {
            if c <= count && count <= end + c {
                let r = Self::dfs(end - 1, c, requirements, dp);
                dp[end][count] = r;
                r
            } else {
                dp[end][count] = 0;
                0
            }
        } else {
            let mut r = 0;
            for i in 0..=end.min(count) {
                r = (r + Self::dfs(end - 1, count - i, requirements, dp)) % MOD;
            }

            dp[end][count] = r;
            r
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3193() {
        assert_eq!(1, Solution::number_of_permutations(2, vec![vec![1, 1]]));
        assert_eq!(
            2,
            Solution::number_of_permutations(3, vec![vec![2, 2], vec![0, 0]])
        );
        assert_eq!(
            1,
            Solution::number_of_permutations(3, vec![vec![2, 2], vec![1, 1], vec![0, 0]])
        );
        assert_eq!(
            1,
            Solution::number_of_permutations(2, vec![vec![0, 0], vec![1, 0]])
        );
    }
}
