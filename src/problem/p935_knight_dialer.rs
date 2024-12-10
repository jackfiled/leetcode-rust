/**
 * [935] Knight Dialer
 */
pub struct Solution {}

// submission codes start here

const MOD: i32 = 1_000_000_007;

impl Solution {
    pub fn knight_dialer(n: i32) -> i32 {
        let n = n as usize;

        let chessboard = vec![
            // 0
            vec![4, 6],
            // 1
            vec![6, 8],
            // 2
            vec![7, 9],
            // 3
            vec![4, 8],
            // 4
            vec![0, 3, 9],
            // 5
            vec![],
            // 6
            vec![0, 1, 7],
            // 7
            vec![2, 6],
            // 8
            vec![1, 3],
            // 9
            vec![2, 4],
        ];

        let mut dp = vec![vec![0; 10], vec![1; 10]];

        for i in 2..=n {
            (0..10).for_each(|j| dp[i % 2][j] = 0);

            for j in 0..10 {
                for &next in chessboard[j].iter() {
                    dp[i % 2][next] = (dp[i % 2][next] + dp[(i - 1) % 2][j]) % MOD;
                }
            }
        }

        let mut result = 0;

        for &i in dp[n % 2].iter() {
            result = (result + i) % MOD;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_935() {
        assert_eq!(10, Solution::knight_dialer(1));
        assert_eq!(20, Solution::knight_dialer(2));
        assert_eq!(136_006_598, Solution::knight_dialer(3131));
    }
}
