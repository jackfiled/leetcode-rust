/**
 * [51] N-Queens
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;

        let mut chessboard = Vec::with_capacity(n);
        let mut result = vec![];

        Self::dfs(0, n, &mut chessboard, &mut result);

        result
    }

    fn dfs(y: usize, n: usize, chessboard: &mut Vec<usize>, result: &mut Vec<Vec<String>>) {
        if y == n {
            result.push(
                chessboard
                    .iter()
                    .map(|&x| {
                        let mut row = vec!['.'; n];
                        row[x] = 'Q';

                        row.into_iter().collect()
                    })
                    .collect(),
            );
            return;
        }

        for x in 0..n {
            if chessboard.iter().any(|&dx| dx == x) {
                continue;
            }

            if chessboard
                .iter()
                .enumerate()
                .any(|(dy, &dx)| x.abs_diff(dx) == y.abs_diff(dy))
            {
                continue;
            }

            chessboard.push(x);
            Self::dfs(y + 1, n, chessboard, result);
            chessboard.pop();
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_51() {
        assert_eq!(2, Solution::solve_n_queens(4).len());
        assert_eq!(vec![vec_string!("Q")], Solution::solve_n_queens(1));
    }
}
