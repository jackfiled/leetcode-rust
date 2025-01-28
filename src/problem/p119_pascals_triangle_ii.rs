/**
 * [119] Pascal's Triangle II
 */
pub struct Solution {}

// submission codes start here
//0 1
//1 1 1
//2 1 2 1
//3 1 3 3 1
//4 1 4 6 4 1
//5 1 5 10 10 5 1
//6 1 6 15 20 15 6 1

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let row_index = row_index as usize;
        let mut matrix = vec![vec![0; row_index + 2]; row_index + 1];
        matrix[0][1] = 1;

        for i in 1..=row_index {
            for j in 1..=i + 1 {
                matrix[i][j] = matrix[i - 1][j - 1] + matrix[i - 1][j];
            }
        }

        matrix[row_index][1..].iter().map(|x| *x).collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_119() {
        assert_eq!(vec![1, 3, 3, 1], Solution::get_row(3));
        assert_eq!(vec![1], Solution::get_row(0));
        assert_eq!(vec![1, 1], Solution::get_row(1));
    }
}
