/**
 * [3218] Minimum Cost for Cutting Cake I
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn minimum_cost(
        m: i32,
        n: i32,
        mut horizontal_cut: Vec<i32>,
        mut vertical_cut: Vec<i32>,
    ) -> i32 {
        let (m, n) = (m as usize, n as usize);
        horizontal_cut.sort_unstable();
        vertical_cut.sort_unstable();

        let mut result = 0;
        let (mut i, mut j) = (0, 0);

        for _ in 0..m + n - 2 {
            if j == n - 1 || i < m - 1 && horizontal_cut[i] < vertical_cut[j] {
                result += horizontal_cut[i] * (n - j) as i32;
                i += 1;
            } else {
                result += vertical_cut[j] * (m - i) as i32;
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
    fn test_3218() {
        assert_eq!(13, Solution::minimum_cost(3, 2, vec![1, 3], vec![5]));
        assert_eq!(15, Solution::minimum_cost(2, 2, vec![7], vec![4]));
    }
}
