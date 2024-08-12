/**
 * [72] Edit Distance
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (word1, word2): (Vec<char>, Vec<char>) = (word1.chars().collect(), word2.chars().collect());
        let (n, m) = (word1.len(), word2.len());
        
        // 存在一个字符串有空串
        if m * n == 0 {
            return (m + n) as i32;
        }
        
        let mut dp = vec![vec![0;m + 1]; n + 1];
        
        // 初始化dp数组
        for i in 0..=n {
            dp[i][0] = i;
        }
        
        for i in 0..=m {
            dp[0][i] = i;
        }
        
        for i in 1..=n {
            for j in 1..=m {
                let left = dp[i - 1][j] + 1;
                let down = dp[i][j - 1] + 1;
                let mut left_down = dp[i - 1][ j -1];
                if word1[i - 1] != word2[j - 1] {
                    left_down += 1;
                }
                
                dp[i][j] = left.min(down).min(left_down);
            }
        }
        
        dp[n][m] as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_72() {
        assert_eq!(3, Solution::min_distance("horse".to_owned(), "ros".to_owned()));
    }
}
