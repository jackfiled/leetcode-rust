/**
 * [2555] Maximize Win From Two Segments
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn maximize_win(prize_positions: Vec<i32>, k: i32) -> i32 {
        let n = prize_positions.len();
        let mut dp = vec![0; n + 1];
        let mut result = 0;

        for i in 0..n {
            let j = Self::binary_search(&prize_positions, prize_positions[i] - k);
            result = result.max(i - j + 1 + dp[j]);

            dp[i + 1] = dp[i].max(i - j + 1);
        }

        result as i32
    }

    fn binary_search(array: &Vec<i32>, target: i32) -> usize {
        let (mut left, mut right) = (0, array.len());

        while left < right {
            let middle = left + (right - left) / 2;
            if array[middle] < target {
                left = middle + 1;
            } else {
                right = middle;
            }
        }

        left
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2555() {
        assert_eq!(7, Solution::maximize_win(vec![1, 1, 2, 2, 3, 3, 5], 2));
        assert_eq!(2, Solution::maximize_win(vec![1, 2, 3, 4], 0));
    }
}
