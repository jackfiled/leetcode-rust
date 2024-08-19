/**
 * [552] Student Attendance Record II
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn check_record(n: i32) -> i32 {
        let mod_number = 1_000_000_007;
        let n =n as usize;
        
        let mut dp = vec![vec![vec![0;3];2];n + 1];
        dp[0][0][0] = 1;
        
        for i in 1..=n {
            // record ends with 'P'
            for j in 0..2 {
                for k in 0..3 {
                    dp[i][j][0] = (dp[i][j][0] + dp[i - 1][j][k]) % mod_number;
                }
            }
            
            // record ends with 'A'
            for k in 0..3 {
                dp[i][1][0] = (dp[i][1][0] + dp[i - 1][0][k]) % mod_number;
            }
            
            // record ends with 'L'
            for j in 0..2 {
                for k in 1..3 {
                    dp[i][j][k] = (dp[i][j][k] + dp[i - 1][j][k - 1]) % mod_number;
                }
            }
        }
        
        let mut result = 0;
        for i in 0..2 {
            for j in 0..3 {
                result = (result + dp[n][i][j]) % mod_number;  
            }
        }
        
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_552() {
        assert_eq!(8, Solution::check_record(2));
        assert_eq!(3, Solution::check_record(1));
        assert_eq!(183_236_316, Solution::check_record(10101));
    }
}
