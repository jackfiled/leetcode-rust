/**
 * [2338] Count the Number of Ideal Arrays
 */
pub struct Solution {}

// submission codes start here

const MOD: usize = 1_000_000_007;
const MAX_N: usize = 10010;
const MAX_P: usize = 15;

impl Solution {
    pub fn ideal_arrays(n: i32, max_value: i32) -> i32 {
        let mut result = 0usize;
        let n = n as usize;
        let (prime_sieves, dp) = Self::calculate_dp_array();

        for x in 1..=max_value {
            let mut current = 1;
            for &p in prime_sieves[x as usize].iter() {
                current = current * dp[n + p - 1][p] % MOD;
            }
            result = (result + current) % MOD;
        }

        result as i32
    }

    fn calculate_dp_array() -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
        // 最小质因数列表
        let mut sieves = vec![0; MAX_N];

        for i in 2..MAX_N {
            if sieves[i] == 0 {
                for j in (i..MAX_N).step_by(i) {
                    sieves[j] = i;
                }
            }
        }

        let mut prime_sieves = vec![vec![]; MAX_N];
        for i in 2..MAX_N {
            let mut x = i;

            while x > 1 {
                let p = sieves[x];
                let mut current = 0;
                while x % p == 0 {
                    x /= p;
                    current += 1;
                }

                prime_sieves[i].push(current);
            }
        }

        let mut dp = vec![vec![0usize; MAX_P + 1]; MAX_N + MAX_P];
        dp[0][0] = 1;
        for i in 1..(MAX_N + MAX_P) {
            dp[i][0] = 1;
            for j in 1..(i.min(MAX_P) + 1) {
                dp[i][j] = (dp[i - 1][j] + dp[i - 1][j - 1]) % MOD;
            }
        }

        (prime_sieves, dp)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2338() {
        assert_eq!(10, Solution::ideal_arrays(2, 5));
        assert_eq!(11, Solution::ideal_arrays(5, 3));
    }
}
