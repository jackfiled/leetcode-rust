/**
 * [73] Set Matrix Zeroes
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        let n = matrix[0].len();

        let mut width_empty = false;
        let mut height_empty = false;

        for i in 0..m {
            if matrix[i][0] == 0 {
                height_empty = true;
            }
        }

        for j in 0..n {
            if matrix[0][j] == 0 {
                width_empty = true;
            }
        }

        for i in 1..m {
            for j in 1..n {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }

        for i in (1..n) {
            if matrix[0][i] == 0 {
                for j in 0..m {
                    matrix[j][i] = 0;
                }
            }
        }

        for i in 1..m {
            if matrix[i][0] == 0 {
                for j in 0..n {
                    matrix[i][j] = 0;
                }
            }
        }

        if width_empty {
            for j in 0..n {
                matrix[0][j] = 0;
            }
        }

        if height_empty {
            for i in 0..m {
                matrix[i][0] = 0;
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_73() {}
}
