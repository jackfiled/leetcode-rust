/**
 * [3343] Count Number of Balanced Permutations
 */
pub struct Solution {}

// submission codes start here

const MOD: usize = 1_000_000_007;

impl Solution {
    pub fn count_balanced_permutations(num: String) -> i32 {
        let num_array: Vec<usize> = num.bytes().map(|c| (c - b'0') as usize).collect();

        let n = num_array.len();
        let sum: usize = num_array.iter().sum();
        if sum % 2 != 0 {
            return 0;
        }

        let mut count = vec![0; 10];
        for &i in num_array.iter() {
            count[i] += 1;
        }

        let target = sum / 2;
        let max_odd_count = (n + 1) / 2;

        let mut combinations = vec![vec![0; max_odd_count + 1]; max_odd_count + 1];
        let mut dp = vec![vec![0; max_odd_count + 1]; target + 1];

        for i in 0..=max_odd_count {
            combinations[i][i] = 1;
            combinations[i][0] = 1;

            for j in 1..i {
                combinations[i][j] = (combinations[i - 1][j] + combinations[i - 1][j - 1]) % MOD;
            }
        }

        dp[0][0] = 1;
        let mut prefix = 0;
        let mut total_sum = 0;
        for i in 0..=9 {
            prefix += count[i];
            total_sum += i * count[i];

            for odd in (prefix
                .checked_sub(n - max_odd_count)
                .or_else(|| Some(0))
                .unwrap()..=prefix.min(max_odd_count))
                .rev()
            {
                let even = prefix - odd;

                for current in (total_sum.checked_sub(target).or_else(|| Some(0)).unwrap()
                    ..=total_sum.min(target))
                    .rev()
                {
                    let mut result = 0;
                    let mut j = count[i].checked_sub(even).or_else(|| Some(0)).unwrap();

                    while j <= count[i].min(odd) && i * j <= current {
                        let ways = combinations[odd][j] * combinations[even][count[i] - j] % MOD;
                        result = (result + ways * dp[current - i * j][odd - j] % MOD) % MOD;
                        j += 1;
                    }

                    dp[current][odd] = result % MOD;
                }
            }
        }

        dp[target][max_odd_count] as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3343() {
        assert_eq!(2, Solution::count_balanced_permutations("123".to_string()));
        assert_eq!(1, Solution::count_balanced_permutations("112".to_string()));
        assert_eq!(
            0,
            Solution::count_balanced_permutations("12345".to_string())
        );
    }
}
