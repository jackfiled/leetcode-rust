/**
 * [2643] Row With Maximum Ones
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let r = mat.iter().enumerate().rev().fold(
            (mat.len() - 1, 0),
            |(r_line, r_count), (line, row)| {
                let c = row.iter().filter(|x| **x == 1).count();

                if c >= r_count {
                    (line, c)
                } else {
                    (r_line, r_count)
                }
            },
        );

        vec![r.0 as i32, r.1 as i32]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2643() {
        assert_eq!(
            vec![0, 1],
            Solution::row_and_maximum_ones(vec![vec![0, 1], vec![1, 0]])
        );
        assert_eq!(
            vec![1, 2],
            Solution::row_and_maximum_ones(vec![vec![0, 0, 0], vec![0, 1, 1]])
        );
        assert_eq!(
            vec![1, 2],
            Solution::row_and_maximum_ones(vec![vec![0, 0], vec![1, 1], vec![0, 0]])
        );
    }
}
