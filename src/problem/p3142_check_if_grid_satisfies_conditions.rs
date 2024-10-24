/**
 * [3142] Check if Grid Satisfies Conditions
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn satisfies_conditions(grid: Vec<Vec<i32>>) -> bool {
        let (m, n) = (grid.len(), grid[0].len());

        for i in 0..n {
            if i != 0 && grid[0][i] == grid[0][i - 1] {
                return false;
            }

            for j in 1..m {
                if grid[j][i] != grid[j - 1][i] {
                    return false;
                }
            }
        }

        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3142() {
        assert!(Solution::satisfies_conditions(vec![
            vec![1, 0, 2],
            vec![1, 0, 2]
        ]));
        assert!(!Solution::satisfies_conditions(vec![
            vec![1, 1, 1],
            vec![0, 0, 0]
        ]));
        assert!(Solution::satisfies_conditions(vec![vec![0]]));
        assert!(!Solution::satisfies_conditions(vec![
            vec![1],
            vec![2],
            vec![3]
        ]));
    }
}
