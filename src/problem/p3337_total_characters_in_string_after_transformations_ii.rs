/**
 * [3337] Total Characters in String After Transformations II
 */
pub struct Solution {}

// submission codes start here
use std::ops::Mul;

const LENGTH: usize = 26;
const MOD: i64 = 1_000_000_007;

#[derive(Debug, Copy, Clone)]
struct Matrix {
    array: [[i64; LENGTH]; LENGTH],
}

impl Matrix {
    fn new() -> Self {
        Self {
            array: [[0; LENGTH]; LENGTH],
        }
    }

    fn unit_matrix() -> Self {
        let mut matrix = Self::new();

        for i in 0..LENGTH {
            matrix.array[i][i] = 1;
        }

        matrix
    }

    fn quick_multiply(&self, mut y: i64) -> Self {
        let mut result = Self::unit_matrix();
        let mut current = self.clone();

        while y > 0 {
            if y & 1 == 1 {
                result = result * current;
            }

            current = current * current;
            y >>= 1;
        }

        result
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Self::new();

        for i in 0..LENGTH {
            for j in 0..LENGTH {
                for k in 0..LENGTH {
                    result.array[i][j] =
                        (result.array[i][j] + self.array[i][k] * rhs.array[k][j]) % MOD;
                }
            }
        }

        result
    }
}

impl Solution {
    pub fn length_after_transformations(s: String, t: i32, nums: Vec<i32>) -> i32 {
        let mut t_matrix = Matrix::new();

        for i in 0..LENGTH {
            for j in 1..=nums[i] {
                let j = j as usize;
                t_matrix.array[(i + j) % LENGTH][i] = 1;
            }
        }

        let transfer_matrix = t_matrix.quick_multiply(t as i64);
        let mut result = 0;
        let mut chars = [0; LENGTH];

        for c in s.bytes() {
            chars[(c - b'a') as usize] += 1;
        }

        for i in 0..LENGTH {
            for j in 0..LENGTH {
                result = (result + transfer_matrix.array[i][j] * chars[j]) % MOD;
            }
        }

        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3337() {
        assert_eq!(
            5,
            Solution::length_after_transformations(
                "abcyy".to_owned(),
                1,
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2]
            )
        );
        assert_eq!(
            7,
            Solution::length_after_transformations(
                "abcyy".to_owned(),
                2,
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2]
            )
        );
        assert_eq!(
            8,
            Solution::length_after_transformations(
                "azbk".to_string(),
                1,
                vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]
            )
        );
    }
}
