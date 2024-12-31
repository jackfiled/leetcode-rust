/**
 * [3219] Minimum Cost for Cutting Cake II
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn minimum_cost(m: i32, n: i32, horizontal_cut: Vec<i32>, vertical_cut: Vec<i32>) -> i64 {
        let (m, n) = (m as usize, n as usize);
        let mut horizontal_cut: Vec<i64> = horizontal_cut.into_iter().map(|x| x as i64).collect();
        horizontal_cut.sort_unstable();
        let mut vertical_cut: Vec<i64> = vertical_cut.into_iter().map(|x| x as i64).collect();
        vertical_cut.sort_unstable();

        let mut result = 0;
        let (mut i, mut j) = (0, 0);

        for _ in 0..m + n - 2 {
            if j == n - 1 || i < m - 1 && horizontal_cut[i] < vertical_cut[j] {
                result += horizontal_cut[i] * (n - j) as i64;
                i += 1;
            } else {
                result += vertical_cut[j] * (m - i) as i64;
                j += 1;
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
    fn test_3219() {
        assert_eq!(13, Solution::minimum_cost(3, 2, vec![1, 3], vec![5]));
        assert_eq!(15, Solution::minimum_cost(2, 2, vec![7], vec![4]));
    }
}
