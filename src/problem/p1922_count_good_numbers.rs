/**
 * [1922] Count Good Numbers
 */
pub struct Solution {}

// submission codes start here

const MOD: i64 = 1_000_000_000 + 7;

impl Solution {
    pub fn count_good_numbers(n: i64) -> i32 {
        let mut result = Self::quick_power(20, n / 2);

        if n % 2 == 1 {
            result = result * 5 % MOD;
        }

        result as i32
    }

    fn quick_power(mut m: i64, mut n: i64) -> i64 {
        let mut result = 1;

        while n > 0 {
            if n & 1 == 1 {
                result = result * m % MOD;
            }

            m = m * m % MOD;
            n = n >> 1;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1922() {
        assert_eq!(5, Solution::count_good_numbers(1));
        assert_eq!(400, Solution::count_good_numbers(4));
        assert_eq!(564_908_303, Solution::count_good_numbers(50));
    }
}
