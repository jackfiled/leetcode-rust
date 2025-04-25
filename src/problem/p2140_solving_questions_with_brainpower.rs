/**
 * [2140] Solving Questions With Brainpower
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let questions: Vec<(i64, usize)> = questions
            .into_iter()
            .map(|a| (a[0] as i64, a[1] as usize))
            .collect();
        let n = questions.len();
        let mut dp = vec![0; n];

        for i in (0..n).rev() {
            let (score, brain_power) = questions[i];
            dp[i] = if i == n - 1 { 0 } else { dp[i + 1] };

            // 选择做i题就需要跳过brian_power道题
            if i + brain_power + 1 < n {
                dp[i] = dp[i].max(dp[i + brain_power + 1] + score);
            } else {
                dp[i] = dp[i].max(score);
            }
        }

        dp[0]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2140() {
        assert_eq!(
            5,
            Solution::most_points(vec![vec![3, 2], vec![4, 3], vec![4, 4], vec![2, 5]])
        );
        assert_eq!(
            7,
            Solution::most_points(vec![
                vec![1, 1],
                vec![2, 2],
                vec![3, 3],
                vec![4, 4],
                vec![5, 5]
            ])
        );
    }
}
