/**
 * [52] N-Queens II
 *
 * The n-queens puzzle is the problem of placing n queens on an n x n chessboard such that no two queens attack each other.
 * Given an integer n, return the number of distinct solutions to the n-queens puzzle.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/13/queens.jpg" style="width: 600px; height: 268px;" />
 * Input: n = 4
 * Output: 2
 * Explanation: There are two distinct solutions to the 4-queens puzzle as shown.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: n = 1
 * Output: 1
 * 
 *  
 * Constraints:
 * 
 * 	1 <= n <= 9
 * 
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/n-queens-ii/
// discuss: https://leetcode.cn/problems/n-queens-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::VecDeque;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let n = n as usize;
        let mut result = 0;
        let mut stack = VecDeque::new();

        for i in 0..n {
            stack.push_back((0usize, i, false));
        }

        let mut x_occupied = vec![false; n];
        let mut y_occupied = vec![false; 2 * n - 1];
        let mut z_occupied = vec![false; 2 * n - 1];

        while let Some((x, y, flag)) = stack.pop_back() {
            if flag {
                x_occupied[y] = false;
                y_occupied[n -  1 + x - y] = false;
                z_occupied[x + y] = false;
            } else {
                x_occupied[y] = true;
                y_occupied[n -  1 + x - y] = true;
                z_occupied[x + y] = true;

                stack.push_back((x, y, true));

                if x == n - 1 {
                    result += 1;
                    continue;
                }

                for j in 0..n {
                    // 注意这里在判断的点是(x + 1, j)
                    if !x_occupied[j] && !y_occupied[n + x - j] && !z_occupied[x + 1 + j] {
                        stack.push_back((x + 1, j, false));
                    }
                }
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_52() {
        assert_eq!(1, Solution::total_n_queens(1));
        assert_eq!(2, Solution::total_n_queens(4));
        assert_eq!(14200, Solution::total_n_queens(12));
    }
}
