/**
 * [1928] Minimum Cost to Reach Destination in Time
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn min_cost(max_time: i32, edges: Vec<Vec<i32>>, passing_fees: Vec<i32>) -> i32 {
        let n = passing_fees.len();
        let max_time = max_time as usize;

        let max_value = i32::MAX / 2;
        let mut dp = vec![vec![max_value;n];max_time + 1];

        dp[0][0] = passing_fees[0];

        for t in 1..=max_time {
            for edge in edges.iter() {
                let (i, j, cost) = (edge[0] as usize, edge[1] as usize, edge[2] as usize);

                if cost <= t {
                    dp[t][i] = dp[t][i].min(dp[t - cost][j] + passing_fees[i]);
                    dp[t][j] = dp[t][j].min(dp[t - cost][i] + passing_fees[j]);
                }
            }
        }
        
        let mut result = max_value;

        for t in 1..=max_time {
            result = result.min(dp[t][n - 1]);
        }

        if result == max_value {
            -1
        } else {
            result
        }
   }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1928() {
        
    }
}
