/**
 * [2312] Selling Pieces of Wood
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn selling_wood(m: i32, n: i32, prices: Vec<Vec<i32>>) -> i64 {
        let mut prices_map = HashMap::with_capacity(prices.len());

        for price in prices {
            prices_map.insert(
                Solution::hash(price[0] as usize, price[1] as usize),
                price[2] as i64,
            );
        }

        let (m, n) = (m as usize, n as usize);

        let mut dp = vec![vec![-1; n + 1]; m + 1];

        Solution::dfs(m, n, &mut dp, &prices_map)
    }

    fn hash(x: usize, y: usize) -> usize {
        return x * 1000 + y;
    }

    fn dfs(x: usize, y: usize, dp: &mut Vec<Vec<i64>>, prices_map: &HashMap<usize, i64>) -> i64 {
        if dp[x][y] != -1 {
            return dp[x][y];
        }

        let mut result = *prices_map.get(&Solution::hash(x, y)).unwrap_or_else(|| &0);

        if x > 1 {
            for i in 1..x {
                result = result.max(
                    Solution::dfs(i, y, dp, prices_map) + Solution::dfs(x - i, y, dp, prices_map),
                );
            }
        }

        if y > 1 {
            for j in 1..y {
                result = result.max(
                    Solution::dfs(x, j, dp, prices_map) + Solution::dfs(x, y - j, dp, prices_map),
                );
            }
        }

        dp[x][y] = result;
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2312() {}
}
