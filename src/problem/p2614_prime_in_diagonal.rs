/**
 * [2614] Prime In Diagonal
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn diagonal_prime(nums: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let mut result = 0;

        for i in 0..n {
            if Self::is_prime(nums[i][i]) {
                result = result.max(nums[i][i]);
            }

            if Self::is_prime(nums[i][n - i - 1]) {
                result = result.max(nums[i][n - i - 1]);
            }
        }

        result
    }

    fn is_prime(num: i32) -> bool {
        if num == 1 {
            return false;
        }

        !(2..=num.isqrt()).any(|x| num % x == 0)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2614() {
        assert_eq!(
            11,
            Solution::diagonal_prime(vec![vec![1, 2, 3], vec![5, 6, 7], vec![9, 10, 11]])
        );
        assert_eq!(
            17,
            Solution::diagonal_prime(vec![vec![1, 2, 3], vec![5, 17, 7], vec![9, 10, 11]])
        );
    }
}
