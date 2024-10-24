/**
 * [1997] First Day Where You Have Been in All the Rooms
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn first_day_been_in_all_rooms(next_visit: Vec<i32>) -> i32 {
        let m = 1_000_000_007;
        let mut dp = vec![0; next_visit.len()];
        dp[0] = 2;

        for i in 1..next_visit.len() {
            let next = next_visit[i] as usize;

            dp[i] = dp[i - 1] + 2;

            if next != 0 {
                dp[i] = (dp[i] - dp[next - 1] + m) % m;
            }
            dp[i] = (dp[i] + dp[i - 1]) % m;
        }

        dp[next_visit.len() - 2]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1997() {}
}
