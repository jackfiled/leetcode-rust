/**
 * [221] Maximal Square
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut dp = vec![vec![0; n]; m];

        let mut result: i32 = 0;
        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == '0' {
                    continue;
                }

                if i == 0 || j == 0 {
                    dp[i][j] = 1;
                } else { 
                    dp[i][j] = dp[i - 1][j - 1].min(dp[i - 1][j]).min(dp[i][j - 1]) + 1;
                }
                
                result = result.max(dp[i][j]);
            }
        }

        result.pow(2)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_221() {
        assert_eq!(1, Solution::maximal_square(vec![vec!['0', '1'], vec!['1', '0']]));
        assert_eq!(0, Solution::maximal_square(vec![vec!['0']]));
    }
}
